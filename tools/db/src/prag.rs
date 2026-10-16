//! The pragmatics layer (first slice, docs/context.md; crate `prag`): labels from TSV → run, sets
//! (`layer = 'pragmatics'`, one set per sentence) and `pragmatics` rows in one transaction.
//!
//! Input — TSV with the header `sent_id form act indirect hedge polarity voice means [trace]` (as written by
//! `prag silver` and `prag eval`). Sentences are looked up by `sent_id`: each must be in the database exactly once,
//! otherwise an error and nothing changes in the database. A run with the same name already exists — an error (no duplicates).
//! Values of closed sets are not checked here — those are `prag` gates; here — database integrity.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::duck::{Duck, Stage};
use crate::tsv::{self, Tsv, V};
use crate::util;

const STATE_SQL: &str = include_str!("../sql/prag-state.sql");
const LOAD_SQL: &str = include_str!("../sql/prag-load.sql");

/// Load options.
pub struct Opts {
    pub run: String,
    /// silver | rust | opus | bronze …
    pub source: String,
    pub tool: String,
    pub origin: String,
    pub method: Option<String>,
    pub params: Value,
    pub path: Option<String>,
    pub note: Option<String>,
}

/// A row of labels.
struct Row {
    sent_id: String,
    cols: Vec<String>,
}

fn read_labels(path: &Path) -> Result<Vec<Row>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut lines = text.lines();
    let head: Vec<&str> = lines.next().context("empty file")?.split('\t').collect();
    let want = ["sent_id", "form", "act", "indirect", "hedge", "polarity", "voice", "means"];
    if head.len() < want.len() || head[..want.len()] != want || (head.len() == 9 && head[8] != "trace") || head.len() > 9 {
        bail!("{}: header {:?}, expected {:?} [+ trace]", path.display(), head, want);
    }
    let mut out = Vec::new();
    for (i, l) in lines.enumerate() {
        if l.is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != head.len() {
            bail!("{}:{}: {} columns instead of {}", path.display(), i + 2, c.len(), head.len());
        }
        let mut cols: Vec<String> = c[1..].iter().map(|x| x.to_string()).collect();
        if cols.len() == 7 {
            cols.push(String::new());
        }
        out.push(Row { sent_id: c[0].to_string(), cols });
    }
    Ok(out)
}

fn bit(s: &str) -> Result<bool> {
    match s {
        "0" | "false" => Ok(false),
        "1" | "true" => Ok(true),
        _ => bail!("not 0|1: {s:?}"),
    }
}

/// Load labels. Returns (run, rows).
pub fn load(duck: &Duck, labels: &Path, o: &Opts) -> Result<(i64, usize)> {
    let rows = read_labels(labels)?;
    if rows.is_empty() {
        bail!("{}: no rows", labels.display());
    }
    let mut seen = HashSet::new();
    for r in &rows {
        if !seen.insert(r.sent_id.as_str()) {
            bail!("{}: sent_id {} twice", labels.display(), r.sent_id);
        }
    }
    // database state
    let st = Stage::new("prag-state")?;
    let mut w = Tsv::create(&st.path("want.tsv"), &["sent_id"])?;
    for r in &rows {
        w.row(&[V::S(&r.sent_id)])?;
    }
    w.finish()?;
    duck.read(&st.sql(STATE_SQL, "q")?)?;
    let m = tsv::read(&st.path("max.tsv"))?;
    let mrow = m.first().context("max.tsv is empty")?;
    let (max_run, max_aid) = (tsv::int(mrow, 0)?, tsv::int(mrow, 1)?);
    for r in tsv::read(&st.path("runs.tsv"))? {
        if tsv::s(&r, 1)? == o.run {
            bail!("run \"{}\" already exists (run {}) — not loading again", o.run, tsv::int(&r, 0)?);
        }
    }
    let mut sents: HashMap<String, Vec<(i64, i64)>> = HashMap::new();
    for r in tsv::read(&st.path("sents.tsv"))? {
        sents.entry(tsv::s(&r, 2)?.to_string()).or_default().push((tsv::int(&r, 0)?, tsv::int(&r, 1)?));
    }
    let missing: Vec<&str> = rows.iter().filter(|r| !sents.contains_key(&r.sent_id)).map(|r| r.sent_id.as_str()).collect();
    if !missing.is_empty() {
        bail!("sentences not in the database: {} (first: {})", missing.len(), missing.iter().take(5).copied().collect::<Vec<_>>().join(", "));
    }
    let ambiguous: Vec<&str> = rows.iter().filter(|r| sents[&r.sent_id].len() > 1).map(|r| r.sent_id.as_str()).collect();
    if !ambiguous.is_empty() {
        bail!("ambiguous sent_id: {} (first: {})", ambiguous.len(), ambiguous.iter().take(5).copied().collect::<Vec<_>>().join(", "));
    }
    // TSV for COPY
    let run = max_run + 1;
    let now = util::now_ts();
    let stage = Stage::new("prag-load")?;
    let mut t = Tsv::create(&stage.path("runs.tsv"), &["run", "name", "kind", "tool", "path", "params", "started", "note"])?;
    let params = serde_json::to_string(&o.params)?;
    t.row(&[V::I(run), V::S(&o.run), V::S("prag"), V::S(&format!("prag {}", o.source)), V::opt(o.path.as_deref()), V::S(&params), V::S(&now), V::opt(o.note.as_deref())])?;
    t.finish()?;
    let mut a = Tsv::create(&stage.path("annsets.tsv"), &["aid", "doc", "sid", "layer", "source", "tool", "run", "origin", "dialect", "created", "method", "gate"])?;
    let mut p = Tsv::create(&stage.path("pragmatics.tsv"), &["aid", "sid", "s_type", "act", "means", "indirect", "hedge", "polarity", "voice", "trace"])?;
    for (k, r) in rows.iter().enumerate() {
        let aid = max_aid + 1 + k as i64;
        let (sid, doc) = sents[&r.sent_id][0];
        a.row(&[V::I(aid), V::I(doc), V::I(sid), V::S("pragmatics"), V::S(&o.source), V::S(&o.tool), V::I(run), V::S(&o.origin), V::S("prag-1"), V::S(&now), V::opt(o.method.as_deref()), V::S("ok")])?;
        let c = &r.cols;
        let (ind, hed) = (bit(&c[2]).with_context(|| format!("{} indirect", r.sent_id))?, bit(&c[3]).with_context(|| format!("{} hedge", r.sent_id))?);
        // empty means (the SLM (small language model) does not write it) and trace — NULL
        fn opt(s: &str) -> V<'_> {
            if s.is_empty() { V::Null } else { V::S(s) }
        }
        p.row(&[V::I(aid), V::I(sid), V::S(&c[0]), V::S(&c[1]), opt(&c[6]), V::B(ind), V::B(hed), V::S(&c[4]), V::S(&c[5]), opt(&c[7])])?;
    }
    a.finish()?;
    let n = p.finish()?;
    let sql = stage.sql(LOAD_SQL, "dir")?.replace("{run}", &run.to_string());
    duck.exec(&sql)?;
    let post = tsv::read(&stage.path("post.tsv"))?;
    let got = tsv::int(post.first().context("post.tsv is empty")?, 0)?;
    if got != n as i64 {
        bail!("after COMMIT the database has {got} rows of run {run}, but the file has {n}");
    }
    Ok((run, n))
}

/// Schema migrations: everything from the first "-- migrate:v" mark to the end of `schema.sql` (steps are idempotent).
pub fn migrate_sql(schema: &str) -> Result<&str> {
    let at = schema.find("-- migrate:v").context("schema.sql has no migrations")?;
    Ok(&schema[at..])
}
