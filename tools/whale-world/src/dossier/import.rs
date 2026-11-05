//! Importers: the material that already exists about a document becomes texts of its dossier.
//! Nothing is fetched and no terminal is asked. Idempotent: a source already registered with the
//! same kind and hash is skipped, so importing twice adds nothing.
//!
//! - `original` (epoch 0): the library text (`text.md`, else a local-only copy), with its licence;
//! - `site-old` (epoch 0): the configured files of earlier site descriptions;
//! - `web` (epoch 0): `web` documents of the document's topic corpus (read through the corpus);
//! - `sol` (epoch 1): consultant drafts registered in the document's topic corpus;
//! - `mova` (epoch 1): the last (or every) round's `description.txt` of each learner state root.
use super::{Dossier, Kind, NewText, TextRow};
use crate::config::{Config, Rounds};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use topic_corpus::corpus::{Corpus, Options};
use topic_corpus::library::Library;

fn mtime(p: &Path) -> Option<u64> {
    std::fs::metadata(p).ok()?.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
}

/// Add unless a text of this kind with the same hash and source is registered.
fn add_once(d: &Dossier, rows: &mut Vec<TextRow>, t: NewText) -> Result<Option<TextRow>> {
    let sha = super::sha(t.text.as_bytes());
    if t.text.trim().is_empty() || rows.iter().any(|r| r.kind == t.kind && r.sha256 == sha && r.source == t.source) {
        return Ok(None);
    }
    let r = d.add(t)?;
    rows.push(r.clone());
    Ok(Some(r))
}

/// The site description directories of `doc` under `root`.
pub fn site_dirs(root: &Path, doc: &str) -> Vec<PathBuf> {
    let c: Vec<char> = doc.chars().collect();
    let p = |n: usize| c.iter().take(n).collect::<String>();
    let mut v = vec![];
    if c.len() >= 3 {
        v.push(root.join(p(1)).join(p(2)).join(p(3)).join(doc));
    }
    if c.len() >= 2 {
        v.push(root.join(p(1)).join(p(2)).join(doc));
    }
    v.into_iter().filter(|d| d.is_dir()).collect()
}

/// Run every importer for `doc`. `lib` is the loaded library (originals need it).
pub fn all(cfg: &Config, lib: Option<&Library>, doc: &str) -> Result<Value> {
    let d = Dossier::open(cfg, doc)?;
    let mut rows = d.rows()?;
    let mut added: Vec<String> = vec![];
    let mut notes: Vec<String> = vec![];
    // Original.
    let mut original: Option<String> = rows.iter().find(|r| r.kind == Kind::Original).map(|r| r.id.clone());
    match lib.and_then(|l| l.get(doc).map(|w| (l, w))) {
        Some((l, w)) => match l.text(w) {
            Some((p, local)) => {
                let text = std::fs::read_to_string(&p)?;
                let src = p.display().to_string();
                if let Some(r) = add_once(&d, &mut rows, NewText { epoch: 0, kind: Kind::Original, text: &text, source: &src, licence: Some(&w.licence), local_only: local || w.text_local_only, inputs: vec![], created_unix: mtime(&p) })? {
                    original = Some(r.id.clone());
                    added.push(r.id);
                }
            }
            None => notes.push("no library text".into()),
        },
        None => notes.push(if lib.is_none() { "no library configured".into() } else { "not in the library".into() }),
    }
    // Earlier site descriptions.
    if let Some(s) = &cfg.site_old {
        for root in &s.roots {
            for dir in site_dirs(root, doc) {
                for f in &s.files {
                    let p = dir.join(f);
                    if !p.is_file() {
                        continue;
                    }
                    let text = std::fs::read_to_string(&p)?;
                    let src = p.display().to_string();
                    if let Some(r) = add_once(&d, &mut rows, NewText { epoch: 0, kind: Kind::SiteOld, text: &text, source: &src, licence: Some(&s.licence), local_only: true, inputs: vec![], created_unix: mtime(&p) })? {
                        added.push(r.id);
                    }
                }
            }
        }
    }
    // Web pages and consultant drafts held by the document's topic corpus.
    if let Some(root) = &cfg.corpora_root {
        let dir = root.join(doc);
        if dir.join("corpus.json").is_file() {
            let mut c = Corpus::load(&dir, Options::default())?;
            let when = mtime(&dir.join("documents.jsonl"));
            let docs: Vec<_> = c.docs.iter().filter(|x| x.kind == "web" || x.kind == "sol").cloned().collect();
            for x in docs {
                let text = c.text(&x.id).map_err(|e| anyhow!("{}: {e}", x.id))?;
                let (kind, epoch, src) = if x.kind == "web" {
                    (Kind::Web, 0, x.cite.get("url").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| x.source.clone()))
                } else {
                    (Kind::Sol, 1, format!("topic-corpus {} {}", c.key, x.id))
                };
                let lic = if x.licence.is_empty() { None } else { Some(x.licence.as_str()) };
                if let Some(r) = add_once(&d, &mut rows, NewText { epoch, kind, text: &text, source: &src, licence: lic, local_only: true, inputs: vec![], created_unix: when })? {
                    added.push(r.id);
                }
            }
        }
    }
    // Mova's drafts from learner states.
    for st in &cfg.mova_state_roots {
        let pd = st.join("papers").join(doc);
        if !pd.is_dir() {
            continue;
        }
        let mut rounds: Vec<PathBuf> = std::fs::read_dir(&pd)?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("round-")) && p.join("description.txt").is_file()).collect();
        rounds.sort();
        if cfg.mova_rounds == Rounds::Last {
            rounds = rounds.into_iter().last().into_iter().collect();
        }
        for r in rounds {
            let p = r.join("description.txt");
            let text = std::fs::read_to_string(&p)?;
            let src = p.display().to_string();
            if let Some(x) = add_once(&d, &mut rows, NewText { epoch: 1, kind: Kind::Mova, text: &text, source: &src, licence: None, local_only: true, inputs: original.iter().cloned().collect(), created_unix: mtime(&p) })? {
                added.push(x.id);
            }
        }
    }
    let mut by_kind = std::collections::BTreeMap::<&str, usize>::new();
    for r in &rows {
        *by_kind.entry(r.kind.name()).or_default() += 1;
    }
    Ok(json!({"doc": doc, "added": added, "texts": rows.len(), "kinds": by_kind, "notes": notes}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dossier::tests::{cfg, tmp};

    #[test]
    fn imports_site_and_mova_once() {
        let d = tmp("import");
        let mut c = cfg(&d);
        let site = d.join("site");
        let sd = site.join("s/sm/smi/smith-2020-trees");
        std::fs::create_dir_all(&sd).unwrap();
        std::fs::write(sd.join("README.md"), "# Trees\n\nAn old description.\n").unwrap();
        c.site_old = Some(crate::config::SiteOld { roots: vec![site], files: vec!["README.md".into(), "summary.md".into()], licence: "ours".into() });
        let st = d.join("state");
        for (r, t) in [("round-001", "first"), ("round-002", "second draft")] {
            std::fs::create_dir_all(st.join("papers/smith-2020-trees").join(r)).unwrap();
            std::fs::write(st.join("papers/smith-2020-trees").join(r).join("description.txt"), t).unwrap();
        }
        c.mova_state_roots = vec![st];
        let v = all(&c, None, "smith-2020-trees").unwrap();
        assert_eq!(v["added"], json!(["smith-2020-trees/0-site-old-1", "smith-2020-trees/1-mova-1"]));
        let x = Dossier::open(&c, "smith-2020-trees").unwrap();
        assert_eq!(x.read(&x.rows().unwrap()[1]).unwrap(), "second draft");
        assert!(x.rows().unwrap().iter().all(|r| !r.public_ok));
        let again = all(&c, None, "smith-2020-trees").unwrap();
        assert_eq!(again["added"], json!([]));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn corpus_pages_and_drafts_stay_internal() {
        use topic_corpus::corpus::Doc;
        use topic_corpus::licence::{classify, Origin};
        let d = tmp("import-web");
        let mut c = cfg(&d);
        assert!(c.public_originals);
        let root = d.join("corpora");
        let mut k = Corpus::open(&root, "smith-2020-trees", "smith-2020-trees", Options::default()).unwrap();
        // A Sea Spider page under CC BY that even claims to be admissible, and a consultant draft.
        let mut w = Doc::new("web:abc", "web", Origin::Seaspider);
        w.licence = "CC BY 4.0".into();
        w.licence_class = classify(&w.licence);
        w.local_only = false;
        w.public_admissible = true;
        w.cite = json!({"url": "https://blog.example.org/trees"});
        k.register(w, "# Blog\n\nTrees are fun.\n").unwrap();
        let mut s = Doc::new("sol:d1", "sol", Origin::Sol);
        s.licence = "CC0 1.0".into();
        s.licence_class = classify(&s.licence);
        k.register(s, "# Draft\n\nTrees [1].\n\n[1] A. 2001.\n").unwrap();
        c.corpora_root = Some(root);
        let v = all(&c, None, "smith-2020-trees").unwrap();
        assert_eq!(v["added"], json!(["smith-2020-trees/0-web-1", "smith-2020-trees/1-sol-1"]));
        let rows = Dossier::open(&c, "smith-2020-trees").unwrap().rows().unwrap();
        assert_eq!(rows[0].source, "https://blog.example.org/trees");
        assert!(rows.iter().all(|r| !r.public_ok), "{rows:?}");
        assert_eq!((rows[1].citations.count, rows[1].citations.unresolved), (1, 0));
        let _ = std::fs::remove_dir_all(d);
    }
}
