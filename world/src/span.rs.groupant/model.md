# answer-span

## model

# answer-span

Subject: world/src/span.rs, whole-file SHA-256 in manifest, baseline 4279da541. Domain: finite acyclic valid trees with consistent child lists, valid indices and terminating dependencies. Observation: a single call return, not semantic truth or reachability. Hold Tree, ans::lemmas and types::FEELING dependencies fixed; parser/model loading and external caller wiring are gaps. All entries are source-derived partial models. Exact-target definitions are not executable inverses; finite tests cannot establish universal correspondence.

## forward

want_of routes metadata; extract finds predicate then dispatches ordered rules, usually cleaning and joining selected indices. Num bypasses clean. minus_question is separately called by external code.

## invariant

Non-Num success contains ordered unique indices, positive content and echo<=half. Inputs unchanged. Acyclic consistent Tree is a termination assumption, not checked here.

## cost

find_pred O(p*n*L) with morphology cost L. Conservative O(n^3+n^2 log n+p*n*L) for repeated subtree/sort/clean operations under constant-cost memberships; clean's front removals cost O(m^2). String bytes, hash worst cases and dependency internals remain gaps.

## epoch-memory

Initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

## checks

Structural inventory checks hashes, references, types and node bounds only. Proposed oracle: enumerate small annotated acyclic trees and compare literal expected returns, including None and labels; mutate priority or echo inequality to obtain a red control. Any executed bounded evidence is recorded separately in root model.md#evidence; catalog examples remain proposed until recorded there.

## composition

Children share immutable Tree and question state. Routing yields Want, predicate selection yields p, dispatch yields candidate indices, normalization yields text; Num bypasses normalization. Guarded branches compose as a union, sequential stages via existential intermediates. Reverse keeps all compatible intermediates; complement is this node's U minus composed R, not union of child complements. Why/clauses overlap extraction. minus_question is separate API and caller wiring is outside file. Do/Clause delegation falls through after After None but most direct branch returns stop on clean None.

## behavior-complement

U=terminating extract calls on valid acyclic trees paired with arbitrary optional (text,rule); R=actual return graph. Inner U minus R excludes Some on empty trees, a Num rule other than nummod, and non-Num success with no content or majority echoed content. Num bypasses cleaning; extending the latter exclusion to Num is false.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## reverse-reconstruction

Observed (text,rule) retains every compatible Want, tree, anchor and candidate index list reaching that label and output. None retains empty trees, absent anchors, rejected candidates and exhausted scans. This necessary-constraint outer preimage preserves repeated words, deleted edges and distinct syntax; there is no unique semantic inverse.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## routing-fiber

Fixed other frame fields: Say/Think map to Say; Want/Obj/WhoObj to Obj; WhoSubj/WhatSubj/WhoIs to Subj; Happen/HappenTo to After iff sub is Some, else Clause; Generic to Clause. Fibers retain all matching kinds and all payloads of the required sub presence.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## predicate-priority

For each preds entry in caller order, rank matching tokens VERB=0, ADJ/AUX=1, NOUN=2, other=3, preserving earliest rank tie. Accept rank<3 then stop. Earlier predicate noun beats later predicate verb. PROPN alone is rejected, even though downstream branches mention PROPN.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## cleaning-fiber

Fixed tree/qlem and observed indices J: retain every input list whose sort/dedup and asymmetric edge trim yields J and whose content passes 2*echo<=content, content>0. Duplicates, permutations and removable edges are lost; text-only observations additionally merge repeated forms.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## early-return-shadow

Inner U minus R on a fixed dispatch path excludes later candidate outputs once an earlier matched branch executes return out, including when clean returns None. Obj final oblique loop retries after failure, and Do/Clause After if-let falls through after None; these exceptions prevent a global no-retry claim.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## numeric-bypass

Want::Num returns raw form of first global base=Nummod OR UPOS=NUM token, label nummod. Predicate and qlem do not constrain it. All changes preserving this first qualifying token preserve observation, regardless of relevance to the question.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## location-time

Where local scan accepts selected locative Obl excluding TIME_N or LOC_ADV Advmod; global fallback scans Obl only. When searches verb_kids if p exists, all indices otherwise; first time-marked Advcl precedes temporal Obl. These lexical heuristics do not prove temporal/spatial semantics.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## reason-horizon

Why scans p and at most two permitted head steps before lexical fallbacks. An ancestor beyond three chain nodes is outside the structural scan, but lexical because/for/since can still recover words there. Complement exclusion requires holding fallback triggers absent.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## feeling-envelope

Feel scans all tokens irrespective of p for first eligible emotional predicate, then emits contiguous envelope of selected adjective/intensifier/conjunction indices. Intermediate unselected words may enter; rejected first candidate ends branch. Emotion noun fallback occurs only when no adjective candidate exists.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## fallback-objective

minus_question chooses maximal CONTENT-count run split by echoed content tokens, earliest tie, count>=2, then cleans once. Maximizing token length or trying a runner-up after cleanup failure defines a different system; equal-content unequal-length runs distinguish them.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## rule-fiber

Rule labels retain branch ambiguity: conj-after has child and sibling paths; obj/ccomp/xcomp occur in Obj and Say. A rule is an operational diagnostic, not a proof of causal or answer truth; different Wants and trees remain compatible.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## evidence

### Answer-span evidence

#### Executed checks

The source-bound syn projection repeated byte-identically. Two complete groupant inventories with the unchanged registry repeated byte-identically and reported valid=true, diagnostics=[], 19 sidecars and six registry group sidecars. This new leaf remains unregistered until the integration step; registry-node.json is its descriptor for inference-core. Inventory validation establishes structure, not model semantics.

The standalone Rust harness probe.rs.txt includes the current authoritative span.rs directly; it links pre-existing world/en dependency rlibs for Tree, question types, morphology and emotion tags. Those dependencies were not rebuilt or certified by this probe. It never loads a parser/model or contacts a teacher.

Thirty-two literal fixtures passed: three predicate-priority/PROPN cases, one numeric echo-bypass case, sixteen question-mask fallback cases, ten kind/sub-presence routing cases, and two object selection/rejection cases. An intentionally false retry-after-echo-rejection prediction was rejected. Two subsequent harness runs produced identical summaries:

    literal fixtures=32; wrong retry model rejected=true

This is a negative formula control, not a production-source mutation test. The result does not cover all Wants, malformed trees, end-to-end answering, parser accuracy, or general inverse completeness. Catalog experiments beyond these fixtures remain prospective.

#### Reproduction

From repository root, generate syntax with mova-modelcheck syntax into a new output and compare with syntax.json. The retained build_models.rs.txt is a Rust authoring helper using serde_json; compile it with --crate-name span_models and an available serde_json rlib, then run from root to regenerate model.md and manifest.json files. Re-generation overwrites those files; review model changes before running it.

Compile probe.rs.txt directly with rustc --edition=2024 --crate-name span_probe, supplying --extern world, --extern en and the dependency directory using local build artifacts. Keep the output binary outside the repository. The relative include resolves the unchanged span.rs beside this sidecar. Dependency artifact versions are an explicit reproducibility limit.

#### Source review observations

Num bypasses clean, predicates and echo filtering. Feel is global, and Say's quote pair is global once a predicate exists. Candidate cleanup often ends a branch with None instead of trying another candidate; Obj final obliques and Do/Clause's After delegation have distinct retry behavior. find_pred prefers earlier caller predicates and rejects rank-3 PROPN-only hits. verb_kids expands only Xcomp children and checks no shared subject, despite its broader comment. minus_question maximizes content count with earliest ties.

These are operational facts or comment/implementation mismatches, not established linguistic bugs. No runtime source, shared registry, active data, or learned state was changed. The partial AST has no macro expansion, name resolution or semantic verification.

## wave6

# Native selection observations

Revision: exact source SHA in this manifest. Observations are single terminating calls. Tree fixtures are directly annotated, acyclic and index-valid; dependencies are compiled from this checkout. No parser or teacher is loaded. Metric is exact equality of optional indices/text and rule labels; zero tolerance. Exhaustive claims apply only to explicitly bounded domains, with no semantic accuracy or general Rust proof.

## fallback-tie-intervention

Fix q={anchor} and the seven-token fixture ruby jade anchor the opal and pearl, with only ruby/jade/anchor/opal/pearl content. Runs have content counts (2,2) and token lengths (2,4). A chooses ruby jade; a token-length maximizer would choose the later run and is excluded. Append content amber to the second run: counts become (2,3), production yields the opal and pearl amber. A third question mask leaves every run below two content words and yields None. This is an exact three-call intervention relation with finite observed equality, not a general optimization or semantic superiority claim. Test fallback_content_objective_and_earliest_tie.

## candidate-expansion-and-retry

Fix the eight-token try/take/ruby/give/jade/seek/pearl/they tree in verb_children_expand_one_xcomp_level_only. Helper output is [take,give,ruby,seek,they]: direct children precede exactly one level of direct Xcomp children; Conj grandchildren and deeper Xcomp grandchildren are excluded, while an independently supplied Xcomp subject is retained. Obj selection observes ruby with obj label. Separately two three-token fixtures differ only in Obj/Iobj versus Obl/Obl relations; with ruby echoed, direct selection returns None while the oblique retry loop returns jade/obl. These observations constrain parent wiring, order and cleanup failure propagation; they do not infer general branch reachability. Oracle arrays and optional text/rule outputs are literal and independent of production selection loops. Tests verb_children_expand_one_xcomp_level_only and object_shadowing_and_oblique_retry_are_distinct.

## History

### 2026-10-29 · epoch-memory

2026-10-29: initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

### 2026-10-29 · evidence

2026-10-29, source baseline 4279da541. Seven nodes: root 12 typed models, six fragments with four each, total 36. Extraction owns Why and clauses as nested refinements; other root children cover routing, predicate selection and normalization. All six fragments have syntax node IDs and exact enclosing byte ranges. Root syntax projection has 3708 nodes, source 30078 bytes, SHA-256 a7db75bd3d52ee90eee6f562d4cef3fccbf7407bbf2531ff5e6377ad41c51b0c.

### 2026-10-30 · document

#### native-epoch-2026-10-30

Prediction before native probes: exact cleanup threshold permits half echoed
content; ties preserve earlier candidates; direct cleanup rejection shadows
later alternatives while final oblique scanning retries. All six synthetic
native tests passed the first debug observation. The complete bounded
preimage checks are documented in normalization.groupant/model.md#wave6 and
predicate.groupant/model.md#wave6; root model.md#wave6 retains intervention and wiring
observations. Current source corrects the helper comment recorded as a
mismatch in the earlier epoch; behavior remains unchanged. The earlier
standalone-link limitation is replaced for these native checks by a fresh
Cargo build of this checkout. General parser correctness, other Wants,
malformed-tree termination and all caller wiring remain unmeasured.
See model.md#evidence for final debug/release repeat and negative-control records.

### 2026-10-30 · evidence

#### Native epoch, 2026-10-30

Baseline 48f537bb250b47e571aaf2758a39c82ab6bddadd. The historical standalone
probe above is retained; the new evidence compiles the actual world library
and its path dependencies with Cargo. Six native tests in span::span_models
construct annotated Tree values directly, without parser assets or teacher
calls. Production behavior is unchanged. The verb_kids comment now accurately
states its one-level Xcomp expansion, lack of Conj expansion, and lack of
subject checking.

Five specialized models are added in model.md#wave6 at root, normalization and
predicate scopes. The original behavior_complement and reverse_reconstruction
entries remain first. Total: seven groupant nodes, six scoped fragments,
41 typed models (previously 36). Two complete bounded domains are measured:
341 index lists for each of four echo masks (1364 clean calls), and all
64 rank triples over VERB/ADJ/NOUN/PROPN. Tests compare entire actual and
independent oracle preimage maps, including None. Marker asymmetry, unequal
token-length ties, content-count intervention, branch shadowing versus retry,
and one-level child expansion use separate literal fixtures.

Regeneration is two-stage: regenerate source syntax, run build_models.rs.txt,
then run wave6_models.rs.txt. The latter adds the new declarations and precise
models; do not run it repeatedly on an already extended manifest. Both helpers
are authoring artifacts, not production modules. The syntax projection and
all six fragment source bindings are refreshed to the current source. Partial
syntax still omits macro expansion, including assertion bodies, and does not
establish semantics.

The initial groupants structural check reported exactly two stale ancestor
bindings (inference-core and mova) because this task owns only the source and
its adjacent sidecar. Ancestor refresh belongs to integration. No leaf
diagnostic was reported. A groupant inventory is not a correctness proof.
Focused Cargo checks emit existing world/dependency warnings; this work makes
no whole-world lint or full-suite claim.

Final focused validation: debug2/debug3 and release1/release2 each passed all
six tests. A Rust validation helper removed build chatter and compared the six
named outcome lines; all four normalized outcomes matched exactly. Native
profiles are CPU-only, no optional GPU features. The threshold mutation
changed the production comparison from echo*2 > content.len() to >=.
Cargo rebuilt it; cleanup_complete_bounded_fibers failed its equality
assertion for mask=1, list=[2,1], returning None instead of Some([1,2]).
This is a compiled production mutation control, supplementing the earlier
false-formula control. The source was restored and all six tests passed again.
The restored syntax projection matched the retained projection byte-for-byte.
The changed-file gate reports world/src/span.rs and missing=[].

Run artifact directory is named octopus-span-wave6-2026-10-30 under the
operator's external runs root. It holds debug1 (initial), debug2/debug3,
release1/release2, normalized results, mutation.log, validation.log,
restored.log, repeated syntax projections and changed.json. No performance
gain or broader language accuracy is inferred from these bounded observations.

Evidence SHA-256 bindings for this native epoch:

| Artifact | SHA-256 |
| --- | --- |
| span.rs | 4d215ccd57884f41e77168839aac2710913efc4b233aa43ebdb2de36c7fcaa2c |
| syntax.json | 0534f29ea3a624c338e297eda8b0c3d0f0e43ba86ef7bc69705540f96bcfbe49 |
| debug2.log | fd05ac54b291712cf6221fe715165144a977b2667042e982c2d38a674dcac6e5 |
| debug3.log | 174bec41ad5869d7b5758d3983b6772dbe6536c4ac482b891b16846afabd5778 |
| release1.log | 7e05e81eb8116ff01b094b96be475bcc754baa9e8414a30885202e65c0445e51 |
| release2.log | 5edd7fa3e25f6624ca4d5caea08ff8e4f4a9f23fe031d94f04e6e72883a29902 |
| mutation.log | 3c15810aad554dffb7550bce1b33cc47a9506fdfe9cff536c9cc14e45885f18e |
| validation.log | c96ef0fd5fe258c834b6dc83c1020c0d2ff9a768f1e8c5aa26a5e97c69e5d5bd |
| restored.log | fd378ccad17e36ecdd69502523a1afd07ec73eede0c920445ba2321f9010a005 |

### 2026-10-30 · document

Heading `# Native selection observations, 2026-10-30` stays in document as `# Native selection observations`.
