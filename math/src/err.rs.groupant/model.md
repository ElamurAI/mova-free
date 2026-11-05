# math-err

## descriptant

### Error categories, retained payloads and diagnostic formatting

Error stores operation, dimensions, thermal misuse or diagnostic text. code maps7 variants to category strings; Display includes payloads and dimension names. R<T> is Result<T,Error>; no numeric computation here.

## approximant

Error stores operation, dimensions, thermal misuse or diagnostic text. code maps7 variants to category strings; Display includes payloads and dimension names. R<T> is Result<T,Error>; no numeric computation here.

code O(1), Display O(payload+dimension name length); owned allocation occurs at caller construction. No external effects beyond formatter sink.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

A fixed Error variant excludes alternate or payload-dependent code labels. This module does not enforce which error callers choose or turn errors into numeric success; formatter failures propagate as fmt::Error.

ERR:temp preimage contains7 TempMisuse variants; ERR:domain contains every static Domain payload. Codes recover category only, not operation/dimensions/text. Display is not a parsing or serialization contract.

Cost: code O(1), Display O(payload+dimension name length); owned allocation occurs at caller construction. No external effects beyond formatter sink.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

A fixed Error variant excludes alternate or payload-dependent code labels. This module does not enforce which error callers choose or turn errors into numeric success; formatter failures propagate as fmt::Error.

## reverse_reconstruction

ERR:temp preimage contains7 TempMisuse variants; ERR:domain contains every static Domain payload. Codes recover category only, not operation/dimensions/text. Display is not a parsing or serialization contract.

## category_mapping

Overflow,DivZero,Dim,Temp,Domain,CheckFailed,NotUnderstood map to ERR:overflow/div0/dim/temp/domain/check and NU.

## payload_preservation

Clone/PartialEq preserve payload and dimensions; code intentionally erases detail. Display includes operation for overflow and dimensional failures.

## thermal_taxonomy

AddPoints,ScalePoint,SumPoints,NegPoint,DeltaMinusPoint,MixedWithPlain,KindMismatch remain distinct despite common ERR:temp.

## failure_boundary

Branch precedence belongs to Rat, units and eval call sites. Error creation itself performs no validation or automatic recovery.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib err:: -- --test-threads=1`, with MATH_IMPLIED absent. 1 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test err::wave27_tests::codes_preserve_categories_not_payloads ... ok
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
