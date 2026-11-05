# world-qframe-extraction

Declarative roles, dictionary lemma fallback, declarative template, frame precedence and rendering.

## forward

decl_clause uses explicit valid predicate or first root, selects first nsubj and first obj in index order, gathers obl/iobj with first case and removes case nodes, takes xcomp/ccomp VERB/ADJ lemmas, scans all words for negation and content. It does not scope content to predicate descendants. frame applies causative rewrite before ordered WH classification; show is a lossy formatting projection.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## role-ties

Type: priority_relation. Domain and observation: Two nsubj children yield the first subtree as subject; second remains eligible content. Two obj children yield first object. Synthetic give/alice/bob/gift/coin demonstrates both tie policies. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## invalid-predicate

Type: error_partition. Domain and observation: Explicit predicate>=len returns default Clause fields with cloned Tree, even if a valid root exists. None uses root; empty Tree returns no predicate. These three cases are tested. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## global-content

Type: constraint_relation. Domain and observation: Content collection ranges over all Tree words excluding selected subject and filtering UPOS/aux/placeholder/negation rules, not merely predicate descendants. Causative re-extraction therefore may retain outer content; no semantic policy change is made. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## invariant

Synthetic trees retain input Word values and deterministic order. Valid indices and finite acyclic subtrees are preconditions for traversal. Models retain ambiguous preimages, distinguish invalid indices from invalid head values and never infer semantic English correctness from structural agreement.

## composition

This fragment shares the same source and dependencies with its siblings; overlap is deliberate. Parser and repair calls are opaque boundaries; the source syntax projection does not expand macros or resolve types.

## cost

Tree indexing O(n) space/time; subtree on a forest O(n*(1+|cut|)+n log n), dominance O(n), text proportional to selected bytes. Clause scans and repeated subtree membership can be O(n^2); frame scans O(n) plus possible clause extraction, string output proportional to bytes. Parsing adds an unmeasured tokenizer/dictionary/annotation/I/O cost and up to O(n) candidate parses. No speedup is claimed.

## checks

Native inline tests: cargo test -p world --lib wave7_qframe -- --test-threads=1. Tree: 216 head assignments, six one-word fiber candidates, 1536 forest/root/cut cases, 384 dominance comparisons, cycle witness and index panic. QFrame: 45 literal classification cells, six WhoObj preimages, 135 complement memberships, priority, duplicate roles, invalid predicates, causative coupling and display fixtures. These counts describe file-level evidence; fragments only inherit relevant checks. Repeated debug/release, compiled mutants and syntax hashes are recorded in ../model.md#evidence at the file root.

## epoch-memory

First model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Review found sub lacks cycle detection and requires a forest; public w can invalidate cached kids if mutated. show(Tree) omits lemma despite its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. No production behavior changed in this epoch.

## limitations

Annotator model loading, OnceLock/error environment interaction, tokenizer accuracy, rerank repairs, dictionary generated tables, ans lemma fallback and runtime I/O are dependency gaps. Tests never load the parser, call a teacher, GPU or live API. No full parser accuracy, general English coverage, expanded semantic AST, executable surrogate or universal correctness claim. Future direct public w mutation and cyclic subtrees are outside the stable forest contract.

## template-and-lemma-boundaries

first_lemma chooses the maximum-count verbal dictionary analysis (last equal
maximum in iterator order), lowercases its lemma, otherwise uses the second
ans::lemmas result or the original form. Dictionary iteration order and
generated records are dependencies, not covered by the synthetic tests.
decl_tree emits explicit subject or someone, then an allowed auxiliary, then
rest and a period; the returned predicate position is the prefix length before
rest. This index assumes annotation preserves token positions; empty rest can
place the index at punctuation. These are source-derived models with runtime
annotation correspondence still open.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Review found sub lacks cycle detection and requires a forest; public w can invalidate cached kids if mutated. show(Tree) omits lemma despite its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. No production behavior changed in this epoch.
