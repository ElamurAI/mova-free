# math-big

## descriptant

### Arbitrary rational parsing, exact roots and decimal presentation

frac rejects zero denominator and normalizes sign. parse_q strips spaces/underscores, splits slash recursively before comma/exponent processing, then multiplies by10^(exp-fraction_digits). exact_decimal factors denominator by2 and5; roots verify r^k=n.

## approximant

frac rejects zero denominator and normalizes sign. parse_q strips spaces/underscores, splits slash recursively before comma/exponent processing, then multiplies by10^(exp-fraction_digits). exact_decimal factors denominator by2 and5; roots verify r^k=n.

O(L) lexical work plus bit-length-dependent GCD/power/root arithmetic. Decimal output O(places) allocation; no memory quota or allocation error interface.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Zero denominators, degree zero, negative even roots and nonperfect powers are excluded. Scale magnitude above100000 is rejected after subtraction. Comma UTF-8 byte slicing and extreme exponent subtraction remain possible panic gaps; f64 conversions may overflow or round.

Reduced n/d has every representable signed pair(kn,kd), k nonzero, as antecedents and many accepted lexical spellings. Fixed root degree maps r back to r^k. show_nice/f64 are many-to-one; exact show retains rational value.

Cost: O(L) lexical work plus bit-length-dependent GCD/power/root arithmetic. Decimal output O(places) allocation; no memory quota or allocation error interface.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

Zero denominators, degree zero, negative even roots and nonperfect powers are excluded. Scale magnitude above100000 is rejected after subtraction. Comma UTF-8 byte slicing and extreme exponent subtraction remain possible panic gaps; f64 conversions may overflow or round.

## reverse_reconstruction

Reduced n/d has every representable signed pair(kn,kd), k nonzero, as antecedents and many accepted lexical spellings. Fixed root degree maps r back to r^k. show_nice/f64 are many-to-one; exact show retains rational value.

## normalization

RBig reduces fractions and keeps positive denominator; sign XOR handles negative inputs and zero.

## decimal_domain

Exact decimal exists iff reduced denominator=2^a5^b and max(a,b)<=max. show_nice uses12 places before exact fraction plus approximate hint.

## root_certificate

Integer nth_root result is checked by power equality; odd negative roots recurse on magnitude; rational roots require both numerator and denominator certificates.

## failure_order

Slash processing precedes comma and exponent handling. to_i64 requires integral rational before checked integer conversion; parse_q comma acceptance is broader than documented digit triples.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib big:: -- --test-threads=1`, with MATH_IMPLIED absent. 2 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test big::tests::big_exact ... ok
test big::tests::parse_and_show ... ok
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
