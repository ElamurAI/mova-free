//! The one shared list of hard cases: sentences Mova reads badly, found by any corpus reader (the
//! Wikipedia corpus's hourly check, the Owls' topic corpora), kept until the parser learns them.
//!
//! One append-only JSONL file, one line per event, written under an exclusive `flock` and synced.
//! The format is `mova-hard-case-v2`; older `mova-hard-case-v1` lines (the topic corpus's first
//! format) are read as v2 with an unknown licence.
//!
//! ```text
//! {"schema":"mova-hard-case-v2","event":"found","key":K,"parser":"en","text":…,"source":"wikipedia",
//!  "id":"wiki:Accra:12","document":{"id":"wiki:Accra","n":12},"where":{…},"why":["witness"],
//!  "licence":"CC BY-SA 4.0","model":"en.bin:31a9…","reader":"run-1",
//!  "draft_conllu":"…","witness_uas":0.45,"review_only":true,"training_authorized":false}
//! {"schema":"mova-hard-case-v2","event":"learned","key":K,"parser":"gparse","model":"gparse:942e…","reader":"recheck"}
//! {"schema":"mova-hard-case-v2","event":"open","key":K,"parser":"gparse","model":"gparse:…","reader":"recheck","why":[…]}
//! ```
//!
//! `key` is the first 16 hex digits of the SHA-256 of the trimmed sentence text, so one sentence met
//! in two corpora is one case. `parser` names whose reading the event is about: `en` (the CPU model
//! `en.bin`, which the corpora read with) or `gparse` (the GPU parser the Eel trains). A case has one
//! status per parser: `open` from its first `found`, `learned` after a `learned` event, open again
//! after a later `found` or `open`. A parser that never judged a case leaves it `unchecked`.
//!
//! Nothing here admits anything to training (`review_only`, `training_authorized: false`).
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::Path;

pub const SCHEMA: &str = "mova-hard-case-v2";
pub const SCHEMA_V1: &str = "mova-hard-case-v1";
/// The parser the corpora read with (`en.bin`).
pub const EN: &str = "en";
/// The GPU parser the Eel trains.
pub const GPARSE: &str = "gparse";

/// The case key: the first 16 hex digits of the SHA-256 of the trimmed text.
pub fn key(text: &str) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(text.trim().as_bytes()).iter().take(8).map(|x| format!("{x:02x}")).collect()
}

/// Where the sentence's document context is: the document's id in its source and the sentence's
/// position in it (1-based for Wikipedia, the reader's index for topic corpora).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub id: String,
    pub n: usize,
}

/// One found hard sentence, as a source reports it.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Found {
    pub text: String,
    /// `wikipedia` or `topic-corpus` (or another reader's name).
    pub source: String,
    /// The source's own sentence id (`wiki:<Title>:<n>`, `<doc>:<s>`).
    pub id: String,
    pub document: Document,
    /// The source's locator (article, page id, revision, dump offset; corpus, doc, sentence).
    #[serde(rename = "where")]
    pub at: Value,
    /// Why it is hard: the source's judges (`witness`, `grammar`, `nonprojective`, `parse`, `tree`,
    /// `frames`, `roots`, `head_outside`, `no_words`, `no_frames`, `no_skeleton`).
    pub why: Vec<String>,
    /// The licence of the text as the source states it; empty when unknown.
    #[serde(default)]
    pub licence: String,
    /// The model that read it badly.
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub reader: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_conllu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub witness_uas: Option<f64>,
}

/// A parser's view of a case.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct View {
    /// `open`, `learned` or `unchecked`.
    pub status: String,
    /// The model of the last event about this parser (empty for v1 lines).
    pub model: String,
}

/// The state of one case: its latest `found` record and one view per parser.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Case {
    pub key: String,
    pub found: Found,
    /// Parser name → view. A parser absent here has not judged the case.
    pub views: BTreeMap<String, View>,
    /// How many `found` events the case has (the same sentence met again).
    pub times_found: usize,
}

impl Case {
    pub fn status(&self, parser: &str) -> &str {
        self.views.get(parser).map_or("unchecked", |v| v.status.as_str())
    }
    /// Open for a parser: open, or never judged by it while its `en` reading is open.
    pub fn open_for(&self, parser: &str) -> bool {
        match self.status(parser) {
            "open" => true,
            "learned" => false,
            _ => self.status(EN) == "open",
        }
    }
}

/// A v1 line as a v2 found record (the topic corpus's first format: no licence, no model).
fn from_v1(v: &Value) -> Option<Found> {
    let at = v.get("where").cloned().unwrap_or(Value::Null);
    let doc = at["doc"].as_str().unwrap_or("").to_string();
    let s = at["s"].as_u64().unwrap_or(0) as usize;
    Some(Found {
        text: v["text"].as_str()?.to_string(),
        source: v["source"].as_str().unwrap_or("topic-corpus").to_string(),
        id: format!("{doc}:{s}"),
        document: Document { id: doc, n: s },
        at,
        why: v["why"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default(),
        licence: String::new(),
        model: String::new(),
        reader: v["reader"].as_str().unwrap_or("").to_string(),
        draft_conllu: None,
        witness_uas: None,
    })
}

/// Parses one line into (key, event, parser, model, found).
fn event(l: &str) -> Option<(String, String, String, String, Option<Found>)> {
    let v: Value = serde_json::from_str(l).ok()?;
    let k = v["key"].as_str()?.to_string();
    let e = v["event"].as_str()?.to_string();
    let schema = v["schema"].as_str().unwrap_or("");
    let parser = v["parser"].as_str().unwrap_or(EN).to_string();
    let model = v["model"].as_str().unwrap_or("").to_string();
    let found = if e == "found" {
        if schema == SCHEMA_V1 {
            from_v1(&v)
        } else {
            serde_json::from_value::<Found>(v.clone()).ok()
        }
    } else {
        None
    };
    Some((k, e, parser, model, found))
}

/// The state of every case, by key, from the whole list (missing file: empty).
pub fn state(path: &Path) -> Result<BTreeMap<String, Case>> {
    let mut m: BTreeMap<String, Case> = BTreeMap::new();
    if !path.exists() {
        return Ok(m);
    }
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    for l in text.lines().filter(|l| !l.trim().is_empty()) {
        let Some((k, e, parser, model, found)) = event(l) else { continue };
        match (e.as_str(), found) {
            ("found", Some(f)) => {
                let c = m.entry(k.clone()).or_insert_with(|| Case { key: k.clone(), found: f.clone(), views: BTreeMap::new(), times_found: 0 });
                c.found = f;
                c.times_found += 1;
                c.views.insert(parser, View { status: "open".into(), model });
            }
            ("learned" | "open", _) => {
                if let Some(c) = m.get_mut(&k) {
                    c.views.insert(parser, View { status: if e == "open" { "open".into() } else { "learned".into() }, model });
                }
            }
            _ => {}
        }
    }
    Ok(m)
}

/// Appends lines under an exclusive lock, synced. Creates the file (0600) and its directory.
pub fn append(path: &Path, lines: &[Value]) -> Result<()> {
    if lines.is_empty() {
        return Ok(());
    }
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(path)?;
    // One writer at a time across all the readers sharing the list.
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let s: String = lines.iter().map(|v| v.to_string() + "\n").collect();
    f.write_all(s.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

/// The `found` event line of a record.
pub fn found_line(f: &Found, parser: &str) -> Result<Value> {
    let mut v = serde_json::to_value(f)?;
    let o = v.as_object_mut().context("found record")?;
    o.insert("schema".into(), json!(SCHEMA));
    o.insert("event".into(), json!("found"));
    o.insert("key".into(), json!(key(&f.text)));
    o.insert("parser".into(), json!(parser));
    o.insert("review_only".into(), json!(true));
    o.insert("training_authorized".into(), json!(false));
    Ok(v)
}

/// Already open for the parser, and nothing to add: a found record that brings a licence the open
/// case lacks (a v1 line) is written again, so the case gets it.
fn known(c: &Case, f: &Found, parser: &str) -> bool {
    c.status(parser) == "open" && !(c.found.licence.is_empty() && !f.licence.is_empty())
}

/// Records found cases for `parser` that are not already open for it (one line per new key).
/// Returns how many lines were written.
pub fn record(path: &Path, found: &[Found], parser: &str) -> Result<usize> {
    let st = state(path)?;
    let mut seen = BTreeSet::new();
    let mut out = vec![];
    for f in found {
        let mut f = f.clone();
        f.text = f.text.trim().to_string();
        let k = key(&f.text);
        if f.text.is_empty() || st.get(&k).is_some_and(|c| known(c, &f, parser)) || !seen.insert(k) {
            continue;
        }
        out.push(found_line(&f, parser)?);
    }
    append(path, &out)?;
    Ok(out.len())
}

/// A `learned` or `open` event line.
pub fn verdict_line(k: &str, learned: bool, parser: &str, model: &str, reader: &str, why: &[String]) -> Value {
    let mut v = json!({"schema": SCHEMA, "event": if learned { "learned" } else { "open" }, "key": k, "parser": parser, "model": model, "reader": reader});
    if !learned && !why.is_empty() {
        v["why"] = json!(why);
    }
    v
}

/// Synchronizes one source's current set of hard sentences with the list, for `parser`: records
/// the new ones as found, and records `learned` for the cases this source found that are open for
/// the parser and no longer in the set. Cases another source found are left alone.
/// Returns (found written, learned written).
pub fn sync_source(path: &Path, source: &str, current: &[Found], parser: &str, model: &str, reader: &str) -> Result<(usize, usize)> {
    let st = state(path)?;
    let now: BTreeSet<String> = current.iter().map(|f| key(&f.text)).collect();
    let mut seen = BTreeSet::new();
    let mut lines = vec![];
    let mut found = 0;
    for f in current {
        let mut f = f.clone();
        f.text = f.text.trim().to_string();
        let k = key(&f.text);
        if f.text.is_empty() || !seen.insert(k.clone()) || st.get(&k).is_some_and(|c| known(c, &f, parser)) {
            continue;
        }
        lines.push(found_line(&f, parser)?);
        found += 1;
    }
    let mut learned = 0;
    for (k, c) in &st {
        if c.found.source == source && c.status(parser) == "open" && !now.contains(k) {
            lines.push(verdict_line(k, true, parser, model, reader, &[]));
            learned += 1;
        }
    }
    append(path, &lines)?;
    Ok((found, learned))
}

/// Re-judges the cases open for `parser` (only those `source` found, when given) with `check`
/// (cases → per case: reasons, empty when read well; `None` when it could not judge the case, which
/// then stays as it was) and records `learned` for those read well.
/// With `all`, the cases already learned are judged too and recorded `open` again when they fail (a
/// regression). Returns (judged, learned, reopened).
pub fn recheck(path: &Path, source: Option<&str>, parser: &str, model: &str, reader: &str, all: bool, check: &mut dyn FnMut(&[&Case]) -> Result<Vec<Option<Vec<String>>>>) -> Result<(usize, usize, usize)> {
    let st = state(path)?;
    let todo: Vec<&Case> = st
        .values()
        .filter(|c| source.is_none_or(|s| c.found.source == s))
        .filter(|c| c.open_for(parser) || (all && c.status(parser) == "learned"))
        .collect();
    if todo.is_empty() {
        return Ok((0, 0, 0));
    }
    let verdicts = check(&todo)?;
    anyhow::ensure!(verdicts.len() == todo.len(), "recheck: one verdict per case");
    let mut lines = vec![];
    let (mut learned, mut reopened) = (0, 0);
    let mut judged = 0;
    for (c, why) in todo.iter().zip(&verdicts) {
        let Some(why) = why else { continue };
        judged += 1;
        let was = c.status(parser);
        if why.is_empty() && was != "learned" {
            lines.push(verdict_line(&c.key, true, parser, model, reader, &[]));
            learned += 1;
        } else if !why.is_empty() && was != "open" {
            lines.push(verdict_line(&c.key, false, parser, model, reader, why));
            reopened += usize::from(was == "learned");
        }
    }
    append(path, &lines)?;
    Ok((judged, learned, reopened))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("hard-case-list-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn f(text: &str, source: &str, n: usize) -> Found {
        Found { text: text.into(), source: source.into(), id: format!("d:{n}"), document: Document { id: "d".into(), n }, at: json!({}), why: vec!["witness".into()], licence: "CC BY-SA 4.0".into(), model: "en.bin:x".into(), reader: "r".into(), draft_conllu: None, witness_uas: Some(0.5) }
    }

    #[test]
    fn v1_lines_read_as_v2_and_views_are_per_parser() {
        let d = tmp("v1");
        let p = d.join("list.jsonl");
        std::fs::write(&p, r#"{"event":"found","key":"8ec2a74d6618e071","reader":"topic-reader-3","schema":"mova-hard-case-v1","source":"topic-corpus","text":"[iss → is]: spelling","where":{"corpus":"c","doc":"lib:x","s":35},"why":["no_skeleton"]}
"#).unwrap();
        let st = state(&p).unwrap();
        let c = &st["8ec2a74d6618e071"];
        assert_eq!(c.found.id, "lib:x:35");
        assert_eq!(c.found.document, Document { id: "lib:x".into(), n: 35 });
        assert_eq!(c.found.licence, "");
        assert_eq!(c.status(EN), "open");
        assert_eq!(c.status(GPARSE), "unchecked");
        assert!(c.open_for(GPARSE));
        // gparse learns it; en still open
        append(&p, &[verdict_line(&c.key, true, GPARSE, "gparse:a", "t", &[])]).unwrap();
        let st = state(&p).unwrap();
        assert_eq!(st["8ec2a74d6618e071"].status(GPARSE), "learned");
        assert!(!st["8ec2a74d6618e071"].open_for(GPARSE));
        assert!(st["8ec2a74d6618e071"].open_for(EN));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn record_dedupes_and_sync_learns_only_its_own_source() {
        let d = tmp("sync");
        let p = d.join("list.jsonl");
        assert_eq!(record(&p, &[f("A b c.", "wikipedia", 1), f(" A b c. ", "wikipedia", 2)], EN).unwrap(), 1);
        assert_eq!(record(&p, &[f("A b c.", "wikipedia", 1)], EN).unwrap(), 0);
        assert_eq!(record(&p, &[f("Other one.", "topic-corpus", 3)], EN).unwrap(), 1);
        // The Wikipedia set no longer has «A b c.» and gains «New.»: one learned, one found;
        // the topic corpus's case stays open.
        assert_eq!(sync_source(&p, "wikipedia", &[f("New.", "wikipedia", 4)], EN, "en.bin:y", "sync").unwrap(), (1, 1));
        let st = state(&p).unwrap();
        assert_eq!(st[&key("A b c.")].status(EN), "learned");
        assert_eq!(st[&key("Other one.")].status(EN), "open");
        assert_eq!(st[&key("New.")].status(EN), "open");
        // An exact repeat writes nothing.
        assert_eq!(sync_source(&p, "wikipedia", &[f("New.", "wikipedia", 4)], EN, "en.bin:y", "sync").unwrap(), (0, 0));
        // A v1-like case without licence gets it from a later report.
        let mut nl = f("Other one.", "topic-corpus", 3);
        nl.licence = String::new();
        let p2 = d.join("l2.jsonl");
        assert_eq!(record(&p2, &[nl.clone()], EN).unwrap(), 1);
        assert_eq!(record(&p2, &[nl], EN).unwrap(), 0);
        assert_eq!(record(&p2, &[f("Other one.", "topic-corpus", 3)], EN).unwrap(), 1);
        assert_eq!(state(&p2).unwrap()[&key("Other one.")].found.licence, "CC BY-SA 4.0");
        // Found again after learned: open again.
        assert_eq!(record(&p, &[f("A b c.", "wikipedia", 1)], EN).unwrap(), 1);
        assert_eq!(state(&p).unwrap()[&key("A b c.")].status(EN), "open");
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn recheck_learns_and_reopens_regressions() {
        let d = tmp("recheck");
        let p = d.join("list.jsonl");
        record(&p, &[f("One.", "wikipedia", 1), f("Two.", "wikipedia", 2)], EN).unwrap();
        // gparse reads «One.» well, «Two.» badly.
        let (n, l, r) = recheck(&p, None, GPARSE, "gparse:a", "t", false, &mut |cs| Ok(cs.iter().map(|c| Some(if c.found.text == "One." { vec![] } else { vec!["grammar".to_string()] })).collect())).unwrap();
        assert_eq!((n, l, r), (2, 1, 0));
        let st = state(&p).unwrap();
        assert_eq!(st[&key("One.")].status(GPARSE), "learned");
        assert_eq!(st[&key("Two.")].status(GPARSE), "open");
        // A later model reads «One.» badly again: with `all`, it is reopened (negative control).
        let (n, l, r) = recheck(&p, None, GPARSE, "gparse:b", "t", true, &mut |cs| Ok(cs.iter().map(|_| Some(vec!["witness".to_string()])).collect())).unwrap();
        assert_eq!((n, l, r), (2, 0, 1));
        assert_eq!(state(&p).unwrap()[&key("One.")].status(GPARSE), "open");
        // en's view never moved.
        assert_eq!(state(&p).unwrap()[&key("One.")].status(EN), "open");
        // A source filter judges only that source's cases.
        record(&p, &[f("Three.", "topic-corpus", 3)], EN).unwrap();
        let (n, l, _) = recheck(&p, Some("topic-corpus"), EN, "topic-reader-3", "t", false, &mut |cs| Ok(cs.iter().map(|_| Some(vec![])).collect())).unwrap();
        assert_eq!((n, l), (1, 1));
        // A case the check cannot judge stays as it was (no `open` written for an unchecked parser).
        record(&p, &[f("Four.", "topic-corpus", 4)], EN).unwrap();
        let before = std::fs::read_to_string(&p).unwrap();
        assert_eq!(recheck(&p, Some("topic-corpus"), GPARSE, "g", "t", false, &mut |cs| Ok(cs.iter().map(|_| None).collect())).unwrap(), (0, 0, 0));
        assert_eq!(std::fs::read_to_string(&p).unwrap(), before);
        assert_eq!(state(&p).unwrap()[&key("Four.")].status(GPARSE), "unchecked");
        assert_eq!(state(&p).unwrap()[&key("One.")].status(EN), "open");
        let _ = std::fs::remove_dir_all(d);
    }
}
