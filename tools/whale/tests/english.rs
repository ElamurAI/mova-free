//! The Whales in English, end to end, with Mova's native model (set MOVA_OCTOPUS_TEST_MODEL to an
//! en.bin; without it these tests report a skip). The served test runs the real `mova-whale`
//! child for every admitted request against a fake Codex executable (`codex` in the service
//! config): no model is ever called. The fixture specification uses the deployment's word bounds
//! (4,000..5,000 words).
use mova_octopus::{Config as DeputyConfig, Deputy};
use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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
    let d = std::env::temp_dir().join(format!("whale-en-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    fs::set_permissions(&d, fs::Permissions::from_mode(0o700)).unwrap();
    d
}

fn sha(p: &Path) -> String {
    mova_octopus::digest(&fs::read(p).unwrap())
}

fn deputy_config(model: &Path, root: &Path) -> DeputyConfig {
    let text = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("deputy.example.json")).unwrap();
    let mut v: Value = serde_json::from_str(&text).unwrap();
    v["model"] = json!(model);
    v["model_sha256"] = json!(sha(model));
    v["ledger"] = json!(root.join("ledger"));
    v["followup_queue"] = json!(root.join("review"));
    serde_json::from_value(v).unwrap()
}

/// (prompt, capabilities, arguments of the first action)
const POSITIVE: [(&str, &[&str], &str); 20] = [
    // A colon hands the identifier to the request whatever relation the parse gives it (0.5: `vocative`).
    ("Survey this paper: paper-2023-gamma.", &["describe"], r#"{"sources":["paper-2023-gamma"]}"#),
    // A sentence-initial "Survey" read as a noun is the imperative a configured request predicate names.
    ("Survey paper-2023-gamma.", &["describe"], r#"{"sources":["paper-2023-gamma"]}"#),
    ("Describe this paper: paper-2024-alpha.", &["describe"], r#"{"sources":["paper-2024-alpha"]}"#),
    ("Describe this paper: https://papers.example.org/beta.", &["describe"], r#"{"sources":["https://papers.example.org/beta"]}"#),
    ("Please survey paper-2023-gamma.", &["describe"], r#"{"sources":["paper-2023-gamma"]}"#),
    ("Please describe paper-2024-alpha and paper-2023-gamma.", &["describe"], r#"{"sources":["paper-2024-alpha","paper-2023-gamma"]}"#),
    ("Could you summarize paper-2024-alpha?", &["describe"], r#"{"sources":["paper-2024-alpha"]}"#),
    ("How many papers have you described?", &["count"], "{}"),
    ("How many sources have you surveyed?", &["count"], "{}"),
    ("What is the status of paper-2024-alpha?", &["status"], r#"{"source":["paper-2024-alpha"]}"#),
    ("What is the status of the Whales?", &["status"], "{}"),
    ("Are you busy?", &["status"], "{}"),
    ("Show the description of paper-2024-alpha.", &["show"], r#"{"source":["paper-2024-alpha"]}"#),
    ("Show me the summary of https://papers.example.org/beta.", &["show"], r#"{"source":["https://papers.example.org/beta"]}"#),
    ("Tell me the status of paper-2023-gamma.", &["status"], r#"{"source":["paper-2023-gamma"]}"#),
    ("What is the status of https://papers.example.org/beta?", &["status"], r#"{"source":["https://papers.example.org/beta"]}"#),
    ("Pause the Whales.", &["pause"], "{}"),
    ("Resume the Whales.", &["resume"], "{}"),
    ("Please pause.", &["pause"], "{}"),
    ("Describe paper-2024-alpha. Then tell me the status.", &["describe", "status"], r#"{"sources":["paper-2024-alpha"]}"#),
];

/// (prompt, failure stage)
const REFUSED: [(&str, &str); 13] = [
    ("Do not describe paper-2024-alpha.", "pragmatic_ambiguity"),
    ("If you can, describe paper-2024-alpha.", "pragmatic_ambiguity"),
    ("Describe the paper.", "argument_count"),
    ("Show the description.", "argument_count"),
    ("Delete paper-2024-alpha.", "unsupported_capability"),
    ("Stop the Whales.", "unsupported_capability"),
    ("Show the descriptions of paper-2024-alpha and paper-2023-gamma.", "argument_count"),
    ("Describe paper-2024-alpha because paper-2023-gamma has a copy.", "argument_unbound"),
    ("Survey the paper.", "argument_count"),
    ("Do not survey this paper: paper-2024-alpha.", "pragmatic_ambiguity"),
    ("Delete this paper: paper-2024-alpha.", "unsupported_capability"),
    ("Describe this paper: paper-2024-alpha, not paper-2023-gamma.", "pragmatic_ambiguity"),
    ("The Whales described paper-2024-alpha.", "pragmatic_ambiguity"),
];

/// Questions about acting operations (describe, pause, resume): refused, never planned as that action;
/// only observing capabilities (count, status, show) answer questions.
const QUESTIONS_ABOUT_ACTIONS: [&str; 6] = [
    "Is paper-2024-alpha described?",
    "Did you describe paper-2024-alpha?",
    "Are the Whales paused?",
    "Is the Whale paused?",
    "How do I pause the Whales?",
    "Have the Whales resumed?",
];

#[test]
fn prompts_map_to_capabilities_and_refusals_queue_nothing() {
    let Some(model) = model() else { return };
    let root = tmp("plan");
    let mut lying = deputy_config(&model, &root);
    mova_whale::animal::check_capabilities(&lying).unwrap();
    lying.capabilities.get_mut("describe").unwrap().intent.observes = true;
    assert!(mova_whale::animal::check_capabilities(&lying).is_err(), "a description acts; it cannot answer questions");
    let deputy = Deputy::load(deputy_config(&model, &root)).unwrap();
    let mut wrong = vec![];
    for (prompt, caps, args) in POSITIVE {
        let r = deputy.review(prompt);
        let got: Vec<&str> = r.plan.iter().map(|a| a.capability.as_str()).collect();
        let first = r.plan.first().map(|a| serde_json::to_value(&a.arguments).unwrap()).unwrap_or(json!(null));
        if r.failure.is_some() || got != caps || first != serde_json::from_str::<Value>(args).unwrap() {
            wrong.push(format!("{prompt}: {got:?} {first} {:?}", r.failure.as_ref().map(|f| (&f.stage, &f.reason))));
        }
    }
    for (prompt, stage) in REFUSED {
        let r = deputy.review(prompt);
        match &r.failure {
            Some(f) if f.stage == stage && !r.followups.is_empty() => {}
            other => wrong.push(format!("refusal {prompt}: {:?} plan {:?}", other.as_ref().map(|f| (&f.stage, &f.reason)), r.plan.iter().map(|a| &a.capability).collect::<Vec<_>>())),
        }
    }
    for prompt in QUESTIONS_ABOUT_ACTIONS {
        let r = deputy.review(prompt);
        let acted = r.plan.iter().any(|a| ["describe", "pause", "resume"].contains(&a.capability.as_str()));
        if r.failure.is_none() || acted || r.followups.is_empty() {
            wrong.push(format!("question {prompt}: {:?} plan {:?}", r.failure.as_ref().map(|f| &f.stage), r.plan.iter().map(|a| &a.capability).collect::<Vec<_>>()));
        }
    }
    let _ = fs::remove_dir_all(&root);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Codex JSONL events whose answer is the JSON body with a description of `words` words.
fn reply(desc: &str) -> String {
    let body = json!({"schema":1,"ideas":["Parse trees can be compared node by node."],"description":desc});
    format!(
        "{}\n{}\n",
        json!({"type":"item.completed","item":{"type":"agent_message","text":body.to_string()}}),
        json!({"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":20}})
    )
}

struct Served {
    dir: PathBuf,
    socket: PathBuf,
    thread: Option<std::thread::JoinHandle<anyhow::Result<()>>>,
}
impl Served {
    fn ask(&self, id: &str, prompt: &str, urgent: bool) -> (String, Value) {
        let v = mova_whale::animal::ask(&self.socket, id, prompt, urgent).unwrap();
        (v["report"]["answer"].as_str().unwrap_or("").to_string(), v)
    }
    fn journal(&self, event: &str) -> Vec<String> {
        fs::read_to_string(self.dir.join("run/requests.jsonl"))
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|v| v["event"] == event)
            .map(|v| v["job"].as_str().unwrap_or("").to_string())
            .collect()
    }
    fn finished(&self) -> usize {
        self.journal("finished").len()
    }
}
impl Drop for Served {
    fn drop(&mut self) {
        let _ = fs::write(self.socket.parent().unwrap().join("STOP"), b"");
        let _ = fs::write(self.dir.join("go"), b"");
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn serve(model: &Path) -> Served {
    let dir = tmp("served");
    // The specification: one `paper` kind from a pinned export, a 4,000..5,000-word description.
    let template = dir.join("template.txt");
    fs::write(&template, "Describe the source as one JSON object.\nMetadata: {{METADATA_JSON}}\nBEGIN SOURCE\n{{SOURCE_TEXT}}\nEND SOURCE\n").unwrap();
    let spec = json!({"schema":"mova-shark-spec-v1","id":"fixture-survey-words","prompt_template":{"path":"template.txt","sha256":sha(&template)},
        "prompt_metadata_fields":["id","title"],
        "kinds":{"paper":{"formats":{"markdown":"text"},"metadata":"export","export_path_field":"text_path"}},
        "output":{"max_reply_bytes":256000,"verbatim_run_tokens":12,
            "length":{"pointer":"/description","unit":"words","min":4000,"max":5000,"profile":"words_4000_5000_v1","forbid":["```"],"project_oversized":true},
            "fields":[{"kind":"equals","pointer":"/schema","value":1},{"kind":"string_list","pointer":"/ideas","min":1,"max":8}]},
        "originality":{"enabled":false},
        "bundle":{"store_prefix":"whale","manifest_kind":"fixture_survey","manifest_status":"generated_unverified","id_prefix":"paper-",
            "text_artifacts":[{"path":"description.md","pointer":"/description"}],"list_artifacts":[{"path":"ideas.md","pointer":"/ideas"}],
            "metadata_artifact":"metadata.json","metadata_fields":["id","title"]}});
    let spec_path = dir.join("spec.json");
    fs::write(&spec_path, serde_json::to_vec_pretty(&spec).unwrap()).unwrap();
    let mut rows = String::new();
    for (id, title) in [("paper-2024-alpha", "Alpha Trees"), ("paper-2022-beta", "Beta Frames"), ("paper-2023-gamma", "Gamma Worlds"), ("paper-2021-delta", "Delta")] {
        let p = dir.join(format!("{id}.md"));
        fs::write(&p, format!("# {title}\n\nThis fixture source about {title} has a few sentences only.\n")).unwrap();
        let tail = id.rsplit('-').next().unwrap();
        rows.push_str(&format!("{}\n", json!({"id":id,"title":title,"text_path":p,"url":format!("https://papers.example.org/{tail}")})));
    }
    let catalog = dir.join("catalog.jsonl");
    fs::write(&catalog, rows).unwrap();
    // Replies: alpha 4,500 words (completed), beta 3,999 (quarantined), gamma 5,001 without a
    // sentence end inside the bounds (quarantined, not projected).
    let sentence = "The paper studies parse trees carefully. ";
    fs::write(dir.join("alpha.jsonl"), reply(&sentence.repeat(750))).unwrap();
    fs::write(dir.join("beta.jsonl"), reply(&format!("{}{}", sentence.repeat(666), "trees ".repeat(3)))).unwrap();
    fs::write(dir.join("gamma.jsonl"), reply(&"tree ".repeat(5001))).unwrap();
    let codex = dir.join("fake-codex");
    fs::write(
        &codex,
        format!(
            "#!/bin/sh\nin=$(cat)\nd={}\nfor id in alpha beta gamma; do\n  case \"$in\" in *paper-20??-$id*)\n    echo $id >> $d/calls.log\n    if [ $id = alpha ]; then while [ ! -f $d/go ]; do sleep 0.05; done; fi\n    cat $d/$id.jsonl; exit 0;;\n  esac\ndone\nexit 3\n",
            dir.display()
        ),
    )
    .unwrap();
    fs::set_permissions(&codex, fs::Permissions::from_mode(0o700)).unwrap();
    let exe = PathBuf::from(env!("CARGO_BIN_EXE_mova-whale"));
    let cfg = json!({"run_root":dir.join("run"),"executable":{"path":exe,"sha256":sha(&exe)},"codex":codex,
        "survey":{"spec":{"path":spec_path,"sha256":sha(&spec_path)},"teacher":{"model":"gpt-fixture","effort":"medium"},
            "store":dir.join("store"),"metadata_export":{"path":catalog,"sha256":sha(&catalog)},"storage_floor_bytes":1},
        "catalog":{"path":catalog,"sha256":sha(&catalog),"kind":"paper","path_field":"text_path","title_field":"title",
            "formats":{".md":"markdown"},"url_fields":["url"]},
        "workers":1,"max_waiting":16,"noun":"paper"});
    let cfg_path = dir.join("animal.json");
    fs::write(&cfg_path, serde_json::to_vec_pretty(&cfg).unwrap()).unwrap();
    let deputy_path = dir.join("deputy.json");
    fs::write(&deputy_path, serde_json::to_vec(&deputy_config(model, &dir)).unwrap()).unwrap();
    let socket = dir.join("sock/whale.sock");
    let (c, d, s) = (cfg_path.clone(), deputy_path.clone(), socket.clone());
    let thread = std::thread::spawn(move || mova_whale::animal::serve(&c, &d, &s));
    let t = Instant::now();
    while !socket.exists() {
        assert!(t.elapsed() < Duration::from_secs(120), "the service did not start");
        std::thread::sleep(Duration::from_millis(50));
    }
    Served { dir, socket, thread: Some(thread) }
}

fn wait(what: &str, f: impl Fn() -> bool) {
    let t = Instant::now();
    while !f() {
        assert!(t.elapsed() < Duration::from_secs(60), "timed out: {what}");
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn served_whales_answer_in_english_while_work_runs() {
    let Some(model) = model() else { return };
    let s = serve(&model);
    let (a, _) = s.ask("r1", "Describe this paper: paper-2024-alpha.", false);
    assert_eq!(a, "I have queued 1 paper for description (octopus-r1-0): paper-2024-alpha (“Alpha Trees”). I am starting on it now.");
    wait("alpha to call the teacher", || fs::read_to_string(s.dir.join("calls.log")).is_ok_and(|c| c.contains("alpha")));
    // A question is answered while the description runs.
    let t = Instant::now();
    let (a, _) = s.ask("r2", "What is the status of paper-2024-alpha?", false);
    assert!(t.elapsed() < Duration::from_millis(1500), "{:?}", t.elapsed());
    assert!(a.starts_with("I am describing paper-2024-alpha (“Alpha Trees”) now (since "), "{a}");
    // Refusals queue nothing and keep a review candidate.
    for (id, p) in [("n1", "Do not describe paper-2023-gamma."), ("n2", "If you can, describe paper-2021-delta."), ("n3", "Stop the Whales.")] {
        let (a, v) = s.ask(id, p, false);
        assert!(a.starts_with("I did not act on this request because"), "{a}");
        assert!(a.contains("I kept it for review as candidate"), "{a}");
        assert!(v["report"]["report"]["followups"].as_array().is_some_and(|f| !f.is_empty()) || v.to_string().contains("candidate_id"));
    }
    assert_eq!(s.journal("queued").len(), 1, "a refused prompt queued work");
    // Priority orders waiting work only: gamma (normal) then beta (urgent) wait behind alpha.
    let (a, _) = s.ask("r3", "Survey this paper: paper-2023-gamma.", false);
    assert!(a.ends_with("It starts when a running description is done."), "{a}");
    let (a, _) = s.ask("r4", "Describe this paper: https://papers.example.org/beta.", true);
    assert!(a.contains("paper-2022-beta (“Beta Frames”)") && a.ends_with("It starts when a running description is done."), "{a}");
    let (a, _) = s.ask("r5", "What is the status of paper-2023-gamma?", false);
    assert_eq!(a, "Paper-2023-gamma (“Gamma Worlds”) is waiting; 1 request is ahead of it.");
    assert_eq!(s.journal("started"), ["octopus-r1-0"], "the running request was preempted");
    fs::write(s.dir.join("go"), b"").unwrap();
    wait("three finished requests", || s.finished() == 3);
    assert_eq!(s.journal("started"), ["octopus-r1-0", "octopus-r4-0", "octopus-r3-0"]);
    assert_eq!(fs::read_to_string(s.dir.join("calls.log")).unwrap(), "alpha\nbeta\ngamma\n");
    let (a, _) = s.ask("r6", "How many papers have you described?", false);
    assert_eq!(a, "I have described 1 paper. 2 of my attempts here did not produce a description and wait for review.");
    let (a, _) = s.ask("r7", "What is the status of paper-2022-beta?", false);
    assert!(a.contains("because the teacher's reply failed my checks (length gate: /description must contain 4000..5000 words"), "{a}");
    let (a, _) = s.ask("r8", "What is the status of paper-2023-gamma?", false);
    assert!(a.contains("length gate: oversized field has no whole paragraph/sentence boundary within 4000..5000"), "{a}");
    let (a, _) = s.ask("r9", "Show the description of paper-2024-alpha.", false);
    assert!(a.starts_with("My description of paper-2024-alpha (“Alpha Trees”) has 4,500 words; I wrote it at "), "{a}");
    assert!(a.contains("It begins: “The paper studies parse trees carefully."), "{a}");
    assert!(a.contains("/description.md.") && a.contains("The bundle is in "), "{a}");
    let (a, _) = s.ask("r10", "Describe paper-2024-alpha.", false);
    assert_eq!(a, "I have already described paper-2024-alpha (“Alpha Trees”) (4,500 words); ask me to show it.");
    let (a, _) = s.ask("r11", "Pause the Whales.", false);
    assert_eq!(a, "I have paused the Whales: no new description starts.");
    let (a, _) = s.ask("r12", "Please survey paper-2021-delta.", false);
    assert!(a.contains("The Whales are paused"), "{a}");
    std::thread::sleep(Duration::from_millis(800));
    assert_eq!(s.journal("started").len(), 3, "paused service admitted work");
    // A repeated request id returns the recorded answer, never a new job.
    let (again, _) = s.ask("r12", "Please survey paper-2021-delta.", false);
    assert_eq!(again, a);
    assert_eq!(s.journal("queued").len(), 4);
    let (a, _) = s.ask("r13", "Resume the Whales.", false);
    assert_eq!(a, "I have resumed the Whales; 1 waiting request can start.");
}
