# algorithm-palindromic-tree

## descriptant

### Distinct byte palindrome tree with overlapping counts and first witnesses.

The implementation catalog records classical attribution and algorithm-specific resource assumptions; test catalogs retain oracle limitations and fault-detection evidence.

## approximant

### Forward, invariant and cost

Maintain roots of lengths -1 and 0 and the longest suffix node. Follow suffix links until the current byte extends a palindrome. Reuse or create its transition, assigning the longest proper palindromic suffix. Count the current suffix once per position, then propagate counts in reverse discovery order. Export nonempty nodes with original byte ranges and suffix indexes.

Invariant: Preserve the recurrence/partition/tree relations and coordinate identity described in model.md#models.

Cost: O(n log 256) time with amortized suffix-link traversal; O(n) nodes, sparse edges and returned entries; MAX_LEN=1_000_000.

Limitations: source syntax is partial and macro bodies opaque. Mathematical descriptions and finite correspondence do not prove unrestricted Rust behavior. No timing or approximation-error advantage is claimed.

## models

# Typed relations

Subject/revision: source path and SHA-256 in manifest. Each section defines a separate observation and relation on immutable byte inputs. Status: partial source-derived specification with finite executed correspondence, not a semantic proof. Approximation direction is stated per relation; converse sets retain ambiguity. Assumptions: supported 32/64-bit Rust platforms, successful ordinary allocation and no external state. Evidence: four native tests per module, repeated debug/release runs and compiled controls in model.md#tests. Counterfactuals keep the named quantities fixed. No executable approximate surrogate or percentage-error claim is made.

## behavior_complement

Universe: byte slices and tree/Error observations with successful allocation. Exclude duplicate palindrome contents, non-palindromic witnesses, empty returned palindromes, out-of-bounds witnesses, count zero, and distinct_count>input length. These necessary constraints form an inner exclusion bound, not a complete Rust behavior classifier.

## reverse_reconstruction

Fix exported ranges, counts and links. Compatible byte inputs include every relabeling that preserves byte equality, so build(a)=build(b) for distinct singleton bytes. Without the original bytes, ranges do not reconstruct content. Keeping bytes and ranges recovers each witnessed palindrome but not a unique call history.

## suffix_link

Each nonempty node links to its longest proper nonempty palindromic suffix, or None for the empty suffix. Target nodes predate children. Substring enumeration checks every shorter suffix independently, including repeated bytes and odd/even lengths.

## occurrence_flow

At each input position only the longest suffix receives a direct count. Every shorter palindromic suffix lies on its suffix chain, so reverse discovery propagation includes overlapping occurrences once each. Each exported count is <=n; root counts also remain <=n. Enumeration counts every start/end pair independently.

## first_witness

A node is created when its palindrome first ends; fixed length makes this its earliest start. Nodes are returned in discovery order. Longest updates only on strictly greater length, retaining the earliest start on ties. first_witness_and_overlap checks equal-length candidates and repeated occurrences.

## root_extension

The length -1 root is always extendable because its reflected index equals the current position. Length 0 handles even extensions. The position-minus-length calculation uses isize after a one-million length check; supported 32/64-bit platforms represent it exactly. This prevents negative indexing and guarantees suffix-search termination.

## resource_partition

Length >MAX_LEN returns LengthLimit before any node allocation. Length <=MAX_LEN admits at most n+2 internal nodes and at most n distinct transitions; sparse BTreeMap avoids a 256-entry array per node. Vec and map allocation failure remains outside typed errors. length_boundary exercises equality and one beyond.

## content_projection

Output does not own input bytes; Range coordinates refer to the original immutable slice passed to build. Comparing entries without preserving byte identity loses palindrome content. empty_and_reconstruction_ambiguity exposes this projection with a and b; integration must join using the same byte sequence.

## tests

### Executed evidence

Syntax projection generated twice with byte equality. Finite tests, structural registration and syntax hashes do not establish a semantic AST, universal proof, or full application integration. Parent joins are owned separately.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and independently enumerated fixtures added with adjacent source-bound models. Maintain roots of lengths -1 and 0 and the longest suffix node. Follow suffix links until the current byte extends a palindrome. Reuse or create its transition, assigning the longest proper palindromic suffix. Count the current suffix once per position, then propagate counts in reverse discovery order. Export nonempty nodes with original byte ranges and suffix indexes. No optimization or speedup is claimed. The small oracle domains permit exact comparison; arbitrary long input correctness is not inferred from them. Output projection can lose input content; reverse models retain this ambiguity.

### 2026-10-30 · tests

2026-10-30: cargo test --offline -p mova-algorithms --lib passed 353 tests (12 new) twice in debug and twice in release. Sorted test identities/results were identical across all four runs: SHA-256 c3c1983072686ac513521c63abfe701413cba39ec62024d46f456e2a09319b61. Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed; narrow rustfmt checked the six owned Rust files.

Filter for this module: palindromic_tree::tests (four tests). Six compiled controls across the owned sources fail named assertions; this file's control log SHA-256 is f4ff4b09e28dcc285dfb974e27e7c0b1b7fdab45475827844acf61a11c0c2544. Exact bytes were restored; the final positive rerun passed all 353 tests and strict Clippy passed again. Raw evidence: $MOVA_DATA/runs/octopus-wave11-text-2026-10-30/.

Structural groupant reports repeated identically with SHA-256 c38c44782edf0a959f3e20d3736c47b2b46c5f09f52c3260f9d1c3966def1720; changed-source registration accepted all six owned sources. The external scaffold retains the unchanged base registry and adds these leaves under a temporary root without a materialized sidecar; integrated ancestors and joins remain the parent's responsibility.
