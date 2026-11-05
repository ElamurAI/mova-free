# world-ideas-scope

## descriptant

### Normalized rarity ranking and cross-context carry

W(t)=BTreeSet of nerves::norm results over alphanumeric tokens. Read all ideas I; df(w)=number of idea rows i with w in W(i.text). Score(i,q)=Σ over w in W(i.text) intersect W(q) of floor(1_000_000/max(df(w),1)). Retain score>0, sort by descending score, kind rank held=0/principle=1/derived=2/other=3, descending time, ascending text, then take k. carried_over(ctx)=read rows with kind held AND ctx nonempty AND stored ctx!=ctx, preserving file order.

## approximant

### Normalized rarity ranking and cross-context carry

W(t)=BTreeSet of nerves::norm results over alphanumeric tokens. Read all ideas I; df(w)=number of idea rows i with w in W(i.text). Score(i,q)=Σ over w in W(i.text) intersect W(q) of floor(1_000_000/max(df(w),1)). Retain score>0, sort by descending score, kind rank held=0/principle=1/derived=2/other=3, descending time, ascending text, then take k. carried_over(ctx)=read rows with kind held AND ctx nonempty AND stored ctx!=ctx, preserving file order.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

Repeated tokens within one idea/query count once. No-overlap/zero-score rows disappear; k=0 returns no rows. Kind priority precedes recency. carried_over performs exact context string comparison and does not sort or deduplicate. These scoped functions read but do not record, seed or rewrite ideas.

Cost: Whole-file read/storage O(B); building distinct-word frequency map adds ordered-map work over total distinct words per row. Ranking computes set intersections then O(N log N) sort and keeps all rows until truncation. Sum uses u64 without checked overflow; frequency counts and allocations have no explicit cap.

## composition

The child joins parent read() persistence to nerves::norm and yields ranked Idea clones or carry candidates. read()/record()/seed_principles implementations are outside its byte scope; parent models their malformed-input and write behavior. Ranking receives file rows but does not make durability or deduplication guarantees.

## models

## behavior_complement

related cannot return a zero-score idea or more than k rows. Repeated apple tokens do not multiply relevance. At equal score, a principle row cannot precede held solely because it is newer. carried_over excludes non-held rows, empty contexts and exact current context; it does not mutate their evidence or time.

## reverse_reconstruction

related(apple,1) returning a held apple row is compatible with adding unrelated rows or repeating apple in the query; the unseen lower-ranked file suffix is not reconstructible. Changing case/plural punctuation can preserve W(q). carried_over(empty) admits all held rows with any nonempty context; its output alone cannot reveal filtered rows or the current context uniquely.

## document_frequency

df counts rows containing a normalized word, not total token occurrences. Duplicate idea rows do count separately, lowering per-word weight; floor division can produce zero when df exceeds 1_000_000.

## lexicographic_ranking

Comparison key is (-score,kind_rank,-time,text). Fully equal keys retain input order under stable sort; source/evidence/context do not break ties.

## query_normalization_join

Ideas words uses nerves::norm after token splitting. It therefore inherits stop filtering and suffix loss, while BTreeSet removes repeated normalized concepts. Talk's cue matcher is not a substitute.

## context_selection

carried_over compares raw strings: whitespace/case changes in a nonempty ctx can make it different. It selects only held ideas and preserves original record fields and ordering.


## tests

ideas::wave28_dialogue::isolated_tsv_update_and_rank exercises held-before-principle equal-overlap ranking, stop-only empty query and carried_over matching/different contexts on isolated TSV. The read/record/source-preservation assertions in that test belong to the parent persistence scope; the compiled record mutant is not credited to this ranking child.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. The child joins parent read() persistence to nerves::norm and yields ranked Idea clones or carry candidates. read()/record()/seed_principles implementations are outside its byte scope; parent models their malformed-input and write behavior. Ranking receives file rows but does not make durability or deduplication guarantees.
