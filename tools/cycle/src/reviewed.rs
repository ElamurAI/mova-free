//! Content-bound reviewed selection. Callers must be quiescent during promotion/rollback.
use crate::{db, R};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
#[allow(dead_code)]
#[path = "../../../prag/src/schema.rs"]
mod prag_schema;

pub const HEADER: &str = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher";
const FILES: &[&str] = &[
    "current.conllu",
    "history.conllu",
    "log.tsv",
    "prag.tsv",
    "reviewed.conllu",
    "reviewed-prag.tsv",
    "reviewed-decisions.jsonl",
];
pub fn hash(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
fn read(p: &Path) -> R<String> {
    fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn write(p: &Path, s: &str) -> R<()> {
    let mut f = fs::File::create(p).map_err(|e| e.to_string())?;
    f.write_all(s.as_bytes())
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
fn sync_dir(p: &Path) -> R<()> {
    fs::File::open(p)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())
}
pub fn ready(db: &Path) -> R<()> {
    if db.join(".reviewed.lock").exists() {
        return Err(
            "reviewed promotion lock present; stop callers and inspect transaction/rollback".into(),
        );
    }
    Ok(())
}
pub fn trees(s: &str) -> R<BTreeMap<String, String>> {
    // Legacy blocks() intentionally ignores headerless fragments. Reviewed inputs must not.
    let mut active = false;
    let mut preamble = false;
    for line in s.lines() {
        if line.is_empty() {
            if preamble {
                return Err("orphan reviewed comment fragment".into());
            }
            active = false;
        } else if line.starts_with("# sent_id = ") {
            active = true;
            preamble = false;
        } else if line.starts_with("# sent_id") {
            return Err("malformed reviewed sent_id header".into());
        } else if !active {
            if !line.starts_with('#') {
                return Err("orphan reviewed token fragment".into());
            }
            preamble = true;
        }
    }
    if preamble {
        return Err("orphan reviewed comment fragment".into());
    }
    let mut out = BTreeMap::new();
    for (id, b) in db::blocks(s) {
        if id.is_empty()
            || b.lines().filter(|l| l.starts_with("# sent_id = ")).count() != 1
            || out.insert(id.clone(), b).is_some()
        {
            return Err(format!("duplicate or malformed sentence {id}"));
        }
    }
    if out.is_empty() && !s.trim().is_empty() {
        return Err("no sentence IDs".into());
    }
    Ok(out)
}
fn text(b: &str) -> R<&str> {
    let v: Vec<_> = b
        .lines()
        .filter_map(|l| l.strip_prefix("# text = "))
        .collect();
    if v.len() != 1 {
        return Err("expected exactly one # text header".into());
    }
    Ok(v[0])
}
fn structure(b: &str) -> R<Vec<String>> {
    let mut heads = Vec::new();
    let mut forms = Vec::new();
    for l in b.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let c: Vec<_> = l.split('\t').collect();
        if c.len() != 10 || c[0].parse::<usize>().ok() != Some(heads.len() + 1) || c[1].is_empty() {
            return Err("reviewed trees require contiguous integer tokens and ten columns".into());
        }
        heads.push(c[6].parse::<usize>().map_err(|_| "invalid reviewed head")?);
        forms.push(c[1].to_string());
    }
    if !db::tree_ok(&heads) {
        return Err("invalid reviewed tree structure".into());
    }
    Ok(forms)
}
pub fn prag(s: &str) -> R<BTreeMap<String, String>> {
    let mut ls = s.lines();
    if ls.next() != Some(HEADER) {
        return Err("wrong pragmatic header".into());
    }
    let mut out = BTreeMap::new();
    for l in ls {
        let c: Vec<_> = l.split('\t').collect();
        if c.len() != 8 || c[0].is_empty() || out.insert(c[0].to_string(), l.to_string()).is_some()
        {
            return Err("duplicate/malformed pragmatic row".into());
        }
    }
    Ok(out)
}
fn str_field<'a>(v: &'a Value, k: &str) -> R<&'a str> {
    v[k].as_str().ok_or_else(|| format!("missing string {k}"))
}
fn equals(v: &Value, k: &str, expected: &str) -> R<()> {
    if str_field(v, k)? != expected {
        return Err(format!("reviewed decision {k} mismatch"));
    }
    Ok(())
}
fn decisions(
    s: &str,
    ts: &BTreeMap<String, String>,
    ps: &BTreeMap<String, String>,
) -> R<BTreeSet<String>> {
    let mut ids = BTreeSet::new();
    for l in s.lines() {
        let v: Value = serde_json::from_str(l).map_err(|e| e.to_string())?;
        let id = str_field(&v, "sent_id")?;
        if !id.starts_with("school:") || !ids.insert(id.to_string()) {
            return Err(format!("ineligible or duplicate reviewed ID {id}"));
        }
        equals(&v, "decision", "allow")?;
        equals(&v, "authorization", "user:2026-11-01")?;
        if !matches!(str_field(&v, "training_split")?, "train" | "dev") {
            return Err("invalid training split".into());
        }
        if !v["split"].is_null() && v["split"] != v["training_split"] {
            return Err("conflicting training split aliases".into());
        }
        equals(&v, "phase", "teaching_text_explain_or_fact")?;
        equals(
            &v,
            "provenance_state",
            "verified_school_sents_613a642_and_head",
        )?;
        if v["heldout"] != false || v["joined_source"] != false {
            return Err("heldout or quarantined reviewed source".into());
        }
        let raw = str_field(&v, "source_phase_record")?;
        equals(&v, "source_phase_sha256", &hash(raw))?;
        let phase: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
        equals(&phase, "id", id)?;
        equals(&phase, "phase", "teaching_text_explain_or_fact")?;
        equals(&phase, "state", "verified_school_sents_613a642_and_head")?;
        if phase["review_only"] == true
            || phase["heldout"] == true
            || phase["training_allowed"] != false
        {
            return Err("review-only or unproven phase evidence".into());
        }
        let t = ts.get(id).ok_or("decision without tree")?;
        let p = ps.get(id).ok_or("decision without pragmatics")?;
        structure(t)?;
        let columns: Vec<_> = p.split('\t').collect();
        prag_schema::parse_labels(&columns[1..])?;
        equals(&v, "source_text_sha256", &hash(text(t)?))?;
        equals(&phase, "observed_text_sha256", &hash(text(t)?))?;
        equals(&v, "tree_sha256", &hash(t))?;
        equals(&v, "prag_sha256", &hash(p))?;
        let old = str_field(&v, "previous_tree_sha256")?;
        if old.len() != 64
            || !old
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err("invalid previous tree hash".into());
        }
    }
    if ids.is_empty() || ids.len() != ts.len() || ids.len() != ps.len() {
        return Err("reviewed tree/prag/decision ID sets differ or empty".into());
    }
    Ok(ids)
}
/// Validate the immutable selection and current content before any DB write or training.
pub fn policy(db: &Path) -> R<Option<BTreeSet<String>>> {
    let present = [
        "reviewed.conllu",
        "reviewed-prag.tsv",
        "reviewed-decisions.jsonl",
    ]
    .map(|n| db.join(n).exists());
    if present.iter().all(|x| !x) {
        return Ok(None);
    }
    if present.iter().any(|x| !x) {
        return Err("incomplete reviewed policy".into());
    }
    let ts = trees(&read(&db.join("reviewed.conllu"))?)?;
    let ps = prag(&read(&db.join("reviewed-prag.tsv"))?)?;
    let ds = read(&db.join("reviewed-decisions.jsonl"))?;
    let ids = decisions(&ds, &ts, &ps)?;
    let transaction = hash(&ds);
    let journal = read(
        &db.join(".reviewed-transactions")
            .join(&transaction)
            .join("journal"),
    )?;
    if journal != "COMMITTED\n"
        && !(journal == "APPLYING\n" && read(&db.join(".reviewed.lock"))? == transaction)
    {
        return Err("reviewed selection lacks its promotion transaction".into());
    }
    let cur = trees(&read(&db.join("current.conllu"))?)?;
    let cp = prag(&read(&db.join("prag.tsv"))?)?;
    for id in &ids {
        if cur.get(id) != ts.get(id) || cp.get(id) != ps.get(id) {
            return Err(format!("stale reviewed current/tree/prag selection {id}"));
        }
    }
    Ok(Some(ids))
}
/// After legacy voting, before writing current, copy the entire selected tree.
pub fn overlay(db: &Path, order: &mut [(String, String)]) -> R<()> {
    if !db.join("reviewed.conllu").exists() {
        return Ok(());
    }
    let ts = trees(&read(&db.join("reviewed.conllu"))?)?;
    for (id, b) in order {
        if let Some(t) = ts.get(id) {
            if text(b)? != text(t)? {
                return Err(format!("vote changed reviewed text {id}"));
            }
            *b = t.clone();
        }
    }
    Ok(())
}

fn snapshot(from: &Path, to: &Path) -> R<()> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for e in fs::read_dir(from).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        if e.file_name() == ".reviewed-transactions" || e.file_name() == ".reviewed.lock" {
            continue;
        }
        let ty = e.file_type().map_err(|e| e.to_string())?;
        if ty.is_symlink() {
            return Err("snapshot refuses symbolic links".into());
        }
        let dst = to.join(e.file_name());
        if ty.is_dir() {
            snapshot(&e.path(), &dst)?;
        } else if ty.is_file() {
            fs::copy(e.path(), &dst).map_err(|e| e.to_string())?;
            fs::File::open(&dst)
                .and_then(|f| f.sync_all())
                .map_err(|e| e.to_string())?;
        } else {
            return Err("snapshot refuses special files".into());
        }
    }
    sync_dir(to)
}
struct Lock {
    p: PathBuf,
    retain: bool,
}
impl Drop for Lock {
    fn drop(&mut self) {
        if !self.retain {
            let _ = fs::remove_file(&self.p);
        }
    }
}
fn lock(db: &Path) -> R<Lock> {
    let p = db.join(".reviewed.lock");
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&p)
        .map_err(|e| format!("exclusive promotion lock: {e}"))?;
    Ok(Lock { p, retain: false })
}
/// One reviewed batch per DB. Identical repeats validate and never append history/log.
pub fn promote(db: &Path, tree_file: &Path, prag_file: &Path, decision_file: &Path) -> R<PathBuf> {
    ready(db)?;
    let mut guard = lock(db)?;
    let tree_text = read(tree_file)?;
    let prag_text = read(prag_file)?;
    let ds = read(decision_file)?;
    let ts = trees(&tree_text)?;
    let ps = prag(&prag_text)?;
    let ids = decisions(&ds, &ts, &ps)?;
    if policy(db)?.is_some() {
        if read(&db.join("reviewed-decisions.jsonl"))? == ds
            && trees(&read(&db.join("reviewed.conllu"))?)? == ts
            && prag(&read(&db.join("reviewed-prag.tsv"))?)? == ps
        {
            return Ok(db.join(".reviewed-transactions").join(hash(&ds)));
        }
        return Err("DB already has a different reviewed batch; use a fresh snapshot DB".into());
    }
    let current = read(&db.join("current.conllu"))?;
    let old = trees(&current)?;
    let old_prag = read(&db.join("prag.tsv"))?;
    let mut merged_prag = prag(&old_prag)?;
    for l in ds.lines() {
        let v: Value = serde_json::from_str(l).map_err(|e| e.to_string())?;
        let id = str_field(&v, "sent_id")?;
        let b = old.get(id).ok_or("reviewed ID absent from current")?;
        equals(&v, "previous_tree_sha256", &hash(b))?;
        if text(b)? != text(&ts[id])? || structure(b)? != structure(&ts[id])? {
            return Err(format!("stale text or tokenization {id}"));
        }
        merged_prag.insert(id.to_string(), ps[id].clone());
    }
    let mut next = String::new();
    for (id, b) in db::blocks(&current) {
        next.push_str(ts.get(&id).unwrap_or(&b));
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let mut hist = read(&db.join("history.conllu"))?;
    let mut log = read(&db.join("log.tsv"))?;
    for id in &ids {
        hist.push_str(&format!("# version = terra-reviewed {now}\n{}", ts[id]));
        log.push_str(&format!("{now}\t{id}\tterra-reviewed\treplaced\n"));
    }
    let rendered_prag = format!(
        "{HEADER}\n{}",
        merged_prag
            .values()
            .map(|p| format!("{p}\n"))
            .collect::<String>()
    );
    let rendered_trees = ts.values().cloned().collect::<String>();
    let rendered_rows = format!(
        "{HEADER}\n{}",
        ps.values().map(|p| format!("{p}\n")).collect::<String>()
    );
    let tx = db.join(".reviewed-transactions").join(hash(&ds));
    fs::create_dir_all(tx.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::create_dir(&tx)
        .map_err(|e| format!("transaction already exists; inspect/rollback first: {e}"))?;
    snapshot(db, &tx.join("before"))?;
    let hashes = inventory(&tx.join("before"))?;
    write(
        &tx.join("before-sha256.json"),
        &serde_json::to_string_pretty(&hashes).map_err(|e| e.to_string())?,
    )?;
    let stage = tx.join("stage");
    fs::create_dir(&stage).map_err(|e| e.to_string())?;
    for (name, value) in FILES.iter().zip([
        next,
        hist,
        log,
        rendered_prag,
        rendered_trees,
        rendered_rows,
        ds,
    ]) {
        write(&stage.join(name), &value)?;
    }
    sync_dir(&stage)?;
    journal(&tx, "PREPARED\n")?;
    sync_dir(tx.parent().unwrap())?;
    write(&guard.p, &tx.file_name().unwrap().to_string_lossy())?;
    sync_dir(db)?;
    // After this boundary any failure retains the lock and before snapshot for explicit rollback.
    guard.retain = true;
    journal(&tx, "APPLYING\n")?;
    for name in FILES {
        fs::rename(stage.join(name), db.join(name)).map_err(|e| e.to_string())?;
    }
    sync_dir(db)?;
    policy(db)?;
    journal(&tx, "COMMITTED\n")?;
    guard.retain = false;
    Ok(tx)
}
/// Restore all snapshot files and remove newly introduced managed files. Never update/history-log.
pub fn rollback(db: &Path, id: &str) -> R<()> {
    if id.len() != 64 || !id.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("rollback requires transaction hash".into());
    }
    let tx = db.join(".reviewed-transactions").join(id);
    let state = read(&tx.join("journal"))?;
    if !["PREPARED\n", "APPLYING\n", "COMMITTED\n", "ROLLING_BACK\n"].contains(&state.as_str()) {
        return Err("transaction not rollback-ready".into());
    }
    let expected: BTreeMap<String, String> =
        serde_json::from_str(&read(&tx.join("before-sha256.json"))?).map_err(|e| e.to_string())?;
    if expected != inventory(&tx.join("before"))? {
        return Err("rollback snapshot hash mismatch".into());
    }
    let mut guard = if db.join(".reviewed.lock").exists() {
        if read(&db.join(".reviewed.lock"))? != id {
            return Err("foreign promotion lock".into());
        }
        Lock {
            p: db.join(".reviewed.lock"),
            retain: true,
        }
    } else {
        lock(db)?
    };
    write(&guard.p, id)?;
    guard.retain = true;
    journal(&tx, "ROLLING_BACK\n")?;
    let before = tx.join("before");
    for entry in fs::read_dir(db).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_name() == ".reviewed-transactions" || entry.file_name() == ".reviewed.lock" {
            continue;
        }
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            fs::remove_dir_all(entry.path()).map_err(|e| e.to_string())?;
        } else {
            fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
        }
    }
    snapshot(&before, db)?;
    sync_dir(db)?;
    journal(&tx, "ROLLED_BACK\n")?;
    guard.retain = false;
    Ok(())
}

/// A policy binds every imported correction; only its explicit train partition can train.
pub fn training_allow(db: &Path) -> R<Option<BTreeSet<String>>> {
    ready(db)?;
    if policy(db)?.is_none() {
        return Ok(None);
    }
    let mut ids = BTreeSet::new();
    for l in read(&db.join("reviewed-decisions.jsonl"))?.lines() {
        let v: Value = serde_json::from_str(l).map_err(|e| e.to_string())?;
        if str_field(&v, "training_split")? == "train" {
            ids.insert(str_field(&v, "sent_id")?.to_string());
        }
    }
    Ok(Some(ids))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree(id: &str, new: bool) -> String {
        let (a, b, tag) = if new { (2, 0, "AUX") } else { (0, 1, "VERB") };
        format!("# sent_id = {id}\n# text = A B\n1\tA\ta\t{tag}\tX\t_\t{a}\t{}\t_\t_\n2\tB\tb\tVERB\tX\t_\t{b}\t{}\t_\t_\n\n",if a==0 {"root"} else {"aux"},if b==0 {"root"} else {"dep"})
    }
    fn row(id: &str) -> String {
        format!("{id}\tdecl\tassert\t0\t0\tnone\tnone\t-")
    }
    fn decision(id: &str, split: &str) -> Value {
        let phase=serde_json::json!({"id":id,"phase":"teaching_text_explain_or_fact","state":"verified_school_sents_613a642_and_head","observed_text_sha256":hash("A B"),"training_allowed":false}).to_string();
        serde_json::json!({"sent_id":id,"decision":"allow","authorization":"user:2026-11-01","training_split":split,"phase":"teaching_text_explain_or_fact","provenance_state":"verified_school_sents_613a642_and_head","heldout":false,"joined_source":false,"source_text_sha256":hash("A B"),"previous_tree_sha256":hash(&tree(id,false)),"tree_sha256":hash(&tree(id,true)),"prag_sha256":hash(&row(id)),"source_phase_sha256":hash(&phase),"source_phase_record":phase})
    }
    fn managed(db: &Path) -> Vec<Option<Vec<u8>>> {
        FILES.iter().map(|n| fs::read(db.join(n)).ok()).collect()
    }
    #[test]
    fn malformed_orphan_fragments_fail() {
        let valid = tree("school:a", false);
        assert!(trees(&format!(
            "{valid}1\tOrphan\t_\tX\tX\t_\t0\troot\t_\t_\n"
        ))
        .is_err());
        assert!(trees(&format!("{valid}# source = orphan\n")).is_err());
        assert!(trees(&format!("{valid}# sent_id=malformed\n")).is_err());
        assert!(trees(&format!("# source = prefix\n{valid}")).is_ok());
    }
    #[test]
    fn joined_preambles_belong_to_following_sentence() {
        let a = tree("school:a", false);
        let b = tree("school:b", false);
        let joined = format!(
            "{}\n# newdoc = next\n# version = round2 7\n# source = next-document\n{b}",
            a.trim_end()
        );
        let parts = db::blocks(&joined);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].1, a);
        assert!(parts[1].1.starts_with("# newdoc = next\n# version = round2 7\n# source = next-document\n# sent_id = school:b\n"));
        assert_eq!(crate::ids_of(&parts[1].1), vec!["school:b"]);
    }
    #[test]
    fn reviewed_selection_transaction_and_training_gates() {
        let tmp = std::env::temp_dir().join(format!("cycle-reviewed-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let d = tmp.join("db");
        fs::create_dir(&d).unwrap();
        let id = "school:lesson:0";
        let dev = "school:lesson:1";
        let other = "school:other:0";
        let question = "question:exam:e1";
        // No blank separator between any pair: all four IDs must survive normalization.
        let original = [
            tree(id, false),
            tree(dev, false),
            tree(other, false),
            tree(question, false),
        ]
        .into_iter()
        .map(|b| b.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n");
        fs::write(d.join("current.conllu"), &original).unwrap();
        let history = format!(
            "# version = round1-opus 1\n{}# version = round1-opus 1\n{}",
            tree(id, false),
            tree(dev, false)
        );
        fs::write(d.join("history.conllu"), &history).unwrap();
        fs::write(d.join("log.tsv"), "old\n").unwrap();
        fs::write(
            d.join("prag.tsv"),
            format!(
                "{HEADER}\n{}\n{}\n{}\n{}\n",
                row(id),
                row(dev),
                row(other),
                row(question)
            ),
        )
        .unwrap();
        fs::create_dir(d.join("extra")).unwrap();
        fs::write(d.join("extra/file"), "preserved").unwrap();
        let tf = tmp.join("trees");
        let pf = tmp.join("prag");
        let df = tmp.join("decisions");
        fs::write(&tf, format!("{}{}", tree(id, true), tree(dev, true))).unwrap();
        fs::write(&pf, format!("{HEADER}\n{}\n{}\n", row(id), row(dev))).unwrap();
        let good = format!("{}\n{}\n", decision(id, "train"), decision(dev, "dev"));
        fs::write(&df, &good).unwrap();
        let before = managed(&d);
        // Every control fails before modifying a DB data file.
        for (key, value) in [
            ("tree_sha256", Value::String("0".repeat(64))),
            ("prag_sha256", Value::String("0".repeat(64))),
            ("previous_tree_sha256", Value::String("0".repeat(64))),
            ("decision", Value::String("deny".into())),
            ("heldout", Value::Bool(true)),
            ("phase", Value::String("exam".into())),
            ("authorization", Value::String("model-approved".into())),
        ] {
            let mut v = decision(id, "train");
            v[key] = value;
            fs::write(&df, format!("{v}\n{}\n", decision(dev, "dev"))).unwrap();
            assert!(promote(&d, &tf, &pf, &df).is_err(), "{key}");
            assert_eq!(before, managed(&d));
            assert!(!d.join(".reviewed.lock").exists());
        }
        fs::write(&df, &good).unwrap();
        fs::write(&pf, format!("wrong\n{}\n{}\n", row(id), row(dev))).unwrap();
        assert!(promote(&d, &tf, &pf, &df).is_err());
        assert_eq!(before, managed(&d));
        fs::write(&pf, format!("{HEADER}\n{}\n{}\n", row(id), row(dev))).unwrap();
        fs::write(
            &tf,
            format!("{}{}{}", tree(id, true), tree(id, true), tree(dev, true)),
        )
        .unwrap();
        assert!(promote(&d, &tf, &pf, &df).is_err());
        assert_eq!(before, managed(&d));
        fs::write(&tf, format!("{}{}", tree(id, true), tree(dev, true))).unwrap();
        // Forged allow cannot turn a question ID or source exam phase into teaching.
        for bad in [question, "test:heldout", "book:unproven"] {
            let t = trees(&tree(bad, true)).unwrap();
            let p = prag(&format!("{HEADER}\n{}\n", row(bad))).unwrap();
            assert!(decisions(&decision(bad, "train").to_string(), &t, &p).is_err());
        }
        let mut v = decision(id, "train");
        let mut source: Value =
            serde_json::from_str(v["source_phase_record"].as_str().unwrap()).unwrap();
        source["phase"] = "exam".into();
        let raw = source.to_string();
        v["source_phase_record"] = raw.clone().into();
        v["source_phase_sha256"] = hash(&raw).into();
        assert!(decisions(
            &v.to_string(),
            &trees(&tree(id, true)).unwrap(),
            &prag(&format!("{HEADER}\n{}\n", row(id))).unwrap()
        )
        .is_err());
        let tx = promote(&d, &tf, &pf, &df).unwrap();
        assert_eq!(
            trees(&read(&d.join("current.conllu")).unwrap())
                .unwrap()
                .len(),
            4
        );
        let after = managed(&d);
        assert_eq!(promote(&d, &tf, &pf, &df).unwrap(), tx);
        assert_eq!(after, managed(&d));
        // Actual old source vote wins without the content-bound overlay.
        let neg = tmp.join("no-overlay");
        fs::create_dir(&neg).unwrap();
        fs::write(neg.join("current.conllu"), tree(id, true)).unwrap();
        fs::write(
            neg.join("history.conllu"),
            read(&d.join("history.conllu")).unwrap(),
        )
        .unwrap();
        db::agree(&neg).unwrap();
        assert_eq!(
            trees(&read(&neg.join("current.conllu")).unwrap()).unwrap()[id],
            tree(id, false)
        );
        db::agree(&d).unwrap();
        db::agree(&d).unwrap();
        assert_eq!(
            trees(&read(&d.join("current.conllu")).unwrap()).unwrap()[id],
            tree(id, true)
        );
        assert_eq!(
            prag(&read(&d.join("prag.tsv")).unwrap()).unwrap()[id],
            row(id)
        );
        fs::write(&tf, tree(id, false)).unwrap();
        assert!(db::update(&d, &tf, "round2-opus").is_err());
        fs::write(
            &pf,
            format!(
                "{HEADER}\n{}\n",
                row(id).replace("\tassert\t", "\tnarrate\t")
            ),
        )
        .unwrap();
        assert!(db::update_prag(&d, &pf).is_err());
        std::env::set_var("CYCLE_DB", &d);
        std::env::remove_var("CYCLE_TRAIN_DBS");
        let train = db::training_file(&d, &db::Heldout::default()).unwrap();
        assert_eq!(crate::ids_of(&read(&train).unwrap()), vec![id]);
        assert!(
            read(&db::training_file(&d, &db::Heldout::of_ids([id.to_string()].into())).unwrap())
                .unwrap()
                .is_empty()
        );
        // Interrupted cross-file write leaves callers blocked; rollback removes every later DB artifact.
        fs::write(
            d.join(".reviewed.lock"),
            tx.file_name().unwrap().to_string_lossy().as_bytes(),
        )
        .unwrap();
        fs::write(tx.join("journal"), "APPLYING\n").unwrap();
        fs::write(d.join("current.conllu"), "partial").unwrap();
        fs::write(d.join("created-later"), "remove").unwrap();
        assert!(db::agree(&d).is_err());
        assert!(db::training_file(&d, &db::Heldout::default()).is_err());
        rollback(&d, tx.file_name().unwrap().to_str().unwrap()).unwrap();
        assert_eq!(before, managed(&d));
        assert_eq!(read(&d.join("extra/file")).unwrap(), "preserved");
        assert!(!d.join("created-later").exists());
        assert!(!train.exists());
        assert!(rollback(&d, tx.file_name().unwrap().to_str().unwrap()).is_err());
        fs::remove_dir_all(tmp).unwrap();
    }
}

fn inventory(dir: &Path) -> R<BTreeMap<String, String>> {
    fn visit(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) -> R<()> {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let p = entry.path();
            let ty = entry.file_type().map_err(|e| e.to_string())?;
            if ty.is_dir() {
                visit(root, &p, out)?;
            } else if ty.is_file() {
                let mut f = fs::File::open(&p).map_err(|e| e.to_string())?;
                let mut hasher = Sha256::new();
                let mut buffer = [0u8; 65536];
                loop {
                    let n = f.read(&mut buffer).map_err(|e| e.to_string())?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                out.insert(
                    p.strip_prefix(root)
                        .map_err(|e| e.to_string())?
                        .to_str()
                        .ok_or("non-UTF8 snapshot path")?
                        .to_string(),
                    format!("{:x}", hasher.finalize()),
                );
            } else {
                return Err("snapshot contains unsupported file type".into());
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    visit(dir, dir, &mut out)?;
    Ok(out)
}

fn journal(tx: &Path, state: &str) -> R<()> {
    write(&tx.join("journal.tmp"), state)?;
    fs::rename(tx.join("journal.tmp"), tx.join("journal")).map_err(|e| e.to_string())?;
    sync_dir(tx)
}
