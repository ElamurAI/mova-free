# onto-eval-answer-scoring

## descriptant

### onto-eval-answer-scoring

S(a,g)=Correct if g∈{yes,no,undetermined} and code(a)=g; otherwise Unknown iff verdict Unknown else Wrong. For g=unknown, Correct iff Unknown else Wrong. For frame g: Correct iff ∃v∈values:v≈ASCII g, else Unknown iff Unknown else Wrong.

## approximant

### Scoped forward relation

S(a,g)=Correct if g∈{yes,no,undetermined} and code(a)=g; otherwise Unknown iff verdict Unknown else Wrong. For g=unknown, Correct iff Unknown else Wrong. For frame g: Correct iff ∃v∈values:v≈ASCII g, else Unknown iff Unknown else Wrong.

Scope bytes [1878,2692) only. Preconditions and exceptions in this scope's catalog; no claims for parent orchestration.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Scoped observations

Frame match excludes Unknown/Wrong status even if verdict Unknown. Gold unknown never yields status Unknown.

## Cost

O(sum of compared value bytes), constant auxiliary state; no IO or mutation.

## composition

### Scope contribution

S(a,g)=Correct if g∈{yes,no,undetermined} and code(a)=g; otherwise Unknown iff verdict Unknown else Wrong. For g=unknown, Correct iff Unknown else Wrong. For frame g: Correct iff ∃v∈values:v≈ASCII g, else Unknown iff Unknown else Wrong.

This fragment contributes only `answer-scoring`. Parent owns input acquisition, surrounding dispatch and downstream joins. Complement and compatible antecedents are scoped in model.md#models rather than copied parent statements.

## models

## behavior_complement

Frame match excludes Unknown/Wrong status even if verdict Unknown. Gold unknown never yields status Unknown.

## reverse_reconstruction

S^-1(Correct,unknown)={a|verdict(a)=Unknown}. S^-1(Correct,frame g)={a|∃v≈ASCII g}; extra values/order/lines unconstrained.

## literal_partition

Four verdict strings reserved, not treated as frames.

## ascii_equivalence

Commerce/commerce collapse; no Unicode normalization.

## fallback_priority

Frame membership precedes Unknown fallback.

## pure_cost

Linear compared bytes, no IO/mutation.

## tests

### Evidence boundary

Scoring, tally, empty majority and tiny hash/UTF8 failure ordering executed. Query execution, graph shuffle/leak/unknown controls/report effects unexecuted; no frozen benchmark set read.

Parent model.md#tests names actual selected tests; this scope does not claim all its branches executed. The compiled control listed in parent receipt applies only to the named mutated boundary.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source byte range [1878,2692) with source and range hashes, sharing parent lexical AST. Partial source-derived scope, no semantic proof.
