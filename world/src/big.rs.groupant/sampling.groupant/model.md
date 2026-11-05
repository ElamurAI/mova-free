# world-big-sampling

## descriptant

### world-big-sampling

E={i | allowed(place(i).book) AND high4(wrapping_mul(i,K))=bucket}; Y=E if |E|<=n else [E[floor(k*|E|/n)]]k<n.

## approximant

### Forward relation

E={i | allowed(place(i).book) AND high4(wrapping_mul(i,K))=bucket}; Y=E if |E|<=n else [E[floor(k*|E|/n)]]k<n.

Domain: Well-formed store; f64 integer conversion/huge lengths not proved.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

Store opens before books.tsv; accepted indices ascending; no sentence-length filter here.

Cost component: [cost](model.md#cost).

## cost

Store-wide scan and candidate allocation O(S log B+C), not O(n).

## composition

Store::place book u32 joins TSV first field; parser token-length gate follows selection.

## models

## behavior_complement

bucket16 cannot match high4. n0 iterates no output indices even though ratio infinity may be computed. Missing/invalid metadata id/group rows excluded; repeated book id last-wins.

## reverse_reconstruction

Y empty admits zero cap, no eligible indices or out-of-range bucket; nonempty Y constrains hash but not skipped text or candidate total.

## world_big_sampling_invariants

Store opens before books.tsv; accepted indices ascending; no sentence-length filter here.

## world_big_sampling_resource_cost

Store-wide scan and candidate allocation O(S log B+C), not O(n).

## world_big_sampling_representation_join

Store::place book u32 joins TSV first field; parser token-length gate follows selection.

## world_big_sampling_domain_partition

Well-formed store; f64 integer conversion/huge lengths not proved.

## tests

### Scoped evidence

Parent selected test names: ["big::wave29_worldlearn::sample_name_domain_and_groups"]. These finite tests touch only the named behaviors, not every statement in this fragment. See [parent evidence](../model.md#tests). control-big: compiled source mutation failed with exit101; source restored byte-for-byte and15positive tests passed.

Scope bytes [1609,2767) are bound to the shared parent syntax projection and full source hash. Only domain/group runtime test; sampling, cache and external judgment source-modeled, no executable surrogate.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. Only domain/group runtime test; sampling, cache and external judgment source-modeled, no executable surrogate.
