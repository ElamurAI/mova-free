# Fish virtual terminals

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

This standalone Rust supervisor is public Mova worker code. Concrete deployment configuration, tasks and runtime data are supplied externally. Each Fish has one disjoint working directory and one newly launched, explicitly owned tmux session running interactive Codex with the configured `model` (default gpt-6.1-terra, recommended for Fish), medium effort, workspace-write sandbox and no automatic escalation. This supervisor drives only Codex terminals; it is separate from the Sonnet default of Fish request preparation (`tools/fish`). It does not use a Claude API, replace the existing annotation pipeline, move deployed workers, or train/import anything.

The supervisor is an independent OS process. A watch pass checks pending reports and admits at most one task per available Fish. Reports enter run_root/review/ for the root agent to inspect. Acceptance/rejection records a decision only; it is not automatic DB admission, code application or training. No mechanism here injects messages into an inactive root chat.

## Commands

The package, binary, session prefix (`peta-fish-ID`), report directory (`.peta-fish-reports`) and tmux owner option (`@peta-fish-owner`) are fixed protocol identifiers of this supervisor; they do not name an actor.

```sh
CARGO_BUILD_JOBS=2 cargo build --manifest-path tools/fish-terminal/Cargo.toml --release --offline --locked -j2
peta-fish-terminal prepare CONFIG.json
peta-fish-terminal launch CONFIG.json one
peta-fish-terminal once CONFIG.json
peta-fish-terminal watch CONFIG.json
peta-fish-terminal status CONFIG.json
peta-fish-terminal review CONFIG.json task0 accept 'Checked source and qualifications; retain as review-only.'
```

Prepare requires a new run root whose parent already exists, canonical precreated disjoint workdirs, a pinned JSONL task queue and no preexisting .peta-fish-reports directory in any workdir. It never overwrites another run. Launch is separate and refuses an existing launch reservation or tmux session. Monitoring never recreates missing sessions. A terminal launch can show a trust/authentication/modal screen; the controller refuses to type through it. Root must inspect the actual owned session and resolve ordinary authorized setup explicitly. No global authentication or permission setting is changed.

The trusted config fields are schema=1, root, queue, queue_sha256, tmux, codex, optional model (a `gpt-*` Codex id, default gpt-6.1-terra; Claude ids are refused, no provider fallback), fish, max_dispatches, poll_ms, task_timeout_secs, idle_marker and pane_command. fish is an array of {id,session,workdir}; session must be peta-fish-ID; configured session names may not prefix one another. Pool size is 1–8 (2 is the recommended minimum), the lifetime reservation cap 1–1024, poll_ms 100–60000 and timeout 1–86400 seconds. All filesystem paths are absolute. Config and queue identity are frozen by hash. English prompts are an operator requirement; language correctness is not mechanically inferred.

Each JSONL queue row has id, fish, prompt and inputs, where inputs is an array of {path,sha256}. Input files must be canonical regular files inside that Fish's workdir and are rehashed before dispatch and report admission. Prompt length is at most 12000 bytes; queue/config/report bounds are 128 KiB. Report bounds apply to the full JSON, not just narrative fields. Input reads are individually bounded to 128 KiB, at most 32 inputs per task.

## Completion and review protocol

The generated prompt gives the exact report path .peta-fish-reports/NONCE.json inside the Fish workdir. The complete JSON object must contain schema=1, task_id, nonce, input_sha256 and an object result. input_sha256 binds the entire canonical serialized task, including source hashes and prompt. The model is asked to publish by temporary-file rename. A report is considered only after two equal observed idle screens; its identity/schema/hash and unchanged inputs are then checked. Idleness alone never completes a task. Invalid reports are quarantined and preserved.

The admitted raw report and its needs_root_review envelope are written under the run root, with training_allowed=false and automatic_application=false. A repeated same-byte publication is idempotent. review writes a bound accept/reject decision; the next watch/once pass reconciles it and frees that Fish for its next queued task. A rejected completed report is not automatically sent back to the model. Quarantine has no automatic reset/retry command.

STOP under the run root stops watch at the next pass boundary. It does not interrupt an active model or erase evidence. A reserved dispatch discovered on restart is quarantined, including uncertainty around Enter; it is never replayed. The cap counts all reservations, including failures. Local process locks prevent cooperating supervisors from overlapping mutations.

## Terminal adapter and required live smoke

The adapter invokes tmux using argv, a literal load-buffer/paste-buffer sequence and journaled boundaries before effects. It requires a launched local ownership record, matching session metadata, exactly one pane across the session, expected pane command, a configured exact idle-marker line, no recognized busy/modal strings, and two stable screen captures. Screen content is not retained; bounded hashes/command/readiness observations are durable. The marker and pane-command must come from an actual observed Codex release. Defaults in tests are not a certified live UI adapter.

A root smoke should use one fresh Fish workdir, one trivial bound input and a one-task cap; inspect the launched session without sending a task until its idle screen is understood. Run once, observe the durable waiting state, then poll until the bound report enters review. Inspect the report and record a decision. Confirm that a second once does not repeat the completed task.

## Evidence and limits

Native controls in `src/lib.rs` cover the configured Codex model (Terra default, Sol accepted, Claude/malformed ids refused), unknown/foreign/busy/changing UI refusal, one-outstanding-task/review handoff, nonce mismatch, changed inputs, timeout, interrupted Enter/no replay after reload, config pin and report symlink refusal, report publication idempotence, the lifetime cap and real subprocess literal-argv/timeout behavior. The real subprocess control uses printf and sleep, never Codex. See the adjacent source groupants; the prospective state model is in `src/lib.rs.groupant/model.md`.

Assume trusted same-user config/filesystem, cooperative model and terminal, and ordinary sync/rename semantics. Session metadata is not authentication against a hostile local user; UI observation is not semantic process verification. A model can emit a syntactically valid but scientifically wrong report. No crash-durability theorem, universal exactly-once external effects, complete Rust/semantic AST or automatic root review is established. A failed launch/dispatch must be inspected, not reidentified as a fresh successful retry.

The adapter configuration for a live smoke is `pane_command=node` with the exact trimmed idle line `› Ask Codex to do anything` (observed on Codex 0.165.0; another Codex release must be observed again before a live run). Nonmodal /permissions help and weekly-limit warnings are not approval requests; a native control admits that screen while still refusing concrete permission/authentication/trust and busy indicators.

A real launch can show trailing blank viewport rows, and an update dialog that keeps an older idle marker above its choices. The adapter removes only trailing blank padding before selecting the content tail, then separately rejects the Update available/Update now dialog and Enter-continue/setup actions. A trim-only implementation, compiled as a negative control, admits Update now. Neither the modal nor a generic nonmodal weekly warning is an instruction to update software.

## Target and read safety

All session targets use =NAME; pane targets use =NAME:. Missing sessions cannot prefix-match another configured name. Pairwise prefix IDs are additionally refused at load. Task, Config, Job and report serialization, task digests and deterministic nonces do not depend on these checks for admitted nonprefix configs.

Bounded reads open O_NONBLOCK | O_NOFOLLOW and fstat a regular file before read. Report directories must be canonical and nonsymlinked. Writerless FIFOs cannot hold up a read. These checks assume cooperative local state, not hostile concurrent rename/session replacement resistance.

Read-only terminal preflight errors persist terminal_unavailable and skip only that Fish. Changed sources, predating reports or invalid report directories persist refused_before_effect with task ID and reason SHA-256 before reservation, allowing other Fish to proceed. Unavailable waiting-report terminals retain their jobs until deadline. Persistence errors and after-reservation uncertainty still stop the pass and retain journals; restart quarantines without replay.

## Local logging and Crab boundary

watch emits one JSON snapshot per completed pass on stdout; status returns durable jobs and last observations without a fresh capture. observations/FISH.json contains category, time, redacted screen hash and command; refusal adds task_id and a reason hash. jobs/TASK.json retains lifecycle and uncertainty. reports/TASK.json contains admitted raw output; review/TASK.json its bound needs_root_review envelope; decisions/TASK.json the explicit root receipt. Credential-bearing tmux stderr is hashed, not copied.

These files and status/review CLI are existing integration points. Crab tools/crab/src/angel.rs status(role=fish) and known_reports(role=fish) currently read legacy master/writer/annotation status, and do not consume this supervisor queue. Its socket uses newline JSON schema=1 with id/op and optional role/task/target_id/limit; status/read_reports allow role=fish and limit 1–3. A trusted adapter should bind the Fish run root and return bounded task/hash/status envelopes through Crab, retaining explicit root decisions. No socket registration, automatic root-chat notification or Crab integration of this queue exists.
