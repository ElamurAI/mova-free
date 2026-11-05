# onto-hsmap-selection

## descriptant

### onto-hsmap-selection

score(c)=WSD(c)+1[preferred]+chapter_share(lf(c)); best replaces only >old+1e-9

## approximant

### Scoped forward

score(c)=WSD(c)+1[preferred]+chapter_share(lf(c)); best replaces only >old+1e-9

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Longest matching preference-fit phrase wins

Live before food in prefer/ckey

One sense→9.99 sentinel, not measured separation

Nonempty scored and valid Cid/lexfile required

No executable percentage surrogate established.

## invariant

### Local observations

Improvement<=1e-9 excludes replacement; negative state excludes bonus

Longest matching preference-fit phrase wins

Live before food in prefer/ckey

One sense→9.99 sentinel, not measured separation

Nonempty scored and valid Cid/lexfile required

## cost

### Scoped cost

O(S) score scan per candidate list plus phrase lookup/WSD work.

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

selection supplies the parent relation: score(c)=WSD(c)+1[preferred]+chapter_share(lf(c)); best replaces only >old+1e-9

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Improvement<=1e-9 excludes replacement; negative state excludes bonus

## reverse_reconstruction

Score vector preimage constrained by sequential threshold and reported margin; common shifts preserve winner

## phrase_order

Longest matching preference-fit phrase wins

## state_priority

Live before food in prefer/ckey

## singleton_margin

One sense→9.99 sentinel, not measured separation

## candidate_domain

Nonempty scored and valid Cid/lexfile required


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [3692,7964); shared parent AST. Excludes unrelated parent operations and added test module.
