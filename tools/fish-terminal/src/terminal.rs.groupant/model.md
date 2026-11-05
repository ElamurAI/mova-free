# private-fish-terminal-tmux

## descriptant

### private-fish-terminal-tmux

An argv-only tmux adapter creates a new fixed session, marks its config owner and checks exactly one pane across the session. Capture returns command, dead state and bounded text. Literal buffers deliver prompts; Enter is a separate effect. Each tmux client is bounded by a five-second timeout and output limits. No model API or shell interpolation is used.

Prospective state/observation reasoning is in ../lib.rs.groupant/model.md ("Prospective state and observation model"). Native checks preserve counterexamples and explicit UI/source limitations. Concrete tasks and configurations remain private; root handles repository relocation/public export separately.

#### Independent review corrections

Domain: fixed ASCII session names on tmux supporting exact-target syntax. Observation: literal argv, ownership marker and single-pane result. Session targets are '=' + name; pane targets are '=' + name + ':'. Thus missing name must not resolve through tmux prefix matching to name2. Config also excludes pairwise prefix IDs. The owner marker binds the whole config, not authenticated process identity.

Counterexample: absent peta-fish-a and present peta-fish-a2 with the same owner. A native fake executable deliberately returns a2 ownership for raw a, rejects exact missing a and checks the actual adapter argv for launch marker, ownership, captures, paste and Enter. This finite witness does not certify arbitrary tmux releases or prevent malicious session replacement between check and effect.

Root separately completed two real Codex TUI tasks and received two structurally bound reports in the external private pilot run. Root rejected the water report for a human/species overclaim and free-form nonenum pragmatics; hearing was accepted only as a provisional syntax critique. No DB/training admission occurred. This is finite end-to-end evidence for the old c215 runtime, not semantic validation or a live smoke of this M1/M2/M3 binary. This implementation worker used no model call or real terminal session.

## model

# Source-bound partial models

## Forward

An argv-only tmux adapter creates a new fixed session, marks its config owner and checks exactly one pane across the session. Capture returns command, dead state and bounded text. Literal buffers deliver prompts; Enter is a separate effect. Each tmux client is bounded by a five-second timeout and output limits. No model API or shell interpolation is used.

## behavior_complement

Domain: the admitted configuration/input and cooperative local effects described above. Universe: transitions over the recorded job/session states and bounded observations. On successful adapter checks, exclude adoption of a session with a different owner marker or multiple panes, shell evaluation of task text, and unbounded client-output admission. A client timeout fails with uncertain terminal effects rather than retrying. The trusted same-user metadata premise excludes malicious local marker forgery.

## reverse_reconstruction

An equal command/screen capture is compatible with many prior Codex histories and does not reconstruct whether the model reasoning is correct. An owner marker binds configuration under cooperative local operation, not cryptographic process authentication. Matching bytes cannot reconstruct hidden child actions.

## effect_partition

Observation partition separates successful client exit, nonzero exit, output-bound failure and timeout. Timeout kills/reaps only the tmux client, not the persistent Codex session. A failed launch after session creation preserves a reservation; no session is automatically killed or recreated.

## Invariant

Configuration/task/source bindings remain explicit; unknown terminal/model effects are not converted to completed work. Review-only outputs do not mutate training, shared DB or production code. Exact byte identity and structural admission are separate from model truth.

## Cost

Each client polls at5ms up to5s; stdout cap32KiB and stderr2KiB. File-backed capture prevents pipe accumulation in memory but assumes trusted local resources; client latency, tmux process state and Codex scheduling are distinct.

## Checks

Core fake-terminal controls cover ownership/busy/unstable states. The real subprocess test verifies literal argv through printf and bounded sleep-client timeout; it does not launch tmux/Codex. Actual CLI help was inspected; interactive marker compatibility remains a root smoke obligation.

Evidence status: source-derived partial model with bounded native controls. No formal proof, full semantic AST, percentage-error bound or live Codex quality guarantee.

## exact_session_observation_exclusion

Domain: fixed ASCII session names on tmux supporting exact-target syntax. Observation: literal argv, ownership marker and single-pane result. Session targets are '=' + name; pane targets are '=' + name + ':'. Thus missing name must not resolve through tmux prefix matching to name2. Config also excludes pairwise prefix IDs. The owner marker binds the whole config, not authenticated process identity.

Counterexample: absent peta-fish-a and present peta-fish-a2 with the same owner. A native fake executable deliberately returns a2 ownership for raw a, rejects exact missing a and checks the actual adapter argv for launch marker, ownership, captures, paste and Enter. This finite witness does not certify arbitrary tmux releases or prevent malicious session replacement between check and effect.

## Separate root live evidence

Root separately completed two real Codex TUI tasks and received two structurally bound reports in the external private pilot run. Root rejected the water report for a human/species overclaim and free-form nonenum pragmatics; hearing was accepted only as a provisional syntax critique. No DB/training admission occurred. This is finite end-to-end evidence for the old c215 runtime, not semantic validation or a live smoke of this M1/M2/M3 binary. This implementation worker used no model call or real terminal session.

## configured_model

Tmux launch passes the configured `Config.model` after `-m` instead of a literal; with the default config the argv is unchanged (`gpt-6.1-terra`).
