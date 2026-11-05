#![cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{Duration, Instant};
use serde_json::json;

struct Fixture { dir: PathBuf, fake: PathBuf }
impl Fixture {
    fn new(prompts: &[&str]) -> Self {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("mova-codex-batch-{}-{stamp}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let f = Self { fake: dir.join("fake-codex"), dir };
        f.requests(prompts, "gpt-6.1-terra");
        f.script(false, false);
        f
    }
    fn requests(&self, prompts: &[&str], model: &str) {
        let text = prompts.iter().enumerate().map(|(i,p)| json!({"custom_id":i.to_string(),
            "params":{"model":model,"output_config":{"effort":"medium"},
            "messages":[{"role":"user","content":[{"type":"text","text":p}]}]}}).to_string()+"\n").collect::<String>();
        std::fs::write(self.dir.join("batch-requests.jsonl"), text).unwrap();
    }
    fn script(&self, fail: bool, hang: bool) {
        let body = format!("#!/bin/sh\nset -eu\ntest -z \"${{ANTHROPIC_API_KEY+x}}\"\ntest -z \"${{OPENAI_API_KEY+x}}\"\nprintf 'called\\n' >> \"$MOVA_TEST_CALLS\"\n{}\ninput=$(cat)\n{}\nprintf '%s\\n' '{{\"type\":\"item.completed\",\"item\":{{\"type\":\"agent_message\",\"text\":\"sample OK\"}}}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":5,\"cached_input_tokens\":0,\"output_tokens\":2}}}}'\n",
            if hang { "sleep 30" } else { ":" },
            if fail { "case \"$input\" in *FAIL*) exit 9;; esac" } else { ":" });
        std::fs::write(&self.fake, body).unwrap();
        std::fs::set_permissions(&self.fake, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    fn run(&self, asynchronous: bool) -> Output {
        Command::new(env!("CARGO_BIN_EXE_annot")).arg("batch-run").arg(&self.dir)
            .env("MOVA_CODEX_BIN", &self.fake).env("MOVA_TEST_CALLS", self.dir.join("calls"))
            .env("ANTHROPIC_API_KEY", "fixture-only").env("OPENAI_API_KEY", "fixture-only")
            .env("MOVA_TEACHER_TIMEOUT_SECS", "1")
            .env("ANNOT_NO_WAIT", if asynchronous { "1" } else { "0" }).output().unwrap()
    }
    fn calls(&self) -> usize { std::fs::read_to_string(self.dir.join("calls")).unwrap_or_default().lines().count() }
    fn complete(&self) -> bool { self.dir.join("batch-results.jsonl").exists() }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.dir); } }

#[test]
fn failure_requires_explicit_resume_and_never_repeats_successes() {
    let f = Fixture::new(&["first", "FAIL"]);
    f.script(true, false);
    assert!(!f.run(false).status.success());
    assert!(!f.complete());
    assert_eq!(f.calls(), 2);
    assert!(!f.run(false).status.success());
    assert_eq!(f.calls(), 2, "failure was retried without explicit resume");
    f.script(false, false);
    std::fs::remove_file(f.dir.join("codex-error.txt")).unwrap();
    let out = f.run(false);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(f.calls(), 3);
    assert_eq!(std::fs::read_to_string(f.dir.join("batch-results.jsonl")).unwrap().lines().count(), 2);
    assert!(f.run(false).status.success());
    assert_eq!(f.calls(), 3);
}

#[test]
fn async_worker_publishes_only_complete_results() {
    let f = Fixture::new(&["first", "second"]);
    let out = f.run(true);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let start = Instant::now();
    while !f.complete() && start.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(f.complete(), "worker did not finish");
    assert_eq!(f.calls(), 2);
    assert_eq!(std::fs::read_to_string(f.dir.join("batch-results.jsonl")).unwrap().lines().count(), 2);
}

#[test]
fn legacy_teacher_is_rejected_before_launch() {
    let f = Fixture::new(&["first"]);
    f.requests(&["first"], "claude-haiku-5-5");
    let out = f.run(false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Codex only"));
    assert_eq!(f.calls(), 0);
    assert!(!f.complete());
}

#[test]
fn changed_requests_cannot_reuse_partial_answers() {
    let f = Fixture::new(&["first", "FAIL"]);
    f.script(true, false);
    assert!(!f.run(false).status.success());
    f.requests(&["changed first", "second"], "gpt-6.1-terra");
    std::fs::remove_file(f.dir.join("codex-error.txt")).unwrap();
    let out = f.run(false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("requests changed"));
    assert_eq!(f.calls(), 2);
}

#[test]
fn timeout_is_a_failure_without_partial_publication() {
    let f = Fixture::new(&[&"long prompt ".repeat(10000)]);
    f.script(false, true);
    let start = Instant::now();
    let out = f.run(false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("timed out"));
    assert!(start.elapsed() < Duration::from_secs(5));
    assert!(!f.complete());
}

#[test]
fn stale_worker_lock_is_reported() {
    let f = Fixture::new(&["first"]);
    std::fs::create_dir(f.dir.join("codex-worker.lock")).unwrap();
    std::fs::write(f.dir.join("codex-worker.lock/pid"), "4294967295").unwrap();
    let out = f.run(false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("worker"));
    assert_eq!(f.calls(), 0);
}

/// A succeeded partial row for request `id` as the worker would write it, without a terminator.
fn success_row(id: &str) -> String {
    json!({"custom_id":id, "provider":"codex", "model":"gpt-6.1-terra", "cost_reported":false,
        "result":{"type":"succeeded","message":{"content":[{"type":"text","text":"prior OK"}],
            "stop_reason":"end_turn","usage":{"input_tokens":1,"cached_input_tokens":0,"output_tokens":1}}}}).to_string()
}
fn ids(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path).unwrap().lines().filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap_or_else(|e| panic!("unparsable row {l:?}: {e}"))["custom_id"].as_str().unwrap().to_string())
        .collect()
}

/// Supplied partial [b] whose last valid row lacks LF: resume must keep b as a record, append a and c
/// as separate records, never re-ask b, and publish three parsable rows in order [b,a,c].
#[test]
fn partial_without_final_lf_keeps_record_separation_on_resume() {
    for (variant, tail) in [("no-lf", ""), ("spaces-no-lf", "   "), ("lf", "\n")] {
        let f = Fixture::new(&["a", "b", "c"]);
        let prior = success_row("1") + tail;
        std::fs::write(f.dir.join("codex-results.partial.jsonl"), &prior).unwrap();
        let out = f.run(false);
        assert!(out.status.success(), "{variant}: {}", String::from_utf8_lossy(&out.stderr));
        assert_eq!(f.calls(), 2, "{variant}: b must not be asked again");
        let bytes = std::fs::read(f.dir.join("batch-results.jsonl")).unwrap();
        assert!(bytes.starts_with(prior.as_bytes()), "{variant}: original partial bytes retained");
        assert_eq!(bytes.last(), Some(&b'\n'), "{variant}: final file ends with LF");
        assert_eq!(ids(&f.dir.join("batch-results.jsonl")), ["1", "0", "2"], "{variant}");
        assert_eq!(std::fs::read_to_string(f.dir.join("batch-results.jsonl")).unwrap().lines().count(), 3, "{variant}");
    }
}

/// Boundary shapes: empty and whitespace-only partial files add no record; an all-done partial without
/// LF publishes with zero asks and a terminator; a truncated last row is refused before any normalization;
/// a failed next request after the repair leaves a cleanly separated partial that resumes without duplicates.
#[test]
fn partial_boundary_shapes_and_failed_next_request() {
    for (variant, prior) in [("empty", String::new()), ("whitespace", "  \n \t".to_string()), ("whitespace-no-lf", "   ".to_string())] {
        let f = Fixture::new(&["a", "b"]);
        std::fs::write(f.dir.join("codex-results.partial.jsonl"), &prior).unwrap();
        let out = f.run(false);
        assert!(out.status.success(), "{variant}: {}", String::from_utf8_lossy(&out.stderr));
        assert_eq!(f.calls(), 2, "{variant}");
        assert_eq!(ids(&f.dir.join("batch-results.jsonl")), ["0", "1"], "{variant}");
    }
    let f = Fixture::new(&["a", "b"]);
    let prior = success_row("0") + "\n" + &success_row("1");
    std::fs::write(f.dir.join("codex-results.partial.jsonl"), &prior).unwrap();
    assert!(f.run(false).status.success());
    assert_eq!(f.calls(), 0, "all done: no ask");
    let bytes = std::fs::read(f.dir.join("batch-results.jsonl")).unwrap();
    assert!(bytes.starts_with(prior.as_bytes()) && bytes.last() == Some(&b'\n'));
    assert_eq!(ids(&f.dir.join("batch-results.jsonl")), ["0", "1"]);
    let f = Fixture::new(&["a", "b"]);
    let truncated = success_row("0");
    std::fs::write(f.dir.join("codex-results.partial.jsonl"), &truncated[..truncated.len() - 5]).unwrap();
    let out = f.run(false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("incomplete"));
    assert_eq!(f.calls(), 0, "invalid partial row must stop before any ask");
    assert_eq!(std::fs::read(f.dir.join("codex-results.partial.jsonl")).unwrap(), truncated[..truncated.len() - 5].as_bytes(), "no normalization of an invalid file");
    let f = Fixture::new(&["a", "FAIL", "c"]);
    f.script(true, false);
    std::fs::write(f.dir.join("codex-results.partial.jsonl"), success_row("0")).unwrap();
    assert!(!f.run(false).status.success());
    assert_eq!(f.calls(), 1, "a is done; FAIL asked once");
    let partial = std::fs::read_to_string(f.dir.join("codex-results.partial.jsonl")).unwrap();
    assert_eq!(partial, success_row("0") + "\n", "repair leaves exactly one LF, no new rows");
    f.script(false, false);
    std::fs::remove_file(f.dir.join("codex-error.txt")).unwrap();
    assert!(f.run(false).status.success());
    assert_eq!(f.calls(), 3);
    assert_eq!(ids(&f.dir.join("batch-results.jsonl")), ["0", "1", "2"]);
}

/// Strict consumer: every physical line must parse as JSON, no trim, no blank-line filter (as several
/// downstream readers of batch-results.jsonl do). Whitespace-only tails (ASCII space/tab/CR) and a
/// whitespace tail after a terminated row must stay glued to the next row, never become a blank line;
/// a bare CR after a complete last row is trailing JSON whitespace and gets a separator.
#[test]
fn strict_line_consumers_see_only_json_rows_after_resume() {
    fn strict_ids(path: &std::path::Path) -> Vec<String> {
        std::fs::read_to_string(path).unwrap().lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap_or_else(|e| panic!("strict reader rejects line {l:?}: {e}"))["custom_id"].as_str().unwrap().to_string())
            .collect()
    }
    let cases: [(&str, String, &[&str]); 6] = [
        ("spaces-only", "   ".to_string(), &["0", "1", "2"]),
        ("tab-only", "\t".to_string(), &["0", "1", "2"]),
        ("cr-only", "\r".to_string(), &["0", "1", "2"]),
        ("row-lf-spaces", success_row("1") + "\n  ", &["1", "0", "2"]),
        ("row-lf-cr", success_row("1") + "\n\r", &["1", "0", "2"]),
        ("row-bare-cr", success_row("1") + "\r", &["1", "0", "2"]),
    ];
    for (variant, prior, expected) in cases {
        let f = Fixture::new(&["a", "b", "c"]);
        std::fs::write(f.dir.join("codex-results.partial.jsonl"), &prior).unwrap();
        let out = f.run(false);
        assert!(out.status.success(), "{variant}: {}", String::from_utf8_lossy(&out.stderr));
        assert_eq!(f.calls(), expected.len() - usize::from(prior.contains("custom_id")), "{variant}");
        let bytes = std::fs::read(f.dir.join("batch-results.jsonl")).unwrap();
        assert!(bytes.starts_with(prior.as_bytes()), "{variant}: prior bytes retained");
        assert_eq!(bytes.last(), Some(&b'\n'), "{variant}");
        assert_eq!(strict_ids(&f.dir.join("batch-results.jsonl")), expected, "{variant}");
    }
}
