# world-induce-gpu

## descriptant

### world-induce-gpu

If n=groups.len()==0 or rel empty return zeros before CUDA. Else size=max(next_power_of_two(2n),1024), slot=high32((key XOR rel*K1)*K2 mod2^64)&mask. Host probes to store key,rel+1,group. One kernel thread/word enumerates1..3 subsets and probes same hash, atomicAdd cnt[group]. CUDA init/compile/load/launch/copy failure exits2.

## approximant

### Forward relation

If n=groups.len()==0 or rel empty return zeros before CUDA. Else size=max(next_power_of_two(2n),1024), slot=high32((key XOR rel*K1)*K2 mod2^64)&mask. Host probes to store key,rel+1,group. One kernel thread/word enumerates1..3 subsets and probes same hash, atomicAdd cnt[group]. CUDA init/compile/load/launch/copy failure exits2.

Domain: Static source only. Nonempty path requires GPU feature/CUDA, monotone offsets length labels+1 within code array, group ids<n, label<MAX, finite sizes/offsetsu32. No native compile/run.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

Rust/CUDA slot wraps same64bit operations. Kernel trusts off[w]..off[w+1], group ids, nonzero21bit codes, labels+1 nonzero. Exact-count intent needs counts fittingu32 and unique group keys; not verified here.

Cost component: [cost](model.md#cost).

## cost

Host table O(size+G*probe), transfers O(codes+offsets+labels+G). Kernel sum C(A,1..3)*probe length plus atomic contention; worst probe linearG. Host/device allocations and JIT costs unmeasured.

## composition

induce::break_counts constructs eligible words/dense group ids with offsets cast tou32. Default CPU build excludes this file; CPU counting tests neither compile nor execute CUDA module.

## models

## behavior_complement

Absent key cannot increment with valid sentinel table. Rel0 stored1, not empty sentinel0. Duplicate keys do not update every duplicate group: first match returns. No bounds validation for off/cnt indices or narrowing casts. Errors exit process rather than return Result.

## reverse_reconstruction

Count vector admits any multiset of words with same matching subset totals, reordered words and unused atom variations; allzero can mean empty labels or no matches. Counts do not recover table layout/word boundaries, and wraparound adds ambiguity beyondu32.

## world_induce_gpu_invariants

Rust/CUDA slot wraps same64bit operations. Kernel trusts off[w]..off[w+1], group ids, nonzero21bit codes, labels+1 nonzero. Exact-count intent needs counts fittingu32 and unique group keys; not verified here.

## world_induce_gpu_resource_cost

Host table O(size+G*probe), transfers O(codes+offsets+labels+G). Kernel sum C(A,1..3)*probe length plus atomic contention; worst probe linearG. Host/device allocations and JIT costs unmeasured.

## world_induce_gpu_representation_join

induce::break_counts constructs eligible words/dense group ids with offsets cast tou32. Default CPU build excludes this file; CPU counting tests neither compile nor execute CUDA module.

## world_induce_gpu_domain_partition

Static source only. Nonempty path requires GPU feature/CUDA, monotone offsets length labels+1 within code array, group ids<n, label<MAX, finite sizes/offsetsu32. No native compile/run.

## tests

### Finite actual-code evidence

Names: []

Selected CPU command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET cargo test --offline --locked -p world --lib --example retention_probe [--release] wave29_worldlearn -- --test-threads=1

15 selected cases passed twice debug and twice release with identical sorted names/outcomes, exits0. No native GPU compile/execution/control. No live teacher, corpus, state activation, GPU, or full-school run. Temporary synthetic files use unique process-id paths and are removed after passing tests.

GPU source/embedded CUDA modeled, no native compile, execution or compiled negative control. No surrogate/equivalence measurement.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. GPU source/embedded CUDA modeled, no native compile, execution or compiled negative control. No surrogate/equivalence measurement.

### 2026-10-31 · tests

Heading `### Finite actual-code evidence — 2026-10-31` stays in tests as `### Finite actual-code evidence`.

Source SHA256: 3c014cfba45cc70d9ce4b55049e1eff3b8f127f93f95676aab0c03185b2d0a9d. Syntax generated independently twice and compared; lexical only, no semantic proof or CUDA compilation. Strict whole-crate Clippy was not repeated merely to reproduce existing dependency/source lints; Cargo warnings retained in receipts.
