# math-rat

## descriptant

### Checked reduced i128 rationals and overflow-free comparison

new checks denominator zero before MIN, divides gcd and normalizes sign. add uses denominator gcd, mul cross-cancels, power squares. cmp compares Euclidean quotients and reverses remainder ordering. int directly creates n/1 without MIN rejection.

## approximant

new checks denominator zero before MIN, divides gcd and normalizes sign. add uses denominator gcd, mul cross-cancels, power squares. cmp compares Euclidean quotients and reverses remainder ordering. int directly creates n/1 without MIN rejection.

GCD/comparison O(log magnitude) fixed-width iterations; power O(log|e|) checked multiplications; parse O(L), formatting O(places). No all-input rounding bound.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Checked operations on normalized safe inputs exclude silent float fallback. Universal safety is not claimed: int(MIN), abs and large rounding powers bypass checks. Intermediate overflow can reject a mathematically representable reduced answer; root search may miss exact large roots.

For reduced n/d, new antecedents are fitting pairs(kn,kd) passing zero/MIN checks. Fixed-b addition inverse a=y-b is constrained by checked intermediates. Rounded decimal output has intervals of antecedents, not a unique rational.

Cost: GCD/comparison O(log magnitude) fixed-width iterations; power O(log|e|) checked multiplications; parse O(L), formatting O(places). No all-input rounding bound.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

Checked operations on normalized safe inputs exclude silent float fallback. Universal safety is not claimed: int(MIN), abs and large rounding powers bypass checks. Intermediate overflow can reject a mathematically representable reduced answer; root search may miss exact large roots.

## reverse_reconstruction

For reduced n/d, new antecedents are fitting pairs(kn,kd) passing zero/MIN checks. Fixed-b addition inverse a=y-b is constrained by checked intermediates. Rounded decimal output has intervals of antecedents, not a unique rational.

## normal_form

new success has d>0 and gcd(abs(n),d)=1; zero becomes0/1. int(MIN) violates safe-abs assumptions.

## failure_precedence

new(MIN,0) is DivZero. sub negates right first. div rejects zero before reciprocal. Negative power of zero precedes exponent-negation overflow.

## comparison

Equal Euclidean quotients recurse on reciprocal remainders with flipped order, avoiding cross-products; equal reduced fractions terminate Equal.

## decimal_root_domain

Decimal parser rejects scientific notation, validates grouped commas. Rounding ties away from zero within bounded places. root verifies only three floating-near candidates.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib rat:: -- --test-threads=1`, with MATH_IMPLIED absent. 6 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test rat::tests::associativity_and_commutativity ... ok
test rat::tests::compare_without_overflow ... ok
test rat::tests::decimal_io ... ok
test rat::tests::exact_arithmetic_no_float_error ... ok
test rat::tests::overflow_is_an_error ... ok
test rat::tests::reduces_and_normalizes_sign ... ok
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
