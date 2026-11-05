//! Review hints: a Whale's own terminal reads the Whale's current description next to the parts
//! of the paper it describes and says what to do better — what is missing, what is wrong or
//! unclear, how to rephrase or present it. The hints are working materials of that paper, kept in
//! its state folder; they guide the Whale's next rounds (which terms and sentences to ask about
//! first, which parts need more), and their text never enters a description: every hint answer is
//! kept with the terminal's answers, so the answer-overlap gate refuses any copy of it.
//!
//! State model (per paper, `papers/<id>/hints/`):
//!
//! - `index.jsonl` — one line per review: `{description_sha256, paper_sha256, round, requests,
//!   hints, file}`. A (description, paper) pair is reviewed at most once: a cached review is read
//!   from its file, never asked again.
//! - `round-NNN.json` — the parsed hints `[{kind, part, text}]` with the request and answer hashes;
//!   `round-NNN.md` — the same, readable.
//!
//! A request carries the description and the text of the parts it describes, at most
//! `max_bytes` (100,000 by default) per request: parts that do not fit go to further requests.
//! Answer lines are read in the shape the request asks for: `MISSING: part | what`, `WRONG:
//! quote | why`, `UNCLEAR: quote | why`, `REPHRASE: part | how`, `ORDER: advice`.
use crate::reader::Reading;
use crate::util::{hash, now, save, write_sync};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;

/// When and how a Whale asks for review hints.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReviewConfig {
    /// Ask for hints at all.
    #[serde(default = "d_on")]
    pub enabled: bool,
    /// A round that adds fewer words than this over the best earlier draft asks for hints.
    #[serde(default = "d_gain")]
    pub min_gain_words: usize,
    /// The bound of one request in UTF-8 bytes, wrappers included.
    #[serde(default = "d_bytes")]
    pub max_bytes: usize,
}
fn d_on() -> bool {
    true
}
fn d_gain() -> usize {
    150
}
fn d_bytes() -> usize {
    100_000
}
impl Default for ReviewConfig {
    fn default() -> Self {
        Self { enabled: d_on(), min_gain_words: d_gain(), max_bytes: d_bytes() }
    }
}
impl ReviewConfig {
    pub fn validate(&self) -> Result<()> {
        if !(20_000..=100_000).contains(&self.max_bytes) {
            return Err(crate::util::error("review: max_bytes 20000..100000"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Hint {
    /// `missing`, `wrong`, `unclear`, `rephrase` or `order`.
    pub kind: String,
    /// The part or quote the hint is about (empty for `order`).
    pub part: String,
    pub text: String,
}

const FRAME: &str = "I am a Whale of Mova. Below is my own description of a scientific paper, written from my own reading, and the text of the parts of the paper it describes. Review my description against the paper and tell me how to improve it. Do not write the description for me and do not rewrite its sentences. Answer only with lines in these shapes, one hint per line, at most 20 lines:\nMISSING: <part title> | <which claim, method or result of that part my description leaves out>\nWRONG: <a short quote from my description> | <why it misstates the paper>\nUNCLEAR: <a short quote from my description> | <what is unclear>\nREPHRASE: <part title> | <how to present that part better: order, transitions, emphasis>\nORDER: <how to order the description as a whole>\n";

/// The requests of one review: each within `max_bytes`, each with the whole description and as
/// many described parts as fit (in paper order). Parts too long for any request are cut at a
/// sentence. None when the description alone does not fit.
pub fn requests(r: &Reading, description: &str, parts: &[usize], max_bytes: usize, part_no: &str) -> Option<Vec<String>> {
    let head = format!("{FRAME}\n=== MY DESCRIPTION ===\n{description}\n\n=== THE PAPER{part_no} ===\n");
    if head.len() + 2000 > max_bytes {
        return None;
    }
    let room = max_bytes - head.len() - 64;
    let mut chunks: Vec<String> = vec![];
    let mut cur = String::new();
    for &p in parts {
        let s = &r.sections[p];
        let mut body = format!("## {}\n", s.title);
        for &i in &s.sentences {
            let t = &r.sentences[i].text;
            if body.len() + t.len() + 1 > room {
                break;
            }
            body.push_str(t);
            body.push(' ');
        }
        body.push('\n');
        if !cur.is_empty() && cur.len() + body.len() > room {
            chunks.push(std::mem::take(&mut cur));
        }
        cur.push_str(&body);
    }
    if !cur.is_empty() {
        chunks.push(cur);
    }
    let n = chunks.len();
    Some(chunks.into_iter().enumerate().map(|(k, c)| format!("{}{c}", head.replace(part_no, &if n > 1 { format!(" (request {} of {n})", k + 1) } else { String::new() }))).collect())
}

/// Hints in an answer: lines `KIND: part | text` (`ORDER: text`); other lines are ignored.
pub fn parse(answer: &str) -> Vec<Hint> {
    let mut out = vec![];
    for line in answer.lines() {
        let l = line.trim().trim_start_matches(['-', '*', ' ']);
        let Some((k, rest)) = l.split_once(':') else { continue };
        let kind = k.trim().to_lowercase();
        if !matches!(kind.as_str(), "missing" | "wrong" | "unclear" | "rephrase" | "order") {
            continue;
        }
        let (part, text) = match rest.split_once('|') {
            Some((p, t)) if kind != "order" => (p.trim().trim_matches(['«', '»', '"', '“', '”']).to_string(), t.trim().to_string()),
            _ => (String::new(), rest.trim().to_string()),
        };
        if !text.is_empty() {
            let h = Hint { kind, part, text };
            if !out.contains(&h) {
                out.push(h);
            }
        }
    }
    out.truncate(40);
    out
}

/// A cached review of this (description, paper) pair, if one was made.
pub fn cached(dir: &Path, description_sha: &str, paper_sha: &str) -> Result<Option<Value>> {
    let p = dir.join("index.jsonl");
    if !p.exists() {
        return Ok(None);
    }
    for l in std::fs::read_to_string(p)?.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(l)?;
        if v["description_sha256"] == description_sha && v["paper_sha256"] == paper_sha {
            return Ok(Some(v));
        }
    }
    Ok(None)
}

/// Store a review's hints (`round-NNN.json`, `.md`) and index it.
pub fn store(dir: &Path, round: usize, description_sha: &str, paper_sha: &str, hints: &[Hint], requests: &[String], answer_shas: &[String]) -> Result<Value> {
    crate::util::private_dir(dir)?;
    let name = format!("round-{round:03}");
    save(&dir.join(format!("{name}.json")), &json!({"round":round,"description_sha256":description_sha,"paper_sha256":paper_sha,"requests":requests.iter().map(|q| json!({"sha256":hash(q),"bytes":q.len()})).collect::<Vec<_>>(),"answers":answer_shas,"hints":hints}))?;
    let mut md = format!("# Review hints, round {round}\n\nDescription {description_sha}, paper {paper_sha}. Working material: guidance for the Whale's own writing, never text for the description.\n\n");
    for h in hints {
        md.push_str(&format!("- **{}**{}: {}\n", h.kind, if h.part.is_empty() { String::new() } else { format!(" «{}»", h.part) }, h.text));
    }
    write_sync(&dir.join(format!("{name}.md")), md.as_bytes())?;
    let v = json!({"description_sha256":description_sha,"paper_sha256":paper_sha,"round":round,"requests":requests.len(),"hints":hints.len(),"file":format!("{name}.json"),"unix":now()});
    let mut line = v.to_string();
    line.push('\n');
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("index.jsonl"))?;
    f.write_all(line.as_bytes())?;
    f.sync_data()?;
    Ok(v)
}

/// The hints of the latest review of this paper.
pub fn latest(dir: &Path) -> Result<Vec<Hint>> {
    let p = dir.join("index.jsonl");
    if !p.exists() {
        return Ok(vec![]);
    }
    let Some(last) = std::fs::read_to_string(&p)?.lines().filter(|l| !l.trim().is_empty()).last().map(String::from) else { return Ok(vec![]) };
    let v: Value = serde_json::from_str(&last)?;
    let f: Value = serde_json::from_slice(&std::fs::read(dir.join(v["file"].as_str().unwrap_or("")))?)?;
    Ok(serde_json::from_value(f["hints"].clone()).unwrap_or_default())
}

/// Words (lower case, three letters or more) the hints mention: the guidance the Whale can use is
/// which terms, sentences and parts to take up first.
pub fn focus(hints: &[Hint]) -> std::collections::BTreeSet<String> {
    hints.iter().filter(|h| matches!(h.kind.as_str(), "missing" | "wrong" | "unclear")).flat_map(|h| crate::reader::content(&format!("{} {}", h.part, h.text))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hints_are_parsed_bounded_and_cached() {
        let a = "Here are my hints.\nMISSING: Results | the UAS of 95.87 on PTB\n- WRONG: «This facilitates evaluation» | it is the framework, not the paper\nREPHRASE: Method | start with the stack, then the pointer\nORDER: results after the method\nnot a hint\nMISSING: Results | the UAS of 95.87 on PTB";
        let h = parse(a);
        assert_eq!(h.len(), 4, "{h:?}");
        assert_eq!((h[0].kind.as_str(), h[0].part.as_str()), ("missing", "Results"));
        assert_eq!(h[3], Hint { kind: "order".into(), part: String::new(), text: "results after the method".into() });
        assert!(focus(&h).contains("uas"));
        let base = crate::learn::tests::tmp("review-hints");
        let d = base.join("hints");
        assert!(cached(&d, "d1", "p1").unwrap().is_none());
        store(&d, 3, "d1", "p1", &h, &["q".into()], &["a".into()]).unwrap();
        assert_eq!(cached(&d, "d1", "p1").unwrap().unwrap()["round"], 3);
        assert!(cached(&d, "d2", "p1").unwrap().is_none());
        assert_eq!(latest(&d).unwrap(), h);
        assert!(std::fs::read_to_string(d.join("round-003.md")).unwrap().contains("never text for the description"));
        let _ = std::fs::remove_dir_all(base);
    }
}
