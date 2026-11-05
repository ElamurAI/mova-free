# prag-invariants-unsaid-anchors

## forward

With a frame-aligned predicate anchor, inspect direct children unused by any anchored role. For each unsaid slot in order, Agent matches passive OblAgent or nonpassive-subject Nsubj; Patient matches NsubjPass or non-time Obj; Recipient matches Iobj. First matching child emits a hit. Other slots abstain. Missing anchors/predicates suppress only this check.

## invariant

Predicate/role token indices are zero-based, hits are one-based frame rows. Token use excludes the dependent regardless of which role consumed it. Frame order and anchor order must agree. Valid Tree cached children and anchor indices are premises; malformed public state can panic and is not silently repaired.

## cost

For F frames, U unsaid slots, D direct children and R anchored roles per frame: O(sum U*D*R) worst-case linear membership checks plus text cost; O(R) temporary used-index storage per frame. No optimization or measured gain is asserted.

## composition

Public APIs sentence, Spread::add/counts/hits consume the same ordered frames. Native fixtures construct Tree/Frame/Anchor directly. The papers frames-check caller invokes frames_of_tree and sentence before ctx.new_line, then Spread::add on retained frames; caller corpus/teacher/persistence paths are inspected only.

## epoch-memory

Initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib invariants:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: With a frame-aligned predicate anchor, inspect direct children unused by any anchored role. For each unsaid slot in order, Agent matches passive OblAgent or nonpassive-subject Nsubj; Patient matches NsubjPass or non-time Obj; Recipient matches Iobj. First matching child emits a hit. Other slots abstain. Missing anchors/predicates suppress only this check. Also require: Predicate/role token indices are zero-based, hits are one-based frame rows. Token use excludes the dependent regardless of which role consumed it. Frame order and anchor order must agree. Valid Tree cached children and anchor indices are premises; malformed public state can panic and is not silently repaired. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

No hit admits absent anchors, absent dependent, used dependent, nonmatching slot or time-object exclusion. These antecedents remain distinct; no-hit cannot identify inheritance correctness. A hit retains slot/row/dependent relation but not all anchor roles or prior discourse.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Anchor exclusion counterfactual fixes the tree/frame and marks a direct child used; its unsaid-own contribution vanishes. Forty finite relation/passive/used/anchor cases assert row 2 and exact hit cardinality. These supplied trees are created directly through Tree::of_words, without a live parser.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
