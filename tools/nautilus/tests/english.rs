//! Nautilus in English, end to end, with Mova's native model (set MOVA_OCTOPUS_TEST_MODEL to an en.bin; without it
//! these tests report a skip): the prompts select the configured capabilities, the refusals stay refusals, and a
//! draft and a count are answered in English from real work on a one-lesson fixture course.
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
    let d = std::env::temp_dir().join(format!("nautilus-en-{name}-{}-{t}", std::process::id()));
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

fn config(model: &Path, root: &Path) -> nautilus::animal::Config {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = here.join("../..");
    let base = repo.join("world/data/dictionary-improved.tsv");
    serde_json::from_value(json!({"layers":{
        "commands":{"source":"commands","out":root.join("commands.tsv"),"base":base,"course":here.join("tests/data/course"),
            "model":model,"top":20,"licence":"test","name":"the commands dictionary"},
        "wikipedia":{"source":"wikipedia","out":root.join("absent/wikipedia.tsv"),"base":base,"corpus":root.join("no-corpus"),
            "top":20,"licence":"test","name":"the Wikipedia dictionary"}}}))
    .unwrap()
}

/// (prompt, capabilities)
const POSITIVE: [(&str, &[&str]); 7] = [
    // A sentence-initial "Draft": a verb in the 0.5 parse; a noun reading is repaired to the imperative.
    ("Draft the Wikipedia dictionary.", &["wikipedia-draft"]),
    ("Make the commands dictionary draft.", &["commands-draft"]),
    ("Please build the commands dictionary.", &["commands-draft"]),
    ("How many entries does the commands dictionary have?", &["commands-stats"]),
    ("How many entries does the Wikipedia dictionary have?", &["wikipedia-stats"]),
    ("Build the Wikipedia dictionary draft.", &["wikipedia-draft"]),
    ("Make the commands dictionary draft. Then tell me how many entries the commands dictionary has.", &["commands-draft", "commands-stats"]),
];

/// (prompt, failure stage)
const REFUSED: [(&str, &str); 6] = [
    ("Do not make the commands dictionary draft.", "pragmatic_ambiguity"),
    ("Delete the commands dictionary.", "unsupported_capability"),
    ("Make the dictionary draft.", "missing_context"),
    ("Draft the dictionary.", "missing_context"),
    ("Do not draft the Wikipedia dictionary.", "pragmatic_ambiguity"),
    ("If you can, make the commands dictionary draft.", "pragmatic_ambiguity"),
];

/// Questions about drafting: refused, never planned as a draft; only the stats capabilities
/// (`observes`) answer questions.
const QUESTIONS_ABOUT_ACTIONS: [&str; 4] = [
    "Is the Wikipedia dictionary drafted?",
    "Has the Wikipedia dictionary been built?",
    "Is the commands dictionary draft ready?",
    "Did you make the commands dictionary draft?",
];

#[test]
fn english_prompts_select_capabilities() {
    let Some(model) = model() else { return };
    let root = tmp("plan");
    let dc = deputy_config(&model, &root);
    nautilus::animal::check_capabilities(&dc, &config(&model, &root)).unwrap();
    let mut lying = dc.clone();
    lying.capabilities.get_mut("commands-draft").unwrap().intent.observes = true;
    assert!(nautilus::animal::check_capabilities(&lying, &config(&model, &root)).is_err(), "a draft acts; it cannot answer questions");
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
        assert!(r.failure.is_some() && !r.plan.iter().any(|a| a.capability.ends_with("-draft")), "{p}: {:?}", r.plan);
        assert_eq!(r.followups.len(), 1, "{p}: a refusal is a review candidate");
    }
}

#[test]
fn english_answers_from_real_work() {
    let Some(model) = model() else { return };
    let root = tmp("run");
    let c = config(&model, &root);
    let d = Deputy::load(deputy_config(&model, &root)).unwrap();
    let a = nautilus::animal::answer(&d, &c, "q1", "How many entries does the commands dictionary have?").unwrap();
    assert_eq!(a["answer"], "I have not made the commands dictionary yet.");
    let a = nautilus::animal::answer(&d, &c, "q2", "Make the commands dictionary draft.").unwrap();
    let text = a["answer"].as_str().unwrap();
    assert!(text.starts_with("I have made the commands dictionary as a draft: 20 entries with "), "{text}");
    assert!(text.ends_with(", from 1 lesson and 0 prompts. It is not reviewed yet, and no teacher wrote any of it."), "{text}");
    let a = nautilus::animal::answer(&d, &c, "q3", "How many entries does the commands dictionary have?").unwrap();
    assert!(a["answer"].as_str().unwrap().starts_with("The commands dictionary has 20 entries, and it is still a draft."), "{a}");
    // the layer is the school layers' format, marked as a draft
    let layer = std::fs::read_to_string(root.join("commands.tsv")).unwrap();
    assert!(layer.starts_with(nautilus::layer::HEAD));
    assert_eq!(nautilus::layer::stats(&layer).entries, 20);
    // a refusal is answered, nothing is made
    let a = nautilus::animal::answer(&d, &c, "q4", "Delete the commands dictionary.").unwrap();
    assert!(a["answer"].as_str().unwrap().starts_with("I did not act on this request"), "{a}");
    // the Wikipedia corpus is absent here: the draft fails in English and writes nothing
    let a = nautilus::animal::answer(&d, &c, "q5", "Build the Wikipedia dictionary draft.").unwrap();
    assert!(a["answer"].as_str().unwrap().starts_with("I could not finish this request"), "{a}");
    assert!(!root.join("absent/wikipedia.tsv").exists());
}
