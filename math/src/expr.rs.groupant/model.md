# math-expr

## descriptant

### Expression tree, occurrence traversal and precedence display

Constructors retain structure without evaluation. has_var ORs occurrences across unary, binary, variadic and both probability children. Display uses precedence and exact decimal constants up to6 places; variable names use fixed8-entry table.

## approximant

Constructors retain structure without evaluation. has_var ORs occurrences across unary, binary, variadic and both probability children. Display uses precedence and exact decimal constants up to6 places; variable names use fixed8-entry table.

O(N) worst-case traversal with short circuit; display O(N+output bytes) plus rational formatting, recursion O(depth); no depth/allocation guard.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

has_var=false excludes the exact variable in all children. Constructors do not exclude empty Add/Mul, invalid variables or unit misuse. Display does not establish parseability, typing or successful evaluation.

has_var=true antecedents are all trees containing at least one occurrence. Display preimages are many-valued: empty Add/Mul both print empty and invalid variable indices print '?'; algebraic equivalence is not structural identity.

Cost: O(N) worst-case traversal with short circuit; display O(N+output bytes) plus rational formatting, recursion O(depth); no depth/allocation guard.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

has_var=false excludes the exact variable in all children. Constructors do not exclude empty Add/Mul, invalid variables or unit misuse. Display does not establish parseability, typing or successful evaluation.

## reverse_reconstruction

has_var=true antecedents are all trees containing at least one occurrence. Display preimages are many-valued: empty Add/Mul both print empty and invalid variable indices print '?'; algebraic equivalence is not structural identity.

## probability_traversal

Both probability and repetition expression are searched; constants/numbers have no variable children.

## precedence

Sub right min2 and Div right min3 protect grouping; power both operands min5. Display precedence does not rewrite evaluation order.

## names_dispatch

Var parse is lowercase exact matching; invalid index names '?'. is_elementary determines evaluator arity dispatch.

## empty_structure

Empty variadic display is empty text; this does not provide an evaluator identity. Formatting effects only through fmt sink.

## tests

### Bounded actual-code evidence

Selected `cargo test -p math --lib expr:: -- --test-threads=1`, with MATH_IMPLIED absent. 2 named tests pass twice debug and twice release; all four sorted outcome sets match and numeric statuses are0.

```text
test expr::tests::prints_with_precedence ... ok
test expr::wave27_tests::variable_occurrence_in_probability_count_and_wrappers ... ok
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
