# crab-mova-deputy

## model

# Crab with Mova understanding — source-bound model

## Descriptant

`crab` is the single Rust deputy: the public Crab angel (control socket, request journal, pins, Dolphin and status
observations) with Mova's own understanding installed as its prompt handler. The caller sends only arbitrary text
and an optional priority (`op:"ask"`); Crab decides, through the native Octopus planner (Mova annotator, world
reader, configured capabilities), what to execute and refuses the rest with a review candidate.

## Forward

`crab serve CRAB_CONFIG DEPUTY_CONFIG`: load the deputy (pinned syntax model), then `serve_full` with
NoExternalRoles and `Understanding`. For an ask: `Deputy::run_with(prompt, id, execute=true, {crab_observe →
CrabInProcess(callback), crab_control → CrabControl(callback)})`, where the callback is Crab's nested dispatch
(status, read_reports and the configured worker operations); the report is
compacted to fit Crab's 64 KiB reply (plan, failure, followups, results; large outputs replaced by their digests).
`crab ask SOCKET ID [urgent] < text` is the client.

## Invariant

Prompt understanding acquires only configured worker control: nested Crab calls are limited to status,
read_reports and worker_status/start/stop/restart on configured worker names, with operation and worker fixed by the
deputy configuration. Legacy role markers (pause/resume/stop), whales and consultations remain separate explicit
protocol operations. One request is
answered at a time; urgent waits behind running work and only reorders the waiting queue.

## Cost

Deputy load once (syntax model); per ask: native parsing of the prompt (linear in its sentences) plus configured
adapters; in-process status avoids a socket round trip and cannot deadlock on Crab's own busy socket.

## Composition

Depends on `mova-crab` (angel, PromptHandler, ask client) and `mova-octopus` (Deputy, CrabInProcess, CrabControl). Deployment
configs stay outside the repository.

## behavior_complement

Not done by this binary: whale/external roles (NoExternalRoles), legacy marker control triggered by prompts, worker
control outside configured capabilities, English messaging to workers, training,
annotation, database writes, any provider model call for understanding. A prompt the planner cannot resolve yields an
explicit refusal and a review candidate rather than a guess.

## reverse_reconstruction

From a reply one recovers the plan (capabilities, native witnesses), failure stage and result digests; the full
report is recoverable from the deputy ledger by invocation id; the prompt text only via its digest and ledger.

## checks

### Checks

Crab: `ask_takes_any_text_and_priority_without_preemption` (54 KB prompt accepted, Crab decides to run, in-process
status allowed and stop refused, mode/invalid priority refused, urgent after running slow ask and before waiting
normal) — red without the queue sort; `ask_without_installed_understanding_is_refused`; all 78 Crab tests twice.
Deputy: 9 tests with the real model. Live: trial Crab on a copied config answered a refused question and an urgent
Crab-health request in-process; then deployed on the real socket and journal, replacing the legacy binary and the
separate deputy angel.

## History

### 2026-11-03 · checks

Heading `### Checks (2026-11-03)` stays in checks as `### Checks`.

#### 2026-11-03 worker control

The binary installs `crab_control` beside `crab_observe`. Evidence is in the Crab serve/prompt worker fixtures and
the deputy native controls; the binary itself was built, not deployed or run live.

2026-11-04: exercised on the live corpora read-only (observed 33 lines → 14 open texts; seed + exercises exam).

### 2026-11-04 · document

#### Dolphin pod (2026-11-04)

Installs `crab_consult` (CrabConsult) in-process next to crab_observe and crab_control.

#### crab_flow (2026-11-04)

Installs CrabFlow in-process next to crab_observe, crab_control and crab_consult.

#### Epoch 2026-11-04: Command-school CLI

New subcommands: `exam DEPUTY_CONFIG CORPUS...`, `review-queue OBSERVED [REVIEWED...]`, `promote OBSERVED DECISIONS REVIEWED` (append-only, 0600, prints training_authorized=false), `exercises DEPUTY_CONFIG NEW_OUT REVIEWED...` (create-new) and `candidates REVIEW_QUEUE` (read-only).
