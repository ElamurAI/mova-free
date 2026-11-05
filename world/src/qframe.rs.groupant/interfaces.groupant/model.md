# world-qframe-interfaces

Enums, Clause/QFrame interfaces, lexical constants and dictionary/token gates.

## forward

QKind::name maps 19 variants to fixed labels. Clause::preds starts with nonempty non-light lemma, then stable-deduplicates xcomp, falling back to nonempty original lemma only when empty. wh_of is exact lowercase mapping. aux_want maps auxiliaries to permitted PTB tags. Dictionary gates use hit>0 and (tot==0 or hit*20>=tot or hit>=3); absent words use specified ed fallback. verb_share absent/zero total is 0.5.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## preds-loss

Type: information_loss. Domain and observation: Duplicate xcomp positions and excluded be/do/have are lost if a surviving xcomp exists. Empty result constrains lemma empty and xcomp empty. Test five lemma choices and duplicate run. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## lexical-boundary

Type: constraint_relation. Domain and observation: Unknown word fallback accepts ed length>4 for can_verb, additionally VBN membership for verb_form; verb_share uses 0.5 independently. Dictionary records and arithmetic overflow bounds are external assumptions. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## token-error

Type: error_partition. Domain and observation: tokens may fall back to ans::toks after annotator error, while parse then returns Generic empty frame. This is token fallback, not successful rule-based parser fallback; no load call is executed in native fixtures. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

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
