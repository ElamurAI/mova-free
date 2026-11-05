# math-qread-gate-occurrence-partitions

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Flatten source occurrences for expression leaf keys. Reject fewer than two occurrences or abs(e.c)>1e-9 before duplicate tests. Finite nonzero constants with magnitude at most 1e-9 pass this local predicate; it is not an exact-zero requirement. With all_used false, reject any repeated occurrence globally. With true, check repetitions separately inside compound and simple leaf partitions. Then require q_leaf0..nums and, when all_used, all0..nums.

## Invariant

An accepted state under valid indices has enough occurrences and covers each required index. all_used allows one cross-partition reuse but rejects within-partition reuse. Earlier numerical gate predicates must already have passed.

## Cost

Flatten O(S), sort O(S log S); partitioning repeats bounded scans/sorts. Required-index contains scans add O(nums*S), with O(S) temporary storage. No change to the existing algorithm is made.

## behavior_complement

Acceptance excludes duplicate occurrences within the same partition under all_used. Global duplicates across the two partitions are not excluded then. Missing required question index excludes acceptance before the all-body omission reason.

## reverse_reconstruction

Accepted8 admits provenance of4*3-4 with all_used=true and direct5+3. The same duplicate reason admits many different values because provenance, not values, controls it; earlier gate reasons constrain compatibility.

## cross_partition_reuse

compound(a,b)-a repeats a across partitions: false mode rejects, true mode accepts when all required occurrences are present. Fixture toggles only all_used.

## question_before_body

A missing j in q_leaf0..nums returns question-unused even when all_used also lacks it. Moving q_leaf0 to nums changes the same omission to two-step-unused.

## occurrence_not_value

Two distinct leaves with equal numeric values have distinct occurrence indices and do not duplicate by equality of values. Compound leaves concatenate occurrence lists, preserving multiplicity.

## coefficient_projection

This stage flattens keys independent of coefficient sign; earlier gate checked near-unit magnitude. Negative uses still consume occurrences. Zero terms already removed by affine add are absent.

## Tests

Actual helper fixture: `qread::wave34::provenance_partitions_and_required_occurrences`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
