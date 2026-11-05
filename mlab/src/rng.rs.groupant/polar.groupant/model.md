# mlab-rng-polar

## descriptant

### mlab-rng-polar

Normal takes spare first; otherwise samples u,v in [-1,1), rejects s<=0 or s>=1, emits u*sqrt(-2ln(s)/s), caches v times same scale.

## approximant

### polar: forward relation

Normal takes spare first; otherwise samples u,v in [-1,1), rejects s<=0 or s>=1, emits u*sqrt(-2ln(s)/s), caches v times same scale.

Domain: All u64 seeds; IEEE f64 libm operations. Cross-platform transcendental bit identity is not proved.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### polar: invariant and cost

s=0 and s=1 rejected; finite accepted s gives finite multiplier for this discrete generator.

Two consecutive accepted normals share one polar radius; cloning preserves spare and future stream.

Scope-specific cost: O(1) cached branch; each rejected pair costs two uniform draws. Expected O(1), unbounded worst-case loop.

## models

## behavior_complement

Cached branch excludes any state advance; rejected points exclude emission.

## reverse_reconstruction

Observed cached z admits any unchanged four-word state paired with spare=Some(z); fresh z admits many accepted (u,v) and prior rejection histories.

## rejection_domain

s=0 and s=1 rejected; finite accepted s gives finite multiplier for this discrete generator.

## cache_transport

Two consecutive accepted normals share one polar radius; cloning preserves spare and future stream.

## branch_relation

Normal takes spare first; otherwise samples u,v in [-1,1), rejects s<=0 or s>=1, emits u*sqrt(-2ln(s)/s), caches v times same scale.

## resource_cost

O(1) cached branch; each rejected pair costs two uniform draws. Expected O(1), unbounded worst-case loop.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Cached branch excludes any state advance; rejected points exclude emission. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [1433,1911); includes no claim beyond this relation. Source hash dccb613f518f32218a910c4eca3ad31df44347dd38b90d431833558e995a2046.
