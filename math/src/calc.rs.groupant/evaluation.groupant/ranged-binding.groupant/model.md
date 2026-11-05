# math-calc-evaluation-ranged-binding

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Validate arity and variable, evaluate lo then hi, check span, save the current binding. Each inclusive iteration ticks, binds k, evaluates body, updates the selected accumulator or early-exits. Normal loop exit restores saved binding before result extraction and capped note. Any ? before restoration retains reached prefix state.

## Invariant

Early forall/exists/first break restores binding. A body failure after insert retains current k; tick failure before insert retains previous k. Empty sum/prod return0/1; empty first/extrema error after restoration.

## Cost

Sum of reached body and accumulator work over at most max(0,hi-lo+1) iterations, plus endpoint evaluation; list/filter retain O(iterations) values. Arbitrary precision body costs and deadlines are not bounded by tick count.

## behavior_complement

Normal successful first excludes leaving the witness bound to k. Body-error transitions exclude guaranteed rollback to the saved k. Empty ranges exclude body evaluation.

## reverse_reconstruction

A sum of0 admits an empty range or cancellation across nonempty iterations. An error with k=1 admits every prior k binding overwritten at that iteration. Returned first witness fixes only the earliest true predicate index within the admitted range, not predicate syntax.

## tick_insert_order

At each step tick runs before env.vars.insert. Resource failure on the next iteration and body failure in that iteration produce different visible bindings.

## early_witness

first/exists stop at the smallest visited true index; forall records the smallest false counterexample. Later body effects are absent. Restored binding and capped trace record follow the break.

## empty_identities

For lo>hi: sum0,prod1,count0,list/filter[],forall true,exists false; first/maxof/minof error. Endpoint evaluation effects still occur.

## restoration_cut

Restoration is after the loop but before first/extrema empty-result error. Errors from body/boolean conversion/arithmetic bypass it. No transaction is promised for vars,steps,approx or earlier notes.

## Tests

Actual helper fixture: `calc::wave34::syntax_zero_and_binding_boundaries`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
