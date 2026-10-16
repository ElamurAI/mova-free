//! v3: description judge — Opus via `claude -p` (like `wdvmm`), fixed rubric, blind: "before" descriptions (no core) and
//! "after" descriptions (core v3) are shuffled together under random numbers; the judge knows neither version nor round.
//!
//! Rubric — four 1–5 scores: C correctness, F completeness, U usefulness, E errors (5 — none) + the main error.
//! Class split: ~2/3 training (seen by the architect), ~1/3 held out (only scored); by name hash.
//! Judge control: 20 descriptions a second time with a differently worded rubric — the scores must agree.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::Result;

use crate::wdvmm;

pub const RUBRIC: &str = "You are a strict judge of short concept descriptions made by a small symbolic world model for fairy tales and fables (real-world facts about a typical instance of the concept). The first line names the word and the chosen sense.\n\
Score each description with four integers 1-5:\n\
C correctness: are the stated facts true for a typical instance of that sense? 5 = all true, 1 = mostly wrong.\n\
F completeness: does it cover what matters for this kind of thing (animal: kind, parts, food, habitat, size, danger; object: parts, material, use, size; food: ingredients, use; person: role)? Lines saying unknown count as missing.\n\
U usefulness: would a story-understanding system learn specific, relevant knowledge? Penalize generic or irrelevant items.\n\
E errors: 5 = no wrong or misleading line, 1 = many. A wrong sense for the word as used in fairy tales is a major error.\n\
Answer one line per id exactly: id<TAB>C<TAB>F<TAB>U<TAB>E<TAB>main error or - (max 12 words)\n\n";

pub const RUBRIC_ALT: &str = "Rate each description of a concept (produced by a rule-based world model used to read fairy tales; it should state typical real-world facts about the concept) on four 1-to-5 scales.\n\
C - are the facts right for an ordinary example of the sense given on the first line (5 all right, 1 mostly false)?\n\
F - how much of the important knowledge is present (for animals: what it is, body parts, diet, where it lives, size, how dangerous; for things: parts, what it is made of, what it is for, size; for foods: what it is made of, how it is used; for people: their role)? Unknown lines are gaps.\n\
U - how useful and specific is it for understanding stories (generic or off-topic lines lower it)?\n\
E - freedom from mistakes (5 none, 1 many); picking the wrong meaning of the word is a serious mistake.\n\
Reply with one line per id, exactly: id<TAB>C<TAB>F<TAB>U<TAB>E<TAB>the main mistake or - (at most 12 words)\n\n";

#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub class: String,
    pub version: String,
    pub category: String,
    pub text: String,
}

/// Training (true) or held-out part — by class name hash (FNV-1a), ~2/3 training.
pub fn is_train(class: &str) -> bool {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in class.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h % 3 != 0
}

/// Description for the judge: without source brackets (fewer tokens; facts remain), up to 900 characters.
pub fn clean(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0;
    for ch in text.chars() {
        match ch {
            '[' => depth += 1,
            ']' if depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out.chars().take(900).collect()
}

pub fn read_items(runs: &Path) -> Result<Vec<Item>> {
    let mut v = Vec::new();
    for ver in ["before", "after"] {
        let t = std::fs::read_to_string(runs.join(format!("describe-{ver}.tsv")))?;
        for l in t.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() >= 6 {
                v.push(Item { id: 0, class: f[0].into(), version: ver.into(), category: f[2].into(), text: f[5].replace("\\n", "\n") });
            }
        }
    }
    // shuffle together (xorshift, fixed seed) and assign random numbers
    let mut x: u64 = 20260926;
    for i in (1..v.len()).rev() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        v.swap(i, (x % (i as u64 + 1)) as usize);
    }
    for (i, it) in v.iter_mut().enumerate() {
        it.id = 1000 + i;
    }
    Ok(v)
}

pub fn prompt(rubric: &str, items: &[&Item]) -> String {
    let mut s = String::from(rubric);
    for it in items {
        let _ = writeln!(s, "=== id {}\n{}\n", it.id, clean(&it.text).trim());
    }
    s
}

/// Parse: id → (C, F, U, E, error).
pub fn parse(ans: &str) -> BTreeMap<usize, ([u8; 4], String)> {
    let mut m = BTreeMap::new();
    for l in ans.lines() {
        let f: Vec<&str> = l.split('\t').map(str::trim).collect();
        if f.len() < 5 {
            continue;
        }
        let Ok(id) = f[0].trim_start_matches("id").trim().parse::<usize>() else { continue };
        let sc: Vec<u8> = f[1..5].iter().filter_map(|x| x.parse::<u8>().ok()).filter(|x| (1..=5).contains(x)).collect();
        if sc.len() == 4 {
            m.insert(id, ([sc[0], sc[1], sc[2], sc[3]], f.get(5).unwrap_or(&"-").to_string()));
        }
    }
    m
}

pub fn mean4(s: &[u8; 4]) -> f64 {
    s.iter().map(|x| *x as f64).sum::<f64>() / 4.0
}

/// Judge run: all descriptions in batches → `describe-judge.tsv`; control — 20 descriptions with the second rubric.
pub fn run(runs: &Path, batch: usize, cap: f64) -> Result<String> {
    let items = read_items(runs)?;
    let log = runs.join("judge-calls.jsonl");
    let out = runs.join("describe-judge.tsv");
    let mut done: BTreeMap<usize, ([u8; 4], String)> = BTreeMap::new();
    if out.exists() {
        for l in std::fs::read_to_string(&out)?.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() >= 10 && f[4] == "main" {
                if let (Ok(id), Ok(c), Ok(fi), Ok(u), Ok(e)) = (f[0].parse(), f[5].parse(), f[6].parse(), f[7].parse(), f[8].parse()) {
                    done.insert(id, ([c, fi, u, e], f[9].to_string()));
                }
            }
        }
    } else {
        std::fs::write(&out, "# id\tclass\tversion\tsplit\trubric\tC\tF\tU\tE\terror\n")?;
    }
    let todo: Vec<&Item> = items.iter().filter(|i| !done.contains_key(&i.id)).collect();
    let file = std::sync::Mutex::new(std::fs::OpenOptions::new().append(true).open(&out)?);
    let chunks: Vec<&[&Item]> = todo.chunks(batch.max(1)).collect();
    let next = std::sync::Mutex::new(0usize);
    let err = std::sync::Mutex::new(None::<String>);
    let work = |rubric: &'static str, tag: &'static str, chunks: &[&[&Item]]| {
        std::thread::scope(|s| {
            for t in 0..2 {
                let (next, err, file, log) = (&next, &err, &file, &log);
                s.spawn(move || loop {
                    if err.lock().unwrap().is_some() {
                        return;
                    }
                    let i = {
                        let mut n = next.lock().unwrap();
                        *n += 1;
                        *n - 1
                    };
                    if i >= chunks.len() {
                        return;
                    }
                    match wdvmm::ask(&runs.join(format!("judge-cwd-{t}")), log, cap, 0.6, &prompt(rubric, chunks[i])) {
                        Ok((ans, c)) => {
                            let m = parse(&ans);
                            let mut s = String::new();
                            for it in chunks[i] {
                                if let Some((sc, e)) = m.get(&it.id) {
                                    let _ = writeln!(s, "{}\t{}\t{}\t{}\t{tag}\t{}\t{}\t{}\t{}\t{}", it.id, it.class, it.version, if is_train(&it.class) { "train" } else { "holdout" }, sc[0], sc[1], sc[2], sc[3], e.replace('\t', " "));
                                }
                            }
                            let _ = std::io::Write::write_all(&mut *file.lock().unwrap(), s.as_bytes());
                            println!("{tag} {}/{}: ${c:.3}", i + 1, chunks.len());
                        }
                        Err(e) => {
                            *err.lock().unwrap() = Some(e.to_string());
                            return;
                        }
                    }
                });
            }
        });
    };
    work(RUBRIC, "main", &chunks);
    if let Some(e) = err.lock().unwrap().take() {
        anyhow::bail!("judge stopped: {e}");
    }
    // judge control: 20 "after" descriptions — with the second rubric
    let has_alt = std::fs::read_to_string(&out)?.lines().any(|l| l.split('\t').nth(4) == Some("alt"));
    if !has_alt {
        let ctl: Vec<&Item> = items.iter().filter(|i| i.version == "after").step_by(15).take(20).collect();
        *next.lock().unwrap() = 0;
        let ch: Vec<&[&Item]> = ctl.chunks(10).collect();
        work(RUBRIC_ALT, "alt", &ch);
    }
    Ok(format!("judge: {} descriptions, spent per log ${:.2}", items.len(), wdvmm::spent(&log)))
}

/// Report: means by version and part, distribution, categories, best and worst, typical errors, judge control.
pub fn report(runs: &Path) -> Result<String> {
    let items = read_items(runs)?;
    let by_id: BTreeMap<usize, &Item> = items.iter().map(|i| (i.id, i)).collect();
    let text = std::fs::read_to_string(runs.join("describe-judge.tsv"))?;
    let mut main: BTreeMap<usize, ([u8; 4], String)> = BTreeMap::new();
    let mut alt: BTreeMap<usize, [u8; 4]> = BTreeMap::new();
    for l in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 10 {
            continue;
        }
        let (Ok(id), Ok(c), Ok(fi), Ok(u), Ok(e)) = (f[0].parse::<usize>(), f[5].parse::<u8>(), f[6].parse::<u8>(), f[7].parse::<u8>(), f[8].parse::<u8>()) else { continue };
        if f[4] == "main" {
            main.insert(id, ([c, fi, u, e], f[9].to_string()));
        } else {
            alt.insert(id, [c, fi, u, e]);
        }
    }
    let mut s = String::from("# Opus judge: descriptions of fairy-tale and fable classes, before and after core v3\n\nBlind: \"before\" descriptions (graph and seeds, v2) and \"after\" descriptions (core v3, then graph) are shuffled under random numbers. Fixed rubric (`judge::RUBRIC`), scores 1–5: C correctness, F completeness, U usefulness, E errors (5 — none).\n\n");
    s.push_str("| version | part | descriptions | C | F | U | E | mean |\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for ver in ["before", "after"] {
        for split in ["train", "holdout", "all"] {
            let v: Vec<&[u8; 4]> = main.iter().filter(|(id, _)| by_id.get(id).is_some_and(|it| it.version == ver && (split == "all" || (split == "train") == is_train(&it.class)))).map(|(_, x)| &x.0).collect();
            let n = v.len().max(1) as f64;
            let m = |k: usize| v.iter().map(|x| x[k] as f64).sum::<f64>() / n;
            let _ = writeln!(s, "| {} | {split} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |", if ver == "before" { "before" } else { "after" }, v.len(), m(0), m(1), m(2), m(3), (m(0) + m(1) + m(2) + m(3)) / 4.0);
        }
    }
    // distribution of the mean score
    s.push_str("\n**Distribution of the mean description score** (rounded to an integer):\n\n| version | 1 | 2 | 3 | 4 | 5 |\n|---|---:|---:|---:|---:|---:|\n");
    for ver in ["before", "after"] {
        let mut h = [0usize; 5];
        for (id, (sc, _)) in &main {
            if by_id.get(id).is_some_and(|it| it.version == ver) {
                h[(mean4(sc).round() as usize).clamp(1, 5) - 1] += 1;
            }
        }
        let _ = writeln!(s, "| {} | {} | {} | {} | {} | {} |", if ver == "before" { "before" } else { "after" }, h[0], h[1], h[2], h[3], h[4]);
    }
    // categories
    s.push_str("\n**By category** (mean before → after) and typical errors after:\n\n");
    let cats: std::collections::BTreeSet<&str> = items.iter().map(|i| i.category.as_str()).collect();
    for cat in cats {
        let avg = |ver: &str| {
            let v: Vec<f64> = main.iter().filter(|(id, _)| by_id.get(id).is_some_and(|it| it.version == ver && it.category == cat)).map(|(_, x)| mean4(&x.0)).collect();
            (v.iter().sum::<f64>() / v.len().max(1) as f64, v.len())
        };
        let (b, nb) = avg("before");
        let (a, _) = avg("after");
        let errs: Vec<String> = main.iter().filter(|(id, (_, e))| e != "-" && by_id.get(id).is_some_and(|it| it.version == "after" && it.category == cat)).map(|(id, (_, e))| format!("{}: {e}", by_id[id].class)).take(4).collect();
        let _ = writeln!(s, "- **{cat}** ({nb} classes): {b:.2} → {a:.2}. {}", errs.join("; "));
    }
    // best and worst after
    let mut after: Vec<(f64, usize)> = main.iter().filter(|(id, _)| by_id.get(id).is_some_and(|it| it.version == "after")).map(|(id, (sc, _))| (mean4(sc), *id)).collect();
    after.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    for (title, pick) in [("3 best after", after.iter().take(3).collect::<Vec<_>>()), ("3 worst after", after.iter().rev().take(3).collect::<Vec<_>>())] {
        let _ = writeln!(s, "\n### {title}\n");
        for (m, id) in pick {
            let it = by_id[id];
            let _ = writeln!(s, "**{}** ({}) — {m:.2}; judge: {}\n\n```\n{}\n```\n", it.class, it.category, main[id].1, clean(&it.text).trim());
        }
    }
    // judge control
    let pairs: Vec<(f64, f64)> = alt.iter().filter_map(|(id, a)| main.get(id).map(|m| (mean4(&m.0), mean4(a)))).collect();
    if !pairs.is_empty() {
        let mad = pairs.iter().map(|(x, y)| (x - y).abs()).sum::<f64>() / pairs.len() as f64;
        let within = pairs.iter().filter(|(x, y)| (x - y).abs() <= 0.5).count();
        let rank = |v: Vec<f64>| -> Vec<f64> {
            let mut idx: Vec<usize> = (0..v.len()).collect();
            idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
            let mut r = vec![0.0; v.len()];
            let mut i = 0;
            while i < idx.len() {
                let mut j = i;
                while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
                    j += 1;
                }
                for k in i..=j {
                    r[idx[k]] = (i + j) as f64 / 2.0;
                }
                i = j + 1;
            }
            r
        };
        let (ra, rb) = (rank(pairs.iter().map(|p| p.0).collect()), rank(pairs.iter().map(|p| p.1).collect()));
        let n = ra.len() as f64;
        let (ma, mb) = (ra.iter().sum::<f64>() / n, rb.iter().sum::<f64>() / n);
        let cov: f64 = ra.iter().zip(&rb).map(|(a, b)| (a - ma) * (b - mb)).sum();
        let sd = |r: &[f64], m: f64| r.iter().map(|a| (a - m).powi(2)).sum::<f64>().sqrt();
        let rho = cov / (sd(&ra, ma) * sd(&rb, mb)).max(1e-9);
        let _ = writeln!(s, "\n**Judge control:** {} descriptions a second time with a differently worded rubric (`RUBRIC_ALT`): mean difference of mean scores {mad:.2}, within ±0.5 — {within}/{}, Spearman ρ = {rho:.2}. {}\n", pairs.len(), pairs.len(), if rho >= 0.6 && mad <= 0.6 { "Consistent ✓" } else { "Weak consistency ✗ — trust the judge scores with caution" });
    }
    Ok(s)
}

/// Architect ("student — architect" round): sees the scores and examples of only the training "after" part and proposes 1–3
/// changes to the SLM architecture. No code is executed — Opus itself evaluates. Output — `architect-r{round}.md`.
pub fn architect(runs: &Path, round: usize, cap: f64) -> Result<String> {
    let items = read_items(runs)?;
    let by_id: BTreeMap<usize, &Item> = items.iter().map(|i| (i.id, i)).collect();
    let text = std::fs::read_to_string(runs.join("describe-judge.tsv"))?;
    let mut rows: Vec<(f64, &Item, [u8; 4], String)> = Vec::new();
    for l in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 10 || f[4] != "main" {
            continue;
        }
        let (Ok(id), Ok(c), Ok(fi), Ok(u), Ok(e)) = (f[0].parse::<usize>(), f[5].parse::<u8>(), f[6].parse::<u8>(), f[7].parse::<u8>(), f[8].parse::<u8>()) else { continue };
        let Some(it) = by_id.get(&id) else { continue };
        if it.version == "after" && is_train(&it.class) {
            rows.push((mean4(&[c, fi, u, e]), it, [c, fi, u, e], f[9].to_string()));
        }
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.class.cmp(&b.1.class)));
    let n = rows.len().max(1) as f64;
    let mut p = String::from("You are the architect of a small deterministic symbolic world model (Rust) that writes short descriptions of concepts from fairy tales and fables. Its materials: Open English WordNet 2025 (synsets, glosses, hypernyms, meronyms), FrameNet 1.7, Wikidata claims (parts P527, use P366, food P1034, material P186, habitat P2974, numbers) confirmed by an LLM, hand-written default properties with exceptions per world layer, and LLM risk estimates. A compiled core covers 294 common concepts; other words fall back to WordNet.\n\
A judge scored descriptions (C correctness, F completeness, U usefulness, E errors; 1-5). Below: averages and examples from the training part only (worst first, then a few best).\n\
Propose 1-3 concrete architecture changes that would raise the scores on unseen classes (not just these examples): new sources or properties, sense-selection rules, how to parse WordNet glosses into fields, field priorities or filtering. For each: WHAT exactly (data + rule), WHY (cite example classes), EXPECTED EFFECT on C/F/U/E, RISK. No code execution is possible; reason from the examples. Max 350 words.\n\n");
    let m = |k: usize| rows.iter().map(|r| r.2[k] as f64).sum::<f64>() / n;
    let _ = writeln!(p, "Training part, {} descriptions: C {:.2}, F {:.2}, U {:.2}, E {:.2}\n", rows.len(), m(0), m(1), m(2), m(3));
    let pick: Vec<&(f64, &Item, [u8; 4], String)> = rows.iter().take(14).chain(rows.iter().rev().take(4)).collect();
    for (sc, it, s4, e) in pick {
        let _ = writeln!(p, "--- {} ({}): mean {sc:.2} [C{} F{} U{} E{}] judge: {e}\n{}\n", it.class, it.category, s4[0], s4[1], s4[2], s4[3], clean(&it.text).chars().take(600).collect::<String>().trim());
    }
    let log = runs.join("judge-calls.jsonl");
    let (ans, c) = wdvmm::ask(&runs.join("judge-cwd-0"), &log, cap, 0.6, &p)?;
    let out = format!("# Architect, round {round}\n\nSaw only the training \"after\" part ({} descriptions). Cost ${c:.2}.\n\n{ans}\n", rows.len());
    std::fs::write(runs.join(format!("architect-r{round}.md")), &out)?;
    Ok(out)
}
