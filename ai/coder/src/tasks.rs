//! Tasks: ~100 from Opus, ~50 adapted from exercises in open books. The reference (solution and output) is seen only by
//! task-set verification, the judge and the control; the MMM gets `Public` — the description without the reference.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::util::{Mix, crate_dir, read_jsonl, run_dir, sha256_hex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub skill: String,
    pub level: u8,
    /// opus | erickson | ods | sicp
    pub source: String,
    /// "Exercise 1.11, pdf p. 53" for books
    #[serde(default)]
    pub source_ref: String,
    pub license: String,
    pub description: String,
    pub solution: String,
    pub expected: Vec<String>,
    /// train | held (after freezing)
    #[serde(default)]
    pub split: String,
}

/// What the MMM sees: no reference.
#[derive(Clone, Debug)]
pub struct Public {
    pub id: String,
    pub skill: String,
    pub level: u8,
    pub description: String,
    pub split: String,
}

impl Task {
    pub fn public(&self) -> Public {
        Public { id: self.id.clone(), skill: self.skill.clone(), level: self.level, description: self.description.clone(), split: self.split.clone() }
    }
}

pub const LIC_OPUS: &str = "Opus (claude-opus-5-5), written for Mova 26.09.2026";
pub const LIC_ERICKSON: &str = "CC BY 4.0 — Jeff Erickson, «Algorithms» (2019), jeffe.cs.illinois.edu/teaching/algorithms; adapted";
pub const LIC_ODS: &str = "CC BY — Pat Morin, «Open Data Structures» (pseudocode ed. 0.1Gβ), opendatastructures.org; adapted, contains material from opendatastructures.org";
pub const LIC_SICP: &str = "CC BY-SA 4.0 — Abelson, Sussman, Sussman, «Structure and Interpretation of Computer Programs», 2nd ed. (mitpress.mit.edu); adapted, under the same license";

/// Parsing an Opus response: blocks `### ID | SKILL | LEVEL [| SOURCE: …]`, `DESCRIPTION:`, `SOLUTION:`, `EXPECTED:`.
pub fn parse_blocks(text: &str, source: &str, license: &str) -> Vec<Task> {
    let mut out = Vec::new();
    let mut cur: Option<Task> = None;
    #[derive(PartialEq)]
    enum Sec {
        None,
        Desc,
        Sol,
        Exp,
    }
    let mut sec = Sec::None;
    let mut sol: Vec<String> = Vec::new();
    let finish = |cur: &mut Option<Task>, sol: &mut Vec<String>, out: &mut Vec<Task>| {
        if let Some(mut t) = cur.take() {
            while sol.last().is_some_and(|l| l.trim().is_empty()) {
                sol.pop();
            }
            t.solution = sol.join("\n") + "\n";
            t.description = t.description.trim().to_string();
            if !t.description.is_empty() && !t.solution.trim().is_empty() {
                out.push(t);
            }
        }
        sol.clear();
    };
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("### ") {
            finish(&mut cur, &mut sol, &mut out);
            let parts: Vec<&str> = h.split('|').map(str::trim).collect();
            let src_ref = parts.iter().find_map(|p| p.strip_prefix("SOURCE:")).map(|s| s.trim().to_string()).unwrap_or_default();
            cur = Some(Task {
                id: parts.first().unwrap_or(&"").to_string(),
                skill: parts.get(1).unwrap_or(&"").to_lowercase(),
                level: parts.get(2).and_then(|s| s.chars().next()).and_then(|c| c.to_digit(10)).unwrap_or(1) as u8,
                source: source.to_string(),
                source_ref: src_ref,
                license: license.to_string(),
                description: String::new(),
                solution: String::new(),
                expected: Vec::new(),
                split: String::new(),
            });
            sec = Sec::None;
            continue;
        }
        let Some(t) = cur.as_mut() else { continue };
        if let Some(d) = line.strip_prefix("DESCRIPTION:") {
            t.description = d.trim().to_string();
            sec = Sec::Desc;
            continue;
        }
        if line.trim() == "SOLUTION:" {
            sec = Sec::Sol;
            continue;
        }
        if line.trim() == "EXPECTED:" {
            sec = Sec::Exp;
            continue;
        }
        match sec {
            Sec::Desc => {
                if !line.trim().is_empty() {
                    t.description.push(' ');
                    t.description.push_str(line.trim());
                }
            }
            Sec::Sol => sol.push(line.to_string()),
            Sec::Exp => {
                if line == ">" {
                    t.expected.push(String::new());
                } else if let Some(p) = line.strip_prefix("> ") {
                    t.expected.push(p.to_string());
                }
            }
            Sec::None => {}
        }
    }
    finish(&mut cur, &mut sol, &mut out);
    out
}

pub fn tasks_path() -> PathBuf {
    crate_dir().join("data/tasks.jsonl")
}

/// Frozen hash of `data/tasks.jsonl` (after `coder freeze`); test `tasks_are_frozen`.
pub const FROZEN_TASKS_SHA256: &str = "bf7d42755a376d9800b6b3f68f4bab7ede5107b00af1a07e7b8f9c5724d88049";

pub fn load() -> Result<Vec<Task>> {
    let p = tasks_path();
    let t: Vec<Task> = read_jsonl(&p)?;
    if t.is_empty() {
        bail!("no tasks: {}", p.display());
    }
    Ok(t)
}

pub fn file_sha(p: &Path) -> Result<String> {
    Ok(sha256_hex(&std::fs::read(p)?))
}

/// Split: stratified by source, seed 20260926, ~2/3 for training.
pub fn split(tasks: &mut [Task], n_held: usize) {
    let mut by_src: std::collections::BTreeMap<String, Vec<usize>> = Default::default();
    for (i, t) in tasks.iter().enumerate() {
        by_src.entry(t.source.clone()).or_default().push(i);
    }
    let total = tasks.len();
    let mut mix = Mix(20260926);
    for (_, mut idx) in by_src {
        mix.shuffle(&mut idx);
        let k = ((idx.len() * n_held) as f64 / total as f64).round() as usize;
        for (j, &i) in idx.iter().enumerate() {
            tasks[i].split = if j < k { "held".into() } else { "train".into() };
        }
    }
}

/// Raw generation responses — in the run.
pub fn gen_dir() -> PathBuf {
    run_dir().join("gen")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tasks_are_frozen() {
        assert_eq!(file_sha(&tasks_path()).unwrap(), FROZEN_TASKS_SHA256, "data/tasks.jsonl changed after freezing");
        let t = load().unwrap();
        assert_eq!(t.len(), 150);
        assert_eq!(t.iter().filter(|t| t.split == "held").count(), 50);
    }

    #[test]
    fn sha256_self_check() {
        assert_eq!(crate::util::sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
}
