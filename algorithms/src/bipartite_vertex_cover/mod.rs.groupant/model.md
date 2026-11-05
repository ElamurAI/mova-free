# algorithm-bipartite-vertex-cover

## descriptant

### Description and epoch memory

Left adjacency rows G and right population R, combined vertices at most 1000000 and total stored edges at most 1000000; duplicate neighbors allowed.

Independent finite oracles inspect both primal/dual cover cardinalities and every stable permutation at small sizes. Original source and fixtures; citation describes the classical method. Models and partial syntax do not prove semantics; macros remain opaque and semantic AST is absent.

## approximant

### Forward and cost

Delegate resource/endpoint validation and matching to Hopcroft-Karp. Let Z start at unmatched left vertices and close under unmatched L-to-R edges and matched R-to-L edges. Return sorted (L minus ZL, ZR). König equality makes the cover cardinality equal to maximum matching cardinality.

Matching O((L+R+E)*sqrt(L+R+1)) time plus O(L+R+E) reachability. O(L+R) auxiliary storage. Endpoint validation precedes indexed traversal; allocation failure is untyped.

Attribution: D. König, Gráfok és mátrixok, Matematikai és Fizikai Lapok 38 (1931), 116–119; existing crate Hopcroft-Karp supplies matching. Implementation and fixtures are original. No optimization or speedup is claimed.

## invariant

### Invariants

Every matching pair is a reciprocal edge. Each reached right has its matched left reached; maximality excludes an unmatched reached right. For any edge (l,r), l outside ZL or r inside ZR; thus every edge is covered. Exactly one endpoint of each matched pair is selected and no unmatched vertex is selected.

Test enumeration uses independent subset/assignment predicates rather than repeating production traversal. Bounds do not prove unbounded behavior.

## catalog

# Typed models

Each relation is source-derived, exact as a specification; finite executable evidence is partial. Assumes deterministic std-only execution with successful allocation. No executable inverse solver or approximation error cap is claimed.

## behavior-complement

Universe: valid graph/count inputs crossed with candidate sorted vertex subsets, together with typed validation errors. Absent behaviors include any uncovered edge, size unequal to maximum matching size, an out-of-range/duplicate/unsorted member, or selecting ZL on the left instead of its complement. With fixed deterministic matching and closure Z, every pair except (L minus ZL,ZR) is absent. Invalid inputs cannot return a cover; resource errors precede endpoint errors. This is an exact source-derived specification with partial finite correspondence, not a proof.

## reverse-reconstruction

For fixed cover (A,B), compatible antecedents are all bounded valid graphs whose deterministic alternating closure has ZL=L minus A and ZR=B; equivalently retain the matching/closure existential intermediate constrained by the actual deterministic Hopcroft-Karp traversal. Merely requiring A,B to cover edges is insufficient. Edge duplicates and extra isolated right vertices can have the same cover, so recovery is ambiguous. Empty graphs with zero or one right vertex both yield empty cover. Error fibers retain all graphs with the same first failing validation field.

## alternating-closure

For fixed matching, start from free left vertices, iterate only unmatched forward edges and matched backward edges. Monotone closure reaches each vertex once. The 3-by-3 exhaustive oracle and duplicate/unbalanced witness check consequences.

## dual-certificate

For any feasible matching M and cover C, |M|<=|C|. Output attains equality. Independent recursive matching enumeration and subset cover minimization check both bounds, rather than trusting delegated matching.

## multiplicity-fiber

Holding vertex labels fixed, repeat any existing neighbor in place: matching and alternating reachability produce the same cover, while input multiplicities differ. Edge count remains below budget; resource rejection can break this equivalence.

## tests

### Bounded checks

All 689 simple graphs for partition sizes 0 through 3: independent cover subset enumeration and recursive matching enumeration; exact duplicate/alternating witness; accepted vertex and edge cap plus rejected overflow/endpoints. Three named tests.

Debug and release run twice with normalized named outcomes. Compiled production mutants flip the cover left complement and receiver replacement comparison respectively; named oracle assertions reject them. Restored positives and strict algorithms clippy are required. External receipts bind final source/evidence hashes. No timing or full-system claim.

See [executed receipts](model.md#evidence) for final source hashes, named control failures and repeated test log hashes. All checks stated above completed.

## evidence

### Executed evidence

Compiled left-selection complement flip was rejected by `exhaustive_cover_and_matching_oracles`: nonminimum cover, size 1 instead of 0.

Raw logs remain in the external wave14 graphs run bundle. Earlier tooling attempts exposed two Clippy style findings, absent rg, and stale syntax outputs; final receipts above bind corrected sources and successful checks. No source correctness repair was required by positive oracle findings. No general proof, speedup, full-system or allocation-failure claim.

## History

### 2026-10-30 · descriptant

2026-10-30 initial epoch: chose existing matching plus explicit alternating closure for cover, and FIFO deferred acceptance plus inverse receiver ranks for marriage. Prospective cost: Matching O((L+R+E)*sqrt(L+R+1)) time plus O(L+R+E) reachability. O(L+R) auxiliary storage. Endpoint validation precedes indexed traversal; allocation failure is untyped. No approximation surrogate is useful for the exact witness contract.

### 2026-10-30 · evidence

Heading `### Executed evidence, 2026-10-30` stays in evidence as `### Executed evidence`.

Source SHA-256: `d09b7306a4a5cc0d996a25e693595834d776de4a4e59b0e28d890a85283b2e56`. Three tests passed twice in debug and twice in release with identical normalized named results across all four runs. Both restored debug suites passed. Strict algorithms all-targets Clippy and narrow rustfmt checks passed. Four syntax projections repeated byte-identically; structural scaffold passed twice with identical reports (one intentionally missing external scaffold sidecar).

- `bipartite_vertex_cover-debug-1.log`: `94e6a905a1a9b63e1f8e60b66003f2edda5cc14b1a726816b89ed8d1174d3f71`

- `bipartite_vertex_cover-debug-2.log`: `87749a1ad1825cbb29c5fb3dea59a110c4f1db3805d7d06c72dc1b79bdfc65e9`

- `bipartite_vertex_cover-release-1.log`: `c70d93ad3656ceab9c61b94d4b294bd4f7daf891cec5868e262df009def95ef7`

- `bipartite_vertex_cover-release-2.log`: `2e195f38bdf4823719c9aa57415f66dd07d95276ed27584ca9fe3259c1be629b`

- `bipartite_vertex_cover-restored.log`: `92cf9fd72052dfc92e2220c8768c7ceddf3b7d36a5337facd4ef6e410e6f9bf8`

- `cover-mutant.log`: `39cfb991835461915e1391d2d151e2b400ecd4d816f2a1c4fdd6d78acd323f34`

### Undated · document

Source snapshot: `d09b7306a4a5cc0d996a25e693595834d776de4a4e59b0e28d890a85283b2e56`. Domain: Left adjacency rows G and right population R, combined vertices at most 1000000 and total stored edges at most 1000000; duplicate neighbors allowed.
