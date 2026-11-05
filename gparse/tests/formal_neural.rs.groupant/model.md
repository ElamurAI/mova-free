# formal-harness

## descriptant

# Formal specification harness descriptant

## forward

For each configured specification, run Lean with a ninety-second limit. Require
successful exit, absence of sorry markers, allowed reported axioms and the
expected report count. Replace exactly one formula in a fresh temporary copy.
Require unsuccessful Lean exit, non-timeout status and an error diagnostic.
Successful completion removes the invocation's directory. The legacy suite has
nine specifications and 38 reports; the focused frozen suite adds one general
set theorem and one negative control. Both tests are ignored by default.

## invariants

Every accepted local trace includes a successful positive check before its
negative check. The source specification is never rewritten by run_specs.
Allowed parsed axioms are propext, Classical.choice and Quot.sound. Count checks
are not theorem-identity checks. A rejected mutant is not sufficient without
the positive audit. No mathematical proof of Rust collection behavior follows
from passing this harness. The finite implementation correspondence in study.rs
is separate evidence, previously covering 64 forward and 512 inverse cases.

## epoch-memory

The existing single-suite implementation was factored into run_specs so the
new frozen check remains independently selectable and uses the same audit.
The union preimage fiber is generally nonunique: overlap with A may occur or
not, while O minus A is mandatory. Requiring P subset A is false whenever O
contains a new source. Its failed Lean proof supplies the negative control.

Useful limitations: report count can be spoofed by duplicate audit lines;
negative diagnostics use a broad substring; panic can leave temporary files;
combined logs lose stream identity. These are source observations, not newly
demonstrated vulnerabilities. General AST extraction, a reverse execution
solver, fault-injection coverage and a Lean-to-Rust refinement proof remain
open. No training, teacher, GPU execution or accuracy gain is claimed.

## catalog

# Formal harness typed model catalog

## absent-acceptance

Type: behavior_complement. Observation: successful run_specs returns.
Status: partial_source_model (specific executed evidence stated below).
Universe: fixed specification list and all process/filesystem observations. Inner bound on absent successful traces: positive nonzero exit, sorry marker, unexpected parsed axiom, wrong report count, nonunique mutation, successful mutant, mutant timeout 124, or mutant log without error. Each violates an explicit assertion. Assumes Rust assertions execute and logs are captured honestly. Oracle: independently perturb each guard; only the frozen false-formula case is newly exercised, other guard faults remain prospective.

## accepted-trace-preimage

Type: reverse_reconstruction. Observation: compatible observations given success.
Status: partial_source_model (specific executed evidence stated below).
B(ok) is the set of source/log/status tuples satisfying every guard in run_specs and all successful IO operations. Preserve all such tuples: acceptance does not uniquely recover a proof, toolchain or execution history. Assumes fixed spec list and trusted process runner. Oracle: construct two distinct valid logs with the same audit result; synthetic log-fiber experiment remains prospective. Real passing harness supplies members, not a complete solver.

## standard-axiom-boundary

Type: axiom_allowlist. Observation: parsed axiom names.
Status: partial_source_model (specific executed evidence stated below).
For every parsed depends-on line, names must lie in {propext, Classical.choice, Quot.sound}; an empty list is allowed. Holds for the parser's single-line bracket grammar, not arbitrary Lean output formats. Oracle: insert a project axiom report and expect panic; unexecuted mutation. FrozenUnion's observed three standard axioms passed.

## audit-cardinality

Type: report_count. Observation: number of printed theorem audits.
Status: partial_source_model (specific executed evidence stated below).
Count is no-axioms lines plus depends-on-axioms lines; require 1 for FrozenUnion, 9 Complementer, 5 Softmax, 4 each Twin/Enrichment/Retention/StudyComparison, 3 School/ExerciseReuse, 2 Activation. Names are not matched: duplicate reports could conceal a missing theorem report. Oracle: omit and duplicate lines; prospective, so count is not identity verification.

## admission-marker-exclusion

Type: proof_completeness_filter. Observation: positive process log.
Status: partial_source_model (specific executed evidence stated below).
Acceptance implies absence of literal sorryAx and declaration uses 'sorry' substrings. This is a textual necessary condition, conditional on Lean diagnostics retaining spelling and theorem audits exposing dependencies. Oracle: inject sorry in a checked theorem and observe rejection; prospective. This filter does not inspect unaudited declarations.

## single-formula-mutation

Type: mutation_locality. Observation: replacement occurrence count.
Status: partial_source_model (specific executed evidence stated below).
Let c be source.matches(from).count(). Mutation is produced only if c=1 and equals replacen(from,to,1); all bytes outside that occurrence remain. Assumes readable UTF-8 source. Oracle: duplicate/remove target and expect assertion; prospective. FrozenUnion's positive control execution established one occurrence for its exact target.

## false-formula-rejection

Type: counterfactual_specification. Observation: strengthened frozen preimage law.
Status: partial_source_model (specific executed evidence stated below).
Replace P subset O with P subset A while keeping the original proof. The resulting claimed equivalence is false: A empty, P=O={x} satisfies union equality but not P subset A. The harness requires a Lean error with nonzero non-timeout status. Observed control rejected; this is a proof-gate control, not a mutation of study.rs.

## timeout-boundary

Type: resource_guard. Observation: Lean subprocess exit 124.
Status: partial_source_model (specific executed evidence stated below).
check wraps each process in timeout 90. Positive timeout fails success; negative timeout is explicitly forbidden even though nonzero. Assumes installed GNU-compatible timeout and normal process termination. Oracle: a sleeping substitute command must not satisfy a negative control; prospective. Ninety seconds is a process limit, not a full IO/runtime guarantee.

## negative-diagnostic-projection

Type: failure_classification. Observation: negative status and combined text.
Status: partial_source_model (specific executed evidence stated below).
Accepted negative observation requires !success and code !=124 and log containing error. This broad predicate can admit infrastructure errors containing that substring; it does not prove every rejection is a mathematical counterexample. Oracle: missing import/error mock distinguishes the limitation; prospective. Our control edits an otherwise checked file and its genuine proof failure was inspected.

## stdout-stderr-loss

Type: information_projection. Observation: concatenated diagnostic string.
Status: partial_source_model (specific executed evidence stated below).
text(o)=lossy_utf8(stdout) followed by lossy_utf8(stderr). This loses original stream attribution, interleaving and invalid byte identity; multiple outputs share one observation. Assumes no separator is inserted. Oracle: stdout ab/stderr c and stdout a/stderr bc give equal text; prospective trivial helper check, no execution claim.

## environment-resolution

Type: configuration_preimage. Observation: Lean executable and working directory.
Status: partial_source_model (specific executed evidence stated below).
Project is LEAN_PROJECT; lake is LEAN_LAKE when set, otherwise HOME/.elan/bin/lake. Compatible environments may differ in all other variables. Missing project or HOME on the fallback path panics. Oracle: vary override and HOME independently; prospective. Existing installed project and lake were used, no dependency download command.

## source-location-binding

Type: path_resolution. Observation: repository specification path.
Status: partial_source_model (specific executed evidence stated below).
Specification root is compile-time CARGO_MANIFEST_DIR/../formal, independent of Lean's project working directory. Holding compiled harness fixed, changing shell cwd does not change this root. Assumes path still exists and source bytes stay stable. Oracle: run the binary from another directory; prospective. Direct rustc validation supplies the same manifest directory explicitly.

## temporary-run-isolation

Type: resource_identity. Observation: temporary directory key.
Status: partial_source_model (specific executed evidence stated below).
Directory name combines process ID and nanosecond system-time stamp; create_dir fails on collisions. It is collision-detecting, not a proof of globally unique names or protection from concurrent hostile replacement. Oracle: force an existing directory under a deterministic clock abstraction; prospective. Both ignored tests can run concurrently and use distinct observed directories.

## cleanup-recovery

Type: failure_prefix_preimage. Observation: remaining mutant source files.
Status: partial_source_model (specific executed evidence stated below).
Normal completion removes the entire per-invocation directory. Panic before remove_dir_all can leave a prefix of mutation files; presence alone is compatible with several failed guard stages. Assumes no external deletion. Oracle: fail after writing a mutant then inspect retained file; prospective. No RAII cleanup or crash durability is modeled.

## serial-prefix

Type: trace_order. Observation: process sequence inside run_specs.
Status: partial_source_model (specific executed evidence stated below).
For n entries, successful local sequence is positive_1, negative_1, ..., positive_n, negative_n. A failed assertion prevents later entries; independent Rust test functions may interleave. Oracle: instrument command invocations and fail entry k; prospective. The new frozen test is separately selectable for bounded development checks.

## conditional-cost

Type: cost_approximation. Observation: wall time and log storage.
Status: partial_source_model (specific executed evidence stated below).
Under successful terminating IO, local wall time is sum of 2n Lean process times plus source/log IO; process budget is 180n seconds, excluding scheduling and cleanup. Captured stdout/stderr consume O(total log bytes), not constant space. A parallel test runner can overlap the two suites. Oracle: external timings and log size; no calibrated throughput claim.

## installed-dependency-boundary

Type: environment_contract. Observation: offline execution scope.
Status: partial_source_model (specific executed evidence stated below).
Harness invokes timeout and lake env lean; it issues no package update or fetch command. Success still depends on installed toolchain, cached imported modules and lake behavior. Oracle: absent-cache run in a network-disabled sandbox; unexecuted. Existing cached Mathlib used. gparse Cargo dependencies include CUDA, so standalone std-only rustc harness avoids unnecessary CUDA compilation.

## frozen-union-fiber

Type: reverse_reconstruction. Observation: compatible parent frozen sets.
Status: partial_source_model (specific executed evidence stated below).
For arbitrary sets of source identifiers, B_A(O)={P | A subset O and O\A subset P and P subset O}. Equivalent to A union P=O by MovaFrozenUnion.union_preimage. If A is not contained in O the fiber is empty; membership inside A remains ambiguous. Assumes Rust source-ID collection corresponds to mathematical sets. General specification proof executed; no mechanized Rust refinement.

## implementation-evidence-boundary

Type: correspondence_scope. Observation: general proof versus finite Rust oracle.
Status: partial_source_model (specific executed evidence stated below).
General Lean relation and study.rs's 64 forward/512 inverse three-ID cases provide different evidence: proof over all sets and finite agreement of actual collection behavior. Their conjunction does not imply parsing correctness, provenance or arbitrary execution refinement. Oracle: expand or mutate Rust collection while rerunning its finite test; not part of this harness change. Prior finite counts come from study group's recorded evidence.

## suite-selection

Type: coverage_projection. Observation: ignored-test filters.
Status: partial_source_model (specific executed evidence stated below).
Default test invocation skips both environment-dependent tests; --ignored runs them, and an exact name filter selects only its suite. Thus a green default test run is compatible with zero Lean checks. Oracle: list tests then run frozen filter and all ignored tests. Focused test passed; full-suite evidence is recorded separately in model.md#tests. This projection prevents treating an ignored result as proof execution.

## tests

### Formal harness evidence

no toolchain or mathlib downloads. Run commands from the repository root:

```sh
CARGO_MANIFEST_DIR="$PWD/gparse" rustc --edition=2021 --test gparse/tests/formal_neural.rs -o "$MOVA_DATA/formal-neural-check"
LEAN_PROJECT="$MOVA_DATA/lean/mathproofs" "$MOVA_DATA/formal-neural-check" frozen_union_preimage_and_false_formula_control --ignored --nocapture
LEAN_PROJECT="$MOVA_DATA/lean/mathproofs" "$MOVA_DATA/formal-neural-check" --ignored --nocapture
```

The std-only standalone Rust harness is the actual integration-test source,
compiled with its Cargo manifest path supplied. This avoids gparse's CUDA build
dependencies. An initial offline cargo test build was stopped after noticing
those unconditional dependencies; no Cargo result is claimed.

Focused result: one passed, one filtered, 2.27 seconds. Full harness: two passed,
zero failed, 53.22 seconds; ten specifications, 39 axiom reports and ten rejected
false-formula controls. Timings are single observations, not cost calibration.
MovaFrozenUnion.union_preimage reports exactly propext, Classical.choice and
Quot.sound. No sorryAx or incomplete-proof diagnostic passed the positive audit.

The new negative formula replaces P subset O by P subset A. For A empty and
P=O={x}, the true forward equation holds but the mutated right side fails.
Lean rejects the mutated proof with type mismatch errors; the harness requires
nonzero, non-timeout status and an error diagnostic. The original source passes.

This is a general set-specification proof, not proof of Rust parsing or execution.
The study group separately records its already executed 64 forward and 512 inverse
membership checks over three source IDs; these were not rerun here. No training,
GPU execution, teacher call, accuracy measurement or implementation refinement.
Catalog oracles without an executed result remain prospective.

Local mova-modelcheck changed inventory passed: the changed Rust source is
registered, with no missing model. Local groupants validation found the new
sidecar structurally valid; its only two diagnostics were intentionally stale
ancestor bindings in model-infrastructure and mova, owned by parent integration.
The parent must refresh these after combining branches. A valid catalog checks
references and hashes, not truth of its mathematical descriptions.

## History

### 2026-10-29 · document

Epoch 1, 2026-10-29. The harness audits installed Lean specifications and tests
that deliberately false variants are rejected. Its exact source hash is in the
manifest. Inputs to the mathematical combiner are Rust source, the future AST,
Lean specifications, captured test outcomes, the forward model, invariant and
typed catalog. AST extraction remains missing. Intersect compatible relations
and retain their assumptions; correlated source descriptions are not independent
verification. Preserve failed predictions and actual check scopes across epochs.

Epoch 1, 2026-10-29. Subject: ../formal_neural.rs; exact SHA in manifest.json.
Twenty distinct source-derived views, not twenty proofs. Domain is one harness
invocation unless stated otherwise; filesystem/processes are trusted and stable.
Each section states its relation, assumptions and oracle. Proposed checks are
not executed evidence. The frozen-set theorem proves a mathematical specification.

### 2026-10-29 · tests

2026-10-29, base d46b1842a. Installed Lean 4.35.0-rc3 and cached Mathlib;
