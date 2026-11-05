# world-school-scenario-assessment-fold-residues

## forward

folds=0 returns None. Otherwise ungrouped result is (index*p[repeat mod 5]+repeat) mod folds in u128. Grouped result is (hash XOR wrapping64(repeat*C)) mod folds; index is ignored.

## invariant

Result lies in [0,folds) and equal group hashes stay together for fixed repeat/folds. Composite folds with noncoprime p may have empty residues. Hash grouping cannot prove semantic nonleakage.

## cost

Constant arithmetic, no allocation; native word sizes and widening are part of the model.

## composition

cross_check_k applies the same helper to row selection for both learn !=fold and check ==fold. No dataset or training is executed; helper membership does not certify the provenance of hashed text or extra training rows.

## behavior-complement

Exclude Some for zero folds, out-of-range residues for positive folds, or different folds for identical hash/repeat/folds at different indices. Do not exclude empty folds when gcd(p,folds)>1.

## reverse-reconstruction

Ungrouped observed j admits indices solving p*i ≡ j-repeat mod folds; if gcd divides the RHS there are gcd residue classes, otherwise none. Grouped fiber is all hashes whose XOR-mixed value has residue j; zero-fold None admits every index/hash/repeat.

## partition_membership

For one positive fold count, equality with the returned residue defines exactly one check partition and folds-1 learn partitions per row.

## affine_collapse

repeat=1 uses multiplier7; folds=7 makes all ungrouped indices return1. A partition assignment invariant is weaker than balanced coverage.

## repeat_wrap

Grouped repeat*C is explicitly wrapping u64 before XOR; ungrouped index*p uses u128. Debug and release must agree at machine-word edges.

## group_information

Equal hash enforces co-location but distinct texts may collide; fold alone cannot reconstruct a hash, index or source record.

## checks

Exact world library selections: school::wave12_models::assessment_partition_has_exactly_one_check_fold. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
