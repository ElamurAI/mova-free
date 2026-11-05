//! Sea Turtle in English, end to end, with Mova's native model (set MOVA_OCTOPUS_TEST_MODEL to an en.bin; without
//! it these tests report a skip): the prompts select the configured capabilities, the refusals stay refusals, and
//! the answers come from real work — the command course as it is in the repository, and the Turtle's journal.
use mova_octopus::{Config as DeputyConfig, Deputy};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn model() -> Option<PathBuf> {
    match std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") {
        Some(p) => Some(PathBuf::from(p)),
        None => {
            eprintln!("SKIPPED: MOVA_OCTOPUS_TEST_MODEL not configured");
            None
        }
    }
}

fn tmp(name: &str) -> PathBuf {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let d = std::env::temp_dir().join(format!("turtle-en-{name}-{}-{t}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn deputy_config(model: &Path, root: &Path) -> DeputyConfig {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("deputy.example.json")).unwrap();
    let mut v: Value = serde_json::from_str(&text).unwrap();
    v["model"] = json!(model);
    v["model_sha256"] = json!(mova_octopus::digest(&std::fs::read(model).unwrap()));
    v["ledger"] = json!(root.join("ledger"));
    v["followup_queue"] = json!(root.join("review"));
    serde_json::from_value(v).unwrap()
}

fn config(root: &Path) -> turtle::animal::Config {
    serde_json::from_value(json!({"journal":root.join("made.jsonl"),"drafts":root.join("drafts"),"layers":{}})).unwrap()
}

const POSITIVE: [(&str, &[&str]); 7] = [
    // The 0.4 parse read «commands» here as a verb (refused then); the 0.5 parse (2483516a) reads it as
    // the object noun, which names the commands course.
    ("Check the commands course.", &["commands-check"]),
    ("Check the command course.", &["commands-check"]),
    ("Please check the command course.", &["commands-check"]),
    ("Could you check the command course?", &["commands-check"]),
    ("Check the commands lessons.", &["commands-check"]),
    ("How many questions did you make?", &["made"]),
    ("How many questions have you made?", &["made"]),
];

const REFUSED: [(&str, &str); 5] = [
    ("Do not check the command course.", "pragmatic_ambiguity"),
    ("Delete the command course.", "unsupported_capability"),
    ("Do not check the commands course.", "pragmatic_ambiguity"),
    ("Delete the commands course.", "unsupported_capability"),
    ("Check the course.", "missing_context"),
];

/// Questions about checking: refused, never planned as a check; only `made` (`observes`) answers questions.
const QUESTIONS_ABOUT_ACTIONS: [&str; 3] = ["Is the command course checked?", "Did you check the command course?", "Has the command course been checked?"];

#[test]
fn english_prompts_select_capabilities() {
    let Some(model) = model() else { return };
    let root = tmp("plan");
    let dc = deputy_config(&model, &root);
    turtle::animal::check_capabilities(&dc, &config(&root)).unwrap();
    let mut lying = dc.clone();
    lying.capabilities.get_mut("commands-check").unwrap().intent.observes = true;
    assert!(turtle::animal::check_capabilities(&lying, &config(&root)).is_err(), "a check acts; it cannot answer questions");
    let d = Deputy::load(dc).unwrap();
    for (p, caps) in POSITIVE {
        let r = d.review(p);
        assert!(r.failure.is_none(), "{p}: {:?}", r.failure);
        let got: Vec<&str> = r.plan.iter().map(|a| a.capability.as_str()).collect();
        assert_eq!(got, caps, "{p}");
    }
    for (p, stage) in REFUSED {
        let r = d.review(p);
        let f = r.failure.as_ref().unwrap_or_else(|| panic!("accepted: {p} -> {:?}", r.plan));
        assert_eq!(f.stage, stage, "{p}: {}", f.reason);
        assert_eq!(r.followups.len(), 1, "{p}: a refusal is a review candidate");
    }
    for p in QUESTIONS_ABOUT_ACTIONS {
        let r = d.review(p);
        assert!(r.failure.is_some() && !r.plan.iter().any(|a| a.capability == "commands-check"), "{p}: {:?}", r.plan);
        assert_eq!(r.followups.len(), 1, "{p}: a refusal is a review candidate");
    }
}

#[test]
fn english_answers_from_real_work() {
    let Some(model) = model() else { return };
    let root = tmp("run");
    let c = config(&root);
    let d = Deputy::load(deputy_config(&model, &root)).unwrap();
    let a = turtle::animal::answer(&d, &c, "q1", "Check the command course.").unwrap();
    let text = a["answer"].as_str().unwrap();
    assert!(text.starts_with("I checked the commands course: 30 lessons with "), "{text}");
    assert!(text.ends_with("Every lesson keeps the school's rules and its right answers are marked at varied places."), "{text}");
    let a = turtle::animal::answer(&d, &c, "q2", "How many questions did you make?").unwrap();
    assert_eq!(a["answer"], "I have not made any questions yet.");
    // make a set of gap questions from a small layer, then ask again
    let layer = root.join("layer.tsv");
    let rows: String = ["pass", "shut", "fetch", "grab", "hold", "carry", "stop", "wait", "check", "bring", "drop", "lift"]
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let examples = vec![format!("Please {w} it now, {i} times. «t:{i}»"), format!("Did you {w} the bag {i} times? «u:{i}»"), format!("They {w} things {i} times a day. «v:{i}»")];
            nautilus::layer::Row { word: w.to_string(), pos: "verb".into(), meaning: String::new(), sense: "Counted.".into(), examples, note: String::new() }.tsv() + "\n"
        })
        .collect();
    std::fs::write(&layer, rows).unwrap();
    let v = turtle::make::make_gap_course(&layer, "t", "a test", &root.join("drafts/t"), &c.journal, 1, &model).unwrap();
    assert_eq!(v["lessons"], 1, "{v}");
    let q = v["questions"].as_u64().unwrap();
    assert!(q >= 20, "{v}");
    let a = turtle::animal::answer(&d, &c, "q3", "How many questions did you make?").unwrap();
    assert_eq!(a["answer"], format!("I have made {q} questions in 1 lesson, in 1 set of drafts."));
    let a = turtle::animal::answer(&d, &c, "q4", "Delete the command course.").unwrap();
    assert!(a["answer"].as_str().unwrap().starts_with("I did not act on this request"), "{a}");
}
