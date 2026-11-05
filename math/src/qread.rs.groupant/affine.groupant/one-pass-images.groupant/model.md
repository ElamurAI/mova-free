# math-qread-affine-one-pass-images

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Start at constant-only Lin. Visit original keys in sorted order. Replace key k by sol[k] scaled by its original coefficient if present, otherwise by var(k); each insertion passes through add pruning. Newly introduced keys are not revisited in the same call. unknowns returns keys below LEAF.

## Invariant

Substitution does not recursively close the solution map. Output constants include weighted solution constants; unsolved input keys remain unless canceled or pruned.

## Cost

For original support K, sum the costs of K successive add calls over accumulated support and each replacement support, including ordered maps and cloning. Output support can grow beyond K.

## behavior_complement

For sol(0)=var1 and sol(1)=leaf0, one subst(var0) excludes leaf0 and returns var1. A key exactly LEAF cannot be reported by unknowns.

## reverse_reconstruction

Output var1 admits unsubstituted var1 and var0 with sol0=var1. Given sol, input expressions whose one-pass weighted images combine to the output are compatible; kernel terms and pruning prevent unique recovery.

## depth_intervention

Applying subst twice to var0 with the chain0→1→LEAF yields LEAF, while one call yields1. Number of calls is part of the observation horizon.

## sorted_rounding_order

BTreeMap traversal fixes original-key order. Every replacement adds into the evolving map, so rounding/pruning can differ from an ideal simultaneous exact sum.

## unknown_leaf_cut

Keys0..LEAF-1 are unknowns, LEAF and above leaves regardless of coefficient or external leaf validity. unknowns is a coordinate classifier, not bounds validation.

## unmapped_identity

With no solution entry, var(k) is inserted at the original coefficient. This preserves unresolved variables subject to pruning; the result does not certify solvability.

## Tests

Actual helper fixture: `qread::wave34::sparse_projection_and_single_substitution`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
