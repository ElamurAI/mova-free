//! Export of sets to CoNLL-U and the lossless check. The file is reassembled as `en::conllu::Doc`
//! (block comments verbatim, rows with ten raw columns, blank lines) and written by the same
//! `Doc::text()` that read it at load time.

use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use en::conllu::{Doc, FullSentence, Row};
use serde_json::Value;

use crate::duck::{Duck, Stage, quote};
use crate::tsv;
use crate::util;

const EXPORT_SQL: &str = include_str!("../../../db/sql/export.sql");
const FILES_SQL: &str = include_str!("../../../db/sql/files.sql");

pub const KEYS: [&str; 12] = ["file", "run", "source", "tool", "layer", "dialect", "origin", "doc", "sent", "sid", "aid", "gate"];

/// Filter `key=value` → pairs; an unknown key is an error.
pub fn filters(args: &[String]) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for a in args {
        let Some((k, v)) = a.split_once('=') else { bail!("filter \"{a}\": expected key=value ({})", KEYS.join(", ")) };
        if !KEYS.contains(&k) {
            bail!("filter \"{k}\": keys are {}", KEYS.join(", "));
        }
        out.push((k.to_string(), v.to_string()));
    }
    Ok(out)
}

/// Export sets by filter. A filter of only `file=…` with one file gives the exact file
/// (blank lines at the start, end of file); otherwise blocks one after another, one blank line between them.
pub fn export(duck: &Duck, f: &[(String, String)]) -> Result<String> {
    let st = Stage::new("export")?;
    let mut sql = String::new();
    for (k, v) in f {
        sql.push_str(&format!("SET VARIABLE {k} = {};\n", quote(v)));
    }
    sql.push_str(&st.sql(EXPORT_SQL, "q")?);
    duck.read(&sql)?;
    let files = tsv::read(&st.path("files.tsv"))?;
    let exact = f.len() == 1 && f[0].0 == "file" && files.len() == 1;
    let mut rows: HashMap<i64, Vec<Row>> = HashMap::new();
    for r in tsv::read(&st.path("rows.tsv"))? {
        if r.len() != 12 {
            bail!("rows.tsv: {} columns instead of 12", r.len());
        }
        let cols: Vec<String> = r[2..].iter().map(|c| c.clone().unwrap_or_default()).collect();
        let cols: [String; 10] = cols.try_into().map_err(|_| anyhow::anyhow!("row does not have 10 columns"))?;
        rows.entry(tsv::int(&r, 0)?).or_default().push(Row(cols));
    }
    let mut doc = Doc { blank_before: 0, sents: Vec::new(), final_newline: true };
    if exact {
        doc.blank_before = tsv::int(&files[0], 1)? as usize;
        doc.final_newline = tsv::s(&files[0], 2)? == "true";
    }
    let sets = tsv::read(&st.path("sets.tsv"))?;
    if sets.is_empty() {
        bail!("filter {f:?} matched no sets");
    }
    for r in sets {
        let aid = tsv::int(&r, 0)?;
        let comments: Vec<String> = serde_json::from_str(tsv::s(&r, 3)?).with_context(|| format!("comments of set {aid}"))?;
        let blank_after = tsv::int(&r, 2)? as usize;
        doc.sents.push(FullSentence { comments, rows: rows.remove(&aid).unwrap_or_default(), blank_after: if exact { blank_after } else { blank_after.max(1) }, ..Default::default() });
    }
    Ok(doc.text())
}

/// Verification summary.
#[derive(Default, Debug)]
pub struct Verify {
    pub files: usize,
    pub bytes_ok: usize,
    pub counts_ok: usize,
    pub card_ok: usize,
    pub card_checked: usize,
    pub bad: Vec<String>,
    /// Not a failure: the file is no longer on disk, but the database reproduces the same bytes (sha1 as at load time).
    pub notes: Vec<String>,
}

/// Sentence counters as in UD statistics: sentences, surface tokens (a multiword token counts once, its
/// words are not counted), words, multiword tokens, empty nodes.
fn ud_counts(text: &str) -> Result<[i64; 5]> {
    let doc = Doc::parse(text)?;
    let mut c = [0i64; 5];
    for s in &doc.sents {
        c[0] += 1;
        let mut inside_to = 0usize;
        for r in &s.rows {
            let id = &r.0[0];
            if let Some((_, b)) = id.split_once('-') {
                c[3] += 1;
                c[1] += 1;
                inside_to = b.parse().unwrap_or(0);
            } else if id.contains('.') {
                c[4] += 1;
            } else {
                c[2] += 1;
                let n: usize = id.parse().unwrap_or(0);
                if n > inside_to {
                    c[1] += 1;
                }
            }
        }
    }
    Ok(c)
}

/// For every loaded file: (1) the export from the database gives the same bytes as the file on disk and the
/// same sha1 as at load time; (2) counters in the database match the file; (3) for treebanks, the part
/// size matches the card (`treebanks/…/README.md`). Any mismatch goes to `bad`.
pub fn verify(duck: &Duck, only: Option<&str>) -> Result<Verify> {
    let st = Stage::new("verify")?;
    duck.read(&st.sql(FILES_SQL, "q")?)?;
    let mut v = Verify::default();
    let mut cards: HashMap<String, Value> = HashMap::new();
    for r in tsv::read(&st.path("files.tsv"))? {
        let (id, path, name, sha) = (tsv::int(&r, 0)?, tsv::s(&r, 1)?, tsv::s(&r, 2)?, tsv::s(&r, 3)?);
        if only.is_some_and(|o| o != name && o != path) {
            continue;
        }
        v.files += 1;
        let file_counts: Vec<i64> = (4..9).map(|i| tsv::int(&r, i)).collect::<Result<_>>()?;
        let db_counts: Vec<i64> = (9..14).map(|i| tsv::int(&r, i)).collect::<Result<_>>()?;
        let out = export(duck, &[("file".into(), id.to_string())])?;
        let disk = std::fs::read(path).ok();
        let out_sha = util::sha1_hex(out.as_bytes());
        match &disk {
            Some(d) if d.as_slice() == out.as_bytes() && out_sha == sha => v.bytes_ok += 1,
            Some(d) => {
                let at = d.iter().zip(out.as_bytes()).position(|(a, b)| a != b).unwrap_or(d.len().min(out.len()));
                v.bad.push(format!("{name}: export differs from the file at byte {at} (lengths {} and {}; database sha1 {})", out.len(), d.len(), if out_sha == sha { "as at load time" } else { "different" }));
            }
            None if out_sha == sha => {
                v.bytes_ok += 1;
                v.notes.push(format!("{name}: file {path} is not on disk; export sha1 is as at load time"));
            }
            None => v.bad.push(format!("{name}: file is not on disk, export sha1 differs")),
        }
        if file_counts == db_counts {
            v.counts_ok += 1;
        } else {
            v.bad.push(format!("{name}: file counters {file_counts:?} ≠ database {db_counts:?} (blocks, rows, words, multiword, empty)"));
        }
        let card = r.get(15).cloned().flatten();
        if let Some(card) = card {
            let part = ["train", "dev", "test"].into_iter().find(|p| name.ends_with(&format!("-ud-{p}.conllu")));
            let c = match cards.get(&card) {
                Some(c) => c.clone(),
                None => {
                    let c = card_volume(&card)?;
                    cards.insert(card.clone(), c.clone());
                    c
                }
            };
            if let Some(p) = part {
                v.card_checked += 1;
                let want: Vec<i64> = c[p].as_array().map(|a| a.iter().filter_map(Value::as_i64).collect()).unwrap_or_default();
                let got = ud_counts(&out)?;
                if want == got {
                    v.card_ok += 1;
                } else {
                    v.bad.push(format!("{name}: size from the database {got:?} ≠ card {want:?} (sentences, tokens, words, multiword, empty)"));
                }
            }
        }
    }
    Ok(v)
}

/// The size table of a treebank card: part → [sentences, tokens, words, fused, empty].
fn card_volume(card: &str) -> Result<Value> {
    let text = std::fs::read_to_string(card).with_context(|| card.to_string())?;
    let mut out = serde_json::Map::new();
    let mut on = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            on = util::CARD_SIZE_HEADINGS.contains(&line.trim());
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if on && cells.len() >= 8 {
            let part = cells[1].trim_matches('*');
            let nums: Vec<i64> = cells[2..7].iter().filter_map(|c| c.chars().filter(char::is_ascii_digit).collect::<String>().parse().ok()).collect();
            if nums.len() == 5 {
                let part = util::card_part(part);
                out.insert(part.to_string(), serde_json::json!(nums));
            }
        }
    }
    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ud_counts_mwt_and_empty() {
        let t = "# sent_id = a\n1-2\tDon't\t_\t_\t_\t_\t_\t_\t_\t_\n1\tDo\tdo\tAUX\tVBP\t_\t3\taux\t_\t_\n2\tn't\tnot\tPART\tRB\t_\t3\tadvmod\t_\t_\n3\tgo\tgo\tVERB\tVB\t_\t0\troot\t_\t_\n3.1\tgo\tgo\tVERB\tVB\t_\t_\t_\t3:conj\t_\n\n";
        assert_eq!(ud_counts(t).unwrap(), [1, 2, 3, 1, 1]);
        assert!(filters(&["file=x".into(), "source=rust".into()]).is_ok());
        assert!(filters(&["zzz=1".into()]).is_err());
        assert!(filters(&["file".into()]).is_err());
    }
}
