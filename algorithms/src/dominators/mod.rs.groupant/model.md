# algorithm-dominators

## descriptant

### dominators mod

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# dominators mod model

## Forward

Directed graph n<=256,m<=65536 with duplicate arcs allowed. Valid start; unreachable vertices output empty. Initialize each reachable non-start dominator set to reachable universe, intersect predecessor sets plus vertex until fixed point; start singleton. O(V^2(V+E)) time conservative, O(V^2+E) memory. Iterative, no recursion. Aho, Sethi, Ullman Compilers (1986). No Lengauer-Tarjan complexity claim.

## Invariant

Unreachable rows remain empty; start remains singleton. Reachable rows shrink monotonically, remain supersets of true dominators, and satisfy predecessor intersection at termination.

## Cost

Directed graph n<=256,m<=65536 with duplicate arcs allowed. Valid start; unreachable vertices output empty. Initialize each reachable non-start dominator set to reachable universe, intersect predecessor sets plus vertex until fixed point; start singleton. O(V^2(V+E)) time conservative, O(V^2+E) memory. Iterative, no recursion. Aho, Sethi, Ullman Compilers (1986). No Lengauer-Tarjan complexity claim.

## nonbehavior

Type: behavior_complement. Let R be the deterministic relation specified by Forward including validation precedence and tie rules, X the declared input type and Y its Result type. Behavior complement is (X times Y) minus R. Specifically outputs violating Unreachable rows remain empty; start remains singleton. Reachable rows shrink monotonically, remain supersets of true dominators, and satisfy predecessor intersection at termination. are excluded; a valid-input Err or a wrong successful witness is excluded. Resource, cycle and structural rejections are determined before algorithmic output.

## compatible_inputs

Type: reverse_reconstruction. Given D, antecedents are graphs whose start-to-v paths all contain exactly D[v] as universal vertices; unreachable v have empty rows. A chain and the chain plus self-loops have identical D, so original edges cannot be recovered.

## deletion_certificate

Type: path_obstruction. For reachable v, d belongs to D[v] iff removing d makes v unreachable from start, counting removed start as reaching nothing.

## row_permutation

Type: representation_invariance. Permuting rows internally or duplicating edges preserves all dominator sets.

## edge_extension

Type: antitone_extension. Adding edges can only remove dominators from vertices already reachable; new reachability must be treated separately.

## self_and_start

Type: endpoint_law. For every reachable v, both start and v belong to D[v]; D[start]={start}. A disconnected component contributes no dominators regardless of its cycles.

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to dominators. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

### Prospective model

Directed graph n<=256,m<=65536 with duplicate arcs allowed. Valid start; unreachable vertices output empty. Initialize each reachable non-start dominator set to reachable universe, intersect predecessor sets plus vertex until fixed point; start singleton. O(V^2(V+E)) time conservative, O(V^2+E) memory. Iterative, no recursion. Aho, Sethi, Ullman Compilers (1986). No Lengauer-Tarjan complexity claim.
Allocation failure is outside typed errors. Resource caps precede vertex/edge validation. No callbacks. Exact categorical output has no justified percentage surrogate; executable approximation profiles unsupported.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to dominators. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/dominators-*.txt. No timing or whole-system improvement claim. Source SHA-256: 86ec8acb6ff360b7264aed23b0c9925626f994bcca5515f63ff5703035c4e733.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
