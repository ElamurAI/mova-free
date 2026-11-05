# math-qread-constraints-single-unknown-passes

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Construct equations from Is constraints, thing comparisons, time comparisons and wholes in that order using state queries. Run four passes over stored equations. After subst(sol), exactly one unknown u with coefficient a produces sol[u]=-(remaining expression)/a; zero or multiple unknowns cause no insertion.

## Invariant

Returned map is an ordered partial solution with no residual consistency check. Single-unknown selection happens after pruning and substitution; returned expressions may retain number leaves. Four passes are fixed.

## Cost

Equation construction uses repeated state/event scans and map folds. Then exactly four equation visits per stored equation, each paying substitution and possibly clone/scale cost. No cubic Gaussian-elimination bound applies.

## behavior_complement

An equation with two remaining unknowns cannot directly insert either on that visit. A residual nonzero constant with zero unknowns is not an error; claiming contradiction detection is excluded by the code.

## reverse_reconstruction

Solutions u=leaf0 admit equations u-leaf0=0 and2u-2leaf0=0 on exact finite fixtures. Additional zero-unknown equations do not change insertion; neither original constraint count nor consistency is reconstructible.

## coefficient_isolation

For q=a*u+r with one surviving unknown, remove u then scale r by-1/a. Threshold pruning may have removed other unknowns first; this is not proof of exact algebra over original equations.

## underdetermined_gap

A sole equation u+v-leaf0 yields no map entry. Empty solution therefore admits both no constraints and unresolved multi-unknown constraints; it is not an unsatisfiability result.

## fixed_pass_horizon

Four passes propagate only chains exposed within that encounter ordering and horizon. Reversing equation order can delay discoveries; no convergence/completeness guarantee is asserted.

## constraint_assembly_order

Is equations precede cmps, time_cmps and wholes. State routing and exact-thing versus head selection determine which constraints exist before numerical passes begin.

## Tests

Actual helper fixture: `qread::wave34::state_fold_and_ordered_solver`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
