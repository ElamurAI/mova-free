# world-qframe-orchestration

Subordinate split, WH/auxiliary recognition, candidate ranking and nominal/verbal frame assembly.

## forward

Choose first eligible SUBS at index>=2, excluding once upon and allowing as only soon/long as. as long as is stored as as soon as by current implementation. WH search is limited to first 3 tokens. Candidate scan stops before relative pronouns, skips determiners/negatives/pronouns and after-determiner positions, prunes later to-verbs, prefers validated share>=0.3 or context, then first validated, then first raw. Auxiliary-as-predicate and nominal fallback have distinct subject/focus conventions.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## candidate-ties

Type: priority_relation. Domain and observation: With fixed candidate sequence C and parser acceptances V, choose first V satisfying share/context, else first V, else first C. Equal shares do not sort candidates. This source-derived policy remains unexecuted because annotation is opaque. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## marker-loss

Type: information_loss. Domain and observation: Both as soon as and as long as map to mark as soon as; token suffix starts after three tokens. Reconstruction of mark alone retains both antecedents; semantic appropriateness is unproved. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## dependency-error

Type: error_partition. Domain and observation: Annotator failure returns Generic, Wh None, empty main/sub, while retaining computed mark and original q. Successful tokenizer output alone cannot establish successful QFrame parsing. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

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

## History

### 2026-10-30 · epoch-memory

2026-10-30: first model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Review found sub lacks cycle detection and requires a forest; public w can invalidate cached kids if mutated. show(Tree) omits lemma despite its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. No production behavior changed in this epoch.
