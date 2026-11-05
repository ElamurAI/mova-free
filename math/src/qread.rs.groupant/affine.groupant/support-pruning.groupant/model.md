# math-qread-affine-support-pruning

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Clone left Lin; update constant by s*o.c and each right coefficient by ordered entry addition; retain only coefficients whose absolute value is strictly greater than1e-12. scale uses zero-left add. Floating operations are in source order, not ideal exact algebra.

## Invariant

On finite inputs, every retained coefficient has magnitude>1e-12; constant is never threshold-pruned. Output keys are a subset of union of input keys.

## Cost

Clone O(A), B ordered-map updates O(B log(A+B)), then O(A+B) retention scan; space O(A+B). Scalar scale still performs map construction and retention.

## behavior_complement

A resulting finite coefficient exactly1e-12 cannot remain. A small constant may remain, so excluding it would contradict the implementation. No exact associativity claim follows from floating arithmetic and intermediate pruning.

## reverse_reconstruction

An absent key admits exact cancellation and every residual within [-1e-12,1e-12]. Given nonpruned output and fixed right argument, compatible left coefficients satisfy the rounded addition and threshold relation; this is not globally unique.

## threshold_endpoint

Retention is strict >; scale(var0,1e-12) drops key0 while2e-12 retains it. This is absolute support projection, not a relative-error certificate.

## constant_map_asymmetry

A constant1e-13 survives scale1 even though the same variable coefficient is removed. Numeric state cannot be reconstructed from support alone.

## cancellation_quotient

add(a,a,-1) removes matched keys, losing their provenance. Distinct canceled unknown/leaf histories share the empty support; inverse must retain this ambiguity.

## nonfinite_limit

NaN coefficient fails the > predicate and disappears; infinite coefficient survives. Finite-domain invariant does not establish universal numerical sanitation.

## Tests

Actual helper fixture: `qread::wave34::sparse_projection_and_single_substitution`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
