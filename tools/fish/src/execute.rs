//! Exclusive, bounded, review-only Claude (Sonnet) or Codex (Terra) CLI execution of
//! source-bound Fish requests. Terra uses the shared isolated `teacher::exec_args` transport.
//! Reservation means an attempt may have been sent: no replay of an existing directory.
use crate::annotation::{self, Adapter, AnnotationDraft, Family, Profile, Task, Use};
use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::{
        fs::{DirBuilderExt, OpenOptionsExt},
        process::CommandExt,
    },
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MAX_OUTPUT: u64 = 2_097_152;
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
fn synced(path: &Path, bytes: &[u8]) -> Result<File> {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(file)
}
fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}
fn bounded_output(path: &Path) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_OUTPUT + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= MAX_OUTPUT, "CLI output exceeds bound");
    Ok(bytes)
}
/// Parse the Claude JSON envelope, then the strict model schema.
/// Unknown envelope fields remain in raw.stdout; only result is interpreted.
fn checked_envelope(bytes: &[u8]) -> Result<Value> {
    let envelope: Value = serde_json::from_slice(bytes).context("Claude JSON envelope")?;
    ensure!(
        envelope["type"] == "result" && envelope["is_error"] == false,
        "Claude CLI returned an error or non-result envelope"
    );
    Ok(envelope)
}
/// Codex JSONL events decoded by the shared teacher decoder into the same result envelope.
fn codex_envelope(bytes: &[u8]) -> Result<Value> {
    let text = std::str::from_utf8(bytes).context("Codex JSONL is not UTF-8")?;
    let mut envelope = teacher::decode_events(text)?;
    envelope["type"] = json!("result");
    Ok(envelope)
}
fn envelope_for(family: &Family, bytes: &[u8]) -> Result<Value> {
    match family {
        Family::Terra => codex_envelope(bytes),
        _ => checked_envelope(bytes),
    }
}
fn response(family: &Family, bytes: &[u8]) -> Result<AnnotationDraft> {
    let envelope = envelope_for(family, bytes)?;
    if let Some(structured) = envelope.get("structured_output") {
        ensure!(
            structured.is_object(),
            "Claude structured_output must be an object"
        );
        return serde_json::from_value(structured.clone())
            .context("strict bound structured annotation reply JSON");
    }
    // Explicit compatibility branch for a strict JSON result string. No fences/prose repair.
    let result = envelope["result"]
        .as_str()
        .context("Claude result string absent")?;
    serde_json::from_str(result).context("strict bound annotation reply JSON")
}
fn group_absent(pgid: u32) -> bool {
    let result = unsafe { libc::kill(-(pgid as i32), 0) };
    result == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}
fn terminate(child: &mut std::process::Child) -> bool {
    // The child starts in a fresh process group. Terminate only that group.
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    child.wait().is_ok()
}

pub fn run(
    task: &Task,
    profile: &Profile,
    out: &Path,
    deadline: u64,
    timeout: u64,
) -> Result<Value> {
    run_bound(task, profile, out, deadline, timeout, None, None)
}

fn run_bound(
    task: &Task,
    profile: &Profile,
    out: &Path,
    deadline: u64,
    timeout: u64,
    task_file_sha256: Option<&str>,
    profile_file_sha256: Option<&str>,
) -> Result<Value> {
    ensure!(
        matches!(profile.family, Family::Sonnet | Family::Terra)
            && matches!(profile.adapter, Adapter::Cli),
        "annotation-execute requires a Sonnet or Terra (Codex) CLI profile; Haiku is preparation-only"
    );
    ensure!(
        !matches!(task.dataset_use, Use::Evaluation),
        "evaluation dataset use is excluded from annotation execution"
    );
    ensure!(
        (1..=900).contains(&timeout),
        "timeout must be 1..900 seconds"
    );
    let executable = profile
        .executable
        .as_deref()
        .context("explicit executable required")?;
    ensure!(
        Path::new(executable).is_absolute(),
        "absolute CLI executable required"
    );
    let terra = matches!(profile.family, Family::Terra);
    let model = profile
        .model
        .as_deref()
        .unwrap_or(if terra { "gpt-6.1-terra" } else { "sonnet" });
    if terra {
        teacher::validate_model(model).context("Terra model required; no provider fallback")?;
    } else {
        ensure!(
            model == "sonnet" || model.starts_with("claude-sonnet-"),
            "Sonnet model required; no provider fallback"
        );
    }
    ensure!(
        now()? < deadline,
        "annotation deadline has passed; CLI not started"
    );
    let evaluation = annotation::evaluate(task)?;
    let request = annotation::render(task, &evaluation, profile)?;
    let prompt = request["stdin_or_terminal_text"]
        .as_str()
        .context("rendered prompt absent")?;
    ensure!(
        prompt.len() <= annotation::MAX_RENDERED,
        "rendered request exceeds 100000 UTF-8 bytes"
    );
    // Terra: the rendered request stays the native contract record; the actual transport is the
    // shared isolated Codex exec (stdin prompt, JSONL events, no tools/sandbox writes).
    let args: Vec<String> = if terra {
        teacher::exec_args(model, "medium")
    } else {
        request["argv"]
            .as_array()
            .context("rendered argv")?
            .iter()
            .map(|v| v.as_str().map(String::from).context("non-string argv"))
            .collect::<Result<_>>()?
    };
    fs::DirBuilder::new()
        .mode(0o700)
        .create(out)
        .context("new exclusive output directory required; existing attempts never replayed")?;
    // Durably reserve before any subprocess. An orphaned reservation needs operator review.
    annotation::write_new(
        &out.join("reservation.json"),
        &json!({
            "schema":1,"task_id":task.id,"task_sha256":evaluation.task_sha256,
            "task_file_sha256":task_file_sha256,
            "profile_file_sha256":profile_file_sha256,
            "profile_sha256":digest(&serde_json::to_vec(profile)?),
            "request_sha256":request["rendered_sha256"],"deadline_unix":deadline,
            "timeout_seconds":timeout,"review_only":true,"training_authorized":false,
            "state":"reserved_may_be_sent","replay_authorized":false,
            "implementation_bindings":evaluation.implementation_bindings,
            "transport_argv":args,
            "executor_sha256":digest(include_bytes!("execute.rs"))
        }),
    )?;
    sync_dir(out)?;
    let mut status = "prelaunch_failed";
    let mut raw_hashes = json!({});
    let mut validation = Value::Null;
    let mut unresolved_count = Value::Null;
    let mut exit_code = Value::Null;
    let mut model_process_started = false;
    let mut model_process_reaped = false;
    let mut model_process_group_absent = false;
    let mut provider_busy_message = Value::Null;
    let mut transient_reconnects = Value::Null;
    let outcome: Result<()> = (|| {
        annotation::write_new(&out.join("evaluation.json"), &evaluation)?;
        annotation::write_new(&out.join("teacher-request.json"), &request)?;
        synced(&out.join("prompt.txt"), prompt.as_bytes())?;
        let stdout = synced(&out.join("raw.stdout"), b"")?;
        let stderr = synced(&out.join("raw.stderr"), b"")?;
        sync_dir(out)?;
        // Check again after rendering and durable writes, immediately before launch.
        ensure!(
            now()? < deadline,
            "annotation deadline passed before launch; CLI not started"
        );
        let mut cmd = Command::new(executable);
        cmd.args(&args)
            .current_dir(fs::canonicalize(out)?)
            .stdin(Stdio::from(File::open(out.join("prompt.txt"))?))
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .process_group(0);
        // Preserve subscription OAuth/session authentication, not API/provider overrides.
        for key in [
            "ANTHROPIC_API_KEY",
            "ANTHROPIC_AUTH_TOKEN",
            "ANTHROPIC_BASE_URL",
            "CLAUDE_CODE_USE_BEDROCK",
            "CLAUDE_CODE_USE_VERTEX",
            "CLAUDE_CODE_USE_FOUNDRY",
        ]
        .into_iter()
        .chain(teacher::CLEARED_ENV)
        {
            cmd.env_remove(key);
        }
        status = "spawn_failed";
        let mut child = cmd.spawn().context("launch configured Claude CLI")?;
        model_process_started = true;
        let pgid = child.id();
        status = "interrupted_unknown";
        let start = Instant::now();
        let result = loop {
            match child.try_wait() {
                Ok(Some(result)) => {
                    model_process_reaped = true;
                    model_process_group_absent = group_absent(pgid);
                    break result;
                }
                Ok(None) => {}
                Err(error) => {
                    model_process_reaped = terminate(&mut child);
                    model_process_group_absent = model_process_reaped && group_absent(pgid);
                    return Err(error.into());
                }
            }
            if start.elapsed() >= Duration::from_secs(timeout) {
                model_process_reaped = terminate(&mut child);
                model_process_group_absent = model_process_reaped && group_absent(pgid);
                status = "timed_out";
                bail!("CLI timed out; attempt must not replay");
            }
            if ["raw.stdout", "raw.stderr"].iter().any(|p| {
                fs::metadata(out.join(p))
                    .map(|m| m.len() > MAX_OUTPUT)
                    .unwrap_or(true)
            }) {
                model_process_reaped = terminate(&mut child);
                model_process_group_absent = model_process_reaped && group_absent(pgid);
                status = "output_limit";
                bail!("CLI output limit; attempt must not replay");
            }
            thread::sleep(Duration::from_millis(50));
        };
        exit_code = json!(result.code());
        File::open(out.join("raw.stdout"))?.sync_all()?;
        File::open(out.join("raw.stderr"))?.sync_all()?;
        let raw = bounded_output(&out.join("raw.stdout"))?;
        let errors = bounded_output(&out.join("raw.stderr"))?;
        raw_hashes = json!({"stdout_sha256":digest(&raw),"stderr_sha256":digest(&errors)});
        status = "child_failed";
        // Terra: an exact provider capacity refusal before any item is "provider busy, nothing
        // done". It remains a durable non-replayed attempt; the supervisor may requeue the job.
        if terra && !result.success() {
            if let Some(message) = std::str::from_utf8(&raw).ok().and_then(teacher::provider_busy) {
                status = "provider_busy";
                provider_busy_message = json!(message);
                bail!("Codex provider busy before any work: {message}");
            }
        }
        ensure!(result.success(), "Claude CLI exited unsuccessfully");
        status = "transport_error";
        envelope_for(&profile.family, &raw)?;
        status = "invalid_response";
        let draft = response(&profile.family, &raw)?;
        // Preserve a parsed draft even when native validation rejects it.
        annotation::write_new(&out.join("reply.json"), &draft)?;
        unresolved_count = json!(draft.unresolved.len());
        validation = annotation::validate_draft(task, &evaluation, &draft)?;
        status = "validated_review_only";
        Ok(())
    })();
    // Preserve and sync diagnostic bytes on every terminal path, including timeout.
    for name in ["raw.stdout", "raw.stderr"] {
        if let Ok(file) = File::open(out.join(name)) {
            let _ = file.sync_all();
        }
    }
    if let (Ok(raw), Ok(errors)) = (
        bounded_output(&out.join("raw.stdout")),
        bounded_output(&out.join("raw.stderr")),
    ) {
        raw_hashes = json!({"stdout_sha256":digest(&raw),"stderr_sha256":digest(&errors)});
        // Observability only: Codex stream-retry notices seen, on every terminal path.
        if terra {
            transient_reconnects =
                json!(teacher::transient_reconnects(&String::from_utf8_lossy(&raw)));
        }
    }
    let receipt = json!({
        "schema":1,"task_id":task.id,"task_sha256":evaluation.task_sha256,
            "task_file_sha256":task_file_sha256,
            "profile_file_sha256":profile_file_sha256,
            "profile_sha256":digest(&serde_json::to_vec(profile)?),
        "status":status,"review_only":true,"training_authorized":false,
        "semantic_correctness":"not established","request_sha256":request["rendered_sha256"],
        "unresolved_count":unresolved_count,"native_findings":validation,
        "raw_output_hashes":raw_hashes,"exit_code":exit_code,
        "provider_busy_message":provider_busy_message,
        "transient_reconnects":transient_reconnects,
        "model_process_started":model_process_started,
        "model_process_reaped":model_process_reaped,
        "model_process_group_absent":model_process_group_absent,
        "error":outcome.as_ref().err().map(|e|format!("{e:#}")),
        "replay_authorized":false,"finished_unix":now()?
    });
    annotation::write_new(&out.join("receipt.json"), &receipt)?;
    sync_dir(out)?;
    outcome?;
    Ok(receipt)
}

pub fn dispatch(args: &[OsString]) -> Result<()> {
    ensure!(
        (5..=6).contains(&args.len()),
        "annotation-execute TASK_JSON PROFILE_JSON NEW_DIR DEADLINE_UNIX [TIMEOUT_SECONDS]"
    );
    let task_bytes = annotation::read(Path::new(&args[1]))?;
    let task: Task = serde_json::from_slice(&task_bytes)?;
    let task_file_sha256 = digest(&task_bytes);
    let profile_bytes = annotation::read(Path::new(&args[2]))?;
    let profile: Profile = serde_json::from_slice(&profile_bytes)?;
    let profile_file_sha256 = digest(&profile_bytes);
    let deadline = args[4].to_str().context("UTF-8 deadline")?.parse()?;
    let timeout = args
        .get(5)
        .map(|s| {
            s.to_str()
                .context("UTF-8 timeout")?
                .parse()
                .context("integer timeout")
        })
        .transpose()?
        .unwrap_or(300);
    let receipt = run_bound(
        &task,
        &profile,
        Path::new(&args[3]),
        deadline,
        timeout,
        Some(&task_file_sha256),
        Some(&profile_file_sha256),
    )?;
    println!("{}", serde_json::to_string(&receipt)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    fn setup(body: &str) -> (std::path::PathBuf, Profile) {
        let root = std::env::temp_dir().join(format!(
            "fish-execute-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let cli = root.join("fake-claude");
        fs::write(&cli, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&cli, fs::Permissions::from_mode(0o700)).unwrap();
        let profile = Profile {
            family: Family::Sonnet,
            adapter: Adapter::Cli,
            executable: Some(cli.to_string_lossy().into_owned()),
            model: Some("sonnet".into()),
        };
        (root, profile)
    }
    fn envelope(task: &Task) -> Vec<u8> {
        let evaluation = annotation::evaluate(task).unwrap();
        let draft = AnnotationDraft {
            schema: 1,
            task_id: task.id.clone(),
            task_sha256: evaluation.task_sha256,
            ast_conllu: task.candidates[0].ast.clone(),
            pragmatics_tsv: task.candidates[0].pragmatics.clone(),
            unresolved: vec!["speaker remains unobserved".into()],
        };
        serde_json::to_vec(&json!({"type":"result","is_error":false,
            "result":serde_json::to_string(&draft).unwrap(),"modelUsage":{"test-only":{}}}))
        .unwrap()
    }
    #[test]
    fn source_bound_reply_preserves_unknowns_and_never_authorizes_training() {
        let mut task = annotation::tests::task();
        task.dataset_use = Use::Unknown;
        let (root, profile) =
            setup("printf '%s\\n' \"$@\" > ../argv; cat > ../submitted; cat ../response.json");
        fs::write(root.join("response.json"), envelope(&task)).unwrap();
        let out = root.join("attempt");
        let receipt = run(&task, &profile, &out, now().unwrap() + 60, 2).unwrap();
        assert_eq!(receipt["status"], "validated_review_only");
        assert_eq!(receipt["unresolved_count"], 1);
        assert_eq!(receipt["training_authorized"], false);
        assert_eq!(receipt["model_process_started"], true);
        assert_eq!(receipt["model_process_reaped"], true);
        assert_eq!(receipt["model_process_group_absent"], true);
        assert!(receipt["native_findings"]["input_findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["code"] == "not_teaching"));
        assert!(out.join("raw.stdout").is_file() && out.join("reply.json").is_file());
        assert!(!out.join("gpu-admission.json").exists());
        assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err());
        let prompt = fs::read_to_string(out.join("prompt.txt")).unwrap();
        assert_eq!(fs::read(root.join("submitted")).unwrap(), prompt.as_bytes());
        let submitted_args = fs::read_to_string(root.join("argv")).unwrap();
        assert!(submitted_args.contains("--safe-mode\n--tools\n\n--disable-slash-commands\n--effort\nmedium\n--no-session-persistence\n--output-format\njson\n"));
        assert_eq!(
            json!(prompt.len()),
            serde_json::from_slice::<Value>(&fs::read(out.join("teacher-request.json")).unwrap())
                .unwrap()["rendered_utf8_bytes"]
        );
        fs::remove_dir_all(root).unwrap();
    }
    fn codex_events(task: &Task, extra: &str) -> Vec<u8> {
        let v: Value = serde_json::from_slice(&envelope(task)).unwrap();
        let msg = json!({"type":"item.completed","item":{"type":"agent_message","text":v["result"]}});
        format!(
            "{{\"type\":\"thread.started\"}}\n{{\"type\":\"turn.started\"}}\n{extra}{msg}\n{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":9,\"output_tokens\":3}}}}\n"
        )
        .into_bytes()
    }
    #[test]
    fn terra_uses_isolated_codex_transport_and_shared_event_decoder() {
        let mut task = annotation::tests::task();
        task.dataset_use = Use::Unknown;
        let (root, mut profile) =
            setup("printf '%s\\n' \"$@\" > ../argv; cat > ../submitted; cat ../response.jsonl");
        profile.family = Family::Terra;
        profile.model = Some("gpt-6.1-terra".into());
        fs::write(root.join("response.jsonl"), codex_events(&task, "")).unwrap();
        let out = root.join("terra");
        let receipt = run(&task, &profile, &out, now().unwrap() + 60, 2).unwrap();
        assert_eq!(receipt["status"], "validated_review_only");
        assert_eq!(receipt["training_authorized"], false);
        let argv = fs::read_to_string(root.join("argv")).unwrap();
        let expected = teacher::exec_args("gpt-6.1-terra", "medium").join("\n") + "\n";
        assert_eq!(argv, expected);
        assert!(argv.contains("--sandbox\nread-only\n") && argv.contains("--disable\nshell_tool\n"));
        let prompt = fs::read_to_string(out.join("prompt.txt")).unwrap();
        assert_eq!(fs::read(root.join("submitted")).unwrap(), prompt.as_bytes());
        let reservation: Value =
            serde_json::from_slice(&fs::read(out.join("reservation.json")).unwrap()).unwrap();
        assert_eq!(reservation["transport_argv"][0], "exec");
        // A tool-use event cannot become an annotation, even with a valid final message.
        let tool = "{\"type\":\"item.completed\",\"item\":{\"type\":\"command_execution\"}}\n";
        fs::write(root.join("response.jsonl"), codex_events(&task, tool)).unwrap();
        let bad = root.join("terra-tool");
        assert!(run(&task, &profile, &bad, now().unwrap() + 60, 2).is_err());
        let r: Value = serde_json::from_slice(&fs::read(bad.join("receipt.json")).unwrap()).unwrap();
        assert_eq!(r["status"], "transport_error");
        // No provider fallback: a Claude model name under the Terra family is refused pre-launch.
        profile.model = Some("sonnet".into());
        assert!(run(&task, &profile, &root.join("terra-model"), now().unwrap() + 60, 2).is_err());
        assert!(!root.join("terra-model").exists());
        // Sol is selectable through the same Codex family.
        profile.model = Some("gpt-6.1-sol".into());
        fs::write(root.join("response.jsonl"), codex_events(&task, "")).unwrap();
        let sol = root.join("sol");
        assert!(run(&task, &profile, &sol, now().unwrap() + 60, 2).is_ok());
        // Haiku remains a preparation profile only; execution refuses it before launch.
        profile.family = Family::Haiku;
        profile.model = Some("haiku".into());
        assert!(run(&task, &profile, &root.join("haiku"), now().unwrap() + 60, 2).is_err());
        assert!(!root.join("haiku").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn terra_stream_reconnect_before_a_completed_turn_is_not_a_failure() {
        // Reduced Fish r3 t04 w00-p000522 sequence (2026-11-05): reconnect notice, answer,
        // completion. Red controls: turn.failed after it, no completion, usage limit.
        let task = annotation::tests::task();
        let (root, mut profile) = setup("cat >/dev/null; cat ../response.jsonl");
        profile.family = Family::Terra;
        profile.model = Some("gpt-6.1-terra".into());
        let notice = "{\"type\":\"error\",\"message\":\"Reconnecting... 2/5 (stream disconnected before completion: WebSocket protocol error: Connection reset without closing handshake)\"}\n";
        let ok = codex_events(&task, notice);
        fs::write(root.join("response.jsonl"), &ok).unwrap();
        let receipt = run(&task, &profile, &root.join("reconnect"), now().unwrap() + 60, 2).unwrap();
        assert_eq!(receipt["status"], "validated_review_only");
        assert_eq!(receipt["transient_reconnects"], 1);
        let text = String::from_utf8(ok).unwrap();
        let completed = text.lines().last().unwrap().to_owned() + "\n";
        let failed = text.replace(&completed,
            "{\"type\":\"turn.failed\",\"error\":{\"message\":\"stream disconnected before completion\"}}\n");
        let incomplete = text.replace(&completed, "");
        let limit = text.replace(notice,
            "{\"type\":\"error\",\"message\":\"Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.\"}\n");
        for (name, raw, reconnects) in [("failed", failed, 1), ("incomplete", incomplete, 1), ("limit", limit, 0)] {
            assert_ne!(raw, text, "{name}");
            fs::write(root.join("response.jsonl"), raw).unwrap();
            let out = root.join(name);
            assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err(), "{name}");
            let r: Value = serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
            assert_eq!(r["status"], "transport_error", "{name}");
            assert_eq!(r["transient_reconnects"], reconnects, "{name}");
            assert!(!out.join("reply.json").exists(), "{name}");
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn terra_capacity_refusal_is_provider_busy_but_partial_output_and_limits_are_not() {
        let task = annotation::tests::task();
        let (root, mut profile) = setup("cat >/dev/null; cat ../response.jsonl; exit 1");
        profile.family = Family::Terra;
        profile.model = Some("gpt-6.1-terra".into());
        let cap = "Selected model is at capacity. Please try a different model.";
        let busy = format!("{{\"type\":\"thread.started\",\"thread_id\":\"t\"}}\n{{\"type\":\"turn.started\"}}\n{{\"type\":\"error\",\"message\":\"{cap}\"}}\n{{\"type\":\"turn.failed\",\"error\":{{\"message\":\"{cap}\"}}}}\n");
        let item = "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"x\"}}\n";
        let limit = busy.replace(cap, "Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.");
        let partial = busy.replacen("{\"type\":\"error\"", &format!("{item}{{\"type\":\"error\""), 1);
        for (name, raw, expected, message) in [
            ("busy", busy.clone(), "provider_busy", json!(cap)),
            ("limit", limit, "child_failed", Value::Null),
            ("partial", partial, "child_failed", Value::Null),
        ] {
            fs::write(root.join("response.jsonl"), raw).unwrap();
            let out = root.join(name);
            assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err());
            let r: Value = serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
            assert_eq!(r["status"], expected, "{name}");
            assert_eq!(r["provider_busy_message"], message, "{name}");
            assert_eq!(r["exit_code"], 1);
            assert_eq!((r["model_process_reaped"].clone(), r["replay_authorized"].clone()), (json!(true), json!(false)));
            assert!(!out.join("reply.json").exists());
        }
        // Sonnet output is never classified through the Codex busy rule.
        profile.family = Family::Sonnet;
        profile.model = Some("sonnet".into());
        fs::write(root.join("response.jsonl"), busy).unwrap();
        let out = root.join("sonnet");
        assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err());
        let r: Value = serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
        assert_eq!(r["status"], "child_failed");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn cli_binds_pretty_task_file_separately_from_typed_task() {
        let task = annotation::tests::task();
        let (root, profile) = setup("cat >/dev/null; cat ../response.json");
        fs::write(root.join("response.json"), envelope(&task)).unwrap();
        let task_bytes = serde_json::to_vec_pretty(&task).unwrap();
        let task_file = root.join("task.json");
        let profile_file = root.join("profile.json");
        fs::write(&task_file, &task_bytes).unwrap();
        fs::write(&profile_file, serde_json::to_vec_pretty(&profile).unwrap()).unwrap();
        let out = root.join("from-cli");
        dispatch(&[
            "annotation-execute".into(),
            task_file.into_os_string(),
            profile_file.into_os_string(),
            out.clone().into_os_string(),
            (now().unwrap() + 60).to_string().into(),
            "2".into(),
        ])
        .unwrap();
        let receipt: Value =
            serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
        let reservation: Value =
            serde_json::from_slice(&fs::read(out.join("reservation.json")).unwrap()).unwrap();
        assert_eq!(receipt["task_file_sha256"], digest(&task_bytes));
        assert_eq!(
            receipt["task_sha256"],
            digest(&serde_json::to_vec(&task).unwrap())
        );
        assert_ne!(receipt["task_file_sha256"], receipt["task_sha256"]);
        assert_eq!(reservation["task_file_sha256"], receipt["task_file_sha256"]);
        assert_eq!(
            receipt["profile_file_sha256"],
            digest(&serde_json::to_vec_pretty(&profile).unwrap())
        );
        assert_eq!(
            receipt["profile_sha256"],
            digest(&serde_json::to_vec(&profile).unwrap())
        );
        assert_eq!(
            reservation["profile_file_sha256"],
            receipt["profile_file_sha256"]
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn structured_output_and_native_utf8_observation_limits_remain_separate() {
        let task = annotation::tests::task();
        let evaluation = annotation::evaluate(&task).unwrap();
        let draft = response(&Family::Sonnet, &envelope(&task)).unwrap();
        let value = serde_json::to_value(&draft).unwrap();
        let fenced = format!(
            "\u{0060}\u{0060}\u{0060}json\n{}\n\u{0060}\u{0060}\u{0060}",
            serde_json::to_string(&draft).unwrap()
        );
        let legacy_bad = json!({"type":"result","is_error":false,"result":fenced});
        assert!(response(&Family::Sonnet, &serde_json::to_vec(&legacy_bad).unwrap()).is_err());
        let structured =
            json!({"type":"result","is_error":false,"result":"","structured_output":value});
        assert!(response(&Family::Sonnet, &serde_json::to_vec(&structured).unwrap()).is_ok());
        let malformed = json!({"type":"result","is_error":false,"result":serde_json::to_string(&draft).unwrap(),"structured_output":"not an object"});
        assert!(response(&Family::Sonnet, &serde_json::to_vec(&malformed).unwrap()).is_err());
        let mut d = draft;
        d.unresolved = vec!["é".repeat(256)];
        assert!(annotation::validate_draft(&task, &evaluation, &d).is_ok());
        d.unresolved = vec!["é".repeat(257)];
        assert!(annotation::validate_draft(&task, &evaluation, &d).is_err());
        d.unresolved = vec!["x".repeat(513)];
        assert!(annotation::validate_draft(&task, &evaluation, &d).is_err());
        d.unresolved = vec!["x".into(); 33];
        assert!(annotation::validate_draft(&task, &evaluation, &d).is_err());
        d.unresolved = vec![" ".into()];
        assert!(annotation::validate_draft(&task, &evaluation, &d).is_err());
    }

    #[test]
    fn provider_and_outer_transport_errors_never_become_quarantinable_drafts() {
        let task = annotation::tests::task();
        let (root, profile) = setup("cat >/dev/null; cat ../response.json");
        for (index, raw) in [
            serde_json::to_vec(
                &json!({"type":"result","is_error":true,"result":"429 quota error"}),
            )
            .unwrap(),
            serde_json::to_vec(
                &json!({"type":"message","is_error":false,"result":"auth required"}),
            )
            .unwrap(),
            b"invalid outer transport".to_vec(),
        ]
        .into_iter()
        .enumerate()
        {
            fs::write(root.join("response.json"), raw).unwrap();
            let out = root.join(format!("transport-{index}"));
            assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err());
            let receipt: Value =
                serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
            assert_eq!(receipt["status"], "transport_error");
            assert_eq!(receipt["model_process_reaped"], true);
            assert_eq!(receipt["model_process_group_absent"], true);
            assert!(!out.join("reply.json").exists());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn process_group_absence_is_observed_not_inferred_from_exit_shape() {
        let mut child = Command::new("sleep")
            .arg("5")
            .process_group(0)
            .spawn()
            .unwrap();
        let pgid = child.id();
        assert!(!group_absent(pgid));
        assert!(terminate(&mut child));
        assert!(group_absent(pgid));
    }

    #[test]
    fn deadline_and_heldout_and_oversize_never_start_cli() {
        let mut task = annotation::tests::task();
        let (root, profile) = setup("touch ../sent; exit 2");
        assert!(run(&task, &profile, &root.join("expired"), 0, 1).is_err());
        task.dataset_use = Use::Evaluation;
        assert!(run(
            &task,
            &profile,
            &root.join("heldout"),
            now().unwrap() + 60,
            1
        )
        .is_err());
        task.dataset_use = Use::Teaching;
        for candidate in &mut task.candidates {
            candidate.ast = format!("# {}\n{}", "x".repeat(50_000), candidate.ast);
            candidate.ast_sha256 = digest(candidate.ast.as_bytes());
        }
        assert!(run(
            &task,
            &profile,
            &root.join("oversize"),
            now().unwrap() + 60,
            1
        )
        .is_err());
        assert!(!root.join("sent").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn malformed_reply_and_timeout_are_durable_nonreplayable_failures() {
        let task = annotation::tests::task();
        let (root,profile)=setup("cat >/dev/null; printf '%s' '{\"type\":\"result\",\"is_error\":false,\"result\":\"{}\"}'");
        let out = root.join("invalid");
        assert!(run(&task, &profile, &out, now().unwrap() + 60, 2).is_err());
        let r: Value =
            serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
        assert_eq!(r["status"], "invalid_response");
        assert_eq!(r["model_process_reaped"], true);
        assert_eq!(r["model_process_group_absent"], true);
        assert!(out.join("raw.stdout").is_file());
        fs::write(
            profile.executable.as_ref().unwrap(),
            "#!/bin/sh\ncat >/dev/null\nsleep 20\n",
        )
        .unwrap();
        let timed = root.join("timed");
        let start = Instant::now();
        assert!(run(&task, &profile, &timed, now().unwrap() + 60, 1).is_err());
        assert!(start.elapsed() < Duration::from_secs(5));
        let r: Value =
            serde_json::from_slice(&fs::read(timed.join("receipt.json")).unwrap()).unwrap();
        assert_eq!(r["status"], "timed_out");
        assert!(!r["replay_authorized"].as_bool().unwrap());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn strict_schema_source_chars_targets_and_canonical_labels_are_gates() {
        let task = annotation::tests::task();
        let evaluation = annotation::evaluate(&task).unwrap();
        let bytes = envelope(&task);
        let mut reply = response(&Family::Sonnet, &bytes).unwrap();
        assert!(annotation::validate_draft(&task, &evaluation, &reply).is_ok());
        reply.pragmatics_tsv = reply
            .pragmatics_tsv
            .replace("\tnarrator\t", "\tcharacter\t");
        let context_check = annotation::validate_draft(&task, &evaluation, &reply).unwrap();
        assert!(context_check["reply_findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["code"] == "speaker_unknown"));
        reply = response(&Family::Sonnet, &bytes).unwrap();
        reply.ast_conllu = reply.ast_conllu.replace("Birds", "Fish");
        assert!(annotation::validate_draft(&task, &evaluation, &reply).is_err());
        reply = response(&Family::Sonnet, &bytes).unwrap();
        reply.pragmatics_tsv = reply.pragmatics_tsv.replace("\tassert\t", "\t ASSERT \t");
        assert!(annotation::validate_draft(&task, &evaluation, &reply).is_err());
        reply = response(&Family::Sonnet, &bytes).unwrap();
        reply.pragmatics_tsv = reply.pragmatics_tsv.lines().next().unwrap().to_string();
        assert!(annotation::validate_draft(&task, &evaluation, &reply).is_err());
        reply = response(&Family::Sonnet, &bytes).unwrap();
        reply.task_sha256 = "0".repeat(64);
        assert!(annotation::validate_draft(&task, &evaluation, &reply).is_err());
        let mut outer: Value = serde_json::from_slice(&bytes).unwrap();
        let mut inner: Value = serde_json::from_str(outer["result"].as_str().unwrap()).unwrap();
        inner["gold"] = json!(true);
        outer["result"] = json!(serde_json::to_string(&inner).unwrap());
        assert!(response(&Family::Sonnet, &serde_json::to_vec(&outer).unwrap()).is_err());
    }
}
