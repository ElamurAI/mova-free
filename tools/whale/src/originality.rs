//! Bounded local lexical originality gate with configured thresholds and a configured corpus.
//! A clear result is relative to the checked documents only.
use crate::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub struct PrimaryDocument {
    pub id: String,
    pub text: String,
    /// Hash of the original file (which may be an archive rather than extracted text).
    pub sha256: String,
    pub coverage_complete: bool,
    pub scope: String,
}
fn sha(s: impl AsRef<[u8]>) -> String {
    crate::util::hash(s)
}
#[derive(Clone)]
struct Token {
    word: String,
    start: usize,
    end: usize,
}
fn tokens(s: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices() {
        // Combining marks belong to the word: decomposed accents must not split a phrase.
        let part = c.is_alphanumeric() || ('\u{0300}'..='\u{036f}').contains(&c);
        if part {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(a) = start.take() {
            out.push(Token { word: s[a..i].to_lowercase(), start: a, end: i });
        }
    }
    if let Some(a) = start {
        out.push(Token { word: s[a..].to_lowercase(), start: a, end: s.len() });
    }
    out
}
fn key(t: &[Token]) -> String {
    t.iter().map(|x| x.word.as_str()).collect::<Vec<_>>().join("\u{1f}")
}
/// Select narrative strings. Object keys named in `exempt` (formulas, code, metadata...) are
/// skipped; every other string, including unknown fields, remains narrative.
pub fn narrative(v: &Value, exempt: &[String]) -> Value {
    fn walk(v: &Value, path: &str, exempt: &[String], out: &mut Vec<Value>) {
        match v {
            Value::String(s) => out.push(json!({"field":path,"text":s})),
            Value::Array(a) => {
                for (i, x) in a.iter().enumerate() {
                    walk(x, &format!("{path}/{i}"), exempt, out);
                }
            }
            Value::Object(m) => {
                for (k, x) in m {
                    if !exempt.iter().any(|e| e == k) {
                        walk(x, &format!("{path}/{k}"), exempt, out);
                    }
                }
            }
            _ => (),
        }
    }
    let mut out = Vec::new();
    walk(v, "", exempt, &mut out);
    json!(out)
}
struct Doc {
    id: String,
    text: String,
    hash: String,
    source_hash: String,
    primary: bool,
    scope: String,
}
fn bound(c: &Value, k: &str, default: u64, min: u64, cap: u64) -> Result<u64> {
    let n = match &c[k] {
        Value::Null => default,
        v => v.as_u64().ok_or_else(|| format!("originality {k} must be an integer"))?,
    };
    if n < min || n > cap {
        return Err(format!("invalid originality bound {k}").into());
    }
    Ok(n)
}
/// Thresholds. Defaults are the generic building-block values; a specification may tighten
/// or relax them within the admitted bounds. They are editorial screening policy, not
/// semantic error bounds.
struct Thresholds {
    exact: usize,
    shingle: usize,
    trigram: usize,
    windows: Vec<usize>,
    near_percent: usize,
    aggregate_min: usize,
    aggregate_percent: usize,
}
fn thresholds(c: &Value) -> Result<Thresholds> {
    let windows = match &c["near_window_tokens"] {
        Value::Null => vec![16, 24, 32],
        v => v
            .as_array()
            .ok_or("near_window_tokens must be an array")?
            .iter()
            .map(|x| x.as_u64().filter(|n| (4..=256).contains(n)).map(|n| n as usize))
            .collect::<Option<Vec<_>>>()
            .ok_or("near_window_tokens entries must be 4..256")?,
    };
    if windows.is_empty() || windows.len() > 8 {
        return Err("near_window_tokens needs 1..8 widths".into());
    }
    let exact = bound(c, "exact_run_tokens", 12, 4, 64)? as usize;
    let shingle = bound(c, "aggregate_shingle_tokens", 5, 2, 32)? as usize;
    let trigram = bound(c, "near_gram_tokens", 3, 2, 16)? as usize;
    if exact == shingle || exact == trigram || shingle == trigram {
        return Err("exact, shingle and near-gram token sizes must be distinct".into());
    }
    Ok(Thresholds {
        exact,
        shingle,
        trigram,
        windows,
        near_percent: bound(c, "near_percent", 70, 1, 100)? as usize,
        aggregate_min: bound(c, "aggregate_min_tokens", 20, 1, 1_000_000)? as usize,
        aggregate_percent: bound(c, "aggregate_percent", 10, 1, 100)? as usize,
    })
}
fn visit(p: &Path, found: &mut Vec<PathBuf>, entries: &mut usize, max: usize, complete: &mut bool, exts: &[String]) -> Result<()> {
    *entries += 1;
    if *entries > max {
        *complete = false;
        return Ok(());
    }
    let m = fs::symlink_metadata(p)?;
    if m.file_type().is_symlink() {
        *complete = false;
        return Ok(());
    }
    if m.is_dir() {
        let mut children = fs::read_dir(p)?
            .take(max + 1)
            .map(|x| x.map(|x| x.path()))
            .collect::<std::io::Result<Vec<_>>>()?;
        if children.len() > max {
            *complete = false;
            children.truncate(max);
        }
        children.sort();
        for child in children {
            if *entries >= max {
                *complete = false;
                break;
            }
            visit(&child, found, entries, max, complete, exts)?;
        }
    } else if m.is_file()
        && p.extension().and_then(|x| x.to_str()).is_some_and(|x| exts.iter().any(|e| e == x))
    {
        found.push(p.to_owned());
    }
    Ok(())
}
/// Check full primary documents and every configured local narrative document.
/// Config: comparison_roots (required, possibly empty), comparison_extensions (default md, txt),
/// cache_path (optional), max_entries, max_bytes, max_document_bytes, max_output_bytes and the
/// thresholds above. No network, teacher request, upload, repair or publication occurs here.
pub fn check(config: &Value, prose: &Value, primary_docs: &[PrimaryDocument]) -> Result<Value> {
    let t = thresholds(config)?;
    let max_entries = bound(config, "max_entries", 20000, 1, 100000)? as usize;
    let max_bytes = bound(config, "max_bytes", 128 << 20, 1, 512 << 20)?;
    let max_doc = bound(config, "max_document_bytes", 8 << 20, 1, 64 << 20)?;
    let max_output = bound(config, "max_output_bytes", 2 << 20, 1, 8 << 20)?;
    let exts: Vec<String> = match &config["comparison_extensions"] {
        Value::Null => vec!["md".into(), "txt".into()],
        v => serde_json::from_value(v.clone())?,
    };
    let mut reasons = Vec::<String>::new();
    let mut docs = Vec::new();
    if primary_docs.is_empty() {
        reasons.push("primary comparison absent".into());
    }
    for p in primary_docs {
        if !p.coverage_complete || tokens(&p.text).is_empty() {
            reasons.push(format!("incomplete or empty primary comparison: {}", p.id));
        }
        if p.text.len() as u64 > max_doc {
            reasons.push(format!("primary exceeds document bound: {}", p.id));
            continue;
        }
        docs.push(Doc { id: p.id.clone(), text: p.text.clone(), hash: sha(&p.text), source_hash: p.sha256.clone(), primary: true, scope: p.scope.clone() });
    }
    let mut complete = true;
    let mut entries = 0;
    let mut paths = Vec::new();
    if let Some(roots) = config["comparison_roots"].as_array() {
        for root in roots {
            let p = root.as_str().ok_or("comparison root must be a path string")?;
            if let Err(e) = visit(Path::new(p), &mut paths, &mut entries, max_entries, &mut complete, &exts) {
                complete = false;
                reasons.push(format!("comparison traversal failed: {p}: {e}"));
            }
        }
    } else {
        complete = false;
        reasons.push("intended comparison_roots not configured".into());
    }
    paths.sort();
    paths.dedup();
    let cache = if let Some(p) = config["cache_path"].as_str() {
        if let Some(parent) = Path::new(p).parent() {
            fs::create_dir_all(parent)?;
        }
        let db = Connection::open(p)?;
        // Short-lived independent readers/writers share a bounded lock wait; no WAL bootstrap race.
        db.busy_timeout(std::time::Duration::from_secs(5))?;
        db.execute_batch("CREATE TABLE IF NOT EXISTS originality_text_v1 (sha TEXT PRIMARY KEY,text TEXT NOT NULL);")?;
        Some(db)
    } else {
        None
    };
    let mut bytes = 0u64;
    for p in paths {
        let size = match fs::metadata(&p) {
            Ok(x) => x.len(),
            Err(e) => {
                complete = false;
                reasons.push(format!("comparison stat failed: {e}"));
                continue;
            }
        };
        if size > max_doc || bytes.saturating_add(size) > max_bytes {
            complete = false;
            reasons.push(format!("comparison byte bound: {}", p.display()));
            continue;
        }
        bytes += size;
        let raw = match fs::read(&p) {
            Ok(x) => x,
            Err(e) => {
                complete = false;
                reasons.push(format!("comparison read failed: {e}"));
                continue;
            }
        };
        if raw.len() as u64 != size {
            complete = false;
            reasons.push("comparison changed while reading".into());
            continue;
        }
        let hash = sha(&raw);
        let text = if let Some(db) = &cache {
            let old: Option<String> = db
                .query_row("SELECT text FROM originality_text_v1 WHERE sha=?1", params![hash], |r| r.get(0))
                .optional()?;
            if let Some(t) = old {
                if sha(&t) != hash {
                    return Err("originality cache tamper/hash mismatch".into());
                }
                t
            } else {
                let t = String::from_utf8(raw)?;
                db.execute("INSERT OR IGNORE INTO originality_text_v1(sha,text) VALUES(?1,?2)", params![hash, t])?;
                let winner: String = db.query_row("SELECT text FROM originality_text_v1 WHERE sha=?1", params![hash], |r| r.get(0))?;
                if sha(&winner) != hash {
                    return Err("originality cache tamper/hash mismatch after concurrent insert".into());
                }
                winner
            }
        } else {
            String::from_utf8(raw)?
        };
        docs.push(Doc { id: p.display().to_string(), text, hash: hash.clone(), source_hash: hash, primary: false, scope: "full configured local narrative file".into() });
    }
    if !complete {
        reasons.push("intended local comparison corpus incompletely indexed".into());
    }
    let fields = prose.as_array().ok_or("narrative must be explicit field/text array")?;
    let output_bytes: usize = fields.iter().map(|f| f["text"].as_str().map(str::len).unwrap_or(0)).sum();
    if output_bytes as u64 > max_output {
        return Err("narrative exceeds originality output bound".into());
    }
    let mut outs = Vec::new();
    let mut total = 0;
    for f in fields {
        let name = f["field"].as_str().ok_or("narrative field missing")?;
        let text = f["text"].as_str().ok_or("narrative text missing")?;
        let ts = tokens(text);
        total += ts.len();
        outs.push((name, text, ts));
    }
    if total == 0 {
        reasons.push("empty narrative".into());
    }
    // The shingle union is the aggregate metric; the short-gram union detects window near
    // copying even where substitutions break every exact run.
    let mut covered: Vec<Vec<bool>> = outs.iter().map(|x| vec![false; x.2.len()]).collect();
    let mut matches = Vec::new();
    let (mut exact_count, mut near_count) = (0, 0);
    let mut doc_receipts = Vec::new();
    let sizes = [t.exact, t.shingle, t.trigram];
    for d in &docs {
        let ts = tokens(&d.text);
        let mut index: BTreeMap<(usize, String), usize> = BTreeMap::new();
        for n in sizes {
            for (i, w) in ts.windows(n).enumerate() {
                index.entry((n, key(w))).or_insert(i);
            }
        }
        let mut matched_in_doc = 0usize;
        for (fi, (name, text, out)) in outs.iter().enumerate() {
            let mut tri = vec![false; out.len()];
            for n in sizes {
                for (i, w) in out.windows(n).enumerate() {
                    let Some(&j) = index.get(&(n, key(w))) else { continue };
                    let located = |kind: &str| json!({"kind":kind,"tokens":n,"field":name,"output_token_start":i,"output_byte_start":out[i].start,"output_byte_end":out[i+n-1].end,"source_id":d.id,"source_sha256":d.source_hash,"text_sha256":d.hash,"source_token_start":j,"source_byte_start":ts[j].start,"source_byte_end":ts[j+n-1].end,"snippet":&text[out[i].start..out[i+n-1].end]});
                    if n == t.trigram {
                        tri[i..i + n].fill(true);
                    }
                    if n == t.shingle {
                        covered[fi][i..i + n].fill(true);
                        matched_in_doc += 1;
                        if matches.len() < 100 {
                            matches.push(located("aggregate_shingle"));
                        }
                    }
                    if n == t.exact {
                        exact_count += 1;
                        if matches.len() < 100 {
                            matches.push(located("exact_run"));
                        }
                    }
                }
            }
            // Bounded local lexical window heuristic, not a semantic paraphrase detector.
            'widths: for &width in &t.windows {
                if out.len() >= width {
                    for i in 0..=out.len() - width {
                        let hits = tri[i..i + width].iter().filter(|x| **x).count();
                        if hits * 100 >= width * t.near_percent {
                            near_count += 1;
                            if matches.len() < 100 {
                                matches.push(json!({"kind":"lexical_near_copy_window","field":name,"output_token_start":i,"output_token_end":i+width,"output_byte_start":out[i].start,"output_byte_end":out[i+width-1].end,"shared_gram_token_count":hits,"window_tokens":width,"source_id":d.id,"source_sha256":d.source_hash,"text_sha256":d.hash,"snippet":&text[out[i].start..out[i+width-1].end]}));
                            }
                            break 'widths;
                        }
                    }
                }
            }
        }
        doc_receipts.push(json!({"id":d.id,"source_sha256":d.source_hash,"text_sha256":d.hash,"primary":d.primary,"scope":d.scope,"bytes":d.text.len(),"tokens":ts.len(),"shared_shingles":matched_in_doc}));
    }
    let matched = covered.iter().flatten().filter(|x| **x).count();
    let aggregate = matched >= t.aggregate_min && matched * 100 >= total * t.aggregate_percent;
    if exact_count > 0 {
        reasons.push("hard exact-run verbatim rule".into());
    }
    if near_count > 0 {
        reasons.push("lexical near-copy window threshold".into());
    }
    if aggregate {
        reasons.push("aggregate shared-shingle union threshold".into());
    }
    let decision = if exact_count > 0 || near_count > 0 || aggregate {
        "blocked_overlap"
    } else if !reasons.is_empty() {
        "pending_incomplete"
    } else {
        "clear_in_checked_corpus"
    };
    Ok(json!({"schema":2,"policy":"local_lexical_originality_v2","decision":decision,"publication_allowed":decision=="clear_in_checked_corpus","reasons":reasons,
        "narrative_sha256":sha(serde_json::to_vec(prose)?),"narrative_fields":fields.iter().map(|x|x["field"].clone()).collect::<Vec<_>>(),
        "output_tokens":total,"output_bytes":output_bytes,"matched_union_output_tokens":matched,
        "aggregate_overlap_fraction":if total==0{0.0}else{matched as f64/total as f64},"exact_run_count":exact_count,"near_copy_window_count":near_count,
        "matches":matches,"match_locations_truncated":matches.len()>=100,"documents":doc_receipts,
        "coverage":{"complete":complete && !primary_docs.is_empty() && primary_docs.iter().all(|x|x.coverage_complete && !x.text.trim().is_empty() && x.text.len() as u64<=max_doc),"entries_visited":entries,"local_bytes_read":bytes,"configured_roots":config["comparison_roots"],"index":"SHA-pinned SQLite text cache; source bytes rehashed each invocation"},
        "thresholds":{"exact_run_tokens":t.exact,"aggregate_shingle_tokens":t.shingle,"near_gram_tokens":t.trigram,"near_window_tokens":t.windows,"near_percent":t.near_percent,"aggregate_min_tokens":t.aggregate_min,"aggregate_percent":t.aggregate_percent},
        "metrics":{"tokenization":"Unicode alphanumeric/combining-mark runs, lowercase; no transliteration or canonical normalization","exact_rule":"any contiguous exact-run match blocks","near_copy":"a window whose token positions are covered by matching short grams at or above near_percent within one comparison document blocks","aggregate":"union of output token positions covered by shared shingles across all documents at or above both aggregate bounds blocks","thresholds":"editorial screening policy; not semantic error bounds"},
        "uncertainty":["Only the complete explicitly configured local corpus was checked; no web coverage claim.","Lexical heuristics can miss semantic copying, translations and substantially rewritten text; manual editorial review remains required.","Common technical phrases may trigger review; no automatic paraphrase or retry."]}))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn p(s: &str) -> PrimaryDocument {
        PrimaryDocument { id: "primary".into(), text: s.into(), sha256: sha(s), coverage_complete: true, scope: "full fixture".into() }
    }
    fn c() -> Value {
        json!({"comparison_roots":[]})
    }
    fn prose(s: &str) -> Value {
        json!([{"field":"/text","text":s}])
    }
    fn dir(tag: &str) -> PathBuf {
        let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let d = base.join(format!("whale-originality-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }
    const SOURCE: &str = "The careful observer records every distinct transition before estimating the probability of any future event because hidden dependencies can otherwise distort the resulting mathematical interpretation.";
    #[test]
    fn exact_copy_and_unicode_case_block() {
        assert_eq!(check(&c(), &prose(SOURCE), &[p(SOURCE)]).unwrap()["decision"], "blocked_overlap");
        let s = "Кожний дослідник ретельно перевіряє усі початкові припущення перед тим як робити висновки про поведінку системи";
        assert_eq!(check(&c(), &prose(&s.to_uppercase()), &[p(s)]).unwrap()["decision"], "blocked_overlap");
    }
    #[test]
    fn lexical_near_copy_without_exact_run_blocks() {
        let out = "The careful observer notes every distinct transition before measuring the probability of any future outcome because hidden dependencies can otherwise skew the resulting mathematical interpretation.";
        let r = check(&c(), &prose(out), &[p(SOURCE)]).unwrap();
        assert_eq!(r["exact_run_count"], 0);
        assert_eq!(r["decision"], "blocked_overlap");
        assert!(r["near_copy_window_count"].as_u64().unwrap() > 0);
    }
    #[test]
    fn configured_thresholds_change_the_decision_within_bounds() {
        let out = "The careful observer notes every distinct transition before measuring the probability of any future outcome because hidden dependencies can otherwise skew the resulting mathematical interpretation.";
        let loose = json!({"comparison_roots":[],"near_percent":100,"aggregate_percent":100,"aggregate_min_tokens":1000});
        assert_eq!(check(&loose, &prose(out), &[p(SOURCE)]).unwrap()["decision"], "clear_in_checked_corpus");
        assert!(check(&json!({"comparison_roots":[],"exact_run_tokens":2}), &prose(out), &[p(SOURCE)]).is_err());
        assert!(check(&json!({"comparison_roots":[],"near_window_tokens":[]}), &prose(out), &[p(SOURCE)]).is_err());
    }
    #[test]
    fn aggregate_union_not_double_counting_and_negative_control() {
        let out = "The careful observer records every distinct transition. Completely fresh unfamiliar unrelated material occupies this different account. Before estimating the probability of any future event. Another independent discussion offers separate observations about geometry. Because hidden dependencies can otherwise distort. New independent phrasing finishes this deliberately separated account.";
        let r = check(&c(), &prose(out), &[p(SOURCE), p(SOURCE)]).unwrap();
        assert_eq!(r["decision"], "blocked_overlap");
        assert!(r["matched_union_output_tokens"].as_u64().unwrap() <= r["output_tokens"].as_u64().unwrap());
        let clear = check(&c(), &prose("Fresh original reasoning describes a lattice with independent coordinates."), &[p(SOURCE)]).unwrap();
        assert_eq!(clear["decision"], "clear_in_checked_corpus");
    }
    #[test]
    fn configured_exempt_fields_are_distinct_but_unknown_prose_never_exempt() {
        let v = json!({"text":"independent narrative","metadata":{"title":SOURCE},"formula":SOURCE,"unexpected_prose":SOURCE});
        let exempt = vec!["metadata".to_string(), "formula".to_string()];
        let n = narrative(&v, &exempt);
        assert_eq!(n.as_array().unwrap().len(), 2);
        assert_eq!(check(&c(), &n, &[p(SOURCE)]).unwrap()["decision"], "blocked_overlap");
        assert_eq!(narrative(&v, &[]).as_array().unwrap().len(), 4);
    }
    #[test]
    fn incomplete_and_missing_scope_fail_closed() {
        let mut source = p(SOURCE);
        source.coverage_complete = false;
        assert_eq!(check(&c(), &prose("unrelated"), &[source]).unwrap()["decision"], "pending_incomplete");
        assert_eq!(check(&json!({}), &prose("unrelated"), &[p(SOURCE)]).unwrap()["decision"], "pending_incomplete");
    }
    #[test]
    fn four_concurrent_cold_cache_guards_share_pinned_text() {
        use std::sync::{Arc, Barrier};
        for round in 0..4 {
            let dir = dir(&format!("concurrent-{round}"));
            fs::write(dir.join("a.md"), SOURCE).unwrap();
            let cfg = json!({"comparison_roots":[dir.join("a.md")],"cache_path":dir.join("cache.sqlite")});
            let barrier = Arc::new(Barrier::new(4));
            let handles = (0..4)
                .map(|_| {
                    let (cfg, barrier) = (cfg.clone(), barrier.clone());
                    std::thread::spawn(move || {
                        barrier.wait();
                        check(&cfg, &prose("Independent unrelated narrative."), &[p("other source")]).map_err(|e| e.to_string())
                    })
                })
                .collect::<Vec<_>>();
            let reports = handles.into_iter().map(|h| h.join().unwrap().unwrap()).collect::<Vec<_>>();
            for r in &reports {
                assert_eq!(r["decision"], "clear_in_checked_corpus");
                assert_eq!(r, &reports[0]);
            }
            let db = Connection::open(dir.join("cache.sqlite")).unwrap();
            assert_eq!(db.query_row("SELECT count(*) FROM originality_text_v1", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
            drop(db);
            fs::remove_dir_all(dir).unwrap();
        }
    }
    #[test]
    fn long_text_and_cache_tamper_controls() {
        let out = (0..25000).map(|i| format!("distinct{i} ")).collect::<String>();
        assert_eq!(check(&c(), &prose(&out), &[p(SOURCE)]).unwrap()["decision"], "clear_in_checked_corpus");
        let dir = dir("tamper");
        fs::write(dir.join("a.md"), SOURCE).unwrap();
        let db = dir.join("index.sqlite");
        let cfg = json!({"comparison_roots":[dir.join("a.md")],"cache_path":db});
        assert_eq!(check(&cfg, &prose("unrelated"), &[p("other")]).unwrap()["decision"], "clear_in_checked_corpus");
        let conn = Connection::open(&db).unwrap();
        conn.execute("UPDATE originality_text_v1 SET text='tampered'", []).unwrap();
        assert!(check(&cfg, &prose("unrelated"), &[p("other")]).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
