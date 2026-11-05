# algorithm-damerau-levenshtein

## descriptant

### Exact unrestricted unit-cost byte edit distance.

The implementation catalog records classical attribution and algorithm-specific resource assumptions; test catalogs retain oracle limitations and fault-detection evidence.

## approximant

### Forward, invariant and cost

Sentinel matrix H uses the last source row containing each target byte and the most recent matching target column. H[i+1,j+1] minimizes insertion, deletion, substitution and H[p,q]+(i-p-1)+1+(j-q-1). The transposition can be followed by further edits, unlike OSA. Rows and columns represent prefix distances with one sentinel offset.

Invariant: Preserve the recurrence/partition/tree relations and coordinate identity described in model.md#models.

Cost: O((n+2)(m+2)) time and words, bounded by MAX_CELLS=1_000_000; empty calls O(1).

Limitations: source syntax is partial and macro bodies opaque. Mathematical descriptions and finite correspondence do not prove unrestricted Rust behavior. No timing or approximation-error advantage is claimed.

## models

# Typed relations

Subject/revision: source path and SHA-256 in manifest. Each section defines a separate observation and relation on immutable byte inputs. Status: partial source-derived specification with finite executed correspondence, not a semantic proof. Approximation direction is stated per relation; converse sets retain ambiguity. Assumptions: supported 32/64-bit Rust platforms, successful ordinary allocation and no external state. Evidence: four native tests per module, repeated debug/release runs and compiled controls in model.md#tests. Counterfactuals keep the named quantities fixed. No executable approximate surrogate or percentage-error claim is made.

## behavior_complement

Universe: byte-slice pairs and Result observations under successful allocation. An inner exclusion model rules out Ok costs outside [abs(n-m),max(n,m)], nonzero identical-input costs, asymmetric pair costs, and Ok on a nonempty matrix exceeding the cell cap. These necessary constraints do not characterize all forbidden costs.

## reverse_reconstruction

Fix target b and successful distance k. Preserve every source a at edit-graph shortest-path distance k from b, subject to resource acceptance. Zero fixes a=b, but positive distances admit many sources. CellLimit has many length-pair antecedents and does not identify bytes. Finite BFS checks fiber membership, not an unbounded inverse solver.

## last_occurrence

Before cell (i,j), last[c] is the greatest source index below i with byte c, and matched_column is the greatest target index below j matching a[i-1]. Snapshot both before updating matched_column. The row table changes only after the row, preventing same-row reuse.

## unrestricted_transposition

Hold CA and ABC fixed: transpose CA to AC, then insert B, giving cost two, while OSA returns three. The last-occurrence gap costs delete and insert intervening characters; it is not the two-prefix OSA recurrence. unrestricted_witness rejects an increased transposition cost.

## metric_triangle

For three accepted length-bounded byte strings, concatenating scripts supplies d(a,c)<=d(a,b)+d(b,c). The recurrence is intended to recover the unrestricted script optimum. triangle_and_zero_fibers checks all 343 binary triples of length <=2; no exhaustive all-byte proof is asserted.

## alphabet_projection

Fix tiny binary endpoints. Mapping any temporary foreign symbol to a fixed endpoint alphabet symbol maps each edit to one valid edit or a null edit, so no foreign symbol improves the optimum. This permits the independent finite BFS alphabet restriction. Arbitrary Unicode semantics are outside the byte domain.

## budget_preimage

Empty inputs bypass all matrix arithmetic. Otherwise checked row/column additions, cell multiplication, and length sum precede CellLimit. Accepted nonempty dimensions make all candidate sums bounded by a small multiple of the one-million cell cap, hence machine-word safe on supported 32/64-bit platforms. Allocation failure is not converted to Error.

## bounded_graph

Binary endpoints have length <=3, so substitution plus insertion/deletion gives upper bound three. A path of <=3 edits from a has length <=len(a)+3<=6. BFS visits all paths to depth three and therefore computes complete minima for all 225 endpoint pairs. The CA/ABC witness uses alphabet ABC and depth three. No broader enumeration is claimed.

## tests

### Executed evidence

Syntax projection generated twice with byte equality. Finite tests, structural registration and syntax hashes do not establish a semantic AST, universal proof, or full application integration. Parent joins are owned separately.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and independently enumerated fixtures added with adjacent source-bound models. Sentinel matrix H uses the last source row containing each target byte and the most recent matching target column. H[i+1,j+1] minimizes insertion, deletion, substitution and H[p,q]+(i-p-1)+1+(j-q-1). The transposition can be followed by further edits, unlike OSA. Rows and columns represent prefix distances with one sentinel offset. No optimization or speedup is claimed. The small oracle domains permit exact comparison; arbitrary long input correctness is not inferred from them. Output projection can lose input content; reverse models retain this ambiguity.

### 2026-10-30 · tests

2026-10-30: cargo test --offline -p mova-algorithms --lib passed 353 tests (12 new) twice in debug and twice in release. Sorted test identities/results were identical across all four runs: SHA-256 c3c1983072686ac513521c63abfe701413cba39ec62024d46f456e2a09319b61. Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed; narrow rustfmt checked the six owned Rust files.

Filter for this module: damerau_levenshtein::tests (four tests). Six compiled controls across the owned sources fail named assertions; this file's control log SHA-256 is 685703fdd0eb4799106108a5b98cee00f1715d56808d323b9eb9f4306565e175. Exact bytes were restored; the final positive rerun passed all 353 tests and strict Clippy passed again. Raw evidence: $MOVA_DATA/runs/octopus-wave11-text-2026-10-30/.

Structural groupant reports repeated identically with SHA-256 c38c44782edf0a959f3e20d3736c47b2b46c5f09f52c3260f9d1c3966def1720; changed-source registration accepted all six owned sources. The external scaffold retains the unchanged base registry and adds these leaves under a temporary root without a materialized sidecar; integrated ancestors and joins remain the parent's responsibility.
