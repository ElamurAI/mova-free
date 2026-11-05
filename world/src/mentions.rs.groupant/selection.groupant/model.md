# world-mentions-selection

Source-bound deterministic projections, with explicit dependency and observation boundaries.

## forward

Tokenize normalized text by alphanumeric/$ boundaries; scan sentence numbers in order; infer thing/adj/verb/context/rate and holder comparison heuristically. Optional tree features precede optional global known facts. fit = 4*thing_match + sum_adj(3 if matched else -1) + 2*nonempty_verb_match + 2*own_context_matches. best excludes every full-field equal Mention to not, then accepts only a unique strictly positive maximum. more_than selects A then excludes A while selecting B and returns absolute difference; sum_of gates on question cues and at least two selected mentions. Arithmetic domain here requires representable i64 sums/differences.

## behavior-complement

Type: behavior_complement. For admissible input/observation pairs U with dependencies fixed, N = {(x,y) in U : y != F(x)}. This is the exact complement of the stated scoped mathematical relation, and only a source-derived hypothesis about unsampled Rust behavior. Unknown external transitions are outside U, not forbidden. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## reverse-reconstruction

Type: reverse_reconstruction. B(y) = {x in D : F(x)=y}, retaining every compatible antecedent and fixed dependency state. Thus x belongs to B(F(x)); unique recovery requires injectivity. Enumeration establishes complete fibers only on each named bounded test domain. Strings, omitted fields and hidden configuration are not recoverable from a projected output. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## positive-unique

Type: constraint_relation. Selected record must uniquely maximize strictly positive fit among records not equal to excluded record. The 64-triple oracle validates selection independently of sort implementation. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## sum-gate

Type: constraint_relation. sum_of first requires a cue, matching things and at least two records. If distinguishing marks vary with positive maximum, it keeps every positive-mark mention, not just maximum-mark mentions. Whole-question token matches do not imply semantic relevance. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## invariant

Preserve ordered observations and all compatible antecedents; input indices and arithmetic must satisfy the scoped preconditions. Equality of a projection does not identify original text or intended meaning. Knowledge tables and normalization are part of the input context.

## cost

For n tokens and m mentions, nested scans can be O(n^2) plus ordered-set/string costs; fit O((adj+ctx+1)*log d) comparisons for d description words; best O(m log m) sort and O(m) auxiliary entries. Parsing, normalization, dictionary facts and global lookup costs are external. Cache retains parsed sentence data without eviction; no speed claim.

## composition

Compose fragment relations by joining shared token order, Mention/Node fields and the same dependency snapshot. Reverse composition is a union over compatible intermediate states, not a Cartesian product of independent fibers. Root includes CLI/reporting and inline tests beyond the scoped fragments. Partial syntax does not resolve types, expand macros or prove semantics.

## checks

Run cargo test -p world --lib mentions::wave8_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles, compiled controls and syntax identity. Each fragment inherits only checks addressing its source scope.

## epoch-memory

First epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.

## limitations

No live annotator, teacher, GPU, training or filesystem report pipeline is exercised. Global tables are fixed build dependencies; parser cache/environment and report I/O are opaque. Native finite fixtures are not language-accuracy validation. No useful numeric approximation metric is established for categorical outputs; executable surrogates remain absent. CLI malformed-tree indexing and i64 overflow outside representable arithmetic domains remain unverified.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.
