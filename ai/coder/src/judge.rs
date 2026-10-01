//! Blind Opus judge: fixed rubric (correctness, completeness, style, explanation — 1–5; "pass" prediction).
//! The judge sees neither the round number nor the architecture version: items get random keys and are shuffled across
//! tasks. An identical (task, program, explanation) triple is not judged again — the verdict comes from the cache.

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::tasks::Task;
use crate::util::{Mix, read_jsonl, run_dir, sha256_hex};
use crate::v0::Solution;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Verdict {
    pub k: String,
    pub correct: u8,
    pub complete: u8,
    pub style: u8,
    pub explain: u8,
    pub pass: bool,
    pub why: String,
}

/// Cache: hash of (task, program, explanation) → verdict.
pub fn cache_path() -> PathBuf {
    run_dir().join("judge-cache.jsonl")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cached {
    pub h: String,
    pub v: Verdict,
}

pub fn item_hash(task: &str, s: &Solution) -> String {
    sha256_hex(format!("{task}\0{}\0{}", s.program, s.explanation).as_bytes())
}

pub fn load_cache() -> Result<HashMap<String, Verdict>> {
    Ok(read_jsonl::<Cached>(&cache_path())?.into_iter().map(|c| (c.h, c.v)).collect())
}

fn trunc(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else { s.chars().take(n).collect::<String>() + " …" }
}

/// Batch prompts for items without a cached verdict. Returns (batches: (hash keys, text)).
pub fn batches(head: &str, items: &[(&Task, &Solution)], cache: &HashMap<String, Verdict>, size: usize, seed: u64) -> Vec<(Vec<String>, String)> {
    batches_opt(head, items, cache, size, seed, true)
}

/// `with_ref = false` — without the reference and the explanation (judge control on mutants).
pub fn batches_opt(head: &str, items: &[(&Task, &Solution)], cache: &HashMap<String, Verdict>, size: usize, seed: u64, with_ref: bool) -> Vec<(Vec<String>, String)> {
    let mut todo: Vec<(String, &Task, &Solution)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (t, s) in items {
        let h = item_hash(&t.id, s);
        if cache.contains_key(&h) || !seen.insert(h.clone()) {
            continue;
        }
        todo.push((h, t, s));
    }
    let mut mix = Mix(seed);
    mix.shuffle(&mut todo);
    todo.chunks(size)
        .map(|ch| {
            let mut text = head.to_string();
            let mut keys = Vec::new();
            for (h, t, s) in ch {
                let key = h[..10].to_string();
                keys.push(h.clone());
                if !with_ref {
                    text.push_str(&format!(
                        "\n=== ITEM k={key}\nTASK: {}\nEXPECTED OUTPUT (exact):\n{}\nCANDIDATE PROGRAM:\n{}\n",
                        t.description,
                        t.expected.iter().map(|l| format!("> {l}")).collect::<Vec<_>>().join("\n"),
                        trunc(&s.program, 3000)
                    ));
                    continue;
                }
                text.push_str(&format!(
                    "\n=== ITEM k={key}\nTASK: {}\nREFERENCE SOLUTION:\n{}EXPECTED OUTPUT (exact):\n{}\nCANDIDATE PROGRAM:\n{}\nCANDIDATE EXPLANATION:\n{}\n",
                    t.description,
                    t.solution,
                    t.expected.iter().map(|l| format!("> {l}")).collect::<Vec<_>>().join("\n"),
                    trunc(&s.program, 3000),
                    trunc(&s.explanation, 1500)
                ));
            }
            (keys, text)
        })
        .collect()
}

pub fn parse(text: &str) -> Vec<Verdict> {
    text.lines().filter_map(|l| serde_json::from_str::<Verdict>(l.trim()).ok()).collect()
}
