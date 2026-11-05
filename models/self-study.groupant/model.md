# self-study

## catalog

# Typed self-study interaction obernants

## absent-joined-success

Type: behavior_complement. Status: partial_inner_bound_specification.

Domain and observation scope: successful outer-step traces.

Relation/operator and approximation: Universe U is serial outer-step traces over parsed packets, local artifacts, two learners and two assessments. R is actual successful finish behavior; N is the subset with a failed binding or provenance gate yet an accepted candidate receipt. Operator N approximates U minus R as a source-grounded candidate inner bound.

Assumptions and limits: Assume trusted execution/files, collision-resistant hashes and the named checks executing on the consumed artifacts. This is not obtained by blindly complementing the forward approximation.

Check/oracle: Inspect finish_improve, verify_outputs and AssessmentContract::bind; mutate one bound identity in a prospective completion fixture and require rejection. Existing assessment binding tests exercise part of the boundary.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## compatible-cycle-antecedents

Type: reverse_reconstruction. Status: partial_relational_specification.

Domain and observation scope: a candidate receipt and its artifact graph.

Relation/operator and approximation: For observed receipt y, K(y)={x | outer(x)=y} is a converse relation over compatible baseline/candidate states, old/new assessments, reviews and run artifacts. Reconstruct constraints from receipt digests, four bindings and parent identities; never select one history from scores alone.

Assumptions and limits: All required files must remain available for a concrete witness. Hashes constrain bytes but are not an executable inverse. Unrecorded environment and parser state yield multiple histories.

Check/oracle: Recheck both assessment contracts and all four grades against a retained candidate graph; any supplied witness must forward-produce the observed deterministic report fields.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## same-question-fibers

Type: assessment_alignment. Status: partial_relational_specification.

Domain and observation scope: new and old before/after grade pairs.

Relation/operator and approximation: Partition grades into fibers by (assessment_sha256,total). Allowed comparison pairs remain inside a fiber; forbidden joins cross fibers even when their totals match. Rpair(g,h) iff id(g)=id(h) and total(g)=total(h).

Assumptions and limits: Assessment identity projects actual scored inputs; no collision and row validation assumed. Different old/new fibers are allowed.

Check/oracle: assessment::same_count_and_model_cannot_bind_a_different_assessment and comparison_matches_lean_finite_table_and_rejects_mixed_pairs exist; prospective group oracle swaps only candidate-before examination material.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## learner-pair-square

Type: learner_alignment. Status: partial_relational_specification.

Domain and observation scope: four grade corners.

Relation/operator and approximation: Let square=(new_before,new_after,old_before,old_after). J(square) iff model(new_before)=model(old_before) and model(new_after)=model(old_after). Compare is defined only when J and the assessment-fiber joins both hold.

Assumptions and limits: Model fingerprints must correspond to the loaded state. Individually valid BoundGrades do not imply J. The two model identities may coincide without violating the join.

Check/oracle: Inspect assessment::compare learner-pair ensure; prospective negative control supplies three learners across four otherwise valid grades and expects rejection.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## candidate-parent-lineage

Type: state_lineage. Status: partial_relational_specification.

Domain and observation scope: baseline study to candidate study.

Relation/operator and approximation: Rlineage(S0,summary1) requires summary1.prior_state_sha256=summary0.state_sha256 and base_model_used=true, plus candidate receipt parent packet/review hashes equal the staged next review. This joins state ancestry and curriculum ancestry.

Assumptions and limits: Summary integrity depends on output receipt hashes and trusted writers; the fields alone do not prove every internal training step.

Check/oracle: Inspect finish_improve before cross-exam creation. Prospective oracle substitutes a correctly hashed candidate trained from a different baseline and expects lineage rejection.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## frozen-source-union-preimage

Type: reverse_reconstruction. Status: partial_relational_specification.

Domain and observation scope: multi-epoch source exclusion.

Relation/operator and approximation: For observed Fnext and current review assessment-source set A, compatible previous sets are {Fprev | Fprev union A=Fnext}. Thus A subset Fnext is necessary and Fnext minus A subset Fprev subset Fnext. Union loses first-introduction attribution.

Assumptions and limits: Set semantics use exact source IDs in one namespace; aliases or renamed source IDs are outside the contract.

## exercise-heldout-boundary

Type: data_separation. Status: partial_relational_specification.

Domain and observation scope: next review and context proposals.

Relation/operator and approximation: Rsep requires sources(new_exercises) intersection F=empty and sources(cloze_proposals) intersection F=empty, while context feedback matches only parent exercise rows. Forbidden transition: an old assessment item becomes feedback or a frozen ID becomes a new exercise.

Assumptions and limits: Source-ID disjointness does not imply semantic or document disjointness. validate emits every packet source as explanation text, so this model makes no claim that frozen source passages are absent from teaching.

Check/oracle: Existing next_review_cannot_train_on_parent_assessment_sources, exercise_feedback_cannot_be_replaced_with_an_assessment_item and clozes_omit_frozen_sources_and_have_literal_role_answers are direct scoped oracles.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## training-exam-channel-split

Type: information_flow. Status: partial_relational_specification.

Domain and observation scope: lesson splitting to train and exam.

Relation/operator and approximation: Split(L)=(T,A) projects header plus explain/fact/exercise lines to T and header plus test/exam lines to A. Train accepts only no-test/no-exam inputs; Exam accepts only no-explain/no-fact/no-exercise inputs. Composition excludes labels passing through the training argument channel.

Assumptions and limits: Labels and answer options occur in A for grading; claims concern argument-channel separation, not semantic independence of all text. Parser behavior and trusted callers remain assumptions.

Check/oracle: Inspect split_lesson, Retained::train, Retained::exam and study_with_base. Prospective oracle adds an exam field to T or explanation to A and expects the respective rejection.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## passive-assessment-projection

Type: state_projection. Status: partial_relational_specification.

Domain and observation scope: checkpoint restore, exam and binding.

Relation/operator and approximation: Let pi(S) be Retained::fingerprint. Rpassive(S,A,Sprime,g) requires pi(Sprime)=pi(S), g.lesson_reads=0 and g.exercise_updates=0. study-exam loads state in a fresh process; bind requires the declared zero counters before grade composition.

Assumptions and limits: Fingerprint equality covers serialized learned state, not all mutable caches or OS effects. Honest counter production and trusted executable assumed.

Check/oracle: Inspect Retained::exam before/after fingerprint and study subprocess arguments; prospective oracle checks saved checkpoint identity before and after an isolated exam.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## teacher-output-chain

Type: provenance_authenticity. Status: partial_relational_specification.

Domain and observation scope: Codex event stream through staged review.

Relation/operator and approximation: Rauth links a successful tool-free decoded Codex turn to provider=codex/is_error=false result bytes, then decoded_review bytes, then reviewed_bytes equality, packet digest validation and stage output hashes. A repair additionally composes a bound patch with an authenticated parent.

Assumptions and limits: Local metadata checks are not digital signatures; a party controlling all files can forge a coherent chain. Transport authentication assumes configured trusted Codex executable and ChatGPT login; semantic truth is separate.

Check/oracle: Existing incomplete_failed_and_tool_turns_are_rejected and restaging_a_repaired_review_preserves_its_authentication_chain are scoped oracles. Inspect reviewed_bytes equality at study/stage entry.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## one-attempt-command-budget

Type: resource_budget. Status: partial_symbolic_model.

Domain and observation scope: one improve versus recovery invocation.

Relation/operator and approximation: AttemptCount(improve)<=1 for the new review directory; review rejects existing teacher-reply or teacher-failure markers. AttemptCount(study)=AttemptCount(prepare-next)=AttemptCount(finish-improve)=0 by their call paths. Successful outer report teacher_calls=1 is a declared path count, not measured billing.

Assumptions and limits: Serial execution and trusted directory ownership assumed. Separate explicit refine invocations are outside this one-step budget. Transport has no automatic retry.

Check/oracle: Inspect main dispatch, improve_with_base, review lock/markers and teacher::ask. Prospective oracle uses a failing stub transport and checks no second invocation on the same directory.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## recovery-prefix-language

Type: failure_prefix. Status: partial_trace_specification.

Domain and observation scope: serial interrupted outer-step artifacts.

Relation/operator and approximation: Let t be a successful operation trace. Model a failed artifact state as completed operations from pref(t), plus failure-specific artifacts and possible partial writes. teacher-failure.json and validation-rejected.json can occur only on failure paths; a failed artifact-name sequence need not be a prefix of a successful one. finish-improve resumes only from sufficiently complete verified study/review artifacts, creates missing learned/cross-exam stages, and revalidates existing ones.

Assumptions and limits: This is a partial trace approximation under ordinary serial I/O, not an exact language of all failure artifacts; crashes can leave partially written individual files. A learned directory without its receipt is not automatically repaired.

Check/oracle: Inspect existence branches and verify_outputs calls in finish_improve. Prospective oracle stops after each durable artifact boundary and classifies success, safe rejection or unfinished output without claiming every prefix resumes.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## write-once-artifact-boundary

Type: artifact_monotonicity. Status: partial_relational_specification.

Domain and observation scope: run files and stage directories.

Relation/operator and approximation: Successful writes extend the artifact-name domain with create_new; existing names reject replacement. stage writes staged.tmp then renames, and finish accepts an existing cycle receipt only when JSON values equal the newly derived report.

Assumptions and limits: Name-domain growth is not atomic transactional durability. Locks and temporary call directories can be removed; the relation applies to designated output artifacts only, with ordinary local filesystem semantics.

Check/oracle: Inspect write_new, Retained::save, stage and finish receipt equality. Prospective oracle attempts a second differing write and verifies original bytes survive.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## retained-material-reuse

Type: reuse_equivalence. Status: partial_relational_specification.

Domain and observation scope: teaching content joined to lesson ledger.

Relation/operator and approximation: For course-compatible teaching l, lookup returns n when ledger[n]=material_digest(l,n). Study reassigns both teaching and exam lesson numbers to n and skips train; otherwise assigns next_lesson_number and trains once. Reuse is equivalence under the teaching digest projection, not filename equality.

Assumptions and limits: Hash collisions excluded. Exam seed depends on assigned lesson number, so synchronizing both files is part of the group relation. Reuse does not imply no exam or feedback cost.

Check/oracle: The fresh-process groupant_retention probe loads a synthetic checkpoint, reuses its teaching digest despite changed placeholder number/title/assessment metadata, rejects changed teaching or course, preserves fingerprint and resolves the next lesson number. The full study_with_base path with synchronized teaching/exam numbering remains source-inspected.

Evidence status: bounded executable correspondence for load/lookup/save in a separate process, with no call to train or the parser. This does not demonstrate previously learned answer quality or execute a school course.

## warm-finish-cost

Type: reuse_cost. Status: partial_symbolic_model.

Domain and observation scope: cold versus cached completion.

Relation/operator and approximation: Cfinish=V+I(no learned)*StudyCandidate+I(no regression)*E(S1,Aold)+I(no candidate-before)*E(S0,Anew)+Bind4+Receipt, where V is artifact reading/hashing. Existing results remove subprocess terms only after their bindings are accepted.

Assumptions and limits: Symbolic costs assume successful stages and serial execution; no numeric speedup, free validation or optimal cache strategy is claimed.

Check/oracle: Inspect all three existence branches in finish_improve. Prospective oracle records subprocess invocations on cold and warm valid graphs and checks absent terms while tampered caches reject.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## bounded-output-unbounded-work

Type: resource_budget. Status: partial_symbolic_model.

Domain and observation scope: packet, proposal and teacher stages.

Relation/operator and approximation: Prepare accepts 8<=sentence_limit<=64; proposal output count<=16; validated review dictionary edits<=6 and question counts=(12,8,4); prompt bytes obey configured positive limit. These independent bounds compose a bounded output shape, not a constant runtime or token bill.

Assumptions and limits: Pool scanning, retained-state size, dictionary size and inference work remain variable. Teacher timeout/output limits bound intended transport behavior under process cleanup assumptions.

Check/oracle: Inspect prepare, propose_questions, validate, prompt_budget and teacher::ask. Prospective controls exceed each exact boundary independently and expect rejection or truncation appropriate to that stage.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## current-dictionary-authority

Type: snapshot_precedence. Status: partial_relational_specification.

Domain and observation scope: old feedback joined to refreshed packet.

Relation/operator and approximation: Prepare-next maps dictionary bytes D to known_record fields and dictionary_sha256 before binding the new packet. Validate requires old meaning/example substrings in changed entries. Priority metadata may retain earlier known flags, while refreshed packet records govern preservation.

Assumptions and limits: Substring preservation is weaker than semantic preservation; repeated or contradictory text may still pass. The current dictionary source must be the intended snapshot.

Check/oracle: Existing dictionary_refresh_recognizes_published_words_and_protects_old_meanings checks refresh and rejection. Inspect prepare_next prompt instruction distinguishing earlier priorities from current records.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## identity-not-counts

Type: information_loss. Status: partial_relational_specification.

Domain and observation scope: score and artifact projections.

Relation/operator and approximation: Let count(g)=(right,total). Its preimage contains many learner/assessment/row triples. count(g)=count(h) implies neither id(g)=id(h) nor equivalent answers. Reconstructing a grade requires identity and row constraints in addition to totals; summaries are checked against bound grades.

Assumptions and limits: Non-injective count projection is structural, not a probabilistic collision claim. Assessment hash identity still requires its own trust assumptions.

Check/oracle: Existing same_count_and_model_cannot_bind_a_different_assessment is the minimal witness; inspect row zip and aggregate recomputation in bind and summary cross-checks in finish.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## promotion-policy-counterfactual

Type: counterfactual_policy. Status: hypothesis.

Domain and observation scope: four already-bound grade scores.

Relation/operator and approximation: Current P=(new_after-new_before>0) and (old_after>=old_before). Define hypothetical Pdelta=(gain>=delta) and (old_after>=old_before), delta>0; for delta=1 on integer scores it equals P, while delta>1 narrows acceptance. No source policy change is proposed.

Assumptions and limits: All identity/binding gates are held fixed; thresholds cannot repair invalid comparisons. This is a counterfactual rule family, not an inverse or a measured utility claim.

Check/oracle: Existing finite-table comparison test checks current Boolean policy. Prospective finite enumeration compares Pdelta to P and verifies implication for delta>=1.

Evidence status: source-grounded partial specification; changed-policy family is hypothetical; no new runtime evidence claimed.

## recommendation-publication-gap

Type: scope_boundary. Status: partial_relational_specification.

Domain and observation scope: evaluated candidate receipt.

Relation/operator and approximation: Rscope maps valid comparison to an evaluated_candidate receipt with published=false, irrespective of recommend. The forbidden inference is recommend=true => canonical artifacts changed or held-out quality improved. Stage remains a draft with complete=false.

Assumptions and limits: Only the modeled command paths are in scope; separate publication workflows are not reconstructed. Adaptive training-source diagnostics are not independent evaluation.

Check/oracle: Inspect finish report literal scope/published and stage receipt flags. Prospective oracle compares canonical artifact hashes before and after a zero-teacher fixture completion; not executed here.

Evidence status: source-grounded partial specification; no new runtime evidence claimed.

## single-turn-artifact-join

Type temporal_artifact_join. Domain: one decoded Codex JSONL turn and the synthetic local normal-review staging path. At most one accepted completion follows a nonblank completed answer; recognized turn/item events after completion reject. Neutral unknown events can remain. The selected answer text must equal review.json bytes before staging. Semantic JSON equality alone is insufficient. A successful local status result retains exact packet/prompt/review/output bindings and published=false.

Preimages include many neutral/reasoning events and metadata values with the same answer/accounting projection. Neither ordering nor a provider field establishes remote identity. The four-test interface probe uses test-owned glue, self_study=None and no teacher process. It supplies partial child-wiring evidence; it does not execute the whole study dispatcher or establish crash atomicity. The legacy decoder binary fails the selected temporal test, and the repaired implementation passes.

## scored-question-projection

Type assessment_channel_join. Domain: the synthetic valid staged lesson, retention::split_lesson and retention::parse_material, followed by AssessmentContract::new/bind/compare. Training retains 12 exercises without test/exam fields; question-only assessment retains 8 tests and 4 exams without teaching/exercises. Bound rows share the assessment projection and matching baseline/candidate labels. Same-count foreign questions, mixed candidate labels and reported updates reject before a comparison can authorize a recommendation.

The probe constructs reports and reuses one assessment on both comparison axes; it tests interface constraints, not trained answers or held-out quality. Structurally valid re-tagged metadata can still bind, because report authenticity is trusted. Score-output preimages retain multiple row choices and histories. Recommendation is a count policy, not publication or statistical evidence. Child tests separately exhaust score-output fibers and four-corner identity variations.

## checkpoint-canonical-fixed-point

Type: canonical_restore_retraction. Status: bounded_source_correspondence.

Domain and observation scope: accepted typed checkpoint payloads p; projection P of retained state to payload, and restoration D that rebuilds indices. C=P composed with D canonicalizes vector-backed maps. Exact scope excludes envelope formatting and nonpersisted focus.

Relation/operator and approximation: P(D(p))=C(p), and C(C(p))=C(p). A state matching an accepted noncanonical payload satisfies P(s)=C(p), not necessarily P(s)=p. Different accepted vector orders can therefore share a semantic fingerprint while their wire hashes differ. Direct save emits the canonical representative; its raw preimage for a noncanonical payload is empty.

Assumptions and limits: Accepted schema, validated indices, ordinary deterministic serialization and collision-resistant digests. A checksum records byte integrity under the decoder's typed serialization; it does not authenticate a teaching history. Rehashing a valid changed weight can yield an accepted different state. The law does not equate answers across engine/parser revisions.

Check/oracle: Native retention permutation and restoration fixtures, plus groupant_retention in a fresh child process, compare fingerprints and canonical re-saves. Unrehashed weight mutation rejects; the same well-formed mutation with a recomputed hash is accepted and changes the fingerprint.

Evidence status: bounded executable fixtures and source inspection; no universal Rust or Lean equivalence proof.

## school-support-boundaries

Type: assessment_support_composition.

Sampling, option validation, supplied-tree matching and rule induction retain distinct inputs, label namespaces, artifacts and effect boundaries. The school-support child is a collection of actual support relations, not a claim that all four files form one pipeline. Native joins exercise canonical option handling and knowledge difficulty strata with fixed supplied lessons. They do not rerun training, establish learned retention or validate course execution. Reverse fibers retain omitted lessons, parser identity, candidate history and any external state required by the selected branch.

## tests

### Evidence and checks

Source inspection covered the six registered descendants, particularly:

- assessment.rs: AssessmentContract::new/bind and compare; exact identities,
  passive counters, rows/options, score totals and four-grade joins.
- study.rs: frozen, validate_context, study_with_base, prepare_next and
  finish_improve; immutable artifact references and cross-assessment wiring.
- retention.rs: retained_lesson, train, exam and split_lesson; reuse, physical
  channel split and learned-state passivity.
- enrich lib.rs: validate, verified_packet, reviewed_bytes, review, stage and
  status; source exclusion, dictionary preservation and teacher/output chain.
- teacher lib.rs: decode_events, command and ask; one attempt, no unexpected
  tools, completed turn accounting and bounded intended transport.
- enrich main.rs: explicit command dispatch; recovery has no teacher call.

Named tests in model.md#catalog are existing source evidence, not newly passed
checks. Their scope does not establish complete group correctness. The finite
comparison table checks score policy separately from source-bound artifact
wiring. A future integration probe should include a valid completion and a
same-count foreign candidate-before assessment, a mixed learner square,
changed context, stale dictionary record, incomplete learned directory and
repeated finish; each mutation should reject at its intended gate.

Structural validation for this change checks parsed manifest JSON, exact
immediate registry children, seven registered descendant source digests, required
obernant order, unique twenty IDs with catalog headings, and existing component
references. The repository changed-source check and integrated groupant checker
are separate parent integration checks; neither proves semantic claims.

## core-model

### Self-study interaction model

#### Forward approximant

Let a state be (P,V,F,D,S0,S1,Aold,Anew,G00,G01,G10,G11,files).
P is a source packet, V a reviewed lesson, F frozen source IDs, D the dictionary
snapshot, S0/S1 retained learner states, Aold/Anew scored assessment projections.
Gij is a grade for learner i on assessment j; artifacts include hashes and
receipts. Child contracts are joined on shared identities, never merely counts.

A successful outer trace is:
validated staged parent -> split teaching/exam -> exercise-before feedback ->
reuse or train baseline -> exercise-after feedback -> save S0 and context ->
isolated old exam -> prepare next packet -> one Codex review -> authenticated
validation/stage -> candidate exercise-before feedback -> learn S1 from S0 ->
candidate exercise-after feedback -> save S1 and context -> isolated new exam ->
old exam on S1 -> new exam on S0 -> bind four grades -> compare ->
write or verify candidate receipt.
Failures can leave completed-operation prefixes, failure-specific artifacts
(such as teacher-failure.json or validation-rejected.json), and partial writes.
They are not necessarily prefixes of a successful artifact-name sequence.

Write B(g,S,A) for AssessmentContract::bind success, including exact row and
aggregate consistency and zero learning counters. The comparison domain requires
B(G00,S0,Aold), B(G01,S0,Anew), B(G10,S1,Aold), B(G11,S1,Anew).
The policy is recommend iff right(G11)>right(G01) and right(G10)>=right(G00).
The old and new assessments need not have equal sizes; each before/after pair
must have the same assessment and total. This gate is a candidate diagnostic,
not publication, causality, or a statistical generalization theorem.

#### Interaction invariants

- Receipt output-key sets and file hashes bind the artifacts consumed by later
  stages. Hash collision resistance and a trusted filesystem/executable are
  assumptions; editable JSON alone is not a cryptographic attestation.
- Candidate summary prior-state identity equals baseline state identity and
  base_model_used is true. Four grades cross-check the same two learners.
- Fnext = Fparent union sources of non-exercise questions in the current review.
  New exercise source IDs avoid F. This is a source-ID boundary; staging still
  includes source passages in teaching explanations.
- Training receives no test/exam fields; exam receives no teaching/exercises.
  A fresh executable loads a checkpoint and question-only file. The learned
  fingerprint is unchanged across exam/feedback; parser caches are outside
  this projection.
- Current dictionary records replace stale packet records before the next
  review. Existing meanings/examples must remain substrings in validated edits.
- write_new/create_new and new run directories reject overwrites. They are
  not multi-file transactions. Interrupted directories may prevent completion.
- recommend=true still leaves published=false. Study and finish do not promote
  canonical dictionary/curriculum artifacts.

#### Cost approximant

Let H be bytes hashed, L retained lessons, T teaching bytes, Q scored questions,
P proposal-pool size, E(S,A) exam cost, and R one remote review cost. A cold outer
step costs baseline study + preparation + R + candidate study +
E(S1,Aold) + E(S0,Anew), plus receipt validation/serialization O(H).
Each study includes retained lookup O(L*T), optional training, feedback,
checkpoint I/O and its own isolated exam. Proposal filtering repeatedly scans
pools; no measured asymptotic improvement is claimed. A warm finish omits an
existing learned study and existing cross-exam subprocesses but still hashes
and binds their artifacts. A count cap on outputs is not a bound on all work.
Remote timeout bounds intended waiting, subject to process cleanup assumptions.

#### Descriptant and epoch memory

The useful prior registry observation is that same-size foreign assessments
can pass count-only checks; exact assessment and row binding must remain in
the group join. Four individually valid grades can still be miswired, so the
learner-pair equality gate is independently necessary. Prior leaf models
separate source identity, passivity and policy; their conjunction supplies a
useful group approximation, not a proof that every I/O path is correct.

This epoch adds specialized models in model.md#catalog. It preserves ambiguity in
reverse queries: totals do not recover questions, saved fingerprints do not
recover unique histories, and frozen unions do not identify which ancestor
introduced a source. No new runtime, teacher, quality or performance result
is claimed. General source namespaces, semantic teacher quality, concurrency,
crash durability, parser correctness and engine-version identity remain gaps.

#### School runtime scope

The school-runtime child now registers the existing parser, answer/update helpers
and legacy course loops in world/src/knowledge.rs. Those course loops can reread
lessons; their scores are not automatically persistent fresh-process exams.
The retained-student boundary still owns question-only checkpoint assessment.
The successful retained outer trace above is one scoped path, not a claim that
every registered school command enforces that protocol. Shared dependencies and
nested decision/witness scopes must not be counted twice in cost models.

## History

### 2026-10-29 · document

All entries share subject/revision: self-study and the exact eight source bindings in manifest.json (inspection 2026-10-29). Behavior denotes one serial invocation or the explicitly stated multi-epoch set relation, not unrestricted reachability. Operators are partial models of cross-child wiring. Evidence below is source inspection and named existing test definitions; executed checks are distinguished in model.md#tests. Suggested oracles remain prospective unless explicitly recorded below. No model is a proof of Rust correspondence, a performance measurement or an executable reverse solver.

### 2026-10-29 · frozen-source-union-preimage

Check/oracle: Inspect frozen and validate_context equality. frozen_union_preimage_matches_all_small_source_sets now enumerates 64 actual forward cases and 512 inverse memberships over three IDs, including duplicate/reordered raw inputs and ignored exercise IDs; executed 2026-10-29.

Evidence status: bounded Rust correspondence plus a separate general set-relation proof in formal/FrozenUnion.lean (MovaFrozenUnion.union_preimage, checked 2026-10-29). The Lean result does not prove Rust execution or performance.

### 2026-10-29 · tests

Inspection date: 2026-10-29. This change supplies models only. No teacher calls,
training, assessment quality measurements, Rust test reruns or Lean proofs were
performed for these new group models. Catalog oracles explicitly distinguish
existing test definitions from prospective integration checks.

2026-10-29 integration: the 23-test enrichment library suite passed, including
64 frozen-set forward cases and 512 inverse memberships in the actual frozen
function. This supplies bounded correspondence for frozen-source-union-preimage.
The group itself still has no complete end-to-end reverse executor or proof.

### 2026-10-30 · tests

#### Executed interface correspondence, 2026-10-30

Four tests in tools/enrich/tests/groupant_pipeline.rs pass debug/release and repeat with identical named outcomes. They call real decode/stage/status/split/parse_material/bind/compare APIs on local synthetic artifacts. The pre-fix decoder binary fails exactly the temporal rejection test with exit 101; the repaired decoder passes. No teacher call, training, retained learner or real assessment corpus is used. Test adapter write order differs from production, and self_study=None, synthetic reports and repeated assessment axes limit the claim.

The enrichment library passes 28 unit tests; its 11 assessment checks include 1566 complete-output score fibers, four-corner identity and renaming/trust tests. A compiled old-score tie mutation is rejected. Teacher passes seven unit tests including 271453 finite traces and projection/accounting fibers; a compiled legacy-decoder mutant is rejected. Both profiles pass. The live teacher smoke remains ignored. New source syntax, all nine child bindings and root bindings are refreshed; structural checks do not establish full workflow correctness.

#### Checkpoint projection and process boundary, 2026-10-30

Sixteen native retention tests and two fresh-process integration tests pass in debug/release; the ignored test is an explicitly invoked subprocess helper, not an unexecuted parent assertion. Five child processes per integration suite cover repeated canonical restores, successor/reuse lookup, write-once output and checksum rejection versus valid resealing. Independent field-order encoding is limited to the synthetic empty-frame fixture. Native fixtures separately cover nonempty indices, provenance and validation gates. A compiled quote-owner production mutant is rejected; a separate false-successor oracle control also fails its intended assertion.

The ten registered source children now include fresh-process-retention-probe. Reuse and canonical fixed-point evidence does not execute study_with_base, train, parser, teacher or exam. Checksum consistency does not authenticate learning; restore passivity does not prove answer quality. External evidence: $MOVA_DATA/runs/octopus-groupants-next-2026-10-30/ and the next-retention/next-process worker runs.

### 2026-10-30 · core-model

Subject/revision: the self-study registry node at the nine exact source digests in
manifest.json, inspected 2026-10-30. This is a partial descriptive groupant,
not a verified implementation of all interactions. Selected child syntax projections are available; the group AST and executable
reverse reconstruction are not materialized.

#### Executable interface epoch 2026-10-30

A ninth source supplies four synthetic child-interface tests: completed teacher turn to literal review bytes and staged outputs, split question channels, and identity-bound score comparison. It is not an execution of the complete outer trace: self_study=None, constructed reports and the same assessment in both axes remain explicit. The teacher decoder now enforces one completed turn; 271453 finite event traces test order and output fibers. Assessment tests enumerate 1566 score quadruples and trust-boundary variations. Report authenticity, actual learning and semantic quality remain separate obligations.
