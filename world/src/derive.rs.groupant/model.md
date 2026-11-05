# rule-derivation

## descriptant

# rule-derivation

## Forward

Load or acquire tagged trees, induce per-class decision lists, derive heads, optionally learn ordered reattachment transformations. Store scans compose ordered selection, raw records, lexical/tag decoding, packing, unpacking, decision lists, transformation replay and reports. Full rescoring, incremental learning and easy-first derivation are alternatives whose equivalence is unverified.

## Invariant

Aligned nonempty sentence vectors and in-range heads are admission assumptions. Plain derive picks first finite VERB, else first VERB, else first NOUN/PROPN/ADJ, else index zero, and checks cycles. Empty derive/derive_easy inputs panic. The final plain cycle repair does not regenerate explanation steps. Helper evidence is not a proof of all file behavior.

## Cost

For sentence lengths n_s, class-list length L and transformations T, conservative derivation/replay bound is O(sum_s (L+T+1)*n_s^2). Fixed-target induction is O(sum_s n_s^2). Search, parser acquisition, I/O and state effects add separate costs. No performance gain claimed.

## Composition

Packing supplies aligned S projections to rules; scan recomputes trees from fixed lists and ordered transformations. Selection order determines the every-twentieth training sample. Shared class and finite identities constrain the interfaces; parent complement is not the union of child complements.

## Epoch memory

Initial epoch: 90 assertions on twelve extracted pure source items and minimal dependency enums; negative control erases closed-class lexical information and fails. No full learner, annotator, GPU, corpus, filesystem-training or state-writing execution. The probe retains real helper source but stubs enum definitions; it does not validate real serialization ordinals or dependencies. Models remain partial even where structural checks pass. Prospective oracles remain explicit in catalog sections.

## catalog

# rule-derivation typed model catalog

## nonbehavior

Type: behavior_complement.

U is admitted finite calls plus fixed dependency/schedule states paired with returns/effects. Candidate inner U minus R contains successful plain derivation preferring a later finite verb over an earlier one, open-class class() retaining a lexical lemma, and scan_store invoking the annotator. Class exclusion tested; root/call-graph exclusions source-inspected.

## antecedents

Type: reverse_reconstruction.

For fixed entrypoint, y admits every source tree, list, initial dependency state and schedule x satisfying R(x,y). Equal heads or scores do not uniquely determine lemmas, gold labels, candidate counts or lists. Executed class/unpack collision witnesses demonstrate bounded information loss; full derivation fibers remain prospective.

## wiring

Type: relational_composition.

Store path: exists ids,records,P,C,S,L,Tr such that select(v,q)=ids, ordered raw_tree(v,ids)=records, pack(records,lemmas,tags)=(P,C), unpack(P,C)=S, lists(induce(S[::20]),30,[])=L and replay(L,Tr,S)=observation. Records and lexical tables must share compatible identity. scan_store captures latest then lemma_table(dir) independently reopens latest: concurrent ref advance can mix snapshots. Require fixed refs/table or explicit compatible append-only identity. No end-to-end scan executed.

## entrypoints

Type: dependency_boundary.

load reads CoNLL-U; from_books/gold_with_tags acquire annotator; scan parses books; scan_store reads raw trees. run_learn selects incremental unless DERIVE_FULL_RESCORE is present. Incremental HashSet traversal and candidate/tie search differ from full learn; equivalence is not established. Proposed oracle: tiny tagged trees through both learners with ties/cycles; not executed.

## agreement

Type: observation_projection.

score/agree count labeled hits only for correct heads and divide by max(words,1); empty aggregate gives zero. scan reports parser-head agreement, not held-out gold accuracy. Its fix search skips correct heads, so relation-only errors are not candidate fixes. Equal numerators/denominators share score fibers. Proposed literal head/label oracle remains unexecuted.

## tests

### Derivation bounded evidence

#### Executed scope

The generator extracts twelve complete production items: closed, class, class_with, S, nth, pending, nth_pending, Cond, cond_sets, makes_cycle, Packed and unpack. The harness supplies small UPos/Rel enum definitions with names and lookup arrays. These stubs exercise equality and indexing semantics; they do not establish actual enum byte ordinals, store decode correctness or external dependencies.

Ninety assertions pass: lexical/open-class/marker projection, directional and strict-span target examples, cycle witnesses including an unrelated existing cycle, all six prefix condition families against independently enumerated bit masks and literal counts, packed field projection and colon/dictionary collisions, and forty complement/converse membership assertions over five class inputs and four observations. Literal expected outputs are independent of production helper algorithms. A source mutation replacing lowercase lexical values with empty strings fails the ordinary class expectation. The runner asserts that the mutant process fails; failure is expected evidence, not a passing corrupted implementation.

#### Reproduction

Run from the repository root, with PROBE_OUT naming an external scratch directory. No parser/model/corpus acquisition occurs.

```sh
mkdir -p "$PROBE_OUT"
rustc --edition=2021 --crate-name derive_probe_generator world/src/derive.rs.groupant/generate-probe.rs.txt -o "$PROBE_OUT/generate"
"$PROBE_OUT/generate" "$PROBE_OUT"
```

Expected stdout:

```text
derive pure source probe: 90 assertions passed
closed-class erasure mutant rejected
```

The generator compiles extracted helpers with rustc and captures expected mutant stderr. It requires the twelve named items to retain their brace-balanced form; this extractor is fixture tooling, not a Rust parser. The separate syntax artifact uses the modelcheck syn projection.

#### Structural checks

The root and three children declare twenty typed models, with behavior_complement first and reverse_reconstruction second at every node. Child envelopes select source-bound syntax node IDs; parent has a full-file partial syntax projection. Regenerate with mova-modelcheck syntax and compare bytes. Register registry-entry.json under inference-core when integrating; parent registration is deliberately outside this branch's ownership.

Run mova-modelcheck groupants with a temporary registry containing the new node, and changed against the task base. Structural validation checks declarations/hashes/references; it does not prove formulas, dependency behavior or correspondence outside the extracted helpers.

#### Limits and future oracles

No live learner, annotator, training stage, book corpus, real store scan, GPU, performance benchmark or full Cargo suite ran. Rule ranking, root choice, transform snapshot application, search shortlist and persistence-failure formulas are inspected source relations, with proposed oracles stated per catalog. Incremental/full learner equivalence and parallel scheduling invariance remain unproven. Store hash capture and lemma_table's independent latest lookup require explicit snapshot/lexical-compatibility assumptions. Saved reports/rule strings are not complete resumable checkpoints. Pure probe repetition checks only its own deterministic output.

#### Observed structural result

Two syntax generations compare byte-identical. Two probe executions print the same 90-pass and rejected-mutant summary. changed against 47188b0a2 reports no changed Rust sources and no missing registrations (this branch adds sidecars only).

A temporary full registry links rule-derivation under inference-core. groupants reports no diagnostics in the new file or its three fragments. Its only three diagnostics are expected integration obligations: inference-core immediate children differ, and inference-core plus mova lack the new transitive derive.rs binding. Those ancestor manifests and the authoritative registry belong to the integrating parent; this branch does not edit them. Consequently the standalone whole-registry result is not claimed green. Integration must refresh those bindings and rerun the complete inventory.

## History

### 2026-10-29 · document

Source-derived partial groupant, 2026-10-29. Authoritative source remains derive.rs. Full file inspected; local models specialize the named source envelope. Macro expansion, semantic AST, parser correctness, live training and general proof remain absent.

Subject: derive.rs SHA-256 2726211da9538f3f5c225292219dfda4d4fd6f15d6b68deaaaf53c834ba60842. Models concern one finite transition or an explicitly described finite scan trace, with admitted input bounds and fixed dependency responses. Behavior-complement entries are candidate inner bounds of actual absent behavior, not complements of a tested sample. Reverse entries retain every compatible antecedent in their declared projection, not unique reconstruction. Source review is evidence distinct from executed helper probes; see model.md#tests.

### 2026-10-29 · tests

2026-10-29, derive.rs SHA-256 2726211da9538f3f5c225292219dfda4d4fd6f15d6b68deaaaf53c834ba60842.

Final integration, 2026-10-29: the authoritative registry now includes rule-derivation under inference-core, with refreshed immediate children and transitive source bindings in inference-core and mova. The complete integrated groupant report is valid with no diagnostics. This resolves the earlier branch-local structural obligations without expanding the executed semantic scope.
