//! The topic corpora's side of the one shared list of hard cases (`mova-hard-case-list`, format
//! `mova-hard-case-v2`): sentences Mova's parser reads badly, found while a corpus is read, kept
//! until the parser learns them. The path comes from the configuration; the same file holds the
//! Wikipedia corpus's cases. This module judges a topic-corpus parse (`issues`), records what it
//! finds with the document's licence and a pointer back to its document (`record`), re-judges its
//! own open cases (`recheck`) and writes a document's sentences as a hard-case bundle (`bundle`)
//! for the Fish priority lane (`tools/hard-cases`).
use anyhow::Result;
pub use hard_case_list::key;
use hard_case_list::{Document, Found, EN};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

/// This reader's source name in the shared list.
pub const SOURCE: &str = "topic-corpus";

/// The issues of one parsed sentence: no words, a tree without exactly one root, a head outside
/// the sentence, or no frame for a sentence of six words or more.
pub fn issues(words: &[(String, String, String, usize, String)], trees: &[usize], frames: usize) -> Vec<String> {
    let mut v = vec![];
    if words.is_empty() {
        v.push("no_words".into());
        return v;
    }
    if trees.iter().any(|&roots| roots != 1) {
        v.push("roots".into());
    }
    if words.iter().any(|w| w.3 > words.len()) {
        v.push("head_outside".into());
    }
    if frames == 0 && words.iter().filter(|w| w.2 != "PUNCT").count() >= 6 {
        v.push("no_frames".into());
    }
    v
}

/// The latest state per key, as JSON: the found record's fields plus `status` (the `en` view).
pub fn state(path: &Path) -> Result<BTreeMap<String, Value>> {
    Ok(hard_case_list::state(path)?
        .into_iter()
        .map(|(k, c)| {
            let mut v = serde_json::to_value(&c.found).unwrap_or(Value::Null);
            v["status"] = json!(c.status(EN));
            v["key"] = json!(k);
            (k, v)
        })
        .collect())
}

/// Record the found cases (text, locator `{corpus, doc, s}`, reasons) of one document, with its
/// licence, that are not already open in the list. Returns how many were new.
pub fn record(path: &Path, found: &[(String, Value, Vec<String>)], licence: &str, reader: &str) -> Result<usize> {
    let fs: Vec<Found> = found
        .iter()
        .map(|(text, at, why)| {
            let doc = at["doc"].as_str().unwrap_or("").to_string();
            let s = at["s"].as_u64().unwrap_or(0) as usize;
            Found {
                text: text.trim().to_string(),
                source: SOURCE.into(),
                id: format!("{doc}:{s}"),
                document: Document { id: doc, n: s },
                at: at.clone(),
                why: why.clone(),
                licence: licence.to_string(),
                model: reader.to_string(),
                reader: reader.to_string(),
                draft_conllu: None,
                witness_uas: None,
            }
        })
        .collect();
    hard_case_list::record(path, &fs, EN)
}

/// Parse every open case this reader found again with `check` (sentence → issues); record
/// `learned` for those without issues now. Cases of other sources are left to their readers.
/// Returns (open before, learned now).
pub fn recheck(path: &Path, reader: &str, check: &dyn Fn(&str) -> Result<Vec<String>>) -> Result<(usize, usize)> {
    let (n, learned, _) = hard_case_list::recheck(path, Some(SOURCE), EN, reader, reader, false, &mut |cases| {
        cases.iter().map(|c| check(&c.found.text).map(Some)).collect()
    })?;
    Ok((n, learned))
}

/// One sentence as a CoNLL-U block: forms, lemmas, UPOS, heads and relations from the reader's
/// words; XPOS, FEATS and MISC unknown.
pub fn conllu(id: &str, text: &str, words: &[(String, String, String, usize, String)]) -> String {
    let mut s = format!("# sent_id = {id}\n# text = {text}\n");
    for (i, w) in words.iter().enumerate() {
        s += &format!("{}\t{}\t{}\t{}\t_\t_\t{}\t{}\t_\t_\n", i + 1, w.0, w.1, w.2, w.3, w.4);
    }
    s
}

/// A parsed document as a hard-case bundle (`mova-hard-bundle-v1`): its sentences in order with
/// their trees, so a hard sentence is annotated in its document's context. The pragmatics rows are
/// left empty: the lane labels them with the pragmatics SLM.
pub fn bundle(corpus: &str, doc: &crate::corpus::Doc, sents: &[crate::corpus::Sentence]) -> Value {
    json!({
        "schema": "mova-hard-bundle-v1",
        "source": SOURCE,
        "document": doc.id,
        "title": doc.title,
        "version": format!("{corpus}:{}:{}", doc.text_sha256, crate::corpus::READER),
        "licence": doc.licence,
        "model": crate::corpus::READER,
        "sentences": sents.iter().map(|s| {
            let id = format!("{}:{}", doc.id, s.s);
            json!({"id": id, "n": s.s, "text": s.text, "conllu": conllu(&id, &s.text, &s.words), "prag": ""})
        }).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_once_open_until_learned() {
        let d = crate::library::tests::tmp("hard");
        let _ = std::fs::remove_dir_all(&d);
        let p = d.join("hard-cases.jsonl");
        let w = |n: usize| (0..n).map(|i| (format!("w{i}"), "w".to_string(), "NOUN".to_string(), 0, "root".to_string())).collect::<Vec<_>>();
        assert_eq!(issues(&w(7), &[1], 0), vec!["no_frames"]);
        assert_eq!(issues(&w(3), &[2], 1), vec!["roots"]);
        assert!(issues(&w(7), &[1], 1).is_empty());
        let f = vec![("A hard sentence .".to_string(), json!({"corpus": "c", "doc": "lib:x", "s": 3}), vec!["no_frames".to_string()])];
        assert_eq!(record(&p, &f, "CC BY 4.0", "r1").unwrap(), 1);
        assert_eq!(record(&p, &f, "CC BY 4.0", "r1").unwrap(), 0);
        let st = state(&p).unwrap();
        let c = &st[&key("A hard sentence .")];
        assert_eq!(c["licence"], "CC BY 4.0");
        assert_eq!(c["document"]["id"], "lib:x");
        assert_eq!(c["id"], "lib:x:3");
        // Still bad: stays open. Then the parser learns it.
        assert_eq!(recheck(&p, "r1", &|_| Ok(vec!["no_frames".into()])).unwrap(), (1, 0));
        assert_eq!(recheck(&p, "r2", &|_| Ok(vec![])).unwrap(), (1, 1));
        assert_eq!(state(&p).unwrap()[&key("A hard sentence .")]["status"], "learned");
        // Met bad again later: open again.
        assert_eq!(record(&p, &f, "CC BY 4.0", "r3").unwrap(), 1);
        assert_eq!(state(&p).unwrap()[&key("A hard sentence .")]["status"], "open");
        // Another source's case is not rechecked by this reader (negative control for the filter).
        hard_case_list::record(&p, &[Found { text: "Wiki one.".into(), source: "wikipedia".into(), id: "wiki:X:1".into(), document: Document { id: "wiki:X".into(), n: 1 }, at: json!({}), why: vec!["witness".into()], licence: "CC BY-SA 4.0".into(), model: "en.bin:x".into(), reader: "run-1".into(), draft_conllu: None, witness_uas: None }], EN).unwrap();
        assert_eq!(recheck(&p, "r4", &|_| Ok(vec![])).unwrap(), (1, 1));
        assert_eq!(state(&p).unwrap()[&key("Wiki one.")]["status"], "open");
        assert_eq!(conllu("d:1", "A b", &[("A".into(), "a".into(), "DET".into(), 2, "det".into()), ("b".into(), "b".into(), "NOUN".into(), 0, "root".into())]), "# sent_id = d:1\n# text = A b\n1\tA\ta\tDET\t_\t_\t2\tdet\t_\t_\n2\tb\tb\tNOUN\t_\t_\t0\troot\t_\t_\n");
        let _ = std::fs::remove_dir_all(d);
    }
}
