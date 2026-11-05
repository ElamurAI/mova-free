# world-sense

## model

# world-sense

Source-bound deterministic projections, with explicit dependency and observation boundaries.

## forward

nodes preserves form/head/relation/UPOS and lowercases lemma, falling back to form for empty/_ lemma. animacy chooses first category from exactly one nonempty animate/inanimate family after exception suppression; both or neither means Unknown. violation checks role-specific fixed selection lists. arguments selects first noun child by index, with AuxPass presence switching agent/patient relation sets. check processes only VERB nodes; after any violation a known, clean swapped pair yields one RoleSwap flag, otherwise subject then object flags. run aligns gold by nonempty sent_id map or position and skips misaligned scoring.

## behavior-complement

Type: behavior_complement. For admissible input/observation pairs U with dependencies fixed, N = {(x,y) in U : y != F(x)}. This is the exact complement of the stated scoped mathematical relation, and only a source-derived hypothesis about unsampled Rust behavior. Unknown external transitions are outside U, not forbidden. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## reverse-reconstruction

Type: reverse_reconstruction. B(y) = {x in D : F(x)=y}, retaining every compatible antecedent and fixed dependency state. Thus x belongs to B(F(x)); unique recovery requires injectivity. Enumeration establishes complete fibers only on each named bounded test domain. Strings, omitted fields and hidden configuration are not recoverable from a projected output. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## argument-fiber

Type: reverse_reconstruction. First noun child masks later attachments: four head choices for a later subject yield the same first-agent projection. Proper names/pronouns are excluded even with matching relation; 24 relation/voice/POS cells use a literal expected table. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## lexical-unknown

Type: constraint_relation. Unknown animacy produces no violation for either role under any verb; lack of flags does not prove plausibility. Three verbs and two roles test an unknown lexical witness, with known mouse/cheese anchors. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## swap-priority

Type: priority_relation. RoleSwap requires at least one current violation, two known nouns and no swapped violation; then it suppresses separate subject/object flags. No current violation exits before considering swaps. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## voice-coupling

Type: encoding_relation. The active cheese-agent/mouse-patient and corresponding passive attachments with AuxPass share kind/verb/words projection. Removing AuxPass changes the interpretation; voice is not inferred from a passive subject alone. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## verb-pos-gate

Type: constraint_relation. AUX or missing/non-VERB tags skip check entirely even for identical lemma and attachments. Noun-only selection likewise prevents proper-name flags. Native gate fixtures isolate this policy. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## gold-projection

Type: information_loss. Strict precision uses any flagged noun head/relation disagreement; lenient adds verb head/relation/POS. Flags count once while recall counts erroneous arcs, so those denominators cannot be equated. Duplicate gold IDs keep the last map insertion; misaligned forms/lengths are not scored. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## invariant

Preserve ordered observations and all compatible antecedents; input indices and arithmetic must satisfy the scoped preconditions. Equality of a projection does not identify original text or intended meaning. Knowledge tables and normalization are part of the input context.

## cost

For n nodes, arguments scans n children per verb and check costs O(v*n) plus lexical queries/string construction; O(n+output bytes) transient storage. Category filtering costs category-count times selection-list membership cost. Report matching uses ordered maps O(s log s) and token comparisons; exact global lookup/I/O costs are external.

## composition

Compose fragment relations by joining shared token order, Mention/Node fields and the same dependency snapshot. Reverse composition is a union over compatible intermediate states, not a Cartesian product of independent fibers. Root includes CLI/reporting and inline tests beyond the scoped fragments. Partial syntax does not resolve types, expand macros or prove semantics.

## checks

Run cargo test -p world --lib sense::wave8_models -- --test-threads=1. Root model.md#evidence records repeated profiles, compiled controls and syntax identity. Each fragment inherits only checks addressing its source scope.

## epoch-memory

First epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.

## limitations

No live annotator, teacher, GPU, training or filesystem report pipeline is exercised. Global tables are fixed build dependencies; parser cache/environment and report I/O are opaque. Native finite fixtures are not language-accuracy validation. No useful numeric approximation metric is established for categorical outputs; executable surrogates remain absent. CLI malformed-tree indexing and i64 overflow outside representable arithmetic domains remain unverified.

## evidence

### Native correspondence evidence

Mentions checks 256 score cells, 64 unique-maximum triples, equality exclusion, lexical loss, question/rate/context and holder dependencies. Sense checks 24 relation/voice/POS cells, four attachment fibers, lexical anchors, swap precedence and active/passive projections.

Compiled production mutants: thing score 4 to 5 fails weighted_score_oracle_and_fibers; admitting PROPN as noun fails argument_projection_table_oracle. Both compile and reach named assertion failures; both restored safely.

Limits: bounded native evidence, no parser/teacher/live state, gold report I/O, language accuracy or performance claims. i64 results must be representable; malformed reporting indices remain outside tested scope. Initial reduced registries against the whole repo failed omitted ownership/unreachable-node checks; corrected validation uses isolated snapshots and full temporary registry wiring.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first epoch. Prediction before tests: bounded native models match deterministic scoring/role projection and preserve ambiguity; changing a score weight or admitting proper names should reject. No production behavior change is proposed. Case normalization, full-field equality and first-child priority are observable policy, not semantic identity.

### 2026-10-30 · evidence

2026-10-30. Five new tests per source, ten combined. No production behavior changed. Ten pass twice debug and twice release; normalized identities/results identical. Restored source passes ten again. Filter: wave8_models with --test-threads=1. Existing unrelated compiler warnings remain.

Source SHA-256: 32033a19e1ffaf797f7eabb7a82bc564ddcf3146db700bbb4000ee6eec91a0a8. Partial syntax SHA-256: a8da3b9eeda96753b285cdbb733b71a84e1a0d6a5a69a79ffc08a15df5398948. Syntax generated twice and byte-compared. Isolated structural checks cover each source and three fragments; ancestor integration is separate.

External run label: octopus-wave8-mentions-2026-10-30. Relative evidence names and SHA-256:

- debug-1.log: a7d7cc7aaa1ee1ed84e463eba5424fab3505b41b97fac81e103d2524feb8e01c

- debug-2.log: 1c77224ebd52ce65e68c537fd6b8584af6ae9933ff5ea91b80ded229142881f5

- release-1.log: 284c98dd7b99f72bd2445dccf6e0c5794917310341d32ace210f03a9c664d15e

- release-2.log: 26cbc32e712428cfb651d0ec29728caefa854a93683bc2a32f803cf5fd045556

- normalized.txt: 9a888fe1e329c2f0b52981f7255914dd7cc5d3c47876c88f0349a3bececb4dee

- mentions-mutant.log: d8286581b7fa5498c4fbc394e6c75d866cc87f32f982ac63595355eb5b3fefdb

- sense-mutant.log: fa5ad351b370c7073dcac9c82177f996d70e21d9e2a75c28679d98f824d79df4

- restored.log: 109f056d4d6b8e6cb87cd22876f6fae0440551c4afd428abf6554bc13a4bc6f2
