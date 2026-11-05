# world-tree

## model

# world-tree

Tree indexes preserve annotated words and expose ordered navigation; parser loading and annotation are opaque dependencies.

## forward

Construct kids[j] as increasing i with head[i]=j+1 and 1<=head[i]<=n. head maps invalid heads to None; root is first head-zero. sub returns sorted reachable indices avoiding cut descendants; dominates follows at most n+1 head steps. text preserves requested repetitions; content uses six UPOS values. parse/of_forms/parse_cased normalize forms and lemmas after optional repair. cap uppercases the first Unicode scalar; name_like consults dictionary and suffix exclusions.

## behavior-complement

Universe U consists of admissible input/observation pairs for the deterministic projections stated above, holding dependencies fixed. N={(x,y) in U | y differs from the stated forward observation F(x)}. This is the exact complement of the scoped mathematical model; Rust correspondence is finite for tested projections and a source-derived hypothesis elsewhere. Unknown parser transitions are excluded from this universe, not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining every compatible antecedent and fixed dependency state. Input inclusion x in B(F(x)) holds by construction; singleton recovery requires injectivity. Enumeration is complete only on the explicitly bounded test domains. Raw text, annotation history and omitted fields are not recoverable from projected outputs.

## head-fiber

Type: reverse_reconstruction. Domain and observation: For fixed n and observation head(i)=None, compatible head numbers are {0} union {h>n}; Some(j) has singleton {j+1}. Other Word fields are unrestricted. Six one-word candidates give five None witnesses. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## root-tie

Type: priority_relation. Domain and observation: With several zero heads root selects the minimum index; later roots remain stored. The 216 assignments test ties and absence. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## cut-reachability

Type: constraint_relation. Domain and observation: For a forest, node j is in sub(i,C) iff its ancestor chain reaches i before any cut node other than i. Cutting i itself never removes it. 1536 root/cut/forest cases compare an independent upward walk. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## cycle-boundary

Type: resource_boundary. Domain and observation: dominates terminates after n+1 iterations even with cycles. sub suppresses the closing edge back to its start. Each vertex occurs in at most one cached child list, so no other cycle is reachable from the start. The finite oracle covers cyclic inputs as well as forests. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## normalization-loss

Type: information_loss. Domain and observation: Lowercasing after annotation loses original casing; reconstruction is a set of case variants conditional on the opaque annotator/repair result, not original text recovery. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## observation-order

Type: encoding_relation. Domain and observation: text([1,0,0]) repeats form 0 and preserves caller order. sub sorts indices; children preserve insertion index order. These are different observable order contracts. Relation is conditional on the fixed source snapshot and stated domain. Evidence: native checks when named, otherwise source-derived prediction; unsampled behavior remains open.

## invariant

Synthetic trees retain input Word values and deterministic order. Valid cached indices are required for traversal; constructor-built child lists may now contain cycles. Models retain ambiguous preimages, distinguish invalid indices from invalid head values and never infer semantic English correctness from structural agreement.

## composition

The immediate fragments compose through the same annotated Word/Tree state and ordered projections. The root additionally covers inline native tests. Do not multiply independent fragment preimages: retain shared words, indices, flags and dependency state. Parser and repair calls are opaque boundaries; the source syntax projection does not expand macros or resolve types.

## cost

Tree indexing O(n) space/time; subtree on constructor-built cached children O(n*(1+|cut|)+n log n), dominance O(n), text proportional to selected bytes. Clause scans and repeated subtree membership can be O(n^2); frame scans O(n) plus possible clause extraction, string output proportional to bytes. Parsing adds an unmeasured tokenizer/dictionary/annotation/I/O cost and up to O(n) candidate parses. No speedup is claimed.

## checks

Native inline tests: cargo test -p world --lib wave7_tree -- --test-threads=1. Tree: 216 head assignments, six one-word fiber candidates, 1536 forest/root/cut cases, 384 dominance comparisons, cycle witness and index panic. QFrame: 45 literal classification cells, six WhoObj preimages, 135 complement memberships, priority, duplicate roles, invalid predicates, causative coupling and display fixtures. These counts describe file-level evidence; fragments only inherit relevant checks. Repeated debug/release, compiled mutants and syntax hashes are recorded in model.md#evidence at the file root.

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
is never run as a mutant. See model.md#cycle-evidence at the file root for hashes.

## Epoch notes

## evidence

Historical first-epoch evidence; current source and cyclic-domain results are in model.md#cycle-evidence.

## cycle-evidence

### Cyclic subtree termination

Current source SHA-256:
Current partial syntax SHA-256:

Tree::of_words accepts cyclic heads. Tree::sub now skips enqueueing its start
node, which closes any reachable cycle in the constructor-built cached graph.
The one-parent property supplies the proof; no production visited array or
hash set is needed. Existing acyclic traversal output and root-cut semantics
are preserved. The show comment now describes the actual displayed fields.

The independent visited-set oracle enumerates 1440 head assignments over
n=1..4 and heads 0..n+1, all roots and all cut subsets: 86078 comparisons.
Its graph search scans raw head assignments, marks visited nodes and compares
the resulting sorted indices with the native implementation. Cases include
self-loops, multi-node cycles, branches, invalid heads and cuts at the start.
The initial expected comparison count was incorrectly entered as 86206;
all relational assertions passed before that count assertion failed. The
correct total is 6+128+3000+82944=86078, and that failed log is retained.

Five native tree tests pass twice in debug and twice in release:
`cargo test -p world --lib wave7_tree -- --test-threads=1`
and the same command with `--release`.
Normalization keeps only per-test name/outcome lines; all four runs match.
Normalized SHA-256:

The compiled mutation replaces the child enqueue iterator with take(0).
The independent finite reachability assertion fails. The mutant is bounded;
the former potentially unbounded algorithm is never used as a control.
Mutation log SHA-256:
The exact source is restored and all five tests pass again.

Syntax generation runs twice and the two artifacts match byte-for-byte.
Current exact-source isolated groupant inventories pass twice and match.
A changed-source check against the temporary combined registry reports only
tree.rs and no missing entry. An intermediate isolated inventory captured
the active mutant and correctly rejected stale source bindings; the restored
copies and reports are the final structural evidence. Shared parent and
registry integration remains outside this commit.

This establishes bounded Rust correspondence plus the stated graph argument,
not full parser accuracy or safety after arbitrary public Word-vector mutation.
Cached children remain private and are formed from one-parent heads; mutation
of public w can still stale observations or invalidate word indices.

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

### 2026-10-30 · evidence

#### Bounded native evidence — 2026-10-30

Base revision: 6c2e5050c83c94b5f802ea4d3defd24bcaae516a.
Source SHA-256: 0a9bf363172cc16dd50355e3a7daca6fbad32131b9fcdb958fc35d0a2afcc579.
Partial syntax SHA-256: 2c7db250b2def98b69188a2ab4e8ea734d1adb84f891f8152893d52835f5bbf0.

Four new inline tests in this file; eight across the two-file batch. The filter
`cargo test -p world --lib wave7_tree -- --test-threads=1` selects this file.
Batch command uses `wave7_`; release adds `--release`. Two debug and two
release executions pass all eight tests (175 unrelated tests filtered out).
Normalization retains only per-test name/outcome lines, discarding compilation
warnings, paths, timing and test-summary durations. All four normalized outputs
match. Normalized SHA-256:
0cfce893ab697963f4ea80d35d8087a1ae3190c96d114086605db962b9f0294a.

Tree evidence enumerates 216 three-word head assignments with out-of-range heads and cycles, six one-word head-fiber candidates, 24 increasing forests times four roots times sixteen cut sets (1536 comparisons), and 384 dominance comparisons. Additional witnesses cover a bounded cyclic ancestor walk, duplicate/reordered text indices, content categories, Unicode capitalization and invalid-index panic. No cyclic subtree walk is executed.

Compiled negative control: change root's position to rposition; head_fibers_and_child_order fails with Some(2) versus Some(0).
The mutant compiles and exits with an assertion failure; it is restored byte-for-byte.
Mutation log SHA-256: 752f99b633787bbcb172237569694306cabdc811f8c939934694a8750ede6fd1.
All eight tests pass again after both restorations.

Syntax generation `mova-modelcheck syntax REPO world/src/tree.rs OUT`
runs twice and the artifacts compare byte-for-byte. Exact-source isolated
inventory copies validate this file and all its fragments twice with identical
reports. The full changed-source gate, using a temporary full registry extended
with the two entry.json records, reports both changed sources and no missing
entries. Shared registry/parent manifests are deliberately left for integration;
isolated checks are not a claim that those parents have already been updated.
An initial minimal registry run against the full checkout reported unrelated
ownership gaps, motivating the isolated exact-source checks.

Raw logs and Rust orchestration programs are in the external run directory
`$MOVA_DATA/runs/octopus-wave7-tree-2026-10-30`: debug1.log, debug2.log,
release1.log, release2.log, restored.log, tree-mutant.log, normalized.txt,
tree-isolated-1.json and tree-isolated-2.json. Native tests use actual
compiled crate methods, not source-text includes or copied reference code.

No production policy changed. No parser model load, teacher call, live API, GPU,
training, corpus evaluation or end-to-end answer-accuracy measurement occurred.
Dependency compilation and syntax validation do not close the runtime I/O,
generated dictionary, parser or semantic-analysis gaps.

### 2026-10-30 · cycle-evidence

Heading `### Cyclic subtree termination — 2026-10-30` stays in cycle-evidence as `### Cyclic subtree termination`.

Follow-up to ec4566983706cb3403d5ac65d4437e76d0d04a9b.

d09ffa6b6b99ab8cc422b8db84ef4b8e2136d62a68dc43db03be029916b17bd4.

27274c3dbc6d54c801d9306a8022d62a08eaf4b5e595072f2977445c50ec578a.

efcdf3e7eb67d5684df095ce85a7b2811f5022771e7a5016b3c5afc80f7d44db.

c930b66c99f577063f76c1edc5763559aec955ab42ff810a1ea65734bb166af4.

Evidence lives in the external run directory
`$MOVA_DATA/runs/octopus-wave7-tree-2026-10-30`:
cycle-debug1.log, cycle-debug2.log, cycle-release1.log, cycle-release2.log,
cycle-restored.log, cycle-mutant.log, cycle-normalized.txt,
cycle-debug-initial-count.log, tree-cycle-restored1.json,
tree-cycle-restored2.json and cycle-changed.json.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
