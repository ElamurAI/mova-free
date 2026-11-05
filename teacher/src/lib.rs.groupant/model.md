# teacher-transport

## descriptant

### Teacher transport descriptant

The combiner inputs are raw Rust, the forward model, the ordered typed family,
invariants, cost assumptions and evidence. AST extraction is not materialized.
A future combiner can intersect source control-flow constraints with compatible
trace fibers and compare prospective oracles against observations; it must keep
unknown operational behavior visible. This sidecar is not an autonomous engine.

Useful epoch observations: the decoder implements last-register assignment,
not a complete Codex event protocol state machine. A completion before the final
answer is accepted. Nonstring final message text clears a previous answer.
A later missing usage completion invalidates earlier valid accounting.
Unknown valid-JSON events are ignored, and cache<=input is not checked.
The bounded offline probe confirms these behaviors. Treat protocol strengthening
as a separate design decision with compatibility tests, not a silent model fix.

The ask timeout governs child polling, not every blocking operation. Source
inspection cannot establish a global return deadline: worker joins, escaped
descendants, filesystem operations and platform differences remain gaps.
No fake-child process, network, authenticated teacher or training run was made.
No production Rust changed; catalogs describe existing behavior and prospective
checks. Twenty models count distinct observations, not independent proofs.

## approximant

### Forward approximant, invariant and cost

For decode_events let state (a,u,h,t,i) initially be (None,None,false,false,false).
Skip blank lines; parse every other line as JSON. Error or turn.failed aborts.
thread.started requires h=t=i=false and u=None, then sets h.
turn.started requires t=i=false and u=None, then sets t.
A recognized item event requires u=None and kind agent_message or reasoning;
set i=true. Completed agent_message assigns text.as_str() to a, even if this
clears it. turn.completed requires u=None and nonblank a, then assigns usage.
Unknown event types are identity, including after completion.
At end require nonblank a, present u, and u64 input/output token counts.
Emit the unchanged answer and accounting projection, defaulting invalid/missing
cached count to zero. This is an exact symbolic projection on terminating finite
inputs; parser failure detail and allocation failure are outside the model.
Headers remain optional for partial/minimal event streams. Multiple messages in
one turn remain valid. IDs are not checked, and future unknown events remain
neutral: this enforces a bounded turn contract, not the full remote protocol.

For ask: validate model, effort, timeout; create parent and per-call directory;
construct isolated CLI, spawn, concurrently write prompt/read bounded streams,
poll status; on timeout/error stop, then join workers; require status, output
bounds, successful exit, prompt write, UTF-8, successful decoding; augment
model/secs; append accounting log; return reply; attempt directory cleanup.
Worker-join errors can mask timeout errors because status is inspected afterward.
Environment, filesystem, OS scheduling and child behavior are explicit inputs,
so deterministic output is claimed only for the pure decoder projection.

Invariants on successful decoder return:
answer.trim() is nonempty; input/output/cache counts are u64; provider=codex;
is_error=false; cost_reported=false; total_cost_usd=0; creation-cache tokens=0.
Zero dollar field is a compatibility placeholder, not measured cost.
No successful trace contains a recognized explicit failure or forbidden item.
Exactly one completion follows the final answer assignment, and no recognized
item follows it. Optional thread/turn headers occur at most once in that order,
before items/completion. These positions bind answer and accounting to one turn.
No invariant establishes a supported remote model name, cache<=input,
authenticated item IDs, remote answer correctness or total protocol coverage.

On successful ask return, prompt write, child success, both byte caps, decoding,
and call-log append have succeeded. Cleanup is best effort, not an invariant.
There is at most one spawn attempt and no automatic retry. Unix group kill is
best effort; full process-tree termination is not proved.

Cost prediction (unmeasured): decoding work is proportional to input bytes plus
JSON parsing and answer copies; retained live state includes the current line's
JSON, latest answer and usage. This is O(N) space worst case for N input bytes.
ask readers retain at most 2(L+1) bytes for L=32 MiB, plus prompt, decoded objects,
thread stacks and buffers. Polling is approximately 20 checks/second during
the wait phase, with scheduler variation. Timeout does not bound join, directory,
or append duration. No network latency, teacher throughput or cost gain measured.

## catalog

# Teacher transport typed obernants

## nonbehavior
Type: behavior_complement. Domain U is all pairs (s,o), s finite UTF-8
JSONL string, o decoder success JSON or error category. R is actual D's graph.
C is the union of success pairs with malformed JSON, explicit failure, rejected
item kind, repeated/late lifecycle start, multiple completions, completion before
a nonblank answer, any recognized item after completion, absent answer/completion,
or invalid required accounting. C is a source-derived inner bound of U minus R:
it does not enumerate wrong successful output values or allocator failures.
Oracle: finite_trace_oracle_preserves_turn_identity checks the success/error
partition and the entire success JSON on its declared finite trace universe.

## antecedents
Type: reverse_reconstruction. For successful y, B(y) is every finite trace
with exactly one completion c, no recognized item after c, at most one optional
thread header before an optional turn header, both before all items and c.
The last completed agent_message before c has text exactly y.result and is
nonblank; c's usage projects to y. All lines parse; failures and forbidden item
kinds are absent. Other events, blank lines, IDs, earlier messages and reasoning
remain arbitrary subject to these positional guards.
This is an exact symbolic preimage on the terminating decoder domain.
Finite enumeration checks all 271453 traces of length <=5 over the twelve
symbols in the source test, obtaining four fibers of 938 traces each.
The finite set is complete only in that universe; no unique inverse is claimed.
The positional oracle independently tests membership, including excluded outputs.

## model-precedence
Type: configuration_preimage. Hold component variable name fixed; observation
is model(name). Its preimage uses global valid-Unicode environment value first,
component value only if global lookup errors, then DEFAULT_MODEL if both error.
Empty global strings shadow component values. Source-derived exact branch
relation, conditional on a stable process environment; no validation here.
Oracle: isolated process matrix of absent, empty and nonempty values.

## model-name
Type: lexical_acceptance_complement. Domain finite Rust strings; validate_model
accepts exactly prefix gpt- with no Unicode whitespace. Excluded successes
include other prefixes or any whitespace. This exact lexical complement says
nothing about provider-supported model names; gpt- alone passes.
Oracle: default and gpt- pass; gpt-x followed by a nonbreaking space rejects.

## effort-domain
Type: effort_reachability. In ask, after valid model, only low, medium, high,
xhigh can reach timeout parsing. All other effort strings are excluded from
spawn-reaching traces. Exact source control-flow relation under normal return.
Oracle: invalid effort fails before filesystem creation or child launch.

## timeout-domain
Type: timeout_budget_preimage. Hold earlier guards passing. Reachable timeout
values are positive u64 parsed from environment, default 1800 on lookup error.
Zero and parse failures cannot reach directory creation. Conditional exact
configuration relation; timeout is not a strict total ask wall-time bound.
Oracle: isolated timeout=0/invalid guards; do not launch a real teacher.

## parse-prefix
Type: first_failure_partition. First malformed JSON, explicit failure,
rejected item, repeated/late header, premature/repeated completion or late item
aborts immediately. Earlier lines must parse and pass their event-local guards;
suffix is arbitrary. Required accounting is checked only after scanning, so a
malformed suffix can mask invalid usage. Exact category-level control-flow model,
not full anyhow-text equivalence. Oracle: malformed/failure suffix regressions,
finite trace oracle; error precedence beyond these cases remains source-derived.

## item-kind
Type: capability_event_complement. For item.started/updated/completed, allowed
kind is agent_message or reasoning. Missing, null and tool kinds reject.
Other event types do not examine item.type. Exact observed guard complement,
not a proof that a remote tool never executed.
Oracle: command_execution at item.completed rejects, at unknown event is ignored.

## answer-fiber
Type: last_assignment_preimage. Hold the one-completion positional guards
and accounting fixed. Each completed agent_message before completion assigns
text.as_str(), including clearing it on nonstring text. The last such assignment
must be the exact nonblank result. Earlier text, item IDs and reasoning cannot
be reconstructed; partial started/updated text does not assign the register.
Exact symbolic preimage under guards. Oracle: exhaustive A/B/null sequences,
realistic_lifecycle_and_partial_messages; the latter preserves progress plus final
messages and partial reasoning/answer events.

## usage-fiber
Type: single_completion_preimage. Hold accepted answer and header/item order
fixed. Exactly one turn.completed supplies usage; input_tokens/output_tokens must
be u64. No earlier or later completion can contribute accounting. Its preimage
contains arbitrary ignored fields and compatible cached-count values, but excludes
every multi-completion transcript even when counts agree.
Oracle: the finite positional oracle distinguishes input counts 10 and 20 and
compares the complete output. Invalid required accounting has eight fixtures.

## blank-answer
Type: whitespace_boundary. Returned answer preserves its original bytes but
requires trim() nonempty. Thus whitespace-only final strings exclude success,
while surrounding whitespace survives. Exact Rust Unicode trim projection.
Oracle: blank Unicode string rejects; padded nonblank answer stays padded.

## unknown-events
Type: extension_invariance. In D, insertion of valid JSON with an unrecognized
or nonstring/missing type preserves output. Blank lines also preserve output.
Exact observation invariance on successful JSON parsing; malformed lines do
not qualify and recognized item types are not neutral.
Oracle: insert null, {}, unknown event and empty lines into a valid transcript.

## ordering-gap
Type: temporal_identity_complement. Current successful traces have exactly
one completion after the final answer assignment, optional ordered headers before
items, and no recognized item/header afterward. Completion-before-answer,
concatenated turns, late headers and post-completion reasoning are excluded.
The legacy two-register decoder is the explicit countermodel: [completion, answer]
and [answer A, completion 10, answer B, completion 20] can succeed there.
This restriction implements the documented one-completed-turn contract while
preserving multiple messages within a turn and unknown extension events.
Identity here is positional within one stream; item/thread IDs are discarded,
not authenticated, and this is not a complete Codex protocol validator.
Oracle: 271453 finite traces, lifecycle fixture, compiled legacy-decoder mutation.

## accounting-loss
Type: lossy_projection_preimage. Hold required u64 input/output fixed. Missing,
negative, string or otherwise non-u64 cached_input_tokens all map to zero.
Any extra usage fields are discarded; creation tokens always zero.
Exact equivalence classes of successful D inputs; cached tokens may exceed
input tokens because there is no numeric cache/input inequality guard.
Oracle: cache string maps zero and oversized cache count is preserved.

## subscription-command
Type: launch_policy_constraint. Fix executable, cwd, model and effort.
Projection is Command program/cwd/args and explicit environment removals.
Constructed args request chatgpt auth, read-only sandbox, disabled web search,
all thirteen feature disables, ignored configuration/rules, ephemeral operation,
JSON/no-color output and stdin prompt. Four API/base URL keys are removed.
The exact construction relation is checked without spawn by
command_uses_stdin_isolation_and_subscription_auth. Unix process-group setup,
pipe descriptors and remote enforcement remain source-derived, not measured.
A matching Command projection cannot reconstruct prompt, inherited environment
or child effects. Different executable implementations share this projection.

## prompt-channel
Type: information_flow_projection. In A, prompt bytes enter piped stdin through
one worker, not command arguments or the explicit JSONL call-log fields.
This source-level dataflow exclusion is scoped to these sinks; child output
may echo prompts and OS/service visibility is not modeled.
Oracle: inspect command and log construction; future fake executable captures stdin.

## output-budget
Type: resource_budget_complement. stdout/stderr workers each read at most L+1,
L=32 MiB. After worker joins and successful status retrieval, successful return
requires both sizes<=L. Parent retained output bytes <=2(L+1), plus prompt,
parser and buffers. Exact per-reader cap; not a total service output cap.
Oracle: fake process emits L and L+1 on each stream; prospective only.

## lifecycle
Type: cleanup_reachability. CallDir drop attempts recursive removal on normal
scope exit, including errors after its construction. Return Ok additionally
requires append-log open/write. Cleanup errors are discarded; cwd and call log
persist. Source-derived conditional relation, no crash/leak impossibility.
Oracle: fake-process success/failure/log-write failure with temp-dir inspection.

## timeout-boundary
Type: liveness_counterexample. Timeout bounds the try_wait polling decision,
not joins or logging. On Unix process-group kill aims to unblock inherited
pipes; descendants escaping the group can retain them. Non-Unix lacks group
kill. Thus total-return-within-timeout is absent from proven invariants.
Partial failure hypothesis under adversarial child/process schedules.
Oracle: bounded fake escaped descendant test; prospective, not executed.

## log-fiber
Type: provenance_projection_preimage. A successful call log projects provider,
model, effort, secs, usage and cost_reported=false. Many prompts, answers and
event traces share these fields; elapsed seconds are nondeterministic.
Exact symbolic projection conditioned on successful append. Logging failure
can return Err after a valid decoded answer; no retry is induced.
Oracle: fake-process log inspection; no uniqueness of answer from log asserted.

## tests

### Evidence and bounded checks

Executed: cargo test -p mova-teacher --offline with a separate target directory:
4 unit tests passed; 1 authenticated live smoke test remained ignored.
These test successful accounting, incomplete/failed/tool/malformed events,
legacy model rejection, and selected command arguments/environment removals.
No production teacher call or training run was made.

The reproducible Rust probe is probe.rs.txt. Compile it as a temporary binary
outside the repository with dependencies mova-teacher (path to this checkout's
teacher crate) and serde_json=1.0.151, then pass the repository root as argument.
It checks 20 unique exact catalog headings and matching section/id records,
required type order, missing AST, registry owner, source SHA-256 and local file
references. Its malformed section negative control must fail validation.
Its 13 bounded decoder/model assertions exercise event ordering, neutral events,
malformed suffixes, failure suffixes, clearing answer/usage, forbidden items,
whitespace, cache defaults/oversized accounting and lexical model edges.
The source is kept as text to avoid registering a tooling fixture as production
Rust. It uses the system sha256sum command only for source identity.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29. Mova's shared subscription-authenticated Codex teacher
transport has one invocation path, lexical model checks, explicit command
configuration, JSONL event decoding, accounting logs and best-effort cleanup.
The source remains ../lib.rs; its exact revision hash is in manifest.json.

### 2026-10-29 · tests

Source revision: manifest.json. Epoch: 2026-10-29.

Executed on 2026-10-29: the Rust probe passed all 13 semantic assertions,
20 catalog records, reference/hash checks and its missing-section negative
control. The temporary harness resolved serde_json 1.0.154 and libc 0.2.191;
the existing workspace unit tests used the workspace lockfile.
Installed mova-modelcheck groupants passed: valid=true, diagnostics=[],
7 file sidecars, 524 missing file sidecars, 1 group sidecar, 3 missing groups.
Missing sidecars remain backlog; no verification is inferred.
Other catalog oracles remain prospective, especially environment-precedence,
timeout configuration, output limits, cleanup, escaped descendants and log
failure. Source-derived claims are not executed process integration tests.
Structural inventory checks do not establish model semantics or AST coverage.

### 2026-10-30 · descriptant

#### Single-turn contract epoch, 2026-10-30

The earlier epoch above records historical behavior. Investigation found a
concrete mismatch between the documented completed-turn contract and two
independent last-value registers: reversed completion/answer and concatenated
turns could pair an answer with unrelated usage. The current decoder excludes
those traces, repeated or late headers, and post-completion recognized items.
Multiple completed messages within a turn still select the last answer; optional
headers and unknown extensions preserve existing minimal trace compatibility.

Before implementation the prediction was O(N) decode cost with three additional
Boolean state fields and constant-time guards per recognized event. No throughput
gain was predicted. The independent positional oracle exhausts 271453 traces
over twelve symbols through length five. It checks the whole success projection
and complementary rejection, yielding four reachable output fibers with 938
compatible traces each. These are finite observations, not a general proof.
Projection fixtures preserve twelve distinct zero-cache antecedents. A realistic
synthetic lifecycle checks progress/final messages and partial events without
calling a teacher.

The existing twenty typed entries were reviewed and temporal, answer, accounting
and reverse models corrected instead of multiplying synonymous records. Partial
syntax is now materialized and source-bound. The old probe is updated to reject
reversed order; its historical results above remain dated evidence.
Command inspection now checks all thirteen feature disables and all four removed
environment keys. It verifies construction, not sandbox enforcement or network
isolation. Live calls, process cleanup, deadlines and remote protocol drift remain
outside measured coverage. model.md#tests records the compiled negative control and
repeated result observations.

### 2026-10-30 · tests

#### Single-turn correspondence, 2026-10-30

Current source is bound in manifest.json and syntax.json. Seven unit tests pass
in debug and release; the authenticated integration test stays ignored.
The new positional oracle enumerates all 271453 traces of lengths zero through
five over twelve literal symbols: thread start, turn start, answers A/B/null,
reasoning, completions with input counts 10/20, unknown event, error, forbidden
tool update, and JSON null. It checks success iff the independently stated
whole-trace constraints hold and compares every field of successful replies.
There are 3752 successful traces: four observed output fibers of 938 antecedents.
This is a complete finite relation, not a proof for arbitrary JSONL.

Additional fixtures exercise 18 cache/ignored-event combinations including twelve
zero-cache preimages, eight invalid required accounting values and nine rejected
item-kind/event combinations. A longer synthetic Codex-shaped lifecycle includes
thread/turn headers, reasoning started/updated/completed, progress messages,
answer started/updated/completed and one completion. It remains accepted;
concatenation and malformed/error suffixes reject. These fixtures are synthetic,
not recorded remote outputs. The construction test checks program/cwd, policy
arguments, all thirteen disables and all four environment removals without spawn.

Compiled negative control: replacing only decode_events with its base-revision
implementation makes finite_trace_oracle_preserves_turn_identity fail on a
two-event completion-before-answer trace. Restoring the current implementation
passes. This tests actual compiled Rust, not a fabricated failing assertion.
Debug and release result records repeat identically:
trace_cases=271453; reachable_fibers={("A", 10): 938, ("A", 20): 938, ("B", 10): 938, ("B", 20): 938}.
Wall times and test scheduling are not expected to repeat.
Focused cargo clippy -p mova-teacher --offline --all-targets -- -D warnings passes.

Reproduce unit evidence with cargo test -p mova-teacher --offline -- --nocapture
and the same command with --release. No live teacher, network, training,
credentials, child cleanup or service enforcement is exercised.
Historical 2026-10-29 probe statements above describe that epoch. The retained
probe now expects partial AST and rejects reversed completion/answer order.

The updated standalone probe passes twenty catalog records, source/hash/reference
checks, the malformed-section control and thirteen decoder/model assertions.
Two syntax generations are byte-identical. The changed-source check finds
teacher/src/lib.rs with no unregistered changed files. The isolated branch's
groupant report has only the two expected stale transitive teacher bindings in
the root and self-study parent manifests; parent integration owns their refresh.
This leaf's syntax/reference/source checks have no diagnostic.

2026-11-05 transient reconnect: the positional oracle alphabet gains a thirteenth symbol, the
reconnect error event, which is neutral before the completion and rejecting after it. It enumerates
402234 traces of length 0..5; debug and release both print
trace_cases=402234; reachable_fibers={("A", 10): 1474, ("A", 20): 1474, ("B", 10): 1474, ("B", 20): 1474}.
reconnect_notice_is_transient_only_before_a_completed_turn checks the reduced r3 sequence with a
synthetic stub answer (success, count 1), two notices (count 2), and red controls: turn.failed
after the notice, no completion, no answer, notice after completion, the notice as turn.failed,
and fifteen non-matching or limit/quota/capacity messages. Mutation controls: skipping every error
event fails three tests (reconnect, lifecycle, oracle); dropping the limit/quota filter fails the
reconnect test. 9 unit tests pass.

### 2026-11-02 · approximant

#### Shared exec arguments (2026-11-02)

`exec_args(model, effort)` and `CLEARED_ENV` expose the exact isolated `codex exec` argument list and
removed credential variables that `command` uses, so other transports (Fish Terra) reuse them
instead of copying. `command` behavior is unchanged; its existing argument test still pins it.

### 2026-11-04 · document

#### provider-busy (2026-11-04)
Type error_partition. provider_busy(text) is Some(m) iff every nonblank line parses, each event is thread.started, turn.started, error or turn.failed, at least one failure exists and every failure message equals one of PROVIDER_BUSY_MESSAGES, which were copied from the Codex CLI 0.165.0 binary's error displays (at capacity, flex capacity, server overloaded, high demand with U+2019). Complement: any item, turn.completed, unknown event, unparsable line, missing message, paraphrase, usage-limit or quota text gives None. It never changes decode_events, which still rejects every failed turn. Reverse: Some(m) reconstructs only the refusal text, not whether the provider billed or queued anything. Evidence: one unit test with a positive capacity case, each listed message, and eight red controls.

### 2026-11-05 · document

#### transient-reconnect (2026-11-05)
Type error_partition. Event-sequence predicate: let E be the nonblank JSONL events and c the index of the
single turn.completed. decode_events succeeds on E iff it succeeds on E with every event R removed, where
R = {i < c : E[i].type = "error" and transient_reconnect(E[i].message) = Some}; every other error,
every turn.failed, and any reconnect notice at or after c still reject. transient_reconnect(m) is
Some((n, k)) iff m = "Reconnecting... " + n + "/" + k + " (" + cause + ")" exactly, with n and k
ASCII decimal (at most nine digits, no sign), 1 <= n <= k, cause nonblank, single line and not
containing "limit" or "quota" (ASCII case-insensitive). The prefix and N/M (cause) shape come from
the Codex CLI 0.165.0 binary (core/src/responses_retry.rs), observed in Fish r3 t04 page
w00-p000522 on 2026-11-05 as "Reconnecting... 2/5 (stream disconnected before completion: ...)"
followed by an agent_message and turn.completed. The counterless "Reconnecting... waiting for
network" is not accepted. transient_reconnects(text) counts such error events anywhere (receipt
observability only). Complement: a reconnect followed by turn.failed, by no completion, by no
answer, or after completion; usage-limit, quota, capacity, paraphrased, padded, signed, zero or
n > k notices; and the same text inside turn.failed all stay failures. provider_busy is unchanged
and returns None on a completed turn. Reverse: a success no longer shows whether reconnects
happened; the count is recoverable only from transient_reconnects on the raw text. The provider's
retry semantics (whether a retried stream duplicates or drops items) are not modeled; the last
completed agent message is still the answer.

### Undated · approximant

Subject: ../lib.rs, SHA-256 718e03bf89dcf8e3c69f3af6fa137d88c70d9797b04f21f1752d998e2a5b6b90.
Scope: source-derived decoder and command/ask control flow, not remote inference.
syntax.json is a source-bound partial syntax projection; macro expansion,
type resolution and semantic AST analysis remain absent.

### Undated · document

All entries bind teacher/src/lib.rs at SHA-256
718e03bf89dcf8e3c69f3af6fa137d88c70d9797b04f21f1752d998e2a5b6b90.
Status throughout: partial source-derived relational specifications.
D is the terminating decode_events projection of finite UTF-8 JSONL strings;
errors are observed by category, not complete anyhow strings. A is ask's
trace projection, conditional on OS/worker termination. No model claims teacher
answer truth, remote CLI enforcement, or a complete operational inverse.
Each entry below specifies domain/observation, operator, approximation and
assumptions. Oracles are prospective unless model.md#tests names executed evidence.
No universal inverse or additive law is asserted.
