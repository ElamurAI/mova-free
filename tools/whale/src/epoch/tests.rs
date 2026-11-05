//! Epoch tests on the fixture paper with a fixture neighbourhood and a fake terminal (never a
//! model). Each gate has its negative control.
use super::*;
use crate::learn::tests::{model_ok, tmp, PAPER};
use crate::terminal::FakeTerminal;

const DOC: &str = "zorbic-2024-trees";
const NEAR: &str = "# Arrays of Heads\n\n## Introduction\n\nZorbic trees keep every head of a sentence in one array for dependency parsing. A zorbic rule attaches each word to its head in the tree. Earlier parsers stored the heads of a sentence in linked lists. The array makes each head lookup a single step for the parser.\n\n## Results\n\nOn three treebanks the zorbic parser was twice as fast as a list parser on long sentences.\n";
const PROMPT: &str = "You write about {{TITLE}} (epoch {{EPOCH}}).\nRead these sources by path:\n{{SOURCES}}\nWrite {{MIN_WORDS}} to {{MAX_WORDS}} words; cite only the numbers above.\n";

fn answer(extra: &str) -> String {
    format!("# Zorbic trees\n\nZorbic trees store every head of a sentence in one array [1]. The parser reads each word of the sentence once [1, 2]. Earlier parsers stored heads in linked lists [3]. {extra}\n\n## References\n\n[1] The paper.\n[2] Mova's text.\n[3] The neighbour.\n")
}

/// A fixture epoch: the paper, a world with one neighbour, a dossier, a pinned prompt and a fake
/// terminal profile.
fn setup(tag: &str, min: usize) -> (PathBuf, EpochConfig) {
    let (d, v) = setup_value(tag, min);
    let c: EpochConfig = serde_json::from_value(v).unwrap();
    c.validate().unwrap();
    (d, c)
}

fn setup_value(tag: &str, min: usize) -> (PathBuf, Value) {
    let d = tmp(&format!("epoch-{tag}"));
    let paper = d.join("paper.md");
    std::fs::write(&paper, PAPER).unwrap();
    let near = d.join("near.md");
    std::fs::write(&near, NEAR).unwrap();
    let world = d.join("world");
    std::fs::create_dir_all(&world).unwrap();
    std::fs::write(world.join(format!("{DOC}.json")), json!({"documents":[{"doc":"near-2023-arrays","title":"Arrays of Heads","relation":"cites","distance":1,"licence":"CC BY 4.0","texts":[{"id":"lib:near-2023-arrays","kind":"original","path":near}]}]}).to_string()).unwrap();
    let prompt = d.join("prompt.md");
    std::fs::write(&prompt, PROMPT).unwrap();
    let v = json!({
        "schema": SCHEMA, "whale": "whale-t", "run_root": d.join("run"), "epoch": 1,
        "documents": {"from": "list", "items": [{"id": DOC, "path": paper, "sha256": file_hash(&paper).unwrap(), "title": "Zorbic Trees for Parsing", "licence": "CC BY 4.0"}]},
        "dossier": {"kind": "files", "root": d.join("dossier")},
        "world": {"kind": "fixture", "dir": world},
        "mova": {"words": {"min": min, "max": min.max(2000) + 1000}, "min_relevance": 1, "gates": {"structure_share_max": 0.6}},
        "sol": {"terminal": crate::terminal::tests::profile(&d, "whale-t"), "prompt": {"path": prompt, "sha256": file_hash(&prompt).unwrap()}, "words": {"min": 20, "max": 400}, "min_support": 0.3}
    });
    (d, v)
}

fn fake(answers: &[(u64, String)]) -> FakeTerminal {
    FakeTerminal { animal: "whale-t".into(), answers: answers.iter().map(|(e, a)| (format!("epoch:{e}:{DOC}"), a.clone())).collect(), limit_after: None, received: vec![] }
}

fn status_of(c: &EpochConfig, epoch: u64) -> Value {
    crate::util::load(&c.epoch_dir(epoch).join("docs").join(DOC).join("status.json"), 1 << 20).unwrap()
}

fn rows(c: &EpochConfig) -> Vec<TextRow> {
    c.dossier.open(Path::new("/nonexistent")).rows(DOC).unwrap()
}

#[test]
fn unknown_config_fields_are_refused() {
    let (d, v) = setup_value("fields", 20);
    assert!(serde_json::from_value::<EpochConfig>(v.clone()).is_ok());
    for (path, key) in [("", "surprise"), ("mova", "padding"), ("sol", "retry"), ("documents", "alphabetical")] {
        let mut bad = v.clone();
        let at = if path.is_empty() { &mut bad } else { &mut bad[path] };
        at[key] = json!(1);
        assert!(serde_json::from_value::<EpochConfig>(bad).is_err(), "{path}.{key} must be refused");
    }
    let c: EpochConfig = serde_json::from_value(v).unwrap();
    let mut bad = c.clone();
    bad.epoch = 0;
    assert!(bad.validate().is_err());
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn the_length_gate_keeps_a_short_text_out_and_asks_nothing() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("short", 4000);
    let mut t = fake(&[(1, answer(""))]);
    run(&c, 1, None, Some(&mut t)).unwrap();
    assert_eq!(status_of(&c, 1)["state"], "mova_short");
    assert!(t.received.is_empty(), "a short Mova text must not reach the consultant");
    assert!(rows(&c).iter().all(|r| r.kind == "original"));
    // Negative control: the same paper under a low minimum is registered and asked about.
    let (d2, c2) = setup("short-control", 20);
    let mut t2 = fake(&[(1, answer(""))]);
    run(&c2, 1, None, Some(&mut t2)).unwrap();
    assert_eq!(status_of(&c2, 1)["state"], "done");
    assert_eq!(t2.received.len(), 1);
    let _ = std::fs::remove_dir_all(d);
    let _ = std::fs::remove_dir_all(d2);
}

#[test]
fn an_epoch_registers_cited_texts_and_resume_never_asks_again() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("flow", 20);
    let mut t = fake(&[(1, answer(""))]);
    let s = run(&c, 1, None, Some(&mut t)).unwrap();
    assert_eq!(s["documents"][0]["state"], "done", "{s}");
    let rs = rows(&c);
    let kinds: Vec<&str> = rs.iter().map(|r| r.kind.as_str()).collect();
    assert_eq!(kinds, vec!["original", "mova", "sol"]);
    let mova = &rs[1];
    let text = c.dossier.open(Path::new("/nonexistent")).read(mova).unwrap();
    // Every sentence of Mova's text ends with its citations; the neighbour is cited by its id.
    assert!(text.contains("## References") && text.contains("`lib:near-2023-arrays`"), "{text}");
    assert!(mova.citations.count >= 2 && mova.citations.unresolved == 0);
    // The consultant got the sources by path, numbered, and its text is never public.
    let q = &t.received[0];
    assert!(q.text.contains("[1] original") && q.text.contains("[2] mova") && q.text.contains("/inputs/"), "{}", q.text);
    let sol = &rs[2];
    assert!(!sol.public_ok && sol.licence_class == "internal");
    assert_eq!(sol.inputs.len(), 3);
    let st = status_of(&c, 1);
    assert!(st["sol"]["claims"]["by_status"]["sourced"].as_u64().unwrap() >= 1, "{st}");
    // Resume: nothing is written or asked again.
    let mut again = fake(&[(1, answer(""))]);
    run(&c, 1, None, Some(&mut again)).unwrap();
    assert!(again.received.is_empty());
    assert_eq!(rows(&c).len(), 3);
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn a_citation_to_an_unknown_source_is_refused() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("unknown", 20);
    let mut t = fake(&[(1, answer("A survey covers zorbic trees too [9]."))]);
    run(&c, 1, None, Some(&mut t)).unwrap();
    let st = status_of(&c, 1);
    assert_eq!(st["state"], "sol_rejected");
    assert!(st["sol"]["refused"].as_str().unwrap().contains("[9]"));
    assert!(rows(&c).iter().all(|r| r.kind != "sol"));
    assert!(c.epoch_dir(1).join("docs").join(DOC).join("sol-answer.md").exists());
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn a_question_without_a_recorded_answer_is_never_sent_again() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("uncertain", 20);
    run(&c, 1, None, None).unwrap();
    assert_eq!(status_of(&c, 1)["state"], "mova");
    // A crash after the question went out: the status says `sol_asking`.
    let p = c.epoch_dir(1).join("docs").join(DOC).join("status.json");
    let mut st = status_of(&c, 1);
    st["state"] = json!("sol_asking");
    save(&p, &st).unwrap();
    let mut t = fake(&[(1, answer(""))]);
    run(&c, 1, None, Some(&mut t)).unwrap();
    assert!(t.received.is_empty());
    assert_eq!(status_of(&c, 1)["state"], "uncertain");
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn a_terminal_limit_stops_the_run() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("limit", 20);
    let mut t = fake(&[(1, answer(""))]);
    t.limit_after = Some(0);
    let s = run(&c, 1, None, Some(&mut t)).unwrap();
    assert_eq!(s["stopped"]["status"], "sol_limit");
    assert_eq!(t.received.len(), 1);
    // The stop is final for this document's epoch: a later run does not ask again.
    let mut t2 = fake(&[(1, answer(""))]);
    run(&c, 1, None, Some(&mut t2)).unwrap();
    assert!(t2.received.is_empty());
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn later_epochs_read_earlier_texts_and_saturate() {
    if !model_ok() {
        return;
    }
    let (d, c) = setup("epochs", 20);
    let first = answer("");
    let mut t = fake(&[(1, first.clone()), (2, first.clone()), (3, first.clone())]);
    run(&c, 1, None, Some(&mut t)).unwrap();
    run(&c, 2, None, Some(&mut t)).unwrap();
    // Epoch 2: Mova's text cites the consultant's text of epoch 1, which is also given to it.
    let draft = std::fs::read_to_string(c.epoch_dir(2).join("docs").join(DOC).join("mova.md")).unwrap();
    assert!(draft.contains(&format!("`{DOC}/1-sol-1`")), "{draft}");
    assert!(t.received[1].text.contains(&format!("`{DOC}/1-sol-1`")));
    // The same consultant text adds no new sourced claim and is not longer: saturated.
    assert_eq!(status_of(&c, 2)["state"], "saturated");
    run(&c, 3, None, Some(&mut t)).unwrap();
    assert_eq!(t.received.len(), 2, "a saturated document is not asked again");
    // Negative control: a longer text with a new sourced claim is not saturated.
    let (d2, c2) = setup("epochs-control", 20);
    let mut t2 = fake(&[(1, first.clone()), (2, answer("The array makes each head lookup a single step for the parser [3]."))]);
    run(&c2, 1, None, Some(&mut t2)).unwrap();
    run(&c2, 2, None, Some(&mut t2)).unwrap();
    assert_eq!(status_of(&c2, 2)["state"], "done");
    let _ = std::fs::remove_dir_all(d);
    let _ = std::fs::remove_dir_all(d2);
}

#[test]
fn two_runs_write_the_same_mova_text() {
    if !model_ok() {
        return;
    }
    let (d1, c1) = setup("det-a", 20);
    let (d2, c2) = setup("det-b", 20);
    run(&c1, 1, None, None).unwrap();
    run(&c2, 1, None, None).unwrap();
    let a = std::fs::read_to_string(c1.epoch_dir(1).join("docs").join(DOC).join("mova.md")).unwrap();
    let b = std::fs::read_to_string(c2.epoch_dir(1).join("docs").join(DOC).join("mova.md")).unwrap();
    assert_eq!(a, b);
    let _ = std::fs::remove_dir_all(d1);
    let _ = std::fs::remove_dir_all(d2);
}

/// A fake consultant for measurements (never a model): it reads the sources the prompt names by
/// path and answers with their sentences, in turn, each cited to its source, up to `words`.
struct ReadingFake {
    animal: String,
    words: usize,
    asked: usize,
}
impl PrivateTerminal for ReadingFake {
    fn animal(&self) -> &str {
        &self.animal
    }
    fn ask(&mut self, q: &Question) -> std::result::Result<crate::terminal::Answer, crate::terminal::AskError> {
        self.asked += 1;
        let mut srcs: Vec<(usize, String, Vec<String>)> = vec![];
        for l in q.text.lines() {
            let Some(rest) = l.strip_prefix('[') else { continue };
            let Some((n, rest)) = rest.split_once(']') else { continue };
            let (Ok(n), Some(path)) = (n.parse::<usize>(), rest.rsplit(" — ").next()) else { continue };
            let id = rest.split('`').nth(1).unwrap_or("").to_string();
            let text = std::fs::read_to_string(path.trim()).unwrap_or_default();
            let body = text.split("\n## References").next().unwrap_or("").replace('\n', " ");
            let sents: Vec<String> = world::quant::sentences(&body).into_iter().filter(|s| s.split_whitespace().count() >= 8 && !s.contains('[') && !s.contains('#')).collect();
            srcs.push((n, id, sents));
        }
        let mut out = String::from("# Article\n\n");
        let (mut w, mut i) = (0, 0);
        while w < self.words && srcs.iter().any(|s| i < s.2.len()) {
            for (n, _, s) in &srcs {
                if let Some(x) = s.get(i) {
                    let x = x.trim_end_matches('.');
                    out.push_str(&format!("{x} [{n}]. "));
                    w += x.split_whitespace().count() + 1;
                }
            }
            i += 1;
        }
        out.push_str("\n\n## References\n\n");
        for (n, id, _) in &srcs {
            out.push_str(&format!("[{n}] `{id}`\n"));
        }
        Ok(crate::terminal::Answer { text: out, elapsed_ms: 0, terminal_id: None, missing: vec![] })
    }
}

/// `WHALE_EPOCH_MEASURE=CONFIG cargo test --release -- --ignored measure_epoch`: one epoch of a
/// real configuration with the reading fake as the consultant; prints the summary.
#[test]
#[ignore]
fn measure_epoch() {
    let Some(p) = std::env::var_os("WHALE_EPOCH_MEASURE") else { return };
    let c = EpochConfig::load(Path::new(&p)).unwrap();
    let mut t = ReadingFake { animal: c.whale.clone(), words: 4500, asked: 0 };
    let started = std::time::Instant::now();
    let s = run(&c, c.epoch, None, Some(&mut t)).unwrap();
    println!("MEASURE {} asked={} ms={}", s, t.asked, started.elapsed().as_millis());
}

/// A fake world tool (a shell script, never the real store): `dossier list|text|add` over a
/// directory of JSONL rows and files, `world neighbourhood|related` from fixed answers.
fn fake_world(d: &Path) -> PathBuf {
    let st = d.join("fake-world");
    std::fs::create_dir_all(st.join("files")).unwrap();
    let meta = json!({"doc": DOC, "docs": [
        {"doc": DOC, "distance": 0, "title": "Zorbic Trees for Parsing", "texts": [{"text": format!("{DOC}/0-original-1"), "kind": "original", "epoch": 0}, {"text": format!("{DOC}/0-site-old-1"), "kind": "site-old", "epoch": 0}]},
        {"doc": "near-2023-arrays", "distance": 1, "title": "Arrays of Heads", "texts": [{"text": "lib:near-2023-arrays", "kind": "neighbour", "epoch": 0}]}
    ]});
    std::fs::write(st.join("meta.json"), meta.to_string()).unwrap();
    std::fs::write(st.join("related.json"), json!({"related": [{"doc": "near-2023-arrays", "edges": [{"kind": "cites", "weight": 1.0}]}]}).to_string()).unwrap();
    let mut content = String::new();
    for s in ["Zorbic trees were described on an older site as arrays of heads for dependency parsing.", "The older site said that a zorbic rule attaches each word to its head."] {
        content.push_str(&json!({"doc": DOC, "text": format!("{DOC}/0-site-old-1"), "sentence": s}).to_string());
        content.push('\n');
    }
    for s in ["Zorbic trees keep every head of a sentence in one array for dependency parsing.", "Earlier parsers stored the heads of a sentence in linked lists.", "The array makes each head lookup a single step for the parser."] {
        content.push_str(&json!({"doc": "near-2023-arrays", "text": "lib:near-2023-arrays", "sentence": s}).to_string());
        content.push('\n');
    }
    std::fs::write(st.join("content.jsonl"), content).unwrap();
    let exe = d.join("fake-world.sh");
    let script = r#"#!/bin/bash
S=__ST__
case "$1 $2" in
"dossier list") f=$S/$4.jsonl; if [ -s $f ]; then echo "[$(paste -sd, $f)]"; else echo "[]"; fi ;;
"dossier text") cat "$S/files/${4//\//_}" ;;
"dossier add") doc=$4; shift 4; inputs=""
  while [ $# -gt 0 ]; do case $1 in --epoch) e=$2;; --kind) k=$2;; --file) f=$2;; --source) src=$2;; --licence) l=$2;; --input) inputs="$inputs\"$2\",";; esac; shift 2; done
  n=$(( $(grep -c "\"epoch\":$e,\"kind\":\"$k\"" $S/$doc.jsonl 2>/dev/null || true) + 1 )); id="$doc/$e-$k-$n"
  sha=$(sha256sum "$f" | cut -d' ' -f1); cp "$f" "$S/files/${id//\//_}"
  row="{\"id\":\"$id\",\"doc\":\"$doc\",\"epoch\":$e,\"kind\":\"$k\",\"source\":\"x\",\"licence\":\"\",\"licence_class\":\"ours\",\"public_ok\":false,\"sha256\":\"$sha\",\"words\":1,\"created_unix\":0,\"inputs\":[${inputs%,}],\"citations\":{\"count\":0,\"unresolved\":0},\"file\":\"texts/x.md\"}"
  echo "$row" >> $S/$doc.jsonl; echo "$row" ;;
"world neighbourhood") if [ "${@: -1}" = "--content" ]; then cat $S/content.jsonl; else cat $S/meta.json; fi ;;
"world related") cat $S/related.json ;;
*) echo "fake world: $*" >&2; exit 2 ;;
esac
"#;
    std::fs::write(&exe, script.replace("__ST__", &st.display().to_string())).unwrap();
    let mut p = std::fs::metadata(&exe).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut p, 0o755);
    std::fs::set_permissions(&exe, p).unwrap();
    exe
}

#[test]
fn the_world_tool_gives_dossier_and_neighbourhood() {
    if !model_ok() {
        return;
    }
    let _s = crate::serial();
    let (d, v) = setup_value("world-cli", 20);
    let exe = fake_world(&d);
    let cfg = d.join("world.json");
    std::fs::write(&cfg, "{}").unwrap();
    let mut v = v;
    v["dossier"] = json!({"kind": "world", "executable": exe, "config": cfg});
    v["world"] = json!({"kind": "world", "executable": exe, "config": cfg, "radius": 1, "max_docs": 5});
    let c: EpochConfig = serde_json::from_value(v).unwrap();
    let mut t = fake(&[(1, answer(""))]);
    let s = run(&c, 1, None, Some(&mut t)).unwrap();
    assert_eq!(s["documents"][0]["state"], "done", "{s}");
    let rs = rows(&c);
    assert_eq!(rs.iter().map(|r| r.kind.as_str()).collect::<Vec<_>>(), vec!["original", "mova", "sol"]);
    // Mova cites the neighbour's text and the document's older description as the world gave them.
    let mova = c.dossier.open(Path::new("/nonexistent")).read(&rs[1]).unwrap();
    assert!(mova.contains("`lib:near-2023-arrays`") && mova.contains(&format!("`{DOC}/0-site-old-1`")), "{mova}");
    // The consultant gets the original, Mova's text and the neighbour's text as staged files.
    let q = &t.received[0].text;
    assert!(q.contains("[3] neighbour neighbour — `lib:near-2023-arrays`") && q.contains("/inputs/"), "{q}");
    assert_eq!(rs[2].inputs.len(), 3);
    let _ = std::fs::remove_dir_all(d);
}
