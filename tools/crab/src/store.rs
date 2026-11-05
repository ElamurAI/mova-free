//! Append-only SQLite annotation versions; selected full teacher revisions have explicit eligibility.
use crate::Result;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};
#[allow(dead_code)]
#[path = "../../../prag/src/schema.rs"]
mod prag_schema;
const HEADER: &str = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn err(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| err(format!("missing string {k}")))
}
fn file_hash(p: &Path) -> Result<String> {
    let mut f = File::open(p)?;
    let mut h = Sha256::new();
    let mut b = vec![0; 1 << 20];
    loop {
        let n = f.read(&mut b)?;
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn pinned(c: &Value, k: &str) -> Result<PathBuf> {
    let p = PathBuf::from(field(c, k)?);
    if !p.is_absolute() {
        return Err(err(format!("{k} must be absolute")));
    }
    if file_hash(&p)? != field(c, &format!("{k}_sha256"))? {
        return Err(err(format!("{k} hash mismatch")));
    }
    Ok(p)
}
fn lines(p: &Path) -> Result<Vec<String>> {
    Ok(BufReader::new(File::open(p)?)
        .lines()
        .collect::<std::io::Result<_>>()?)
}
#[derive(Clone, Debug)]
struct Tree {
    id: String,
    text: String,
    body: String,
    raw_sha: String,
    forms: Vec<String>,
}
fn tree(b: &str) -> Result<Tree> {
    let mut active = false;
    for l in b.lines() {
        if l.starts_with("# sent_id = ") {
            active = true;
        } else if l.starts_with("# sent_id") {
            return Err(err("malformed sent_id header"));
        } else if !l.is_empty() && !l.starts_with('#') && !active {
            return Err(err("token fragment before sent_id"));
        }
    }
    let ids: Vec<_> = b
        .lines()
        .filter_map(|l| l.strip_prefix("# sent_id = "))
        .collect();
    let txt: Vec<_> = b
        .lines()
        .filter_map(|l| l.strip_prefix("# text = "))
        .collect();
    if ids.len() != 1 || txt.len() != 1 || ids[0].is_empty() || txt[0].is_empty() {
        return Err(err("tree needs exactly one ID/text"));
    }
    let mut heads = Vec::new();
    let mut forms = Vec::new();
    for l in b.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<_> = l.split('\t').collect();
        if f.len() != 10
            || f.iter().any(|x| x.is_empty())
            || f[0].parse::<usize>().ok() != Some(heads.len() + 1)
        {
            return Err(err("integer-token ten-column requirement"));
        }
        let h: usize = f[6].parse()?;
        if (h == 0) != (f[7] == "root") {
            return Err(err("root relation mismatch"));
        }
        heads.push(h);
        forms.push(f[1].into());
    }
    if heads.is_empty() || heads.iter().filter(|&&h| h == 0).count() != 1 {
        return Err(err("root count"));
    }
    for i in 0..heads.len() {
        let mut j = i + 1;
        let mut seen = BTreeSet::new();
        while j != 0 {
            if j > heads.len() || !seen.insert(j) {
                return Err(err("head cycle/range"));
            }
            j = heads[j - 1];
        }
    }
    Ok(Tree {
        id: ids[0].into(),
        text: txt[0].into(),
        body: format!("{}\n\n", b.trim_end()),
        raw_sha: hash(b.as_bytes()),
        forms,
    })
}
fn trees(p: &Path) -> Result<BTreeMap<String, Tree>> {
    let s = fs::read_to_string(p)?;
    let mut out = BTreeMap::new();
    for b in s.split_inclusive("\n\n") {
        if b.trim().is_empty() {
            continue;
        }
        let t = tree(b)?;
        if out.insert(t.id.clone(), t).is_some() {
            return Err(err("duplicate tree ID"));
        }
    }
    Ok(out)
}
// Open parsers may omit # text. Only identity/token-bound metadata is adapted, never predictions.
fn witness_trees(p: &Path, teacher: &BTreeMap<String, Tree>) -> Result<BTreeMap<String, Tree>> {
    let s = fs::read_to_string(p)?;
    let mut out = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for b in s.split_inclusive("\n\n") {
        if b.trim().is_empty() {
            continue;
        }
        let ids: Vec<_> = b
            .lines()
            .filter_map(|l| l.strip_prefix("# sent_id = "))
            .collect();
        if ids.len() != 1 {
            continue;
        }
        let id = ids[0];
        let Some(t) = teacher.get(id) else { continue };
        if !seen.insert(id) {
            return Err(err("duplicate selected witness ID"));
        }
        let forms: Option<Vec<String>> = b
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .enumerate()
            .map(|(i, l)| {
                let f: Vec<_> = l.split('\t').collect();
                if f.len() != 10 || f[0].parse::<usize>().ok() != Some(i + 1) {
                    None
                } else {
                    Some(f[1].to_owned())
                }
            })
            .collect();
        if forms.as_ref() != Some(&t.forms) {
            continue;
        }
        let texts: Vec<_> = b
            .lines()
            .filter_map(|l| l.strip_prefix("# text = "))
            .collect();
        let adapted = if texts.is_empty() {
            format!("# text = {}\n{}", t.text, b)
        } else if texts.len() == 1 && texts[0] == t.text {
            b.to_owned()
        } else {
            continue;
        };
        if let Ok(w) = tree(&adapted) {
            out.insert(id.into(), w);
        }
    }
    Ok(out)
}
fn prags(p: &Path) -> Result<BTreeMap<String, String>> {
    let s = fs::read_to_string(p)?;
    let mut ls = s.lines();
    if ls.next() != Some(HEADER) {
        return Err(err("pragmatic header"));
    }
    let mut out = BTreeMap::new();
    for l in ls {
        let f: Vec<_> = l.split('\t').collect();
        if f.len() != 8 || f[0].is_empty() {
            return Err(err("pragmatic row shape"));
        }
        prag_schema::parse_labels(&f[1..]).map_err(err)?;
        if out.insert(f[0].into(), l.into()).is_some() {
            return Err(err("duplicate pragmatic ID"));
        }
    }
    Ok(out)
}
struct Gate {
    phases: BTreeMap<String, Value>,
    current: BTreeMap<String, String>,
    held_ids: BTreeSet<String>,
    held_text: BTreeSet<String>,
    quarantine: BTreeSet<String>,
    vocab: [BTreeSet<String>; 3],
    identity: String,
}
impl Gate {
    fn load(c: &Value) -> Result<Self> {
        let phase = pinned(c, "phase_index")?;
        let current = pinned(c, "current_school_tsv")?;
        let held = pinned(c, "heldout")?;
        let mut phases = BTreeMap::new();
        for l in lines(&phase)? {
            if l.is_empty() {
                continue;
            }
            let v: Value = serde_json::from_str(&l)?;
            let id = field(&v, "id")?.to_string();
            if phases.insert(id, v).is_some() {
                return Err(err("duplicate phase ID"));
            }
        }
        let mut cur = BTreeMap::new();
        for l in lines(&current)? {
            let f: Vec<_> = l.splitn(4, '\t').collect();
            if f.len() != 4 {
                return Err(err("school TSV shape"));
            }
            if cur
                .insert(format!("school:{}:{}:{}", f[0], f[1], f[2]), f[3].into())
                .is_some()
            {
                return Err(err("duplicate current ID"));
            }
        }
        let mut held_ids = BTreeSet::new();
        let mut held_text = BTreeSet::new();
        for l in lines(&held)? {
            if let Some(x) = l.strip_prefix("# sent_id = ") {
                held_ids.insert(x.into());
            }
            if let Some(x) = l.strip_prefix("# text = ") {
                held_text.insert(hash(x.as_bytes()));
            }
        }
        let quarantine: BTreeSet<_> = c["quarantine"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(Value::as_str)
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
        let model = Path::new(
            c["model_dir"]
                .as_str()
                .or_else(|| c["gparse_model"].as_str())
                .ok_or_else(|| err("model_dir required"))?,
        );
        let mut vocab: [BTreeSet<String>; 3] = Default::default();
        let mut vh = Vec::new();
        for (i, n) in ["xpos.txt", "upos.txt", "rels.txt"].iter().enumerate() {
            let s = fs::read_to_string(model.join(n))?;
            vh.push(hash(s.as_bytes()));
            vocab[i] = s.lines().skip(1).map(String::from).collect();
        }
        let identity=hash(serde_json::to_string(&json!({"phase":c["phase_index_sha256"],"current":c["current_school_tsv_sha256"],"heldout":c["heldout_sha256"],"quarantine":quarantine,"target_vocabs":vh,"policy":"verified teaching lesson sha256[0]%10"}))?.as_bytes());
        Ok(Self {
            phases,
            current: cur,
            held_ids,
            held_text,
            quarantine,
            vocab,
            identity,
        })
    }
    fn forbidden(&self, t: &Tree) -> Option<&'static str> {
        if self.held_ids.contains(&t.id) || self.held_text.contains(&hash(t.text.as_bytes())) {
            Some("heldout ID/text")
        } else if self.quarantine.contains(&t.id) {
            Some("operator quarantine/joined source")
        } else {
            None
        }
    }
    fn eligible(&self, t: &Tree) -> std::result::Result<String, String> {
        if let Some(r) = self.forbidden(t) {
            return Err(r.into());
        }
        if !t.id.starts_with("school:") {
            return Err("non-school teaching scope".into());
        }
        let p = self.phases.get(&t.id).ok_or("missing phase")?;
        if p["phase"] != "teaching_text_explain_or_fact"
            || p["state"] != "verified_school_sents_613a642_and_head"
            || p["training_allowed"] != false
            || p["review_only"] == true
            || p["heldout"] == true
            || p["observed_text_sha256"] != hash(t.text.as_bytes())
        {
            return Err("unverified phase/revision".into());
        }
        if self.current.get(&t.id) != Some(&t.text) {
            return Err("current teaching text mismatch".into());
        }
        for l in t
            .body
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            let f: Vec<_> = l.split('\t').collect();
            for (i, j) in [4, 3, 7].iter().enumerate() {
                if !self.vocab[i].contains(f[*j]) {
                    return Err("unknown/reserved target vocabulary".into());
                }
            }
        }
        let lesson = t.id.rsplit_once(':').ok_or("lesson suffix")?.0;
        Ok(if Sha256::digest(lesson.as_bytes())[0] % 10 == 0 {
            "dev"
        } else {
            "train"
        }
        .into())
    }
}
fn open(c: &Value, gate: Option<&Gate>) -> Result<Connection> {
    let db = Path::new(field(c, "db")?);
    if !db.is_absolute() {
        return Err(err("absolute db required"));
    }
    if let Some(p) = db.parent() {
        fs::create_dir_all(p)?;
    }
    let con = Connection::open(db)?;
    con.busy_timeout(Duration::from_secs(30))?;
    con.execute_batch("PRAGMA journal_mode=WAL;PRAGMA synchronous=FULL;PRAGMA foreign_keys=ON;CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);CREATE TABLE IF NOT EXISTS batches(key TEXT PRIMARY KEY,sha256 TEXT NOT NULL,status TEXT NOT NULL,payload TEXT NOT NULL,detail TEXT NOT NULL);CREATE TABLE IF NOT EXISTS versions(key TEXT PRIMARY KEY,id TEXT NOT NULL,source TEXT NOT NULL,tree_sha TEXT NOT NULL,prag_sha TEXT NOT NULL,tree TEXT NOT NULL,prag TEXT,eligible INTEGER NOT NULL,split TEXT,reason TEXT NOT NULL,provenance TEXT NOT NULL);CREATE TABLE IF NOT EXISTS batch_versions(batch_key TEXT NOT NULL,version_key TEXT NOT NULL,PRIMARY KEY(batch_key,version_key),FOREIGN KEY(batch_key) REFERENCES batches(key),FOREIGN KEY(version_key) REFERENCES versions(key));CREATE TABLE IF NOT EXISTS selection(id TEXT PRIMARY KEY,version_key TEXT NOT NULL,FOREIGN KEY(version_key) REFERENCES versions(key));CREATE TABLE IF NOT EXISTS rejections(batch_key TEXT NOT NULL,id TEXT NOT NULL,reason TEXT NOT NULL,PRIMARY KEY(batch_key,id,reason),FOREIGN KEY(batch_key) REFERENCES batches(key));CREATE INDEX IF NOT EXISTS versions_id_source ON versions(id,source);CREATE INDEX IF NOT EXISTS batch_versions_version ON batch_versions(version_key,batch_key);CREATE INDEX IF NOT EXISTS versions_pair ON versions(id,tree_sha,prag_sha);")?;
    let s: Option<String> = con
        .query_row("SELECT value FROM meta WHERE key='schema'", [], |r| {
            r.get(0)
        })
        .optional()?;
    if s.as_deref().is_some_and(|s| s != "peta-versions-v1") {
        return Err(err("unsupported store schema"));
    }
    con.execute(
        "INSERT OR IGNORE INTO meta VALUES('schema','peta-versions-v1')",
        [],
    )?;
    if let Some(g) = gate {
        con.execute(
            "INSERT OR IGNORE INTO meta VALUES('eligibility',?1)",
            [&g.identity],
        )?;
        let old: String =
            con.query_row("SELECT value FROM meta WHERE key='eligibility'", [], |r| {
                r.get(0)
            })?;
        if old != g.identity {
            return Err(err(
                "eligibility inputs changed; deliberate new policy/store required",
            ));
        }
    }
    Ok(con)
}
fn seen(con: &Connection, key: &str, sha: &str) -> Result<bool> {
    let old: Option<String> = con
        .query_row("SELECT sha256 FROM batches WHERE key=?1", [key], |r| {
            r.get(0)
        })
        .optional()?;
    match old {
        Some(s) if s != sha => Err(err(format!("mutated receipt {key}"))),
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
fn begin(con: &Connection, key: &str, sha: &str, payload: &str) -> Result<()> {
    con.execute(
        "INSERT INTO batches VALUES(?1,?2,'processing',?3,'')",
        params![key, sha, payload],
    )?;
    Ok(())
}
fn reject(con: &Connection, b: &str, id: &str, why: &str) -> Result<()> {
    con.execute(
        "INSERT OR IGNORE INTO rejections VALUES(?1,?2,?3)",
        params![b, id, why],
    )?;
    Ok(())
}
fn version(
    con: &Connection,
    b: &str,
    t: &Tree,
    source: &str,
    prag: Option<&str>,
    split: Option<&str>,
    reason: &str,
    provenance: &str,
) -> Result<()> {
    let th = hash(t.body.as_bytes());
    let ph = prag.map(|x| hash(x.as_bytes())).unwrap_or_default();
    let key = hash(serde_json::to_string(&(source, &t.id, &th, &ph))?.as_bytes());
    con.execute(
        "INSERT OR IGNORE INTO versions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
        params![
            key,
            t.id,
            source,
            th,
            ph,
            t.body,
            prag,
            split.is_some() as i32,
            split,
            reason,
            provenance
        ],
    )?;
    con.execute(
        "INSERT OR IGNORE INTO batch_versions VALUES(?1,?2)",
        params![b, key],
    )?;
    if split.is_some() {
        let stored: (i64, Option<String>) = con.query_row(
            "SELECT eligible,split FROM versions WHERE key=?1",
            [&key],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        if stored.0 != 1 || stored.1.as_deref() != split {
            return Err(err("existing version eligibility/split mismatch"));
        }
        let old:Option<(String,String)>=con.query_row("SELECT v.tree_sha,v.prag_sha FROM selection s JOIN versions v ON s.version_key=v.key WHERE s.id=?1",[&t.id],|r|Ok((r.get(0)?,r.get(1)?))).optional()?;
        if let Some((a, p)) = old {
            if a != th || p != ph {
                reject(
                    con,
                    b,
                    &t.id,
                    "conflicting eligible revision retained; prior selection preserved",
                )?;
            }
        } else {
            con.execute("INSERT INTO selection VALUES(?1,?2)", params![t.id, key])?;
        }
    }
    Ok(())
}
fn seed(c: &Value, g: &Gate, con: &mut Connection) -> Result<()> {
    if c["seed_decisions"].is_null() {
        return Ok(());
    }
    let dp = pinned(c, "seed_decisions")?;
    let key = "seed-reviewed";
    let sha = file_hash(&dp)?;
    if seen(con, key, &sha)? {
        return Ok(());
    }
    let ts = trees(&pinned(c, "seed_reviewed_trees")?)?;
    let ps = prags(&pinned(c, "seed_reviewed_prag")?)?;
    let ds = fs::read_to_string(dp)?;
    let mut dec = BTreeMap::new();
    for l in ds.lines() {
        let d: Value = serde_json::from_str(l)?;
        let id = field(&d, "sent_id")?.to_string();
        if dec.insert(id, d).is_some() {
            return Err(err("duplicate seed decision"));
        }
    }
    if ts.len() != ps.len() || ts.len() != dec.len() || ts.is_empty() {
        return Err(err("seed pair cardinality"));
    }
    let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
    if seen(&tx, key, &sha)? {
        return Ok(());
    }
    begin(&tx, key, &sha, &ds)?;
    for (id, t) in &ts {
        let p = ps.get(id).ok_or_else(|| err("seed missing prag"))?;
        let d = dec.get(id).ok_or_else(|| err("seed missing decision"))?;
        if d["decision"] != "allow"
            || !field(d, "authorization")?.starts_with("user:")
            || d["tree_sha256"] != hash(t.body.as_bytes())
            || d["prag_sha256"] != hash(p.as_bytes())
            || d["source_text_sha256"] != hash(t.text.as_bytes())
            || d["heldout"] != false
            || d["joined_source"] != false
        {
            return Err(err("seed binding mismatch"));
        }
        let split = g.eligible(t).map_err(err)?;
        if d["training_split"] != split {
            return Err(err("seed lesson split mismatch"));
        }
        version(
            &tx,
            key,
            t,
            "peta-reviewed-seed",
            Some(p),
            Some(&split),
            "explicit reviewed seed",
            &serde_json::to_string(d)?,
        )?;
    }
    tx.execute(
        "UPDATE batches SET status='committed',detail='content-bound seed' WHERE key=?1",
        [key],
    )?;
    tx.commit()?;
    Ok(())
}

struct Campaign {
    root: PathBuf,
    source_sha: String,
    queue_sha: String,
    queue: BTreeMap<String, Value>,
    identity: Value,
}
fn campaign(v: &Value) -> Result<Campaign> {
    let (root, cfg, cp) = if let Some(s) = v.as_str() {
        let r = PathBuf::from(s);
        let p = r
            .parent()
            .ok_or_else(|| err("campaign root parent"))?
            .join("config.json");
        (r, serde_json::from_slice::<Value>(&fs::read(&p)?)?, p)
    } else {
        (
            PathBuf::from(field(v, "root")?),
            serde_json::from_slice::<Value>(&fs::read(field(v, "config")?)?)?,
            PathBuf::from(field(v, "config")?),
        )
    };
    if !root.is_absolute() {
        return Err(err("campaign root absolute"));
    }
    let identity: Value = serde_json::from_slice(&fs::read(root.join("campaign.json"))?)?;
    if identity["config_sha256"] != file_hash(&cp)? {
        return Err(err("campaign config identity mismatch"));
    }
    if identity["heldout_sha256"] != file_hash(Path::new(field(&cfg, "heldout")?))? {
        return Err(err("campaign context-exclusion identity mismatch"));
    }
    let source = pinned(&cfg, "source")?;
    let q = pinned(&cfg, "queue")?;
    let source_sha = file_hash(&source)?;
    let queue_sha = file_hash(&q)?;
    if identity["source_sha256"] != source_sha || identity["queue_sha256"] != queue_sha {
        return Err(err("campaign/source/queue identity mismatch"));
    }
    let mut queue = BTreeMap::new();
    for l in lines(&q)? {
        if l.is_empty() {
            continue;
        }
        let item: Value = serde_json::from_str(&l)?;
        let id = field(&item, "id")?.to_string();
        if queue.insert(id, item).is_some() {
            return Err(err("duplicate campaign queue ID"));
        }
    }
    Ok(Campaign {
        root,
        source_sha,
        queue_sha,
        queue,
        identity,
    })
}
const WITNESSES: &[(&str, &str)] = &[
    ("cycle/round1/pre/out/stanza.conllu", "stanza"),
    ("cycle/round1/pre/out/trankit.conllu", "trankit"),
    ("cycle/round1/pre/out/spacy-trf.conllu", "spacy-trf"),
    ("cycle/round1/mmm-gparse.conllu", "gparse"),
    ("cycle/round1/rust-b.conllu", "rust-witness"),
    ("cycle/round1/udpipe-c.conllu", "udpipe-witness"),
    ("cycle/round1/opus-a.conllu", "legacy-named-neural-witness"),
    ("cycle/round1/pre/agreed.conllu", "pre-treeagree"),
    ("agreed.conllu", "job-agreed"),
];
fn quarantine_batch(
    con: &mut Connection,
    key: &str,
    sha: &str,
    payload: &str,
    why: &str,
    accepted: &Value,
) -> Result<()> {
    let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
    if !seen(&tx, key, sha)? {
        begin(&tx, key, sha, payload)?;
        reject(&tx, key, "", why)?;
        let lost = accepted.as_array().map(Vec::len).unwrap_or(0);
        if let Some(ids) = accepted.as_array() {
            for id in ids.iter().filter_map(Value::as_str) {
                reject(
                    &tx,
                    key,
                    id,
                    "validity of whole batch unresolved; accepted ID not imported",
                )?;
            }
        }
        let detail = json!({"error":why,"lost_accepted_ids":lost,"accepted":accepted,"recovery":"explicitly investigate and create a corrected immutable campaign; no automatic replay of quarantined receipt"});
        tx.execute(
            "UPDATE batches SET status='quarantined',detail=?2 WHERE key=?1",
            params![key, detail.to_string()],
        )?;
    }
    tx.commit()?;
    Ok(())
}
fn ingest_job(con: &mut Connection, g: &Gate, c: &Campaign, path: &Path) -> Result<()> {
    let bytes = fs::read(path)?;
    let sha = hash(&bytes);
    let key = format!(
        "{}:{}",
        c.root.display(),
        path.file_name().unwrap().to_string_lossy()
    );
    if seen(con, &key, &sha)? {
        return Ok(());
    }
    let payload_owned=match std::str::from_utf8(&bytes){Ok(s)=>s.to_owned(),Err(_)=>json!({"invalid_utf8_receipt_hex":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>()}).to_string()};
    let payload = payload_owned.as_str();
    let r: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(e) => {
            return quarantine_batch(
                con,
                &key,
                &sha,
                payload,
                &format!("malformed receipt: {e}"),
                &Value::Null,
            )
        }
    };
    let Some(job) = r["job"].as_u64() else {
        return quarantine_batch(
            con,
            &key,
            &sha,
            payload,
            "receipt job missing",
            &r["details"]["accepted"],
        );
    };
    if path.file_name().unwrap().to_string_lossy() != format!("{job:08}.json")
        || r["source_sha256"] != c.source_sha
        || r["queue_sha256"] != c.queue_sha
    {
        return quarantine_batch(
            con,
            &key,
            &sha,
            payload,
            "receipt identity mismatch",
            &r["details"]["accepted"],
        );
    }
    let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
    if seen(&tx, &key, &sha)? {
        return Ok(());
    }
    begin(&tx, &key, &sha, payload)?;
    let result = (|| -> Result<()> {
        if r["outcome"] != "checked" {
            reject(&tx, &key, "", "receipt not checked; no versions imported")?;
            return Ok(());
        }
        let accepted = r["details"]["accepted"]
            .as_array()
            .ok_or_else(|| err("checked receipt missing accepted"))?;
        let reserved: BTreeSet<_> = r["ids"]
            .as_array()
            .ok_or_else(|| err("reserved IDs missing"))?
            .iter()
            .filter_map(Value::as_str)
            .collect();
        let mut unique = BTreeSet::new();
        let dir = c.root.join("jobs").join(format!("{job:08}"));
        let old = trees(&dir.join("original.conllu"))?;
        let final_file = dir.join("cycle/round1/final.conllu");
        let prag_file = dir.join("cycle/round1/prag-final.tsv");
        let ts = trees(&final_file)?;
        let ps = prags(&prag_file)?;
        let provenance = serde_json::to_string(
            &json!({"receipt_sha256":sha,"receipt":r,"campaign":c.identity,"final_file_sha256":file_hash(&final_file)?,"prag_file_sha256":file_hash(&prag_file)?,"semantic_correctness":"not established; silver annotation"}),
        )?;
        let mut witnesses = Vec::new();
        for (name, source) in WITNESSES {
            let file = dir.join(name);
            if file.is_file() {
                match witness_trees(&file, &ts) {
                    Ok(w) => witnesses.push((*name, *source, w, file_hash(&file)?)),
                    Err(e) => reject(&tx, &key, "", &format!("witness {name} invalid: {e}"))?,
                }
            }
        }
        for id in accepted {
            let id = id.as_str().ok_or_else(|| err("accepted ID string"))?;
            if !reserved.contains(id) || !unique.insert(id) {
                return Err(err("unexpected/duplicate accepted ID"));
            }
            let t = ts
                .get(id)
                .ok_or_else(|| err("accepted missing final tree"))?;
            let o = old
                .get(id)
                .ok_or_else(|| err("accepted missing original"))?;
            let p = ps.get(id).ok_or_else(|| err("accepted missing prag"))?;
            let q = c
                .queue
                .get(id)
                .ok_or_else(|| err("accepted absent queue"))?;
            if q["state"] != "pending_annotation_review"
                || q["tree_sha256"] != o.raw_sha
                || o.text != t.text
                || o.forms != t.forms
            {
                return Err(err("queue/original/text/token identity mismatch"));
            }
            if let Some(reason) = g.forbidden(t) {
                reject(&tx, &key, id, reason)?;
                continue;
            }
            let eligible = g.eligible(t);
            let split = eligible.as_ref().ok().map(String::as_str);
            let reason = eligible
                .as_ref()
                .err()
                .map(String::as_str)
                .unwrap_or("verified current teaching pair");
            version(
                &tx,
                &key,
                t,
                "peta-codex-final",
                Some(p),
                split,
                reason,
                &provenance,
            )?;
            for (name, source, w, fsha) in &witnesses {
                if let Some(w) = w.get(id) {
                    if w.text == t.text && w.forms == t.forms {
                        let wp = serde_json::to_string(
                            &json!({"artifact":name,"artifact_sha256":fsha,"receipt_sha256":sha,"role":"parser/agreement witness; not teacher revision","text_header_adapter":"missing text metadata supplied only after exact ID and token-form join to teacher; raw artifact SHA retained"}),
                        )?;
                        version(
                            &tx,
                            &key,
                            w,
                            source,
                            None,
                            None,
                            "unreviewed parser/agreement witness",
                            &wp,
                        )?;
                    } else {
                        reject(
                            &tx,
                            &key,
                            id,
                            &format!("witness {name} text/token mismatch"),
                        )?
                    }
                }
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => {
            tx.execute("UPDATE batches SET status='committed',detail='checked receipt imported' WHERE key=?1",[&key])?;
            tx.commit()?;
            Ok(())
        }
        Err(e) => {
            drop(tx);
            quarantine_batch(
                con,
                &key,
                &sha,
                payload,
                &e.to_string(),
                &r["details"]["accepted"],
            )
        }
    }
}
/// Imports only new durable receipts. A quarantined batch is not retried without deliberate recovery.
pub fn ingest(config: &Value) -> Result<Value> {
    let gate = Gate::load(config)?;
    let mut con = open(config, Some(&gate))?;
    seed(config, &gate, &mut con)?;
    for v in config["campaigns"]
        .as_array()
        .ok_or_else(|| err("campaigns array required"))?
    {
        let c = campaign(v)?;
        let mut jobs: Vec<_> =
            fs::read_dir(c.root.join("ledger"))?.collect::<std::io::Result<_>>()?;
        jobs.sort_by_key(|e| e.file_name());
        for e in jobs {
            if e.path().extension().and_then(|x| x.to_str()) == Some("json") {
                ingest_job(&mut con, &gate, &c, &e.path())?;
            }
        }
    }
    status_connection(&con)
}
type Selected = (String, String, String, String, String, String);
fn selected(con: &Connection) -> Result<Vec<Selected>> {
    let mut st=con.prepare("SELECT v.id,v.tree_sha,v.prag_sha,v.tree,v.prag,v.split FROM selection s JOIN versions v ON s.version_key=v.key WHERE v.eligible=1 ORDER BY v.id")?;
    let rows = st
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
            ))
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}
#[derive(Clone)]
struct Witness {
    body: String,
    batch: String,
    receipt_sha: String,
}
fn witnesses(
    con: &Connection,
    ids: &BTreeSet<String>,
) -> Result<BTreeMap<(String, String), Witness>> {
    // Witnesses must belong to a committed batch containing the exact selected teacher pair.
    let mut st=con.prepare("SELECT w.id,w.source,w.tree,w.tree_sha,b.key,b.sha256 FROM versions w JOIN batch_versions bw ON bw.version_key=w.key JOIN batches b ON b.key=bw.batch_key JOIN batch_versions bt ON bt.batch_key=b.key JOIN versions t ON t.key=bt.version_key JOIN selection s ON s.id=t.id JOIN versions selected ON selected.key=s.version_key WHERE w.prag IS NULL AND w.source IN ('stanza','trankit','spacy-trf') AND b.status='committed' AND t.id=w.id AND t.prag IS NOT NULL AND t.tree_sha=selected.tree_sha AND t.prag_sha=selected.prag_sha ORDER BY w.source,w.id,b.key,w.tree_sha")?;
    let mut out = BTreeMap::new();
    for r in st.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, String>(4)?,
            r.get::<_, String>(5)?,
        ))
    })? {
        let (id, source, b, sha, batch, receipt_sha) = r?;
        if ids.contains(&id) {
            if hash(b.as_bytes()) != sha {
                return Err(err("witness integrity mismatch"));
            }
            out.entry((source, id)).or_insert(Witness {
                body: b,
                batch,
                receipt_sha,
            });
        }
    }
    Ok(out)
}
fn generation(rows: &[Selected], wit: &BTreeMap<(String, String), Witness>) -> String {
    let mut h = Sha256::new();
    for (id, th, ph, _, _, split) in rows {
        for x in [id, th, ph, split] {
            h.update((x.len() as u64).to_le_bytes());
            h.update(x.as_bytes());
        }
    }
    for ((s, id), b) in wit {
        for x in [s, id, &hash(b.body.as_bytes())] {
            h.update((x.len() as u64).to_le_bytes());
            h.update(x.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn cross_split_aliases(rows: &[Selected]) -> Result<BTreeSet<String>> {
    let mut dev = BTreeSet::new();
    for r in rows.iter().filter(|r| r.5 == "dev") {
        dev.insert(hash(tree(&r.3)?.text.as_bytes()));
    }
    let mut aliases = BTreeSet::new();
    for r in rows.iter().filter(|r| r.5 == "train") {
        if dev.contains(&hash(tree(&r.3)?.text.as_bytes())) {
            aliases.insert(r.0.clone());
        }
    }
    Ok(aliases)
}
fn status_connection(con: &Connection) -> Result<Value> {
    let rows = selected(con)?;
    let aliases = cross_split_aliases(&rows)?;
    let ids = rows
        .iter()
        .filter(|r| r.5 == "train" && !aliases.contains(&r.0))
        .map(|r| r.0.clone())
        .collect();
    let wit = witnesses(con, &ids)?;
    let gen = generation(&rows, &wit);
    let count = |sql: &str| -> Result<i64> { Ok(con.query_row(sql, [], |r| r.get(0))?) };
    Ok(
        json!({"schema":"peta-versions-v1","generation":gen,"generation_sha256":gen,"selected_ids":rows.len(),"train_ids":rows.iter().filter(|r|r.5=="train"&&!aliases.contains(&r.0)).count(),"selected_train_ids":rows.iter().filter(|r|r.5=="train").count(),"excluded_train_text_alias_ids":aliases,"dev_ids":rows.iter().filter(|r|r.5=="dev").count(),"versions":count("SELECT count(*) FROM versions")?,"batches":count("SELECT count(*) FROM batches")?,"quarantined_batches":count("SELECT count(*) FROM batches WHERE status='quarantined'")?,"rejections":count("SELECT count(*) FROM rejections")?,"semantic_correctness":"not established"}),
    )
}
pub fn status(config: &Value) -> Result<Value> {
    let mut con = open(config, None)?;
    let tx = con.transaction()?;
    let answer = status_connection(&tx)?;
    tx.commit()?;
    Ok(answer)
}
fn new_file(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new().create_new(true).write(true).open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    Ok(())
}
/// An export is create-new and read-transaction consistent. Its manifest is the completion marker.
pub fn export(config: &Value, out: &Path) -> Result<Value> {
    let gate = Gate::load(config)?;
    let mut con = open(config, Some(&gate))?;
    let tx = con.transaction()?;
    let rows = selected(&tx)?;
    let aliases = cross_split_aliases(&rows)?;
    let ids = rows
        .iter()
        .filter(|r| r.5 == "train" && !aliases.contains(&r.0))
        .map(|r| r.0.clone())
        .collect();
    let wit = witnesses(&tx, &ids)?;
    let gen = generation(&rows, &wit);
    fs::create_dir(out)?;
    let mut train = String::new();
    let mut dev = String::new();
    let mut agreed_input = String::new();
    let mut prov = String::new();
    let mut parsers: BTreeMap<&str, String> = [
        ("stanza", String::new()),
        ("trankit", String::new()),
        ("spacy-trf", String::new()),
    ]
    .into_iter()
    .collect();
    let mut witness_provenance = String::new();
    let mut complete = 0;
    let mut missing = Vec::new();
    for (id, th, ph, b, p, split) in &rows {
        let t = tree(b)?;
        if t.id != *id
            || hash(b.as_bytes()) != *th
            || hash(p.as_bytes()) != *ph
            || gate.eligible(&t).map_err(err)? != *split
        {
            return Err(err("selected integrity/eligibility mismatch"));
        }
        if *split == "train" {
            if aliases.contains(id) {
                continue;
            }
            train.push_str(b);
            let all = parsers.keys().all(|source| {
                wit.get(&(source.to_string(), id.clone())).is_some_and(|w| {
                    tree(&w.body)
                        .map(|w| w.text == t.text && w.forms == t.forms)
                        .unwrap_or(false)
                })
            });
            if all {
                agreed_input.push_str(b);
                complete += 1;
                for (source, body) in &mut parsers {
                    let witness = &wit[&(source.to_string(), id.clone())];
                    body.push_str(&witness.body);
                    witness_provenance.push_str(&serde_json::to_string(&json!({"sent_id":id,"source":source,"tree_sha256":hash(witness.body.as_bytes()),"batch_key":witness.batch,"receipt_sha256":witness.receipt_sha,"binding":"same committed batch as exact selected teacher tree/prag"}))?);
                    witness_provenance.push('\n');
                }
            } else {
                missing.push(id.clone());
            }
        } else if *split == "dev" {
            dev.push_str(b);
        } else {
            return Err(err("selected split invalid"));
        }
        prov.push_str(&serde_json::to_string(&json!({"sent_id":id,"tree_sha256":th,"prag_sha256":ph,"split":split,"prag":p,"role":"selected teacher full revision"}))?);
        prov.push('\n');
    }
    new_file(&out.join("train.conllu"), train.as_bytes())?;
    new_file(&out.join("dev.conllu"), dev.as_bytes())?;
    new_file(
        &out.join("agreed-input-train.conllu"),
        agreed_input.as_bytes(),
    )?;
    new_file(&out.join("provenance.jsonl"), prov.as_bytes())?;
    fs::create_dir(out.join("parsers"))?;
    fs::create_dir(out.join("parsers/out"))?;
    let mut wh = BTreeMap::new();
    for (source, body) in parsers {
        let name = format!("parsers/out/{source}.conllu");
        new_file(&out.join(&name), body.as_bytes())?;
        wh.insert(name, hash(body.as_bytes()));
    }
    File::open(out.join("parsers/out"))?.sync_all()?;
    File::open(out.join("parsers"))?.sync_all()?;
    new_file(
        &out.join("witness-provenance.jsonl"),
        witness_provenance.as_bytes(),
    )?;
    let manifest = json!({"schema":"peta-export-v1","generation":gen,"generation_sha256":gen,"train_ids":rows.iter().filter(|r|r.5=="train"&&!aliases.contains(&r.0)).count(),"selected_train_ids":rows.iter().filter(|r|r.5=="train").count(),"excluded_train_text_alias_ids":aliases,"dev_ids":rows.iter().filter(|r|r.5=="dev").count(),"train_path":out.join("train.conllu"),"dev_path":out.join("dev.conllu"),"agreed_input_train_path":out.join("agreed-input-train.conllu"),"agreed_input_train_ids":complete,"missing_training_witness_ids":missing,"parsers_dir":out.join("parsers"),"manifest_path":out.join("manifest.json"),"train_sha256":hash(train.as_bytes()),"dev_sha256":hash(dev.as_bytes()),"agreed_input_train_sha256":hash(agreed_input.as_bytes()),"provenance_sha256":hash(prov.as_bytes()),"witness_provenance_sha256":hash(witness_provenance.as_bytes()),"witness_provenance_path":out.join("witness-provenance.jsonl"),"parser_hashes":wh,"eligibility_sha256":gate.identity,"semantic_correctness":"not established; silver teacher versions"});
    new_file(
        &out.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest)?,
    )?;
    File::open(out)?.sync_all()?;
    tx.commit()?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        dir: PathBuf,
        c: Value,
        ids: Vec<String>,
    }
    fn sample(id: &str, text: &str) -> String {
        format!(
            "# text = {text}\n# sent_id = {id}\n1\t{text}\t{text}\tNOUN\tNN\t_\t0\troot\t_\t_\n\n"
        )
    }
    fn put(p: &Path, s: &str) {
        if let Some(d) = p.parent() {
            fs::create_dir_all(d).unwrap()
        }
        fs::write(p, s).unwrap();
    }
    fn pin(c: &mut Value, key: &str, p: &Path) {
        c[key] = json!(p);
        c[format!("{key}_sha256")] = json!(file_hash(p).unwrap());
    }
    impl Fixture {
        fn new() -> Self {
            let dir = std::env::temp_dir().join(format!(
                "peta-store-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::SeqCst)
            ));
            fs::create_dir(&dir).unwrap();
            let ids = (0..4)
                .map(|i| format!("school:test:lesson{i}:0"))
                .collect::<Vec<_>>();
            let phase = dir.join("phase.jsonl");
            let mut ps = String::new();
            let mut current = String::new();
            for (i, id) in ids.iter().enumerate() {
                ps.push_str(&serde_json::to_string(&json!({"id":id,"phase":"teaching_text_explain_or_fact","state":"verified_school_sents_613a642_and_head","training_allowed":false,"observed_text_sha256":hash(format!("word{i}").as_bytes())})).unwrap());
                ps.push('\n');
                current.push_str(&format!("test\tlesson{i}\t0\tword{i}\n"));
            }
            put(&phase, &ps);
            let cur = dir.join("current.tsv");
            put(&cur, &current);
            let held = dir.join("held.conllu");
            put(&held, &sample("protected:test", "secret"));
            let model = dir.join("model");
            for (n, b) in [
                ("xpos.txt", "_\nNN\nVB\n"),
                ("upos.txt", "_\nNOUN\nVERB\n"),
                ("rels.txt", "_\nroot\n"),
            ] {
                put(&model.join(n), b)
            }
            let mut c = json!({"db":dir.join("store.sqlite"),"model_dir":model,"quarantine":[],"campaigns":[]});
            pin(&mut c, "phase_index", &phase);
            pin(&mut c, "current_school_tsv", &cur);
            pin(&mut c, "heldout", &held);
            Self { dir, c, ids }
        }
        fn campaign(&mut self, count: usize, missing_prag: Option<usize>) {
            let root = self.dir.join("campaign");
            fs::create_dir(&root).unwrap();
            fs::create_dir(root.join("ledger")).unwrap();
            let source = self.dir.join("source.conllu");
            let mut src = String::new();
            let mut queue = String::new();
            for i in 0..count {
                let id = &self.ids[i];
                let b = sample(id, &format!("word{i}"));
                src.push_str(&b);
                queue.push_str(&serde_json::to_string(&json!({"id":id,"state":"pending_annotation_review","tree_sha256":hash(b.as_bytes())})).unwrap());
                queue.push('\n');
            }
            put(&source, &src);
            let qp = self.dir.join("queue.jsonl");
            put(&qp, &queue);
            let conf = self.dir.join("config.json");
            let mut descriptor = json!({"root":root,"config":conf,"heldout":self.c["heldout"]});
            pin(&mut descriptor, "source", &source);
            pin(&mut descriptor, "queue", &qp);
            put(&conf, &descriptor.to_string());
            let identity = json!({"source_sha256":descriptor["source_sha256"],"queue_sha256":descriptor["queue_sha256"],"config_sha256":file_hash(&conf).unwrap(),"heldout_sha256":self.c["heldout_sha256"]});
            put(&root.join("campaign.json"), &identity.to_string());
            for i in 0..count {
                let id = &self.ids[i];
                let b = sample(id, &format!("word{i}"));
                let j = root.join("jobs").join(format!("{i:08}"));
                put(&j.join("original.conllu"), &b);
                put(&j.join("cycle/round1/final.conllu"), &b);
                if missing_prag != Some(i) {
                    put(
                        &j.join("cycle/round1/prag-final.tsv"),
                        &format!("{HEADER}\n{id}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture\n"),
                    );
                }
                for source in ["stanza", "trankit", "spacy-trf"] {
                    put(
                        &j.join(format!("cycle/round1/pre/out/{source}.conllu")),
                        &b.lines()
                            .filter(|l| !l.starts_with("# text = "))
                            .map(|l| format!("{l}\n"))
                            .collect::<String>(),
                    );
                }
                let r = json!({"job":i,"ids":[id],"outcome":"checked","source_sha256":identity["source_sha256"],"queue_sha256":identity["queue_sha256"],"details":{"accepted":[id]}});
                put(
                    &root.join("ledger").join(format!("{i:08}.json")),
                    &r.to_string(),
                );
            }
            self.c["campaigns"] = json!([descriptor]);
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn two_batches_idempotent_export_and_real_witnesses() {
        let mut f = Fixture::new();
        f.campaign(2, None);
        let a = ingest(&f.c).unwrap();
        assert_eq!(a["selected_ids"], 2);
        assert_eq!(a["versions"], 8);
        let b = ingest(&f.c).unwrap();
        assert_eq!(a, b);
        let out = f.dir.join("export");
        let e = export(&f.c, &out).unwrap();
        assert_eq!(e["generation"], a["generation"]);
        assert_eq!(e["agreed_input_train_ids"], e["train_ids"]);
        assert!(out.join("parsers/out/stanza.conllu").is_file());
        assert!(export(&f.c, &out).is_err());
        let frozen = fs::read(out.join("manifest.json")).unwrap();
        let g = Gate::load(&f.c).unwrap();
        let mut db = open(&f.c, Some(&g)).unwrap();
        let tx = db
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .unwrap();
        begin(&tx, "third", "sha", "{}").unwrap();
        let t = tree(&sample(&f.ids[2], "word2")).unwrap();
        let split = g.eligible(&t).unwrap();
        version(
            &tx,
            "third",
            &t,
            "peta-codex-final",
            Some(&format!(
                "{}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture",
                t.id
            )),
            Some(&split),
            "fixture",
            "{}",
        )
        .unwrap();
        tx.execute(
            "UPDATE batches SET status='committed' WHERE key='third'",
            [],
        )
        .unwrap();
        tx.commit().unwrap();
        let after = status(&f.c).unwrap();
        assert_ne!(after["generation"], a["generation"]);
        assert_eq!(fs::read(out.join("manifest.json")).unwrap(), frozen);
    }
    #[test]
    fn missing_pair_quarantines_whole_batch() {
        let mut f = Fixture::new();
        f.campaign(2, Some(1));
        let a = ingest(&f.c).unwrap();
        assert_eq!(a["selected_ids"], 1);
        assert_eq!(a["quarantined_batches"], 1);
        assert_eq!(a, ingest(&f.c).unwrap());
    }
    #[test]
    fn phase_question_alias_and_target_labels_fail_closed() {
        let f = Fixture::new();
        let g = Gate::load(&f.c).unwrap();
        let good = tree(&sample(&f.ids[0], "word0")).unwrap();
        assert!(g.eligible(&good).is_ok());
        let question = tree(&sample("question:test:exam", "word0")).unwrap();
        assert!(g.eligible(&question).is_err());
        let alias = tree(&sample(&f.ids[0], "secret")).unwrap();
        assert_eq!(g.forbidden(&alias), Some("heldout ID/text"));
        assert!(g.eligible(&alias).is_err());
        let changed = tree(&sample(&f.ids[0], "changed")).unwrap();
        assert!(g.eligible(&changed).is_err());
        let bad = tree(&sample(&f.ids[0], "word0").replace("\tNN\t", "\tBAD\t")).unwrap();
        assert!(g.eligible(&bad).is_err());
        assert!(tree(
            &(sample(&f.ids[0], "word0").trim_end().to_owned() + &sample(&f.ids[1], "word1"))
        )
        .is_err());
        assert!(tree("1\tX\tX\tNOUN\tNN\t_\t0\troot\t_\t_\n").is_err());
    }
    #[test]
    fn conflicts_keep_prior_whole_version() {
        let f = Fixture::new();
        let g = Gate::load(&f.c).unwrap();
        let mut db = open(&f.c, Some(&g)).unwrap();
        let t = tree(&sample(&f.ids[0], "word0")).unwrap();
        let other = tree(&t.body.replace("\tNOUN\tNN\t", "\tVERB\tVB\t")).unwrap();
        let p = format!("{}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture", t.id);
        let split = g.eligible(&t).unwrap();
        for (b, t) in [("one", &t), ("two", &other)] {
            let tx = db
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .unwrap();
            begin(&tx, b, b, "{}").unwrap();
            version(
                &tx,
                b,
                t,
                "peta-codex-final",
                Some(&p),
                Some(&split),
                "fixture",
                "{}",
            )
            .unwrap();
            tx.commit().unwrap();
        }
        let s = selected(&db).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].3, t.body);
        assert_eq!(status_connection(&db).unwrap()["rejections"], 1);
    }
    #[test]
    fn concurrent_writers_serialize_without_lost_batches() {
        let f = Fixture::new();
        let g = Gate::load(&f.c).unwrap();
        drop(open(&f.c, Some(&g)).unwrap());
        let mut handles = Vec::new();
        for i in 0..4 {
            let c = f.c.clone();
            let id = f.ids[i].clone();
            handles.push(std::thread::spawn(move || {
                let g = Gate::load(&c).unwrap();
                let mut db = open(&c, Some(&g)).unwrap();
                let t = tree(&sample(&id, &format!("word{i}"))).unwrap();
                let split = g.eligible(&t).unwrap();
                let tx = db
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .unwrap();
                let b = format!("thread{i}");
                begin(&tx, &b, &b, "{}").unwrap();
                version(
                    &tx,
                    &b,
                    &t,
                    "peta-codex-final",
                    Some(&format!(
                        "{}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture",
                        t.id
                    )),
                    Some(&split),
                    "fixture",
                    "{}",
                )
                .unwrap();
                tx.commit().unwrap();
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let s = status(&f.c).unwrap();
        assert_eq!(s["selected_ids"], 4);
        assert_eq!(s["batches"], 4);
    }
    #[test]
    fn changed_receipt_and_changed_policy_refuse() {
        let mut f = Fixture::new();
        f.campaign(1, None);
        ingest(&f.c).unwrap();
        let root = Path::new(f.c["campaigns"][0]["root"].as_str().unwrap());
        let receipt = root.join("ledger/00000000.json");
        let mut bytes = fs::read(&receipt).unwrap();
        bytes.push(b' ');
        fs::write(receipt, bytes).unwrap();
        assert!(ingest(&f.c).is_err());
        f.c["quarantine"] = json!([f.ids[0]]);
        assert!(ingest(&f.c).is_err());
    }
    #[test]
    fn unrelated_teacher_batch_cannot_supply_witnesses() {
        let mut f = Fixture::new();
        f.campaign(1, None);
        ingest(&f.c).unwrap();
        let g = Gate::load(&f.c).unwrap();
        let mut con = open(&f.c, Some(&g)).unwrap();
        let t = tree(&sample(&f.ids[0], "word0")).unwrap();
        let other = tree(&t.body.replace("\tNOUN\tNN\t", "\tVERB\tVB\t")).unwrap();
        let before = status(&f.c).unwrap()["generation"].clone();
        let tx = con
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .unwrap();
        begin(&tx, "unrelated", "sha", "{}").unwrap();
        version(
            &tx,
            "unrelated",
            &other,
            "peta-codex-final",
            Some(&format!(
                "{}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture",
                t.id
            )),
            Some(&g.eligible(&t).unwrap()),
            "fixture",
            "{}",
        )
        .unwrap();
        version(
            &tx,
            "unrelated",
            &other,
            "stanza",
            None,
            None,
            "fixture",
            "{}",
        )
        .unwrap();
        version(&tx, "unrelated", &t, "gparse", None, None, "fixture", "{}").unwrap();
        tx.execute(
            "UPDATE batches SET status='committed' WHERE key='unrelated'",
            [],
        )
        .unwrap();
        tx.commit().unwrap();
        assert_eq!(before, status(&f.c).unwrap()["generation"]);
        let selected = selected(&con).unwrap();
        let ids = selected.iter().map(|r| r.0.clone()).collect();
        let w = witnesses(&con, &ids).unwrap();
        assert_eq!(w[&("stanza".into(), t.id)].body, t.body);
    }
    #[test]
    fn malformed_receipt_is_terminal_but_other_jobs_progress() {
        let mut f = Fixture::new();
        f.campaign(2, None);
        let root = Path::new(f.c["campaigns"][0]["root"].as_str().unwrap());
        put(&root.join("ledger/00000000.json"), "broken JSON");
        let a = ingest(&f.c).unwrap();
        assert_eq!(a["selected_ids"], 1);
        assert_eq!(a["quarantined_batches"], 1);
        assert_eq!(a, ingest(&f.c).unwrap());
    }
    #[test]
    fn ineligible_existing_version_cannot_be_selected() {
        let f = Fixture::new();
        let mut con = open(&f.c, None).unwrap();
        let tx = con
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .unwrap();
        begin(&tx, "fixture", "sha", "{}").unwrap();
        let t = tree(&sample(&f.ids[0], "word0")).unwrap();
        let p = format!("{}\tdecl\tassert\t0\t0\tnone\tnarrator\tfixture", t.id);
        version(
            &tx,
            "fixture",
            &t,
            "peta-codex-final",
            Some(&p),
            None,
            "fixture",
            "{}",
        )
        .unwrap();
        assert!(version(
            &tx,
            "fixture",
            &t,
            "peta-codex-final",
            Some(&p),
            Some("train"),
            "fixture",
            "{}"
        )
        .is_err());
        assert!(selected(&tx).unwrap().is_empty());
    }
    #[test]
    fn equal_text_across_lessons_excludes_train_alias() {
        let a = sample("school:x:train:0", "same");
        let b = sample("school:x:dev:0", "same");
        let c = sample("school:x:train2:0", "different");
        let rows = vec![
            (
                "school:x:train:0".into(),
                "a".into(),
                "p".into(),
                a,
                "prag".into(),
                "train".into(),
            ),
            (
                "school:x:dev:0".into(),
                "b".into(),
                "p".into(),
                b,
                "prag".into(),
                "dev".into(),
            ),
            (
                "school:x:train2:0".into(),
                "c".into(),
                "p".into(),
                c,
                "prag".into(),
                "train".into(),
            ),
        ];
        assert_eq!(
            cross_split_aliases(&rows).unwrap(),
            BTreeSet::from(["school:x:train:0".into()])
        );
    }
}
