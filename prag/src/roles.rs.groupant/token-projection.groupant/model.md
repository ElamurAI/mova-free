# prag-roles-token-projection

## forward

Read supplied tree into event/speech frames. Choose first unused eligible predicate matching the first verb lemma token. For each filled slot choose closest nonpredicate form/lemma match; pronoun slots retain Ent.pron. Sort and deduplicate role tuples.

## invariant

Output positions are 1-based; tuples sorted unique. Equal-distance ties choose earliest token. Missing lexical matches produce no tuple.

## cost

External frame reading plus O(FN) token scans per fixed frame slot; O(R log R) role sorting.

## composition

Native Word -> Tree::of_words -> frames_of_tree -> roles/world_docs executes. CLI annotation/cache paths inspected only; temporary Ctx is not retained learned state.

## epoch-memory

Initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

## checks

cargo test -p prag --lib roles::wave12_models -- --test-threads=1. File-root ../model.md#evidence binds repeated profiles and mutants. No live parser/teacher/corpus or persisted exam evaluation.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: Read supplied tree into event/speech frames. Choose first unused eligible predicate matching the first verb lemma token. For each filled slot choose closest nonpredicate form/lemma match; pronoun slots retain Ent.pron. Sort and deduplicate role tuples. Also require: Output positions are 1-based; tuples sorted unique. Equal-distance ties choose earliest token. Missing lexical matches produce no tuple. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

Same tuples admit different parses/context states and duplicate frame emissions. Repeated lexical mentions are not uniquely recoverable from roles. Relation B(y) retains every modeled compatible antecedent; no unique inverse or complete executed solver.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## coupling

Native supplied cat/eat/fish tree has exactly agent (1,2) and patient (3,2). Parser quality and repeated-reference semantic correctness remain unproved.

Typed entries bind the manifest revision, stated domain and observation, assuming stable compiled dependencies and bounded execution. Status is partial source-derived relation with finite named checks. Partial syntax leaves macro bodies opaque and is no semantic proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.
