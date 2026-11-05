//! Seed confirmation through the shared Codex teacher. Suggestions still pass the existing gates.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

/// Spent according to the log (JSON lines with `cost_usd`).
pub fn spent(log: &Path) -> f64 {
    std::fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter_map(|v| v["cost_usd"].as_f64())
        .sum()
}

/// One LLM call; returns the answer text and the cost.
pub fn ask(cwd: &PathBuf, log: &Path, cap: f64, reserve: f64, prompt: &str) -> Result<(String, f64)> {
    let s = spent(log);
    if s + reserve > cap {
        bail!("cap ${cap:.2}: spent ${s:.2}, reserve ${reserve:.2} — stopping");
    }
    std::fs::create_dir_all(cwd)?;
    let effort = std::env::var("ONTO_EFFORT").unwrap_or_else(|_| "medium".into());
    let t0 = std::time::Instant::now();
    let v = mova_teacher::ask(cwd, &mova_teacher::model("ONTO_MODEL"), &effort, prompt)?;
    let cost = v["total_cost_usd"].as_f64().unwrap_or(0.0);
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(log)?;
    writeln!(f, "{}", serde_json::json!({"step": "wd-confirm", "secs": t0.elapsed().as_secs_f64(), "cost_usd": cost, "input_tokens": v["usage"]["input_tokens"], "output_tokens": v["usage"]["output_tokens"]}))?;
    if v["is_error"].as_bool() == Some(true) {
        bail!("claude -p: {}", v["result"]);
    }
    Ok((v["result"].as_str().unwrap_or_default().to_string(), cost))
}

/// Candidates from `wd-candidates.tsv` for the LLM: object values with confidence in [lo, hi) — disputed 0.45–0.8 (inherited or with
/// a non-strict anchor) — up to `n`, evenly across properties. Returns the file's lines.
pub fn pick_disputed(tsv: &str, n: usize, lo: f64, hi: f64) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = tsv.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').map(String::from).collect()).filter(|f: &Vec<String>| f.len() >= 13 && f[7].is_empty()).collect();
    rows.retain(|f| f[6].parse::<f64>().map(|c| (lo..hi).contains(&c)).unwrap_or(false));
    // round-robin over properties — so that the sample is not all about parts
    let mut by: std::collections::BTreeMap<String, Vec<Vec<String>>> = Default::default();
    for f in rows {
        by.entry(f[3].clone()).or_default().push(f);
    }
    let mut out = Vec::new();
    let mut i = 0;
    while out.len() < n && by.values().any(|v| v.len() > i) {
        for v in by.values() {
            if let Some(f) = v.get(i) {
                if out.len() < n {
                    out.push(f.clone());
                }
            }
        }
        i += 1;
    }
    out
}

pub fn prompt(rows: &[Vec<String>]) -> String {
    let mut s = String::from(
        "You check candidate common-sense facts for a world model (real world, not fairy tales). Each line: id, concept (English lemma, WordNet sense), property, value (from Wikidata), where it came from.\n\
Properties: food = what it eats; habitat = where it lives; part = it has this part; no_part = it lacks this part; material = made of; color; use = what it is for.\n\
For EACH id answer one line exactly: id<TAB>yes|no|refine<TAB>why (max 12 words; for refine — the better value).\n\
Answer yes only if a typical instance of the concept has it (default knowledge); inherited from a too general class or wrong sense → no.\n\n",
    );
    for (i, f) in rows.iter().enumerate() {
        s.push_str(&format!("{}\t{} ({})\t{}\t{}\t{}\n", i + 1, f[0], f[2], f[3], f[4], f[5]));
    }
    s
}

/// Parse the answer: id → (verdict, why).
pub fn parse(ans: &str, n: usize) -> Vec<Option<(String, String)>> {
    let mut v = vec![None; n];
    for l in ans.lines() {
        let f: Vec<&str> = l.split('\t').map(str::trim).collect();
        if f.len() < 2 {
            continue;
        }
        let Ok(i) = f[0].trim_start_matches('#').parse::<usize>() else { continue };
        let verdict = match f[1].to_ascii_lowercase().as_str() {
            "yes" => "yes",
            "no" => "no",
            "refine" => "refine",
            _ => continue,
        };
        if (1..=n).contains(&i) {
            v[i - 1] = Some((verdict.to_string(), f.get(2).unwrap_or(&"").to_string()));
        }
    }
    v
}

#[cfg(test)]
mod t {
    #[test]
    fn parse_answer() {
        let v = super::parse("1\tyes\tcrows eat insects\n2\tNO\ttoo general\ngarbage\n3\trefine\tfeathers, not scales\n9\tyes\textra", 3);
        assert_eq!(v[0].as_ref().unwrap().0, "yes");
        assert_eq!(v[1].as_ref().unwrap().0, "no");
        assert_eq!(v[2].as_ref().unwrap().0, "refine");
    }
}

#[cfg(test)]
mod wave29_ontowiki {
    use super::*;
    #[test]
    fn parse_overwrite_and_budget_refusal() {
        assert_eq!(
            parse("1\tyes\ta\n1\tNO\tb\n0\tyes\nx\tno\n2\tmaybe", 2),
            vec![Some(("no".into(), "b".into())), None]
        );
        let dir = std::env::temp_dir().join(format!("onto-vmm-wave29-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let log = dir.join("log");
        std::fs::write(&log, "bad\n{\"cost_usd\":2.0}\n{\"cost_usd\":\"3\"}\n").unwrap();
        assert_eq!(spent(&log), 2.0);
        let cwd = dir.join("must-not-create");
        assert!(
            ask(&cwd, &log, 1.0, 0.0, "unused")
                .unwrap_err()
                .to_string()
                .starts_with("cap")
        );
        assert!(!cwd.exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
