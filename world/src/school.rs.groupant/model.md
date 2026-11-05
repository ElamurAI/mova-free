# world-school

## model

# world-school

## forward

The school composes exact elementary arithmetic, word-feature selection, mutable averaged-perceptron state, vote-based actor effects, synthetic curricula, assessment routing and retained-state publication. Evidence here concerns supplied-input native calls, not a live school run.

## invariant

Op results are representable i64 or None. Selection ties use fixed discriminants. Student serialization preserves accumulator activation and numeric prediction for format-safe keys. Every supplied item belongs to exactly one valid assessment fold. Learning updates and world arithmetic require representable intermediate i64 values.

## cost

Compose child costs without double-counting shared word extraction. Checked arithmetic and partition remain O(1); i128 score/vote widening changes scalar representation only. TSV encoding O(K log K+B) includes union of keys; actor decoding O(R log V log E) with O(VE) storage. Parsing, teacher, training and I/O remain opaque; no timing gain.

## composition

main.rs dispatches main_args. Actual Student/Actors load/save delegate to pure codecs. Student answer calls choose then Op::apply; learn/tick update exactly the maps serialized by save. cross_check_k calls assessment_fold for both learn/check membership. school_tree consumes parent Reading/Ev helpers. Native fixtures execute supplied Reading -> play -> answer_of and learn -> encode -> decode -> answer. Parent public-API joins are separate evidence.

## epoch-memory

Initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.

## checks

cargo test -p world --lib school::wave12_models -- --test-threads=1. Root model.md#evidence binds exact source/evidence hashes, repeated debug/release outcomes and a compiled production mutation. Named fixture domains appear in composition. Syntax is a source-bound partial projection; macros remain opaque. No semantic AST or live school execution.

## behavior-complement

Subject/revision: manifest hash and byte scope. Universe: supplied finite inputs, fixed dependencies, successful allocation and the explicit arithmetic premises of this scope. Inner absent-behavior relation: Exclude (op,a,b,Some(z)) when checked arithmetic is invalid or z is the wrong exact result. Exclude selections violating maximal i128 score or the smallest-discriminant tie rule. Exclude codec transitions that synthesize accumulator presence or remove an explicit zero accumulator. Exclude assignment to two check folds. Under representable updates, exclude hint bonuses on a correct chosen operation. External parser/teacher behavior is not excluded. These are source-derived constraints with finite named witnesses where indicated; external behavior remains unknown.

## reverse-reconstruction

Subject/revision: manifest hash. Operator: compatible preimage/converse, holding dependencies and admissibility fixed. All operand pairs yielding the observed checked result; all weight tables satisfying the winner inequalities; all vote maps with the observed elected marginals; all index/hash residues matching the observed fold; and all codec states with the same canonical rows and accumulator activation. Retain shared identities when composing these fibers. Reports and summaries lose training history; equal answers do not identify operations or exercises. Partial relational description, not a complete inverse search or a unique causal history.

## coupling

Subject/revision: manifest hash. Constraint relation on stage identities and ordered intermediate states: main.rs dispatches main_args. Actual Student/Actors load/save delegate to pure codecs. Student answer calls choose then Op::apply; learn/tick update exactly the maps serialized by save. cross_check_k calls assessment_fold for both learn/check membership. school_tree consumes parent Reading/Ev helpers. Native fixtures execute supplied Reading -> play -> answer_of and learn -> encode -> decode -> answer. Parent public-API joins are separate evidence. Supplied native joins and source inspection are distinct evidence.

## observation-fiber

Subject/revision: manifest hash. Domain: scoped inputs with the declared observation. The information-loss relation equates inputs whose observation agrees: Equal numeric answers can result from different operations, e.g. 2+2=2*2. Training checks numeric equality, so an incorrect operation with the right number can tick without correction. Identical w with different acc can predict differently; graduation drops acc and therefore merges these states. Finite tests do not establish unrestricted completeness.

## evidence

### School native evidence

Sixteen school::wave12_models tests passed twice in debug and twice in release, with equal normalized named identities/results; exact restoration after the mutant passed all sixteen again. Commands: cargo test -p world --lib school::wave12_models -- --test-threads=1 and the same with --release. This is a focused suite; 365 other tests were filtered out, including the existing elementary-lesson test. Existing unrelated compiler warnings remain; no full-world Clippy or whole-suite claim.

#### Witnesses and correction semantics

- Old Op/score/marginal-vote witnesses failed with overflow. Checked Op arithmetic returns None; i128 aggregation preserves ordering, including MAX+MAX scores.
- Extracted old fold arithmetic failed at the second grouped repeat and at extreme ungrouped indices. Grouped hashing now explicitly wraps u64; ungrouped affine arithmetic computes mathematical modulo in u128.
- A legacy current-only TSV state answered 6 before saving and 10 after reload. Serialization now preserves missing accumulators. Review witnessed the opposite failure from dropping a zero accumulator: 10 became 6; explicit zero rows now persist. Orphan accumulators serialize with zero current rows; current-only zero rows remain omitted.
- Duplicate actor rows MAX,1,-1 previously panicked before cancellation. Decode sums in i128, then clamps the final total once to i64. Saturation is explicitly lossy beyond i64; representable duplicate totals are order independent.

#### Finite domains and retained-state boundaries

The independent arithmetic oracle covers 625 operand pairs times four operations, plus five extreme invalid cases. Scoring checks actual tie order, reason truncation and accumulator precedence. The update fixture checks real hinted correction/tick maps then actual codec reload and identical answer. Actor fixtures cover partial sentinels, independent marginal election and duplicate clamping. A supplied Reading -> play -> answer_of join preserves transfer total twelve and answers ten; no parser runs. Fold oracles check repeats 0..8, folds 1/2/5/7, four indices including usize::MAX and four group hashes. Constant template checks cover elementary split/order and explicitly acknowledge scenario overlap.

Codec exactness assumes keys contain no tabs/newlines and do not begin '#'; Student public zero-only/orphan map shapes canonicalize, and Actors empty inner maps are omitted. Native tests use retained-format keys and nonempty actor vote maps. They do not certify arbitrary string-key serialization, filesystem transactions, crash consistency, fresh-process activation or graduation, whose export omits acc and Actors votes.

#### Negative control and restoration

The compiled production mutant changed Add.checked_add to Add.checked_sub. school::wave12_models::arithmetic_matches_widened_integer_oracle failed its named equality assertion, with successful compilation. Exact source bytes were restored and hashed before the sixteen-test positive rerun. Compile errors are not counted as mutant rejection.

#### Structural evidence

Two fresh partial syntax projections compare byte-identically. Two groupant reports compare byte-identically with zero diagnostics and valid=true. The external full-base registry scaffold adds an unmaterialized wrapper root over the unchanged registered tree and this leaf: one missing wrapper group is explicit, not production coverage. Eight fragments include two nested Student children; root and every fragment have concrete complement/preimage/constraint/information-loss relations. Changed-source validation covers only world/src/school.rs with no missing registration. Central registry and parent bindings belong to integration.

#### Limits retained after review

Student tick/learn, table/world arithmetic, fractional divisors and scenario generators still assume admissible bounded input. Exact elementary templates differ, but ordinary scenario giving pools overlap. Noncoprime fold multipliers can leave folds empty; hashes/partitions do not authenticate source separation. No live school, parser, teacher, course, heldout, active-state I/O, site publication or GPU run was performed. No speed, accuracy, semantic-AST or universal proof claim follows.

#### Raw evidence bindings

Raw logs remain in the external task run under these basenames; SHA-256 binds exact bytes.

## wave34-composition

### Wave34 composition

Student codecs feed table selection and later answer arithmetic; Actors codec feeds independent marginal election, ordered event-play and first-selected answer schema. assessment_fold is separate and supplies complementary train/check membership predicates. No live orchestration is validated. Costs retain separate word-feature extraction; do not sum overlapped parent/child source costs.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## wave34-evidence

### Wave34 current evidence

#### Current selections

- school::wave34_models::answer_priority_preserves_shadowed_absence
- school::wave12_models::retained_student_format_is_ordered_and_legacy_tolerant
- school::wave12_models::legacy_retention_does_not_change_answer_table
- school::wave12_models::zero_accumulator_activation_survives_retention
- school::wave12_models::actor_retention_and_partial_vote_projection
- school::wave12_models::supplied_reading_transfer_and_fraction_use_prior_state
- school::wave12_models::assessment_partition_has_exactly_one_check_fold

#### Controls

Two compiled production mutants in the worker slice are rejected by named assertions (exit 101), with exact original backup restoration and green selected reruns (exit 0): inclusive span changed to exclusive; restore drops saved CR, causing duplicate restoration count to become two. The latter uses a custom assertion message, not an incidental runtime panic. No compile failures or survivors are counted as kills.

#### Provenance and limits

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.

### 2026-10-30 · evidence

Heading `### School native evidence, 2026-10-30` stays in evidence as `### School native evidence`.

Final source SHA-256: dc5128329bbebd814b03529e075bc05be358ce4d7500dcab325af8236a2d39a9.

| artifact | SHA-256 |
|---|---|
| old-witness.log | 23ad32a8b1601467e3c47a2bc1c58e06f01111e4fd33ac9282cdaa0686a97d1f |
| split-witness.log | abafd41d156e1e6b1b561a70aa04c6a44f77ec54c93ed374fe64f2883524162f |
| retention-witness.log | fc07778cb0e9a0ce6f7f6f0d115945263ba4fa7d900429a4470abd7c12506a0f |
| review-witness.log | cff97af2d2f2a8d362c3cd9a54a5353c9fdeba7a8c820ff0ae28701250a3eb5d |
| final-debug-1.log | 2cc531a861bba6f4015ec86e64a1b8f8131a34cfb63b89b350dcdda53345b4fa |
| final-debug-2.log | 458b33fe442044ae6acd805126100296729af61c8989f603f825b46c9ca03fd6 |
| final-release-1.log | 601ddffefeaf13ba771ede66d8a3599551c629fa359a351cff8da9f2188448fe |
| final-release-2.log | e90f48a424a70c407ff16aea74ee3590af0f915f8bb653cea6d14199383cfb6d |
| normalized-tests.txt | 81fae03203a074620304b7c2fcf8eaf2e2f4df02e56a33aff3e0e7ead1a84a47 |
| mutant.log | 7fdfeecce1e6b6bf115b5c5a273a5f266ee5dc99cafdf769c3cac69f6ad2208a |
| mutation-summary.log | 6077ef6be1e95ab632711e632c71497a7aed468ac7b693de1a381ce6295ffa76 |
| restored.log | 9411772b64f4102f5ea0e6b9073f37d5991436a8bbabbea7a75b8fa71597dbb9 |
| syntax-1.json | 8f5f77641aced44fee33579ace4b3a7335f45fe7cb353eb796a3ccd3f64980f5 |
| syntax-2.json | 8f5f77641aced44fee33579ace4b3a7335f45fe7cb353eb796a3ccd3f64980f5 |
| groupants-1.json | 76466cc6cae22c8520f58938a0557f037678d277b7dca7a9599d2dc5abd2bf5c |
| groupants-2.json | 76466cc6cae22c8520f58938a0557f037678d277b7dca7a9599d2dc5abd2bf5c |
| changed.json | c1c5b4618691363134c0f72800a8a5debff3a429ab375a5b163c998c8813de66 |

### 2026-10-31 · wave34-evidence

Heading `### Wave34 current evidence — 2026-10-31` stays in wave34-evidence as `### Wave34 current evidence`.

Source SHA-256: 06aaec6f562c2acc167046945b0bc2737bb8bcda4fa16de616e7124ba848a69b. Five genuinely new nested groupants, six typed views each; earlier models and IDs preserved. Current native evidence is 7 exact tests, each passing twice debug and twice release. Two appended tests across the school slice cover priority/None shadowing and restore preflight/duplicate selection. Production bytes are unchanged; the original source is an exact prefix and the appended ranges are cfg(test) only.

Raw source-bound receipts, normalized target-qualified outcomes, syntax pairs, backup hashes and prospective model are in $MOVA_DATA/runs/octopus-wave34-school-2026-10-31. Syntax generated twice is byte-identical, partial and macro-opaque. Historical model.md#evidence and previous model checks remain historical. Supplied helpers and temporary files only: no live lessons, model loads, corpus, teacher, frozen suites, network or publication. World arithmetic retains representability/nonzero-divisor premises. No performance improvement or approximate-error cap is claimed.

### 2026-11-05 · school data outside the repository

Doc comment: the lessons written as data live in courses/lessons of the school data.
