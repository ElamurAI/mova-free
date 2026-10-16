//! Lossless loading of CoNLL-U into the database: the file is read by `en::conllu::Doc` (comments, multiword tokens,
//! empty nodes, DEPS, MISC, blank lines), Rust prepares TSV, `sql/load.sql` moves them in one
//! transaction. Row numbers (run, file, doc, sid, aid) are assigned by Rust, starting from the largest in the database.
//!
//! Sentences:
//! - `new` — every block of the file is a new sentence (treebanks, fairy tales);
//! - `auto` — a block with the same `sent_id` and `text` as one already in the database lands on that sentence,
//!   otherwise a new one (Mova corpus: silver lands on bronze);
//! - `existing` — only onto existing sentences, otherwise an error (calibration runs on EWT test sentences).
//!
//! Documents:
//! - `newdoc` — from `# newdoc` to the next one; blocks before the first one form the file's document; `meta::*` of the
//!   first block give title, author, genre, license (as in GUM and the fairy tales);
//! - `idprefix` — by `sent_id` without its last part: `tatoeba`, `seeds:<seed>`, `papers:<dir>`;
//! - `file` — one document per file.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result, bail};
use en::conllu::{Col, Doc, FullSentence};
use en::gram::{Feats, Rel, Tag, UPos};
use en::udreg::Registry;
use serde_json::{Map, Value, json};

use crate::duck::{Duck, Stage};
use crate::tsv::{self, Tsv, V};
use crate::util;

const STATE_SQL: &str = include_str!("../../../db/sql/state.sql");
const LOAD_SQL: &str = include_str!("../../../db/sql/load.sql");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Docs {
    NewDoc,
    IdPrefix,
    File,
}

impl Docs {
    pub fn parse(s: &str) -> Result<Docs> {
        Ok(match s {
            "newdoc" => Docs::NewDoc,
            "idprefix" => Docs::IdPrefix,
            "file" => Docs::File,
            _ => bail!("documents: newdoc | idprefix | file, not \"{s}\""),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Match {
    New,
    Auto,
    Existing,
}

impl Match {
    pub fn parse(s: &str) -> Result<Match> {
        Ok(match s {
            "new" => Match::New,
            "auto" => Match::Auto,
            "existing" => Match::Existing,
            _ => bail!("sentence matching: new | auto | existing, not \"{s}\""),
        })
    }
}

pub const SOURCES: [&str; 6] = ["gold", "silver", "bronze", "opus", "rust", "udpipe"];

/// The run that the file's sets belong to.
#[derive(Clone, Debug)]
pub struct RunInfo {
    pub name: String,
    pub kind: String,
    pub tool: String,
    pub path: String,
    pub params: Value,
    pub started: Option<String>,
    pub note: Option<String>,
    /// Annotation description for sets without `# annotation`.
    pub method: Option<String>,
    /// Default license and genre of documents (treebank card).
    pub license: Option<String>,
    pub genre: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Opts {
    pub source: String,
    pub dialect: String,
    pub layer: String,
    pub kind: String,
    pub docs: Docs,
    pub matching: Match,
    pub lang: String,
    pub run: Option<String>,
    pub method: Option<String>,
    pub license: Option<String>,
    pub genre: Option<String>,
    pub origin: Option<String>,
    pub tool: Option<String>,
    /// A run given by the caller (`db tales`); otherwise derived from the kind and the file path.
    pub run_info: Option<RunInfo>,
}

impl Opts {
    pub fn new(source: &str, dialect: &str) -> Result<Opts> {
        if !SOURCES.contains(&source) {
            bail!("source: {}, not \"{source}\"", SOURCES.join(" | "));
        }
        Ok(Opts {
            source: source.into(),
            dialect: dialect.into(),
            layer: "ud".into(),
            kind: "file".into(),
            docs: Docs::NewDoc,
            matching: Match::Auto,
            lang: "en".into(),
            run: None,
            method: None,
            license: None,
            genre: None,
            origin: None,
            tool: None,
            run_info: None,
        })
    }

    fn tool(&self) -> String {
        self.tool.clone().unwrap_or_else(|| {
            match self.source.as_str() {
                "gold" => "human",
                "silver" | "opus" => "opus",
                "bronze" | "rust" => "rust",
                other => other,
            }
            .into()
        })
    }
}

#[derive(Debug, Default)]
pub struct Report {
    pub name: String,
    pub run: String,
    pub skipped: bool,
    pub sents: usize,
    pub created: usize,
    pub matched: usize,
    pub docs: usize,
    pub rows: usize,
    pub words: usize,
    pub gate_bad: usize,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.skipped {
            return write!(f, "{}: already in the database — skipped", self.name);
        }
        write!(
            f,
            "{} → {}: blocks {}, new sentences {}, onto existing {}, new documents {}, rows {}, words {}, failed the gate {}",
            self.name, self.run, self.sents, self.created, self.matched, self.docs, self.rows, self.words, self.gate_bad
        )
    }
}

/// Database state before loading.
struct State {
    max: [i64; 5],
    runs: HashMap<String, i64>,
    files: HashMap<String, String>,
    /// key → (doc, largest ord, largest para)
    docs: HashMap<String, (i64, i64, i64)>,
    /// sent_id → [(sid, doc, text)]
    sents: HashMap<String, Vec<(i64, i64, String)>>,
}

impl State {
    fn query(duck: &Duck, want: &[&str]) -> Result<State> {
        let st = Stage::new("state")?;
        let mut w = Tsv::create(&st.path("want.tsv"), &["sent_id"])?;
        let mut seen = HashSet::new();
        for id in want {
            if seen.insert(*id) {
                w.row(&[V::S(id)])?;
            }
        }
        w.finish()?;
        duck.read(&st.sql(STATE_SQL, "q")?)?;
        let m = tsv::read(&st.path("max.tsv"))?;
        let row = m.first().context("max.tsv is empty")?;
        let mut max = [0i64; 5];
        for (i, x) in max.iter_mut().enumerate() {
            *x = tsv::int(row, i)?;
        }
        let mut s = State { max, runs: HashMap::new(), files: HashMap::new(), docs: HashMap::new(), sents: HashMap::new() };
        for r in tsv::read(&st.path("runs.tsv"))? {
            s.runs.insert(tsv::s(&r, 1)?.to_string(), tsv::int(&r, 0)?);
        }
        for r in tsv::read(&st.path("files.tsv"))? {
            s.files.insert(tsv::s(&r, 0)?.to_string(), tsv::s(&r, 1)?.to_string());
        }
        for r in tsv::read(&st.path("docs.tsv"))? {
            s.docs.insert(tsv::s(&r, 1)?.to_string(), (tsv::int(&r, 0)?, tsv::int(&r, 2)?, tsv::int(&r, 3)?));
        }
        for r in tsv::read(&st.path("sents.tsv"))? {
            s.sents.entry(tsv::s(&r, 2)?.to_string()).or_default().push((tsv::int(&r, 0)?, tsv::int(&r, 1)?, tsv::s(&r, 3)?.to_string()));
        }
        Ok(s)
    }
}

/// Comment "# key = value" → (key, Some(value)); "# word" → (word, None).
pub fn comment_kv(line: &str) -> (String, Option<String>) {
    let body = line.strip_prefix('#').unwrap_or(line).trim_start();
    match body.split_once(" = ") {
        Some((k, v)) => (k.trim().to_string(), Some(v.to_string())),
        None => match body.strip_suffix(" =") {
            Some(k) => (k.trim().to_string(), Some(String::new())),
            None => (body.trim().to_string(), None),
        },
    }
}

fn doc_level(key: &str) -> bool {
    key == "newdoc" || key == "newdoc id" || key.starts_with("meta::") || key.starts_with("global.")
}

/// `en` gates, the same for all sources: tree (heads, one root, `root` only on it, no cycles),
/// closed lists of UPOS, PTB, relations and FEATS pairs, the UD 2.18 registry for English. "_" means the layer
/// is absent, not a violation. The first reason, or "ok".
pub fn gate(fs: &FullSentence, reg: &Registry) -> String {
    if let Err(e) = en::convert::tree_gate(fs) {
        return e;
    }
    for (k, t) in fs.sent.tokens.iter().enumerate() {
        let r = fs.word(k);
        let (u, x, f, d) = (r.get(Col::Upos), r.get(Col::Xpos), r.get(Col::Feats), r.get(Col::Deprel));
        if u != "_" && UPos::parse(u).is_none() {
            return format!("word {}: UPOS \"{u}\" outside UD", k + 1);
        }
        if x != "_" && Tag::parse(x).is_none() {
            return format!("word {}: XPOS \"{x}\" outside PTB", k + 1);
        }
        if Feats::parse(f).1 > 0 {
            return format!("word {}: FEATS \"{f}\" — pair outside the list", k + 1);
        }
        if Rel::parse(d).is_none() {
            return format!("word {}: relation \"{d}\" outside the list", k + 1);
        }
        if let Some(up) = t.upos {
            // lemma "_" — no lemma layer (ESLSpok, GUMReddit): lemma checks (AUX, cop) are skipped
            // by substituting "be", which passes both; features and relation are checked as usual
            let lemma = if t.lemma == "_" { "be" } else { t.lemma.as_str() };
            if let Some(m) = reg.check(up, t.feats, t.rel, lemma).into_iter().next() {
                return format!("word {}: {m}", k + 1);
            }
        }
    }
    "ok".into()
}

/// Treebank card `treebanks/<language>/<code>/README.md`: license, genres, authors, part sizes.
/// Card labels are English or (older cards) Ukrainian (`util::CARD_*`); keys in the result are English.
fn card(lang: &str, dir: &str) -> Result<(String, Value)> {
    let code = dir.split_once('-').map_or(dir, |x| x.1).to_lowercase();
    let repo = std::env::var("MOVA_REPO").map(std::path::PathBuf::from).unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let path = repo.join("treebanks").join(lang).join(&code).join("README.md");
    let path = util::abs(&path).with_context(|| format!("treebank card {dir}"))?;
    let text = std::fs::read_to_string(&path)?;
    let mut fields = Map::new();
    let mut volume = Map::new();
    let mut in_volume = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            in_volume = util::CARD_SIZE_HEADINGS.contains(&line.trim());
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() < 4 {
            continue;
        }
        if in_volume {
            let part = cells[1].trim_matches('*');
            let nums: Vec<i64> = cells[2..cells.len() - 1].iter().filter_map(|c| c.chars().filter(char::is_ascii_digit).collect::<String>().parse().ok()).collect();
            let part = util::card_part(part);
            if ["train", "dev", "test", "total"].contains(&part) && nums.len() == 5 {
                volume.insert(part.to_string(), json!(nums));
            }
        } else if let Some((_, key)) = util::CARD_FIELDS.iter().find(|(label, _)| *label == cells[1]) {
            fields.insert(key.to_string(), json!(cells[2]));
        }
    }
    if fields.get("ud_name").and_then(Value::as_str) != Some(dir) {
        bail!("{}: card is not for {dir}", path.display());
    }
    let p = path.to_string_lossy().into_owned();
    Ok((p.clone(), json!({"card": p, "fields": fields, "volume": volume})))
}

const CALIB_RUNS: &str = include_str!("../../../db/calib-runs.tsv");

/// Effort of a calibration run: from `db/calib-runs.tsv` (with an explanation), otherwise from the directory name, otherwise "unknown".
fn calib_effort(name: &str) -> (String, Option<String>) {
    for line in CALIB_RUNS.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() >= 3 && c[0] == name {
            return (c[1].to_string(), Some(c[2].to_string()));
        }
    }
    let e = ["high", "medium", "low"].into_iter().find(|e| name.contains(&format!("-{e}"))).unwrap_or("unknown");
    (e.to_string(), None)
}

/// An `annot` calibration run: mode from the first step of `calls.jsonl`, effort (`calib_effort`), call totals.
fn calib_run(dir: &Path) -> (String, String, Value) {
    let name = util::file_name(dir);
    let (effort, effort_note) = calib_effort(&name);
    let effort = effort.as_str();
    let (mut step, mut calls, mut out_tokens, mut secs) = (String::new(), 0u64, 0u64, 0f64);
    for line in std::fs::read_to_string(dir.join("calls.jsonl")).unwrap_or_default().lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
        if step.is_empty() {
            step = v["step"].as_str().unwrap_or("").to_string();
        }
        calls += 1;
        out_tokens += v["output_tokens"].as_u64().unwrap_or(0);
        secs += v["secs"].as_f64().unwrap_or(0.0);
    }
    let flow = match step.as_str() {
        "annotate" => "two-pass: Opus's first view → Rust → Opus reconciliation",
        "correct" => "draft: Rust → one Opus correction",
        "patch" => "patch mode: Rust → Opus patches, FEATS from Rust",
        _ => "unknown mode",
    };
    let report = std::fs::read_to_string(dir.join("report.txt")).unwrap_or_default().lines().next().unwrap_or("").to_string();
    let params = json!({"flow": step, "effort": effort, "effort_note": effort_note, "calls": calls, "output_tokens": out_tokens, "secs": (secs * 10.0).round() / 10.0, "report": report, "model": "claude-opus-5-5"});
    (format!("annot calib · {flow} · effort {effort}"), effort.to_string(), params)
}

fn run_info(opts: &Opts, abs: &Path) -> Result<RunInfo> {
    let parent = util::parent_name(abs);
    let dir = abs.parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default();
    let started = util::mtime_ts(abs);
    let mut r = match opts.kind.as_str() {
        "treebank" => {
            let (_, params) = card(&opts.lang, &parent)?;
            let f = &params["fields"];
            let release = opts.dialect.replace("ud-", "UD ");
            RunInfo {
                name: format!("{}:{parent}", opts.dialect),
                kind: "treebank".into(),
                tool: release.clone(),
                path: dir,
                license: f["license"].as_str().map(str::to_string),
                genre: f["genres"].as_str().map(str::to_string),
                params,
                started,
                note: Some(format!("{release}, treebank gold")),
                method: Some(format!("{release}: annotation of treebank {parent}")),
            }
        }
        "calib" => {
            let (method, _, params) = calib_run(abs.parent().context("run directory")?);
            RunInfo { name: format!("annot:{parent}"), kind: "calib".into(), tool: "annot calib".into(), path: dir, params, started, note: None, method: Some(method), license: None, genre: None }
        }
        "corpus" => RunInfo {
            name: format!("corpus:{}", util::stem(abs)),
            kind: "corpus".into(),
            tool: format!("annot {}", opts.source),
            path: abs.to_string_lossy().into_owned(),
            params: json!({"readme": "corpus/README.md"}),
            started,
            note: None,
            method: None,
            license: None,
            genre: None,
        },
        _ => RunInfo {
            name: opts.run.clone().unwrap_or_else(|| parent.clone()),
            kind: opts.kind.clone(),
            tool: "db load".into(),
            path: dir,
            params: json!({}),
            started,
            note: None,
            method: None,
            license: None,
            genre: None,
        },
    };
    if let Some(n) = &opts.run {
        r.name = n.clone();
    }
    Ok(r)
}

/// Calibration stage by the run file name.
fn stage(stem: &str) -> Option<&'static str> {
    Some(match stem {
        "opus-a" => "Opus's first view",
        "final" => "Opus final",
        "rust-b" => "Rust annotator en",
        "udpipe-c" => "UDPipe 2 (NC, comparison only)",
        _ => return None,
    })
}

/// Run date from `# annotation` ("… /annot/runs/silver-ud-2026-09-25").
fn annotation_date(a: &str) -> Option<String> {
    let at = a.find("/runs/")? + "/runs/".len();
    let dir: String = a[at..].chars().take_while(|c| !c.is_whitespace() && !matches!(c, ';' | ',' | '/' | ')')).collect();
    util::date_suffix(&dir)
}

struct DocCur {
    id: i64,
    ord: i64,
    para: i64,
}

pub fn load_file(duck: &Duck, reg: &Registry, opts: &Opts, path: &Path) -> Result<Report> {
    let abs = util::abs(path)?;
    let abs_s = abs.to_string_lossy().into_owned();
    let name = format!("{}/{}", util::parent_name(&abs), util::file_name(&abs));
    let bytes = std::fs::read(&abs).with_context(|| abs_s.clone())?;
    let sha = util::sha1_hex(&bytes);
    let text = String::from_utf8(bytes).with_context(|| format!("{abs_s}: not UTF-8"))?;
    let doc = Doc::parse(&text).with_context(|| abs_s.clone())?;
    if doc.sents.is_empty() {
        bail!("{abs_s}: no sentences");
    }
    let want: Vec<&str> = if opts.matching == Match::New { Vec::new() } else { doc.sents.iter().map(|s| s.sent.id.as_str()).filter(|x| !x.is_empty()).collect() };
    let st = State::query(duck, &want)?;
    let mut rep = Report { name: name.clone(), ..Default::default() };
    if let Some(old) = st.files.get(&abs_s) {
        if *old == sha {
            rep.skipped = true;
            return Ok(rep);
        }
        bail!("{abs_s}: file changed after loading (sha1 {old} → {sha}); remove the old load first");
    }
    let run = match &opts.run_info {
        Some(r) => r.clone(),
        None => run_info(opts, &abs)?,
    };
    rep.run = run.name.clone();
    let (run_id, new_run) = match st.runs.get(&run.name) {
        Some(&r) => (r, false),
        None => (st.max[0] + 1, true),
    };
    let file_id = st.max[1] + 1;
    let (mut next_doc, mut next_sid, mut next_aid) = (st.max[2], st.max[3], st.max[4]);
    let parent = util::parent_name(&abs);
    let fstem = util::stem(&abs);
    let origin = opts.origin.clone().unwrap_or_else(|| match opts.kind.as_str() {
        "treebank" => parent.clone(),
        "corpus" => fstem.clone(),
        _ => format!("{parent}/{fstem}"),
    });
    let prefix = (opts.kind == "treebank").then(|| format!("{parent}:{fstem}"));
    let file_doc_key = prefix.clone().unwrap_or_else(|| origin.clone());
    let file_created = util::mtime_ts(&abs);
    let method_default = opts.method.clone().or_else(|| match (opts.kind.as_str(), stage(&fstem)) {
        ("calib", Some(s)) => run.method.as_ref().map(|m| format!("{m} · {s}")),
        _ => run.method.clone(),
    });
    let license_default = opts.license.clone().or_else(|| run.license.clone());
    let genre_default = opts.genre.clone().or_else(|| run.genre.clone());
    let tool = opts.tool();

    let stg = Stage::new("load")?;
    let mut t_runs = Tsv::create(&stg.path("runs.tsv"), &["run", "name", "kind", "tool", "path", "params", "started", "note"])?;
    let mut t_files = Tsv::create(&stg.path("files.tsv"), &["file", "run", "path", "name", "bytes", "sha1", "blank_before", "final_newline", "sents", "rows", "words", "mwt", "empty"])?;
    let mut t_docs = Tsv::create(&stg.path("docs.tsv"), &["doc", "key", "source", "title", "author", "lang", "license", "genre", "meta"])?;
    let mut t_sents = Tsv::create(&stg.path("sentences.tsv"), &["sid", "doc", "para", "ord", "sent_id", "text", "comments"])?;
    let mut t_sets = Tsv::create(
        &stg.path("annsets.tsv"),
        &["aid", "doc", "sid", "layer", "source", "tool", "run", "origin", "dialect", "created", "method", "gate", "file", "pos", "blank_after", "comments"],
    )?;
    let mut t_toks = Tsv::create(&stg.path("tokens.tsv"), &["aid", "k", "id", "form", "lemma", "upos", "xpos", "feats", "head", "deprel", "deps", "misc"])?;

    if new_run {
        let params = run.params.to_string();
        t_runs.row(&[V::I(run_id), V::S(&run.name), V::S(&run.kind), V::S(&run.tool), V::S(&run.path), V::S(&params), V::opt(run.started.as_deref()), V::opt(run.note.as_deref())])?;
    }
    let (mut rows_n, mut words_n, mut mwt_n, mut empty_n) = (0i64, 0i64, 0i64, 0i64);
    for s in &doc.sents {
        for r in &s.rows {
            rows_n += 1;
            let id = r.get(Col::Id);
            if id.contains('-') {
                mwt_n += 1;
            } else if id.contains('.') {
                empty_n += 1;
            } else {
                words_n += 1;
            }
        }
    }
    t_files.row(&[
        V::I(file_id),
        V::I(run_id),
        V::S(&abs_s),
        V::S(&name),
        V::I(text.len() as i64),
        V::S(&sha),
        V::I(doc.blank_before as i64),
        V::B(doc.final_newline),
        V::I(doc.sents.len() as i64),
        V::I(rows_n),
        V::I(words_n),
        V::I(mwt_n),
        V::I(empty_n),
    ])?;

    let mut docs: HashMap<String, DocCur> = HashMap::new();
    let mut newdoc_keys: HashSet<String> = HashSet::new();
    let mut cur_doc: Option<String> = None;
    let mut newdocs = 0usize;
    for (pos, fs) in doc.sents.iter().enumerate() {
        let kv: Vec<(String, Option<String>)> = fs.comments.iter().map(|c| comment_kv(c)).collect();
        let get = |k: &str| kv.iter().find(|(a, _)| a == k).and_then(|(_, v)| v.clone());
        let sent_id = if fs.sent.id.is_empty() { format!("{origin}#{}", pos + 1) } else { fs.sent.id.clone() };
        let text = fs.sent.text.clone();
        // the document new sentences go to (for newdoc — even if this sentence already exists)
        let newdoc = kv.iter().any(|(k, _)| k == "newdoc" || k == "newdoc id");
        if opts.docs == Docs::NewDoc && newdoc {
            newdocs += 1;
            let id = get("newdoc id").unwrap_or_else(|| format!("#{newdocs}"));
            let mut key = match &prefix {
                Some(p) => format!("{p}:{id}"),
                None if id.starts_with('#') => format!("{file_doc_key}{id}"),
                None => id.clone(),
            };
            // the same newdoc id a second time in the file — a separate document marked "~n"
            let base = key.clone();
            let mut n = 1;
            while !newdoc_keys.insert(key.clone()) {
                n += 1;
                key = format!("{base}~{n}");
            }
            cur_doc = Some(key);
        }
        // sentence: existing or new
        let found: Vec<&(i64, i64, String)> = st.sents.get(&sent_id).map(|v| v.iter().filter(|x| x.2 == text).collect()).unwrap_or_default();
        let (sid, doc_id) = match (opts.matching, found.len()) {
            (Match::Auto | Match::Existing, 1) => {
                rep.matched += 1;
                (found[0].0, found[0].1)
            }
            (Match::Auto | Match::Existing, n) if n > 1 => bail!("{name}: sentence {sent_id} — {n} identical ones in the database, ambiguous match"),
            (Match::Existing, _) => bail!("{name}: sentence {sent_id} \"{text}\" is not in the database (matching existing)"),
            _ => {
                let key = match opts.docs {
                    Docs::NewDoc => cur_doc.clone().unwrap_or_else(|| file_doc_key.clone()),
                    Docs::IdPrefix => sent_id.rsplit_once(':').map_or(sent_id.clone(), |x| x.0.to_string()),
                    Docs::File => file_doc_key.clone(),
                };
                if !docs.contains_key(&key) {
                    let cur = match st.docs.get(&key) {
                        Some(&(id, ord, para)) => {
                            if opts.matching == Match::New {
                                bail!("{name}: document {key} is already in the database — loading again?");
                            }
                            DocCur { id, ord, para }
                        }
                        None => {
                            next_doc += 1;
                            rep.docs += 1;
                            // document fields: from the comments of the block that opened it, and from the run
                            let mut meta = Map::new();
                            meta.insert("file".into(), json!(name));
                            let (source, title, author, license, genre);
                            match opts.docs {
                                Docs::NewDoc => {
                                    for (k, v) in &kv {
                                        if doc_level(k) {
                                            meta.insert(k.strip_prefix("meta::").unwrap_or(k).to_string(), json!(v));
                                        }
                                    }
                                    let m = |k: &str| kv.iter().find(|(a, _)| a == &format!("meta::{k}")).and_then(|(_, v)| v.clone());
                                    source = m("source").unwrap_or_else(|| match opts.kind.as_str() {
                                        "treebank" => format!("{} {parent}", opts.dialect.replace("ud-", "UD ")),
                                        _ => run.name.clone(),
                                    });
                                    title = m("title").or_else(|| get("newdoc id"));
                                    author = m("author");
                                    license = m("license").or_else(|| license_default.clone());
                                    genre = m("genre").or_else(|| genre_default.clone());
                                }
                                Docs::IdPrefix => {
                                    let src = get("source");
                                    if let Some(s) = &src {
                                        meta.insert("source".into(), json!(s));
                                    }
                                    source = key.split(':').next().unwrap_or(&key).to_string();
                                    title = key.split_once(':').map(|x| x.1.to_string());
                                    author = None;
                                    license = src.as_deref().and_then(|s| s.rsplit_once(" (")).map(|x| x.1.trim_end_matches(')').to_string()).or_else(|| license_default.clone());
                                    genre = genre_default.clone();
                                }
                                Docs::File => {
                                    source = run.name.clone();
                                    title = Some(util::file_name(&abs));
                                    author = None;
                                    license = license_default.clone();
                                    genre = genre_default.clone();
                                }
                            }
                            let meta = Value::Object(meta).to_string();
                            t_docs.row(&[
                                V::I(next_doc),
                                V::S(&key),
                                V::S(&source),
                                V::opt(title.as_deref()),
                                V::opt(author.as_deref()),
                                V::S(&opts.lang),
                                V::opt(license.as_deref()),
                                V::opt(genre.as_deref()),
                                V::S(&meta),
                            ])?;
                            DocCur { id: next_doc, ord: 0, para: 0 }
                        }
                    };
                    docs.insert(key.clone(), cur);
                }
                let d = docs.get_mut(&key).expect("document");
                if kv.iter().any(|(k, _)| k == "newpar" || k == "newpar id") {
                    d.para += 1;
                }
                d.ord += 1;
                next_sid += 1;
                rep.created += 1;
                let mut sc = Map::new();
                for (k, v) in &kv {
                    if !doc_level(k) && k != "sent_id" && k != "text" && k != "annotation" {
                        sc.insert(k.clone(), json!(v));
                    }
                }
                let sc = Value::Object(sc).to_string();
                t_sents.row(&[V::I(next_sid), V::I(d.id), V::opt_i((d.para > 0).then_some(d.para)), V::I(d.ord), V::S(&sent_id), V::S(&text), V::S(&sc)])?;
                (next_sid, d.id)
            }
        };
        // annotation set and rows
        next_aid += 1;
        let annotation = get("annotation");
        let created = annotation.as_deref().and_then(annotation_date).or_else(|| file_created.clone());
        let method = annotation.clone().or_else(|| method_default.clone());
        let g = gate(fs, reg);
        if g != "ok" {
            rep.gate_bad += 1;
        }
        let comments = serde_json::to_string(&fs.comments)?;
        t_sets.row(&[
            V::I(next_aid),
            V::I(doc_id),
            V::I(sid),
            V::S(&opts.layer),
            V::S(&opts.source),
            V::S(&tool),
            V::I(run_id),
            V::S(&origin),
            V::S(&opts.dialect),
            V::opt(created.as_deref()),
            V::opt(method.as_deref()),
            V::S(&g),
            V::I(file_id),
            V::I(pos as i64),
            V::I(fs.blank_after as i64),
            V::S(&comments),
        ])?;
        for (k, r) in fs.rows.iter().enumerate() {
            let c = &r.0;
            t_toks.row(&[
                V::I(next_aid),
                V::I(k as i64 + 1),
                V::S(&c[0]),
                V::S(&c[1]),
                V::S(&c[2]),
                V::S(&c[3]),
                V::S(&c[4]),
                V::S(&c[5]),
                V::S(&c[6]),
                V::S(&c[7]),
                V::S(&c[8]),
                V::S(&c[9]),
            ])?;
        }
    }
    rep.sents = doc.sents.len();
    rep.rows = rows_n as usize;
    rep.words = words_n as usize;
    for t in [t_runs, t_files, t_docs, t_sents, t_sets, t_toks] {
        t.finish()?;
    }
    let sql = stg.sql(LOAD_SQL, "dir")?.replace("{file}", &file_id.to_string());
    duck.exec(&sql).with_context(|| format!("loading {name}"))?;
    // reconciliation: as much landed in the database as is in the file
    let post = tsv::read(&stg.path("post.tsv"))?;
    let p = post.first().context("post.tsv is empty")?;
    let (a, t, w) = (tsv::int(p, 0)?, tsv::int(p, 1)?, tsv::int(p, 2)?);
    if a != doc.sents.len() as i64 || t != rows_n || w != words_n {
        bail!("{name}: in the database sets {a}, rows {t}, words {w}; in the file {} / {rows_n} / {words_n}", doc.sents.len());
    }
    Ok(rep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_and_dates() {
        assert_eq!(comment_kv("# sent_id = a-1"), ("sent_id".into(), Some("a-1".into())));
        assert_eq!(comment_kv("# newpar"), ("newpar".into(), None));
        assert_eq!(comment_kv("# text = a = b"), ("text".into(), Some("a = b".into())));
        assert_eq!(comment_kv("# meta::genre = academic"), ("meta::genre".into(), Some("academic".into())));
        assert_eq!(comment_kv("# empty ="), ("empty".into(), Some(String::new())));
        assert!(doc_level("meta::title") && doc_level("newdoc id") && doc_level("global.Entity") && !doc_level("newpar id"));
        assert_eq!(
            annotation_date("Mova annot: Opus (max) A → Rust B → Opus final; gates UD 2.18; $MOVA_DATA/annot/runs/silver-ud-2026-09-25").as_deref(),
            Some("2026-09-25 00:00:00")
        );
        assert_eq!(annotation_date("Mova bronze: Rust only (en), not checked by Opus"), None);
        assert_eq!(stage("final"), Some("Opus final"));
        // effort: the table outweighs the name; without both — "unknown"
        assert_eq!(calib_effort("calib-2026-09-25-high").0, "max");
        assert_eq!(calib_effort("calib-ud-patch300-2026-09-26").0, "high");
        assert_eq!(calib_effort("calib-ud-patch-low-2026-09-25"), ("low".to_string(), None));
        assert_eq!(calib_effort("calib-zzz").0, "unknown");
    }

    #[test]
    fn treebank_card_labels() {
        // the committed EWT card (Ukrainian labels for now) and an English card as the generator writes it
        let (_, p) = card("en", "UD_English-EWT").unwrap();
        assert!(p["fields"]["license"].as_str().is_some_and(|s| !s.is_empty()), "{p}");
        assert!(p["volume"]["train"].is_array() && p["volume"]["total"].is_array(), "{p}");
        let dir = std::env::temp_dir().join(format!("mova-db-card-{}", std::process::id()));
        let cdir = dir.join("treebanks/xx/zz");
        std::fs::create_dir_all(&cdir).unwrap();
        std::fs::write(cdir.join("README.md"), "| Field | Value |\n|---|---|\n| UD name | UD_Xx-ZZ |\n| License | CC BY 4.0 |\n\n## Size\n\n| Split | Sentences | Tokens | Words | Multiword tokens | Empty nodes |\n|---|---:|---:|---:|---:|---:|\n| test | 1 | 2 | 2 | 0 | 0 |\n| **total** | 1 | 2 | 2 | 0 | 0 |\n").unwrap();
        // SAFETY: test-only env change; no other test reads MOVA_REPO concurrently with a different value
        unsafe { std::env::set_var("MOVA_REPO", &dir) };
        let r = card("xx", "UD_Xx-ZZ");
        unsafe { std::env::remove_var("MOVA_REPO") };
        let (_, p) = r.unwrap();
        assert_eq!(p["fields"]["license"], "CC BY 4.0");
        assert_eq!(p["volume"]["total"], json!([1, 2, 2, 0, 0]));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
