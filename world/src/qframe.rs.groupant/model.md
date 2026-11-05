# world-qframe

## model

# world-qframe

Question interfaces, lexical gates, parser orchestration, declarative extraction, ordered classification and display.

## forward

tokens uses annotator tokenizer if available and fallback toks otherwise. parse cuts the first eligible subordinate marker from position 2, locates WH among first three main tokens, checks auxiliary/candidates, builds declarative trees, extracts Clause and classifies with frame. Parser loading/annotation are opaque; decl_clause and frame are exercised directly on synthetic annotated trees. show formats selected fields.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## classification-fiber

Type: reverse_reconstruction. Domain and observation: On lemma walk, no words and five focuses, WhoObj has six antecedents: Who or Whose times Obj, Adv or None. The full 9x5 literal table establishes finite converse membership without assuming uniqueness. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## priority-shadow

Type: priority_relation. Domain and observation: Why wins over feeling and special-description flags; HowMany wins Num; What feeling with non-subject focus precedes description. Three simultaneous-trigger witnesses distinguish the order. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## causative-coupling

Type: composition_relation. Domain and observation: What+Subj+make/cause/lead/force/drive with first xcomp/ccomp re-extracts that child; subject becomes old object, negation survives, passive becomes false, focus Adv. Feel versus Why depends on new lemma. Test couples actual Tree, decl_clause and frame. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## suffix-loss

Type: information_loss. Domain and observation: about and happen target use the first matching preposition and retain the entire following token suffix, not a parsed phrase. Earlier tokens cannot be recovered from these projections. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## parser-budget

Type: resource_boundary. Domain and observation: Candidate validation and nominal split search can invoke annotation O(n) times; costs depend on external annotator and dictionary. No latency or accuracy bound is measured. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## display-projection

Type: encoding_relation. Domain and observation: show emits kind/focus, selected flags, main fields and optional sub fields; q and many raw Tree fields are absent. Equal display strings are not equal QFrames. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## invariant

Synthetic trees retain input Word values and deterministic order. Valid indices and finite acyclic subtrees are preconditions for traversal. Models retain ambiguous preimages, distinguish invalid indices from invalid head values and never infer semantic English correctness from structural agreement.

## composition

The immediate fragments compose through the same annotated Word/Tree state and ordered projections. The root additionally covers inline native tests. Do not multiply independent fragment preimages: retain shared words, indices, flags and dependency state. Parser and repair calls are opaque boundaries; the source syntax projection does not expand macros or resolve types.

## cost

Tree indexing O(n) space/time; subtree on a forest O(n*(1+|cut|)+n log n), dominance O(n), text proportional to selected bytes. Clause scans and repeated subtree membership can be O(n^2); frame scans O(n) plus possible clause extraction, string output proportional to bytes. Parsing adds an unmeasured tokenizer/dictionary/annotation/I/O cost and up to O(n) candidate parses. No speedup is claimed.

## checks

Native inline tests: cargo test -p world --lib wave7_qframe -- --test-threads=1. Tree: 216 head assignments, six one-word fiber candidates, 1536 forest/root/cut cases, 384 dominance comparisons, cycle witness and index panic. QFrame: 45 literal classification cells, six WhoObj preimages, 135 complement memberships, priority, duplicate roles, invalid predicates, causative coupling and display fixtures. These counts describe file-level evidence; fragments only inherit relevant checks. Repeated debug/release, compiled mutants and syntax hashes are recorded in model.md#evidence at the file root.

## epoch-memory

First model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Review found sub lacks cycle detection and requires a forest; public w can invalidate cached kids if mutated. show(Tree) omits lemma despite its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. No production behavior changed in this epoch.

## limitations

Annotator model loading, OnceLock/error environment interaction, tokenizer accuracy, rerank repairs, dictionary generated tables, ans lemma fallback and runtime I/O are dependency gaps. Tests never load the parser, call a teacher, GPU or live API. No full parser accuracy, general English coverage, expanded semantic AST, executable surrogate or universal correctness claim. Future direct public w mutation and cyclic subtrees are outside the stable forest contract.

## evidence

### Bounded native evidence

QFrame evidence checks a literal 9-WH by 5-focus table (45 cells), its complete six-element WhoObj fiber and 135 complement memberships. Extra witnesses check three precedence collisions, six specialized predicates, first subject/object ties, oblique-case exclusion, invalid and absent predicates, causative child re-extraction, flag preservation, suffix selection, stable xcomp deduplication and exact display.

Compiled negative control: change Why classification to Generic; kind_fibers_and_priority fails with Generic versus Why.
The mutant compiles and exits with an assertion failure; it is restored byte-for-byte.
All eight tests pass again after both restorations.

Syntax generation `mova-modelcheck syntax REPO world/src/qframe.rs OUT`
runs twice and the artifacts compare byte-for-byte. Exact-source isolated
inventory copies validate this file and all its fragments twice with identical
reports. The full changed-source gate, using a temporary full registry extended
with the two entry.json records, reports both changed sources and no missing
entries. Shared registry/parent manifests are deliberately left for integration;
isolated checks are not a claim that those parents have already been updated.
An initial minimal registry run against the full checkout reported unrelated
ownership gaps, motivating the isolated exact-source checks.

No production policy changed. No parser model load, teacher call, live API, GPU,
training, corpus evaluation or end-to-end answer-accuracy measurement occurred.
Dependency compilation and syntax validation do not close the runtime I/O,
generated dictionary, parser or semantic-analysis gaps.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first model epoch. Prediction: finite synthetic projection checks agree exactly and reject an altered root/classification policy. Review found sub lacks cycle detection and requires a forest; public w can invalidate cached kids if mutated. show(Tree) omits lemma despite its comment. QFrame uses all-tree content and normalizes as long as to as soon as. Preserve these observations; broader policy changes require an explicit semantic witness. No production behavior changed in this epoch.

### 2026-10-30 · evidence

Heading `### Bounded native evidence — 2026-10-30` stays in evidence as `### Bounded native evidence`.

Base revision: 6c2e5050c83c94b5f802ea4d3defd24bcaae516a.

Source SHA-256: bcffcce37a9ab02dca723787937f7962f084acb7436bba4ba37bd4a98ea9db5b.

Partial syntax SHA-256: 7dbf9dbd62bac59a01e7e19cd8f07c6f8274287f94fa6fa2eeb95d21ac5a2831.

Four new inline tests in this file; eight across the two-file batch. The filter
`cargo test -p world --lib wave7_qframe -- --test-threads=1` selects this file.
Batch command uses `wave7_`; release adds `--release`. Two debug and two
release executions pass all eight tests (175 unrelated tests filtered out).
Normalization retains only per-test name/outcome lines, discarding compilation
warnings, paths, timing and test-summary durations. All four normalized outputs
match. Normalized SHA-256:
0cfce893ab697963f4ea80d35d8087a1ae3190c96d114086605db962b9f0294a.

Mutation log SHA-256: e23aa496f2ecace47d69b9ce0bd8e5d4db03ed00b3d2fce65ffa0a3c176dd81c.

Raw logs and Rust orchestration programs are in the external run directory
`$MOVA_DATA/runs/octopus-wave7-tree-2026-10-30`: debug1.log, debug2.log,
release1.log, release2.log, restored.log, qframe-mutant.log, normalized.txt,
qframe-isolated-1.json and qframe-isolated-2.json. Native tests use actual
compiled crate methods, not source-text includes or copied reference code.
