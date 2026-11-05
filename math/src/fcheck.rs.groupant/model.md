# math-fcheck

## descriptant

### fcheck

eval_f64 projects legacy Expr to SI Option<f64>. Qty multiplies scale powers, Temp converts to kelvin, Convert ignores target. Var=None; arithmetic may return Some(NaN/inf). Calls evaluate arguments first, fixed-arity branches index positions; median sorts, aggregates fold. Independent-event groups compute products p^k and(1-p)^k. Empty Add/Sum0, Mul1, probability all/none1 and at-least0; mean empty NaN, min/max infinities, median empty panics.

## approximant

### Forward and cost

eval_f64 projects legacy Expr to SI Option<f64>. Qty multiplies scale powers, Temp converts to kelvin, Convert ignores target. Var=None; arithmetic may return Some(NaN/inf). Calls evaluate arguments first, fixed-arity branches index positions; median sorts, aggregates fold. Independent-event groups compute products p^k and(1-p)^k. Empty Add/Sum0, Mul1, probability all/none1 and at-least0; mean empty NaN, min/max infinities, median empty panics.

O(N) visited nodes/unit factors plus per-call buffers; median clones/sorts k floats O(k log k)/O(k). Option collect stops at first None; otherwise even ignored extra args evaluate. No recursion cap or numerical error certificate.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

No dimension, arity, probability range or finite-result validation. Missing positional args panic; extras evaluate then may be ignored. Median partial_cmp.unwrap can panic on NaN. Rat conversion rounds. Temperature values are absolute kelvin points. Caller validation can restrict domain but is not done here. No external effects.

eval_f64 projects legacy Expr to SI Option<f64>. Qty multiplies scale powers, Temp converts to kelvin, Convert ignores target. Var=None; arithmetic may return Some(NaN/inf). Calls evaluate arguments first, fixed-arity branches index positions; median sorts, aggregates fold. Independent-event groups compute products p^k and(1-p)^k. Empty Add/Sum0, Mul1, probability all/none1 and at-least0; mean empty NaN, min/max infinities, median empty panics.

SI y admits all supported expression trees/units/conversion wrappers with that magnitude. Known temperature scale restricts affine inverse but machine rounding still merges values. None admits any reached variable without identifying tree or location. Nonfinite output does not identify a unique invalid operation; source representation is lost.

## composition

### Composition

eval_f64 projects legacy Expr to SI Option<f64>. Qty multiplies scale powers, Temp converts to kelvin, Convert ignores target. Var=None; arithmetic may return Some(NaN/inf). Calls evaluate arguments first, fixed-arity branches index positions; median sorts, aggregates fold. Independent-event groups compute products p^k and(1-p)^k. Empty Add/Sum0, Mul1, probability all/none1 and at-least0; mean empty NaN, min/max infinities, median empty panics.

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: eval_f64 projects legacy Expr to SI Option<f64>. Qty multiplies scale powers, Temp converts to kelvin, Convert ignores target. Var=None; arithmetic may return Some(NaN/inf). Calls evaluate arguments first, fixed-arity branches index positions; median sorts, aggregates fold. Independent-event groups compute products p^k and(1-p)^k. Empty Add/Sum0, Mul1, probability all/none1 and at-least0; mean empty NaN, min/max infinities, median empty panics. These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

SI y admits all supported expression trees/units/conversion wrappers with that magnitude. Known temperature scale restricts affine inverse but machine rounding still merges values. None admits any reached variable without identifying tree or location. Nonfinite output does not identify a unique invalid operation; source representation is lost.

## si_transport

Convert(e,target) shares SI value with e. C adds273.15; F=(x-32)*5/9+273.15; K identity.

## empty_fold

Empty addition0/product1; empty events all/none1/at-least0. Mean NaN, min/max infinities, median panic differ.

## partial_failure

Variable gives None, invalid float domain gives Some(NaN/inf), malformed arity/sort may panic. These observations are distinct.

## probability_observation

AtLeast=1-product(1-p)^k; None=product(1-p)^k; All=product p^k. Independence/range/count assumptions are external.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test fcheck::wave28::si_temperature_empty_and_nonfinite ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-fcheck.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.
