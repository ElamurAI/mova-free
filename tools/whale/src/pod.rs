//! A pod's shared knowledge: one store for all Whales of a pod, keyed by normalized term
//! (`knowledge::norm_term`), with the provenance of every entry (which Whale learned it, on which
//! paper, from which answer). A term one Whale has asked is never asked by another; what one Whale
//! learned, every Whale of the pod explains with.
//!
//! State model (the pod's private `dir`):
//!
//! - `terms.jsonl` — published term knowledge: `{term, display, whale, paper, item, facts,
//!   answer_sha256, unix}`; `item` is the id in the publishing Whale's own store, so
//!   `(whale, item)` is published once.
//! - `claims.jsonl` — `{term, whale, paper, state, unix}`: a term a Whale has asked (or is about to
//!   ask). The first claim wins; a claimed term is not asked by anyone else.
//! - `review.jsonl` — bare-category facts («Goldberg is a surname», «the label») flagged for
//!   review instead of being kept (`knowledge::bare`).
//! - `answers/SHA.txt` — copies of the answers behind published facts, so that every Whale's
//!   answer-overlap gate also covers answers other Whales received.
//! - `pod.lock` — taken (blocking `flock`) around every check-and-append.
//!
//! Learners of one pod run in parallel threads; the order of their appends depends on timing.
//! `items` is ordered by (term, whale, item), so what a Whale reads does not depend on the order of
//! the file, only on its content.
use crate::knowledge::{self, bare, norm_term, Fact, Item, Store};
use crate::util::{error, now, write_sync};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    /// The normalized term.
    pub term: String,
    /// The form the asking Whale's paper used.
    pub display: String,
    pub whale: String,
    pub paper: String,
    /// The item id in the publishing Whale's own store.
    pub item: String,
    pub facts: Vec<Fact>,
    #[serde(default)]
    pub answer_sha256: Option<String>,
    pub unix: u64,
}

pub struct Pod {
    pub dir: PathBuf,
    pub entries: Vec<Entry>,
    /// Normalized term -> its first claim.
    pub claims: BTreeMap<String, Value>,
    /// `whale:item` already published (with or without kept facts).
    published: BTreeSet<String>,
}

struct Lock(File);
impl Drop for Lock {
    fn drop(&mut self) {
        unsafe { libc::flock(self.0.as_raw_fd(), libc::LOCK_UN) };
    }
}

fn append(path: &Path, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(path)?;
    let mut line = v.to_string();
    line.push('\n');
    f.write_all(line.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

fn lines(path: &Path) -> Result<Vec<Value>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| error(format!("{}: {e}", path.display())))).collect()
}

impl Pod {
    pub fn open(dir: &Path) -> Result<Self> {
        if !dir.is_absolute() {
            return Err(error("pod: the shared knowledge directory must be absolute"));
        }
        crate::util::private_dir(dir)?;
        std::fs::create_dir_all(dir.join("answers"))?;
        let mut p = Self { dir: dir.to_path_buf(), entries: vec![], claims: BTreeMap::new(), published: BTreeSet::new() };
        p.reload()?;
        Ok(p)
    }

    fn lock(&self) -> Result<Lock> {
        let f = OpenOptions::new().create(true).truncate(false).read(true).write(true).mode(0o600).open(self.dir.join("pod.lock"))?;
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(Lock(f))
    }

    /// Read the pod's files again (other Whales append while this one learns).
    pub fn reload(&mut self) -> Result<()> {
        self.entries = lines(&self.dir.join("terms.jsonl"))?.into_iter().map(serde_json::from_value).collect::<std::result::Result<_, _>>()?;
        self.claims.clear();
        for v in lines(&self.dir.join("claims.jsonl"))? {
            if let Some(t) = v["term"].as_str() {
                self.claims.entry(t.to_string()).or_insert(v);
            }
        }
        self.published = self.entries.iter().map(|e| format!("{}:{}", e.whale, e.item)).collect();
        for v in lines(&self.dir.join("review.jsonl"))? {
            if let (Some(w), Some(i)) = (v["whale"].as_str(), v["item"].as_str()) {
                self.published.insert(format!("{w}:{i}"));
            }
        }
        Ok(())
    }

    /// Whether any Whale of the pod has asked (claimed) the term or the pod knows it.
    pub fn has(&self, term: &str) -> bool {
        let t = norm_term(term);
        self.claims.contains_key(&t) || self.entries.iter().any(|e| e.term == t && !e.facts.is_empty())
    }

    /// Claim a term before asking it: true when this Whale may ask it (nobody claimed it and the
    /// pod has no facts for it), and then the claim is recorded.
    pub fn claim(&mut self, term: &str, whale: &str, paper: &str) -> Result<bool> {
        let _l = self.lock()?;
        self.reload()?;
        if self.has(term) {
            return Ok(false);
        }
        let v = json!({"term":norm_term(term),"display":term,"whale":whale,"paper":paper,"state":"claimed","unix":now()});
        append(&self.dir.join("claims.jsonl"), &v)?;
        self.claims.insert(norm_term(term), v);
        Ok(true)
    }

    /// Publish a Whale's own term knowledge and asked terms: every term item not published yet
    /// (its non-bare facts as an entry, its bare facts to `review.jsonl`), and a claim for every
    /// term it has asked. Returns the number of entries added.
    pub fn publish(&mut self, whale: &str, store: &Store) -> Result<usize> {
        let _l = self.lock()?;
        self.reload()?;
        let mut added = 0;
        for it in store.items.iter().filter(|i| i.kind == "term") {
            let key = format!("{whale}:{}", it.id);
            if self.published.contains(&key) {
                continue;
            }
            let term = norm_term(&it.key);
            let (bad, good): (Vec<Fact>, Vec<Fact>) = it.facts.iter().cloned().partition(bare);
            for f in &bad {
                append(&self.dir.join("review.jsonl"), &json!({"term":term,"display":it.key,"whale":whale,"paper":it.paper,"item":it.id,"fact":f,"reason":"bare_category","unix":now()}))?;
            }
            let sha = it.provenance["answer_sha256"].as_str().map(String::from);
            if let Some(h) = &sha {
                let dst = self.dir.join("answers").join(format!("{h}.txt"));
                if !dst.exists() {
                    if let Ok(text) = std::fs::read_to_string(store.dir.join("answers").join(format!("{h}.txt"))) {
                        write_sync(&dst, text.as_bytes())?;
                    }
                }
            }
            if !good.is_empty() {
                let e = Entry { term: term.clone(), display: it.key.clone(), whale: whale.into(), paper: it.paper.clone(), item: it.id.clone(), facts: good, answer_sha256: sha, unix: now() };
                append(&self.dir.join("terms.jsonl"), &serde_json::to_value(&e)?)?;
                self.entries.push(e);
                added += 1;
            } else if bad.is_empty() {
                // Nothing kept and nothing to review: recorded as published so it is not looked at
                // again (the claim below still stops other Whales asking it).
                append(&self.dir.join("review.jsonl"), &json!({"term":term,"display":it.key,"whale":whale,"paper":it.paper,"item":it.id,"reason":"no_facts","unix":now()}))?;
            }
            self.published.insert(key);
        }
        for (k, v) in &store.asked {
            let Some(t) = k.strip_prefix("term:") else { continue };
            let t = norm_term(t);
            if !self.claims.contains_key(&t) {
                let c = json!({"term":t,"whale":whale,"paper":v["paper"],"state":v["state"],"unix":now()});
                append(&self.dir.join("claims.jsonl"), &c)?;
                self.claims.insert(t, c);
            }
        }
        Ok(added)
    }

    /// Record a fact flagged for review (a bare category) that was not kept.
    pub fn review(&self, whale: &str, paper: &str, term: &str, fact: &Fact) -> Result<()> {
        let _l = self.lock()?;
        append(&self.dir.join("review.jsonl"), &json!({"term":norm_term(term),"display":term,"whale":whale,"paper":paper,"fact":fact,"reason":"bare_category","unix":now()}))
    }

    /// The pod's term knowledge as knowledge items of other Whales (`exclude` is the reading Whale
    /// itself, whose own store already holds its items), ordered by (term, whale, item). Item ids
    /// are `pod:<whale>:<item>`.
    pub fn items(&self, exclude: &str) -> Vec<Item> {
        let mut v: Vec<&Entry> = self.entries.iter().filter(|e| e.whale != exclude && !e.facts.is_empty()).collect();
        v.sort_by(|a, b| (&a.term, &a.whale, &a.item).cmp(&(&b.term, &b.whale, &b.item)));
        v.into_iter()
            .map(|e| Item {
                id: format!("pod:{}:{}", e.whale, e.item),
                whale: e.whale.clone(),
                kind: "term".into(),
                key: e.display.clone(),
                paper: e.paper.clone(),
                facts: e.facts.clone(),
                provenance: json!({"source":"pod","whale":e.whale,"paper":e.paper,"item":e.item,"answer_sha256":e.answer_sha256}),
                unix: e.unix,
            })
            .collect()
    }

    /// Every answer text behind the pod's published facts.
    pub fn answers(&self) -> Result<Vec<String>> {
        let mut v: Vec<PathBuf> = std::fs::read_dir(self.dir.join("answers"))?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        v.sort();
        v.iter().map(|p| Ok(std::fs::read_to_string(p)?)).collect()
    }
}

/// Facts of an answer split into the kept ones and the bare-category ones flagged for review.
pub fn split_bare(facts: Vec<Fact>) -> (Vec<Fact>, Vec<Fact>) {
    facts.into_iter().partition(|f| !knowledge::bare(f))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fact(rel: &str, obj: &str) -> Fact {
        Fact { relation: rel.into(), subject: String::new(), verb: String::new(), object: obj.into() }
    }
    fn item(id: &str, key: &str, facts: Vec<Fact>) -> Item {
        Item { id: id.into(), whale: "w".into(), kind: "term".into(), key: key.into(), paper: "p".into(), facts, provenance: json!({"source":"terminal"}), unix: 0 }
    }

    /// One store for the pod: a term one Whale asked is not asked by another, plural and case
    /// variants are one term, bare-category facts go to review, and what one Whale learned the
    /// others read (red control: without the claim the second Whale would ask again).
    #[test]
    fn one_term_one_question_across_the_pod() {
        let d = crate::learn::tests::tmp("pod");
        let mut pod = Pod::open(&d.join("pod")).unwrap();
        let mut s1 = Store::open(&d.join("w1")).unwrap();
        s1.retain(item("k00001", "MRFs", vec![fact("is_a", "a graphical model")])).unwrap();
        s1.retain(item("k00002", "Goldberg", vec![fact("is_a", "the family name")])).unwrap();
        s1.mark_asked("term:MRFs", json!({"state":"answered","paper":"p"})).unwrap();
        s1.mark_asked("term:Goldberg", json!({"state":"answered","paper":"p"})).unwrap();
        assert_eq!(pod.publish("whale-1", &s1).unwrap(), 1);
        // Publishing again adds nothing.
        assert_eq!(pod.publish("whale-1", &s1).unwrap(), 0);
        // «MRF», «mrfs» and «MRFs» are one term; another Whale may not ask it.
        assert!(pod.has("MRF") && pod.has("mrfs"));
        assert!(!pod.claim("MRF", "whale-2", "q").unwrap());
        // The surname was asked (claimed) but its bare fact is under review, not knowledge.
        assert!(!pod.claim("Goldberg", "whale-2", "q").unwrap());
        let review = std::fs::read_to_string(d.join("pod/review.jsonl")).unwrap();
        assert!(review.contains("bare_category") && review.contains("the family name"), "{review}");
        // A new term is claimed once.
        assert!(pod.claim("Treebanks", "whale-2", "q").unwrap());
        assert!(!pod.claim("treebank", "whale-3", "r").unwrap(), "a plural variant was asked again");
        // The other Whales read whale-1's knowledge; whale-1 reads none of its own back.
        let items = pod.items("whale-2");
        assert_eq!(items.len(), 1);
        assert_eq!((items[0].id.as_str(), items[0].facts[0].object.as_str()), ("pod:whale-1:k00001", "a graphical model"));
        assert!(pod.items("whale-1").is_empty());
        // A second process sees the same state.
        let again = Pod::open(&d.join("pod")).unwrap();
        assert_eq!(again.items("whale-9"), items);
        let _ = std::fs::remove_dir_all(d);
    }
}
