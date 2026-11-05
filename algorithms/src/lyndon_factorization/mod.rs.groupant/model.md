# algorithm-lyndon-factorization

## descriptant

### Duval factorization into nonincreasing byte Lyndon factors.

The implementation catalog records classical attribution and algorithm-specific resource assumptions; test catalogs retain oracle limitations and fault-detection evidence.

## approximant

### Forward, invariant and cost

At each unprocessed start compare scan and matched positions. Strict increase resets matched to start; equality advances it. On descent or EOF, period=scan-matched and emit period-length factors while start<=matched. Returned factors cover the input in order and are lexicographically nonincreasing Lyndon words.

Invariant: Preserve the recurrence/partition/tree relations and coordinate identity described in model.md#models.

Cost: O(n) byte comparisons and O(k) returned ranges, O(1) auxiliary words; no policy cap; Vec allocation semantics.

Limitations: source syntax is partial and macro bodies opaque. Mathematical descriptions and finite correspondence do not prove unrestricted Rust behavior. No timing or approximation-error advantage is claimed.

## models

# Typed relations

Subject/revision: source path and SHA-256 in manifest. Each section defines a separate observation and relation on immutable byte inputs. Status: partial source-derived specification with finite executed correspondence, not a semantic proof. Approximation direction is stated per relation; converse sets retain ambiguity. Assumptions: supported 32/64-bit Rust platforms, successful ordinary allocation and no external state. Evidence: four native tests per module, repeated debug/release runs and compiled controls in model.md#tests. Counterfactuals keep the named quantities fixed. No executable approximate surrogate or percentage-error claim is made.

## behavior_complement

Universe: finite byte slices and range-vector observations under successful allocation. Exclude gaps, overlaps, empty factors, endpoints beyond n, factors not strictly below every nontrivial rotation, and increasing adjacent factor contents. Empty input excludes any nonempty result. This is an exact-target factorization constraint; universal Rust correspondence remains unproved.

## reverse_reconstruction

Fix returned factor ranges. All byte assignments yielding those factors are compatible: ab and xy share [0..2]. Range outputs alone lose contents. If bytes are retained, concatenating referenced factors reconstructs those bytes exactly, but does not invert ranges in isolation.

## rotation_primitivity

A nonempty word is Lyndon exactly when strictly smaller than all nontrivial rotations. Equality with a rotation rejects periodic words. The independent oracle explicitly forms rotations; its false <= mutant admits aa as a factor and fails partition uniqueness.

## nonincreasing_partition

Fix an input and enumerate all 2^(n-1) cut patterns (with one empty partition at n=0). Retain partitions whose factors are Lyndon and whose neighbors are nonincreasing. exhaustive_partitions finds exactly one for each of 511 binary words of length <=8 and compares complete ranges.

## period_emission

At the scan stop, period=scan-matched>0. Repeated emissions consume positive equal-length segments while start<=matched; start+period never exceeds scan<=n. Periodic ababab emits three copies of ab, not a single periodic factor.

## unsigned_order

Comparison is u8 order, including 0 and 255. No locale, scalar decoding or normalization intervenes. periodic_and_unsigned_bytes checks the high-byte boundary; changing alphabet by a strictly increasing byte map preserves factor ranges where such a map exists.

## coverage_reconstruction

Ranges are consecutive half-open intervals starting at zero and ending at n. Flattening the slices reproduces the exact original bytes, including duplicates. coverage_and_order joins factorization to direct reconstruction for banana, mississippi and a periodic sample.

## resource_progress

Each outer iteration advances start by at least one; matched remains below scan, and scan<=n. Standard Duval amortization charges comparisons to emitted positions, giving linear work although some positions are revisited. Output length is <=n; storage allocation is fallible through Vec behavior, with no added typed resource policy.

## tests

### Executed evidence

Syntax projection generated twice with byte equality. Finite tests, structural registration and syntax hashes do not establish a semantic AST, universal proof, or full application integration. Parent joins are owned separately.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and independently enumerated fixtures added with adjacent source-bound models. At each unprocessed start compare scan and matched positions. Strict increase resets matched to start; equality advances it. On descent or EOF, period=scan-matched and emit period-length factors while start<=matched. Returned factors cover the input in order and are lexicographically nonincreasing Lyndon words. No optimization or speedup is claimed. The small oracle domains permit exact comparison; arbitrary long input correctness is not inferred from them. Output projection can lose input content; reverse models retain this ambiguity.

### 2026-10-30 · tests

2026-10-30: cargo test --offline -p mova-algorithms --lib passed 353 tests (12 new) twice in debug and twice in release. Sorted test identities/results were identical across all four runs: SHA-256 c3c1983072686ac513521c63abfe701413cba39ec62024d46f456e2a09319b61. Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed; narrow rustfmt checked the six owned Rust files.

Filter for this module: lyndon_factorization::tests (four tests). Six compiled controls across the owned sources fail named assertions; this file's control log SHA-256 is 8a76fec8034a442b67da42d4d373666c850f0cc04e0693d189e7b582456da519. Exact bytes were restored; the final positive rerun passed all 353 tests and strict Clippy passed again. Raw evidence: $MOVA_DATA/runs/octopus-wave11-text-2026-10-30/.

Structural groupant reports repeated identically with SHA-256 c38c44782edf0a959f3e20d3736c47b2b46c5f09f52c3260f9d1c3966def1720; changed-source registration accepted all six owned sources. The external scaffold retains the unchanged base registry and adds these leaves under a temporary root without a materialized sidecar; integrated ancestors and joins remain the parent's responsibility.
