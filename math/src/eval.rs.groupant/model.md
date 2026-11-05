# math-eval

## descriptant

### Quantity evaluation and independent answer gate

eval recursively produces Q and appends Ctx evidence. Exact pairs use Rat; mixed approximate pairs use f64. Aggregates convert to first operand units. answer uses fresh context then gate checks independent fcheck SI value and recorded booleans.

## approximant

eval recursively produces Q and appends Ctx evidence. Exact pairs use Rat; mixed approximate pairs use f64. Aggregates convert to first operand units. answer uses fresh context then gate checks independent fcheck SI value and recorded booleans.

O(N) traversal plus numeric costs; aggregate O(k log k); exact probability enumeration O(T*2^T) for T<=16. Trace size includes formatted intermediates; stack/allocation unbounded. Gate tolerance is policy, not universal numerical assurance.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Point+point and point scaling are excluded, dimension mismatch excludes conversion. Gate rejects any false check or SI disagreement; raw eval does not enforce gate. Empty Add/Mul indexing, nonfinite handling, unchecked count sum and exponent narrowing remain gaps.

Output Q admits many Expr/environment/trace antecedents and equivalent units. Fixed-scale exact conversion has unique inverse only when all arithmetic succeeds. Gate accepts SI-equivalent representations, not a unique syntax or provenance.

Cost: O(N) traversal plus numeric costs; aggregate O(k log k); exact probability enumeration O(T*2^T) for T<=16. Trace size includes formatted intermediates; stack/allocation unbounded. Gate tolerance is policy, not universal numerical assurance.

Finite fixtures do not prove all-domain correctness or resource behavior.

## composition

Expr -> eval using Rat/units -> Q plus Ctx -> gate using fcheck. Raw eval success is insufficient: existing checks or SI mismatch can reject. Context mutations survive raw errors; answer owns fresh context. No global relative-error budget follows.

## models

## behavior_complement

Point+point and point scaling are excluded, dimension mismatch excludes conversion. Gate rejects any false check or SI disagreement; raw eval does not enforce gate. Empty Add/Mul indexing, nonfinite handling, unchecked count sum and exponent narrowing remain gaps.

## reverse_reconstruction

Output Q admits many Expr/environment/trace antecedents and equivalent units. Fixed-scale exact conversion has unique inverse only when all arithmetic succeeds. Gate accepts SI-equivalent representations, not a unique syntax or provenance.

## temperature_kind

Point-point subtraction yields delta of first scale; point+delta preserves point. Products normalize dimensionless mixed units and percent factors; even zero cannot scale a point.

## failure_precedence

Children evaluate left-to-right before parent arity/type checks; earlier context survives failure. Point rejection precedes zero division. same_kind precedes aggregate conversion then SumPoints.

## gate_boundary

Independent SI tolerance is1e-9*(1+max(abs(b),abs(fv))). fcheck None records failure. Raw eval success alone is insufficient; checks can overflow while constructing evidence.

## empty_nonfinite

Empty aggregate is NotUnderstood; empty probability products give1 (at-least-once0). Missing variable is Domain; first duplicate binding wins. NaN cmp falls back Equal; no general finite-number invariant.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib eval:: -- --test-threads=1`, with MATH_IMPLIED absent. 4 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test eval::gate_tests::gate_rejects_wrong_answers ... ok
test eval::tests::brief_samples ... ok
test eval::tests::mixed_units_and_points ... ok
test eval::tests::negative_controls ... ok
```

Compiled mutation: build status0, selected test status101, exact original bytes restored, positive rerun status0. Receipt binds failed assertion log hash. Controls are source changes during external experiments only; no mutant committed.

Strict `cargo clippy -p math --lib --no-deps -- -D warnings` exits101 on113 existing non-test diagnostics. Production code is unchanged; only three bounded test modules were added.

No corpus, teacher, GPU, full crate suite, held-out or training runs. Bounded successes do not prove all-domain semantics; malformed/public states, allocation and overflow gaps remain in models. Lexical syntax generated twice with identical bytes; no semantic AST proof.

## prospective

Before edits: inspected all seven complete sources (2939 lines). Model boundaries: arith token quantities and subset beam use f64 pruning [0,1e12), rounded millionth keys, stable score ordering; exact Tree recomputation alone does not certify beam completeness. n<32 required by masks; environment MATH_IMPLIED affects quantities; explain equal magnitudes depend on HashMap order. big uses arbitrary rational reduction and guarded denominator/root domains; parse_q byte slicing commas and exponent subtraction are potential panic gaps. rat new rejects MIN, div-zero precedes MIN, checked operations can overflow before cancellation; int(MIN), abs and huge rounding places bypass those assurances. expr recursively traverses probability counts and values; display is noninjective and is not a serialization grammar. units has ordered Vec monomials and unchecked i8 exponent arithmetic; conversions require same dimensions before rational scale operations; temperatures affine. eval traverses children before parent arity/type validation, keeps prior context on failure, distinguishes raw eval from independent gate, has empty Add/Mul indexing and nonfinite/large-count gaps. err code maps variants many-to-one, retaining diagnostic payload only in Error/Display.
Plan: existing inspected module tests + bounded independent new witnesses, repeated debug/release outcomes, compiled mutations in source with restoration. No dataset, teacher, filesystem product mutation or training. No useful percent surrogate for exact categorical contracts. Nested arith search and eval gate models should make wiring explicit. Runtime witnesses do not establish all-domain correctness.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-derived partial model; bounded synthetic evidence only. No algorithm changes or speed claim.

### 2026-10-31 · tests

Heading `### Bounded actual-code evidence — 2026-10-31` stays in tests as `### Bounded actual-code evidence`.
