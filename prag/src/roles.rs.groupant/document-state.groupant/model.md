# prag-roles-document-state

## forward

School IDs group by course/lesson, books by book. Stable sort uses document, lesson number and sentence number. Reset Ctx and queue at each document; world_docs observes context before reading the sentence and keeps at most events recent strings. Return rows in original order.

## invariant

Cross-document recent events are excluded. Zero event bound retains no event strings. Equal sort keys preserve input order.

## cost

O(S log S) sorting plus external frame reading and summary costs; queue storage O(events).

## composition

Native Word -> Tree::of_words -> frames_of_tree -> roles/world_docs executes. CLI annotation/cache paths inspected only; temporary Ctx is not retained learned state.

## epoch-memory

Initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

## checks

cargo test -p prag --lib roles::wave12_models -- --test-threads=1. File-root ../model.md#evidence binds repeated profiles and mutants. No live parser/teacher/corpus or persisted exam evaluation.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: School IDs group by course/lesson, books by book. Stable sort uses document, lesson number and sentence number. Reset Ctx and queue at each document; world_docs observes context before reading the sentence and keeps at most events recent strings. Return rows in original order. Also require: Cross-document recent events are excluded. Zero event bound retains no event strings. Equal sort keys preserve input order. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

A prior summary admits every history with the same reported salience and retained event suffix. Omitted events, entities and original lesson IDs cannot be recovered. Relation B(y) retains every modeled compatible antecedent; no unique inverse or complete executed solver.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## coupling

Native tests check shuffled sentence IDs, document isolation, prior-state timing and bounds zero, one and two. These temporary worlds are not retained exam knowledge.

Typed entries bind the manifest revision, stated domain and observation, assuming stable compiled dependencies and bounded execution. Status is partial source-derived relation with finite named checks. Partial syntax leaves macro bodies opaque and is no semantic proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.
