//! Sea Spider in English, end to end, with Mova's native model (set MOVA_OCTOPUS_TEST_MODEL to an
//! en.bin; without it these tests report a skip). Network only to a local HTTP fixture on
//! 127.0.0.1: the host a.test is pinned there and private addresses are allowed in this test
//! configuration only.
use mova_octopus::{Config as DeputyConfig, Deputy};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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
    let d = std::env::temp_dir().join(format!("seaspider-en-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
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
    // Loopback fixtures only: the public example keeps the default (refuse non-public hosts).
    v["slot_policy"] = json!({"allow_private_hosts": true});
    serde_json::from_value(v).unwrap()
}

/// (prompt, capabilities, arguments of the first action)
const POSITIVE: [(&str, &[&str], &str); 20] = [
    ("Fetch https://mova.elamur.ai/.", &["fetch"], r#"{"urls":["https://mova.elamur.ai/"]}"#),
    ("Please fetch https://mova.elamur.ai/ and https://mova.elamur.ai/free/en/releases/.", &["fetch"],
        r#"{"urls":["https://mova.elamur.ai/","https://mova.elamur.ai/free/en/releases/"]}"#),
    ("Please fetch these pages: https://a.example.org/x, https://b.example.org/y", &["fetch"],
        r#"{"urls":["https://a.example.org/x","https://b.example.org/y"]}"#),
    ("Please download https://mova.elamur.ai/about.", &["fetch"], r#"{"urls":["https://mova.elamur.ai/about"]}"#),
    // "Can ...?" asks for permission (robots.txt); "Could you ...?" is a polite request.
    ("Could you download https://mova.elamur.ai/about?", &["fetch"], r#"{"urls":["https://mova.elamur.ai/about"]}"#),
    ("Can you download https://mova.elamur.ai/about?", &["robots"], r#"{"target":["https://mova.elamur.ai/about"]}"#),
    ("May we fetch https://mova.elamur.ai/about?", &["robots"], r#"{"target":["https://mova.elamur.ai/about"]}"#),
    // A function word whose dictionary variants include "can" ("a") does not make a permission question.
    ("Fetch https://mova.elamur.ai/ for a friend.", &["fetch"], r#"{"urls":["https://mova.elamur.ai/"]}"#),
    ("Read https://mova.elamur.ai/about.", &["fetch"], r#"{"urls":["https://mova.elamur.ai/about"]}"#),
    ("Are we allowed to fetch mova.elamur.ai?", &["robots"], r#"{"target":["mova.elamur.ai"]}"#),
    ("Can you read https://mova.elamur.ai/about?", &["robots"], r#"{"target":["https://mova.elamur.ai/about"]}"#),
    ("Is MovaBot allowed to crawl https://mova.elamur.ai/free/?", &["robots"], r#"{"target":["https://mova.elamur.ai/free/"]}"#),
    ("How many pages have you stored?", &["stats"], "{}"),
    ("How many pages have you fetched?", &["stats"], "{}"),
    ("How many pages have you stored from mova.elamur.ai?", &["stats"], r#"{"host":["mova.elamur.ai"]}"#),
    ("What did you get from https://mova.elamur.ai/?", &["page"], r#"{"url":["https://mova.elamur.ai/"]}"#),
    ("What is the status of the current run?", &["status"], "{}"),
    ("Is the queue empty?", &["status"], "{}"),
    ("Are you busy?", &["status"], "{}"),
    ("Fetch https://mova.elamur.ai/. Then tell me the status of the run.", &["fetch", "status"], r#"{"urls":["https://mova.elamur.ai/"]}"#),
];

/// (prompt, failure stage)
const REFUSED: [(&str, &str); 16] = [
    // A polar question is read through an observed subject ("Is the queue empty?" above), never
    // through a predicate no capability names; negation and the past stay refused.
    ("Is the queue not empty?", "pragmatic_ambiguity"),
    ("Was the queue empty?", "unsupported_capability"),
    ("Is the weather nice?", "unsupported_capability"),
    ("Empty the queue.", "unsupported_capability"),
    ("Do not fetch https://mova.elamur.ai/.", "pragmatic_ambiguity"),
    ("If you can, fetch https://mova.elamur.ai/.", "pragmatic_ambiguity"),
    ("Fetch the page.", "argument_count"),
    ("What did you get?", "argument_count"),
    ("Delete https://mova.elamur.ai/.", "unsupported_capability"),
    ("Are we allowed to fetch https://mova.elamur.ai/a and https://mova.elamur.ai/b?", "argument_count"),
    ("Fetch https://mova.elamur.ai/ because https://example.org/ has a copy.", "argument_unbound"),
    ("Fetch https://mova.elamur.ai/, which https://example.org/ has.", "argument_unbound"),
    // A copular wh-question is read through its subject ("status"), but the status capability
    // takes no URL: refused at argument binding.
    ("What is the status of https://mova.elamur.ai/?", "argument_unbound"),
    ("Fetch ftp://mova.elamur.ai/x.", "argument_invalid"),
    ("Fetch https://user:secret@mova.elamur.ai/.", "argument_invalid"),
    ("Fetch mova.elamur.ai/about.", "argument_invalid"),
];

/// Questions about the fetch action: never planned as a fetch.
const QUESTIONS_ABOUT_ACTIONS: [&str; 5] = ["Did you fetch https://mova.elamur.ai/?", "Fetch https://mova.elamur.ai/?", "Is https://mova.elamur.ai/ fetched?", "Has https://mova.elamur.ai/about been downloaded?", "Why did you fetch https://mova.elamur.ai/?"];

#[test]
fn english_prompts_select_capabilities_and_arguments() {
    let Some(model) = model() else { return };
    let root = tmp("plan");
    let mut lying = deputy_config(&model, &root);
    seaspider::animal::check_capabilities(&lying).unwrap();
    lying.capabilities.get_mut("fetch").unwrap().intent.observes = true;
    assert!(seaspider::animal::check_capabilities(&lying).is_err(), "a fetch acts; it cannot answer questions");
    let d = Deputy::load(deputy_config(&model, &root)).unwrap();
    for (p, caps, args) in POSITIVE {
        let r = d.review(p);
        assert!(r.failure.is_none(), "{p}: {:?}", r.failure);
        let got: Vec<&str> = r.plan.iter().map(|a| a.capability.as_str()).collect();
        assert_eq!(got, caps, "{p}");
        assert_eq!(serde_json::to_value(&r.plan[0].arguments).unwrap(), serde_json::from_str::<Value>(args).unwrap(), "{p}");
    }
    for (p, stage) in REFUSED {
        let r = d.review(p);
        let f = r.failure.as_ref().unwrap_or_else(|| panic!("accepted: {p} -> {:?}", r.plan));
        assert_eq!(f.stage, stage, "{p}: {}", f.reason);
        assert_eq!(r.followups.len(), 1, "{p}: a refusal is a review candidate");
    }
    // Without the loopback allowance the public policy refuses non-public hosts.
    let mut strict = deputy_config(&model, &root);
    strict.slot_policy = Default::default();
    let d = Deputy::load(strict).unwrap();
    assert_eq!(d.review("Fetch http://127.0.0.1:8080/.").failure.unwrap().stage, "argument_invalid");
    assert_eq!(d.review("Fetch http://a.test/.").failure.unwrap().stage, "argument_invalid");
    // A question about fetching never fetches: only observing capabilities answer questions
    // (`observes` in the deputy configuration; the stage depends on which observing reading is tried).
    let d = Deputy::load(deputy_config(&model, &root)).unwrap();
    for p in QUESTIONS_ABOUT_ACTIONS {
        let r = d.review(p);
        // An observing reading may answer (robots for "Why did you fetch X?"); a fetch never runs.
        assert!(!r.plan.iter().any(|a| a.capability == "fetch"), "{p}: {:?}", r.plan);
        assert_eq!(r.followups.len(), usize::from(r.failure.is_some()), "{p}: a refusal is a review candidate");
    }
    let _ = std::fs::remove_dir_all(root);
}

struct Fixture {
    port: u16,
    log: Arc<Mutex<Vec<(String, Instant, Instant)>>>,
}

fn fixture() -> Fixture {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    let log: Arc<Mutex<Vec<(String, Instant, Instant)>>> = Arc::default();
    let lg = log.clone();
    std::thread::spawn(move || {
        for s in l.incoming() {
            let Ok(mut s) = s else { continue };
            let lg = lg.clone();
            std::thread::spawn(move || {
                let t0 = Instant::now();
                let mut buf = Vec::new();
                let mut chunk = [0u8; 1024];
                while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                    match s.read(&mut chunk) {
                        Ok(0) | Err(_) => return,
                        Ok(n) => buf.extend_from_slice(&chunk[..n]),
                    }
                }
                let head = String::from_utf8_lossy(&buf).to_string();
                let path = head.split_whitespace().nth(1).unwrap_or("/").to_string();
                let (ct, body) = if path == "/robots.txt" {
                    ("text/plain", "User-agent: *\nDisallow: /private\n".to_string())
                } else {
                    if path == "/slow" {
                        std::thread::sleep(Duration::from_millis(3000));
                    }
                    ("text/html; charset=utf-8", format!("<html><title>Page {path}</title><a href=\"/next\">next</a></html>"))
                };
                let _ = write!(s, "HTTP/1.1 200 OK\r\nContent-Type: {ct}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
                lg.lock().unwrap().push((path, t0, Instant::now()));
            });
        }
    });
    Fixture { port, log }
}

fn ask(sock: &Path, id: &str, prompt: &str, urgent: bool) -> (String, Value) {
    let v = seaspider::animal::ask(sock, id, prompt, urgent).unwrap();
    assert_eq!(v["ok"], true, "{prompt}: {v}");
    (v["report"]["answer"].as_str().unwrap().to_string(), v["report"]["report"].clone())
}

#[test]
fn serve_answers_in_english_and_never_preempts() {
    let Some(model) = model() else { return };
    let root = tmp("serve");
    let fx = fixture();
    let u = |p: &str| format!("http://a.test:{}{p}", fx.port);
    let paths = || fx.log.lock().unwrap().iter().map(|x| x.0.clone()).collect::<Vec<_>>();
    let config = root.join("seaspider.json");
    // Callers: the Octopus and Crab only (the Whales are refused here, admitted by default).
    std::fs::write(&config, json!({"robot":{"contact_url":"https://example.invalid/movabot"},"store":"store",
        "network":{"allow_private_addresses":true,"resolve":{"a.test":"127.0.0.1"}},
        "service":{"lock":root.join("seaspider.lock"),"callers":["octopus","crab"]}}).to_string()).unwrap();
    let deputy = root.join("deputy.json");
    std::fs::write(&deputy, serde_json::to_vec(&deputy_config(&model, &root)).unwrap()).unwrap();
    let dir = root.join("private");
    let sock = dir.join("seaspider.sock");
    let (c2, d2, s2) = (config.clone(), deputy.clone(), sock.clone());
    let server = std::thread::spawn(move || seaspider::animal::serve(&c2, &d2, &s2).unwrap());
    let t = Instant::now();
    while !sock.exists() {
        assert!(t.elapsed() < Duration::from_secs(120), "server did not start");
        std::thread::sleep(Duration::from_millis(50));
    }
    let (a, _) = ask(&sock, "q1", "How many pages have you stored?", false);
    assert_eq!(a, "I have stored 0 pages from 0 hosts (0 bytes); the store keeps 0 compressed bytes in all.");
    // Singleton: a second Sea Spider with the same lease refuses to start and names the first.
    let e = seaspider::animal::serve(&config, &deputy, &root.join("second").join("s.sock")).unwrap_err();
    let r = e.downcast_ref::<seaspider::singleton::Refused>().expect("refused by the lease");
    assert!(r.to_string().contains(&format!("serve {}", sock.display())), "{r}");
    assert!(!root.join("second").exists());
    // Who may ask: Crab is on the list (through the generic relay), a Whale is not.
    let crab = mova_octopus::relay::ask_animal(&sock, "crab", "c1", "How many pages have you stored?", false).unwrap();
    assert_eq!(crab.answer, "I have stored 0 pages from 0 hosts (0 bytes); the store keeps 0 compressed bytes in all.");
    let whale = mova_octopus::relay::ask_animal(&sock, "whale", "w1", &format!("Fetch {}.", u("/n7")), false).unwrap();
    assert_eq!(whale.answer, "I did not act on this request because whale is not among the callers I answer (octopus, crab).");
    // The Crab deputy's "Ask Sea Spider: «…»" relays the quoted English and returns the answer.
    let mut crab_deputy = serde_json::to_value(deputy_config(&model, &root.join("crab"))).unwrap();
    crab_deputy["capabilities"] = json!({"seaspider-ask":{"handler":"animal_ask",
        "parameters":{"animal":"Sea Spider","socket":sock,"caller":"crab"},
        "intent":{"request_predicates":["ask","tell","send"],"observations":["ask","tell","send"],"entities":["spider"]},
        "slots":[{"name":"task","kinds":["text"],"min":1,"max":1}]}});
    let crab_deputy = Deputy::load(serde_json::from_value(crab_deputy).unwrap()).unwrap();
    let rep = crab_deputy.run("Ask Sea Spider: «Is the queue empty?»", "crab-1", true).unwrap();
    assert!(rep.failure.is_none(), "{:?}", rep.failure.as_ref().map(|f| serde_json::to_string(f).unwrap()));
    assert_eq!(rep.results[0]["output"]["answer"], "I am not fetching anything now. No job is waiting.");
    assert_eq!(rep.results[0]["output"]["request"], "Is the queue empty?");
    let journal = std::fs::read_to_string(dir.join("requests.jsonl")).unwrap();
    let rows: Vec<Value> = journal.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    let who: Vec<(&str, bool)> = rows.iter().map(|r| (r["from"].as_str().unwrap(), r["admitted"].as_bool().unwrap())).collect();
    assert_eq!(who, [("octopus", true), ("crab", true), ("whale", false), ("crab", true)]);
    assert_eq!(rows[1]["peer_pid"], std::process::id());
    assert!(!std::fs::read_to_string(dir.join("jobs.jsonl")).unwrap_or_default().contains("/n7"), "a refused caller queues nothing");
    let (a, _) = ask(&sock, "q2", "Is the queue empty?", false);
    assert_eq!(a, "I am not fetching anything now. No job is waiting.");
    // A robots question with nothing stored waits for its own check while the worker is idle.
    let (a, _) = ask(&sock, "q3", &format!("Can you read {}?", u("/ok")), false);
    assert_eq!(a, format!("Yes, {} allows MovaBot to fetch {} (no rule matches it). I have just read robots.txt.", u("/robots.txt"), u("/ok")));
    // The next one is answered from the stored robots.txt, without a request.
    let n = paths().len();
    let (a, _) = ask(&sock, "q4", &format!("Are we allowed to fetch {}?", u("/private/x")), false);
    assert!(a.starts_with(&format!("No, {} disallows {} for MovaBot (rule Disallow: /private). I read robots.txt at ", u("/robots.txt"), u("/private/x"))), "{a}");
    assert_eq!(paths().len(), n);
    // Priority without preemption: a slow job runs; a normal and an urgent job wait; the urgent
    // one goes first after the slow job has finished.
    let (a, _) = ask(&sock, "f1", &format!("Fetch {}.", u("/slow")), false);
    assert_eq!(a, format!("I have queued 1 page for fetching (octopus-f1-0): {}. I am starting on it now.", u("/slow")));
    let (a, _) = ask(&sock, "f2", &format!("Fetch {} and {}.", u("/n1"), u("/private/x")), false);
    assert!(a.ends_with("It starts when the current job is done."), "{a}");
    let (a, _) = ask(&sock, "f3", &format!("Please fetch {}.", u("/u1")), true);
    assert!(a.ends_with("It starts when the current job is done."), "{a}");
    let asked = Instant::now();
    let (a, _) = ask(&sock, "q5", "What is the status of the current run?", false);
    assert!(asked.elapsed() < Duration::from_millis(1500), "a status question waits for no fetch");
    assert!(a.starts_with("I am working on octopus-f1-0 (started "), "{a}");
    assert!(a.contains(&format!("0 of 1 page is done, now {}. 2 jobs are waiting.", u("/slow"))), "{a}");
    let t = Instant::now();
    loop {
        let (a, _) = ask(&sock, &format!("w{}", t.elapsed().as_millis()), "Is the queue empty?", false);
        if a.starts_with("I am not fetching anything now. No job is waiting.") {
            break;
        }
        assert!(t.elapsed() < Duration::from_secs(60), "jobs did not finish: {a}");
        std::thread::sleep(Duration::from_millis(300));
    }
    let pages: Vec<String> = paths().into_iter().filter(|p| p != "/robots.txt").collect();
    assert_eq!(pages, ["/slow", "/u1", "/n1"], "urgent before normal, the running job never interrupted, robots honoured");
    let log = fx.log.lock().unwrap().clone();
    let slow_end = log.iter().find(|x| x.0 == "/slow").unwrap().2;
    let u1_start = log.iter().find(|x| x.0 == "/u1").unwrap().1;
    assert!(u1_start >= slow_end, "the urgent job started only after the running one ended");
    let (a, _) = ask(&sock, "q6", "What is the status of the current run?", false);
    assert!(a.contains(": I fetched 1 of 2 pages."), "{a}");
    assert!(a.ends_with(&format!("I could not fetch {} because robots.txt disallows it (rule Disallow: /private).", u("/private/x"))), "{a}");
    let (a, _) = ask(&sock, "q7", &format!("What did you get from {}?", u("/u1")), false);
    assert!(a.starts_with(&format!("I fetched {} at ", u("/u1"))), "{a}");
    assert!(a.ends_with("HTTP 200, text/html; charset=utf-8, 60 bytes. Its title is “Page /u1”. It links to 1 page."), "{a}");
    let (a, _) = ask(&sock, "q8", &format!("What did you get from {}?", u("/private/x")), false);
    assert_eq!(a, format!("I could not fetch {} because robots.txt disallows it (rule Disallow: /private).", u("/private/x")));
    let (a, _) = ask(&sock, "q9", "How many pages have you stored from a.test?", false);
    assert_eq!(a, "I have stored 3 pages from a.test. I could not fetch 1 more page: robots.txt disallowed 1.");
    // Refusals: no job, no request, an English reason and a review candidate.
    let n = paths().len();
    for (i, p) in [
        format!("Do not fetch {}.", u("/n9")),
        format!("If you can, fetch {}.", u("/n9")),
        "Fetch the page.".to_string(),
        format!("Delete {}.", u("/u1")),
        format!("Fetch {} because {} has a copy.", u("/n9"), u("/n8")),
        format!("Are we allowed to fetch {} and {}?", u("/a"), u("/b")),
    ]
    .iter()
    .enumerate()
    {
        let (a, rep) = ask(&sock, &format!("r{i}"), p, false);
        assert!(a.starts_with("I did not act on this request because I could not read it as one I can do: "), "{p}: {a}");
        assert!(a.contains("I kept it for review as candidate "), "{a}");
        assert!(rep["results"].as_array().unwrap().is_empty());
    }
    let (a, _) = ask(&sock, "r9", "Is the queue empty?", false);
    assert!(a.starts_with("I am not fetching anything now. No job is waiting."), "{a}");
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(paths().len(), n, "a refused prompt sends no request");
    // A repeated request id returns the journaled answer, not a new job.
    let (again, _) = ask(&sock, "f1", &format!("Fetch {}.", u("/slow")), false);
    assert_eq!(again, format!("I have queued 1 page for fetching (octopus-f1-0): {}. I am starting on it now.", u("/slow")));
    assert_eq!(paths().len(), n);
    std::fs::write(dir.join("STOP"), b"").unwrap();
    server.join().unwrap();
    let _ = std::fs::remove_dir_all(root);
}
