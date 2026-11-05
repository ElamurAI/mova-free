# world-induce-subset-counts

## descriptant

### world-induce-subset-counts

K(a,b,c)=a|(b<<21)|(c<<42), zero slots omitted. skeys emits C(A,1)+C(A,2)+C(A,3). add_fixes iff head=gold.head AND rel!=gold.rel; breaks iff both equal. Filter fix>=min, sort net desc, cardinality asc, Rule asc.

## approximant

### Forward relation

K(a,b,c)=a|(b<<21)|(c<<42), zero slots omitted. skeys emits C(A,1)+C(A,2)+C(A,3). add_fixes iff head=gold.head AND rel!=gold.rel; breaks iff both equal. Filter fix>=min, sort net desc, cardinality asc, Rule asc.

Domain: Aligned shapes; valid internal feature f<32,v<65535; count/index width bounds.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

Collision-free keys require nonzero21bit codes and<=3slots; input codes sorted unique. u32 decrement assumes balanced prior insertion; source does not check underflow balance.

Cost component: [cost](model.md#cost).

## cost

Cubic subset work, G candidate sort and up to16 worker local maps/count vectors.

## composition

CPU native test supplies features directly; GPU route re-encodes enum labels and offsets tou32, separate unexecuted claim.

## models

## behavior_complement

Four distinct atoms yield14 keys, excluding4-way subset. Wrong heads never counted. Same from/subset shares break count among all to targets; empty candidates return empty.

## reverse_reconstruction

Decode valid packed key injectively recovers included codes, not full word feature vector. Count fiber includes arbitrary sentence order and incorrect-head words; cannot recover individual repair examples.

## world_induce_subset_counts_invariants

Collision-free keys require nonzero21bit codes and<=3slots; input codes sorted unique. u32 decrement assumes balanced prior insertion; source does not check underflow balance.

## world_induce_subset_counts_resource_cost

Cubic subset work, G candidate sort and up to16 worker local maps/count vectors.

## world_induce_subset_counts_representation_join

CPU native test supplies features directly; GPU route re-encodes enum labels and offsets tou32, separate unexecuted claim.

## world_induce_subset_counts_domain_partition

Aligned shapes; valid internal feature f<32,v<65535; count/index width bounds.

## tests

### Scoped evidence

Parent selected test names: ["induce::wave29_worldlearn::counting_requires_correct_heads_and_orders_candidates", "induce::wave29_worldlearn::packed_subsets_and_empty_bootstrap", "induce::wave29_worldlearn::permissive_parse_and_set_valued_changes"]. These finite tests touch only the named behaviors, not every statement in this fragment. See [parent evidence](../model.md#tests). control-induce: compiled source mutation failed with exit101; source restored byte-for-byte and15positive tests passed.

Scope bytes [10798,19325) are bound to the shared parent syntax projection and full source hash. Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.
