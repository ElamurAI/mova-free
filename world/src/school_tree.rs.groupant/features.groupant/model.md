## forward

Compute supplied-tree heads, clauses, subjects, passive and counted attachments.

## invariant

Aligned arrays and valid queried positions required. Clause member indices ascend, never duplicate across groups, and omit punctuation. Cycles without predicates drop their tokens after bounded traversal. head returns None for zero/out-of-range. subject returns first discovered subject/expletive within four inspected predicates. Exact relation subtype matters for passive and counted. No general answer-correctness or arithmetic-safety invariant.

## cost

n words: kids O(n), is_pred O(n). clauses O(n^2) predicate discovery plus n walks of <=n steps with linear predicate membership, conservative O(n^3); groups O(n) storage. subject<=4 child scans; counted may nest case_of in kids, O(n^2). Full fold repeats scans and lexical/parser calls. Relation closure clones relations and scans holdings; every successful step appends a previously missing actor, bounded by finite endpoint set under safe arithmetic. No measured full-reader cost.

## composition

Source-bound parent supplies exact shared state and dependency assumptions. Independently projected fragments do not prove their wiring.

## epoch-memory

Initial scope. Preserve unknown lexical, data and effect semantics. Source inspection and executed bounded helpers remain distinct.

## checks

Nine native tests in wave9_models (five prules, four school_tree), repeated twice per debug/release profile, plus compiled production mutants; see ../model.md#evidence in the file parent. Binomial word oracle n<=12, gate finite grid, supplied-head graph universe 6^4, feature boundaries and reverse fibers. No real learning/data-loading or parser-to-Reading run. Prospective math recorded before implementation; large-n old-code failure precedes repair.

## domain

Fix source revision, external dependency outputs, configuration and sufficient resources. Observe one terminating call or explicitly bounded trace. Arithmetic domains exclude overflow except named refusal tests. Specialized relations are source-derived partial models unless a finite oracle is explicitly named. No unrestricted reverse solver is implemented.

## behavior-complement

On aligned supplied arrays and valid query indices, no punctuation member or duplicate group membership, invalid-head Some, non-Nummod counted target, or passive result without passive child is admitted. Finite supplied graph/feature tests check these inner exclusions without parser calls.


## reverse-reconstruction

Retain all supplied trees satisfying observed positional/label features. Clauses erase ordinary forms, tags and dropped cycles; native sam/holds and lee/gives have identical clauses. Complete finite head enumeration has fixed relation/POS labels, so cannot recover arbitrary original sentence text.


## ancestor-partition

Follow head edges to first predicate; without predicate before invalid edge or repeated vertex, omit. Omit nonpredicate Cc beneath predicate. Group and sort by minimal member. Independent visited-set oracle agrees on all 1296 four-token head graphs, including head5 out-of-range, self cycles and punctuation roots; fixed relation/POS labels restrict the conclusion.

## History

### 2026-10-30 · epoch-memory

Initial 2026-10-30 scope. Preserve unknown lexical, data and effect semantics. Source inspection and executed bounded helpers remain distinct.
