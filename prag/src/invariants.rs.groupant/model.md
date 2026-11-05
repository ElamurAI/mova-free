# prag-invariants

## model

# prag-invariants

## forward

Domain X is tagged Sentence(Tree,ordered frames,aligned anchors), or finite Spread action trace starting at default state. Sentence requires valid cached Tree child indices and in-range predicate anchors, with anchors[i] belonging to frames[i]. For frame i, row=i+1; append copula-no-who iff Fact Is/IsA or non-Bare verb be lacks agent, time-in-where iff time_word(place.id,place.named), and who-is-what iff equal nonempty ASCII-case-insensitive agent/patient IDs survive reflexive and both-bare exclusions. Then each unsaid slot uses its aligned predicate's first unused qualifying child; Agent accepts passive OblAgent or nonpassive Nsubj, Patient NsubjPass or non-time Obj, Recipient Iobj; other slots abstain. Spread appends only VerbNet/nonempty-signature frames to key (verb,signature), class, ordered (id,i+1) occurrences. counts=(number of keys,number of keys with >1 class). Winner per key is lexicographically smallest class among maximal occurrence counts; hits emits every losing occurrence in key/class order and within-class insertion order.

## invariant

Sentence hits preserve 1-based frame row association; anchors and token indices stay zero-based and are never substituted for frame row numbers. Missing anchor/predicate suppresses only unsaid-own. Any anchored-role use excludes that token for all unsaid slots; reflexives and both-bare unresolved identities exclude who-is-what. Multiple hits per row and repeated unsaid slots are possible. Spread counts occurrences, including repeated add calls/IDs; no deduplication or absolute majority is required. Single-class groups have no losers, ties choose smaller class and different verb/signature keys never share a winner.

## cost

Sentence: O(F) direct checks plus sum over frames of O(U*D*R) unsaid child scans/membership, lexical lookup and text bytes; O(R) temporary used indices plus output. Spread add: O(F(log G+log C)) map work and copied strings; O(K) retained accepted occurrences. counts scans class-map cardinalities; hits scans classes and losing occurrences with repeated detail formatting. No corpus/parser/teacher/persistence cost is measured.

## composition

Public APIs sentence, Spread::add/counts/hits consume the same ordered frames. Native fixtures construct Tree/Frame/Anchor directly. The papers frames-check caller invokes frames_of_tree and sentence before ctx.new_line, then Spread::add on retained frames; caller corpus/teacher/persistence paths are inspected only.

## epoch-memory

Initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

Documentation audit: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

## checks

cargo test -p prag --lib invariants:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

For the tagged input domain X above and matching output schema Y, let J(x,y) be the concrete forward/invariant constraints above. C_inner={(x,y) in X×Y | not J(x,y)} is a source-derived inner model of absent behavior. In particular exclude candidate outputs that violate: Sentence hits preserve 1-based frame row association; anchors and token indices stay zero-based and are never substituted for frame row numbers. Missing anchor/predicate suppresses only unsaid-own. Any anchored-role use excludes that token for all unsaid slots; reflexives and both-bare unresolved identities exclude who-is-what. Multiple hits per row and repeated unsaid slots are possible. Spread counts occurrences, including repeated add calls/IDs; no deduplication or absolute majority is required. Single-class groups have no losers, ties choose smaller class and different verb/signature keys never share a winner. The native finite checks support selected members/nonmembers; no general Rust correspondence proof or claim about malformed state outside X is made.

## reverse-reconstruction

B(tag,y) retains all well-formed trees, ordered frames, aligned anchors and sentence-hit traces satisfying the predicates, or all initial-default Spread add histories and class occurrence maps yielding the selected counts/hits observation. Sentence absence can arise from valid roles, reflexive/bare exclusions, missing anchors, used children or absent matching dependents; retain these distinct causes. Spread counts lose proportions and IDs; losing hits omit winner occurrence IDs and ignored frames. Occurrence duplicates and class insertion order remain latent state. Neither diagnostic hits nor their absence reconstruct actual referents or semantic truth.

## intervention

Hold a valid Tree and frame sequence fixed and add a qualifying dependent token to the aligned anchor's used-role set: its unsaid-own eligibility disappears regardless of which anchored role consumes it; other frame checks are unaffected. Separately hold a Spread occurrence map fixed and reverse the lexicographic winner tie policy: only keys with equal maximal counts can change winner and losing rows. The finite anchor and plurality oracles exercise these two scoped interventions; neither establishes real semantic correctness.

## evidence

### Native evidence

Nine scoped tests (three new wave12_models, six existing) pass twice in debug and twice in release. No production semantic edit. Finite new fixtures cover 64 class histograms, three separate verb/signature groups, 40 dependency/anchor combinations, 16 copula combinations and five identity/reflexive controls. The compiled production mutant reversing the class tie break fails finite_spread_plurality_ties_rows_and_groups.

Each profile has equal normalized named test identities/results. Both mutants were rejected by a named runtime assertion, not compilation failure. Original bytes were restored exactly and all seven wave12 tests then passed. No live parser, witness model, teacher, DB, corpus or course was run. Existing reader tests use supplied annotation and local compiled lexical dependencies. PRAG_AGREE_ALL was removed from test process environment; optional P3-P5 branches are inspected only.

Syntax generation repeated byte-identically. Full-base external registry scaffold structural validation repeated identically with no diagnostics; the external scaffold parent intentionally has no sidecar. Both changed sources are registered in that scaffold. Root owns final registry/ancestor integration. Structural checks establish bindings and references, not semantic verification.

Only newly added test modules were rustfmt-formatted. Legacy production style remains. Focused 15-test total is not the full prag suite.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

Documentation audit, 2026-10-30: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Source SHA-256: 64ddcdabea1704fc3216d06f5f299ec06171dc4432b7f5c9b7d4b7c4a4fbf190. Syntax SHA-256: a2746509f599ccbbec1e9aa13a262c4aa04d96c6da7cf35c0445f558d6e0a25c. Raw logs: $MOVA_DATA/runs/octopus-wave12-invariants-2026-10-30/.

#### Evidence hashes

- old-witness.log: 0f2a352305c080d627b7856fd2fc4c5e4e2b07de14098d660b9952aa7eb2be9f
- checks.log: 1e5808b504258641a740e7469f1be74613e690e71748a2a31f3217850091426e
- restored.log: 55bccc707d523e3f4ab9f4afde315b97c0e04d650269acbba8c3f4fe54792c26
- groupants1.json: 35c49caab79383b7ba65e0f2bdfff85db6dcff8eaa6dc3406b41798a455389c5
- groupants2.json: 35c49caab79383b7ba65e0f2bdfff85db6dcff8eaa6dc3406b41798a455389c5
- changed.json: 46789fd6edb9c2b279d19a59b1f012a0fb5cc5d6422e84bc333669b0a07d9a76
- debug-invariants-1.log: 8b4c65dc9f993c78efb6764f3f34ce93637464b9844e4dfeba284b9587f75849
- debug-invariants-2.log: 8b4c65dc9f993c78efb6764f3f34ce93637464b9844e4dfeba284b9587f75849
- release-invariants-1.log: f85ef58a0395a138d7ec5a9a5351c52edb385ed1951cd400ac8c6a3528c99ec0
- release-invariants-2.log: f85ef58a0395a138d7ec5a9a5351c52edb385ed1951cd400ac8c6a3528c99ec0
- mutant-invariants.log: aed1a02ecb0152604c42bcecffb1fcd3ba6e412b678f9a87ca38669504c42a29
