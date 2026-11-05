# world-tree-ingestion

Model path, loading, token/case adapters and explicit Word indexing.

## forward

model_path selects WORLD_EN_MODEL, else MOVA_DATA/en/models/en.bin, else HOME/mova-data/en/models/en.bin. ANN memoizes Result once, including failure. of_forms returns empty without annotate for empty forms, otherwise case heuristics, annotate, optional repair, lowercase and index. parse_cased preserves input case until annotate. of_words only indexes; it does not normalize.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## failure-cache

Type: state_transition. Domain and observation: The first load Result fixes ANN for the process; later environment changes cannot cause another load. Error formatting can use a later model_path, so displayed path need not identify the original attempted file. Source model only; no global environment mutation test. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## head-validation

Type: constraint_relation. Domain and observation: Indexing ignores head 0 and head>n, accepts self-heads and cycles, and never repairs them. Exhaustive 216 head assignments check the indexing relation. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## case-intervention

Type: counterfactual. Domain and observation: Bypassing the casing heuristic before annotation changes opaque annotator input, whereas of_words preserves exact mixed-case strings. No parser-accuracy ordering between adapters is claimed. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## invariant

Synthetic trees retain input Word values and deterministic order. Valid cached indices are required for traversal; constructor-built child lists may now contain cycles. Models retain ambiguous preimages, distinguish invalid indices from invalid head values and never infer semantic English correctness from structural agreement.

## composition

This fragment shares the same source and dependencies with its siblings; overlap is deliberate. Parser and repair calls are opaque boundaries; the source syntax projection does not expand macros or resolve types.

## cost

Tree indexing O(n) space/time; subtree on constructor-built cached children O(n*(1+|cut|)+n log n), dominance O(n), text proportional to selected bytes. Clause scans and repeated subtree membership can be O(n^2); frame scans O(n) plus possible clause extraction, string output proportional to bytes. Parsing adds an unmeasured tokenizer/dictionary/annotation/I/O cost and up to O(n) candidate parses. No speedup is claimed.

## checks

Native inline tests: cargo test -p world --lib wave7_tree -- --test-threads=1. Tree: 216 head assignments, six one-word fiber candidates, 1536 forest/root/cut cases, 384 dominance comparisons, cycle witness and index panic. QFrame: 45 literal classification cells, six WhoObj preimages, 135 complement memberships, priority, duplicate roles, invalid predicates, causative coupling and display fixtures. These counts describe file-level evidence; fragments only inherit relevant checks. Repeated debug/release, compiled mutants and syntax hashes are recorded in ../model.md#evidence at the file root.

## epoch-memory

First model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Initial review found sub lacked cycle detection; the follow-up below repairs termination without a visited allocation; public w can invalidate cached kids if mutated. show(Tree) omits lemma; the follow-up corrects its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. The initial epoch changed no production behavior; the follow-up changes cyclic subtree termination.

## limitations

Annotator model loading, OnceLock/error environment interaction, tokenizer accuracy, rerank repairs, dictionary generated tables, ans lemma fallback and runtime I/O are dependency gaps. Tests never load the parser, call a teacher, GPU or live API. No full parser accuracy, general English coverage, expanded semantic AST, executable surrogate or universal correctness claim. Public w mutation may stale cached kids or shrink w; arbitrary mutation remains outside the stable snapshot contract. Constructor-built cyclic subtrees are covered by the follow-up.


## cycle-guard-followup

Cuts remove traversal at non-start vertices before expansion; the start is
retained even when listed in cut. The result is the sorted set reachable from i
in cached children without passing a non-start cut vertex. The complement
contains exactly input/output pairs differing from this set; the reverse
relation retains every compatible head/cut assignment, including root-cut and
outside-component cut ambiguity. The original finite forest observations remain
valid, and the new graph enumeration checks the extended domain.

The guard costs one comparison per explored edge, O(1) extra guard storage,
with O(v) output/stack and O(v*|cut|+v log v) time for v visited/reached vertices
(up to n). No O(n) visited allocation is introduced in production. The
independent oracle deliberately does allocate visited flags and scans raw
heads; this is test cost, not production cost.

Five native tree tests pass. The new oracle enumerates n=1..4, every head from
0 through n+1, every root and every cut subset: 86078 comparisons. It checks
self-loops, multi-node cycles, branching components, roots and invalid heads
using independent visited-set reachability. An initial hand-entered comparison
total of 86206 was corrected to 86078 after all relational assertions passed;
the original failed count log is preserved. A bounded compiled mutant dropping
all children fails the oracle; the old potentially unbounded cycle algorithm
is never run as a mutant. See ../model.md#cycle-evidence at the file root for hashes.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Initial review found sub lacked cycle detection; the follow-up below repairs termination without a visited allocation; public w can invalidate cached kids if mutated. show(Tree) omits lemma; the follow-up corrects its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. The initial epoch changed no production behavior; the follow-up changes cyclic subtree termination.

### 2026-10-30 · cycle-guard-followup

2026-10-30, source-bound follow-up. Model before implementation: child lists
are built by assigning each Word index to at most one parent list. Thus every
vertex has at most one incoming edge in the parent-to-child graph. A cycle
reachable from start i must include i: entering any other cycle from outside
would give its entry vertex two incoming edges. Consequently the only repeated
vertex during downward traversal can be i. Suppressing enqueue(child==i)
breaks that closing edge while retaining every reachable vertex once. This
also handles a self-loop and permits branching outward from a cycle.
