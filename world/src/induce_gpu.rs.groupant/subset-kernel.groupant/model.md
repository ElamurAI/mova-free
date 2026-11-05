# world-induce-gpu-subset-kernel

## descriptant

### world-induce-gpu-subset-kernel

w=blockIdx*blockDim+threadIdx; w>=nwords returns. Iterate i<j<k in off word range; probe(ci),probe(ci|cj<<21),probe(ci|cj<<21|ck<<42). Match key+rel+1 then atomicAdd1 and return.

## approximant

### Forward relation

w=blockIdx*blockDim+threadIdx; w>=nwords returns. Iterate i<j<k in off word range; probe(ci),probe(ci|cj<<21),probe(ci|cj<<21|ck<<42). Match key+rel+1 then atomicAdd1 and return.

Domain: Well-formed host arrays, reachable sentinel; static-only evidence.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

No off/cnt bounds checks;21bit atom slots differ from flat offsets. Counts exact only below wrap threshold and trusted valid host table.

Cost component: [cost](model.md#cost).

## cost

256threads/block, cubic subsets per word, linear worst probe and serialized contested atomics.

## composition

Host must match hash constants/mask/sentinel; this scoped CUDA string is not executed by lexical Rust AST projection.

## models

## behavior_complement

Out-of-range thread writes nothing. Missing lookup terminates at sentinel; malformed full table may not terminate. Duplicate equal slots route only first match.

## reverse_reconstruction

Observed c[g] admits all input words producing same c[g] queried subsets modulo2^32, no unique recovery of feature grouping or unused values.

## world_induce_gpu_subset_kernel_invariants

No off/cnt bounds checks;21bit atom slots differ from flat offsets. Counts exact only below wrap threshold and trusted valid host table.

## world_induce_gpu_subset_kernel_resource_cost

256threads/block, cubic subsets per word, linear worst probe and serialized contested atomics.

## world_induce_gpu_subset_kernel_representation_join

Host must match hash constants/mask/sentinel; this scoped CUDA string is not executed by lexical Rust AST projection.

## world_induce_gpu_subset_kernel_domain_partition

Well-formed host arrays, reachable sentinel; static-only evidence.

## tests

### Scoped evidence

Parent selected test names: []. These finite tests touch only the named behaviors, not every statement in this fragment. See [parent evidence](../model.md#tests). No native GPU compile/execution/control.

Scope bytes [545,2160) are bound to the shared parent syntax projection and full source hash. GPU source/embedded CUDA modeled, no native compile, execution or compiled negative control. No surrogate/equivalence measurement.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. GPU source/embedded CUDA modeled, no native compile, execution or compiled negative control. No surrogate/equivalence measurement.
