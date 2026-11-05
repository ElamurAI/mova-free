//! Export of the Whales' finished descriptions into an editorial texts repository
//! (`mova-whale-texts-v1`).
//!
//! For each paper whose learner passed its gates in a batch at or after `from_batch` (read from the
//! batch log and each learner's `status.json`), the exporter writes under `<repo>/<dir>/`:
//!
//! - `<paper>/index.md`: the main article, the last gate-passed round's description with a source
//!   marker after each run of sentences that share their sources (from the round's `draft.json` and
//!   the learners' knowledge provenance) and a bibliography of footnotes: the described paper, the
//!   consultation notes, library neighbours and the paper's own references that the text names;
//!   pages and originals held in the dossier are listed by URL and licence, never copied;
//! - `<paper>/seo.json`: title and meta description;
//! - `<paper>/shadow/<epoch>-<kind>-<n>/index.md`: the dossier's Mova drafts, consultant drafts and
//!   earlier descriptions (one per distinct text, the main article's own text excluded);
//! - `index.md`: the list of exported papers; `export.json`: the exporter's manifest.
//!
//! Every Markdown file starts with front matter that states visibility and licence and carries
//! `export_sha256`, the hash of the file with that value zeroed. A file whose hash no longer
//! matches (edited by a person) or that the exporter did not write is never overwritten: it is
//! skipped and reported. `seo.json` is checked against the manifest the same way. Output is a pure
//! function of the inputs (no clock, sorted maps), and a file is written only when its content
//! changes, so a second run changes nothing. Nothing is deleted.
use crate::config::Config as WorldConfig;
use crate::dossier::{Dossier, Kind, TextRow};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use topic_corpus::library::{Library, RefEntry, Work};

pub const SCHEMA: &str = "mova-whale-texts-v1";
const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const HASH_KEY: &str = "export_sha256: ";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: String,
    /// The `mova-whale-world-v1` configuration (dossiers, library).
    pub world: PathBuf,
    /// The batch log (`batches.jsonl`): `opened` events with their assignments.
    pub batches: PathBuf,
    /// Learner name → its state root (`papers/<paper>/round-NNN/`, `knowledge/items.jsonl`).
    pub learners: BTreeMap<String, PathBuf>,
    /// The first batch whose passed papers are exported.
    #[serde(default)]
    pub from_batch: u64,
    /// The directory inside the repository (a plain name).
    pub dir: String,
    /// Who wrote the descriptions, as stated in the files.
    pub describer: String,
    /// Visibility stated in every file (for example `internal`).
    pub visibility: String,
    /// Licence statement of the exported texts.
    pub licence: String,
}

impl Config {
    pub fn load(p: &Path) -> Result<Config> {
        let b = std::fs::read(p).with_context(|| format!("config {}", p.display()))?;
        let c: Config = serde_json::from_slice(&b).with_context(|| format!("config {}", p.display()))?;
        c.validate()?;
        Ok(c)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA {
            bail!("schema must be {SCHEMA}");
        }
        let mut paths: Vec<&Path> = vec![&self.world, &self.batches];
        paths.extend(self.learners.values().map(PathBuf::as_path));
        if let Some(p) = paths.iter().find(|p| !p.is_absolute()) {
            bail!("every path must be absolute: {}", p.display());
        }
        if self.dir.is_empty() || self.dir.starts_with('.') || !self.dir.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!("dir must be a plain lowercase name");
        }
        if self.learners.is_empty() || self.visibility.trim().is_empty() || self.licence.trim().is_empty() || self.describer.trim().is_empty() {
            bail!("learners, describer, visibility and licence are required");
        }
        Ok(())
    }
}

/// A passed paper chosen for export.
#[derive(Clone, Debug, PartialEq)]
pub struct Pick {
    pub paper: String,
    pub whale: String,
    pub batch: u64,
    pub title: String,
    pub licence: String,
    /// The last round whose gates passed.
    pub round: PathBuf,
}

/// Papers assigned in batches `>= from_batch` (a later assignment of the same paper wins) whose
/// learner state says `passed` and holds a passing round with a description.
pub fn passed(cfg: &Config) -> Result<Vec<Pick>> {
    let mut assigned: BTreeMap<String, (u64, String, String, String)> = BTreeMap::new();
    let log = std::fs::read_to_string(&cfg.batches).with_context(|| format!("{}", cfg.batches.display()))?;
    for l in log.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(l).with_context(|| format!("{}", cfg.batches.display()))?;
        let batch = v["batch"].as_u64().unwrap_or(0);
        if v["event"] != "opened" || batch < cfg.from_batch {
            continue;
        }
        for a in v["assignments"].as_array().into_iter().flatten() {
            let (Some(id), Some(whale)) = (a["paper"]["id"].as_str(), a["whale"].as_str()) else { continue };
            let s = |k: &str| a["paper"][k].as_str().unwrap_or_default().to_string();
            assigned.insert(id.to_string(), (batch, whale.to_string(), s("title"), s("licence")));
        }
    }
    let mut out = vec![];
    for (paper, (batch, whale, title, licence)) in assigned {
        let Some(root) = cfg.learners.get(&whale) else { continue };
        let pd = root.join("papers").join(&paper);
        let status: Value = match std::fs::read(pd.join("status.json")) {
            Ok(b) => serde_json::from_slice(&b).with_context(|| format!("{}/status.json", pd.display()))?,
            Err(_) => continue,
        };
        if status["state"] != "passed" {
            continue;
        }
        if let Some(round) = best_round(&pd)? {
            out.push(Pick { paper, whale, batch, title, licence, round });
        }
    }
    Ok(out)
}

/// The last `round-NNN` whose `gates.json` passed and that holds `description.txt` and `draft.json`.
pub fn best_round(paper_dir: &Path) -> Result<Option<PathBuf>> {
    let mut rounds: Vec<PathBuf> = match std::fs::read_dir(paper_dir) {
        Ok(rd) => rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("round-"))).collect(),
        Err(_) => return Ok(None),
    };
    rounds.sort();
    for r in rounds.into_iter().rev() {
        let Ok(g) = std::fs::read(r.join("gates.json")) else { continue };
        let g: Value = serde_json::from_slice(&g).with_context(|| format!("{}/gates.json", r.display()))?;
        if g["pass"] == true && r.join("description.txt").is_file() && r.join("draft.json").is_file() {
            return Ok(Some(r));
        }
    }
    Ok(None)
}

#[derive(Clone, Debug, Deserialize)]
struct Item {
    text: String,
    kind: String,
    #[serde(default)]
    sources: Vec<usize>,
    #[serde(default)]
    knowledge: Vec<String>,
    #[serde(default)]
    section: Option<usize>,
}

/// Knowledge provenance per learner, read lazily.
struct Provenance<'a> {
    learners: &'a BTreeMap<String, PathBuf>,
    items: BTreeMap<String, BTreeMap<String, Value>>,
}

impl Provenance<'_> {
    fn get(&mut self, whale: &str, id: &str) -> Result<Value> {
        if !self.items.contains_key(whale) {
            let mut m = BTreeMap::new();
            if let Some(root) = self.learners.get(whale) {
                let p = root.join("knowledge/items.jsonl");
                if p.is_file() {
                    for l in std::fs::read_to_string(&p)?.lines().filter(|l| !l.trim().is_empty()) {
                        let v: Value = serde_json::from_str(l).with_context(|| format!("{}", p.display()))?;
                        if let Some(k) = v["id"].as_str() {
                            m.insert(k.to_string(), v["provenance"].clone());
                        }
                    }
                }
            }
            self.items.insert(whale.to_string(), m);
        }
        Ok(self.items[whale].get(id).cloned().unwrap_or(Value::Null))
    }

    /// The source id of a knowledge item used by `whale`: `paper`, `consult:<whale>` or a corpus
    /// document id. Pod items (`pod:<whale>:<id>`) are resolved in that learner's knowledge.
    fn source(&mut self, whale: &str, key: &str) -> Result<String> {
        if key.starts_with("p-") {
            return Ok("paper".into());
        }
        let (owner, id) = match key.strip_prefix("pod:").and_then(|r| r.split_once(':')) {
            Some((w, k)) => (w.to_string(), k.to_string()),
            None => (whale.to_string(), key.to_string()),
        };
        let p = self.get(&owner, &id)?;
        Ok(match p["source"].as_str() {
            Some("topic-corpus") => p["doc"].as_str().map(str::to_string).unwrap_or_else(|| "paper".into()),
            Some("terminal") | Some("reread") => format!("consult:{owner}"),
            _ => "paper".into(),
        })
    }
}

/// `Surname … YYYY` mentions in a sentence that match an entry of the paper's reference list
/// (same surname within the four words before the year); indices into `refs`.
pub fn ref_mentions(sentence: &str, refs: &[RefEntry]) -> Vec<usize> {
    let words: Vec<&str> = sentence.split(|c: char| !(c.is_alphanumeric() || c == '-' || c == '\'')).filter(|w| !w.is_empty()).collect();
    let mut out = BTreeSet::new();
    for (i, w) in words.iter().enumerate() {
        let y = w.trim_end_matches(|c: char| c.is_ascii_lowercase());
        if y.len() != 4 || !y.chars().all(|c| c.is_ascii_digit()) || !(y.starts_with("19") || y.starts_with("20")) {
            continue;
        }
        let year: i64 = y.parse().unwrap_or(0);
        let before = &words[i.saturating_sub(4)..i];
        for (k, r) in refs.iter().enumerate() {
            if r.year == Some(year) && r.title.is_some() && r.author.as_deref().is_some_and(|a| a.chars().count() >= 2 && before.contains(&a)) {
                out.insert(k);
            }
        }
    }
    out.into_iter().collect()
}

fn who(authors: &[String]) -> String {
    match authors.len() {
        0 => String::new(),
        1 => authors[0].clone(),
        2..=5 => format!("{} and {}", authors[..authors.len() - 1].join(", "), authors[authors.len() - 1]),
        _ => format!("{} et al.", authors[0]),
    }
}

fn link(doi: Option<&str>, acl: Option<&str>, arxiv: Option<&str>) -> Option<String> {
    if let Some(x) = doi.filter(|x| !x.is_empty()) {
        Some(format!("https://doi.org/{}", topic_corpus::library::bare_doi(x)))
    } else if let Some(x) = acl.filter(|x| !x.is_empty()) {
        Some(format!("https://aclanthology.org/{x}/"))
    } else {
        arxiv.filter(|x| !x.is_empty()).map(|x| format!("https://arxiv.org/abs/{x}"))
    }
}

/// A bibliography entry for a library work.
pub fn work_entry(w: &Work) -> String {
    let mut s = String::new();
    let a = who(&w.authors);
    if !a.is_empty() {
        s.push_str(&a);
        s.push(' ');
    }
    if let Some(y) = w.year {
        s.push_str(&format!("({y}). "));
    }
    let title = w.title.trim().trim_end_matches('.');
    match link(w.doi.as_deref(), w.acl.as_deref(), w.arxiv.as_deref()) {
        Some(l) => s.push_str(&format!("[{title}]({l}).")),
        None => s.push_str(&format!("{title}.")),
    }
    if !w.licence.is_empty() {
        s.push_str(&format!(" Licence: {}.", w.licence));
    }
    s
}

fn ref_entry(r: &RefEntry, lib: Option<&Library>) -> String {
    if let Some(w) = lib.and_then(|l| l.resolve(r)) {
        return format!("{} Cited by the described paper.", work_entry(w));
    }
    let mut s = format!("{} ({}). {}.", r.author.as_deref().unwrap_or("?"), r.year.map(|y| y.to_string()).unwrap_or_default(), r.title.as_deref().unwrap_or("").trim().trim_end_matches('.'));
    if let Some(l) = link(r.doi.as_deref(), r.acl.as_deref(), r.arxiv.as_deref()).or_else(|| r.url.clone()) {
        s.push_str(&format!(" {l}"));
    }
    s.push_str(" Cited by the described paper (reference as extracted from its text).");
    s
}

/// Metadata of the described paper: the library work when present, else the batch log's fields.
#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<i64>,
    pub licence: String,
    pub entry: String,
}

fn meta(p: &Pick, lib: Option<&Library>) -> Meta {
    match lib.and_then(|l| l.get(&p.paper)) {
        Some(w) => Meta { title: w.title.clone(), authors: w.authors.clone(), year: w.year, licence: w.licence.clone(), entry: work_entry(w) },
        None => Meta {
            title: p.title.clone(),
            authors: vec![],
            year: None,
            licence: p.licence.clone(),
            entry: format!("{}. Library id `{}`.{}", p.title.trim_end_matches('.'), p.paper, if p.licence.is_empty() { String::new() } else { format!(" Licence: {}.", p.licence) }),
        },
    }
}

fn label(id: &str) -> String {
    let s: String = id.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c.to_ascii_lowercase() } else { '-' }).collect();
    s.trim_matches('-').to_string()
}

/// The cited article body: paragraphs with footnote markers and the footnote definitions.
pub struct Cited {
    pub body: String,
    pub notes: Vec<(String, String)>,
    /// `sentence` when the draft reproduces the description exactly, else `paragraph`.
    pub mode: &'static str,
    pub words: usize,
}

/// Cite one round. `refs` is the described paper's reference list.
pub fn cite(p: &Pick, m: &Meta, learners: &BTreeMap<String, PathBuf>, refs: &[RefEntry], lib: Option<&Library>) -> Result<Cited> {
    let description = std::fs::read_to_string(p.round.join("description.txt"))?;
    let draft: Value = serde_json::from_slice(&std::fs::read(p.round.join("draft.json"))?).with_context(|| format!("{}/draft.json", p.round.display()))?;
    let items: Vec<Item> = serde_json::from_value(draft["items"].clone()).context("draft.json items")?;
    let mut prov = Provenance { learners, items: BTreeMap::new() };
    // Paragraphs: runs of items of the same section, in draft order (as the description is joined).
    let mut paras: Vec<Vec<(String, Vec<String>)>> = vec![];
    let mut last: Option<Option<usize>> = None;
    for it in &items {
        let mut ids: Vec<String> = vec![];
        if matches!(it.kind.as_str(), "source" | "reading" | "structure") || !it.sources.is_empty() {
            ids.push("paper".into());
        }
        for k in &it.knowledge {
            let id = prov.source(&p.whale, k)?;
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        for k in ref_mentions(&it.text, refs) {
            ids.push(format!("ref:{k}"));
        }
        if ids.is_empty() {
            ids.push("paper".into());
        }
        if last != Some(it.section) {
            paras.push(vec![]);
            last = Some(it.section);
        }
        paras.last_mut().unwrap().push((it.text.trim().to_string(), ids));
    }
    let plain = paras.iter().map(|ps| ps.iter().map(|x| x.0.as_str()).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n\n");
    if plain.trim() != description.trim() {
        // The draft does not reproduce the description: cite the paper once per paragraph.
        paras = description.trim().split("\n\n").map(|t| vec![(t.trim().to_string(), vec!["paper".to_string()])]).collect();
    }
    let mode = if plain.trim() == description.trim() { "sentence" } else { "paragraph" };
    let mut order: Vec<String> = vec![];
    let mut body = String::new();
    let mut words = 0;
    for ps in &paras {
        let mut out: Vec<String> = vec![];
        for (i, (text, ids)) in ps.iter().enumerate() {
            words += text.split_whitespace().count();
            let mut sorted = ids.clone();
            sorted.sort();
            let next_same = ps.get(i + 1).is_some_and(|(_, n)| {
                let mut n = n.clone();
                n.sort();
                n == sorted
            });
            if next_same {
                out.push(text.clone());
                continue;
            }
            let mut marks = String::new();
            for id in ids {
                if !order.contains(id) {
                    order.push(id.clone());
                }
                marks.push_str(&format!("[^{}]", note_label(id)));
            }
            let t = text.trim_end();
            let ends = t.ends_with(['.', '?', '!']);
            out.push(format!("{t}{}{marks}", if ends { "" } else { "." }));
        }
        body.push_str(&out.join(" "));
        body.push_str("\n\n");
    }
    let mut notes = vec![];
    for id in &order {
        let entry = if id == "paper" {
            format!("{} The described paper; cited, not copied.", m.entry)
        } else if let Some(w) = id.strip_prefix("consult:") {
            format!("Notes from the consultation of «{w}» with its consultant (knowledge the describers share): internal working material, not a published source; used for explanations of terms only.")
        } else if let Some(k) = id.strip_prefix("ref:") {
            ref_entry(&refs[k.parse::<usize>()?], lib)
        } else if let Some(w) = id.strip_prefix("lib:").and_then(|x| lib.and_then(|l| l.get(x))) {
            work_entry(w)
        } else {
            format!("Document `{id}` of the paper's topic corpus (internal; cited, not copied).")
        };
        notes.push((note_label(id), entry));
    }
    Ok(Cited { body, notes, mode, words })
}

fn note_label(id: &str) -> String {
    if id == "paper" {
        "paper".into()
    } else if let Some(k) = id.strip_prefix("ref:") {
        format!("ref-{k}")
    } else {
        label(id)
    }
}

/// Front matter from ordered `(key, value)` pairs; strings are JSON-quoted (valid YAML).
fn front(fields: &[(&str, Value)]) -> String {
    let mut s = String::from("---\n");
    for (k, v) in fields {
        s.push_str(&format!("{k}: {}\n", serde_json::to_string(v).unwrap()));
    }
    s.push_str(&format!("{HASH_KEY}{ZERO}\n---\n\n"));
    s
}

/// Fill `export_sha256` with the hash of the file whose hash value is zero.
pub fn seal(text: &str) -> String {
    let h = crate::dossier::sha(text.as_bytes());
    text.replacen(&format!("{HASH_KEY}{ZERO}"), &format!("{HASH_KEY}{h}"), 1)
}

/// Whether a Markdown file still carries the content the exporter sealed into it.
pub fn intact(text: &str) -> bool {
    let Some(fm) = text.strip_prefix("---\n").and_then(|r| r.split_once("\n---\n")).map(|x| x.0) else { return false };
    let Some(line) = fm.lines().find(|l| l.starts_with(HASH_KEY)) else { return false };
    let h = &line[HASH_KEY.len()..];
    if h.len() != 64 || h == ZERO {
        return false;
    }
    let zeroed = text.replacen(line, &format!("{HASH_KEY}{ZERO}"), 1);
    crate::dossier::sha(zeroed.as_bytes()) == h
}

/// What happened to the files of one run.
#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub written: Vec<String>,
    pub unchanged: usize,
    /// Edited by a person (hash mismatch): kept, not overwritten.
    pub edited: Vec<String>,
    /// Present but not written by the exporter: kept, not overwritten.
    pub foreign: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub schema: String,
    /// Exported papers: id → their index entry.
    pub papers: BTreeMap<String, Value>,
    /// Every file the exporter wrote (relative to the repository) → its sha256 as written.
    pub files: BTreeMap<String, String>,
}

struct Out<'a> {
    repo: &'a Path,
    old: &'a Manifest,
    new: Manifest,
    report: Report,
}

impl Out<'_> {
    /// Write `rel` unless unchanged; refuse to overwrite a file edited by a person or not ours.
    fn put(&mut self, rel: &str, content: &str, sealed: bool) -> Result<()> {
        let p = self.repo.join(rel);
        let new_sha = crate::dossier::sha(content.as_bytes());
        if let Ok(cur) = std::fs::read(&p) {
            let cur_sha = crate::dossier::sha(&cur);
            if cur_sha == new_sha {
                self.report.unchanged += 1;
                self.new.files.insert(rel.into(), new_sha);
                return Ok(());
            }
            let ours = match self.old.files.get(rel) {
                Some(s) => *s == cur_sha && (!sealed || intact(&String::from_utf8_lossy(&cur))),
                None => sealed && intact(&String::from_utf8_lossy(&cur)),
            };
            if !ours {
                let known = self.old.files.contains_key(rel) || (sealed && String::from_utf8_lossy(&cur).contains(HASH_KEY));
                if known {
                    self.report.edited.push(rel.into());
                } else {
                    self.report.foreign.push(rel.into());
                }
                if let Some(s) = self.old.files.get(rel) {
                    self.new.files.insert(rel.into(), s.clone());
                }
                return Ok(());
            }
        }
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d)?;
        }
        let tmp = p.with_file_name(format!(".{}.export-tmp", p.file_name().unwrap().to_string_lossy()));
        std::fs::write(&tmp, content)?;
        std::fs::rename(&tmp, &p)?;
        self.report.written.push(rel.into());
        self.new.files.insert(rel.into(), new_sha);
        Ok(())
    }
}

/// A source as stated in the texts repository: a URL as it is, a local path by its file name only
/// (machine paths stay in the dossier row, named by its text id).
fn public_source(s: &str) -> String {
    if s.starts_with('/') {
        Path::new(s).file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default()
    } else {
        s.to_string()
    }
}

fn kind_label(k: Kind) -> &'static str {
    match k {
        Kind::Mova => "Mova draft",
        Kind::Sol => "consultant draft",
        Kind::SiteOld => "earlier description",
        Kind::Web => "web page",
        Kind::Original => "original",
    }
}

fn year_s(y: Option<i64>) -> String {
    y.map(|y| y.to_string()).unwrap_or_else(|| "n.d.".into())
}

/// Export the passed papers (all, or only `only`) into `repo/<dir>/`. `lib` gives titles, authors,
/// links and reference lists; without it the batch log's title and licence are used.
pub fn export(cfg: &Config, wcfg: &WorldConfig, lib: Option<&Library>, repo: &Path, only: &[String]) -> Result<Value> {
    if !repo.is_dir() {
        bail!("{} is not a directory", repo.display());
    }
    let mut picks = passed(cfg)?;
    let mut not_passed = vec![];
    if !only.is_empty() {
        for d in only {
            if !picks.iter().any(|p| &p.paper == d) {
                not_passed.push(d.clone());
            }
        }
        picks.retain(|p| only.contains(&p.paper));
    }
    let man_rel = format!("{}/export.json", cfg.dir);
    let old: Manifest = match std::fs::read(repo.join(&man_rel)) {
        Ok(b) => serde_json::from_slice(&b).with_context(|| man_rel.clone())?,
        Err(_) => Manifest { schema: SCHEMA.into(), ..Default::default() },
    };
    let mut out = Out { repo, old: &old, new: Manifest { schema: SCHEMA.into(), papers: old.papers.clone(), files: BTreeMap::new() }, report: Report::default() };
    let mut per = vec![];
    for p in &picks {
        let m = meta(p, lib);
        let refs = match lib.and_then(|l| l.get(&p.paper).map(|w| (l, w))) {
            Some((l, w)) => l.bibliography(w)?,
            None => vec![],
        };
        let c = cite(p, &m, &cfg.learners, &refs, lib)?;
        let description = std::fs::read(p.round.join("description.txt"))?;
        let desc_sha = crate::dossier::sha(&description);
        let draft_sha = crate::dossier::sha(&std::fs::read(p.round.join("draft.json"))?);
        let gates: Value = serde_json::from_slice(&std::fs::read(p.round.join("gates.json"))?)?;
        let status: Value = serde_json::from_slice(&std::fs::read(p.round.parent().unwrap().join("status.json"))?)?;
        let round_name = p.round.file_name().unwrap().to_string_lossy().to_string();
        let round_no: u64 = round_name.trim_start_matches("round-").parse().unwrap_or(0);
        let byline = if m.authors.is_empty() { String::new() } else { format!("{} ", who(&m.authors)) };
        // Dossier texts: shadows (ours) and held sources (cited, never copied).
        let mut shadows: Vec<(TextRow, String)> = vec![];
        let mut held: Vec<TextRow> = vec![];
        if Dossier::exists(wcfg, &p.paper) {
            let d = Dossier::open(wcfg, &p.paper)?;
            let mut seen: BTreeSet<String> = BTreeSet::from([desc_sha.clone()]);
            for r in d.rows()? {
                match r.kind {
                    Kind::Original | Kind::Web => held.push(r),
                    _ => {
                        if seen.insert(r.sha256.clone()) {
                            let t = d.read(&r)?;
                            shadows.push((r, t));
                        }
                    }
                }
            }
        }
        let base = format!("{}/{}", cfg.dir, p.paper);
        // Main article.
        let mut md = front(&[
            ("id", json!(base)),
            ("kind", json!("main")),
            ("title", json!(m.title)),
            ("paper", json!(p.paper)),
            ("authors", json!(m.authors)),
            ("year", json!(m.year)),
            ("paper_licence", json!(m.licence)),
            ("visibility", json!(cfg.visibility)),
            ("licence", json!(cfg.licence)),
            ("describer", json!(format!("{} ({})", cfg.describer, p.whale))),
            ("writer", json!(if status["last_round"]["round"].as_u64() == Some(round_no) { status["last_round"]["writer"].as_str() } else { None })),
            ("batch", json!(p.batch)),
            ("round", json!(round_no)),
            ("gates", json!({"pass": gates["pass"], "words": gates["words"], "words_min": gates["words_min"], "words_max": gates["words_max"], "traceability": gates["traceability"], "verbatim_max": gates["verbatim_max"]})),
            ("description_sha256", json!(desc_sha)),
            ("draft_sha256", json!(draft_sha)),
            ("citations", json!(c.mode)),
            ("words", json!(c.words)),
            ("generator", json!(format!("mova-whale-world export-texts ({SCHEMA})"))),
        ]);
        md.push_str(&format!("# {}\n\n", m.title.trim()));
        md.push_str(&format!("*A description of {}({}), written by Mova. Internal and closed for now.*\n\n", byline, year_s(m.year)));
        md.push_str(&c.body);
        if !held.is_empty() {
            md.push_str("## Sources held for this paper\n\nCited, not copied: the texts stay in the machine store.\n\n");
            for r in &held {
                md.push_str(&format!("- {} ({}): {}. Licence: {}.\n", kind_label(r.kind), r.id, if r.source.starts_with("http") { r.source.clone() } else { format!("library copy `{}`", public_source(&r.source)) }, r.licence));
            }
            md.push('\n');
        }
        md.push_str("## Bibliography\n\n");
        for (l, e) in &c.notes {
            md.push_str(&format!("[^{l}]: {e}\n"));
        }
        out.put(&format!("{base}/index.md"), &seal(&md), true)?;
        // SEO metadata.
        let seo = json!({
            "title": format!("{}: a paper description", m.title.trim().trim_end_matches('.')),
            "description": format!("A description of «{}»{} ({}): its question, method, results and terms, with sources.", m.title.trim().trim_end_matches('.'), if m.authors.is_empty() { String::new() } else { format!(" by {}", who(&m.authors)) }, year_s(m.year)),
        });
        out.put(&format!("{base}/seo.json"), &(serde_json::to_string_pretty(&seo)? + "\n"), false)?;
        // Shadows.
        let mut shadow_ids = vec![];
        for (r, t) in &shadows {
            let stem = r.id.rsplit('/').next().unwrap_or(&r.id).to_string();
            let mut s = front(&[
                ("id", json!(format!("{base}/shadow/{stem}"))),
                ("kind", json!("shadow")),
                ("paper", json!(p.paper)),
                ("text_id", json!(r.id)),
                ("text_kind", json!(r.kind.name())),
                ("epoch", json!(r.epoch)),
                ("source", json!(public_source(&r.source))),
                ("text_licence", json!(r.licence)),
                ("visibility", json!(cfg.visibility)),
                ("licence", json!(cfg.licence)),
                ("sha256", json!(r.sha256)),
                ("words", json!(r.words)),
                ("generator", json!(format!("mova-whale-world export-texts ({SCHEMA})"))),
            ]);
            // The text's own title stays the page heading; the shadow label follows it.
            let t = t.trim();
            let (h1, rest) = match t.strip_prefix("# ") {
                Some(r) => r.split_once('\n').map(|(h, b)| (h.trim().to_string(), b.trim_start())).unwrap_or((r.trim().to_string(), "")),
                None => (m.title.trim().to_string(), t),
            };
            s.push_str(&format!("# {h1}\n\n*Shadow: {}, epoch {} (`{}`). Internal and closed for now.*\n\n", kind_label(r.kind), r.epoch, r.id));
            s.push_str(rest.trim_end());
            s.push('\n');
            out.put(&format!("{base}/shadow/{stem}/index.md"), &seal(&s), true)?;
            shadow_ids.push(stem);
        }
        let entry = json!({"title": m.title, "authors": who(&m.authors), "year": m.year, "words": c.words, "batch": p.batch, "whale": p.whale, "round": round_no, "shadows": shadow_ids, "held": held.len(), "citations": c.mode, "notes": c.notes.len()});
        out.new.papers.insert(p.paper.clone(), entry.clone());
        per.push(json!({"paper": p.paper, "words": c.words, "batch": p.batch, "whale": p.whale, "round": round_no, "shadows": shadow_ids.len(), "notes": c.notes.len(), "citations": c.mode}));
    }
    // The index page over every paper exported so far.
    let mut idx = front(&[
        ("id", json!(format!("{}/index", cfg.dir))),
        ("kind", json!("index")),
        ("visibility", json!(cfg.visibility)),
        ("licence", json!(cfg.licence)),
        ("papers", json!(out.new.papers.len())),
        ("generator", json!(format!("mova-whale-world export-texts ({SCHEMA})"))),
    ]);
    idx.push_str(&format!("# Paper descriptions\n\nDescriptions of scientific papers by {}. Internal and closed for now: not for publication. Each paper has its main description (`<paper>/index.md`, gate-passed, with its sources), its metadata (`seo.json`) and its shadows (`<paper>/shadow/<epoch>-<kind>-<n>/`: Mova drafts, consultant drafts and earlier descriptions). Papers and pages the descriptions rely on are cited, never copied.\n\n", cfg.describer));
    idx.push_str("| Paper | Authors | Year | Words | Batch | Describer | Shadows |\n|---|---|---|---:|---:|---|---:|\n");
    for (id, e) in &out.new.papers {
        let cell = |s: &str| s.replace('|', "\\|");
        idx.push_str(&format!(
            "| [{}]({id}/index.md) | {} | {} | {} | {} | {} | {} |\n",
            cell(e["title"].as_str().unwrap_or(id)),
            cell(e["authors"].as_str().unwrap_or("")),
            e["year"].as_i64().map(|y| y.to_string()).unwrap_or_default(),
            e["words"],
            e["batch"],
            e["whale"].as_str().unwrap_or(""),
            e["shadows"].as_array().map(|a| a.len()).unwrap_or(0)
        ));
    }
    out.put(&format!("{}/index.md", cfg.dir), &seal(&idx), true)?;
    // The manifest last (machine-owned; rewritten when it changes).
    let man = serde_json::to_string_pretty(&out.new)? + "\n";
    let mp = repo.join(&man_rel);
    if std::fs::read(&mp).ok().as_deref() != Some(man.as_bytes()) {
        std::fs::create_dir_all(mp.parent().unwrap())?;
        std::fs::write(&mp, &man)?;
        out.report.written.push(man_rel);
    } else {
        out.report.unchanged += 1;
    }
    Ok(json!({"papers": per, "exported": per.len(), "not_passed": not_passed, "report": out.report}))
}

#[cfg(test)]
mod tests;
