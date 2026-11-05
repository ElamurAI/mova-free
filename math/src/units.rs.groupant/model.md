# math-units

## descriptant

### Exact SI factors and affine temperature coordinates

Dim stores length/mass/time/temperature i8 exponents. Mono public ordered Vec merges first matching units and removes zeros. factor checks dimensions then source.scale/target.scale. Scale computes K=(x+offset)*delta_factor and inverse K/factor-offset.

## approximant

Dim stores length/mass/time/temperature i8 exponents. Mono public ordered Vec merges first matching units and removes zeros. factor checks dimensions then source.scale/target.scale. Scale computes K=(x+offset)*delta_factor and inverse K/factor-offset.

Mono mul O(a+b+ab) including cloning, scanning and retention, dim/scale O(k) plus rational powers; table lookup bounded finite. i8 arithmetic is unchecked: debug panic/release wrapping possible outside exponent domain.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Dimension mismatch excludes factor output before numeric scaling. Public duplicate monomials and i8 overflow are not excluded. Scale itself permits negative Kelvin; point-vs-delta restrictions are eval responsibility.

For fixed factors, conversion antecedent x=y*f_to/f_from subject to Rat representability. Fixed scale inverse x=K/f-offset is unique arithmetically. Dimension vectors have many monomial antecedents and lose unit/order information.

Cost: Mono mul O(a+b+ab) including cloning, scanning and retention, dim/scale O(k) plus rational powers; table lookup bounded finite. i8 arithmetic is unchecked: debug panic/release wrapping possible outside exponent domain.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

Dimension mismatch excludes factor output before numeric scaling. Public duplicate monomials and i8 overflow are not excluded. Scale itself permits negative Kelvin; point-vs-delta restrictions are eval responsibility.

## reverse_reconstruction

For fixed factors, conversion antecedent x=y*f_to/f_from subject to Rat representability. Fixed scale inverse x=K/f-offset is unique arithmetically. Dimension vectors have many monomial antecedents and lose unit/order information.

## coordinates

Percent dimensionless factor1/100; speed(1,0,-1,0), pressure(-1,1,-2,0). Units use exact rational definitions.

## affine_temperature

C offset27315/100; F offset45967/100, factor5/9. Deltas never add offsets; exact inverse requires representable intermediate arithmetic.

## ordered_monomials

Equality is Vec equality, not physical equivalence. Public duplicate atoms are not globally canonicalized; inv negates exponents.

## parse_failure

Parser supports exact atoms, one slash and mph alias, not general printed powers. factor checks mismatch then source and target scales in order.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib units:: -- --test-threads=1`, with MATH_IMPLIED absent. 4 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test units::tests::dimension_mismatch_is_error ... ok
test units::tests::exact_definitions ... ok
test units::tests::mono_algebra_and_display ... ok
test units::tests::temperature_is_affine ... ok
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
