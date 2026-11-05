# mlab-rng-stream

## descriptant

### mlab-rng-stream

Seed expansion applies four wrapping splitmix rounds; next_u64 returns rotl(s1*5,7)*9 then XOR/shift/rotl45 transition. Uniform=(result>>11)/2^53.

## approximant

### stream: forward relation

Seed expansion applies four wrapping splitmix rounds; next_u64 returns rotl(s1*5,7)*9 then XOR/shift/rotl45 transition. Uniform=(result>>11)/2^53.

Domain: All u64 seeds; IEEE f64 libm operations. Cross-platform transcendental bit identity is not proved.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### stream: invariant and cost

Representable uniforms lie on j/2^53, 0<=j<2^53.

reseed replaces both state and spare; uniform draws do not consume spare.

Scope-specific cost: O(1) wrapping integer operations, four words; uniform consumes exactly one next_u64.

## models

## behavior_complement

Uniform excludes 1 and negative values; wrapping state arithmetic excludes debug overflow panics in the multiplications/addition explicitly marked wrapping.

## reverse_reconstruction

A uniform value admits all low11-bit result antecedents plus compatible state words. Output alone does not recover the seed.

## uniform_grid

Representable uniforms lie on j/2^53, 0<=j<2^53.

## seed_reset

reseed replaces both state and spare; uniform draws do not consume spare.

## branch_relation

Seed expansion applies four wrapping splitmix rounds; next_u64 returns rotl(s1*5,7)*9 then XOR/shift/rotl45 transition. Uniform=(result>>11)/2^53.

## resource_cost

O(1) wrapping integer operations, four words; uniform consumes exactly one next_u64.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Uniform excludes 1 and negative values; wrapping state arithmetic excludes debug overflow panics in the multiplications/addition explicitly marked wrapping. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [386,1433); includes no claim beyond this relation. Source hash dccb613f518f32218a910c4eca3ad31df44347dd38b90d431833558e995a2046.
