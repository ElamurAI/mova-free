# octopus-deputy-cli

## model

# Native deputy plaintext CLI

## Descriptant
The executable accepts trusted configuration and invocation identity as process arguments and ordinary English as stdin. The prompt itself needs no labels or formal envelope.

## Forward
Domain: plan|run, configuration path, bounded invocation ID, UTF-8 stdin. Read at most 8193 bytes as raw bytes (bounded_prompt): a prefix of at most 8192 bytes must be valid UTF-8; a longer prefix may end inside a multi-byte character and is decoded lossily, which never shortens it, so it stays over budget; load/verify the configured model, then call Deputy::run. Print its source-bound JSON report. The extra byte lets the library distinguish an over-budget prompt from a valid 8192-byte input.

## Invariant
Input text remains data. Model absence/hash disagreement fails loading; there is no model fallback. Execution mode is explicit in process arguments, not inferred from instruction text.

## Cost
Bounded stdin read plus config/model loading and library work. Native model loading is not a throughput measurement.

## Composition
The CLI relies on the library's full-plan admission, reservation, handler and receipt boundaries. JSON transport preserves the report; trained pragmatics is still explicitly missing.

## behavior_complement
Domain: CLI process inputs; excluded results include treating prompt JSON fields as privileged commands, calling a large model on unsupported input, or claiming a successful native run after model-load failure. Assumptions: trusted binary/config/argv. Counterexample: a natural request to train remains ordinary data and gets a bounded library refusal. Evidence: source inspection and actual plaintext invocations.

## reverse_reconstruction
A report binds the exact observed prompt and configuration hash. Report identity cannot uniquely reconstruct the original stdin transport: equivalent file redirection and pipe input are compatible. A process error alone does not identify a unique bad argument or missing artifact. Evidence: source-derived relation; no complete failure-space enumeration.

## input_budget_exclusion

Domain: stdin byte length. A captured length above 8192 is refused by the library; the CLI reads only 8193. Very large input is not fully retained. Counterexample: truncating to exactly 8192 would hide over-budget input, hence the extra byte. Counterexample: decoding the 8193-byte prefix strictly turned an over-budget prompt whose byte 8193 starts a two-byte character into a UTF-8 error instead of the library's length refusal; over_budget_prefix_cut_inside_a_character_stays_over_budget is red with the strict decoding. Evidence: direct source inspection; no full-stream provenance claim for rejected oversized inputs.

## invocation_identity
The invocation ID comes from trusted argv and is distinct from prompt meaning. Equal prompt text with a fresh ID is a new observation; equal completed ID is a cached receipt. Counterexample: requesting current status by replaying a completed ID returns past status. Evidence: library no-replay controls and documented process wiring.

## checks

The bin unit test over_budget_prefix_cut_inside_a_character_stays_over_budget (cargo test --bin mova-octopus) checks an over-budget prefix cut inside a two-byte character, a valid 8192-byte prompt and invalid UTF-8 within budget; it fails with the earlier read_to_string decoding (negative control).

## History

### 2026-11-02 · checks

2026-11-02: four Rust tests pass, including ten ordinary positive prompts, eight negative prompts, actual native model/UD/frame invocation, lossless u128 feature serialization, mocked Unix IPC, receipt cache/conflict/no-replay, and rejected-plan no execution. Set MOVA_OCTOPUS_TEST_MODEL to run the native integration; absent model explicitly skips it. Real status and combined log/status runs succeeded. Empty stderr gives no health evidence. No training/generalization claim.

### 2026-11-04 · input_budget_exclusion

Domain: stdin byte length. A captured length above 8192 is refused by the library; the CLI reads only 8193. Very large input is not fully retained. Counterexample: truncating to exactly 8192 would hide over-budget input, hence the extra byte. Counterexample (2026-11-04): decoding the 8193-byte prefix strictly turned an over-budget prompt whose byte 8193 starts a two-byte character into a UTF-8 error instead of the library's length refusal; over_budget_prefix_cut_inside_a_character_stays_over_budget is red with the strict decoding. Evidence: direct source inspection; no full-stream provenance claim for rejected oversized inputs.

### 2026-11-04 · checks

2026-11-04: the bin unit test over_budget_prefix_cut_inside_a_character_stays_over_budget (cargo test --bin mova-octopus) checks an over-budget prefix cut inside a two-byte character, a valid 8192-byte prompt and invalid UTF-8 within budget; it fails with the earlier read_to_string decoding (negative control).
