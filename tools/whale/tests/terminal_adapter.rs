//! The Whale's terminal adapter (`MovaTerminal` over `mova-terminal`) end to end against a fake
//! LLM: a bash script in a real tmux server on a private socket. No model is started or asked.
//! The full Whale run additionally needs MOVA_OCTOPUS_TEST_MODEL (Mova's en.bin); without it that
//! test reports a skip.
use mova_whale::terminal::{AskError, MovaTerminal, PrivateTerminal, Question, TerminalConfig};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{Duration, Instant},
};

const FAKE: &str = r#"#!/bin/bash
# A fake LLM terminal: answers each question into the answer file and prints the marker, or
# shows a usage-limit message and stays silent.
mode="$1"; marker="$2"
stty -echo 2>/dev/null
echo "$marker"
file=""
while IFS= read -r line; do
  case "$line" in
    "Answer file: "*) file="${line#Answer file: }";;
    "End of question.")
      if [ "$mode" = limit ]; then echo "You've hit your usage limit. Try again later."; continue; fi
      printf 'A zorbic rule attaches every word to its head.\n' > "$file"
      # slow: the answer file exists while the LLM is still working (as Codex does), then idle.
      if [ "$mode" = slow ]; then printf 'Working (1s - esc to interrupt)'; sleep 1.5; printf '\r\033[2K'; fi
      echo "$marker";;
  esac
done
"#;

struct Fx {
    dir: PathBuf,
    socket: String,
}
impl Drop for Fx {
    fn drop(&mut self) {
        let _ = Command::new("tmux").args(["-L", &self.socket, "kill-server"]).output();
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn tmux() -> Option<PathBuf> {
    ["/usr/bin/tmux", "/usr/local/bin/tmux", "/bin/tmux"].iter().map(PathBuf::from).find(|p| p.exists())
}

/// A fake terminal for `animal`, held in session `<animal>-sol`; returns its Whale-side config.
fn fx(name: &str, animal: &str, mode: &str) -> Option<(Fx, TerminalConfig)> {
    let Some(tmux) = tmux() else {
        eprintln!("SKIPPED: tmux not installed");
        return None;
    };
    let socket = format!("whale-term-{name}-{}", std::process::id());
    let dir = std::env::temp_dir().join(&socket);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("fake.sh"), FAKE).unwrap();
    let fx = Fx { dir: dir.clone(), socket: socket.clone() };
    let session = format!("{animal}-sol");
    let owner = format!("mova-{animal}-sol");
    assert!(Command::new(&tmux).args(["-L", &socket, "new-session", "-d", "-s", &session, "-x", "200", "-y", "50", "bash"]).arg(dir.join("fake.sh")).args([mode, "TERMINAL READY"]).status().unwrap().success());
    Command::new(&tmux).args(["-L", &socket, "set-option", "-t", &session, "@mova_crab_owner", &owner]).status().unwrap();
    let t = Instant::now();
    loop {
        let out = Command::new(&tmux).args(["-L", &socket, "capture-pane", "-p", "-t", &format!("={session}:")]).output().unwrap();
        if String::from_utf8_lossy(&out.stdout).contains("TERMINAL READY") {
            break;
        }
        assert!(t.elapsed() < Duration::from_secs(5), "fake terminal did not start");
        std::thread::sleep(Duration::from_millis(100));
    }
    let profile = dir.join("profile.json");
    fs::write(&profile, json!({"animal":animal,"session":session,"tmux":tmux,"socket":socket,"pane_command":"bash","owner_marker":owner,
        "idle_marker":"TERMINAL READY","idle_window":2,"root":dir.join("terminal"),"answer_timeout_seconds":20}).to_string()).unwrap();
    Some((fx, TerminalConfig { profile, poll_ms: 100 }))
}

fn question(subject: &str) -> Question {
    Question { id: "whale-1-q1".into(), kind: "term".into(), subject: subject.into(), text: format!("What does «{subject}» mean in this paper?\n- context line"), context: vec!["context line".into()], items: vec![] }
}

#[test]
fn the_adapter_returns_the_answer_file() {
    let Some((_f, c)) = fx("answer", "whale-1", "answer") else { return };
    let mut t = MovaTerminal::open(&c).unwrap();
    assert_eq!(t.animal(), "whale-1");
    let a = t.ask(&question("zorbic")).unwrap();
    assert_eq!(a.text, "A zorbic rule attaches every word to its head.\n");
    assert!(a.terminal_id.is_some());
}

#[test]
fn an_answer_counts_only_when_the_terminal_is_idle_again() {
    // The 2026-11-04 round 1 asked its next question while the LLM had written its answer file
    // but was still working; the terminal refused it as «not idle». The adapter now waits.
    let Some((_f, c)) = fx("slow", "whale-4", "slow") else { return };
    let mut t = MovaTerminal::open(&c).unwrap();
    let first = t.ask(&question("zorbic")).unwrap();
    assert!(first.elapsed_ms >= 1000, "returned before the terminal was idle: {} ms", first.elapsed_ms);
    let mut q2 = question("cycle");
    q2.id = "whale-4-q2".into();
    assert!(t.ask(&q2).is_ok(), "the next question was refused");
}

#[test]
fn a_usage_limit_ends_the_question_without_a_resend() {
    let Some((_f, c)) = fx("limit", "whale-2", "limit") else { return };
    let mut t = MovaTerminal::open(&c).unwrap();
    let start = Instant::now();
    match t.ask(&question("zorbic")) {
        Err(AskError::Limit(r)) => assert!(r.contains("usage limit"), "{r}"),
        other => panic!("expected a limit, got {other:?}"),
    }
    assert!(start.elapsed() < Duration::from_secs(10), "the limit was not seen before the timeout");
    // The question stays pending in the terminal; another one is refused, nothing is re-sent.
    match t.ask(&question("other")) {
        Err(AskError::Failed(r)) => assert!(r.contains("never replayed"), "{r}"),
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn a_whale_reads_writes_and_asks_through_its_own_terminal() {
    let Some(model) = std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") else {
        eprintln!("SKIPPED: MOVA_OCTOPUS_TEST_MODEL not configured");
        return;
    };
    std::env::set_var("WORLD_EN_MODEL", model);
    let Some((f, c)) = fx("paper", "whale-3", "answer") else { return };
    let paper = f.dir.join("paper.md");
    fs::write(&paper, "# Zorbic Trees\n\n## Method\n\nThe parser attaches every word to its head with a zorbic rule. Zorbic rules never create a cycle in the tree.\n\n## Results\n\nZorbic parsing is faster than the baseline on long sentences.\n").unwrap();
    let sha = mova_octopus::digest(&fs::read(&paper).unwrap());
    let cfg: mova_whale::paper::PaperConfig = serde_json::from_value(json!({"schema":"mova-shark-paper-v1","whale":"whale-3","run_root":f.dir.join("run"),
        "paper":{"id":"zorbic-2024-trees","path":paper,"sha256":sha,"licence":"CC BY 4.0"},"words":{"min":10,"max":400},"max_questions":1,
        "terminal":c})).unwrap();
    let mut t = MovaTerminal::open(&cfg.terminal).unwrap();
    let m = mova_whale::paper::run(&cfg, &mut t).unwrap();
    assert_eq!((m["questions_asked"].as_u64(), m["questions_answered"].as_u64()), (Some(1), Some(1)), "{m}");
    let desc = fs::read_to_string(f.dir.join("run/description.txt")).unwrap();
    assert!(desc.contains("«Method»"), "{desc}");
    assert!(!desc.contains("A zorbic rule attaches"), "the terminal's answer reached the description");
    let glossary = fs::read_to_string(f.dir.join("run/glossary.txt")).unwrap();
    assert!(glossary.starts_with("«Zorbic»:"), "{glossary}");
    let journal: Vec<Value> = fs::read_to_string(f.dir.join("run/journal.jsonl")).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert!(journal.iter().any(|v| v["event"] == "answer" && v["terminal_question"].is_string()));
}
