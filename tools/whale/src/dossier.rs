//! The epoch's one integration point with two stores it does not own: the document dossiers and
//! the global world.
//!
//! **Dossier** (`ROOT/<doc>/texts/<epoch>-<kind>-<n>.md` plus `ROOT/<doc>/texts.jsonl`): every
//! text of a document — its original, Mova's and the consultant's texts of every epoch, older
//! descriptions and web descriptions — with one row each: `{id, doc, epoch, kind, source, licence,
//! licence_class, public_ok, sha256, words, created_unix, inputs, citations: {count, unresolved}}`.
//! Rows are appended under `ROOT/<doc>/texts.lock`; a text is written once (`create_new`) and never
//! rewritten; registering the same text again (same epoch, kind and SHA-256) returns its row, so
//! a resumed epoch never duplicates a text. Generated texts (`mova`, `sol`) are never public
//! (`public_ok` false whatever their inputs): the texts are internal for now.
//!
//! **Global world** (`WorldConfig`): a document's neighbourhood, nearest first — the documents it
//! cites, is cited by, is similar to, shares authors or entities with — each with its texts by
//! path or content. The writer takes all content of those texts; nothing inside a text is capped.
//! Sources: the world tool's CLI (`world neighbourhood|related CONFIG DOC … --epoch N`, every
//! sentence with `--content`), a closed topic corpus built by `mova-topic-corpus` (its `graph.json`
//! order and its registry), or a fixture directory (`DIR/<doc>.json`, tests). The dossier is the
//! world tool's store through its CLI (`dossier list|text|add`) or this module's own files.
use crate::util::{error, hash, now, try_lease};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub const KINDS: [&str; 5] = ["original", "mova", "sol", "site-old", "web"];

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Citations {
    pub count: usize,
    pub unresolved: usize,
}

/// One row of `texts.jsonl`. Fields other writers add are ignored.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TextRow {
    pub id: String,
    pub doc: String,
    pub epoch: u64,
    pub kind: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub licence: String,
    #[serde(default)]
    pub licence_class: String,
    #[serde(default)]
    pub public_ok: bool,
    pub sha256: String,
    #[serde(default)]
    pub words: usize,
    #[serde(default)]
    pub created_unix: u64,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub citations: Citations,
}

/// A text to register.
pub struct NewText<'a> {
    pub doc: &'a str,
    pub epoch: u64,
    pub kind: &'a str,
    pub text: &'a str,
    pub source: String,
    pub licence: String,
    /// For `original`, `site-old` and `web`: may the text feed a public output (open licence, not
    /// local-only). Ignored for generated kinds, which are never public.
    pub open: bool,
    pub inputs: Vec<String>,
    pub citations: Citations,
}

/// A licence's class: `cc0`, `pd`, `by`, `restricted` (NC, ND or SA terms), `internal` (our own
/// generated texts) or `unknown`.
pub fn licence_class(kind: &str, licence: &str) -> &'static str {
    if matches!(kind, "mova" | "sol") {
        return "internal";
    }
    let l = licence.to_lowercase();
    let has = |x: &str| l.split(|c: char| !c.is_alphanumeric()).any(|w| w == x);
    if l.starts_with("cc0") {
        "cc0"
    } else if l.starts_with("public domain") {
        "pd"
    } else if l.starts_with("cc by") && (has("nc") || has("nd") || has("sa")) {
        "restricted"
    } else if l.starts_with("cc by") {
        "by"
    } else {
        "unknown"
    }
}

/// A document id usable as one directory name.
pub fn valid_doc(doc: &str) -> bool {
    !doc.is_empty() && doc.len() <= 200 && !doc.starts_with('.') && doc.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

pub struct Dossier {
    pub root: PathBuf,
}

impl Dossier {
    pub fn doc_dir(&self, doc: &str) -> Result<PathBuf> {
        if !valid_doc(doc) {
            return Err(error(format!("dossier: {doc:?} is not a valid document id")));
        }
        Ok(self.root.join(doc))
    }
    pub fn rows(&self, doc: &str) -> Result<Vec<TextRow>> {
        let p = self.doc_dir(doc)?.join("texts.jsonl");
        if !p.exists() {
            return Ok(vec![]);
        }
        std::fs::read_to_string(&p)?.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| error(format!("{}: {e}", p.display())))).collect()
    }
    /// The file of a row: `texts/<stem>.md`, the stem being the id after the document.
    pub fn path(&self, row: &TextRow) -> Result<PathBuf> {
        let stem = row.id.rsplit('/').next().unwrap_or(&row.id);
        if stem.is_empty() || stem.contains("..") || !stem.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')) {
            return Err(error(format!("dossier: bad text id {}", row.id)));
        }
        Ok(self.doc_dir(&row.doc)?.join("texts").join(format!("{stem}.md")))
    }
    /// Register a text; the same text of the same epoch and kind returns its existing row.
    pub fn register(&self, t: NewText) -> Result<TextRow> {
        if !KINDS.contains(&t.kind) {
            return Err(error(format!("dossier: unknown kind {}", t.kind)));
        }
        let dir = self.doc_dir(t.doc)?;
        crate::util::private_dir(&self.root)?;
        crate::util::private_dir(&dir)?;
        crate::util::private_dir(&dir.join("texts"))?;
        let _l = loop {
            match try_lease(&dir.join("texts.lock"), true)? {
                Some(f) => break f,
                None => std::thread::sleep(std::time::Duration::from_millis(50)),
            }
        };
        let rows = self.rows(t.doc)?;
        let sha = hash(t.text.as_bytes());
        if let Some(r) = rows.iter().find(|r| r.epoch == t.epoch && r.kind == t.kind && r.sha256 == sha) {
            return Ok(r.clone());
        }
        let n = rows.iter().filter(|r| r.epoch == t.epoch && r.kind == t.kind).count() + 1;
        let stem = format!("{}-{}-{n}", t.epoch, t.kind);
        let generated = matches!(t.kind, "mova" | "sol");
        let row = TextRow {
            id: format!("{}/{stem}", t.doc),
            doc: t.doc.into(),
            epoch: t.epoch,
            kind: t.kind.into(),
            source: t.source,
            licence_class: licence_class(t.kind, &t.licence).into(),
            licence: t.licence,
            public_ok: !generated && t.open,
            sha256: sha,
            words: crate::contract::word_count(t.text),
            created_unix: now(),
            inputs: t.inputs,
            citations: t.citations,
        };
        let p = self.path(&row)?;
        let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&p).map_err(|e| error(format!("{}: {e}", p.display())))?;
        f.write_all(t.text.as_bytes())?;
        f.sync_all()?;
        let mut j = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(dir.join("texts.jsonl"))?;
        j.write_all((serde_json::to_string(&row)? + "\n").as_bytes())?;
        j.sync_data()?;
        Ok(row)
    }
}

/// A text of a neighbour document with its citation id: by path, or (from the world's CLI) its
/// content as the world read it.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NearText {
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub epoch: u64,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(skip)]
    pub content: Option<String>,
}

impl NearText {
    /// The text: its content, or its file (at most 16 MB).
    pub fn text(&self) -> Option<String> {
        if let Some(c) = &self.content {
            return Some(c.clone());
        }
        let b = crate::util::read_bounded(self.path.as_ref()?, 16 << 20).ok()?;
        String::from_utf8(b).ok()
    }
}

/// A document of the neighbourhood, nearest first.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NearDoc {
    pub doc: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub relations: Vec<String>,
    #[serde(default)]
    pub distance: u32,
    #[serde(default)]
    pub licence: String,
    #[serde(default)]
    pub public_ok: bool,
    #[serde(default)]
    pub texts: Vec<NearText>,
}

/// The neighbourhood answer of the world's CLI or a fixture: `{"documents": [...]}` (or
/// `neighbours`), each document with `doc` (or `id`), `title`, `relations` (or one `relation`),
/// `distance`, `licence`, `public_ok`, `texts: [{id, kind, epoch, path}]`.
pub fn parse_neighbourhood(v: &Value, doc: &str) -> Result<Vec<NearDoc>> {
    let list = v.get("documents").or_else(|| v.get("neighbours")).and_then(Value::as_array).ok_or_else(|| error("world: the neighbourhood has no `documents` list"))?;
    let mut out = vec![];
    let mut seen = BTreeSet::new();
    for d in list {
        let mut d = d.clone();
        if d.get("doc").is_none() {
            if let Some(id) = d.get("id").cloned() {
                d["doc"] = id;
            }
        }
        if d.get("relations").is_none() {
            if let Some(r) = d.get("relation").and_then(Value::as_str) {
                d["relations"] = serde_json::json!([r]);
            }
        }
        let n: NearDoc = serde_json::from_value(d).map_err(|e| error(format!("world: a neighbour: {e}")))?;
        if n.doc != doc && seen.insert(n.doc.clone()) {
            out.push(n);
        }
    }
    // Nearest first; the world's own order breaks ties (never the id).
    out.sort_by_key(|n| n.distance);
    Ok(out)
}

/// Where the neighbourhood comes from.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum WorldConfig {
    /// The global world's CLI (`EXECUTABLE world neighbourhood|related CONFIG DOC … --epoch N`): the
    /// document and its nearest documents with all sentences of their texts of earlier epochs.
    #[serde(rename = "world")]
    World {
        executable: PathBuf,
        config: PathBuf,
        #[serde(default = "d_radius")]
        radius: u32,
        #[serde(default = "d_docs")]
        max_docs: usize,
    },
    /// A paper's closed topic corpus (`mova-topic-corpus build`): `CORPORA_ROOT/<doc>/`.
    #[serde(rename = "topic_corpus")]
    TopicCorpus {
        corpora_root: PathBuf,
        #[serde(default = "d_docs")]
        max_docs: usize,
    },
    /// `DIR/<doc>.json` in the CLI's format (tests).
    #[serde(rename = "fixture")]
    Fixture { dir: PathBuf },
    /// No neighbourhood: the document's own texts only.
    #[serde(rename = "none")]
    None,
}
fn d_radius() -> u32 {
    1
}
fn d_docs() -> usize {
    12
}

impl WorldConfig {
    pub fn validate(&self) -> Result<()> {
        let ok = match self {
            WorldConfig::World { executable, config, radius, max_docs } => executable.is_absolute() && config.is_absolute() && (1..=4).contains(radius) && (1..=1000).contains(max_docs),
            WorldConfig::TopicCorpus { corpora_root, max_docs } => corpora_root.is_absolute() && (1..=1000).contains(max_docs),
            WorldConfig::Fixture { dir } => dir.is_absolute(),
            WorldConfig::None => true,
        };
        if ok {
            Ok(())
        } else {
            Err(error("world: absolute paths, radius 1..4, max_docs 1..1000"))
        }
    }
    /// The document's neighbourhood at `epoch` (what earlier epochs wrote), nearest first; the
    /// world's CLI also gives the document itself (distance 0). A failure stops the epoch (no retry).
    pub fn neighbourhood(&self, doc: &str, epoch: u64) -> Result<Vec<NearDoc>> {
        match self {
            WorldConfig::World { executable, config, radius, max_docs } => world_neighbourhood(executable, config, doc, *radius, *max_docs, epoch),
            WorldConfig::Fixture { dir } => {
                let p = dir.join(format!("{doc}.json"));
                if !p.exists() {
                    return Ok(vec![]);
                }
                parse_neighbourhood(&crate::util::load(&p, 64 << 20)?, doc)
            }
            WorldConfig::TopicCorpus { corpora_root, max_docs } => topic_neighbourhood(corpora_root, doc, *max_docs),
            WorldConfig::None => Ok(vec![]),
        }
    }
}

/// A topic corpus as a neighbourhood: the works the paper cites (bibliography order), similar
/// works (by similarity), works of the same authors, then works citing it (most cited first) —
/// the order of the corpus's `graph.json`; each with its library text from the registry.
fn topic_neighbourhood(root: &Path, doc: &str, max: usize) -> Result<Vec<NearDoc>> {
    if !valid_doc(doc) {
        return Err(error(format!("world: {doc:?} is not a valid document id")));
    }
    let dir = root.join(doc);
    if !dir.join("graph.json").exists() {
        return Ok(vec![]);
    }
    let g = crate::util::load(&dir.join("graph.json"), 256 << 20)?;
    let mut reg: std::collections::BTreeMap<String, Value> = Default::default();
    for l in std::fs::read_to_string(dir.join("documents.jsonl"))?.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(l)?;
        if let Some(id) = v["id"].as_str() {
            reg.insert(id.to_string(), v);
        }
    }
    let mut out: Vec<NearDoc> = vec![];
    for (list, rel) in [("references", "cites"), ("similar", "similar"), ("same_author", "same_author"), ("cited_by", "cited_by")] {
        for el in g[list].as_array().cloned().unwrap_or_default() {
            let Some(id) = el["work"]["id"].as_str().or_else(|| el["id"].as_str()) else { continue };
            if id == doc {
                continue;
            }
            if let Some(n) = out.iter_mut().find(|n| n.doc == id) {
                if !n.relations.iter().any(|r| r == rel) {
                    n.relations.push(rel.into());
                }
                continue;
            }
            let Some(r) = reg.get(&format!("lib:{id}")) else { continue };
            let path = PathBuf::from(r["source"].as_str().unwrap_or(""));
            if r["source_kind"] != "file" || !path.is_absolute() || !path.is_file() || out.len() >= max {
                continue;
            }
            out.push(NearDoc {
                doc: id.into(),
                title: r["title"].as_str().unwrap_or("").into(),
                relations: vec![rel.into()],
                distance: 1,
                licence: r["licence"].as_str().unwrap_or("").into(),
                public_ok: r["public_admissible"] == true,
                texts: vec![NearText { id: format!("lib:{id}"), kind: "original".into(), epoch: 0, path: Some(path), content: None }],
            });
        }
    }
    Ok(out)
}

/// Where the dossiers are: this module's own files (`files`) or the dossier store of the global
/// world's tool (`world`: its CLI `dossier list|text|add CONFIG …`; originals there may be
/// compressed, so texts are read through it).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum DossierConfig {
    #[serde(rename = "files")]
    Files { root: PathBuf },
    #[serde(rename = "world")]
    World { executable: PathBuf, config: PathBuf },
}

impl DossierConfig {
    pub fn validate(&self) -> Result<()> {
        let ok = match self {
            DossierConfig::Files { root } => root.is_absolute(),
            DossierConfig::World { executable, config } => executable.is_absolute() && config.is_absolute(),
        };
        if ok {
            Ok(())
        } else {
            Err(error("dossier: absolute paths"))
        }
    }
    /// The store; `scratch` holds texts handed to the world's CLI for `add`.
    pub fn open(&self, scratch: &Path) -> Store {
        match self {
            DossierConfig::Files { root } => Store::Files(Dossier { root: root.clone() }),
            DossierConfig::World { executable, config } => Store::World { executable: executable.clone(), config: config.clone(), scratch: scratch.to_path_buf() },
        }
    }
}

pub enum Store {
    Files(Dossier),
    World { executable: PathBuf, config: PathBuf, scratch: PathBuf },
}

/// Run a child and return its stdout; a failure is an error carrying its stderr.
fn child(exe: &Path, args: &[String]) -> Result<Vec<u8>> {
    let out = std::process::Command::new(exe).args(args).stdin(std::process::Stdio::null()).output().map_err(|e| error(format!("{}: {e}", exe.display())))?;
    if !out.status.success() {
        return Err(error(format!("{} {}: {}", exe.display(), args.first().map(String::as_str).unwrap_or(""), String::from_utf8_lossy(&out.stderr).trim())));
    }
    Ok(out.stdout)
}

impl Store {
    pub fn rows(&self, doc: &str) -> Result<Vec<TextRow>> {
        match self {
            Store::Files(d) => d.rows(doc),
            Store::World { executable, config, .. } => {
                if !valid_doc(doc) {
                    return Err(error(format!("dossier: {doc:?} is not a valid document id")));
                }
                Ok(serde_json::from_slice(&child(executable, &["dossier".into(), "list".into(), config.display().to_string(), doc.into()])?)?)
            }
        }
    }
    /// A plain file holding the text, when there is one (the `files` store).
    pub fn file(&self, row: &TextRow) -> Option<PathBuf> {
        match self {
            Store::Files(d) => d.path(row).ok(),
            Store::World { .. } => None,
        }
    }
    pub fn read(&self, row: &TextRow) -> Result<String> {
        match self {
            Store::Files(d) => Ok(String::from_utf8(crate::util::read_bounded(&d.path(row)?, 64 << 20)?)?),
            Store::World { executable, config, .. } => Ok(String::from_utf8(child(executable, &["dossier".into(), "text".into(), config.display().to_string(), row.id.clone()])?)?),
        }
    }
    /// Register a text; the same text of the same epoch and kind returns its existing row.
    pub fn register(&self, t: NewText) -> Result<TextRow> {
        match self {
            Store::Files(d) => d.register(t),
            Store::World { executable, config, scratch } => {
                let sha = hash(t.text.as_bytes());
                if let Some(r) = self.rows(t.doc)?.into_iter().find(|r| r.epoch == t.epoch && r.kind == t.kind && r.sha256 == sha) {
                    return Ok(r);
                }
                crate::util::private_dir(scratch)?;
                let f = scratch.join(format!("{}-{}-{}-{}.md", t.doc, t.epoch, t.kind, crate::util::prefix(&sha, 12)));
                crate::util::write_sync(&f, t.text.as_bytes())?;
                let mut a: Vec<String> = ["dossier", "add", &config.display().to_string(), t.doc, "--epoch", &t.epoch.to_string(), "--kind", t.kind, "--file", &f.display().to_string(), "--source", &t.source].iter().map(|s| s.to_string()).collect();
                if !t.licence.is_empty() {
                    a.extend(["--licence".to_string(), t.licence.clone()]);
                }
                for i in &t.inputs {
                    a.extend(["--input".to_string(), i.clone()]);
                }
                let row: TextRow = serde_json::from_slice(&child(executable, &a)?)?;
                if matches!(t.kind, "mova" | "sol") && row.public_ok {
                    return Err(error(format!("dossier: {} was registered as public", row.id)));
                }
                Ok(row)
            }
        }
    }
}

/// The world CLI's neighbourhood at `epoch` (texts of earlier epochs): the document itself
/// (distance 0) and its nearest documents with every sentence of their visible texts, each text
/// rebuilt as Markdown under one part heading; relations from the direct edges (`related`).
fn world_neighbourhood(exe: &Path, config: &Path, doc: &str, radius: u32, max_docs: usize, epoch: u64) -> Result<Vec<NearDoc>> {
    let c = config.display().to_string();
    let (r, m, e) = (radius.to_string(), max_docs.to_string(), epoch.to_string());
    let base = |extra: &[&str]| -> Vec<String> { ["world", "neighbourhood", &c, doc, "--radius", &r, "--max-docs", &m, "--epoch", &e].iter().chain(extra.iter()).map(|s| s.to_string()).collect() };
    let meta: Value = serde_json::from_slice(&child(exe, &base(&[]))?)?;
    let rel: Value = serde_json::from_slice(&child(exe, &["world".into(), "related".into(), c.clone(), doc.into(), m.clone(), "--epoch".into(), e.clone()])?)?;
    let content = child(exe, &base(&["--content"]))?;
    let mut sentences: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for l in String::from_utf8(content)?.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(l)?;
        if let (Some(t), Some(s)) = (v["text"].as_str(), v["sentence"].as_str()) {
            sentences.entry(t.to_string()).or_default().push(s.to_string());
        }
    }
    let edges: std::collections::BTreeMap<String, Vec<String>> = rel["related"].as_array().cloned().unwrap_or_default().iter().filter_map(|x| Some((x["doc"].as_str()?.to_string(), x["edges"].as_array()?.iter().filter_map(|e| e["kind"].as_str().map(String::from)).collect()))).collect();
    let mut out = vec![];
    for d in meta["docs"].as_array().cloned().unwrap_or_default() {
        let Some(id) = d["doc"].as_str() else { continue };
        let title = d["title"].as_str().unwrap_or(id).to_string();
        let texts = d["texts"].as_array().cloned().unwrap_or_default().iter().filter_map(|t| {
            let tid = t["text"].as_str()?.to_string();
            let body = sentences.get(&tid)?;
            let md = format!("# {title}\n\n## Text\n\n{}\n", body.chunks(5).map(|c| c.join(" ")).collect::<Vec<_>>().join("\n\n"));
            Some(NearText { id: tid, kind: t["kind"].as_str().unwrap_or("").into(), epoch: t["epoch"].as_u64().unwrap_or(0), path: None, content: Some(md) })
        }).collect();
        let relations = edges.get(id).cloned().unwrap_or_else(|| vec![if id == doc { "self".into() } else { "near".into() }]);
        out.push(NearDoc { doc: id.into(), title, relations, distance: d["distance"].as_u64().unwrap_or(0) as u32, licence: String::new(), public_ok: false, texts });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("mova-dossier-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn texts_are_registered_once_and_generated_texts_are_never_public() {
        let d = Dossier { root: tmp("reg") };
        let t = |kind: &'static str, text: &'static str, open: bool| NewText { doc: "pa-2024-x", epoch: 1, kind, text, source: "test".into(), licence: "CC BY 4.0".into(), open, inputs: vec![], citations: Citations::default() };
        let a = d.register(t("mova", "One text here.", true)).unwrap();
        assert_eq!(a.id, "pa-2024-x/1-mova-1");
        assert!(!a.public_ok && a.licence_class == "internal");
        // The same text again: the same row, no second file.
        assert_eq!(d.register(t("mova", "One text here.", true)).unwrap(), a);
        let b = d.register(t("mova", "Another text.", true)).unwrap();
        assert_eq!(b.id, "pa-2024-x/1-mova-2");
        // A consultant's text is never public, even from open inputs (negative control: an
        // original with the same flag is).
        assert!(!d.register(t("sol", "Sol wrote this.", true)).unwrap().public_ok);
        assert!(d.register(t("original", "The paper.", true)).unwrap().public_ok);
        assert_eq!(d.rows("pa-2024-x").unwrap().len(), 4);
        assert_eq!(std::fs::read_to_string(d.path(&b).unwrap()).unwrap(), "Another text.");
        assert!(d.register(t("blog", "x", true)).is_err());
        assert!(d.doc_dir("../etc").is_err());
        let _ = std::fs::remove_dir_all(&d.root);
    }

    #[test]
    fn a_neighbourhood_is_nearest_first_without_the_document() {
        let v = serde_json::json!({"documents":[
            {"doc":"far","distance":2,"relation":"similar","texts":[]},
            {"id":"near","distance":1,"relations":["cites"],"texts":[{"id":"lib:near","path":"/x/near.md"}]},
            {"doc":"self","distance":0,"texts":[]}
        ]});
        let n = parse_neighbourhood(&v, "self").unwrap();
        assert_eq!(n.iter().map(|x| x.doc.as_str()).collect::<Vec<_>>(), vec!["near", "far"]);
        assert_eq!(n[1].relations, vec!["similar".to_string()]);
    }
}
