# kg-textgen-order

## descriptant

### kg-textgen-order

Bigram observations and subset maximum-score order.

## approximant

### Forward relation

Observe sorted explicit heads, add start/end transitions. logp=ln((count+.1)/(from+3.2)). DP[mask,last] updates unused next and backpointers, final adds end transition. D[{i},i]=logp(<s>,c_i); for j∉S, D[S∪{j},j]=max_i∈S[D[S,i]+logp(c_i,c_j)]. Final score max_i D[All,i]+logp(c_i,</s>); backpointers recover maximizing permutation.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

For n1..12 and finite counts, recovered order is a head permutation maximizing this recurrence.

#### Cost and resources

O(n^2 2^n) time/O(n2^n) state; each score constructs string keys.

## cost

### Cost and resources

O(n^2 2^n) time/O(n2^n) state; each score constructs string keys.

## composition

### Composition

Bigram observations and subset maximum-score order.

Observe sorted explicit heads, add start/end transitions. logp=ln((count+.1)/(from+3.2)). DP[mask,last] updates unused next and backpointers, final adds end transition. D[{i},i]=logp(<s>,c_i); for j∉S, D[S∪{j},j]=max_i∈S[D[S,i]+logp(c_i,c_j)]. Final score max_i D[All,i]+logp(c_i,</s>); backpointers recover maximizing permutation.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

No transition reuses a selected head. Private n0 is unsafe, and public generate guards it; no unbounded-n contract.

## reverse_reconstruction

Order corresponds to all count tables with same chosen optimum/ties; observations cannot reconstruct sentence histories uniquely.

## class_alias

Prep and quantity details collapse through class before observation; objects retain case.

## tie_policy

Strict > retains first predecessor, last equal final max endpoint selected.

## smoothing

Unseen transition count0 remains finite log probability with alpha.1 and fixed32 classes.

## position_tie

Heads sorted by tuple(word,class); tied coordinates use lexical class, not original member order.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- textgen::tests::subject_verb_object
- textgen::wave29::variant_priority_and_head_bounds

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
