//! v3: LLM (Opus via `claude -p`, like `wdvmm`) — a second opinion for the core.
//!
//! - `confirm`: candidates from `seeds/wd-candidates.tsv` without an LLM verdict in any `seeds/wd-confirm*.tsv` → «yes / no /
//!   refine + why» → `seeds/wd-confirm-v3.tsv`. Contested ones first (confidence < 0.8), then confident ones.
//! - `risks`: risks of every core concept (act → harm → probability) → `seeds/core-risks.tsv`. The probability is an
//!   order-of-magnitude estimate from the LLM, not measured statistics; it is labelled so in the core.
//!
//! One thread (ONTO_VMM_THREADS=2 — two; ≤ 2 `claude` processes), the cap is the sum of `cost_usd` from the log; exceeding it stops without retries.

use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};

use crate::model::*;
use crate::wdvmm;

/// Core fields checked by the LLM (colour and «without part» stay candidates).
pub const VMM_PROPS: &[&str] = &["food", "habitat", "part", "material", "use", "mass", "height", "length", "lifespan", "speed"];

pub fn is_number_prop(p: &str) -> bool {
    matches!(p, "mass" | "height" | "length" | "lifespan" | "speed")
}

/// Verdict key: concept, synset, property, value, source.
pub type Key = (String, String, String, String, String);

/// All LLM verdicts from `seeds/wd-confirm*.tsv`: key → (verdict, why, file).
pub fn verdicts(seeds: &Path) -> Result<HashMap<Key, (String, String, String)>> {
    let mut m = HashMap::new();
    let mut files: Vec<PathBuf> = std::fs::read_dir(seeds)?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("wd-confirm") && n.ends_with(".tsv"))).collect();
    files.sort();
    for p in files {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        for l in std::fs::read_to_string(&p)?.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() < 7 {
                continue;
            }
            m.insert((f[0].into(), f[1].into(), f[2].into(), f[3].into(), f[4].into()), (f[6].to_string(), f.get(7).unwrap_or(&"").to_string(), name.clone()));
        }
    }
    Ok(m)
}

/// Candidate rows (TSV fields), without comments.
pub fn candidates(path: &Path) -> Result<Vec<Vec<String>>> {
    Ok(std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {}", path.display()))?
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| l.split('\t').map(String::from).collect::<Vec<_>>())
        .filter(|f| f.len() >= 13)
        .collect())
}

pub fn key_of(f: &[String]) -> Key {
    (f[0].clone(), f[2].clone(), f[3].clone(), f[4].clone(), f[5].clone())
}

/// Synset → concept (for explanations in the prompt).
pub fn synset_index(g: &Graph) -> HashMap<String, Cid> {
    (0..g.n_concepts() as Cid).filter(|&c| g.src[c as usize] == Src::Oewn).map(|c| (g.cid_str(c), c)).collect()
}

fn gloss(g: &Graph, idx: &HashMap<String, Cid>, syn: &str) -> String {
    idx.get(syn).map(|&c| g.gloss_of(c).chars().take(70).collect()).unwrap_or_default()
}

fn num_value(f: &[String]) -> String {
    let sd = if f[8].is_empty() { "?".to_string() } else { f[8].clone() };
    format!("mean {} {}, sd {sd}, range {}–{}, n={}", f[7], f[12], f[9], f[10], f[11])
}

pub fn confirm_prompt(g: &Graph, idx: &HashMap<String, Cid>, rows: &[Vec<String>]) -> String {
    let mut s = String::from(
        "You check candidate common-sense facts for a world model (real world, not fairy tales). Each line: id, concept (English lemma — WordNet gloss), property, value (from Wikidata), where it came from.\n\
Properties: food = what it typically eats (main diet, not every possible prey); habitat = where it typically lives; part = a typical instance has this part; material = what it is typically made of; use = what it is typically used or kept for; mass/height/length/lifespan/speed = distribution for a typical adult (mean, sd, range, n = number of Wikidata values).\n\
For EACH id answer one line exactly: id<TAB>yes|no|refine<TAB>why (max 12 words; for refine — the better value).\n\
Answer yes only if a typical instance of the concept has it (default knowledge). No: inherited from a too general class, a different sense of the word, not really a part, an occasional food. Numbers: yes if a typical adult value lies in the range and the mean is within a factor of 2 of it; otherwise no or refine.\n\n",
    );
    for (i, f) in rows.iter().enumerate() {
        let v = if is_number_prop(&f[3]) { num_value(f) } else { f[4].clone() };
        s.push_str(&format!("{}\t{} — {}\t{}\t{}\t{}\n", i + 1, f[0].replace('_', " "), gloss(g, idx, &f[2]), f[3], v, f[5]));
    }
    s
}

/// Candidates for the LLM: no verdict, core fields, confidence ≥ 0.45; contested first (< 0.8), then confident.
pub fn pick_unverified(cands: &[Vec<String>], seen: &HashMap<Key, (String, String, String)>) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = cands
        .iter()
        .filter(|f| VMM_PROPS.contains(&f[3].as_str()) && !seen.contains_key(&key_of(f)) && f[6].parse::<f64>().map(|c| c >= 0.45).unwrap_or(false))
        .cloned()
        .collect();
    let mut uniq = HashSet::new();
    v.retain(|f| uniq.insert(key_of(f)));
    v.sort_by_key(|f| (f[6].parse::<f64>().map(|c| c >= 0.8).unwrap_or(true), f[0].clone(), f[3].clone()));
    v
}

pub fn risk_prompt(g: &Graph, idx: &HashMap<String, Cid>, rows: &[(String, String, String)]) -> String {
    let mut s = String::from(
        "For a world model of the real world (not fairy tales): for each concept list the typical risks of harm to an ordinary adult person in ordinary contact.\n\
Answer one line per risk, exactly: id<TAB>act<TAB>harm<TAB>p<TAB>why (max 10 words)\n\
act: touch (pet, hold, handle) | approach (be near it) | eat (eat or drink it) | use (use it as intended)\n\
harm: bite | scratch | sting | venom | kick | crush | cut | burn | poison | shock | allergy | fall | drown\n\
p: probability of that harm per such contact, a number 0–1 (order of magnitude is enough: 0.5, 0.1, 0.01, 0.001).\n\
At most 3 risks per concept, only with p >= 0.001. If none is notable: id<TAB>none\n\n",
    );
    for (i, (lemma, _dom, syn)) in rows.iter().enumerate() {
        s.push_str(&format!("{}\t{} — {}\n", i + 1, lemma.replace('_', " "), gloss(g, idx, syn)));
    }
    s
}

pub const ACTS: &[&str] = &["touch", "approach", "eat", "use"];
pub const HARMS: &[&str] = &["bite", "scratch", "sting", "venom", "kick", "crush", "cut", "burn", "poison", "shock", "allergy", "fall", "drown"];

/// Parsing risks: id → [(act, harm, p, why)]; `none` — empty list (explicitly «no risks»).
pub fn parse_risks(ans: &str, n: usize) -> Vec<Option<Vec<(String, String, f64, String)>>> {
    let mut v: Vec<Option<Vec<(String, String, f64, String)>>> = vec![None; n];
    for l in ans.lines() {
        let f: Vec<&str> = l.split('\t').map(str::trim).collect();
        let Ok(i) = f.first().unwrap_or(&"").trim_start_matches('#').parse::<usize>() else { continue };
        if !(1..=n).contains(&i) || f.len() < 2 {
            continue;
        }
        let slot = v[i - 1].get_or_insert_with(Vec::new);
        if f[1].eq_ignore_ascii_case("none") {
            continue;
        }
        if f.len() < 4 {
            continue;
        }
        let (act, harm) = (f[1].to_ascii_lowercase(), f[2].to_ascii_lowercase());
        let Ok(p) = f[3].parse::<f64>() else { continue };
        if ACTS.contains(&act.as_str()) && HARMS.contains(&harm.as_str()) && (0.0..=1.0).contains(&p) {
            slot.push((act, harm, p, f.get(4).unwrap_or(&"").replace('\t', " ")));
        }
    }
    v
}

/// Batches in two threads; `f(batch) -> output lines` are appended to `out` under a lock.
fn run_batches<T: Sync>(batches: &[&[T]], out: &Path, f: &(dyn Fn(usize, &[T]) -> Result<(String, f64)> + Sync)) -> Result<f64> {
    let file = Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(out)?);
    let next = Mutex::new(0usize);
    let cost = Mutex::new(0.0f64);
    let err: Mutex<Option<anyhow::Error>> = Mutex::new(None);
    std::thread::scope(|s| {
        let threads: usize = std::env::var("ONTO_VMM_THREADS").ok().and_then(|x| x.parse().ok()).unwrap_or(1).clamp(1, 2);
        for _ in 0..threads {
            s.spawn(|| loop {
                if err.lock().unwrap().is_some() {
                    return;
                }
                let i = {
                    let mut n = next.lock().unwrap();
                    let i = *n;
                    *n += 1;
                    i
                };
                if i >= batches.len() {
                    return;
                }
                match f(i, batches[i]) {
                    Ok((lines, c)) => {
                        *cost.lock().unwrap() += c;
                        let _ = file.lock().unwrap().write_all(lines.as_bytes());
                        println!("batch {}/{}: ${c:.3}", i + 1, batches.len());
                    }
                    Err(e) => {
                        eprintln!("batch {}: {e}", i + 1);
                        *err.lock().unwrap() = Some(e);
                        return;
                    }
                }
            });
        }
    });
    let c = *cost.lock().unwrap();
    if let Some(e) = err.into_inner().unwrap() {
        return Err(e.context(format!("stopped; spent in this run ${c:.2}")));
    }
    Ok(c)
}

pub fn confirm(g: &Graph, seeds: &Path, runs: &Path, batch: usize, cap: f64, limit: usize) -> Result<String> {
    let cands = candidates(&seeds.join("wd-candidates.tsv"))?;
    let seen = verdicts(seeds)?;
    let mut rows = pick_unverified(&cands, &seen);
    rows.truncate(limit);
    let idx = synset_index(g);
    let out = seeds.join("wd-confirm-v3.tsv");
    if !out.exists() {
        std::fs::write(&out, "# onto v3: candidates from wd-candidates.tsv without an LLM verdict (contested < 0.8 first, then confident) — LLM second opinion (claude-opus-5-5, effort high, onto core-vmm confirm). Not the truth.\n# concept\tsynset\tprop\tvalue\tsource\tconfidence\tvmm\twhy\n")?;
    }
    let log = runs.join("vmm-calls.jsonl");
    let batches: Vec<&[Vec<String>]> = rows.chunks(batch.max(1)).collect();
    let n = rows.len();
    let cost = run_batches(&batches, &out, &|i, ch| {
        let (ans, c) = wdvmm::ask(&runs.join(format!("vmm-cwd-{}", i % 2)), &log, cap, 1.0, &confirm_prompt(g, &idx, ch))?;
        let mut s = String::new();
        for (f, v) in ch.iter().zip(wdvmm::parse(&ans, ch.len())) {
            let (verdict, why) = v.unwrap_or(("no answer".into(), String::new()));
            s.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{verdict}\t{}\n", f[0], f[2], f[3], f[4], f[5], f[6], why.replace('\t', " ")));
        }
        Ok((s, c))
    })?;
    Ok(format!("LLM, candidate push: {n} rows, {} batches, ${cost:.2} (log ${:.2}) → {}", batches.len(), wdvmm::spent(&log), out.display()))
}

pub fn risks(g: &Graph, core: &[(String, String, String)], seeds: &Path, runs: &Path, batch: usize, cap: f64) -> Result<String> {
    let idx = synset_index(g);
    let out = seeds.join("core-risks.tsv");
    std::fs::write(&out, "# onto v3: risks of core concepts — order-of-magnitude estimate from the LLM (claude-opus-5-5, effort high, onto core-vmm risks), not measured statistics.\n# p — probability of harm per one such contact for an ordinary adult; none — the LLM sees no notable risk.\n# concept\tdomain\tsynset\tact\tharm\tp\twhy\n")?;
    let log = runs.join("vmm-calls.jsonl");
    let batches: Vec<&[(String, String, String)]> = core.chunks(batch.max(1)).collect();
    let cost = run_batches(&batches, &out, &|i, ch| {
        let (ans, c) = wdvmm::ask(&runs.join(format!("vmm-cwd-{}", i % 2)), &log, cap, 1.0, &risk_prompt(g, &idx, ch))?;
        let mut s = String::new();
        for ((lemma, dom, syn), v) in ch.iter().zip(parse_risks(&ans, ch.len())) {
            match v {
                None => s.push_str(&format!("{lemma}\t{dom}\t{syn}\t-\t-\t-\tno answer\n")),
                Some(rs) if rs.is_empty() => s.push_str(&format!("{lemma}\t{dom}\t{syn}\tnone\tnone\t0\tno risks\n")),
                Some(rs) => {
                    for (a, h, p, why) in rs {
                        s.push_str(&format!("{lemma}\t{dom}\t{syn}\t{a}\t{h}\t{p}\t{why}\n"));
                    }
                }
            }
        }
        Ok((s, c))
    })?;
    Ok(format!("LLM, risks: {} concepts, ${cost:.2} (log ${:.2}) → {}", core.len(), wdvmm::spent(&log), out.display()))
}

#[cfg(test)]
mod t {
    #[test]
    fn parse_risk_answer() {
        let v = super::parse_risks("1\ttouch\tbite\t0.3\twild lion\n1\tapproach\tcrush\t0.01\tcharge\n2\tnone\n3\tpet\tbite\t0.2\tbad act\n4\ttouch\tbite\t7\tbad p", 4);
        assert_eq!(v[0].as_ref().unwrap().len(), 2);
        assert!(v[1].as_ref().unwrap().is_empty());
        assert!(v[2].as_ref().unwrap().is_empty(), "act outside the list is rejected");
        assert!(v[3].as_ref().unwrap().is_empty(), "p outside [0, 1] is rejected");
    }
}
