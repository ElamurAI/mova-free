# math-qread-constraints-temporal-fold

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

At depth<3 and with a named owner, inspect the full event list for direct facts and applicable owner relations; reject moved-after-relation histories. Otherwise initialize an optional unknown from initial_ev and any told Is state, then fold events matching owner/thing/time, excluding Is/Rel and outside events for owner=None. Set and Add both add; Sub subtracts.

## Invariant

Only the final fold filters t<=upto. Relation/direct/initial-state decisions use the full history. Derived events are not excluded from this fold. Depth bounds relation recursion but not event count.

## Cost

O(E) full scans per relation level, at most depth3 recursive relation exploration with bounded branching, plus affine additions for selected events. Cost includes support growth and predicate evaluation, not just event count.

## behavior_complement

For all-owner fold, outside entries cannot contribute. A Sub cannot add its quantity. Set does not overwrite prior totals here. Excluding all future influence would be false because branch decisions inspect all history.

## reverse_reconstruction

A folded expression may arise from Set(a), Add(a), or multiple canceled events. A visible initial unknown at early upto can arise because a later Is exists; output alone cannot reconstruct a time-prefix history.

## set_add_equivalence

Within the accumulation branch, Set and Add share st.unwrap_or_default().add(val,1). They differ upstream in initial/relation decisions; local equality is not global interchangeability.

## outside_owner_projection

outside suppresses events only for owner=None. Querying the named outside owner may include them, so all-owner sum need not equal sum over every named owner.

## future_condition_influence

A first Add with class not make gains initial unknown1000+k when any matching Is exists, even after upto. Fixture constructs this finite counterexample to prefix causality.

## subtraction_cancellation

Set(a),Sub(b),Set(b) yields a through coefficient cancellation; at cutoff before the final Set it yields a-b. Direct fixture exercises both boundaries.

## Tests

Actual helper fixture: `qread::wave34::state_fold_and_ordered_solver`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
