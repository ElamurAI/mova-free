# algorithm-quickselect-rust-approximant

## descriptant

### Executable quickselect approximant

Earlier exact observations remain in the parent. Expanded AST, formal proof and production evaluation remain absent.

## model

# Executable quickselect model

## Forward

Sample evenly spaced positions and select sample quantile. Count L values below and E equal to candidate over full input. Its rank interval is [L,L+E-1]. Compute distance d from target k; return if 100*d<=percent*n, else clone and exact select, reporting d=0. Products use u128 on <=64-bit usize targets. This provides rank error, not numerical value error. Immutable API does not promise production in-place partition.

## Invariant

Immutable slices of u64 with valid zero-based index; invalid index returns None. Metric: distance(index, returned value sorted-rank interval)/input_length. Caps are upper bounds. Production source is unchanged; explicit standalone harness only.

## behavior_complement

For valid input, exclude results absent from input, with reported distance differing from full rank counts, or 100*d>percent*n. Invalid k excludes Some. This necessary complement approximation also permits some candidates excluded by deterministic sample choice.

## reverse_reconstruction

For observed v,d,k,n,b, compatible multisets contain v at counts L,E>0 with distance(k,[L,L+E-1])=d and 100*d<=percent*n. Exact fallback requires k in interval and d=0. Compatible orderings additionally satisfy deterministic sample/fallback conditions; permutations can change branch. Conservative preimage, not unique recovery.

## Cost

Sample sizes 257,129,65,33 for caps 1%,2%,5%,10%; stack sample and O(n) certificate pass. On rejection clone and standard exact select, O(n) work and O(n) space. No universal speedup; tight budgets or adversaries may trigger fallback.

## evidence

Compile standalone with rustc --edition 2024 --test rust_approximant.rs --extern mova_approx=PATH_TO_RLIB, then execute. Runtime contract: tools/approx. Release timing belongs to the integrated harness; no speed factor claimed here.

The same tests passed on two executions. External compiled negative-control mutants were rejected: replacing the sqrt result with zero fails its differential bound test; unconditionally accepting sampled rank candidates fails both quickselect tests. Mutants were outside the repository and never changed production sources.

## History

### 2026-10-29 · descriptant

2026-10-29: real Rust companion with four budgets. The mathematical combiner connects source, domain, metric, runtime certificate and independent oracle. Source hashes alone establish no accuracy or speed claim.

2026-10-29, rustc 1.99.0: two tests pass. Four budgets, sizes 1..1024, sorted/reverse/random/equal/duplicates, endpoint/interior ranks, empty/out-of-range rejection. Independent sorted-vector rank oracle. Sample-position adversary triggers exact fallback in every profile; bad ranks rejected.

### 2026-10-29 · evidence

2026-10-29, rustc 1.99.0: two tests pass. Four budgets, sizes 1..1024, sorted/reverse/random/equal/duplicates, endpoint/interior ranks, empty/out-of-range rejection. Independent sorted-vector rank oracle. Sample-position adversary triggers exact fallback in every profile; bad ranks rejected.

### 2026-10-30 · evidence

#### Current cap epoch, 2026-10-30

Supported profiles are 1%, 2%, 5%, 10% with sample sizes 257, 129, 65, 33.
Prior 15%/25% profiles are retired. Three optimized standalone tests passed
twice using the updated shared contract. Each profile accepts a nonexact rank
on a sorted 4096-item fixture and rejects an adversarial sample through exact
fallback. Independent sorted-rank intervals check duplicates and measured
distance. A compiled mutant accepting every sample failed two tests.
Classification: normalized_rank / certified_sample / per_input_certificate.
The analytical implication comes from the full-input integer certificate;
sampling alone supplies no bound. Earlier timing describes its original epoch.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
