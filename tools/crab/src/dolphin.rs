//! Low-priority, finite-quota independent annotation audit. No teacher or runtime invocation.
//! Main annotations are opened read-only. Reservations consume lifetime sentence quota.
use crate::Result;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
#[allow(dead_code)]
#[path = "../../../prag/src/schema.rs"]
mod prag_schema;
const PROMPT_LIMIT: usize = 12_000;
const TOKEN_LIMIT: usize = 96;
fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn hash(s: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(s.as_ref()))
}
fn string<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| error(format!("missing string {k}")))
}
fn config_paths(c: &Value) -> Result<(PathBuf, PathBuf, Value)> {
    let run = PathBuf::from(string(c, "run_root")?);
    let fish_path = PathBuf::from(string(c, "fish_config")?);
    if !run.is_absolute() || !fish_path.is_absolute() {
        return Err(error("absolute run_root/fish_config required"));
    }
    let fish: Value = serde_json::from_slice(&fs::read(&fish_path)?)?;
    let db = PathBuf::from(string(&fish, "db")?);
    if !db.is_absolute() {
        return Err(error("absolute source db required"));
    }
    Ok((run.join("dolphin.sqlite3"), db, fish))
}
// Conservative decimal ppm: truncating finer precision cannot exceed the configured fraction.
fn fraction_ppm(c: &Value) -> Result<u64> {
    let v = c.get("dolphin_fraction").cloned().unwrap_or(json!(0.005));
    let f = v
        .as_f64()
        .ok_or_else(|| error("dolphin_fraction must be numeric"))?;
    if !f.is_finite() || !(0.0..=0.01).contains(&f) {
        return Err(error("dolphin_fraction must be between zero and 0.01"));
    }
    let s = v.to_string();
    let (mant, exp) = if let Some((a, b)) = s.split_once(['e', 'E']) {
        (a, b.parse::<i32>()?)
    } else {
        (s.as_str(), 0)
    };
    let mant = mant.strip_prefix('-').unwrap_or(mant);
    let decimals = mant.split_once('.').map(|(_, b)| b.len()).unwrap_or(0) as i32;
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut n: u128 = digits.parse()?;
    let scale = 6 + exp - decimals;
    if scale >= 0 {
        n = n
            .checked_mul(
                10u128
                    .checked_pow(scale as u32)
                    .ok_or_else(|| error("fraction scale"))?,
            )
            .ok_or_else(|| error("fraction overflow"))?;
    } else if -scale > 38 {
        n = 0;
    } else {
        n /= 10u128.pow((-scale) as u32);
    }
    Ok(n.min(10_000) as u64)
}
fn quota(n: u64, ppm: u64) -> u64 {
    ((n as u128 * ppm as u128) / 1_000_000).min((n / 100) as u128) as u64
}
fn source(db: &Path) -> Result<Connection> {
    let c = Connection::open_with_flags(db, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    c.busy_timeout(Duration::from_secs(30))?;
    c.execute_batch("PRAGMA query_only=ON;")?;
    let schema: String = c.query_row("SELECT value FROM meta WHERE key='schema'", [], |r| {
        r.get(0)
    })?;
    if schema != "peta-versions-v1" {
        return Err(error("unsupported source annotation schema"));
    }
    Ok(c)
}
// journal_mode=WAL is a database transition: SQLite's busy handler does not reliably
// serialize simultaneous bootstrap PRAGMAs. Keep this OS lease separate from quota transactions.
struct BootstrapLock(fs::File);
impl Drop for BootstrapLock {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
fn bootstrap_lock(path: &Path, limit: Duration) -> Result<BootstrapLock> {
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(path)?;
    let started = Instant::now();
    loop {
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            return Ok(BootstrapLock(file));
        }
        let e = std::io::Error::last_os_error();
        if !matches!(e.raw_os_error(), Some(libc::EWOULDBLOCK | libc::EINTR)) {
            return Err(e.into());
        }
        if started.elapsed() >= limit {
            return Err(error(
                "Dolphin queue bootstrap lease timeout; no reservation attempted",
            ));
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn queue(path: &Path, db: &Path) -> Result<Connection> {
    fs::create_dir_all(path.parent().ok_or_else(|| error("queue parent"))?)?;
    let _bootstrap = bootstrap_lock(
        &path.with_extension("bootstrap.lock"),
        Duration::from_secs(5),
    )?;
    let mut c = Connection::open(path)?;
    c.busy_timeout(Duration::from_secs(30))?;
    let mode: String = c.query_row("PRAGMA journal_mode", [], |r| r.get(0))?;
    if !mode.eq_ignore_ascii_case("wal") {
        c.execute_batch("PRAGMA journal_mode=WAL;")?;
    }
    c.execute_batch("PRAGMA synchronous=FULL;")?;
    let tables: u64 = c.query_row(
        "SELECT count(*) FROM sqlite_schema WHERE type='table' AND name IN('meta','requests')",
        [],
        |r| r.get(0),
    )?;
    if tables != 2 {
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch("CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);
 CREATE TABLE IF NOT EXISTS requests(request_id TEXT PRIMARY KEY,sent_id TEXT UNIQUE NOT NULL,state TEXT NOT NULL CHECK(state IN('reserved','submitted','completed','quarantined')),selected_key TEXT NOT NULL,base_tree_sha TEXT NOT NULL,base_prag_sha TEXT NOT NULL,source_tree TEXT NOT NULL,source_prag TEXT NOT NULL,source_split TEXT NOT NULL,source_provenance TEXT NOT NULL,source_provenance_sha TEXT NOT NULL,request_json TEXT NOT NULL,report_json TEXT,report_sha TEXT,review_tree TEXT,review_prag TEXT,reason TEXT NOT NULL,reserved_n INTEGER NOT NULL,reserved_budget INTEGER NOT NULL,fraction_ppm INTEGER NOT NULL);")?;
        tx.execute(
            "INSERT OR IGNORE INTO meta VALUES('schema','dolphin-review-only-v1')",
            [],
        )?;
        tx.execute(
            "INSERT OR IGNORE INTO meta VALUES('source_db',?1)",
            [db.to_string_lossy().as_ref()],
        )?;
        tx.commit()?;
    }
    let schema: String = c.query_row("SELECT value FROM meta WHERE key='schema'", [], |r| {
        r.get(0)
    })?;
    let prior: String = c.query_row("SELECT value FROM meta WHERE key='source_db'", [], |r| {
        r.get(0)
    })?;
    if schema != "dolphin-review-only-v1" || prior != db.to_string_lossy() {
        return Err(error("Dolphin schema/source identity mismatch"));
    }
    Ok(c)
}
#[derive(Clone)]
struct Tree {
    id: String,
    text: String,
    body: String,
    forms: Vec<String>,
}
fn tree(s: &str) -> Result<Tree> {
    let mut active = false;
    let mut ended = false;
    let mut ids = Vec::new();
    let mut texts = Vec::new();
    let mut heads = Vec::new();
    let mut forms = Vec::new();
    let upos = [
        "ADJ", "ADP", "ADV", "AUX", "CCONJ", "DET", "INTJ", "NOUN", "NUM", "PART", "PRON", "PROPN",
        "PUNCT", "SCONJ", "SYM", "VERB", "X",
    ];
    for l in s.lines() {
        if l.is_empty() {
            if !heads.is_empty() {
                ended = true;
            }
            continue;
        }
        if ended {
            return Err(error("extra tree/fragment after sentence"));
        }
        if let Some(id) = l.strip_prefix("# sent_id = ") {
            active = true;
            ids.push(id.to_string());
        } else if l.starts_with("# sent_id") {
            return Err(error("malformed sent_id"));
        } else if let Some(t) = l.strip_prefix("# text = ") {
            texts.push(t.to_string());
        } else if !l.starts_with('#') {
            if !active {
                return Err(error("token before sent_id"));
            }
            let f: Vec<_> = l.split('\t').collect();
            if f.len() != 10
                || f.iter().any(|x| x.is_empty())
                || f[0].parse::<usize>().ok() != Some(heads.len() + 1)
            {
                return Err(error("integer ten-column tokens required"));
            }
            if !upos.contains(&f[3]) {
                return Err(error("unknown UPOS"));
            }
            if f[5] != "_" {
                let mut keys = BTreeSet::new();
                for feature in f[5].split('|') {
                    let (k, v) = feature
                        .split_once('=')
                        .ok_or_else(|| error("malformed UD feature"))?;
                    if k.is_empty()
                        || v.is_empty()
                        || !keys.insert(k)
                        || !k
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || "[]".contains(c))
                        || !v
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || ",[]".contains(c))
                    {
                        return Err(error("malformed/duplicate UD feature"));
                    }
                }
            }
            if f[8] != "_" {
                return Err(error(
                    "enhanced dependency rows outside this bounded adapter",
                ));
            }
            let h: usize = f[6].parse()?;
            let base = f[7].split(':').next().unwrap();
            let rels = [
                "acl",
                "advcl",
                "advmod",
                "amod",
                "appos",
                "aux",
                "case",
                "cc",
                "ccomp",
                "clf",
                "compound",
                "conj",
                "cop",
                "csubj",
                "dep",
                "det",
                "discourse",
                "dislocated",
                "expl",
                "fixed",
                "flat",
                "goeswith",
                "iobj",
                "list",
                "mark",
                "nmod",
                "nsubj",
                "nummod",
                "obj",
                "obl",
                "orphan",
                "parataxis",
                "punct",
                "reparandum",
                "root",
                "vocative",
                "xcomp",
            ];
            if !rels.contains(&base)
                || !f[7]
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == ':' || c == '_')
                || (h == 0) != (f[7] == "root")
            {
                return Err(error("dependency relation/root mismatch"));
            }
            heads.push(h);
            forms.push(f[1].into());
        }
    }
    if ids.len() != 1
        || texts.len() != 1
        || ids[0].is_empty()
        || texts[0].is_empty()
        || heads.is_empty()
        || heads.len() > TOKEN_LIMIT
        || heads.iter().filter(|&&h| h == 0).count() != 1
    {
        return Err(error("single bounded tree metadata/root required"));
    }
    for i in 0..heads.len() {
        let mut j = i + 1;
        let mut seen = BTreeSet::new();
        while j != 0 {
            if j > heads.len() || !seen.insert(j) {
                return Err(error("head range/cycle"));
            }
            j = heads[j - 1];
        }
    }
    Ok(Tree {
        id: ids.remove(0),
        text: texts.remove(0),
        body: format!("{}\n\n", s.trim_end()),
        forms,
    })
}
fn prag(s: &str, id: &str) -> Result<String> {
    if s.contains(['\n', '\r']) {
        return Err(error("one pragmatic TSV row without header required"));
    }
    let f: Vec<_> = s.split('\t').collect();
    if f.len() != 8 || f[0] != id {
        return Err(error("prag ID/shape mismatch"));
    }
    prag_schema::parse_labels(&f[1..]).map_err(error)?;
    Ok(s.into())
}
fn target_labels(t: &Tree, fish: &Value) -> Result<()> {
    let p = Path::new(string(fish, "model_dir")?);
    let mut vocab = Vec::new();
    for n in ["xpos.txt", "upos.txt", "rels.txt"] {
        vocab.push(
            fs::read_to_string(p.join(n))?
                .lines()
                .skip(1)
                .map(String::from)
                .collect::<BTreeSet<_>>(),
        );
    }
    for l in t
        .body
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let f: Vec<_> = l.split('\t').collect();
        for (i, j) in [4, 3, 7].iter().enumerate() {
            if !vocab[i].contains(f[*j]) {
                return Err(error("unknown/reserved annotation target label"));
            }
        }
    }
    Ok(())
}
fn count_selected(c: &Connection) -> Result<u64> {
    Ok(c.query_row("SELECT count(*) FROM selection s JOIN versions v ON v.key=s.version_key WHERE v.eligible=1 AND v.prag IS NOT NULL AND v.split IN('train','dev')",[],|r|r.get(0))?)
}
/// Reserve one unique sentence for independent review. Any unfinished reservation blocks new work.
/// A restart must inspect status and resolve it explicitly; next never returns a previous task again.
pub fn next(config: &Value) -> Result<Option<Value>> {
    if config["dolphin_auto"] != true {
        return Ok(None);
    }
    let ppm = fraction_ppm(config)?;
    let (path, db, fish) = config_paths(config)?;
    let mut src = source(&db)?;
    let source_tx = src.transaction()?;
    let n = count_selected(&source_tx)?;
    let budget = quota(n, ppm);
    let mut q = queue(&path, &db)?;
    let tx = q.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let used: u64 = tx.query_row("SELECT count(*) FROM requests", [], |r| r.get(0))?;
    if used > budget {
        return Err(error(
            "existing reservations exceed current quota; no new audit permitted",
        ));
    }
    let pending: u64 = tx.query_row(
        "SELECT count(*) FROM requests WHERE state IN('reserved','submitted')",
        [],
        |r| r.get(0),
    )?;
    if used >= budget || pending > 0 {
        return Ok(None);
    }
    let mut statement=source_tx.prepare("SELECT v.id,v.key,v.tree_sha,v.prag_sha FROM selection s JOIN versions v ON v.key=s.version_key WHERE v.eligible=1 AND v.prag IS NOT NULL AND v.split IN('train','dev') ORDER BY v.id")?;
    let mut ranked = statement
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(statement);
    ranked.sort_by_key(|r| (hash(r.0.as_bytes()), r.0.clone()));
    for (id, key, th, ph) in ranked {
        let already: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM requests WHERE sent_id=?1)",
            [&id],
            |r| r.get(0),
        )?;
        if already {
            continue;
        }
        let (body, row, split, provenance): (String, String, String, String) = source_tx
            .query_row(
                "SELECT tree,prag,split,provenance FROM versions WHERE key=?1",
                [&key],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )?;
        let parsed = match tree(&body) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if parsed.id != id
            || hash(&body) != th
            || hash(&row) != ph
            || prag(&row, &id).is_err()
            || target_labels(&parsed, &fish).is_err()
        {
            continue;
        }
        let request_id = format!(
            "dolphin-{}",
            hash(serde_json::to_vec(&(&id, &key, &th, &ph))?)
        );
        let task=format!("Independent LOW PRIORITY linguistic audit of ONE sentence. This is review-only, not a training authorization or gold judgment. Do not run tools, read other data, change IDs/text/token forms/order, or write any DB. Review AST and pragmatic labels; preserve uncertainty in critique. Polarity is speaker evaluation, not grammatical negation alone. Return only JSON with request_id,base_tree_sha256,base_prag_sha256,tree (full CoNLL-U string),prag (one eight-column TSV row, no header),critique.\nrequest_id={request_id}\nbase_tree_sha256={th}\nbase_prag_sha256={ph}\nAST:\n{}PRAG:\n{}\n",parsed.body,row);
        if task.len() > PROMPT_LIMIT {
            continue;
        }
        let payload = json!({"request_id":request_id,"sent_id":id,"task":task,"priority":"low","review_only":true,"training_allowed":false,"base_tree_sha256":th,"base_prag_sha256":ph,"source_version_key":key,"source_provenance_sha256":hash(&provenance),"tokens":parsed.forms.len(),"prompt_bytes":task.len(),"reserved_selected_ids":n,"reserved_budget":budget,"fraction_ppm":ppm});
        tx.execute("INSERT INTO requests(request_id,sent_id,state,selected_key,base_tree_sha,base_prag_sha,source_tree,source_prag,source_split,source_provenance,source_provenance_sha,request_json,reason,reserved_n,reserved_budget,fraction_ppm) VALUES(?1,?2,'reserved',?3,?4,?5,?6,?7,?8,?9,?10,?11,'not dispatched by this module',?12,?13,?14)",params![request_id,id,key,th,ph,body,row,split,provenance,hash(&provenance),payload.to_string(),n,budget,ppm])?;
        tx.commit()?;
        source_tx.commit()?;
        return Ok(Some(payload));
    }
    Ok(None)
}

/// Preserve one structured independent revision as review-only, or quarantine the raw critique.
/// id is request_id, not sentence ID. An identical terminal report is idempotent; replacement refuses.
pub fn finish(config: &Value, id: &str, report: &Value) -> Result<Value> {
    let (path, db, fish) = config_paths(config)?;
    let rendered = serde_json::to_string(report)?;
    if rendered.len() > 128_000 {
        return Err(error(
            "report exceeds 128000 bytes; reservation retained for manual reconciliation",
        ));
    }
    let rh = hash(&rendered);
    let mut q = queue(&path, &db)?;
    let tx = q.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let reserved:Option<(String,String,String,String,String,String,String,Option<String>)>=tx.query_row("SELECT sent_id,selected_key,base_tree_sha,base_prag_sha,source_tree,source_prag,state,report_sha FROM requests WHERE request_id=?1",[id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?,r.get(7)?))).optional()?;
    let (sent_id, key, th, ph, body, old_prag, state, prior) =
        reserved.ok_or_else(|| error("unknown Dolphin request_id"))?;
    if let Some(prior) = prior {
        if prior != rh {
            return Err(error("terminal Dolphin report cannot be replaced"));
        }
        let reason: String = tx.query_row(
            "SELECT reason FROM requests WHERE request_id=?1",
            [id],
            |r| r.get(0),
        )?;
        return Ok(
            json!({"request_id":id,"sent_id":sent_id,"state":state,"reason":reason,"idempotent":true,"review_only":true,"training_allowed":false,"report_sha256":rh}),
        );
    }
    let validation = (|| -> Result<(String, String)> {
        if string(report, "request_id")? != id
            || string(report, "base_tree_sha256")? != th
            || string(report, "base_prag_sha256")? != ph
        {
            return Err(error("report/base revision binding mismatch"));
        }
        let original = tree(&body)?;
        if hash(&body) != th || hash(&old_prag) != ph {
            return Err(error("reserved source content hash mismatch"));
        }
        let src = source(&db)?;
        let current:Option<(String,String,String)>=src.query_row("SELECT v.key,v.tree_sha,v.prag_sha FROM selection s JOIN versions v ON v.key=s.version_key WHERE s.id=?1 AND v.eligible=1",[&sent_id],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        if current != Some((key.clone(), th.clone(), ph.clone())) {
            return Err(error("selected source version changed after reservation"));
        }
        let proposed = tree(string(report, "tree")?)?;
        if proposed.id != sent_id
            || proposed.text != original.text
            || proposed.forms != original.forms
        {
            return Err(error("proposed ID/text/token mismatch"));
        }
        target_labels(&proposed, &fish)?;
        let p = prag(string(report, "prag")?, &sent_id)?;
        Ok((proposed.body, p))
    })();
    let (result_state, reason, proposed_tree, proposed_prag) = match validation {
        Ok((t, p)) => (
            "completed",
            "validated independent revision; review-only, no training admission".to_string(),
            Some(t),
            Some(p),
        ),
        Err(e) => ("quarantined", e.to_string(), None, None),
    };
    tx.execute("UPDATE requests SET state=?2,report_json=?3,report_sha=?4,review_tree=?5,review_prag=?6,reason=?7 WHERE request_id=?1 AND report_sha IS NULL",params![id,result_state,rendered,rh,proposed_tree,proposed_prag,reason])?;
    tx.commit()?;
    Ok(
        json!({"request_id":id,"sent_id":sent_id,"state":result_state,"reason":reason,"idempotent":false,"review_only":true,"training_allowed":false,"report_sha256":rh,"revision_tree_sha256":proposed_tree.as_ref().map(hash),"revision_prag_sha256":proposed_prag.as_ref().map(hash),"semantic_correctness":"independent review, not established gold"}),
    )
}
/// Read reservation metadata for explicit reconciliation. Pending tasks are never returned for replay.
pub fn status(config: &Value) -> Result<Value> {
    let (path, db, _) = config_paths(config)?;
    let ppm = fraction_ppm(config)?;
    let mut src = source(&db)?;
    let st = src.transaction()?;
    let n = count_selected(&st)?;
    let budget = quota(n, ppm);
    if !path.exists() {
        return Ok(
            json!({"schema":"dolphin-review-only-v1","selected_ids":n,"quota":budget,"fraction_ppm":ppm,"lifetime_reserved_ids":0,"pending":[],"training_allowed":false,"replay_allowed":false}),
        );
    }
    let mut q = queue(&path, &db)?;
    let tx = q.transaction()?;
    let used: u64 = tx.query_row("SELECT count(*) FROM requests", [], |r| r.get(0))?;
    let mut states = BTreeMap::new();
    let mut state_rows =
        tx.prepare("SELECT state,count(*) FROM requests GROUP BY state ORDER BY state")?;
    for r in state_rows.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, u64>(1)?)))? {
        let (s, n) = r?;
        states.insert(s, n);
    }
    drop(state_rows);
    let mut pending = Vec::new();
    let mut ps=tx.prepare("SELECT request_id,sent_id,state,base_tree_sha,base_prag_sha FROM requests WHERE state IN('reserved','submitted') ORDER BY request_id")?;
    for row in ps.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, String>(4)?,
        ))
    })? {
        let (id, sent, state, th, ph) = row?;
        pending.push(json!({"request_id":id,"sent_id":sent,"state":state,"base_tree_sha256":th,"base_prag_sha256":ph,"replay_allowed":false,"resolution":"inspect the original caller/terminal log, then finish explicitly; never auto-submit"}));
    }
    drop(ps);
    tx.commit()?;
    st.commit()?;
    Ok(
        json!({"schema":"dolphin-review-only-v1","selected_ids":n,"quota":budget,"fraction_ppm":ppm,"lifetime_reserved_ids":used,"quota_breached":used>budget,"states":states,"pending":pending,"training_allowed":false,"replay_allowed":false,"main_db_mutated":false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: PathBuf,
        db: PathBuf,
        c: Value,
    }
    fn sample(id: &str) -> String {
        format!("# sent_id = {id}\n# text = Word.\n1\tWord\tword\tNOUN\tNN\t_\t0\troot\t_\t_\n2\t.\t.\tPUNCT\t.\t_\t1\tpunct\t_\t_\n\n")
    }
    impl Fixture {
        fn new(n: usize) -> Self {
            let root = std::env::temp_dir().join(format!(
                "dolphin-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::SeqCst)
            ));
            fs::create_dir(&root).unwrap();
            let db = root.join("annotations.sqlite3");
            let src = Connection::open(&db).unwrap();
            src.execute_batch("CREATE TABLE meta(key TEXT PRIMARY KEY,value TEXT);INSERT INTO meta VALUES('schema','peta-versions-v1');CREATE TABLE versions(key TEXT PRIMARY KEY,id TEXT,tree_sha TEXT,prag_sha TEXT,tree TEXT,prag TEXT,split TEXT,eligible INTEGER,provenance TEXT);CREATE TABLE selection(id TEXT PRIMARY KEY,version_key TEXT);").unwrap();
            for i in 0..n {
                let id = format!("school:fixture:lesson{i}:0");
                let b = sample(&id);
                let p = format!("{id}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture");
                let key = format!("version{i}");
                src.execute("INSERT INTO versions VALUES(?1,?2,?3,?4,?5,?6,?7,1,'{\"source\":\"fixture\"}')",params![key,id,hash(&b),hash(&p),b,p,if i%2==0{"train"}else{"dev"}]).unwrap();
                src.execute("INSERT INTO selection VALUES(?1,?2)", params![id, key])
                    .unwrap();
            }
            drop(src);
            let model = root.join("model");
            fs::create_dir(&model).unwrap();
            for (name, body) in [
                ("xpos.txt", "_\nNN\n.\n"),
                ("upos.txt", "_\nNOUN\nPUNCT\n"),
                ("rels.txt", "_\nroot\npunct\n"),
            ] {
                fs::write(model.join(name), body).unwrap();
            }
            let fish = root.join("fish.json");
            fs::write(&fish, json!({"db":db,"model_dir":model}).to_string()).unwrap();
            let c = json!({"run_root":root,"fish_config":fish,"dolphin_fraction":0.01,"dolphin_auto":true});
            Self { root, db, c }
        }
        fn report(&self, request: &Value) -> Value {
            let src = Connection::open(&self.db).unwrap();
            let (b, p): (String, String) = src
                .query_row(
                    "SELECT tree,prag FROM versions WHERE key=?1",
                    [request["source_version_key"].as_str().unwrap()],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .unwrap();
            json!({"request_id":request["request_id"],"base_tree_sha256":request["base_tree_sha256"],"base_prag_sha256":request["base_prag_sha256"],"tree":b,"prag":p,"critique":"Independent bounded fixture review"})
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn hard_floor_quota_small_n_and_default() {
        let f = Fixture::new(99);
        assert!(next(&f.c).unwrap().is_none());
        assert_eq!(status(&f.c).unwrap()["quota"], 0);
        let f = Fixture::new(100);
        let mut disabled = f.c.clone();
        disabled["dolphin_auto"] = json!(false);
        assert!(next(&disabled).unwrap().is_none());
        disabled["dolphin_auto"] = json!(true);
        disabled.as_object_mut().unwrap().remove("dolphin_fraction");
        assert!(next(&disabled).unwrap().is_none());
        assert_eq!(status(&disabled).unwrap()["quota"], 0);
        assert!(next(&f.c).unwrap().is_some());
        assert_eq!(status(&f.c).unwrap()["lifetime_reserved_ids"], 1);
        assert!(next(&f.c).unwrap().is_none());
        let mut high = f.c.clone();
        high["dolphin_fraction"] = json!(0.011);
        assert!(next(&high).is_err());
        assert_eq!(
            fraction_ppm(&json!({"dolphin_fraction":0.0000001})).unwrap(),
            0
        );
    }
    #[test]
    fn restart_never_replays_pending_and_quarantine_consumes_quota() {
        let f = Fixture::new(100);
        let r = next(&f.c).unwrap().unwrap();
        assert!(next(&f.c).unwrap().is_none());
        let s = status(&f.c).unwrap();
        assert_eq!(s["pending"].as_array().unwrap().len(), 1);
        assert_eq!(s["pending"][0]["replay_allowed"], false);
        let critique = json!("Plain critique without a structured annotation");
        let out = finish(&f.c, r["request_id"].as_str().unwrap(), &critique).unwrap();
        assert_eq!(out["state"], "quarantined");
        assert_eq!(out["training_allowed"], false);
        assert!(next(&f.c).unwrap().is_none());
        let audit = Connection::open(f.root.join("dolphin.sqlite3")).unwrap();
        let stored: String = audit
            .query_row("SELECT report_json FROM requests", [], |r| r.get(0))
            .unwrap();
        assert_eq!(stored, critique.to_string());
    }
    #[test]
    fn valid_revision_is_distinct_review_only_and_main_unchanged() {
        let f = Fixture::new(200);
        let before = hash(fs::read(&f.db).unwrap());
        let r = next(&f.c).unwrap().unwrap();
        let mut report = f.report(&r);
        report["tree"] = json!(report["tree"]
            .as_str()
            .unwrap()
            .replace("Word\tword\t", "Word\trevised\t"));
        let out = finish(&f.c, r["request_id"].as_str().unwrap(), &report).unwrap();
        assert_eq!(out["state"], "completed");
        assert_eq!(out["training_allowed"], false);
        assert_ne!(out["revision_tree_sha256"], r["base_tree_sha256"]);
        assert_eq!(before, hash(fs::read(&f.db).unwrap()));
        let again = finish(&f.c, r["request_id"].as_str().unwrap(), &report).unwrap();
        assert_eq!(again["idempotent"], true);
        report["critique"] = json!("different");
        assert!(finish(&f.c, r["request_id"].as_str().unwrap(), &report).is_err());
        let r2 = next(&f.c).unwrap().unwrap();
        assert_ne!(r["sent_id"], r2["sent_id"]);
        finish(&f.c, r2["request_id"].as_str().unwrap(), &f.report(&r2)).unwrap();
        assert!(next(&f.c).unwrap().is_none());
        assert_eq!(status(&f.c).unwrap()["lifetime_reserved_ids"], 2);
    }
    #[test]
    fn changed_source_and_malformed_reports_quarantine() {
        let f = Fixture::new(200);
        let r = next(&f.c).unwrap().unwrap();
        let report = f.report(&r);
        let src = Connection::open(&f.db).unwrap();
        src.execute(
            "UPDATE versions SET prag_sha='changed' WHERE key=?1",
            [r["source_version_key"].as_str().unwrap()],
        )
        .unwrap();
        drop(src);
        let out = finish(&f.c, r["request_id"].as_str().unwrap(), &report).unwrap();
        assert_eq!(out["state"], "quarantined");
        assert!(out["reason"].as_str().unwrap().contains("changed"));
        let r2 = next(&f.c).unwrap().unwrap();
        let mut report = f.report(&r2);
        report["prag"] = json!(report["prag"]
            .as_str()
            .unwrap()
            .replace("\tdecl\t", "\tBAD\t"));
        assert_eq!(
            finish(&f.c, r2["request_id"].as_str().unwrap(), &report).unwrap()["state"],
            "quarantined"
        );
    }
    #[test]
    fn concurrent_reservations_are_single_and_never_exceed_quota() {
        let f = Fixture::new(500);
        let handles = (0..4)
            .map(|_| {
                let c = f.c.clone();
                std::thread::spawn(move || next(&c).unwrap().is_some())
            })
            .collect::<Vec<_>>();
        let wins = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .filter(|x| *x)
            .count();
        assert_eq!(wins, 1);
        let s = status(&f.c).unwrap();
        assert_eq!(s["lifetime_reserved_ids"], 1);
        assert_eq!(s["quota"], 5);
    }
    #[test]
    fn malformed_tree_and_base_echo_are_rejected() {
        let f = Fixture::new(200);
        let r = next(&f.c).unwrap().unwrap();
        let mut report = f.report(&r);
        report["base_tree_sha256"] = json!("wrong");
        assert_eq!(
            finish(&f.c, r["request_id"].as_str().unwrap(), &report).unwrap()["state"],
            "quarantined"
        );
        let r = next(&f.c).unwrap().unwrap();
        let mut report = f.report(&r);
        report["tree"] = json!(report["tree"]
            .as_str()
            .unwrap()
            .replace("\t0\troot\t", "\t1\troot\t"));
        assert_eq!(
            finish(&f.c, r["request_id"].as_str().unwrap(), &report).unwrap()["state"],
            "quarantined"
        );
        assert!(tree("1\tWord\tword\tNOUN\tNN\t_\t0\troot\t_\t_\n").is_err());
        assert!(tree(&sample("school:test:0").replace("\tNN\t_\t", "\tNN\tBAD\t")).is_err());
    }
    #[test]
    fn token_and_prompt_bounds_never_reserve_oversize_input() {
        let f = Fixture::new(100);
        let src = Connection::open(&f.db).unwrap();
        let ids = src
            .prepare("SELECT id FROM versions ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<_>>>()
            .unwrap();
        for id in &ids {
            let mut b = format!("# sent_id = {id}\n# text = Long bounded control.\n");
            for i in 1..=97 {
                b.push_str(&format!(
                    "{i}\tWord\tword\tNOUN\tNN\t_\t{}\t{}\t_\t_\n",
                    if i == 1 { 0 } else { 1 },
                    if i == 1 { "root" } else { "dep" }
                ));
            }
            b.push('\n');
            src.execute(
                "UPDATE versions SET tree=?2,tree_sha=?3 WHERE id=?1",
                params![id, b, hash(&b)],
            )
            .unwrap();
        }
        assert!(next(&f.c).unwrap().is_none());
        for id in &ids {
            let b = sample(id).replace("# text = Word.", &format!("# text = {}", "Ж".repeat(7000)));
            src.execute(
                "UPDATE versions SET tree=?2,tree_sha=?3 WHERE id=?1",
                params![id, b, hash(&b)],
            )
            .unwrap();
        }
        assert!(next(&f.c).unwrap().is_none());
        assert_eq!(status(&f.c).unwrap()["lifetime_reserved_ids"], 0);
    }
    #[test]
    fn development_revision_remains_outside_training() {
        let f = Fixture::new(100);
        let src = Connection::open(&f.db).unwrap();
        src.execute("UPDATE versions SET split='dev'", []).unwrap();
        drop(src);
        let before = hash(fs::read(&f.db).unwrap());
        let r = next(&f.c).unwrap().unwrap();
        let result = finish(&f.c, r["request_id"].as_str().unwrap(), &f.report(&r)).unwrap();
        assert_eq!(result["state"], "completed");
        assert_eq!(result["review_only"], true);
        assert_eq!(result["training_allowed"], false);
        assert_eq!(before, hash(fs::read(&f.db).unwrap()));
        let q = Connection::open(f.root.join("dolphin.sqlite3")).unwrap();
        let split: String = q
            .query_row("SELECT source_split FROM requests", [], |r| r.get(0))
            .unwrap();
        assert_eq!(split, "dev");
    }
    #[test]
    fn bootstrap_lease_wait_is_bounded_and_release_recovers() {
        let f = Fixture::new(100);
        let path = f.root.join("dolphin.bootstrap.lock");
        let owner = bootstrap_lock(&path, Duration::from_millis(20)).unwrap();
        let start = Instant::now();
        assert!(bootstrap_lock(&path, Duration::from_millis(20)).is_err());
        assert!(start.elapsed() >= Duration::from_millis(20));
        assert!(start.elapsed() < Duration::from_secs(2));
        assert!(!f.root.join("dolphin.sqlite3").exists());
        drop(owner);
        assert!(next(&f.c).unwrap().is_some());
    }
}
