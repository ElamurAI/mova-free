//! What a hard case may become.
//!
//! **Track** (by the licence of its text, `licence/DATA.md`):
//! - `shipped` — may train the model that ships (CC BY, CC0, public domain, our own texts);
//! - `internal` — may train only a separate internal model, never installed where the shipped model
//!   lives (by default CC BY-SA: the rules say «as little as possible», never shipped or carried into a
//!   release; the shipped `en.bin`/`gparse` are CC BY-SA through EWT today, but the operator has not
//!   decided whether that stays, and training them on Wikipedia would close the other way out);
//! - `measure` — never trained, kept for measurements (non-commercial, no-derivatives, research-only,
//!   unknown).
//!
//! **Split** (by its document, so no document is in two splits): `train` (default 70 %), `dev` (15 %,
//! the Eel's development gate) and `measure` (15 %, the fixed before/after sample, never used to
//! select a checkpoint).
//!
//! **Kinds**: the lane takes only cases flagged by syntax judges (by default all but `no_skeleton`,
//! which marks what the Whales' reader could not restate: mostly table cells and headings).
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const TRACKS: [&str; 3] = ["shipped", "internal", "measure"];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    /// Licence class → track. A class absent here is `measure`.
    #[serde(default = "tracks")]
    pub tracks: BTreeMap<String, String>,
    /// The judges whose cases the lane takes.
    #[serde(default = "kinds")]
    pub kinds: Vec<String>,
    /// Percent of documents in `train` and in `dev`; the rest is `measure`.
    #[serde(default = "train")]
    pub train_percent: u32,
    #[serde(default = "dev")]
    pub dev_percent: u32,
    /// Also send `measure`-track cases to the teacher (their annotations are measured against, never trained).
    #[serde(default)]
    pub annotate_measure_track: bool,
    /// Leave out texts that are markup the corpus's reader let through (wiki tables, templates,
    /// links, tags): a reading bug to fix in the reader, not a sentence to annotate.
    #[serde(default = "yes")]
    pub skip_markup: bool,
}

fn yes() -> bool {
    true
}

/// Markup the corpus's reader let through: wiki table rows, templates, links, tags, or three pipes.
pub fn markup(text: &str) -> bool {
    ["|-", "{|", "|}", "{{", "}}", "[[", "]]", "rowspan=", "colspan=", "<ref", "</"].iter().any(|m| text.contains(m)) || text.matches('|').count() >= 3
}

fn tracks() -> BTreeMap<String, String> {
    [("cc-by", "shipped"), ("cc0", "shipped"), ("public-domain", "shipped"), ("ours", "shipped"), ("cc-by-sa", "internal")]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}
fn kinds() -> Vec<String> {
    ["parse", "tree", "frames", "grammar", "nonprojective", "witness", "roots", "head_outside", "no_words", "no_frames"].into_iter().map(String::from).collect()
}
fn train() -> u32 {
    70
}
fn dev() -> u32 {
    15
}

impl Default for Policy {
    fn default() -> Self {
        Policy { tracks: tracks(), kinds: kinds(), train_percent: train(), dev_percent: dev(), annotate_measure_track: false, skip_markup: true }
    }
}

impl Policy {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.tracks.values().all(|t| TRACKS.contains(&t.as_str())), "policy.tracks: shipped, internal or measure");
        ensure!(self.train_percent + self.dev_percent <= 100 && self.dev_percent > 0, "policy: train + dev ≤ 100, dev > 0");
        Ok(())
    }

    pub fn track(&self, licence: &str) -> &str {
        self.tracks.get(class(licence)).map_or("measure", String::as_str)
    }

    pub fn admits(&self, why: &[String]) -> bool {
        why.iter().any(|w| self.kinds.contains(w))
    }

    pub fn split(&self, source: &str, document: &str) -> &'static str {
        let h = crate::sha(format!("{source}\0{document}"));
        let v = u32::from_str_radix(&h[..8], 16).unwrap_or(0) % 100;
        if v < self.train_percent {
            "train"
        } else if v < self.train_percent + self.dev_percent {
            "dev"
        } else {
            "measure"
        }
    }
}

/// The licence class of a stated licence: `cc-by`, `cc-by-sa`, `cc0`, `public-domain`, `ours`,
/// `restricted` (non-commercial, no-derivatives, research-only, no-reuse) or `unknown`.
pub fn class(licence: &str) -> &'static str {
    let l = licence.to_lowercase();
    let t: Vec<&str> = l.split(|c: char| !c.is_ascii_alphanumeric()).filter(|x| !x.is_empty()).collect();
    let has = |w: &str| t.contains(&w);
    if t.is_empty() {
        return "unknown";
    }
    if has("nc") || has("nd") || l.contains("noncommercial") || l.contains("non-commercial") || l.contains("noderiv") || l.contains("research") || l.contains("nonexclusive") {
        return "restricted";
    }
    if has("cc0") || l.contains("cc-zero") {
        return "cc0";
    }
    if l.contains("public domain") || l == "pd" {
        return "public-domain";
    }
    if l.contains("elamur") || l == "ours" {
        return "ours";
    }
    if (has("cc") || l.contains("creative commons")) && has("by") {
        return if has("sa") || l.contains("sharealike") { "cc-by-sa" } else { "cc-by" };
    }
    "unknown"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn licences_choose_tracks() {
        let p = Policy::default();
        assert_eq!(class("CC BY-SA 4.0"), "cc-by-sa");
        assert_eq!(p.track("CC BY-SA 4.0"), "internal");
        assert_eq!(p.track("CC BY 4.0"), "shipped");
        assert_eq!(p.track("CC0 1.0"), "shipped");
        assert_eq!(p.track("Public domain"), "shipped");
        // red: non-commercial, no-derivatives, research-only, unknown and empty never train
        for l in ["CC BY-NC-SA 3.0", "CC BY-ND 4.0", "research only", "http://arxiv.org/licenses/nonexclusive-distrib/1.0/", "", "all rights reserved"] {
            assert_eq!(p.track(l), "measure", "{l}");
        }
        // the operator's decision is one line of configuration
        let mut q = p.clone();
        q.tracks.insert("cc-by-sa".into(), "shipped".into());
        assert_eq!(q.track("CC BY-SA 4.0"), "shipped");
        q.tracks.insert("cc-by-sa".into(), "anywhere".into());
        assert!(q.validate().is_err());
    }

    #[test]
    fn splits_are_by_document_and_stable() {
        let p = Policy::default();
        let mut n = BTreeMap::new();
        for i in 0..2000 {
            *n.entry(p.split("wikipedia", &format!("wiki:A{i}"))).or_insert(0) += 1;
        }
        assert!(n["train"] > 1250 && n["dev"] > 200 && n["measure"] > 200, "{n:?}");
        assert_eq!(p.split("wikipedia", "wiki:Accra"), p.split("wikipedia", "wiki:Accra"));
        assert!(p.admits(&["witness".into()]));
        assert!(!p.admits(&["no_skeleton".into()]));
        assert!(markup("|- | 1983 | rowspan=4|Herself | British Female Solo Artist"));
        assert!(markup("see {{cite web}}"));
        assert!(!markup("Wilde released her debut single \"Kids in America\" in January 1981."));
        assert!(!markup("A | B"));
    }
}
