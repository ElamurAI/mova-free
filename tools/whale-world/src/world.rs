//! The global world (`mova-whale-world-v1`): every text of every dossier, epoch after epoch, and the
//! cached parses of the documents' topic-corpus neighbours, read once by Mova and kept as one graph —
//! entities with every mention, statements with their sentence and document, a word → sentence →
//! document index and document edges. Nothing is dropped: the store holds all content; only the
//! storage is economical (zstd shards and hash buckets, read lazily through an in-process LRU).
//!
//! `world(e)` is the graph over the texts of epochs `< e` (originals, earlier descriptions and pages
//! are epoch 0, so `world(1)` holds them). One store serves every `world(e)`: each text keeps its
//! epoch, and a query at epoch `e` reads only texts of earlier epochs. `add` is incremental: texts
//! already indexed are never read again; a new text is parsed once and its postings are merged into
//! the buckets its words and entities fall in.
//!
//! Layout under `world_root`: `world.json`, `docs.jsonl`, `texts.jsonl` (append-only),
//! `edges.jsonl`, `shards/<t>.jsonl.zst`, `index/w-XX.json.zst` (256 word buckets),
//! `index/e-XX.json.zst` (64 entity buckets), `doc-entities/<doc>.json.zst`. A build holds
//! `world.lock`; `pending.json` marks a build in progress and makes the next one refuse
//! (an interrupted build is not repaired silently); a finished build renames it `last-build.json`.
use crate::config::Config;
use crate::dossier::{Dossier, Kind};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use topic_corpus::corpus::{self, Corpus, Options, Word};
use topic_corpus::licence::Class;

pub const SCHEMA: &str = "mova-whale-world-v1";
pub const W_BUCKETS: u64 = 256;
pub const E_BUCKETS: u64 = 64;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub schema: String,
    pub reader: String,
    /// The highest epoch built: the store holds `world(epoch)`.
    pub epoch: u64,
    pub docs: usize,
    pub texts: usize,
    pub sentences: usize,
    pub statements: usize,
    pub entities: usize,
    pub edges: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WDoc {
    pub id: String,
    pub title: String,
    /// `dossier` or `neighbour`.
    pub kind: String,
    pub texts: Vec<u32>,
    /// The library graph's edges of this document were added.
    #[serde(default)]
    pub graph: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WText {
    pub t: u32,
    /// A dossier text id (`<doc>/<epoch>-<kind>-<n>`) or a corpus document (`lib:<id>`).
    pub id: String,
    pub doc: String,
    pub epoch: u64,
    /// A dossier kind or `neighbour`.
    pub kind: String,
    pub sha256: String,
    pub public_ok: bool,
    pub licence_class: Class,
    pub sentences: usize,
    pub statements: usize,
    /// Statements by kind (`claim`, `method`, `result`, `statement`).
    #[serde(default)]
    pub statement_kinds: BTreeMap<String, usize>,
    /// Neighbour roles in its topic corpus (`reference`, `similar`, …).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WSent {
    pub s: usize,
    pub part: String,
    pub text: String,
    pub words: Vec<Word>,
    pub frames: Vec<String>,
    /// `claim`, `method`, `result`, `statement` for sentences with frames; empty otherwise.
    #[serde(default)]
    pub kind: String,
    /// Entity keys met in the sentence.
    #[serde(default)]
    pub ents: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    /// `cites` (from cites to), `similar`, `same_author`.
    pub kind: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EntityRec {
    /// `acronym`, `name`, `term` with counts.
    pub kinds: BTreeMap<String, u64>,
    pub forms: BTreeMap<String, u64>,
    /// `(text, sentence)` of every mention.
    pub postings: Vec<(u32, u32)>,
    /// Acronym expansions: `(text, sentence, expansion)`.
    #[serde(default)]
    pub defs: Vec<(u32, u32, String)>,
}

impl EntityRec {
    pub fn kind(&self) -> &str {
        self.kinds.iter().max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0))).map(|x| x.0.as_str()).unwrap_or("term")
    }
}

type WordBucket = BTreeMap<String, Vec<(u32, u32)>>;
type EntBucket = BTreeMap<String, EntityRec>;
/// Per document: entity → (kind, mentions) over its texts, with the epoch of each text's share.
pub type DocEnts = BTreeMap<String, (String, u64)>;

/// FNV-1a: a stable bucket for a key.
pub fn bucket(key: &str, n: u64) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in key.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h % n
}

/// The index key of a word form.
pub fn word_key(form: &str) -> Option<String> {
    let k = form.to_lowercase();
    if k.chars().any(char::is_alphanumeric) {
        Some(k)
    } else {
        None
    }
}

/// The entity key of a surface (`GECs` → `gec`, `Treebanks` → `treebank`).
pub fn entity_key(surface: &str) -> String {
    mova_whale::knowledge::norm_term(surface)
}

/// An acronym, also in the plural (`GECs`).
pub fn acr(w: &str) -> bool {
    topic_corpus::world::acronym(w) || w.strip_suffix('s').is_some_and(topic_corpus::world::acronym)
}

/// Entities of one parsed sentence: `(key, kind, surface)`. Acronyms; runs of capitalized proper
/// nouns (names); noun lemmas of four letters or more and noun compounds (terms).
pub fn entities(words: &[Word]) -> Vec<(String, &'static str, String)> {
    let mut out = vec![];
    let mut i = 0;
    while i < words.len() {
        let w = &words[i];
        if acr(&w.0) {
            out.push((entity_key(&w.0), "acronym", w.0.clone()));
            i += 1;
            continue;
        }
        let cap = |w: &Word| w.2 == "PROPN" && w.0.chars().next().is_some_and(char::is_uppercase) && !acr(&w.0) && w.0.chars().all(|c| c.is_alphabetic() || c == '-' || c == '\'');
        if cap(w) {
            let mut j = i;
            while j < words.len() && cap(&words[j]) {
                j += 1;
            }
            let s: Vec<&str> = words[i..j].iter().map(|w| w.0.as_str()).collect();
            let s = s.join(" ");
            out.push((entity_key(&s), "name", s));
            i = j;
            continue;
        }
        let noun = |w: &Word| w.2 == "NOUN" && w.1.chars().count() >= 4 && w.1.chars().all(|c| c.is_alphabetic() || c == '-');
        if noun(w) {
            out.push((w.1.to_lowercase(), "term", w.1.clone()));
        }
        if w.4 == "compound" && w.3 > 0 {
            if let Some(h) = words.get(w.3 - 1) {
                if (w.2 == "NOUN" || w.2 == "ADJ") && noun(h) && w.1.chars().all(|c| c.is_alphabetic() || c == '-') && w.1.chars().count() >= 2 {
                    let s = format!("{} {}", w.1, h.1);
                    out.push((s.to_lowercase(), "term", s));
                }
            }
        }
        i += 1;
    }
    out
}

/// The world's reader: the topic corpus reader plus the heading fallback of `annotate_text`.
pub fn reader() -> String {
    format!("{}+heading-fallback-1", corpus::READER)
}

/// Read a text with Mova (`topic_corpus::corpus::annotate`). The Whales' reader treats everything
/// before the first section heading as front matter; a text with no section the reader admits
/// (a plain draft, a PDF extraction without headings) is read again with one `## Text` heading
/// after its title line, so its sentences are not lost.
pub fn annotate_text(s: &str) -> Result<Vec<corpus::Sentence>> {
    let first = corpus::annotate(s, usize::MAX)?;
    if !first.is_empty() {
        return Ok(first);
    }
    let (head, rest) = match s.split_once('\n') {
        Some((h, r)) if h.starts_with("# ") => (format!("{h}\n\n"), r),
        _ => (String::new(), s),
    };
    corpus::annotate(&format!("{head}## Text\n\n{rest}"), usize::MAX)
}

pub fn write_private(p: &Path, b: &[u8]) -> Result<()> {
    corpus::write_private(p, b)
}

fn read_zjson<T: for<'de> Deserialize<'de> + Default>(p: &Path) -> Result<T> {
    if !p.exists() {
        return Ok(T::default());
    }
    let b = zstd::decode_all(std::fs::File::open(p)?)?;
    serde_json::from_slice(&b).with_context(|| format!("{}", p.display()))
}

fn write_zjson<T: Serialize>(p: &Path, v: &T) -> Result<()> {
    write_private(p, &zstd::encode_all(&serde_json::to_vec(v)?[..], 9)?)
}

fn jsonl<T: for<'de> Deserialize<'de>>(p: &Path) -> Result<Vec<T>> {
    if !p.exists() {
        return Ok(vec![]);
    }
    std::fs::read_to_string(p)?.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).with_context(|| format!("{}", p.display()))).collect()
}

/// A bounded in-process cache, least recently used dropped first (deterministic: ties by key).
pub struct Lru<K: Ord + Clone, V> {
    cap: usize,
    clock: u64,
    map: BTreeMap<K, (u64, V)>,
}

impl<K: Ord + Clone, V: Clone> Lru<K, V> {
    pub fn new(cap: usize) -> Self {
        Lru { cap: cap.max(1), clock: 0, map: BTreeMap::new() }
    }
    pub fn get(&mut self, k: &K) -> Option<V> {
        self.clock += 1;
        let c = self.clock;
        self.map.get_mut(k).map(|e| {
            e.0 = c;
            e.1.clone()
        })
    }
    pub fn put(&mut self, k: K, v: V) {
        self.clock += 1;
        self.map.insert(k, (self.clock, v));
        while self.map.len() > self.cap {
            let old = self.map.iter().min_by_key(|(_, e)| e.0).map(|(k, _)| k.clone()).unwrap();
            self.map.remove(&old);
        }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

pub struct World {
    pub root: PathBuf,
    pub meta: Meta,
    pub docs: BTreeMap<String, WDoc>,
    pub texts: Vec<WText>,
    pub edges: Vec<Edge>,
    shards: Lru<u32, std::rc::Rc<Vec<WSent>>>,
    wb: Lru<u64, std::rc::Rc<WordBucket>>,
    eb: Lru<u64, std::rc::Rc<EntBucket>>,
}

struct Lock(std::fs::File);
impl Drop for Lock {
    fn drop(&mut self) {
        unsafe { libc::flock(self.0.as_raw_fd(), libc::LOCK_UN) };
    }
}

impl World {
    /// Open the world (an empty one when nothing is built yet; nothing is created).
    pub fn open(cfg: &Config) -> Result<World> {
        let root = cfg.world_root.clone();
        let meta: Meta = if root.join("world.json").exists() { serde_json::from_slice(&std::fs::read(root.join("world.json"))?)? } else { Meta { schema: SCHEMA.into(), reader: reader(), ..Default::default() } };
        if meta.schema != SCHEMA {
            bail!("{} is not a {SCHEMA} store", root.display());
        }
        let docs = jsonl::<WDoc>(&root.join("docs.jsonl"))?.into_iter().map(|d| (d.id.clone(), d)).collect();
        let texts: Vec<WText> = jsonl(&root.join("texts.jsonl"))?;
        if texts.iter().enumerate().any(|(i, t)| t.t as usize != i) {
            bail!("{}: text indices are not dense", root.display());
        }
        let edges = jsonl(&root.join("edges.jsonl"))?;
        let l = &cfg.limits;
        Ok(World { root, meta, docs, texts, edges, shards: Lru::new(l.cache_shards), wb: Lru::new(l.cache_buckets), eb: Lru::new(l.cache_buckets) })
    }

    pub fn shard_path(&self, t: u32) -> PathBuf {
        self.root.join("shards").join(format!("{t}.jsonl.zst"))
    }

    /// The parsed sentences of text `t` (lazy, cached).
    pub fn shard(&mut self, t: u32) -> Result<std::rc::Rc<Vec<WSent>>> {
        if let Some(v) = self.shards.get(&t) {
            return Ok(v);
        }
        let b = zstd::decode_all(std::fs::File::open(self.shard_path(t)).with_context(|| format!("shard {t}"))?)?;
        let v: Vec<WSent> = String::from_utf8(b)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect::<Result<_>>()?;
        let v = std::rc::Rc::new(v);
        self.shards.put(t, v.clone());
        Ok(v)
    }

    pub fn sentence(&mut self, t: u32, s: u32) -> Result<WSent> {
        self.shard(t)?.get(s as usize).cloned().ok_or_else(|| anyhow!("text {t} has no sentence {s}"))
    }

    fn wpath(&self, b: u64) -> PathBuf {
        self.root.join("index").join(format!("w-{b:02x}.json.zst"))
    }
    fn epath(&self, b: u64) -> PathBuf {
        self.root.join("index").join(format!("e-{b:02x}.json.zst"))
    }

    /// Postings of a word key.
    pub fn word(&mut self, key: &str) -> Result<Vec<(u32, u32)>> {
        let b = bucket(key, W_BUCKETS);
        let m = match self.wb.get(&b) {
            Some(m) => m,
            None => {
                let m = std::rc::Rc::new(read_zjson::<WordBucket>(&self.wpath(b))?);
                self.wb.put(b, m.clone());
                m
            }
        };
        Ok(m.get(key).cloned().unwrap_or_default())
    }

    /// The record of an entity key.
    pub fn entity(&mut self, key: &str) -> Result<Option<EntityRec>> {
        let b = bucket(key, E_BUCKETS);
        let m = match self.eb.get(&b) {
            Some(m) => m,
            None => {
                let m = std::rc::Rc::new(read_zjson::<EntBucket>(&self.epath(b))?);
                self.eb.put(b, m.clone());
                m
            }
        };
        Ok(m.get(key).cloned())
    }

    /// Entity counts of a document over all its texts (epoch per text not separated: see
    /// `doc_entities_at`).
    pub fn doc_entities(&self, doc: &str) -> Result<BTreeMap<String, BTreeMap<u32, (String, u64)>>> {
        read_zjson(&self.root.join("doc-entities").join(format!("{}.json.zst", corpus::file_of(doc))))
    }

    /// Entity → (kind, mentions) of a document over its texts visible at `epoch`.
    pub fn doc_entities_at(&self, doc: &str, epoch: u64) -> Result<DocEnts> {
        let mut out = DocEnts::new();
        for (k, per) in self.doc_entities(doc)? {
            for (t, (kind, n)) in per {
                if self.visible(t, epoch) {
                    let e = out.entry(k.clone()).or_insert((kind.clone(), 0));
                    e.1 += n;
                }
            }
        }
        Ok(out)
    }

    /// Whether text `t` belongs to `world(epoch)`.
    pub fn visible(&self, t: u32, epoch: u64) -> bool {
        self.texts.get(t as usize).is_some_and(|x| x.epoch < epoch)
    }

    pub fn text(&self, t: u32) -> &WText {
        &self.texts[t as usize]
    }

    /// Statistics of `world(epoch)`.
    pub fn stats(&self, epoch: u64) -> Value {
        let vis: Vec<&WText> = self.texts.iter().filter(|t| t.epoch < epoch).collect();
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        let mut per_doc: BTreeMap<&str, usize> = BTreeMap::new();
        for t in &vis {
            *kinds.entry(t.kind.as_str()).or_default() += 1;
            *per_doc.entry(t.doc.as_str()).or_default() += 1;
        }
        let mut sk: BTreeMap<&str, usize> = BTreeMap::new();
        for t in &vis {
            for (k, n) in &t.statement_kinds {
                *sk.entry(k.as_str()).or_default() += n;
            }
        }
        let dossiers = self.docs.values().filter(|d| d.kind == "dossier").count();
        json!({"epoch": epoch, "built_epoch": self.meta.epoch, "docs": per_doc.len(), "dossier_docs": dossiers, "texts": vis.len(), "text_kinds": kinds,
               "sentences": vis.iter().map(|t| t.sentences).sum::<usize>(), "statements": vis.iter().map(|t| t.statements).sum::<usize>(), "statement_kinds": sk,
               "store": self.meta, "texts_per_doc": per_doc})
    }

    /// Add the texts of epochs `< epoch` of `docs` (default: every dossier) that are not yet in
    /// the store, with their topic-corpus neighbours and library edges. Returns a report.
    pub fn add(&mut self, cfg: &Config, epoch: u64, docs: &[String]) -> Result<Value> {
        for d in [self.root.clone(), self.root.join("shards"), self.root.join("index"), self.root.join("doc-entities")] {
            mova_whale::util::private_dir(&d).map_err(|e| anyhow!("{e}"))?;
        }
        let f = std::fs::OpenOptions::new().create(true).truncate(false).write(true).mode(0o600).open(self.root.join("world.lock"))?;
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            bail!("another build holds {}", self.root.join("world.lock").display());
        }
        let _lock = Lock(f);
        if self.root.join("pending.json").exists() {
            bail!("{}: an earlier build was interrupted (pending.json); rebuild the world into a new root", self.root.display());
        }
        // Reload under the lock; a store read by another reader is not mixed.
        let fresh = World::open(cfg)?;
        self.meta = fresh.meta;
        self.docs = fresh.docs;
        self.texts = fresh.texts;
        self.edges = fresh.edges;
        if !self.texts.is_empty() && self.meta.reader != reader() {
            bail!("{} was read by {}, this build reads with {}: build into a new world root", self.root.display(), self.meta.reader, reader());
        }
        let docs: Vec<String> = if docs.is_empty() { Dossier::list(cfg)? } else { docs.to_vec() };
        let known: BTreeSet<String> = self.texts.iter().map(|t| t.id.clone()).collect();
        // 1. Collect new texts (all read and hash-checked before anything is written).
        struct New {
            text: WText,
            title: String,
            doc_kind: &'static str,
            body: Body,
        }
        enum Body {
            Text(String),
            Parsed(Vec<corpus::Sentence>),
        }
        let mut new: Vec<New> = vec![];
        let mut new_edges: Vec<Edge> = vec![];
        let lib_titles = |c: &Corpus, id: &str| c.doc(id).map(|d| d.title.clone()).unwrap_or_default();
        for doc in &docs {
            if !Dossier::exists(cfg, doc) {
                bail!("{doc} has no dossier");
            }
            let d = Dossier::open(cfg, doc)?;
            let rows = d.rows()?;
            let title = self.docs.get(doc).map(|x| x.title.clone()).filter(|t| !t.is_empty()).unwrap_or_else(|| rows.iter().find(|r| r.kind == Kind::Original).and_then(|r| d.read(r).ok()).and_then(|t| t.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].trim().to_string())).unwrap_or_default());
            for r in rows.iter().filter(|r| r.epoch < epoch && !known.contains(&r.id)) {
                let body = d.read(r)?;
                new.push(New {
                    text: WText { t: 0, id: r.id.clone(), doc: doc.clone(), epoch: r.epoch, kind: r.kind.name().into(), sha256: r.sha256.clone(), public_ok: r.public_ok, licence_class: r.licence_class, sentences: 0, statements: 0, statement_kinds: BTreeMap::new(), roles: vec![] },
                    title: title.clone(),
                    doc_kind: "dossier",
                    body: Body::Text(body),
                });
            }
            if !self.docs.contains_key(doc) && !new.iter().any(|n| &n.text.doc == doc) {
                new.push(New { text: WText { t: u32::MAX, id: String::new(), doc: doc.clone(), epoch: 0, kind: String::new(), sha256: String::new(), public_ok: false, licence_class: Class::Unknown, sentences: 0, statements: 0, statement_kinds: BTreeMap::new(), roles: vec![] }, title: title.clone(), doc_kind: "dossier", body: Body::Text(String::new()) });
            }
            // Neighbours from the document's topic corpus: cached parses only.
            if let (true, Some(root)) = (cfg.limits.neighbours, &cfg.corpora_root) {
                let dir = root.join(doc);
                if dir.join("corpus.json").is_file() {
                    let mut c = Corpus::load(&dir, Options::default())?;
                    let ids: Vec<String> = c.docs.iter().filter(|x| x.kind == "paper" && x.id.starts_with("lib:") && x.id != format!("lib:{doc}")).map(|x| x.id.clone()).collect();
                    for id in ids {
                        let x = c.doc(&id).unwrap().clone();
                        let lid = id.trim_start_matches("lib:").to_string();
                        for r in &x.roles {
                            let (from, to, kind) = match r.as_str() {
                                "reference" => (doc.clone(), lid.clone(), "cites"),
                                "cited_by" => (lid.clone(), doc.clone(), "cites"),
                                "similar" => (doc.clone(), lid.clone(), "similar"),
                                "same_author" => (doc.clone(), lid.clone(), "same_author"),
                                _ => continue,
                            };
                            new_edges.push(Edge { from, to, kind: kind.into(), weight: 1.0 });
                        }
                        if known.contains(&id) || new.iter().any(|n| n.text.id == id) || Dossier::exists(cfg, &lid) || !c.cached(&id) {
                            continue;
                        }
                        let parsed = c.parsed(&id)?;
                        let title = lib_titles(&c, &id);
                        new.push(New {
                            text: WText { t: 0, id: id.clone(), doc: lid, epoch: 0, kind: "neighbour".into(), sha256: x.text_sha256.clone(), public_ok: false, licence_class: x.licence_class, sentences: 0, statements: 0, statement_kinds: BTreeMap::new(), roles: x.roles.clone() },
                            title,
                            doc_kind: "neighbour",
                            body: Body::Parsed(parsed),
                        });
                    }
                }
            }
        }
        // Library edges for dossier documents without them.
        let need_graph: Vec<&String> = docs.iter().filter(|d| !self.docs.get(*d).is_some_and(|x| x.graph)).collect();
        let mut graphed: BTreeSet<String> = BTreeSet::new();
        if let (false, Some(lc)) = (need_graph.is_empty(), &cfg.library) {
            let lib = topic_corpus::library::Library::load(lc)?;
            for doc in need_graph {
                let Ok(g) = topic_corpus::graph::build(&lib, doc, &topic_corpus::config::Limits::default()) else { continue };
                for r in &g.references {
                    if let Some(w) = &r.work {
                        new_edges.push(Edge { from: doc.clone(), to: w.id.clone(), kind: "cites".into(), weight: 1.0 });
                    }
                }
                for n in &g.cited_by {
                    new_edges.push(Edge { from: n.id.clone(), to: doc.clone(), kind: "cites".into(), weight: 1.0 });
                }
                for n in &g.similar {
                    new_edges.push(Edge { from: doc.clone(), to: n.id.clone(), kind: "similar".into(), weight: (n.score.unwrap_or(0.0) * 1e6).round() / 1e6 });
                }
                for n in &g.same_author {
                    new_edges.push(Edge { from: doc.clone(), to: n.id.clone(), kind: "same_author".into(), weight: 1.0 });
                }
                graphed.insert(doc.clone());
            }
        }
        // 2. Parse and index.
        let first = self.texts.len() as u32;
        let real: Vec<&New> = new.iter().filter(|n| n.text.t != u32::MAX).collect();
        write_private(&self.root.join("pending.json"), &serde_json::to_vec(&json!({"first": first, "texts": real.iter().map(|n| n.text.id.clone()).collect::<Vec<_>>()}))?)?;
        let mut wpost: BTreeMap<u64, WordBucket> = BTreeMap::new();
        let mut epost: BTreeMap<u64, EntBucket> = BTreeMap::new();
        let mut dents: BTreeMap<String, BTreeMap<String, BTreeMap<u32, (String, u64)>>> = BTreeMap::new();
        let mut added: Vec<WText> = vec![];
        let mut sentences = 0;
        let mut statements = 0;
        for n in &new {
            if !self.docs.contains_key(&n.text.doc) {
                self.docs.insert(n.text.doc.clone(), WDoc { id: n.text.doc.clone(), title: n.title.clone(), kind: n.doc_kind.into(), texts: vec![], graph: false });
            }
            if n.text.t == u32::MAX {
                continue;
            }
            let t = first + added.len() as u32;
            let parsed = match &n.body {
                Body::Text(s) => annotate_text(s)?,
                Body::Parsed(p) => p.clone(),
            };
            let mut out = String::new();
            let mut wt = n.text.clone();
            wt.t = t;
            for (i, s) in parsed.into_iter().enumerate() {
                let ents = entities(&s.words);
                let forms: Vec<String> = s.words.iter().map(|w| w.0.clone()).collect();
                let mut keys: BTreeSet<String> = BTreeSet::new();
                for (k, kind, surface) in &ents {
                    let rec = epost.entry(bucket(k, E_BUCKETS)).or_default().entry(k.clone()).or_default();
                    if keys.insert(k.clone()) {
                        rec.postings.push((t, i as u32));
                        if *kind == "acronym" {
                            if let Some(x) = mova_whale::knowledge::expansion(surface, &forms) {
                                rec.defs.push((t, i as u32, x));
                            }
                        }
                    }
                    *rec.kinds.entry(kind.to_string()).or_default() += 1;
                    *rec.forms.entry(surface.clone()).or_default() += 1;
                    let de = dents.entry(n.text.doc.clone()).or_default().entry(k.clone()).or_default().entry(t).or_insert((kind.to_string(), 0));
                    de.1 += 1;
                }
                let mut wkeys: BTreeSet<String> = BTreeSet::new();
                for w in &s.words {
                    if w.2 == "PUNCT" {
                        continue;
                    }
                    wkeys.extend(word_key(&w.0));
                    wkeys.extend(word_key(&w.1));
                }
                for k in wkeys {
                    wpost.entry(bucket(&k, W_BUCKETS)).or_default().entry(k).or_default().push((t, i as u32));
                }
                let kind = if s.frames.is_empty() { String::new() } else { topic_corpus::world::kind(&s).to_string() };
                if !kind.is_empty() {
                    wt.statements += 1;
                    *wt.statement_kinds.entry(kind.clone()).or_default() += 1;
                }
                let ws = WSent { s: i, part: s.part, text: s.text, words: s.words, frames: s.frames, kind, ents: keys.into_iter().collect() };
                out.push_str(&(serde_json::to_string(&ws)? + "\n"));
                wt.sentences += 1;
            }
            write_private(&self.shard_path(t), &zstd::encode_all(out.as_bytes(), 9)?)?;
            sentences += wt.sentences;
            statements += wt.statements;
            self.docs.get_mut(&n.text.doc).unwrap().texts.push(t);
            added.push(wt);
        }
        for (b, m) in wpost {
            let p = self.wpath(b);
            let mut cur: WordBucket = read_zjson(&p)?;
            for (k, v) in m {
                cur.entry(k).or_default().extend(v);
            }
            write_zjson(&p, &cur)?;
        }
        let mut new_entities = 0;
        for (b, m) in epost {
            let p = self.epath(b);
            let mut cur: EntBucket = read_zjson(&p)?;
            for (k, v) in m {
                let e = cur.entry(k).or_insert_with(|| {
                    new_entities += 1;
                    EntityRec::default()
                });
                for (x, n) in v.kinds {
                    *e.kinds.entry(x).or_default() += n;
                }
                for (x, n) in v.forms {
                    *e.forms.entry(x).or_default() += n;
                }
                e.postings.extend(v.postings);
                e.defs.extend(v.defs);
            }
            write_zjson(&p, &cur)?;
        }
        for (doc, m) in dents {
            let p = self.root.join("doc-entities").join(format!("{}.json.zst", corpus::file_of(&doc)));
            let mut cur: BTreeMap<String, BTreeMap<u32, (String, u64)>> = read_zjson(&p)?;
            for (k, per) in m {
                cur.entry(k).or_default().extend(per);
            }
            write_zjson(&p, &cur)?;
        }
        for d in &graphed {
            if let Some(x) = self.docs.get_mut(d) {
                x.graph = true;
            }
        }
        // 3. Tables: texts appended; docs, edges and meta rewritten (small).
        let mut tf = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(self.root.join("texts.jsonl"))?;
        for t in &added {
            tf.write_all((serde_json::to_string(t)? + "\n").as_bytes())?;
        }
        tf.sync_all()?;
        self.texts.extend(added.iter().cloned());
        let mut es: Vec<Edge> = self.edges.drain(..).chain(new_edges).collect();
        es.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)).then_with(|| a.kind.cmp(&b.kind)).then(a.weight.total_cmp(&b.weight)));
        es.dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);
        self.edges = es;
        write_private(&self.root.join("edges.jsonl"), self.edges.iter().map(|e| serde_json::to_string(e).unwrap() + "\n").collect::<String>().as_bytes())?;
        write_private(&self.root.join("docs.jsonl"), self.docs.values().map(|d| serde_json::to_string(d).unwrap() + "\n").collect::<String>().as_bytes())?;
        self.meta.schema = SCHEMA.into();
        self.meta.reader = reader();
        self.meta.epoch = self.meta.epoch.max(epoch);
        self.meta.docs = self.docs.len();
        self.meta.texts = self.texts.len();
        self.meta.sentences += sentences;
        self.meta.statements += statements;
        self.meta.entities += new_entities;
        self.meta.edges = self.edges.len();
        write_private(&self.root.join("world.json"), &serde_json::to_vec_pretty(&self.meta)?)?;
        std::fs::rename(self.root.join("pending.json"), self.root.join("last-build.json"))?;
        self.shards = Lru::new(self.shards.cap);
        self.wb = Lru::new(self.wb.cap);
        self.eb = Lru::new(self.eb.cap);
        Ok(json!({"epoch": epoch, "added_texts": added.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), "sentences": sentences, "statements": statements, "new_entities": new_entities, "graphed": graphed, "meta": self.meta}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(form: &str, lemma: &str, upos: &str, head: usize, rel: &str) -> Word {
        (form.into(), lemma.into(), upos.into(), head, rel.into())
    }

    #[test]
    fn entities_of_a_sentence() {
        let s = vec![w("Christopher", "christopher", "PROPN", 2, "compound"), w("Bryant", "bryant", "PROPN", 3, "nsubj"), w("annotates", "annotate", "VERB", 0, "root"), w("error", "error", "NOUN", 5, "compound"), w("types", "type", "NOUN", 3, "obj"), w("for", "for", "ADP", 7, "case"), w("GECs", "GECs", "PROPN", 3, "obl")];
        let e = entities(&s);
        let keys: Vec<(&str, &str)> = e.iter().map(|x| (x.0.as_str(), x.1)).collect();
        assert_eq!(keys, vec![("christopher bryant", "name"), ("error", "term"), ("error type", "term"), ("type", "term"), ("gec", "acronym")]);
        assert_eq!(bucket("gec", 64), bucket("gec", 64));
    }

    #[test]
    fn lru_drops_the_least_recent() {
        let mut l = Lru::new(2);
        l.put(1, "a");
        l.put(2, "b");
        assert_eq!(l.get(&1), Some("a"));
        l.put(3, "c");
        assert_eq!((l.get(&2), l.len()), (None, 2));
    }
}
