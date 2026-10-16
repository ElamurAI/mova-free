//! Training questions and answers per paragraph (docs/context.md, "five-year-old test" — "training questions
//! from the LLM"; crate `qagen`): TSV → run and `qa` rows in one transaction.
//!
//! Input — TSV with the header `doc_key para k qtype ex_or_im anchors question answer` (as written by
//! `qagen gate`); `anchors` — comma-separated `sent_id`s of source sentences. Here — database integrity:
//! - the document is in the database;
//! - each anchor is a database sentence exactly once, and of this very document and paragraph;
//! - (document, paragraph, number) does not repeat in the file, numbers start at 1;
//! - no run with the same name exists yet.
//!
//! Replacement (`--replaces <run>`, migration v4; `qagen fix`): each row of the file replaces the row of that
//! run with the same document, paragraph and number; that row must be in the database and not yet replaced. The new
//! row gets `replaces` (the old qid), the old one gets `replaced_by` (the new qid); nothing is deleted.
//!
//! A violation is an error, and nothing changes in the database. Closed sets of type, explicitness and source and
//! a non-empty answer are enforced by CHECKs of table `qa` (migration v3); content gates live in `qagen`.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::duck::{Duck, Stage};
use crate::tsv::{self, Tsv, V};
use crate::util;

const STATE_SQL: &str = include_str!("../sql/qa-state.sql");
const LOAD_SQL: &str = include_str!("../sql/qa-load.sql");

pub const HEADER: [&str; 8] = ["doc_key", "para", "k", "qtype", "ex_or_im", "anchors", "question", "answer"];

/// Load options.
pub struct Opts {
    pub run: String,
    /// opus | gold | silver | rust
    pub source: String,
    pub tool: String,
    pub params: Value,
    pub path: Option<String>,
    pub note: Option<String>,
    /// name of the run whose rows the file's rows replace
    pub replaces: Option<String>,
}

/// A file row.
struct Row {
    doc_key: String,
    para: i64,
    k: i64,
    qtype: String,
    ex_or_im: String,
    anchors: Vec<String>,
    question: String,
    answer: String,
}

fn read_rows(path: &Path) -> Result<Vec<Row>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut lines = text.lines();
    let head: Vec<&str> = lines.next().context("empty file")?.split('\t').collect();
    if head != HEADER {
        bail!("{}: header {:?}, expected {:?}", path.display(), head, HEADER);
    }
    let mut out = Vec::new();
    for (i, l) in lines.enumerate() {
        if l.is_empty() {
            continue;
        }
        let at = || format!("{}:{}", path.display(), i + 2);
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != HEADER.len() {
            bail!("{}: {} columns instead of {}", at(), c.len(), HEADER.len());
        }
        let para: i64 = c[1].parse().with_context(|| format!("{}: paragraph \"{}\"", at(), c[1]))?;
        let k: i64 = c[2].parse().with_context(|| format!("{}: number \"{}\"", at(), c[2]))?;
        if k < 1 {
            bail!("{}: question number {k} < 1", at());
        }
        let anchors: Vec<String> = c[5].split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        if anchors.is_empty() {
            bail!("{}: no anchors", at());
        }
        out.push(Row {
            doc_key: c[0].to_string(),
            para,
            k,
            qtype: c[3].to_string(),
            ex_or_im: c[4].to_string(),
            anchors,
            question: c[6].to_string(),
            answer: c[7].to_string(),
        });
    }
    Ok(out)
}

/// Load questions. Returns (run, rows).
pub fn load(duck: &Duck, file: &Path, o: &Opts) -> Result<(i64, usize)> {
    let rows = read_rows(file)?;
    if rows.is_empty() {
        bail!("{}: no rows", file.display());
    }
    let mut seen = HashSet::new();
    for r in &rows {
        if !seen.insert((r.doc_key.as_str(), r.para, r.k)) {
            bail!("{}: {} paragraph {} question {} twice", file.display(), r.doc_key, r.para, r.k);
        }
    }
    // database state
    let st = Stage::new("qa-state")?;
    let docs_want: BTreeSet<&str> = rows.iter().map(|r| r.doc_key.as_str()).collect();
    let sents_want: BTreeSet<&str> = rows.iter().flat_map(|r| r.anchors.iter().map(String::as_str)).collect();
    let mut w = Tsv::create(&st.path("want-docs.tsv"), &["doc_key"])?;
    for d in &docs_want {
        w.row(&[V::S(d)])?;
    }
    w.finish()?;
    let mut w = Tsv::create(&st.path("want-sents.tsv"), &["sent_id"])?;
    for s in &sents_want {
        w.row(&[V::S(s)])?;
    }
    w.finish()?;
    duck.read(&st.sql(STATE_SQL, "q")?)?;
    let m = tsv::read(&st.path("max.tsv"))?;
    let mrow = m.first().context("max.tsv is empty")?;
    let (max_run, max_qid) = (tsv::int(mrow, 0)?, tsv::int(mrow, 1)?);
    let mut old_run = None;
    for r in tsv::read(&st.path("runs.tsv"))? {
        if tsv::s(&r, 1)? == o.run {
            bail!("run \"{}\" already exists (run {}) — not loading again", o.run, tsv::int(&r, 0)?);
        }
        if o.replaces.as_deref() == Some(tsv::s(&r, 1)?) {
            old_run = Some(tsv::int(&r, 0)?);
        }
    }
    if let Some(name) = &o.replaces
        && old_run.is_none()
    {
        bail!("run \"{name}\" does not exist — nothing to replace");
    }
    let mut docs: HashMap<String, i64> = HashMap::new();
    for r in tsv::read(&st.path("docs.tsv"))? {
        docs.insert(tsv::s(&r, 1)?.to_string(), tsv::int(&r, 0)?);
    }
    let missing: Vec<&&str> = docs_want.iter().filter(|d| !docs.contains_key(**d)).collect();
    if !missing.is_empty() {
        bail!("documents not in the database: {} (first: {:?})", missing.len(), missing.iter().take(5).collect::<Vec<_>>());
    }
    // sent_id → [(sid, doc, para)]
    let mut sents: HashMap<String, Vec<(i64, i64, Option<i64>)>> = HashMap::new();
    for r in tsv::read(&st.path("sents.tsv"))? {
        let para = r.get(2).and_then(|c| c.as_deref()).map(str::parse::<i64>).transpose()?;
        sents.entry(tsv::s(&r, 3)?.to_string()).or_default().push((tsv::int(&r, 0)?, tsv::int(&r, 1)?, para));
    }
    let mut bad = Vec::new();
    for r in &rows {
        let doc = docs[&r.doc_key];
        for a in &r.anchors {
            match sents.get(a).map(Vec::as_slice) {
                None | Some([]) => bad.push(format!("{a}: sentence not in the database")),
                Some([(_, d, p)]) if *d != doc || *p != Some(r.para) => {
                    bad.push(format!("{a}: not from paragraph {} of document {} (question {})", r.para, r.doc_key, r.k))
                }
                Some([_]) => {}
                Some(v) => bad.push(format!("{a}: ambiguous sent_id ({} sentences)", v.len())),
            }
        }
    }
    if !bad.is_empty() {
        bail!("anchors: {} errors (first: {})", bad.len(), bad.iter().take(5).cloned().collect::<Vec<_>>().join("; "));
    }
    // replacement: (document, paragraph, number) → qid of the old run's row not yet replaced
    let mut replaces: Vec<Option<i64>> = vec![None; rows.len()];
    if let Some(old_run) = old_run {
        let mut old: HashMap<(i64, i64, i64), (i64, Option<i64>)> = HashMap::new();
        for r in tsv::read(&st.path("oldqa.tsv"))? {
            if tsv::int(&r, 1)? == old_run {
                let by = r.get(5).and_then(|c| c.as_deref()).map(str::parse::<i64>).transpose()?;
                old.insert((tsv::int(&r, 2)?, tsv::int(&r, 3)?, tsv::int(&r, 4)?), (tsv::int(&r, 0)?, by));
            }
        }
        let name = o.replaces.as_deref().unwrap_or_default();
        for (i, r) in rows.iter().enumerate() {
            match old.get(&(docs[&r.doc_key], r.para, r.k)) {
                None => bad.push(format!("{} paragraph {} question {}: run \"{name}\" has no such row", r.doc_key, r.para, r.k)),
                Some((qid, Some(by))) => bad.push(format!("{} paragraph {} question {}: row {qid} is already replaced by row {by}", r.doc_key, r.para, r.k)),
                Some((qid, None)) => replaces[i] = Some(*qid),
            }
        }
        if !bad.is_empty() {
            bail!("replacement: {} errors (first: {})", bad.len(), bad.iter().take(5).cloned().collect::<Vec<_>>().join("; "));
        }
    }
    // TSV for COPY
    let run = max_run + 1;
    let now = util::now_ts();
    let stage = Stage::new("qa-load")?;
    let mut t = Tsv::create(&stage.path("runs.tsv"), &["run", "name", "kind", "tool", "path", "params", "started", "note"])?;
    let params = serde_json::to_string(&o.params)?;
    t.row(&[V::I(run), V::S(&o.run), V::S("qa"), V::S(&o.tool), V::opt(o.path.as_deref()), V::S(&params), V::S(&now), V::opt(o.note.as_deref())])?;
    t.finish()?;
    let mut q = Tsv::create(&stage.path("qa.tsv"), &["qid", "run", "doc", "para", "k", "question", "qtype", "ex_or_im", "answer", "anchors", "source", "replaces"])?;
    for (i, r) in rows.iter().enumerate() {
        let anchors = serde_json::to_string(&r.anchors.iter().map(|a| sents[a][0].0).collect::<Vec<i64>>())?;
        q.row(&[
            V::I(max_qid + 1 + i as i64),
            V::I(run),
            V::I(docs[&r.doc_key]),
            V::I(r.para),
            V::I(r.k),
            V::S(&r.question),
            V::S(&r.qtype),
            V::S(&r.ex_or_im),
            V::S(&r.answer),
            V::S(&anchors),
            V::S(&o.source),
            V::opt_i(replaces[i]),
        ])?;
    }
    let n = q.finish()?;
    let sql = stage.sql(LOAD_SQL, "dir")?.replace("{run}", &run.to_string());
    duck.exec(&sql)?;
    let post = tsv::read(&stage.path("post.tsv"))?;
    let prow = post.first().context("post.tsv is empty")?;
    let got = tsv::int(prow, 0)?;
    if got != n as i64 {
        bail!("after COMMIT the database has {got} rows of run {run}, but the file has {n}");
    }
    let want = replaces.iter().flatten().count() as i64;
    let (linked, marked) = (tsv::int(prow, 1)?, tsv::int(prow, 2)?);
    if linked != want || marked != want {
        bail!("after COMMIT the replacement does not reconcile: rows with replaces {linked}, old rows with replaced_by {marked}, expected {want}");
    }
    Ok((run, n))
}
