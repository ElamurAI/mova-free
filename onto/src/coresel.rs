//! v3: core selection — `onto core-select` → `data/core-props.tsv`, from which `build.rs` compiles Rust types.
//!
//! **Selection rule** (candidates `seeds/wd-candidates.tsv`, LLM verdicts `seeds/wd-confirm*.tsv`):
//! - the LLM said "yes" — into the core (`vmm-yes`);
//! - confidence ≥ 0.7 and the LLM did not say "no" — into the core (`confident`; "refine" — `vmm-refine` with a note);
//! - the rest stay candidates.
//!
//! **Every "concept × field" cell is explicit:** a value (`known`), "no risks" (`none`), "unknown" with a reason
//! (`unknown`) or "does not apply to the domain" (`n/a`). `build.rs` does not let an empty cell through — the completeness gate.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

use crate::corevmm::{self, is_number_prop};
use crate::model::*;
use crate::wdwalk;

/// Core fields in record order.
pub const FIELDS: [&str; 11] = ["food", "habitat", "part", "material", "use", "mass", "height", "length", "lifespan", "speed", "risk"];

/// Whether a field applies to a domain. An animal is not made of a material, a table does not eat; people and magical beings — only sizes and risks.
pub fn applicable(domain: &str, field: &str) -> bool {
    match domain {
        "animal" => field != "material",
        "plant" => !matches!(field, "food" | "material" | "speed"),
        "food" => matches!(field, "material" | "use" | "part" | "mass" | "risk"),
        "household" => !matches!(field, "food" | "habitat" | "lifespan" | "speed"),
        "family" | "role" => matches!(field, "mass" | "height" | "risk"),
        "magical" => field == "risk",
        _ => false,
    }
}

/// Source from a candidate row: `Q18498 grey wolf P1034 normal ← anchor Q8331 fox (2 steps)`,
/// `distribution over 1 descendants Q8331 fox (P2067): Q200400` → (Q, P, rank, steps).
pub fn parse_source(s: &str) -> (u32, u16, &'static str, u8) {
    let num_after = |pfx: char| -> Option<u64> {
        let mut it = s.char_indices().filter(|(_, c)| *c == pfx);
        it.find_map(|(i, _)| {
            let d: String = s[i + 1..].chars().take_while(|c| c.is_ascii_digit()).collect();
            let before_ok = i == 0 || !s[..i].chars().last().is_some_and(|c| c.is_alphanumeric());
            (before_ok && !d.is_empty()).then(|| d.parse().ok()).flatten()
        })
    };
    let q = num_after('Q').unwrap_or(0) as u32;
    let p = num_after('P').unwrap_or(0) as u16;
    let rank = if s.contains(" preferred") {
        "preferred"
    } else if s.contains(" normal") {
        "normal"
    } else if s.contains(" deprecated") {
        "deprecated"
    } else {
        "-"
    };
    let steps = s.find(" steps)").and_then(|i| s[..i].rsplit('(').next()).and_then(|x| x.trim().parse().ok()).unwrap_or(0);
    (q, p, rank, steps)
}

/// Q → OEWN synset from the strict v2 alignment (`accepted`, `also`) — only for the needed Qs.
pub fn q_synsets(align: &Path, want: &std::collections::HashSet<u32>) -> Result<HashMap<u32, String>> {
    let mut m = HashMap::new();
    if !align.exists() {
        return Ok(m);
    }
    let text = std::fs::read_to_string(align).with_context(|| format!("cannot read {}", align.display()))?;
    for l in text.lines() {
        let mut f = l.split('\t');
        let (Some(syn), Some(q), Some(st)) = (f.next(), f.next(), f.next()) else { continue };
        if !(st == "accepted" || st == "also") || !syn.ends_with("-n") {
            continue;
        }
        if let Ok(q) = q.parse::<u32>() {
            if want.contains(&q) {
                m.entry(q).or_insert_with(|| syn.to_string());
            }
        }
    }
    Ok(m)
}

pub struct SelReport {
    pub text: String,
}

/// Selection: `data/core.tsv` + candidates + verdicts + risks → `data/core-props.tsv`.
pub fn select(g: &Graph, core_path: &Path, seeds: &Path, align: &Path, out: &Path) -> Result<SelReport> {
    let core = wdwalk::read_core(g, core_path)?;
    let cands = corevmm::candidates(&seeds.join("wd-candidates.tsv"))?;
    let seen = corevmm::verdicts(seeds)?;
    // object values → synsets
    let want: std::collections::HashSet<u32> = cands.iter().filter_map(|f| f[4].strip_prefix('Q').and_then(|r| r.split(' ').next()).and_then(|q| q.parse().ok())).collect();
    let qsyn = q_synsets(align, &want)?;
    // risks
    let mut risks: HashMap<(String, String), Vec<Vec<String>>> = HashMap::new();
    let rp = seeds.join("core-risks.tsv");
    if rp.exists() {
        for l in std::fs::read_to_string(&rp)?.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let f: Vec<String> = l.split('\t').map(String::from).collect();
            if f.len() >= 7 {
                risks.entry((f[0].clone(), f[1].clone())).or_default().push(f);
            }
        }
    }
    let mut by: HashMap<(String, String), Vec<&Vec<String>>> = HashMap::new();
    for f in &cands {
        by.entry((f[0].clone(), f[3].clone())).or_default().push(f);
    }
    let mut s = String::from(
        "# onto v3: core — every \"concept × field\" cell is explicit (onto core-select; compiled by build.rs, completeness gate).\n\
# status: known — a value; none — explicitly empty (no risks); unknown — unknown, reason in note; n/a — the field does not apply to the domain.\n\
# confirm: vmm-yes — LLM \"yes\"; confident — confidence ≥ 0.7 with no LLM \"no\"; vmm-refine — same, LLM refined it (note); vmm-estimate — LLM estimate (risks).\n\
# Numbers: mean sd min max n unit (empty sd — unknown); risk: value \"action harm\", mean — probability per contact.\n\
# concept\tdomain\tsynset\tfield\tstatus\tvalue\tvalue_synset\tq\tp\trank\tsteps\tconf\tconfirm\tmean\tsd\tmin\tmax\tn\tunit\tnote\n",
    );
    let (mut n_known, mut n_unknown, mut n_na, mut n_vals) = (0usize, 0usize, 0usize, 0usize);
    let mut per_field: std::collections::BTreeMap<&str, [usize; 3]> = Default::default();
    for c in &core {
        let syn = c.synset.map(|x| g.cid_str(x)).unwrap_or_else(|| "-".into());
        let head = format!("{}\t{}\t{syn}", c.lemma, c.domain);
        for field in FIELDS {
            let pf = per_field.entry(field).or_default();
            if !applicable(&c.domain, field) {
                let _ = writeln!(s, "{head}\t{field}\tn/a\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tfield does not apply to domain {}", c.domain);
                n_na += 1;
                pf[2] += 1;
                continue;
            }
            if field == "risk" {
                match risks.get(&(c.lemma.clone(), c.domain.clone())) {
                    Some(rs) if rs.iter().any(|r| r[3] == "none") => {
                        let _ = writeln!(s, "{head}\trisk\tnone\t\t\t\t\t\t\t\tvmm-estimate\t0\t\t\t\t\t\tLLM: no notable risks");
                        n_known += 1;
                        pf[0] += 1;
                    }
                    Some(rs) if rs.iter().any(|r| r[3] != "-") => {
                        for r in rs.iter().filter(|r| r[3] != "-") {
                            let _ = writeln!(s, "{head}\trisk\tknown\t{} {}\t\t\t\t\t\t\tvmm-estimate\t{}\t\t\t\t\tp\t{}", r[3], r[4], r[5], r[6]);
                            n_vals += 1;
                        }
                        n_known += 1;
                        pf[0] += 1;
                    }
                    _ => {
                        let _ = writeln!(s, "{head}\trisk\tunknown\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tLLM did not assess risks");
                        n_unknown += 1;
                        pf[1] += 1;
                    }
                }
                continue;
            }
            let rows = by.get(&(c.lemma.clone(), field.to_string())).map(|v| v.as_slice()).unwrap_or(&[]);
            // only rows of the same synset (fish the animal and fish the food are different concepts)
            let rows: Vec<&&Vec<String>> = rows.iter().filter(|f| f[2] == syn).collect();
            let mut acc: Vec<(&Vec<String>, &str, String)> = Vec::new();
            let (mut no, mut weak) = (0, 0);
            for f in &rows {
                let conf: f64 = f[6].parse().unwrap_or(0.0);
                let v = seen.get(&corevmm::key_of(f));
                let (take, confirm, note) = match v.map(|x| x.0.as_str()) {
                    Some("yes") => (true, "vmm-yes", v.map(|x| x.1.clone()).unwrap_or_default()),
                    Some("no") => (false, "", String::new()),
                    Some("refine") if conf >= 0.7 => (true, "vmm-refine", format!("LLM refined: {}", v.map(|x| x.1.as_str()).unwrap_or(""))),
                    _ if v.is_none() && conf >= 0.7 => (true, "confident", String::new()),
                    _ => (false, "", String::new()),
                };
                if take {
                    acc.push((f, confirm, note));
                } else if v.is_some_and(|x| x.0 == "no") {
                    no += 1;
                } else {
                    weak += 1;
                }
            }
            if is_number_prop(field) && acc.len() > 1 {
                // one number per field: best confirmation, then confidence
                let rank = |c: &str| match c {
                    "vmm-yes" => 0,
                    "vmm-refine" => 1,
                    _ => 2,
                };
                acc.sort_by(|a, b| rank(a.1).cmp(&rank(b.1)).then(b.0[6].parse::<f64>().unwrap_or(0.0).total_cmp(&a.0[6].parse::<f64>().unwrap_or(0.0))));
                acc.truncate(1);
            }
            if acc.is_empty() {
                let why = if rows.is_empty() {
                    "no candidates from Wikidata".to_string()
                } else {
                    format!("{} candidates: LLM \"no\" — {no}, unconfirmed (confidence < 0.7 without \"yes\") — {weak}; left as candidates", rows.len())
                };
                let _ = writeln!(s, "{head}\t{field}\tunknown\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t{why}");
                n_unknown += 1;
                pf[1] += 1;
                continue;
            }
            n_known += 1;
            pf[0] += 1;
            let mut dedup = std::collections::HashSet::new();
            for (f, confirm, note) in acc {
                if !dedup.insert(f[4].clone()) {
                    continue;
                }
                n_vals += 1;
                let (q, p, rank, steps) = parse_source(&f[5]);
                let (value, vsyn) = if is_number_prop(field) {
                    (String::new(), String::new())
                } else {
                    let vq: Option<u32> = f[4].strip_prefix('Q').and_then(|r| r.split(' ').next()).and_then(|x| x.parse().ok());
                    (f[4].clone(), vq.and_then(|q| qsyn.get(&q).cloned()).unwrap_or_default())
                };
                let nums = if is_number_prop(field) { format!("{}\t{}\t{}\t{}\t{}\t{}", f[7], f[8], f[9], f[10], f[11], f[12]) } else { "\t\t\t\t\t".into() };
                let _ = writeln!(s, "{head}\t{field}\tknown\t{value}\t{vsyn}\t{q}\t{p}\t{rank}\t{steps}\t{}\t{confirm}\t{nums}\t{}", f[6], note.replace('\t', " "));
            }
        }
    }
    std::fs::write(out, &s)?;
    let mut text = format!(
        "core: {} concepts × {} fields = {} cells: known {n_known}, unknown {n_unknown}, not applicable {n_na}; values {n_vals}\n",
        core.len(),
        FIELDS.len(),
        core.len() * FIELDS.len()
    );
    text.push_str("| field | known | unknown | not applicable |\n|---|---:|---:|---:|\n");
    for (f, v) in &per_field {
        let _ = writeln!(text, "| {f} | {} | {} | {} |", v[0], v[1], v[2]);
    }
    Ok(SelReport { text })
}
