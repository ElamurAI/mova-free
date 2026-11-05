# algorithm-weighted-interval-scheduling

## descriptant

### weighted_interval_scheduling mod.rs

Original standard-library implementation of a classical dynamic programming algorithm, not a novelty claim. Source is authoritative; sidecar models and syntax are partial. Models compose recurrence, bounds and public result; tests observe this composition rather than establish a universal proof. No approximate execution profile is supplied because this API promises exact results; finite evidence is not an analytical error guarantee.

## model

# weighted_interval_scheduling mod.rs model

## Forward

Reject first original index with start>=end. Sort by (end,start,index). For item j let p be count of earlier ends<=start. B(j+1)=max(B(j),B(p)+weight). Take only strict improvements, then reconstruct original indices in chronological order.

## Invariant

B(j) is optimum over first j sorted items. Every predecessor p<=j and every reconstruction transition strictly decreases prefix. Chosen intervals are unique and nonoverlapping; sum of their weights equals B(n). Input untouched.

## Cost

O(n log n) comparisons and O(n) words; validation errors take O(first-invalid-index+1) time and O(1) space. No timing measured.

## nonbehavior

Type: behavior_complement. Exclude invalid-interval success, nonfirst invalid-index errors, duplicate/out-of-range output indices, overlapping chosen intervals, wrong sum, suboptimal score or tie behavior inconsistent with strict improvements. Allocation aborts are outside returned Result.

## preimages

Type: reverse_reconstruction. A successful (indices,total) admits all valid item arrays whose deterministic recurrence chooses those indices with that sum. Endpoints can shift while preserving compatibility. Weights of excluded items may vary while remaining dominated. Neither input nor unique optimum follows from output.

## touching

Type: compatibility_relation. Half-open positive-duration intervals are compatible exactly when one end<=the other's start. Positive duration ensures sorted predecessor candidates precede the current item; replacing <= by < loses adjacency.

## tie

Type: choice_relation. At equal candidate and previous score, skip current sorted item. Duplicate intervals select earliest original index if positive. Zero weights never create a strict gain. This witness rule does not promise lexicographically minimal original-index vectors.

## affine

Type: metamorphic_relation. Uniform endpoint translation preserving i64 bounds preserves order, compatibility and exact Schedule. Multiplying all weights by positive integer preserving u64/u128 bounds preserves chosen indices and scales score; zero multiplier does not preserve witness.

## permutation

Type: metamorphic_relation. Input permutation preserves optimum weight, but indices transform and equal-key ties may pick a different original item. Thus only score invariance is unconditional.

## error-fiber

Type: failure_partition. Err InvalidInterval{i} admits arrays with all earlier start<end and item i start>=end; later items unconstrained. After validation, DimensionOverflow precedes allocations; checked WeightOverflow occurs during DP. On usual 32/64-bit targets the latter two are unreachable for valid allocated non-ZST Item slices; allocator exhaustion is not caught.

## trace

Type: reverse_reconstruction. Given sorted order and all take/predecessor arrays, reconstruct by decreasing prefixes; chosen transitions jump to predecessor, skips decrement. Final score alone cannot recover these arrays: one weight-seven interval and adjacent weight-three/four intervals share score seven.

## evidence

### Evidence

LCS independent oracle enumerates subsequences of each first argument and scans the second, checking all 63 squared binary-word pairs through length five (3,969 pairs). Scheduling independently enumerates subsets and pairwise compatibility for all 5,461 arrays through length six over four item choices. Metamorphic tests include explicit input-fiber ambiguity, boundary and invalid-input cases. Allocator exhaustion and unreachable allocated-slice dimension/weight overflow cases were not executed.

Strict Clippy all-targets with -D warnings passed. rustfmt --check passed for all four owned Rust files. Installed mova-modelcheck syntax ran twice per source and produced byte-identical projections. Syntax is partial: macros and semantic typing are not expanded.

## prospective

### Prospective

Search of algorithms/src found neither named implementation. Original implementations of classical dynamic programming, no algorithmic novelty claimed. LCS saves the overwritten diagonal and uses one shorter row: O(nm) equality calls, O(min(n,m)) memory. Scheduling sorts by (end,start,index), binary-searches compatible prefixes and reconstructs strict improvements: O(n log n) time, O(n) memory. Adjacent half-open intervals are compatible; invalid start>=end rejected first; ties skip the current sorted item. Inputs unchanged. Validate independent exponential oracles, boundary and metamorphic relations, repeated debug/release outcomes and compiled production mutants. Allocation exhaustion and pathological Eq are outside checked behavior; no timing claims.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective cost and invariants retained in implementation model.md#prospective. The reverse models retain ambiguous input fibers and separate behavior exclusions from reconstruction. No timing optimization claimed.

### 2026-10-30 · evidence

Heading `### Evidence epoch 2026-10-30` stays in evidence as `### Evidence`.

Final source validation: two debug and two release executions of the complete mova-algorithms package passed 176 unit tests and one doctest each. Nine new tests belong to these two algorithms (four LCS, five scheduling). Sorted named test outcomes were identical across all four runs; elapsed time and build chatter were excluded. Normalized outcome SHA-256: c38c9be833de92508d6cd6304a57c36c196c4c4ef57561fa9c7f361dd7816255.

Compiled production negative control: replace end <= start by end < start; real tests fail. Mutant log SHA-256 08941153398c7a62ca10a1734b7fe22517d3bcb3f0b900dc6482e8e19d33b966. Original source restored before final repeated runs.

Artifacts live under $MOVA_DATA/runs/octopus-wave4-dynamic-2026-10-30. A temporary local lib declaration was used for package integration tests and restored. Registry-entry copies are supplied for main-instance integration; shared registry hierarchy checks remain the integrator's responsibility. These observations establish bounded correctness evidence and sensitivity to the named fault, not a proof or benchmark gain.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.
