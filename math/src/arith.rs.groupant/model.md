# math-arith

## descriptant

### Quantity extraction, hashed features, subset beam and exact replay

prepare truncates extracted quantities then builds ordered pair/relevance features. search enumerates disjoint masks by population count and scores child trees plus crossing pairs. predict adds relevance; learn ticks before prediction and updates mismatches. Exact replay uses big Q and can reject rational zero division.

## approximant

prepare truncates extracted quantities then builds ordered pair/relevance features. search enumerates disjoint masks by population count and scores child trees plus crossing pairs. predict adds relevance; learn ticks before prediction and updates mismatches. Exact replay uses big Q and can reject rational zero division.

O(n²F) feature work, conservative O(3^n B² n²) search plus tree cloning and O(2^n B) retained candidates. Requires n<32 for masks; no enforced general resource cap.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Generated internal candidates exclude nonfinite, negative and >=1e12 values; leaves bypass that filter. Zero divisors exclude division. Beam truncation and round(value*1e6) dedup exclude valid exact alternatives too; no completeness or natural-language correctness claim.

For value y, antecedents are all texts, environments, weight states and retained trees whose exact replay equals y. Gold acceptance instead uses floating tolerance. Repeated values, skipped quantities and commutative operations prevent unique recovery.

Cost: O(n²F) feature work, conservative O(3^n B² n²) search plus tree cloning and O(2^n B) retained candidates. Requires n<32 for masks; no enforced general resource cap.

Finite fixtures do not prove all-domain correctness or resource behavior.

## composition

quantities -> prepare -> subset search -> predict/gold -> Tree::exact. Float pruning and rounded keys can lose exact solutions; replay certifies selected arithmetic, not completeness. Relevance added after search changes cross-mask choice.

## models

## behavior_complement

Generated internal candidates exclude nonfinite, negative and >=1e12 values; leaves bypass that filter. Zero divisors exclude division. Beam truncation and round(value*1e6) dedup exclude valid exact alternatives too; no completeness or natural-language correctness claim.

## reverse_reconstruction

For value y, antecedents are all texts, environments, weight states and retained trees whose exact replay equals y. Gold acceptance instead uses floating tolerance. Repeated values, skipped quantities and commutative operations prevent unique recovery.

## beam_boundary

beam=0 still retains first distinct internal candidate; leaves survive max_leaves=0. Lowest-bit split avoids duplicate partitions.

## tie_policy

Stable finite score sort preserves traversal order; rounded i64 keys saturate for large values. Explain equal-magnitude feature ties depend on HashMap iteration.

## quantity_coordinates

Context is +/-3 token positions. Percent divides by100, tokenizer discards unary minus. Unit constants append; MATH_IMPLIED=1 adds general constants before max_q truncation.

## learning_state

Weights, accumulator and stamps use lazy elapsed-tick averaging divided by max(t,1). Feature multiplicity matters. Functions perform no teacher/corpus/persistent writes.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib arith:: -- --test-threads=1`, with MATH_IMPLIED absent. 3 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test arith::tests::gold_tree_found_and_negative_control ... ok
test arith::tests::quantities_words_percent_consts ... ok
test arith::wave27_tests::exact_tree_and_empty_search ... ok
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
