# world-primers

## model

# world-primers

## forward

Extract numbered OCR arithmetic stories and whole answers, associate page/chapter keys, and optionally filter by a bounded two-operation numeric reachability heuristic. All execution evidence uses supplied strings.

## invariant

Accepted records preserve source book tag and observed numeric answer under the scoped parser. Acceptance is not proof of correct OCR, story semantics, mathematical solvability or external source provenance.

## cost

Inline processing linear text scans plus allocations; paged extraction can scan every section/problem per answer; reachability O(U^3) time and O(U^2) intermediates for U distinct parsed numbers. No runtime improvement claimed.

## composition

main.rs dispatches main_args; extraction reads sorted file paths, then problems -> answer/story helpers -> reachable -> JSON. Paged and OpenStax extraction attach answers by keys before story filtering. Native fixtures exercise string helpers and numeric normalization only.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib primers::wave11_models -- --test-threads=1. See file-root model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: Accepted records preserve source book tag and observed numeric answer under the scoped parser. Acceptance is not proof of correct OCR, story semantics, mathematical solvability or external source provenance. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: All OCR strings and book/section metadata whose selected records and reachability checks yield an observed tuple. Punctuation, ignored lines and book association ambiguities prevent a unique original. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: main.rs dispatches main_args; extraction reads sorted file paths, then problems -> answer/story helpers -> reachable -> JSON. Paged and OpenStax extraction attach answers by keys before story filtering. Native fixtures exercise string helpers and numeric normalization only. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: JSON uses Answer as f64, so values above 2^53 may lose exactness; extracted IDs depend on final enumeration. This preexisting output boundary is modeled, not repaired or exercised against external books. Finite correspondence only; unrestricted soundness/completeness is not established.

## evidence

### Wave11 corpus evidence

Old production witnesses failed for lost frame attrs, earlier-marker priority and integer overflow; the first expanded run also failed for dotted page answer 57. Narrow fixes preserve tested behaviors. Final compiled controls remove frame attrs and select the earliest marker; both fail named equality assertions, then exact bytes are restored and the complete nine tests rechecked in both profiles.

The page-header numeric token is consumed with its header so it cannot become a fictitious item. Tests compare separated and glued page headers with/without em-dash decoration. Missing-answer/next-label ambiguity remains a limit.

Syntax projections were generated twice identically. Full legacy registry plus an external scaffold root validates the two new leaves and seven fragments; repeated structural reports are byte-identical and changed-source registration covers both sources. The absent scaffold sidecar and unmodeled legacy files are explicitly reported; this is structural evidence, not a semantic proof. Central registry/ancestor integration belongs to the integration task.

No benchmark speedup or percentage-error profile is claimed. Narrow rustfmt covers new helper/test regions and answer_of; legacy formatting remains. git diff --check passes. No whole-world Clippy claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

### 2026-10-30 · evidence

2026-10-30. Raw logs: $MOVA_DATA/runs/octopus-wave11-corpus-2026-10-30/. All inputs are supplied synthetic strings; no external datasets, heldout, teacher, parser, GPU or school-state run.

Nine native tests (five fbench, four primers) pass twice debug and twice release with filter wave11_models and --test-threads=1. Normalized test identities/results agree across all four runs: SHA-256 972d11c4eda3cd2e504f80b80b3200c82a910d46cfa94fa09496a3ce28b302e5. Tests include 625 independent category-mask rotations, cap/empty cases, 16 canonical frame roundtrips, TSV ID/text and malformed numeric boundaries, exact score-count join, 36 marker pairs, page-key/segmentation fixtures, and 1476 wide-integer reachability oracle comparisons plus number-word normalization.

#### Evidence hashes

- old-witness.log: a68670bbcf50d158cabc1d420dfa67d4eb1920ba1b812ef045aa5563fde4d32c
- debug-1.log: ad6cc26df36a150b18d8d7e4827bd105f171c8d9000db37a728afa8a81e77025
- final-debug-1.log: d7e90f59ff77149b36505a74df1639c6fd28e010032b8c1389cb54172e46c7b5
- final-debug-2.log: 47a6175544ee6e65474de86b2e40d3f6a66beb349a70503694495e18e21225f0
- final-release-1.log: bdded48f18636d1122d25e2d7bd17edce53b0141ecd2d9923c5b126b176075bd
- final-release-2.log: dd8275279402a216d68cbe48c52c5c551d2f309dc769d65defb48863980a6021
- mutant-fbench.log: c766689bb77eebf924032fa928cae71b18c512ae89a12126cc9cfccc4df2d7a0
- mutant-primers.log: 455915bd143836c2200e1e06e5cac7d708848a76de64f4b2e5c14ea2dbd6df9d
- fbench-syntax-1.json: 0d7cf9f39f1e751bfe98cca00e4771a7b3f1a697a4737fde049601d2cbf770f8
- primers-syntax-1.json: ccb6e4b9948d241fc41abf15370870b5df6565ed4f5281952ad59587025e3646
- groupants-3.json: 84651c567bac98038f7dbb395cc1c5da34cbd72c3b0d93d8726d702584ad45a8
- groupants-4.json: 84651c567bac98038f7dbb395cc1c5da34cbd72c3b0d93d8726d702584ad45a8
- changed.json: 0f15327488b49e6e981c3a61dd983c0f3278efee8856f263078fddb6f3534de5

Bound source world/src/primers.rs: cbe95a74a9eb2777637696d161a3b2412c4305b1c5104f4037ab69d5de56a5e5.
