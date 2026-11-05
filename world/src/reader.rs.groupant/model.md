# world-reader

## model

# Question reader: world/index and own-text join

## forward
Inputs are a fact World fw, its Index ix, own World, own sentence trees, Qa and parser state. read parses the question, computes baseline(own,q), selects a Cand from state_answer(fw,ix,q,frame), then joins its one-based sentence to own trees. No candidate or confidence below 6 yields baseline. Otherwise describe/number with a nonempty slot prefer that slot; coverage >=0.5 prefers an available span except those slot cases; next comes slot, then any available span, then baseline. Happen clauses and adjective feelings may raise effective coverage to 0.5. Observation includes answer text, source, confidence, reason, frame and sentence; it is not correctness against a reference.

## invariant
qlemmas is a set; cover counts retained word occurrences, including subjects and subordinate content. For finite sentence/frame strings coverage lies in [0,1], empty denominator is zero. sconf never raises a finite score and unsupported short matches cap at 5.5. Slot candidates drop whitespace-only strings. Missing or zero sentence references yield no anchor tree after the checked subtraction repair. Fact order and source/own-world alignment are preconditions of meaningful interpretation, not enforced by Tale.

## cost
Let N be own sentences, F indexed facts, W retained question word occurrences, T anchor tokens, L lexical expansion cost, E entity candidates and K fields per candidate. Coverage builds a sentence lemma set in O(T L) expected hash work and checks O(W L); section envelope O(N times selected-section count). Candidate routing invokes branch-dependent searches and scans (roughly O(F K W L) plus index ranking), entity resolution and rendering external costs. No measured throughput bound. Parsing/model load and baseline search are separate dependencies; span_dev adds corpus I/O and reference scoring. Allocation/overflow failures are outside this finite cost model.

## behavior-complement
On finite scores and the stated join domain, forbidden observations include state answers below confidence 6, a coverage value above one, and a valid tree at anchor zero. No-fact routing cannot produce a candidate. This is a partial exclusion relation over input/output pairs; it does not forbid semantically incorrect high-confidence answers, shuffled-world answers, or unsupported causal guesses. Full complement of language meaning remains unknown.

## reverse-reconstruction
B(y) retains every (fw,ix,own,trees,q,parser state) whose forward observations equal y. Baseline text loses the candidate or its missing slot; identical slot text can originate from different facts. Even a fixed score 5.5 has uncapped and capped antecedents. Candidate IDs, words and parse traces are needed to narrow this relation, never to assert a unique original world. The finite confidence fiber test enumerates 12 antecedents in its declared 40-input domain.

## coverage-multiplicity
Hold sentence tree fixed. Coverage is H/W with occurrence counts, while qlemmas collapses repetitions. The frames [apple,pear] and [apple,apple,pear] have the same lemma set but cover 1/2 and 2/3 against apple. Test enumerates 36 count pairs and a subject/subordinate join. This is a sufficient counterexample to substituting set Jaccard for cover.

## section-envelope
sec_span returns the min/max one-based positions matching selected section IDs, else (1,N). It includes gaps whose sections are not selected; index Query retains exact section IDs separately. Empty text yields (1,0). Domain N<=65535 avoids narrowing and addition overflow. Fixtures cover separated duplicate section IDs and absent sections.

## confidence-fiber
For finite score s, predicate bit p and matched-word weight w, sconf=s if p or w>=2, otherwise min(s,5.5). On scores {0,5,5.5,6,9}, p in Booleans, w in {0,1,2,3}, the 5.5 fiber consists of eight direct inputs and four capped inputs. The gate is a heuristic, not a probability calibration.

## anchor-join
An index fact sentence refers to own trees by sent-1, including the deliberate shuffled-world control in main.rs. Safe lookup does not establish that these worlds describe the same event. Zero and out-of-range anchors are absent. The old expression subtracted before filtering, panicking for zero in debug; the checked-subtraction regression records this correction.

## temporal-priority
Feeling prioritizes cause-linked, nearby, then earlier facts with branch-dependent scores. Happen refuses a supplied subordinate clause with no accepted anchor; it tries two cause effects, removes echoes, then two later facts. Do scores nonnegative events by remaining words minus 0.05 per sentence distance; strict improvement preserves first tied input. These are source-derived partial models; supplied helper tests do not validate every branch.

## acquisition-boundary
Public read invokes qframe::parse; trees invokes the global annotator and returns acquisition errors. qlemmas, cover, section and confidence helpers consume supplied data without parsing. span_dev uses held reference answers for development and is not a runtime guarantee. No annotator acquisition, live teacher, corpus-quality or end-to-end world-building claim follows from the tests.

## composition
lexical.groupant models question-word observations; candidate.groupant models state routing; arbitration.groupant models candidate/anchor/fallback coupling. The children share frame, fact order, and world identity. Their composition includes external qframe, ans, index, span, types and World operations, not merely three independent function products. Reader belongs under an inference/QA interaction parent; treeread is a distinct quantitative path.

## checks
Native filter reader::wave8_models exercises five deterministic tests: confidence partition/fibers, coverage multiplicity, section envelopes, checked anchors, lemma-skip/labels. See model.md#evidence for executed profiles and mutations. Catalog routing relations beyond these helpers are inspected, not exhaustively executed.

## epoch-memory

First source-bound model. Prospective prediction: zero anchor must be absent, confidence without a predicate or two words cannot cross six, and coverage must retain multiplicity. Native checks expose the zero-anchor subtraction fault; repair preserves all positive anchors. No numerical surrogate is proposed: categorical routing and language adequacy lack an established approximation metric. Partial syntax projection has opaque macros, no expansion or semantic AST.

## evidence

### Executed bounded correspondence

Five native tests in `reader::wave8_models`; ten combined `wave8_models` tests passed twice in debug and twice in release with `--test-threads=1`, and once after restoring both compiled controls. The normalized ten test identities/results match across all five runs. No ignored tests, annotator acquisition, GPU, live teacher, corpus or held-out evaluation. Existing unrelated compiler warnings remain.

Reader control raises the production confidence word threshold from two to three; `confidence_partition_and_fibers` fails equality. Treeread control replaces the production total sum with product; `total_oracle_and_sum_fibers` fails equality. Both compiled, exited with failing assertions, and were restored before the final native pass. Separately, `anchor_zero_and_range_are_absent` failed with subtraction overflow before the checked-subtraction repair. These are test sensitivity observations, not proof of all branches.

Raw logs are external under run-relative names `debug-1.log`, `debug-2.log`, `release-1.log`, `release-2.log`, `restored.log`, `reader-mutation.log`, `treeread-mutation.log`, and `before-fix.log`; no host paths are committed. A separate one-file registry validates each root and its three fragments; the main registry and inter-file ancestry are integrated by the parent task. Models deliberately retain parser, overflow, entity identity and reading-quality gaps.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound model. Prospective prediction: zero anchor must be absent, confidence without a predicate or two words cannot cross six, and coverage must retain multiplicity. Native checks expose the zero-anchor subtraction fault; repair preserves all positive anchors. No numerical surrogate is proposed: categorical routing and language adequacy lack an established approximation metric. Partial syntax projection has opaque macros, no expansion or semantic AST.

### 2026-10-30 · evidence

Heading `### Executed bounded correspondence — 2026-10-30` stays in evidence as `### Executed bounded correspondence`.

- Source SHA-256: `383aeebdc514654bdeeba383bfe6b5c00d6c799ba65f06f29621da0392dfaf1b`.

- Partial syntax SHA-256: `173ba70840d7e607893502e32dc6ce30abfc8827df8e34f3cd0b29edcb138148`; two generations byte-identical.

- Combined normalized result SHA-256: `a3d9d7b1ab323e2b7450b0080a8719c9182c20a69724b0bb8378fb8b754c1071`.

- Compiled control log SHA-256: `bf5f619d77d524f16742fff34c85dfb0799b924cf14e1e78da19433876ce968f`.

Isolated structural validation: copied this exact source/sidecar into a temporary Git repository with its one-node registry; all root/three-fragment declarations passed twice, with byte-identical reports and zero diagnostics. Report SHA-256: `e00d388b2bdac0b3b632ec832280cd392d6eddedc44074660c464c732b957b8c`. The full worktree cannot use a one-node registry because existing unrelated sidecars retain their own owners; this isolated check is deliberately separate from the parent's complete integration check.
