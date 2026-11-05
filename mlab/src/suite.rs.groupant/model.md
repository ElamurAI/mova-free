# mlab-suite

## descriptant

### mlab-suite

Line-oriented scenario parser, exact/prefix first-mismatch comparator, fresh-interpreter runner and output-only tree/VM equivalence gate.

#### Limits

Partial lexical syntax, no macro expansion or semantic resolution. Finite synthetic CPU checks are not a proof. Frozen suites, corpus, teacher, GPU, allocation exhaustion and host IO failure injection were not run. No useful executable percent surrogate for categorical control/effects; unsupported gap, no 1/2/5/10% claim.

## approximant

### Forward relation

parse maps ### headers/code/---/expectations to Scenario values; finish removes trailing blank code and adds newline. check strips one final newline and compares max(expected,got) lines. run_fs maps scenarios to fresh Interp+FS; vm_gate stops at first byte-different tree/VM output.

#### Domain

Valid source-generated AST/values within memory and integer/index limits; deterministic bounded synthetic observations. Arbitrary public malformed representations are outside safety claims.

#### Executable approximant gap

Exact categorical state/dispatch behavior has no supported percentage-error surrogate here.

## invariant

### Invariant

First_diff is 1-based earliest differing line; absent lines are <end>. Scenarios do not share default workspace/FS. Gate total counts only executed prefix and fully_native uses static fallback==0 and dyn_runs==0.

#### Cost

Parse/check O(input/output plus expectation bytes) with retained copies. Runs sum program costs with fresh state; gate executes twice per scenario until mismatch and retains one mismatch. Arbitrary programs can loop indefinitely; no timeout.

## composition

### Parent composition

parse-check constructs Scenario code/expectations; run-gate executes code in fresh environments and supplies captured output to check. vm_gate compares engine output directly and deliberately does not use expected answers; shared faults may pass equivalence.

#### Limits

Partial lexical syntax, no macro expansion or semantic resolution. Finite synthetic CPU checks are not a proof. Frozen suites, corpus, teacher, GPU, allocation exhaustion and host IO failure injection were not run. No useful executable percent surrogate for categorical control/effects; unsupported gap, no 1/2/5/10% claim.

## models

# Typed models

## behavior_complement

Expect=[] accepts only empty output. Prefix does not ignore extra output lines. Gate equality excludes differing bytes but does not exclude identical wrong answers.

## reverse_reconstruction

Parser is many-to-one due to ignored lines/fields and trimmed code. A mismatch tuple fixes only first divergent position; subsequent output is unconstrained.

## newline_boundary

Exactly one trailing LF is stripped; a\n\n is two lines, whereas empty string is zero and \n is one empty line.

## scenario_isolation

run_fs calls factory once per scenario before execution; default run_opts uses fresh Vfs.

## gate_prefix

On first mismatch increment total/stats, store mismatch, break. Unvisited suffix contributes nothing.

## count_semantics

native/fallback aggregate static compiled statement counters; fallback_runs/dyn_runs count dynamic triggers; fully_native is not proof of full language correctness.


## tests

### Executed checks

Selected exact names: wave30_synthetic_suite_boundaries, wave30_synthetic_gate_fail_fast. All selected native commands ran twice debug and twice release with matching sorted named outcomes (12 distinct tests, 48 positive executions across subsystem). Five compiled assertion mutants fail with exit101; restored byte-exactly and rerun green. Per-source associations appear in external source-test-map.json.

Controls change expected parse state, VM persisted value, suite factory count, SHA known digest and routing tie winner. These are compiled assertion controls exercising real code, not claims of implementation-mutant completeness. lib.rs is compile/type identity evidence only.

No frozen suites, Monte Carlo or recursion-heavy EDGE runs. No speed claim. Scoped Clippy limits retained externally; lexical syntax generated twice identically.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source model preceded test additions. No production semantics changed. Seven legacy files are modeled, with source-scoped fragments for complex files. Retain parse-before-effects versus runtime partial effects, output equality versus correctness, and independent layers routing as distinct observations.

### 2026-10-31 · tests

Heading `### Executed checks — 2026-10-31` stays in tests as `### Executed checks`.
