# algorithm-transitive-reduction-dag

## descriptant

### transitive_reduction_dag mod

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# transitive_reduction_dag mod model

## Forward

Directed DAG n<=256,m<=65536, normalize duplicate arcs; reject cycles incl loops. Compute Boolean Warshall reachability, retain edge u->v iff no other direct successor w reaches v. Ascending rows; closure invariant, unique minimal reachability-equivalent DAG. O(V^3+E) time O(V^2) memory. Aho, Garey, Ullman (1972).

## Invariant

Warshall after k permits intermediate vertices through k. Reduction keeps precisely original edges with no alternate path; all original reachability remains.

## Cost

Directed DAG n<=256,m<=65536, normalize duplicate arcs; reject cycles incl loops. Compute Boolean Warshall reachability, retain edge u->v iff no other direct successor w reaches v. Ascending rows; closure invariant, unique minimal reachability-equivalent DAG. O(V^3+E) time O(V^2) memory. Aho, Garey, Ullman (1972).

## nonbehavior

Type: behavior_complement. Let R be the deterministic relation specified by Forward including validation precedence and tie rules, X the declared input type and Y its Result type. Behavior complement is (X times Y) minus R. Specifically outputs violating Warshall after k permits intermediate vertices through k. Reduction keeps precisely original edges with no alternate path; all original reachability remains. are excluded; a valid-input Err or a wrong successful witness is excluded. Resource, cycle and structural rejections are determined before algorithmic output.

## compatible_inputs

Type: reverse_reconstruction. Given DAG H already transitively reduced, every graph formed by adding any subset of H's reachable pairs reduces to H. Duplicate input edges also collapse. Cyclic Error recovers any directed cycle, without a unique witness.

## essential_edge

Type: deletion_certificate. Edge (u,v) is retained iff deleting it from the normalized input destroys u-to-v reachability. This characterizes unique DAG reduction.

## closure_fiber

Type: constructive_preimage. For fixed reduced H, H and its transitive closure and every intermediate graph share the same reduction. On chain 0->1->2, adding 0->2 illustrates the nonunique fiber.

## idempotence

Type: fixed_point. Reduction R satisfies reduction(R)=R. Every retained edge is essential in R.

## representation

Type: representation_invariance. Input row order and duplicate count do not affect sorted output. A self-loop always causes Cycle; empty graph returns empty.

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to transitive_reduction_dag. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

### Prospective model

Directed DAG n<=256,m<=65536, normalize duplicate arcs; reject cycles incl loops. Compute Boolean Warshall reachability, retain edge u->v iff no other direct successor w reaches v. Ascending rows; closure invariant, unique minimal reachability-equivalent DAG. O(V^3+E) time O(V^2) memory. Aho, Garey, Ullman (1972).
Allocation failure is outside typed errors. Resource caps precede vertex/edge validation. No callbacks. Exact categorical output has no justified percentage surrogate; executable approximation profiles unsupported.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to transitive_reduction_dag. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/transitive_reduction_dag-*.txt. No timing or whole-system improvement claim. Source SHA-256: 58125a7da360e46fdcf8bb5b9f613f5c24ea808e44c576488abbfacebb512da7.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
