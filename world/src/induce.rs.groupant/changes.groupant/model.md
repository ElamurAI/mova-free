# world-induce-changes

## descriptant

### world-induce-changes

Add r appends iff r absent; Remove r filters all equal; Replace a,b maps every a in place. holds(Add r)=exists broader x covering r; holds(Replace)=old absent OR new present.

## approximant

### Forward relation

Add r appends iff r absent; Remove r filters all equal; Replace a,b maps every a in place. holds(Add r)=exists broader x covering r; holds(Replace)=old absent OR new present.

Domain: Finite Rule vectors including duplicate/empty atoms; semantic validity caller-owned.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

Caller vector immutable, order retained, multiplicity preserved by Replace; equality compares full atom vectors.

Cost component: [cost](model.md#cost).

## cost

O(R*A) equality/clones with no I/O/parser effects.

## composition

family apply_scoped adds scope preservation; broader holds is redundancy heuristic not inverse of apply.

## models

## behavior_complement

holds(Add r) true need not imply apply(Add r)=identity: broader unequal rule covers but exact append still occurs. Replace cannot reorder untouched rules.

## reverse_reconstruction

Remove fiber Y consists of lists deleting all r to Y. Replace chooses subset of resulting new positions with old antecedents, preserving others. Add output may come from identical already-containing list or prefix before final appended r.

## world_induce_changes_invariants

Caller vector immutable, order retained, multiplicity preserved by Replace; equality compares full atom vectors.

## world_induce_changes_resource_cost

O(R*A) equality/clones with no I/O/parser effects.

## world_induce_changes_representation_join

family apply_scoped adds scope preservation; broader holds is redundancy heuristic not inverse of apply.

## world_induce_changes_domain_partition

Finite Rule vectors including duplicate/empty atoms; semantic validity caller-owned.

## tests

### Scoped evidence

Parent selected test names: ["induce::wave29_worldlearn::counting_requires_correct_heads_and_orders_candidates", "induce::wave29_worldlearn::packed_subsets_and_empty_bootstrap", "induce::wave29_worldlearn::permissive_parse_and_set_valued_changes"]. These finite tests touch only the named behaviors, not every statement in this fragment. See [parent evidence](../model.md#tests). control-induce: compiled source mutation failed with exit101; source restored byte-for-byte and15positive tests passed.

Scope bytes [58143,59518) are bound to the shared parent syntax projection and full source hash. Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.
