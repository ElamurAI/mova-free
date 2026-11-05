//! Shared, subscription-authenticated Codex teacher transport. No Anthropic fallback.
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};

pub const DEFAULT_MODEL: &str = "gpt-6.1-terra";
static NEXT: AtomicU64 = AtomicU64::new(0);

pub fn model(component_variable: &str) -> String {
    std::env::var("MOVA_TEACHER_MODEL").or_else(|_| std::env::var(component_variable))
        .unwrap_or_else(|_| DEFAULT_MODEL.into())
}

pub fn validate_model(model: &str) -> Result<()> {
    ensure!(model.starts_with("gpt-") && !model.chars().any(char::is_whitespace),
        "Mova training uses Codex only; unsupported teacher model {model:?}. Re-prepare legacy Claude requests.");
    Ok(())
}

/// Decode one completed Codex turn; lifecycle headers are optional, but ordered when present.
/// Multiple agent messages within the turn are allowed; the last completed one is the answer.
/// An error, tool use, repeated turn, or post-completion item cannot become a training answer.
/// Sole exception: an `error` event whose message is an exact Codex stream-retry notice
/// (`transient_reconnect`) before completion is skipped; the turn must still reach
/// `turn.completed` with a final agent message and no `turn.failed`, so a reconnect followed by
/// failure or by no completion stays an error.
pub fn decode_events(text: &str) -> Result<Value> {
    let (mut answer, mut usage) = (None, None);
    let (mut started, mut items_seen, mut thread_seen) = (false, false, false);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).context("Codex JSONL event")?;
        match v["type"].as_str() {
            Some("error") if usage.is_none()
                && v["message"].as_str().and_then(transient_reconnect).is_some() => {}
            Some("error" | "turn.failed") => bail!("Codex teacher turn failed"),
            Some("thread.started") => {
                ensure!(!thread_seen && !started && !items_seen && usage.is_none(),
                    "Codex teacher received a repeated or late thread start");
                thread_seen = true;
            }
            Some("turn.started") => {
                ensure!(!started && !items_seen && usage.is_none(),
                    "Codex teacher received a repeated or late turn start");
                started = true;
            }
            Some("turn.completed") => {
                ensure!(usage.is_none(), "Codex teacher received multiple completed turns");
                ensure!(answer.as_ref().is_some_and(|s: &String| !s.trim().is_empty()),
                    "Codex turn completed before a teacher answer");
                usage = Some(v["usage"].clone());
            }
            Some("item.started" | "item.updated" | "item.completed") => {
                ensure!(usage.is_none(), "Codex teacher received an item after completion");
                items_seen = true;
                let kind = v["item"]["type"].as_str().unwrap_or("");
                ensure!(matches!(kind, "agent_message" | "reasoning"), "Codex teacher used unexpected item type {kind:?}");
                if v["type"] == "item.completed" && kind == "agent_message" {
                    answer = v["item"]["text"].as_str().map(String::from);
                }
            }
            _ => {}
        }
    }
    let answer = answer.filter(|s| !s.trim().is_empty()).context("Codex returned no teacher answer")?;
    let u = usage.context("Codex turn did not complete")?;
    let input = u["input_tokens"].as_u64().context("Codex input token accounting missing")?;
    let output = u["output_tokens"].as_u64().context("Codex output token accounting missing")?;
    // Compatibility with existing call logs and result parsers; subscription cost is not reported.
    Ok(json!({"result": answer, "is_error": false, "provider": "codex",
        "total_cost_usd": 0.0, "cost_reported": false,
        "usage": {"input_tokens": input, "output_tokens": output,
            "cache_read_input_tokens": u["cached_input_tokens"].as_u64().unwrap_or(0),
            "cache_creation_input_tokens": 0}}))
}

/// Exact Codex CLI 0.165.0 error texts for a provider that refused the turn for capacity/load
/// (strings taken from the `codex` binary's error displays; the first was observed in Fish r2 on
/// 2026-11-04). Usage, quota, auth and policy errors are deliberately absent: they are not "busy".
pub const PROVIDER_BUSY_MESSAGES: [&str; 4] = [
    "Selected model is at capacity. Please try a different model.",
    "Flex capacity unavailable.",
    "Server overloaded; retry later.",
    "We\u{2019}re currently experiencing high demand, which may cause temporary errors.",
];

/// `Some(message)` only when the JSONL is a provider-busy refusal before any work: every nonblank
/// line is a lifecycle header (`thread.started`, `turn.started`) or an `error`/`turn.failed` event,
/// at least one failure is present, and every failure message is exactly one of
/// `PROVIDER_BUSY_MESSAGES`. Any item, completion, unknown event, other message or unparsable line
/// gives `None` (the attempt then stays an ordinary, possibly-sent failure).
pub fn provider_busy(text: &str) -> Option<&'static str> {
    let mut found = None;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).ok()?;
        let message = match v["type"].as_str()? {
            "thread.started" | "turn.started" => continue,
            "error" => v["message"].as_str()?,
            "turn.failed" => v["error"]["message"].as_str()?,
            _ => return None,
        };
        let known = PROVIDER_BUSY_MESSAGES.iter().copied().find(|m| *m == message)?;
        found = Some(known);
    }
    found
}

/// Prefix of the Codex CLI 0.165.0 stream-retry notice (`core/src/responses_retry.rs`), which
/// `codex exec --json` emits as `{"type":"error","message":"Reconnecting... N/M (cause)"}` while it
/// retries a dropped response stream (observed in Fish r3 on 2026-11-05 as
/// `Reconnecting... 2/5 (stream disconnected before completion: ...)`, followed by a completed turn).
/// The sibling `Reconnecting... waiting for network` has no attempt counter and is not accepted.
pub const RECONNECT_PREFIX: &str = "Reconnecting... ";

/// `Some((attempt, max))` only for the exact shape `Reconnecting... N/M (cause)`: N and M are
/// unsigned decimal integers with 1 <= N <= M, the cause is a nonblank single line, and it does not
/// mention a limit or quota (those are never treated as transient). Anything else gives `None`.
pub fn transient_reconnect(message: &str) -> Option<(u64, u64)> {
    let rest = message.strip_prefix(RECONNECT_PREFIX)?;
    let (counter, cause) = rest.split_once(' ')?;
    let (n, m) = counter.split_once('/')?;
    let number = |s: &str| (!s.is_empty() && s.len() <= 9 && s.bytes().all(|b| b.is_ascii_digit()))
        .then(|| s.parse::<u64>().ok()).flatten();
    let (n, m) = (number(n)?, number(m)?);
    let cause = cause.strip_prefix('(')?.strip_suffix(')')?;
    let lower = cause.to_ascii_lowercase();
    (1 <= n && n <= m && !cause.trim().is_empty() && !cause.contains(['\n', '\r'])
        && !lower.contains("limit") && !lower.contains("quota")).then_some((n, m))
}

/// Number of `error` events in the JSONL whose message is a `transient_reconnect` notice, wherever
/// they occur (observability for receipts; it does not decide success, `decode_events` does).
pub fn transient_reconnects(text: &str) -> u64 {
    text.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|v| v["type"] == "error" && v["message"].as_str().and_then(transient_reconnect).is_some())
        .count() as u64
}

struct CallDir(PathBuf);
impl Drop for CallDir {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

/// Isolated, tool-free, subscription-authenticated `codex exec` arguments reading the prompt
/// from stdin and writing JSONL events (decode with `decode_events`). Shared by other callers.
pub fn exec_args(model: &str, effort: &str) -> Vec<String> {
    let mut a: Vec<String> = ["exec", "--ignore-user-config", "--ignore-rules", "--ephemeral",
        "--skip-git-repo-check", "--sandbox", "read-only", "--json", "--color", "never",
        "--model", model, "-c", "approval_policy=\"never\"", "-c", "forced_login_method=\"chatgpt\"",
        "-c", "web_search=\"disabled\"", "-c", "project_doc_max_bytes=0"]
        .into_iter().map(String::from).collect();
    a.extend(["-c".into(), format!("model_reasoning_effort={}", json!(effort))]);
    for feature in ["shell_tool", "multi_agent", "apps", "plugins", "hooks", "browser_use",
        "browser_use_external", "computer_use", "image_generation", "view_image", "memories",
        "skill_search", "unbounded_connection_retries"] {
        a.extend(["--disable".into(), feature.into()]);
    }
    a.push("-".into());
    a
}

/// Provider credential variables removed so only the ChatGPT subscription login is used.
pub const CLEARED_ENV: [&str; 4] = ["ANTHROPIC_API_KEY", "OPENAI_API_KEY", "CODEX_API_KEY", "OPENAI_BASE_URL"];

fn command(executable: &str, cwd: &Path, model: &str, effort: &str) -> Command {
    let mut c = Command::new(executable);
    c.args(exec_args(model, effort));
    c.current_dir(cwd).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    for key in CLEARED_ENV {
        c.env_remove(key);
    }
    #[cfg(unix)] {
        use std::os::unix::process::CommandExt;
        c.process_group(0);
    }
    c
}

fn stop(child: &mut std::process::Child) {
    #[cfg(unix)] unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL); }
    let _ = child.kill();
    let _ = child.wait();
}

/// One bounded invocation, no automatic retries. Prompts go through stdin, never command arguments.
pub fn ask(cwd: &Path, model: &str, effort: &str, prompt: &str) -> Result<Value> {
    validate_model(model)?;
    ensure!(matches!(effort, "low" | "medium" | "high" | "xhigh"), "unsupported Codex effort {effort:?}");
    let timeout: u64 = std::env::var("MOVA_TEACHER_TIMEOUT_SECS").unwrap_or_else(|_| "1800".into())
        .parse().context("MOVA_TEACHER_TIMEOUT_SECS must be an integer")?;
    ensure!(timeout > 0, "MOVA_TEACHER_TIMEOUT_SECS must be positive");
    std::fs::create_dir_all(cwd)?;
    let dir = CallDir(cwd.join(format!("codex-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed))));
    std::fs::create_dir(&dir.0)?;
    let executable = std::env::var("MOVA_CODEX_BIN").unwrap_or_else(|_| "codex".into());
    let start = Instant::now();
    let mut child = command(&executable, &dir.0, model, effort).spawn()
        .context("start Codex teacher (install Codex CLI and run codex login --device-auth)")?;
    let mut stdin = child.stdin.take().context("Codex stdin")?;
    let stdout = child.stdout.take().context("Codex stdout")?;
    let stderr = child.stderr.take().context("Codex stderr")?;
    let payload = prompt.to_owned();
    let writer = std::thread::spawn(move || stdin.write_all(payload.as_bytes()));
    const LIMIT: u64 = 32 * 1024 * 1024;
    let reader = std::thread::spawn(move || { let mut b = Vec::new(); stdout.take(LIMIT + 1).read_to_end(&mut b).map(|_| b) });
    let errors = std::thread::spawn(move || { let mut b = Vec::new(); stderr.take(LIMIT + 1).read_to_end(&mut b).map(|_| b) });
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) if start.elapsed() < Duration::from_secs(timeout) => std::thread::sleep(Duration::from_millis(50)),
            Ok(None) => { stop(&mut child); break Err(anyhow::anyhow!("Codex teacher timed out after {timeout}s; not retried")); }
            Err(e) => { stop(&mut child); break Err(e.into()); }
        }
    };
    #[cfg(unix)] unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL); }
    let written = writer.join().map_err(|_| anyhow::anyhow!("Codex stdin worker panicked"))?;
    let out = reader.join().map_err(|_| anyhow::anyhow!("Codex stdout worker panicked"))??;
    let err = errors.join().map_err(|_| anyhow::anyhow!("Codex stderr worker panicked"))??;
    let status = status?;
    ensure!(out.len() <= LIMIT as usize && err.len() <= LIMIT as usize, "Codex output exceeded the limit");
    ensure!(status.success(), "Codex teacher exited with {status}: {}", String::from_utf8_lossy(&err).chars().take(600).collect::<String>());
    written.context("write Codex teacher prompt")?;
    let mut reply = decode_events(std::str::from_utf8(&out).context("Codex output is not UTF-8")?)?;
    reply["model"] = json!(model);
    reply["secs"] = json!(start.elapsed().as_secs_f64());
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(cwd.join("codex-calls.jsonl"))?;
    writeln!(log, "{}", json!({"provider":"codex", "model":model, "effort":effort,
        "secs":reply["secs"], "usage":reply["usage"], "cost_reported":false}))?;
    Ok(reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    const GOOD: &str = "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"1 OK\"}}\n{\"type\":\"turn.completed\",\"usage\":{\"input_tokens\":10,\"cached_input_tokens\":3,\"output_tokens\":2}}\n";
    const BUSY: &str = "{\"type\":\"thread.started\",\"thread_id\":\"t\"}\n{\"type\":\"turn.started\"}\n{\"type\":\"error\",\"message\":\"Selected model is at capacity. Please try a different model.\"}\n{\"type\":\"turn.failed\",\"error\":{\"message\":\"Selected model is at capacity. Please try a different model.\"}}\n";
    #[test] fn provider_busy_is_only_an_exact_capacity_refusal_before_any_item() {
        assert_eq!(provider_busy(BUSY), Some(PROVIDER_BUSY_MESSAGES[0]));
        assert!(decode_events(BUSY).is_err());
        // Red controls: usage limit, partial output, completion, empty, mixed and paraphrased messages.
        let limit = BUSY.replace("Selected model is at capacity. Please try a different model.",
            "Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.");
        assert_eq!(provider_busy(&limit), None);
        let item = "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"x\"}}\n";
        let partial = BUSY.replacen("{\"type\":\"error\"", &format!("{item}{{\"type\":\"error\""), 1);
        assert_eq!(provider_busy(&partial), None);
        assert_eq!(provider_busy(&format!("{BUSY}{GOOD}")), None);
        assert_eq!(provider_busy(""), None);
        assert_eq!(provider_busy("{\"type\":\"thread.started\"}\n{\"type\":\"turn.started\"}\n"), None);
        assert_eq!(provider_busy(&BUSY.replacen("at capacity.", "at capacity!", 1)), None);
        assert_eq!(provider_busy(&format!("{BUSY}not json\n")), None);
        assert_eq!(provider_busy("{\"type\":\"turn.failed\"}\n"), None);
        for m in PROVIDER_BUSY_MESSAGES {
            assert_eq!(provider_busy(&format!("{}\n", json!({"type":"turn.failed","error":{"message":m}}))), Some(m));
        }
    }
    #[test] fn completed_turn_keeps_answer_and_usage() {
        let r = decode_events(GOOD).unwrap();
        assert_eq!(r["result"], "1 OK");
        assert_eq!(r["usage"]["cache_read_input_tokens"], 3);
        assert_eq!(r["cost_reported"], false);
    }
    #[test] fn incomplete_failed_and_tool_turns_are_rejected() {
        assert!(decode_events(GOOD.lines().next().unwrap()).is_err());
        assert!(decode_events(&format!("{GOOD}{{\"type\":\"turn.failed\"}}")).is_err());
        assert!(decode_events(&format!("{{\"type\":\"item.completed\",\"item\":{{\"type\":\"command_execution\"}}}}\n{GOOD}")).is_err());
        assert!(decode_events("not json").is_err());
    }
    // Reduced Fish r3 t04 w00-p000522 sequence (2026-11-05); the answer is a synthetic stub.
    const RECONNECT: &str = "Reconnecting... 2/5 (stream disconnected before completion: WebSocket protocol error: Connection reset without closing handshake)";
    fn reconnect_trace(tail: &[Value]) -> String {
        let mut t = vec![json!({"type":"thread.started","thread_id":"t"}), json!({"type":"turn.started"}),
            json!({"type":"error","message":RECONNECT})];
        t.extend_from_slice(tail);
        t.iter().map(Value::to_string).collect::<Vec<_>>().join("\n")
    }
    #[test] fn reconnect_notice_is_transient_only_before_a_completed_turn() {
        let answer = json!({"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"{\"stub\":1}"}});
        let done = json!({"type":"turn.completed","usage":{"input_tokens":32304,"cached_input_tokens":32128,"output_tokens":8225}});
        let ok = reconnect_trace(&[answer.clone(), done.clone()]);
        assert_eq!(transient_reconnect(RECONNECT), Some((2, 5)));
        assert_eq!(decode_events(&ok).unwrap()["result"], "{\"stub\":1}");
        assert_eq!(transient_reconnects(&ok), 1);
        assert_eq!(provider_busy(&ok), None);
        // Red controls: failure after reconnect, no completion, no answer, reconnect after completion.
        let failed = json!({"type":"turn.failed","error":{"message":"stream disconnected before completion"}});
        assert!(decode_events(&reconnect_trace(&[answer.clone(), failed.clone()])).is_err());
        assert!(decode_events(&reconnect_trace(&[failed])).is_err());
        assert!(decode_events(&reconnect_trace(&[answer.clone()])).is_err());
        assert!(decode_events(&reconnect_trace(&[])).is_err());
        assert!(decode_events(&reconnect_trace(&[done.clone()])).is_err());
        let late = json!({"type":"error","message":RECONNECT});
        assert!(decode_events(&reconnect_trace(&[answer.clone(), done.clone(), late])).is_err());
        // Limits, quota, capacity and other error texts stay fatal even when the turn completes.
        for m in ["Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.",
            PROVIDER_BUSY_MESSAGES[0], "Reconnecting... waiting for network",
            "Reconnecting... 1/5 (usage limit reached)", "Reconnecting... 1/5 (exceeded quota)",
            "Reconnecting... 6/5 (stream disconnected)", "Reconnecting... 0/5 (stream disconnected)",
            "Reconnecting... 1/5 ()", "Reconnecting... 1/5 (x", "Reconnecting... +1/5 (x)",
            "Reconnecting...  1/5 (x)", "reconnecting... 1/5 (x)", "Reconnecting... 1/5 (x)\n",
            "Reconnecting... 1/ (x)", "Reconnecting... 1/5 (x) "] {
            let t = ok.replace(&json!(RECONNECT).to_string(), &json!(m).to_string());
            assert_ne!(t, ok);
            assert!(decode_events(&t).is_err(), "{m:?}");
            assert_eq!(transient_reconnects(&t), 0, "{m:?}");
        }
        // The notice is accepted only as an `error` event, not as a turn failure.
        let as_failure = ok.replace(&json!({"type":"error","message":RECONNECT}).to_string(),
            &json!({"type":"turn.failed","error":{"message":RECONNECT}}).to_string());
        assert_ne!(as_failure, ok);
        assert!(decode_events(&as_failure).is_err());
        let twice = reconnect_trace(&[json!({"type":"error","message":"Reconnecting... 3/5 (x)"}), answer, done]);
        assert_eq!(decode_events(&twice).unwrap()["usage"]["output_tokens"], 8225);
        assert_eq!(transient_reconnects(&twice), 2);
    }
    // Whole-trace positional oracle, independent of decoder register updates.
    fn trace_oracle(trace: &[usize]) -> Option<(&'static str, u64)> {
        let positions = |symbol| trace.iter().enumerate()
            .filter_map(move |(i, &s)| (s == symbol).then_some(i)).collect::<Vec<_>>();
        if trace.iter().any(|s| matches!(s, 9 | 10)) { return None; }
        let completions: Vec<_> = trace.iter().enumerate()
            .filter(|(_, s)| matches!(s, 6 | 7)).map(|(i, _)| i).collect();
        if completions.len() != 1 { return None; }
        let end = completions[0];
        if positions(12).iter().any(|&i| i > end) { return None; }
        let items: Vec<_> = trace.iter().enumerate()
            .filter(|(_, s)| matches!(s, 2..=5)).map(|(i, _)| i).collect();
        if items.iter().any(|&i| i > end) { return None; }
        let headers = [positions(0), positions(1)];
        if headers.iter().any(|v| v.len() > 1) { return None; }
        for h in headers.iter().flatten() {
            if *h > end || items.iter().any(|i| i < h) { return None; }
        }
        if let (Some(thread), Some(turn)) = (headers[0].first(), headers[1].first())
            && thread > turn { return None; }
        let last = trace[..end].iter().rfind(|s| matches!(s, 2..=4))?;
        let answer = match last { 2 => "A", 3 => "B", _ => return None };
        Some((answer, if trace[end] == 6 { 10 } else { 20 }))
    }

    #[test]
    fn finite_trace_oracle_preserves_turn_identity() {
        let alphabet = [
            json!({"type":"thread.started","thread_id":"fixture"}),
            json!({"type":"turn.started"}),
            json!({"type":"item.completed","item":{"type":"agent_message","text":"A"}}),
            json!({"type":"item.completed","item":{"type":"agent_message","text":"B"}}),
            json!({"type":"item.completed","item":{"type":"agent_message","text":null}}),
            json!({"type":"item.completed","item":{"type":"reasoning","text":"ignored"}}),
            json!({"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":2}}),
            json!({"type":"turn.completed","usage":{"input_tokens":20,"output_tokens":2}}),
            json!({"type":"future.metadata"}),
            json!({"type":"error"}),
            json!({"type":"item.updated","item":{"type":"command_execution"}}),
            json!(null),
            json!({"type":"error","message":"Reconnecting... 1/5 (stream disconnected)"}),
        ];
        let lines: Vec<_> = alphabet.iter().map(Value::to_string).collect();
        let mut checked = 0;
        let mut fibers = std::collections::BTreeMap::<(String, u64), usize>::new();
        for length in 0..=5 {
            for mut code in 0..13usize.pow(length) {
                let mut trace = vec![0; length as usize];
                for symbol in &mut trace { *symbol = code % 13; code /= 13; }
                let input = trace.iter().map(|&s| lines[s].as_str()).collect::<Vec<_>>().join("\n");
                let actual = decode_events(&input);
                match trace_oracle(&trace) {
                    None => assert!(actual.is_err(), "unexpected success: {input}"),
                    Some((answer, input_tokens)) => {
                        let actual = actual.unwrap_or_else(|e| panic!("{input}: {e}"));
                        assert_eq!(actual, json!({"result":answer,"is_error":false,
                            "provider":"codex","total_cost_usd":0.0,"cost_reported":false,
                            "usage":{"input_tokens":input_tokens,"output_tokens":2,
                                "cache_read_input_tokens":0,"cache_creation_input_tokens":0}}));
                        *fibers.entry((answer.into(), input_tokens)).or_default() += 1;
                    }
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 402234);
        assert_eq!(fibers.len(), 4);
        assert!(fibers.values().all(|&count| count > 1));
        println!("trace_cases={checked}; reachable_fibers={fibers:?}");
    }

    #[test]
    fn projection_preimages_and_rejected_accounting() {
        let prefix = json!({"type":"item.completed","item":{"type":"agent_message","text":" padded "}});
        let mut preimages = 0;
        for cache in [Value::Null, json!(-1), json!("bad"), json!(0), json!(3), json!(99)] {
            for ignored in [json!(null), json!({"type":"future","payload":1}), json!({})] {
                let usage = json!({"input_tokens":10,"output_tokens":2,
                    "cached_input_tokens":cache,"discarded":"metadata"});
                let trace = format!("{ignored}\n{prefix}\n{}\n{ignored}\n",
                    json!({"type":"turn.completed","usage":usage}));
                let result = decode_events(&trace).unwrap();
                assert_eq!(result["result"], " padded ");
                assert_eq!(result["usage"]["cache_read_input_tokens"], cache.as_u64().unwrap_or(0));
                assert!(result["usage"].get("discarded").is_none());
                if cache.as_u64().unwrap_or(0) == 0 { preimages += 1; }
            }
        }
        assert_eq!(preimages, 12);
        for field in ["input_tokens", "output_tokens"] {
            for invalid in [Value::Null, json!(-1), json!("2"), json!(1.5)] {
                let mut usage = json!({"input_tokens":10,"output_tokens":2});
                usage[field] = invalid;
                assert!(decode_events(&format!("{prefix}\n{}",
                    json!({"type":"turn.completed","usage":usage}))).is_err());
            }
        }
        for event in ["item.started", "item.updated", "item.completed"] {
            for kind in [Value::Null, json!("command_execution"), json!("web_search")] {
                let item = json!({"type":event,"item":{"type":kind}});
                assert!(decode_events(&format!("{item}\n{GOOD}")).is_err());
            }
        }
    }

    #[test]
    fn realistic_lifecycle_and_partial_messages() {
        let trace = [
            json!({"type":"thread.started","thread_id":"fixture"}),
            json!({"type":"turn.started"}),
            json!({"type":"item.started","item":{"id":"r","type":"reasoning","text":""}}),
            json!({"type":"item.updated","item":{"id":"r","type":"reasoning","text":"partial"}}),
            json!({"type":"item.completed","item":{"id":"r","type":"reasoning","text":"reason"}}),
            json!({"type":"item.completed","item":{"id":"a","type":"agent_message","text":"progress"}}),
            json!({"type":"item.started","item":{"id":"b","type":"agent_message","text":""}}),
            json!({"type":"item.updated","item":{"id":"b","type":"agent_message","text":"partial"}}),
            json!({"type":"item.completed","item":{"id":"b","type":"agent_message","text":"final"}}),
            json!({"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":2}}),
        ].iter().map(Value::to_string).collect::<Vec<_>>().join("\n");
        assert_eq!(decode_events(&trace).unwrap()["result"], "final");
        assert!(decode_events(&format!("{trace}\n{trace}")).is_err());
        for suffix in [json!({"type":"error"}), json!({"type":"turn.failed"})] {
            assert!(decode_events(&format!("{trace}\n{suffix}")).is_err());
        }
        assert!(decode_events(&format!("{trace}\nnot-json")).is_err());
    }

    #[test] fn old_teacher_models_are_rejected() {
        for name in ["claude-haiku-5-5", "opus", ""] { assert!(validate_model(name).is_err()); }
        validate_model(DEFAULT_MODEL).unwrap();
    }
    #[test] fn command_uses_stdin_isolation_and_subscription_auth() {
        let c = command("codex", Path::new("/tmp"), DEFAULT_MODEL, "medium");
        let args: Vec<_> = c.get_args().map(|x| x.to_string_lossy().to_string()).collect();
        assert!(args.iter().any(|x| x == "forced_login_method=\"chatgpt\""));
        assert!(args.iter().any(|x| x == "--ignore-user-config"));
        assert_eq!(args.last().unwrap(), "-");
        assert_eq!(c.get_program(), "codex");
        assert_eq!(c.get_current_dir(), Some(Path::new("/tmp")));
        for pair in [
            ["--sandbox", "read-only"], ["--color", "never"],
            ["--model", DEFAULT_MODEL], ["-c", "approval_policy=\"never\""],
            ["-c", "web_search=\"disabled\""], ["-c", "project_doc_max_bytes=0"],
            ["-c", "model_reasoning_effort=\"medium\""],
        ] { assert!(args.windows(2).any(|w| w == pair)); }
        for flag in ["--ignore-rules", "--ephemeral", "--skip-git-repo-check", "--json"] {
            assert!(args.iter().any(|x| x == flag));
        }
        let disabled: std::collections::BTreeSet<_> = args.windows(2)
            .filter(|w| w[0] == "--disable").map(|w| w[1].as_str()).collect();
        assert_eq!(disabled, ["shell_tool", "multi_agent", "apps", "plugins", "hooks",
            "browser_use", "browser_use_external", "computer_use", "image_generation",
            "view_image", "memories", "skill_search", "unbounded_connection_retries"]
            .into_iter().collect());
        for key in ["ANTHROPIC_API_KEY", "OPENAI_API_KEY", "CODEX_API_KEY", "OPENAI_BASE_URL"] {
            assert!(c.get_envs().any(|(k,v)| k == key && v.is_none()));
        }
    }
}
