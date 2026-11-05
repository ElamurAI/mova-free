# world-mentions

## model

# world-mentions

Source-bound deterministic projections, with explicit dependency and observation boundaries.

## forward

Tokenize normalized text by alphanumeric/$ boundaries; scan sentence numbers in order; infer thing/adj/verb/context/rate and holder comparison heuristically. Optional tree features precede optional global known facts. fit = 4*thing_match + sum_adj(3 if matched else -1) + 2*nonempty_verb_match + 2*own_context_matches. best excludes every full-field equal Mention to not, then accepts only a unique strictly positive maximum. more_than selects A then excludes A while selecting B and returns absolute difference; sum_of gates on question cues and at least two selected mentions. Arithmetic domain here requires representable i64 sums/differences.

## behavior-complement

Type: behavior_complement. For admissible input/observation pairs U with dependencies fixed, N = {(x,y) in U : y != F(x)}. This is the exact complement of the stated scoped mathematical relation, and only a source-derived hypothesis about unsampled Rust behavior. Unknown external transitions are outside U, not forbidden. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## reverse-reconstruction

Type: reverse_reconstruction. B(y) = {x in D : F(x)=y}, retaining every compatible antecedent and fixed dependency state. Thus x belongs to B(F(x)); unique recovery requires injectivity. Enumeration establishes complete fibers only on each named bounded test domain. Strings, omitted fields and hidden configuration are not recoverable from a projected output. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## score-fiber

Type: reverse_reconstruction. The score projection forgets value, sentence, rate, raw words, holder and tree. The 256 own/shared mask pairs partition into score fibers; shared-only context contributes zero. Distinct descriptions can therefore have equal scores. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## identity-exclusion

Type: constraint_relation. best uses derived full Mention equality, not pointer or occurrence identity. Excluding a record removes all exact duplicates, while changing sentence alone restores eligibility without changing fit; native equality_exclusion_and_identity_projection is the witness. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## tie-boundary

Type: priority_relation. On 64 triples of two-bit thing/verb features, enumerate the unique positive maximum independently. Zero winners, all nonpositive scores or multiple maximum occurrences return None; stable sort order cannot resolve a tie. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## question-context

Type: state_transition. with_question controls told question mentions, while asked_holder scans questions in either mode. Thus filtering all-mode output need not always equal story-mode output when question holder transitions affect later sentences; the native fixture has a final question. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## dependency-gates

Type: constraint_relation. MOVA_TREE exactly 1 enables parse features; MOVA_NOFACTS absent enables facts only for all-mode. MOVA_DICTFACTS presence enables dictionary fallback when global measures are empty. Known-fact value dedup ignores units and emits sentence usize::MAX. These environment-sensitive paths are source-inspected only. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## lexical-loss

Type: information_loss. words lowercases and splits hyphens/sign punctuation; stem strips one ordered suffix after irregular mapping. This is a matching key, not morphological identity. Names are collected only from noninitial capitalized words; holder truth can remain None even for an English name. Subject/revision: this manifest's source hash. Domain and observation are restricted by forward and the named view. Evidence is finite native correspondence where named, otherwise source inspection; no universal Rust proof.

## invariant

Preserve ordered observations and all compatible antecedents; input indices and arithmetic must satisfy the scoped preconditions. Equality of a projection does not identify original text or intended meaning. Knowledge tables and normalization are part of the input context.

## cost

For n tokens and m mentions, nested scans can be O(n^2) plus ordered-set/string costs; fit O((adj+ctx+1)*log d) comparisons for d description words; best O(m log m) sort and O(m) auxiliary entries. Parsing, normalization, dictionary facts and global lookup costs are external. Cache retains parsed sentence data without eviction; no speed claim.

## composition

Compose fragment relations by joining shared token order, Mention/Node fields and the same dependency snapshot. Reverse composition is a union over compatible intermediate states, not a Cartesian product of independent fibers. Root includes CLI/reporting and inline tests beyond the scoped fragments. Partial syntax does not resolve types, expand macros or prove semantics.

## checks

Run cargo test -p world --lib mentions::wave8_models -- --test-threads=1. Root model.md#evidence records repeated profiles, compiled controls and syntax identity. Each fragment inherits only checks addressing its source scope.

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

Source SHA-256: a2a239daf36ac4f2d4ceba92ecebdbd7a519128ff01f57a0e9041eaf7c1215dd. Partial syntax SHA-256: 28ef4a820b2a2a817d0df88a6f0b13c3001280c76ef108537c58ac9e1d1e0984. Syntax generated twice and byte-compared. Isolated structural checks cover each source and three fragments; ancestor integration is separate.

External run label: octopus-wave8-mentions-2026-10-30. Relative evidence names and SHA-256:

- debug-1.log: a7d7cc7aaa1ee1ed84e463eba5424fab3505b41b97fac81e103d2524feb8e01c

- debug-2.log: 1c77224ebd52ce65e68c537fd6b8584af6ae9933ff5ea91b80ded229142881f5

- release-1.log: 284c98dd7b99f72bd2445dccf6e0c5794917310341d32ace210f03a9c664d15e

- release-2.log: 26cbc32e712428cfb651d0ec29728caefa854a93683bc2a32f803cf5fd045556

- normalized.txt: 9a888fe1e329c2f0b52981f7255914dd7cc5d3c47876c88f0349a3bececb4dee

- mentions-mutant.log: d8286581b7fa5498c4fbc394e6c75d866cc87f32f982ac63595355eb5b3fefdb

- sense-mutant.log: fa5ad351b370c7073dcac9c82177f996d70e21d9e2a75c28679d98f824d79df4

- restored.log: 109f056d4d6b8e6cb87cd22876f6fae0440551c4afd428abf6554bc13a4bc6f2
