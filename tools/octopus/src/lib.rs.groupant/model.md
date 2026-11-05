# octopus-deputy-library

## model

# Native deputy library

## Descriptant
The library composes existing Mova syntax and question-frame machinery with an externally configured intent registry and installed Rust handlers. Its first adapters are read-only Crab observation and bounded trusted-log inspection. Actual AST observations and bounded skeleton intent are separate from missing trained pragmatics. This is a useful constrained deputy, not evidence of arbitrary task understanding.

## Forward
Domain: at most 8192 UTF-8 prompt bytes, 1..8 sentences, at most 160 native words per sentence, 1..32 configured capabilities and at most eight selected actions. Let T(p,m) be actual native UD, A(T) structural tree admission, R(T,c) configured rule selection and H the installed handler registry. Planning returns a failure if any sentence is unadmitted or unmatched/ambiguous; otherwise it returns deterministic named actions with native node witnesses. Execution requires a durable invocation reservation before H calls. A receipt binds exact prompt/config/model/native-source observations and handler outputs.

## Invariant
Failure at planning implies zero tool calls. The planner never replaces a missing trained pragmatics observation with a fabricated one. Configured adapter parameters are trusted external configuration; prompt text cannot choose a filesystem path or shell command. Candidate priority is independent of semantic or training admission. Feats u128 values serialize losslessly as decimal strings.

## Cost
For n bounded tokens and c capabilities, lexical matching and native ancestry scans are bounded polynomial work; native parsing cost is inherited and unmeasured here. At most eight adapter calls occur, sequentially. Log work is bounded by 65536 bytes and 32 retained candidates. No numerical approximation or speedup bound is admitted.

## Composition
Native tree admission checks shape and cycles, not grammatical correctness. Configured intent rules use native lemmas, POS, root and ancestry observations, then adapters enforce their own trusted transport limits. All-sentence admission precedes reservation. The diagnostic handoff retains missing-context/unsupported distinctions and cannot directly become a training sample.

## behavior_complement
Domain: configured input/model/handler state; observation: calls made before receipt. Excluded behaviors are execution after any planning failure, execution of unconfigured capability IDs, and automatic shell/teacher/DB operations through the supplied adapters. Relation: failure(plan) implies call_count=0. Assumption: installed code and trusted configuration are cooperative; arbitrary custom handlers need separate review. Counterexample: a supported status phrase appended to an unsupported delete predicate cannot authorize a partial run. Evidence: source inspection and negative controls, not a full-program theorem.

## reverse_reconstruction
Domain: a source-bound plan or failure trace. Compatible antecedents are all ordinary prompts whose actual native analyses and configured rules produce that trace class. Multiple paraphrases yield the same capability plan, so no unique input can be reconstructed from plan alone. Raw prompt/hash restores the exact observed prompt identity, not its intended meaning. Counterexample: polite and imperative status requests share a plan. Evidence: finite native paraphrase controls.

## error_partition
Observation partition: input boundary, syntax_unadmitted, missing_context, pragmatic_ambiguity, unsupported_capability and execution_failure. Valid syntax plus an unsupported action is not syntax failure. Missing trained pragmatics is a separately recorded observation gap, not invented engine output. Counterexample: a training request is outside the capability domain even when its native tree is structurally valid. Evidence: finite controls plus source branches.

## observation_scope_exclusion
For log tail B and lexical markers M, candidate lines are selected by substring occurrence and explicit size limits. The complement within the observed tail includes nonmatching lines and a possible discarded partial first line. It says nothing about unobserved file history or real operational health. Empty B cannot imply healthy=true. Counterexample: a recovered old error remains a candidate; a silent worker may emit no errors. Evidence: bounded-log test and actual empty trusted stderr observation.

## replay_uncertainty
Domain: cooperative invocation ledger with durable reservation and optional receipt. A completed equal-binding ID maps to its old receipt; an unequal binding is excluded. An incomplete reservation has compatible histories including no call and successful call before crash, so execution state is not reconstructible and replay is refused. Evidence: mocked Unix call, cache, ID conflict and removed-receipt controls. Assumptions: ledger is trusted; no adversarial edits or filesystem durability theorem.

## utf8_suffix_observation

Type: partial_observation_exclusion. Subject: `diagnose`. Domain: a trusted regular log descriptor with observed length L, byte cap B = 65536 and start s = max(0, L-B). Read at most L-s bytes into X. Let k = 0 when s = 0; otherwise k is one past the first newline in X, or |X| if no newline occurs. Decode only X[k..]. The observation preserves SHA256(X), s, |X|, k and decoded start s+k; matching lines retain absolute byte offsets. The first partial line remains outside the observation, as before. Reading does not expand past the length observed before a concurrent append.

Excluded inference: neither a skipped prefix nor an empty decoded suffix establishes the absence of earlier errors or worker health. A continuation byte at the beginning of a bounded window does not imply malformed UTF-8 in the full log. Conversely, malformed UTF-8 inside the admitted suffix remains a diagnostic error; no lossy repair is applied. Compatible antecedents include arbitrarily many different earlier histories and dropped prefix contents, so the lexical summary is not invertible; the retained hash binds only observed bytes.

Counterexample: a valid 65537-byte log begins with the two-byte alpha character, followed by a long first line and an error line. The 65536-byte window starts at the continuation byte. Validating the entire window before excluding its first line previously refused this valid input and lost the subsequent candidate. Native regression now retains that candidate at its exact source offset. A long single line yields no admitted suffix; malformed bytes in admitted first/late lines remain rejected. The old source fails the new test; five native library tests pass, including actual syntax/plaintext controls. This is finite evidence, no throughput comparison or semantic proof. Concurrent truncation/writes are not an atomic snapshot guarantee.

## polar_question_admission

## crab_control_admission

Type: partial_exclusion. `CrabControl` maps a configured capability to one of worker_status, worker_start,
worker_stop, worker_restart and a worker name taken from trusted parameters; the prompt only selects the capability.
Config validation checks these parameters at load (only operation and worker keys, admitted operation, identifier
worker, worker required except for status). Excluded: consult_opus, legacy markers, Whale operations, extra
parameters, prompt-supplied worker names. Vocabulary (start/launch, restart, stop, entity aliases) is configuration:
with a test configuration "Start Eel.", "Restart Eel, please.", "Is Eel running?" and "Stop Eel." reach the four
operations; "Do not stop Eel.", "If Eel is running, restart it.", "Delete Eel.", "Eel stopped.", "Stop Whale." and
"Kill Eel." are refused before any call. Without an installed handler the plan fails closed at execution.
Negative controls: admitting any operation turns the unit test red; adding "kill" to the stop vocabulary turns the
native test red.

## most_specific_reading

## checks

### Worker control

`crab_control_admits_only_configured_worker_operations` (no model) and the native controls with the real model:
six new English control positives, six control refusals, four prompts executed through `run_with` with an
in-process `crab_control` override, a refusal that never reaches Crab, fail-closed execution without the override.
10 tests twice with MOVA_OCTOPUS_TEST_MODEL; two red controls restored.

## History

### 2026-11-02 · checks

2026-11-02: four Rust tests pass, including ten ordinary positive prompts, eight negative prompts, actual native model/UD/frame invocation, lossless u128 feature serialization, mocked Unix IPC, receipt cache/conflict/no-replay, and rejected-plan no execution. Set MOVA_OCTOPUS_TEST_MODEL to run the native integration; absent model explicitly skips it. Real status and combined log/status runs succeeded. Empty stderr gives no health evidence. No training/generalization claim.

2026-11-02 log-boundary follow-up: five library tests pass with the actual native syntax model enabled (7.73 seconds). New UTF-8 suffix fixture fails against old source with exit 101, then passes with exact offsets and original tail hash. Long no-newline input remains an empty observation; malformed admitted UTF-8 still fails. Deterministic syntax projection ran twice byte-identically. Receipts remain external. No worker restart, paid model, DB, training or assessment ran.

### 2026-11-03 · polar_question_admission

2026-11-03 teaching step from a real refusal ("Is Crab running normally?" was read as a declarative). Domain: one
sentence's native words (the native list has no punctuation). Rule: the first word is a non-past AUX attached to the
root clause (subject-auxiliary inversion) ⇒ a polar question asking to observe current state; its root is then a
request head, exactly like a what/how question. Still refused: declaratives ("Crab is running normally."), negation
("Is Crab not running?") and past-tense inversion ("Was Crab running normally?"). Admitted without "?" too, because
inversion alone marks the question. Negative control: forcing the rule off fails the native control test.

### 2026-11-03 · most_specific_reading

2026-11-03: among candidate capabilities for the same entity, a candidate whose matched observations are a strict
subset of another's is dropped; with equal matches the candidate whose configured observations were said more
completely (coverage) wins. "Stop Eel now." → urgent stop (immediate), "Stop Eel." → ordinary stop. Crossing or equal
coverage stays ambiguous and is refused. Negative control: without the pruning "Stop Eel now." is refused as ambiguous.

### 2026-11-03 · checks

Heading `### 2026-11-03 worker control` stays in checks as `### Worker control`.

### 2026-11-04 · document

#### Epoch 2026-11-04 (English)

2026-11-04: optional typed slots per capability (`slots`, `slot_policy`), masking before sentence splitting only when some capability declares slots, `bind` (each placeholder to exactly one action through `slots::bound_head`, slot counts checked; stages argument_invalid/unbound/ambiguous/count), `addressee` intent flag and `native_lemma_observations` (matched by native lemma only, because dictionary variants of 'a' include 'can'). Defaulted fields are not serialized: configs without them hash as before; existing native controls pass unchanged.

#### Dolphin pod (2026-11-04)

Dolphin pod. `CrabConsult` (handler `crab_consult`) maps a configured {operation enqueue|state, dolphin} or {operation list} to consult_enqueue/consult_state/dolphin_list; enqueue requires exactly one slot `task` of kind text and sends only its bound value. Config `group_aliases` (group word -> member aliases, validated against configured entities): a singular group word without a member alias refuses as pragmatic_ambiguity unless a matched action names the word itself; the check also replaces another failure. Intent `collective` binds its entities only in the plural. "which" joins "what"/"how" as a question word.

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04 (Whales in English): id slots are masked only when a capability declares one (mask_with); a copular wh-question rooted in what/which is read through its nominal subject noun (copular_wh_subject) when that noun is a configured observation.

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: Registers `relay` and its built-in handler `animal_ask` (AnimalRelay) in Deputy::load; Config::validate checks `animal_ask` capabilities with relay::validate.

#### crab_flow (2026-11-04)

CrabFlow maps trusted parameters {operation: status, group?} -> {op: flow_status, group?} and
{operation: attention} -> {op: attention}; any other key, operation or group shape is refused before Crab
is called; no prompt arguments. Config.validate admits up to 64 capabilities (the deployment holds 33).
Measured with en.bin 2026-11-04: "Do the Fish have work?", "Show the Fish queues.", "Is any worker
clogged?", "Do the workers have work?", "What needs my attention?" plan to the flow capabilities;
"Are the Fish starved?" (past narrative) and "Is any Fish clogged?" (root "fish") are refused.

#### Epoch 2026-11-04: Deputy frame, ambiguity candidates and deduplicated review queue

Each observation carries `deputy_frame` (`frame.rs`). A repaired polar frame makes the recovered verb or adjective the checked predicate and the subject noun not a predicate; inversion also counts when the parser hangs the auxiliary on a name inside the subject. Ambiguity refusals carry `candidates` (entity aliases or capabilities) and list them in the reason. The review queue holds one file per (prompt, failure stage) — `dedupe_key` — with every reproducing build in `reproduced_by` (candidate id, source/model/config/native-source hashes, reason, times seen); report followups keep their per-build `candidate_id`.

### 2026-11-04 · checks

2026-11-04 (English): cargo test --manifest-path tools/octopus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL (16 tests including the unchanged native controls; two identical runs on 2026-11-04). Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

2026-11-04: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs on 2026-11-04)

2026-11-04: cargo test --manifest-path tools/octopus/Cargo.toml --offline with MOVA_OCTOPUS_TEST_MODEL (22 unit tests incl. 4 socket, 2 relay; 2026-11-04)

2026-11-04: 31 lib tests with MOVA_OCTOPUS_TEST_MODEL=en.bin 0.4, incl. native frames from real refused prompts, dedupe and execution-failure reading; red controls (no repair, per-build queue files, no executed_failed, ambiguity without candidates) each turned the native test red; restored.

2026-11-04: a repaired frame keeps the unrepaired admission of an observed subject noun ("Is the queue empty?"); seaspider English tests (2) were red without it and pass with it; nautilus, turtle, whale tests pass.

### 2026-11-05 · document

#### Epoch 2026-11-05: model-robust binding after the en.bin retrain

Observation (2026-11-05): `crab exam` on the seed corpus gave 50/53 both with en.bin 2483516a and with the model before it (bff784ac, built from the pre-redaction commit): the trees of the three failures are identical under both models, so the redaction retrain did not cause them; the regression predates it (the native deputy test written against en.bin 0.4 planned "Show Crab's recent reports."). Forward changes: `review` collects the configured request predicates and reasons over `frame::read_repairing` words (an imperative read as a noun phrase is repaired; the observation keeps the native words); `bind` tries `slots::bound_head` first and, for a text argument only, `slots::colon_head` (a quotation after a colon closing a request clause). Invariant: every argument still reaches one action's request head of the same sentence and a slot of its kind; nothing outside a quoted span is sent. behavior_complement: no binding of a span before the request, after "Note:", in a later sentence, or followed by words; no repair of an imperative with any verb in the sentence. reverse_reconstruction: native words, frame (with its repair mark) and bound argument nodes are in the observation and the plan.

### 2026-11-05 · checks

2026-11-05: 34 lib tests twice with MOVA_OCTOPUS_TEST_MODEL=en.bin 2483516a: colon-introduced Dolphin tasks for four Dolphins with « » and " ", quotation negatives never sent, "Show Crab's (recent) reports." planned through the repaired frame; the frame test compares roles with or without the repair marker. Red controls (colon rule removed; imperative repair removed) each turned native tests red; restored. `crab exam` seed corpus (66 entries, 13 new): 66/66 (live build 203013d8e: 57/66), two identical runs. Dependents: seaspider, nautilus, turtle, whale English/reader tests fail identically on main with en.bin 2483516a (model drift outside this change); unit tests pass.

### 2026-11-05 · document

#### Epoch 2026-11-05: questions only observe

Found while re-running the animals' English tests under en.bin 2483516a: a polar question about an action ran it — "Is Eel stopped?", "Did you stop Eel?", "Is the Wikipedia dictionary drafted?" and "Is paper X described?" each planned the stop, the draft or the description, because a question makes its root a request head and the asked verb is also the capability's observation. Model: a question (frame mood polar or wh, or any sentence whose own terminator is '?', except a modal request "Could you …?") asks to observe; only a capability declared `observes` may answer it. Acting capabilities are withheld; if nothing remains the sentence is refused as pragmatic_ambiguity. The default is acting (fail closed). Load-time validation refuses `observes` on the deputy's own acting operations; each animal checks its own. The question-mark signal is linear and model-independent: the 0.5 parse reads "Have the Whales resumed?" as the causative imperative "have X resumed".

A polar frame now admits its predicate through an observed subject whether or not the parse needed the frame repair: under 0.4 "Is the queue empty?" was repaired (subject as root), under 0.5 it parses correctly with «empty» as root and was refused. `bind` applies `slots::colon_head` to every slot kind: "Survey this paper: paper-2023-gamma." hangs the identifier as `vocative` under 0.5.

Invariant: no question sentence contributes an action whose capability is not declared observing. Cost: one comparison per candidate. behavior_complement: questions never queue, start, stop, restart, enqueue or relay; reverse_reconstruction: the frame mood, the terminator and the withheld capability are recoverable from the observation and the failure stage.
