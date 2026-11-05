//! Checks of the texts export: selection by batch and gates, sentence citations, idempotence,
//! files edited by a person kept, foreign files kept, held pages and originals cited but never
//! copied. No model is needed.
use super::*;
use crate::dossier::tests::{cfg as world_cfg, tmp};
use crate::dossier::NewText;

const PAPER: &str = "smith-2020-trees";
const WEB: &str = "SHARE-ALIKE SENTENCE that must never be copied into the texts repository.";
const ORIGINAL: &str = "ORIGINAL SENTENCE of the paper that must never be copied.";
const DESCRIPTION: &str = "I read «Trees».\n\nTrees grow slowly in cold places. As I understand it, «GEC» corrects errors.\n\nRoots matter";

fn round(dir: &Path, description: &str, items: Value, pass: bool) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("description.txt"), description).unwrap();
    std::fs::write(dir.join("draft.json"), serde_json::to_vec(&json!({"available_words": 10, "items": items})).unwrap()).unwrap();
    std::fs::write(dir.join("gates.json"), serde_json::to_vec(&json!({"pass": pass, "words": 4100, "words_min": 4000, "words_max": 5000, "traceability": 1.0, "verbatim_max": 12})).unwrap()).unwrap();
}

fn items() -> Value {
    json!([
        {"kind": "structure", "text": "I read «Trees».", "sources": [], "knowledge": [], "section": null},
        {"kind": "source", "text": "Trees grow slowly in cold places.", "sources": [3], "knowledge": [], "section": 0},
        {"kind": "explain", "text": "As I understand it, «GEC» corrects errors.", "sources": [], "knowledge": ["k00001"], "section": 0},
        {"kind": "source", "text": "Roots matter", "sources": [7], "knowledge": [], "section": 1}
    ])
}

fn status(dir: &Path, state: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("status.json"), serde_json::to_vec(&json!({"state": state, "last_round": {"round": 1, "writer": "writer-test"}})).unwrap()).unwrap();
}

/// A batch log (batch 12: one passed paper, too early; batch 13: one passed, one stopped), one
/// learner state and a dossier with a page under CC BY-SA, the original, an earlier description,
/// a consultant draft and a Mova draft equal to the description.
fn fixture(d: &Path) -> (Config, WorldConfig) {
    let st = d.join("state/shark-4");
    let pd = st.join("papers").join(PAPER);
    status(&pd, "passed");
    round(&pd.join("round-001"), DESCRIPTION, items(), true);
    round(&pd.join("round-002"), "A later, failing draft.", json!([{"kind": "source", "text": "A later, failing draft.", "sources": [1]}]), false);
    for (p, s) in [("jones-2019-early", "passed"), ("lee-2021-short", "stopped")] {
        let x = st.join("papers").join(p);
        status(&x, s);
        round(&x.join("round-001"), "Early text.", json!([{"kind": "source", "text": "Early text.", "sources": [1]}]), s == "passed");
    }
    std::fs::create_dir_all(st.join("knowledge")).unwrap();
    std::fs::write(st.join("knowledge/items.jsonl"), "{\"id\":\"k00001\",\"key\":\"GEC\",\"provenance\":{\"source\":\"terminal\"}}\n").unwrap();
    let a = |id: &str| json!({"paper": {"id": id, "title": format!("Title of {id}"), "licence": "CC BY 4.0"}, "whale": "owl-4"});
    let log = [json!({"event": "opened", "batch": 12, "assignments": [a("jones-2019-early")]}), json!({"event": "opened", "batch": 13, "assignments": [a(PAPER), a("lee-2021-short")]})];
    std::fs::write(d.join("batches.jsonl"), log.iter().map(|v| v.to_string() + "\n").collect::<String>()).unwrap();
    let w = world_cfg(d);
    let ds = Dossier::open(&w, PAPER).unwrap();
    let add = |epoch, kind, text: &str, source: &str, licence: Option<&str>| {
        ds.add(NewText { epoch, kind, text, source, licence, local_only: true, inputs: vec![], created_unix: Some(1) }).unwrap();
    };
    add(0, Kind::Original, &format!("# Trees\n\n{ORIGINAL}\n"), "/library/s/sm/smith-2020-trees/text.md", Some("CC BY 4.0"));
    add(0, Kind::Web, &format!("# Trees on a wiki\n\n{WEB}\n"), "https://wiki.example.org/Trees", Some("CC BY-SA 4.0"));
    add(0, Kind::SiteOld, "# Trees\n\nAn earlier description of ours.\n", "/site/s/sm/smi/smith-2020-trees/README.md", Some("ours"));
    add(1, Kind::Sol, "A consultant's draft about trees [1].\n\n[1] Smith 2020.\n", "terminal", None);
    add(1, Kind::Mova, DESCRIPTION, "/state/round-001/description.txt", None);
    let c = Config {
        schema: SCHEMA.into(),
        world: d.join("world.json"),
        batches: d.join("batches.jsonl"),
        learners: BTreeMap::from([("owl-4".to_string(), st)]),
        from_batch: 13,
        dir: "papers".into(),
        describer: "the test Owls".into(),
        visibility: "internal".into(),
        licence: "ours, internal and closed".into(),
    };
    c.validate().unwrap();
    (c, w)
}

fn files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut m = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(p) = stack.pop() {
        for e in std::fs::read_dir(&p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else {
                m.insert(p.strip_prefix(root).unwrap().display().to_string(), std::fs::read(&p).unwrap());
            }
        }
    }
    m
}

fn strip_marks(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("[^") {
        out.push_str(&rest[..i]);
        rest = &rest[i + rest[i..].find(']').unwrap() + 1..];
    }
    out.push_str(rest);
    out
}

#[test]
fn selects_passed_papers_from_the_first_batch_on() {
    let d = tmp("export-select");
    let (c, _) = fixture(&d);
    let p = passed(&c).unwrap();
    assert_eq!(p.len(), 1, "{p:?}");
    assert_eq!((p[0].paper.as_str(), p[0].batch, p[0].round.file_name().unwrap().to_str().unwrap()), (PAPER, 13, "round-001"));
    // Negative control: from batch 12 the early paper is passed too.
    let mut c12 = c.clone();
    c12.from_batch = 12;
    assert_eq!(passed(&c12).unwrap().len(), 2);
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn exports_cited_article_idempotently() {
    let d = tmp("export-idem");
    let (c, w) = fixture(&d);
    let repo = d.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    let first = export(&c, &w, None, &repo, &[]).unwrap();
    assert_eq!(first["exported"], 1, "{first}");
    let before = files(&repo);
    let main = String::from_utf8(before["papers/smith-2020-trees/index.md"].clone()).unwrap();
    assert!(intact(&main));
    assert!(main.contains("visibility: \"internal\""), "{main}");
    assert!(main.contains("cold places.[^paper] As I understand it, «GEC» corrects errors.[^consult-owl-4]"), "{main}");
    assert!(main.contains("[^paper]: Title of smith-2020-trees."), "{main}");
    assert!(main.contains("citations: \"sentence\""), "{main}");
    // Stripped of its markers, the article body is the description (the last sentence closed).
    let body = main.split_once("*\n\n").unwrap().1.split("\n## ").next().unwrap();
    assert_eq!(strip_marks(body).trim(), format!("{DESCRIPTION}."));
    // Shadows: the earlier description and the consultant draft; the Mova draft equal to the main text is not repeated.
    let shadows: Vec<&String> = before.keys().filter(|k| k.contains("/shadow/")).collect();
    assert_eq!(shadows, vec!["papers/smith-2020-trees/shadow/0-site-old-1/index.md", "papers/smith-2020-trees/shadow/1-sol-1/index.md"]);
    assert!(before.contains_key("papers/smith-2020-trees/seo.json") && before.contains_key("papers/index.md") && before.contains_key("papers/export.json"));
    assert!(!before.keys().any(|k| k.contains("jones") || k.contains("lee-2021")));
    // Second run: nothing written, every byte the same.
    let second = export(&c, &w, None, &repo, &[]).unwrap();
    assert_eq!(second["report"]["written"], json!([]), "{second}");
    assert_eq!(files(&repo), before);
    // A fresh repository gets the same bytes (no clock, no unordered maps).
    let repo2 = d.join("repo2");
    std::fs::create_dir_all(&repo2).unwrap();
    export(&c, &w, None, &repo2, &[]).unwrap();
    assert_eq!(files(&repo2), before);
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn edited_and_foreign_files_are_kept() {
    let d = tmp("export-edit");
    let (c, w) = fixture(&d);
    let repo = d.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    export(&c, &w, None, &repo, &[]).unwrap();
    let mp = repo.join("papers/smith-2020-trees/index.md");
    let sealed = std::fs::read_to_string(&mp).unwrap();
    let seo = repo.join("papers/smith-2020-trees/seo.json");
    // A person edits the article and the metadata; then the upstream description changes.
    let edited = sealed.replace("# Title of smith-2020-trees", "# Trees, edited by an editor");
    std::fs::write(&mp, &edited).unwrap();
    std::fs::write(&seo, "{\"title\": \"Edited\", \"description\": \"Edited.\"}\n").unwrap();
    assert!(!intact(&edited));
    let r1 = c.learners["owl-4"].join("papers").join(PAPER).join("round-001");
    let new_items = json!([{"kind": "source", "text": "A new upstream description.", "sources": [1]}]);
    round(&r1, "A new upstream description.", new_items, true);
    let r = export(&c, &w, None, &repo, &[]).unwrap();
    let mut got: Vec<&str> = r["report"]["edited"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
    got.sort();
    assert_eq!(got, vec!["papers/smith-2020-trees/index.md", "papers/smith-2020-trees/seo.json"], "{r}");
    assert_eq!(std::fs::read_to_string(&mp).unwrap(), edited);
    assert!(std::fs::read_to_string(&seo).unwrap().contains("Edited"));
    // The edit stays detected on the next run (the manifest keeps the old hash).
    let again = export(&c, &w, None, &repo, &[]).unwrap();
    assert_eq!(again["report"]["edited"].as_array().unwrap().len(), 2, "{again}");
    // Negative control: with the editor's change undone, the gate lets the upstream change through.
    std::fs::write(&mp, &sealed).unwrap();
    let r = export(&c, &w, None, &repo, &[]).unwrap();
    assert!(r["report"]["written"].as_array().unwrap().iter().any(|x| x == "papers/smith-2020-trees/index.md"), "{r}");
    assert!(std::fs::read_to_string(&mp).unwrap().contains("A new upstream description.[^paper]"));
    // A file the exporter did not write is never replaced.
    let repo2 = d.join("repo2");
    std::fs::create_dir_all(repo2.join("papers/smith-2020-trees")).unwrap();
    std::fs::write(repo2.join("papers/smith-2020-trees/index.md"), "# A person's own article\n").unwrap();
    let r = export(&c, &w, None, &repo2, &[]).unwrap();
    assert_eq!(r["report"]["foreign"], json!(["papers/smith-2020-trees/index.md"]), "{r}");
    assert_eq!(std::fs::read_to_string(repo2.join("papers/smith-2020-trees/index.md")).unwrap(), "# A person's own article\n");
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn held_pages_and_originals_are_cited_never_copied() {
    let d = tmp("export-web");
    let (c, w) = fixture(&d);
    // Negative control: the dossier does hold both texts, so a copy would be possible.
    let ds = Dossier::open(&w, PAPER).unwrap();
    let rows = ds.rows().unwrap();
    assert!(ds.read(&rows[1]).unwrap().contains(WEB) && ds.read(&rows[0]).unwrap().contains(ORIGINAL));
    let repo = d.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    export(&c, &w, None, &repo, &[]).unwrap();
    let all = files(&repo);
    for (k, v) in &all {
        let s = String::from_utf8_lossy(v);
        assert!(!s.contains("SHARE-ALIKE SENTENCE") && !s.contains("ORIGINAL SENTENCE"), "{k} copies a held text");
    }
    let main = String::from_utf8_lossy(&all["papers/smith-2020-trees/index.md"]).to_string();
    assert!(main.contains("web page (smith-2020-trees/0-web-1): https://wiki.example.org/Trees. Licence: CC BY-SA 4.0."), "{main}");
    assert!(main.contains("original (smith-2020-trees/0-original-1): library copy `text.md`. Licence: CC BY 4.0."), "{main}");
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn reference_mentions_and_seal() {
    let refs: Vec<RefEntry> = serde_json::from_value(json!([
        {"raw": "Emily Bender et al. 2011. Parser evaluation.", "author": "Bender", "year": 2011, "title": "Parser evaluation over local and non-local deep dependencies"},
        {"raw": "Mahwah, NJ: Erlbaum.", "author": "Mahwah", "year": 2015, "title": "Neural machine translation"}
    ]))
    .unwrap();
    assert_eq!(ref_mentions("An alternative strategy; Bender et al., 2011.", &refs), vec![0]);
    assert_eq!(ref_mentions("Bender wrote in 2011a about it.", &refs), vec![0]);
    // Negative controls: another year, a year without the surname nearby.
    assert!(ref_mentions("Bender et al., 2012.", &refs).is_empty());
    assert!(ref_mentions("In 2011 the data were collected by a different team entirely.", &refs).is_empty());
    let s = seal(&format!("---\nid: \"x\"\n{HASH_KEY}{ZERO}\n---\n\nBody.\n"));
    assert!(intact(&s));
    assert!(!intact(&s.replace("Body.", "Body!")));
    assert!(!intact("# no front matter\n"));
}
