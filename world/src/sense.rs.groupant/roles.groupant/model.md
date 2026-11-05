# world-sense-roles

Source-bound deterministic projections, with explicit dependency and observation boundaries.

## forward

nodes preserves form/head/relation/UPOS and lowercases lemma, falling back to form for empty/_ lemma. animacy chooses first category from exactly one nonempty animate/inanimate family after exception suppression; both or neither means Unknown. violation checks role-specific fixed selection lists. arguments selects first noun child by index, with AuxPass presence switching agent/patient relation sets. check processes only VERB nodes; after any violation a known, clean swapped pair yields one RoleSwap flag, otherwise subject then object flags. run aligns gold by nonempty sent_id map or position and skips misaligned scoring.

## behavior-complement

Type: behavior_complement. For admissible input/observation pairs U with dependencies fixed, N = {(x,y) in U : y != F(x)}. This is the exact complement of the stated scoped mathematical relation, and only a source-derived hypothesis about unsampled Rust behavior. Unknown external transitions are outside U, not forbidden. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## reverse-reconstruction

Type: reverse_reconstruction. B(y) = {x in D : F(x)=y}, retaining every compatible antecedent and fixed dependency state. Thus x belongs to B(F(x)); unique recovery requires injectivity. Enumeration establishes complete fibers only on each named bounded test domain. Strings, omitted fields and hidden configuration are not recoverable from a projected output. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## noun-voice-gate

Type: constraint_relation. At fixed verb index, only head=v+1 nouns qualify. AuxPass among any child switches role relation sets. First qualifying index wins. 24 table cells and duplicate-child fibers are finite evidence. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## swap-observation

Type: reverse_reconstruction. RoleSwap kind/indices loses lexical reasons and many token fields. Active/passive fixtures are distinct antecedents sharing a flag projection; the full why strings are not asserted identical. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## invariant

Preserve ordered observations and all compatible antecedents; input indices and arithmetic must satisfy the scoped preconditions. Equality of a projection does not identify original text or intended meaning. Knowledge tables and normalization are part of the input context.

## cost

For n nodes, arguments scans n children per verb and check costs O(v*n) plus lexical queries/string construction; O(n+output bytes) transient storage. Category filtering costs category-count times selection-list membership cost. Report matching uses ordered maps O(s log s) and token comparisons; exact global lookup/I/O costs are external.

## composition

Compose fragment relations by joining shared token order, Mention/Node fields and the same dependency snapshot. Reverse composition is a union over compatible intermediate states, not a Cartesian product of independent fibers. Root includes CLI/reporting and inline tests beyond the scoped fragments. Partial syntax does not resolve types, expand macros or prove semantics.

## checks

Run cargo test -p world --lib sense::wave8_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles, compiled controls and syntax identity. Each fragment inherits only checks addressing its source scope.

## epoch-memory

First epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.

## limitations

No live annotator, teacher, GPU, training or filesystem report pipeline is exercised. Global tables are fixed build dependencies; parser cache/environment and report I/O are opaque. Native finite fixtures are not language-accuracy validation. No useful numeric approximation metric is established for categorical outputs; executable surrogates remain absent. CLI malformed-tree indexing and i64 overflow outside representable arithmetic domains remain unverified.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.
