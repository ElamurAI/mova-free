# study-orchestrator

## descriptant

### Study orchestrator descriptant

Inputs to this groupant are [study.rs](../study.rs), its future AST,
[artifact validation](../lib.rs), [assessment binding](../assessment.rs),
[retained learning](../../../../world/src/knowledge/retention.rs), and the nearby
forward, invariant, typed-family and evidence files. AST is not materialized.

The forward approximant models guarded state/artifact transitions. The first
obernant models a subset of absent transitions under an explicit call-trace
universe; the second reconstructs compatible antecedents from frozen sets and
receipts. The reuse-cost extension distinguishes avoided work from unavoidable
validation and assessment. The combiner should intersect these views with actual
source and evidence, preserving ambiguity where observations lose information.
Agreement among documents is not independent Rust verification.

Useful epoch observations: the sixteen-cloze cap does not cap raw generation;
lesson reuse still runs two feedback passes and an exam; a successful grade
comparison needs the same questions before and after; existing partial output
paths can block recovery; receipt metadata alone cannot audit teacher calls.
These observations guide cheap focused checks before any training experiment.

Current gaps: generated AST, executable reverse reconstruction, formal trace
proof, exhaustive IO/concurrency coverage and measured cost calibration. The
existing source tests cover local proposal, binding and dictionary boundaries;
they do not prove the full outer workflow or teacher quality. New evidence should
record its precise scope and retain these limitations until it resolves them.

New useful observation: the row-ID filter excludes '-' and '.' but does not
parse integers; integer-only eligibility in the earlier forward description was
too strong. Known-meaning retention is substring preservation, not entailment.
Cache existence controls whether production is skipped, while later binding
controls acceptance. A sixteen-result cap loses candidates without bounding
raw discovery. No new timing, accuracy, crash-recovery or teacher-cost evidence
was collected by this expansion; proposed oracles remain future checks.

## approximant

### Forward transition approximant

Subject: [study.rs](../study.rs), revision and SHA in [manifest](manifest.json).
Domain: one serial invocation with immutable parent artifacts, stable dictionary
snapshot and no concurrent filesystem writer. Model filesystem bytes as F,
retained state as M, frozen IDs as Z, and external process outcomes as E.
Behavior is a finite call trace ending in success or the first propagated error;
it is not unrestricted reachability. Dependency operations remain abstract.

For study_with_base(parent,new,base), the successful path is:

1. Require current staged parent, nonlegacy stage, verified packet, training
   partition and validated review. Split teaching from assessment. Load M from
   base, or create it from the teaching course. Record its fingerprint h0.
2. Query retained_lesson. If present, retain that lesson number; otherwise
   allocate next_lesson_number. Rewrite both material headers to that number.
   Create a fresh directory and write teaching.lesson and examination.lesson.
3. Compute exercise-only feedback B on M. If already retained, let M1=M and
   emit the explicit reuse training report; otherwise M1=train(M,l,false).
   Compute feedback A on M1 and save model.json plus exercise-before.json.
4. Set Z1=Zparent union {source_id(q): phase(q) != exercise}. Build context
   from A, lexical priorities, and cloze candidates excluding Z1. Validate and
   write study-context.json. Z1 is sorted and deduplicated by BTreeSet.
5. Spawn the current executable with study-exam, model and examination paths.
   Write stderr, require child success, require matching learner hash and zero
   lesson reads/updates, and bind its grade through AssessmentContract.
6. Write summary, then study-receipt with hashes of exactly seven named outputs.
   Success observes a studied receipt; earlier failure can leave a partial tree.

prepare_next verifies the parent, receipt, seven output hashes and context;
requires at least three nonfrozen sentences; refreshes dictionary records from
ENRICH_CURRENT_DICTIONARY, then CYCLE_ENRICH_DICTIONARY, then the default path.
It binds the context and parent hashes into a new packet, checks prompt budget,
creates a fresh directory and writes review inputs. It makes no teacher call.

improve_with_base creates a fresh root, executes study, prepare_next, one review
call, stage and finish_improve in sequence. Each error terminates this invocation.
finish_improve verifies baseline outputs and staged review/binding, learns from
the baseline checkpoint only if learned/ is absent, verifies candidate lineage,
and binds both existing exams. It runs missing cross exams: candidate model on
old questions and baseline model on candidate questions. It binds all four
grades, then delegates comparison to [assessment.rs](../assessment.rs):
gain=new_after-new_before; recommend=(gain>0 and old_after>=old_before).
The resulting candidate receipt is created or must equal the existing JSON.
No publication follows this recommendation.

Proposal projection: retain eligible ten-column rows whose ID contains neither
'-' nor '.' (the generator does not itself parse an integer ID); retain NOUN
or PROPN nsubj/obj with a VERB head, unique literal answer occurrence and valid
text boundaries. For each raw candidate, collect canonical-distinct answers of
the same role; require three distractors, deduplicate (source,answer,role), take
the first three ordered alternatives and stop after sixteen outputs. This is
source-text cloze generation, not a semantic truth check of distractors.

The model predicts control flow and artifact binding, not student accuracy,
teacher truth, atomic writes, crash recovery or hash collision resistance.

## composition

# Study fragment composition

## composition (composition)

The file retains its 23 catalog models. Three immediate fragments add local observations: proposal (with pool child), feedback (with priority child), and prepare-finish. These are partial, overlapping behavioral scopes, not a replacement for the inter-file registry.

The study driver obtains exercise feedback F from retained state, feeds F to priority aggregation, computes frozen set B from review plus inherited bindings, and feeds (packet,B) to proposal generation. It then constructs context C from these outputs and validates C before publication. Proposal and feedback are parallel data dependencies of C, not sequential state updates. Parent context admission is stronger than either child: it binds parent hashes, exercise identities, frozen boundaries and lexical metadata together. Retained state fingerprint checks constrain the feedback child; they do not prove all dependency internals.

prepare_next consumes authenticated C and the exact seven-output study receipt, refreshes current dictionary records and budgets its prompt before creating the next directory. External review and staging connect preparation to finish. finish reuses or creates candidate study, verifies lineage, binds all four learner/assessment pairs and compares them. Composition requires stable artifact bytes and dependency semantics; hashes establish identity rather than correctness. Filesystem failures retain prefixes and no rollback is modeled.

If R denotes this coupled relation, its converse keeps every compatible intermediate (F,B,C,artifacts), with no assumption of injectivity. Parent behavior complement is relative to the whole study trace universe; unioning child complements is invalid because children have different inputs and successful local outputs can be wired incorrectly. Unknown dependencies, subprocess faults and complete failure partitions remain gaps.

Costs sum executed stages only. Pool scans are included in proposal cost and priority scans in feedback cost; overlapping nested scopes must not be counted again. Cached finish avoids selected work only after verification, so neither zero cost nor unconditional success follows from file existence.

## epoch-memory

## catalog

# Specialized study obernant catalog

## bound-context-provenance

Type: provenance_relation. Observation: context, parent packet and parent review byte identities.
Status: partial_source_model.
Domain: successful verify_context over fixed artifacts. Relation P(c,p,r,b) holds iff h(c)=b.context_sha256, h(p)=b.parent_packet_sha256, h(r)=b.parent_review_sha256, validate_context succeeds and frozen(c)=frozen(b). Assumptions: stable bytes and trusted binding; SHA equality is consistency, not authentication. Oracle: bound_feedback_survives_restage_and_rejects_tampering; mutate each bound artifact independently. Evidence: verify_context/copy_context source; existing test covers context tampering, not every mutation.

## reservation-capacity

Type: resource_reservation. Observation: remaining nonfrozen sentences before another review.
Status: partial_source_model.
Domain: one prepare_next or a bound study chain. Operator A(P,Z)=[s in P.sentences: id(s) not in Z]; admission requires |A|>=3, while Z'=Z union assessment_ids. With a fixed sentence multiset, |A(P,Z')|<=|A(P,Z)|. Assumptions: counts are sentence entries, not necessarily unique IDs; no replenishment. Oracle: count survivors after freezing all but two/three entries. Evidence: frozen/prepare_next source and next_review_cannot_train_on_parent_assessment_sources; exact capacity boundary has no dedicated test. Reservation alone does not guarantee three usable cloze answers.

## exercise-feedback-projection

Type: information_projection. Observation: exercise identity and omitted assessment rows.
Status: partial_source_model.
Domain: review questions and split lesson passed to exercise_feedback. Operator E(r)=ordered filter(phase=exercise); feedback position i retains (i,source_id,question,answer) from E(r)[i]. validate_context enforces this projection; assessment rows are outside it. Assumptions: validated review and split_lesson contract. Oracle: exercise_feedback_cannot_be_replaced_with_an_assessment_item. Evidence: source plus that existing test. Expected answers are training labels; this projection does not prove arbitrary source text contains no assessment information.

## feedback-state-stability

Type: state_preservation. Observation: fingerprint before and after answering exercises.
Status: partial_source_model.
Domain: a successful exercise_feedback call with Retained state M. Relation F(M,rows,M') requires fingerprint(M')=fingerprint(M), with rows derived by student.answer using lesson/index-seeded shuffling. Assumptions: fingerprint covers relevant retained state and answer indices are valid dependency outputs. Oracle: compare fingerprints around feedback, including a zero-exercise case. Evidence: explicit source guard; no standalone fault-injection test. Hash equality does not prove absence of transient changes or external side effects.

## weak-support-priority

Type: ranking_operator. Observation: lexical ordering from weak exercise support.
Status: partial_source_model.
Domain: packet words and feedback. W={f: !f.correct or !f.expected_has_support}; H(w)=count of f in W whose source is in w.source_ids or whose tokenized question contains w.headword. Operator sort by (-H,known,headword). Assumptions: tokens and headwords use existing representations; metadata from earlier snapshot. Oracle: a correct answer without support still contributes a hit, while understood alone does not enter W; ties order unknown before known. Evidence: priorities source only; proposed boundary examples are unexecuted. validate_context does not recompute H.

## cold-assessment-boundary

Type: isolation_contract. Observation: saved checkpoint and question-only child assessment.
Status: partial_source_model.
Domain: successful study_with_base child examination. Relation Cold(M,Q,g) requires child success, g.state_sha256=fingerprint(M), zero lesson_reads/exercise_updates and AssessmentContract(Q).bind(g,M). Assumptions: child executable, Retained::exam and split_lesson satisfy their dependencies; fresh process does not imply an OS sandbox. Oracle: bounded_inverse_model_requires_every_binding_guard in assessment.rs covers binding mutations. Evidence: subprocess source plus dependency test; no subprocess fault matrix. This contract does not establish factual accuracy.

## seven-output-identity

Type: artifact_identity. Observation: exact named study output closure.
Status: partial_source_model.
Domain: verify_outputs against a receipt. Operator V(R,F)=keys(R.outputs)=K and forall k in K, h(F[k])=R.outputs[k], where K is the seven names in model.md#invariants. Assumptions: immutable filesystem during reads and trusted receipt origin. Oracle: delete a key, add a key, change one file, or edit exam-process.stderr; the last lies outside V. Evidence: verify_outputs source; no dedicated seven-output mutation test. Byte integrity neither authenticates coordinated replacements nor checks every JSON semantic.

## candidate-lineage

Type: lineage_relation. Observation: candidate continuation from the baseline checkpoint.
Status: partial_source_model.
Domain: successful finish_improve with baseline B and candidate C summaries. Relation L(B,C)=C.prior_state_sha256=B.state_sha256 and C.base_model_used=true, plus candidate receipt parent hashes matching staged review status. Assumptions: receipt output verification and honest producer summaries; baseline checkpoint is actually supplied when learned is absent. Oracle: modify prior hash or swap review while keeping other artifacts; expect rejection. Evidence: finish_improve source only. Metadata cannot reconstruct gradients or prove causal credit for score changes.

## failure-prefix-recovery

Type: prefix_reconstruction. Observation: compatible trace prefixes from partial study files.
Status: partial_source_model.
Domain: serial study writes with first-error termination. Define ordered milestones material, checkpoint, feedback-file, context, exam-process, summary, receipt. B(S)={prefixes p of this sequence whose completed writes are consistent with observed files S}; allow a partially written final file. Assumptions: fresh directory, no deletion or concurrent writes, dependency writes abstracted; exam subprocess may leave partial exam output. Oracle: inject an IO failure at each milestone and compare surviving tree. Evidence: source ordering only; crash matrix unexecuted. A directory is compatible with failure and is not proof of success; exact failure causes remain ambiguous.

## cached-finish-admissibility

Type: cache_validity. Observation: existing learned and cross-exam artifacts on finish reentry.
Status: partial_source_model.
Domain: finish_improve reentry. Operator reuse(a)=exists(a) selects skipping production, while accept(a)=reuse(a) and subsequent verification/binding success. Thus exists does not imply accept; an incomplete learned directory blocks recreation. Assumptions: stable artifacts and same baseline/review. Oracle: a complete matching run may reuse; absent receipt inside learned must error; foreign cached grade must fail binding. Evidence: finish_improve source and assessment binding tests for the foreign-grade boundary; end-to-end restart checks unexecuted. Matching final receipt still requires preceding verification.

## retained-lesson-reuse

Type: observational_equivalence. Observation: training-skipped retained material and retained lesson numbering.
Status: partial_source_model.
Domain: study_with_base when retained_lesson returns Some(n). Operator T_reuse(M,l)=M and assigned_number=n; no train call, but both feedback passes, model save and exam still run. Observation is retained fingerprint, not full summary bytes. Assumptions: retained_lesson identifies equivalent material under the retention dependency and fingerprint is stable. Oracle: reuse same material with a saved base and compare prior/final fingerprints and training flags. Evidence: branch source only; no new training experiment. Equivalence to fresh training, and identical wall_seconds, are not claimed.

## dictionary-meaning-extension

Type: semantic_preservation_boundary. Observation: current dictionary records and permitted reviewed extensions.
Status: partial_source_model.
Domain: refresh_dictionary followed by review validation in lib.rs. Operator D'(w)=current_bytes_map[w]; known_record is replaced even when earlier priority metadata differs. For reviewed known w, require old_meaning substring-of new_meaning and old_example substring-of new_example. Assumptions: six-field unique current records and stable dictionary snapshot; substring preservation is syntactic. Oracle: dictionary_refresh_recognizes_published_words_and_protects_old_meanings. Evidence: that existing test and source. Semantic contradiction can survive substring retention; this is not a truth or logical-entailment proof.

## cloze-literal-admissibility

Type: proposal_filter. Observation: eligible token spans before distractor selection.
Status: partial_source_model.
Domain: rows of nonfrozen packet sentences. Operator C keeps ten-column noncomment rows with no '-' or '.' in ID, NOUN/PROPN subject/object, matching VERB head, single literal occurrence and nonalphanumeric span boundaries. Observation: source ID, role, predicate lemma and sentence-with-gap. Assumptions: valid text encoding and existing single predicate; the filter does not parse IDs as integers. Oracle: clozes_omit_frozen_sources_and_have_literal_role_answers. Evidence: generator source and existing test; all malformed row cases untested. No semantic disambiguation is inferred from a unique spelling.

## distractor-canonical-quotient

Type: equivalence_quotient. Observation: same-role answer alternatives and deterministic representative choice.
Status: partial_source_model.
Domain: raw candidates C and target answer a with role r. Define x~y iff canonical(x)=canonical(y); pool is classes of answers with role r excluding [a]. Require >=3 classes, order by canonical key, take first three; BTreeMap collection keeps the last raw spelling for each key. Assumptions: fixed raw iteration and canonical function. Oracle: a_missing_or_identical_distractor_pool_does_not_invent_choices; case variants should occupy one class. Evidence: source and existing insufficient-pool test; representative ordering boundary untested. Distinct strings do not prove semantically wrong alternatives.

## proposal-quota-loss

Type: truncation_preimage. Observation: information discarded by the sixteen-result cap.
Status: partial_source_model.
Domain: raw candidates after admissibility/distractor checks. Let D deduplicate by (source,exact_answer,role) in traversal order; output Q=take_16(D(C)). Preimage B(Q)={C:take_16(D(C))=Q}. Assumptions: fixed ordering and fixed distractor pools; appending raw candidates may change pools, so suffix invariance needs that extra condition. Oracle: construct >16 accepted candidates with stable pools, observe length 16 and unrecoverable tail. Evidence: propose_questions source only; cap experiment unexecuted. Quota bounds output storage, not discovery work or semantic coverage.

## prompt-byte-admission

Type: budget_contract. Observation: prepared next-review prompt admission before directory creation.
Status: partial_source_model.
Domain: prepare_next serialized prompt p and configured ENRICH_PROMPT_MAX_BYTES. Operator Admit(p,L)=(L parses as usize and L>0 and byte_len(p)<=L), default L=150000 when env lookup fails. Assumptions: shared prompt_budget implementation, fixed encoding/serialization. Oracle: L=0, exact byte length and one byte below; malformed integer rejects. Evidence: prepare_next and lib.rs::prompt_budget source only. This byte bound is neither a token quota nor a measured teacher-cost bound.

## score-threshold-counterfactual

Type: decision_counterfactual. Observation: promotion threshold on four identity-bound grades.
Status: modeling_hypothesis_with_source_baseline.
Domain: four valid grades over two fixed assessments and two learners. Current operator R0=(new_after-new_before>0 and old_after>=old_before); prospective R_delta=(gain>=delta and old_after>=old_before), integer delta>=1, satisfies R_delta subset R0. Assumptions: same pair bindings and bounded integer scores; unchanged regression condition. Oracle: comparison_matches_lean_finite_table_and_rejects_mixed_pairs covers R0; sweep delta over its finite score table to check containment. Evidence: existing comparison test and source; R_delta is an unimplemented modeling hypothesis, not a measured quality gain.

## teacher-call-accounting

Type: resource_accounting. Observation: review call sites versus reported teacher-call metadata.
Status: partial_source_model.
Domain: one improve_with_base or finish_improve invocation. Operator C(trace)=count of direct review invocations: C(improve)<=1, C(finish)=0; failures before review give zero. The cycle receipt field equals 1 even on finish reentry, so metadata-to-actual-call reconstruction is multivalued. Assumptions: count this file's call sites only; shared review transport and its retries/billing are opaque. Oracle: inject failures before and after the review call and count invocations; rerun finish with valid artifacts and observe unchanged report. Evidence: source only, no paid call or transport audit executed. No monetary or token cost follows from this integer.

## frozen-union-preimage

Type: reverse_reconstruction. Status: bounded_rust_correspondence.
Domain: source-ID sets P (parent), A (non-exercise review) and observed frozen O.
Forward F(P,A)=P union A. Converse B_A(O) is empty if A is not a subset of O;
otherwise B_A(O)={P | O minus A subset P subset O}. Its cardinality on a finite
universe is 2^|A| when A subset O; freezing loses which parent supplied shared IDs.
Assume exact source-ID equality and the phase projection in frozen; renamed or
aliased identities are outside this relation. The three-ID Rust oracle tests
64 forward cases and all 512 inverse memberships, including duplicated/reordered
raw inputs and exercise-only IDs. Evidence: frozen_union_preimage_matches_all_small_source_sets,
executed. Finite correspondence is not a general machine proof.

## lexical-priority-bijection

Type: input_bijection. Status: bounded_rust_correspondence.
Domain: valid context metadata with unique packet headwords W. For priority list L,
admission requires |L|=|W|, every headword in W, no repeated headword, and matching
known/source fields. Thus headwords(L) is a permutation of W. Ordering, weak-hit
counts and prose reasons are not recomputed by this gate. The behavior complement
contains duplicated, omitted or foreign headwords under these assumptions.
The two-word/one-foreign oracle enumerates nine pairs (two accepted permutations)
and rejects two metadata mutations. The duplicate case failed against the old
count-plus-membership code, then passed after adding uniqueness. Evidence:
lexical_priority_admission_is_a_bijection_not_just_a_count, executed.
The extra ordered set uses O(n) space and O(n log n) time within an existing
O(n squared) membership loop; no measured speedup is claimed.

## tests

### Evidence and simple checks

All six names live in [study.rs](../study.rs), module study::tests:

- clozes_omit_frozen_sources_and_have_literal_role_answers checks grounded
  output and the all-blocked empty result.
- a_missing_or_identical_distractor_pool_does_not_invent_choices checks a small
  pool yields no fabricated alternatives.
- exercise_feedback_cannot_be_replaced_with_an_assessment_item accepts valid
  context then rejects an assessment question substituted into feedback.
- next_review_cannot_train_on_parent_assessment_sources rejects frozen-source
  exercises, then accepts after clearing that boundary.
- bound_feedback_survives_restage_and_rejects_tampering checks a valid restage
  and rejects modified context while the old staged run remains valid.
- dictionary_refresh_recognizes_published_words_and_protects_old_meanings checks
  current records, old-meaning protection and malformed dictionary rejection.

Relevant command, when a build is authorized:

    cargo test --release -p mova-enrich --lib study::tests

Use the worktree's separate CARGO_TARGET_DIR. This is a check recipe, not a run
record. [assessment.rs](../assessment.rs) contains same_count_and_model_cannot_bind_a_different_assessment
and the compare policy used by finish. Source anchors for otherwise untested
model terms are study_with_base, verify_outputs, validate_context, prepare_next,
finish_improve and propose_questions. [retention.rs](../../../../world/src/knowledge/retention.rs)
implements retained_lesson, train and fingerprint; this sidecar does not prove
those dependency internals.

Manual projection checks: with A={b,c}, Z1={a,b,c}, reverse frozen reconstruction
admits {a}, {a,b}, {a,c}, {a,b,c}; union each with A recovers Z1. A result containing
a frozen proposal contradicts the generator's prefilter. An existing incomplete
learned directory instead predicts a verification error; it must not be called
an absent behavior. These are model examples, not executed Rust tests.

Coverage gaps remain for all failure-stage cuts, process faults, supplied-context
semantic forgery, raw pool scaling and complete finish recovery. Existing test
names and source guards support only the stated partial models.

## complementer

### Reverse reconstruction: compatible study histories

Type: reverse_reconstruction. Subject: [study.rs](../study.rs), manifest SHA.
Domain: successful serial traces in model.md#approximant. Observation: selected
receipt/context fields and, when available, the referenced immutable artifacts.
Operator B(y)={x: there exists an admissible trace from x whose observation is y}.
Status: partial relational specification; no executable solver or unique inverse.
Evidence: source guards and [tests](model.md#tests); completeness is not established.

For observed frozen set Z1, let A be this review's nonexercise source IDs.
Compatible inherited sets satisfy Zparent union A=Z1. Hence reconstruction is
empty if A is not a subset of Z1; otherwise Z1\A subset Zparent subset Z1.
IDs in A may already have been frozen or may be new. The union forgets which;
it cannot recover a unique previous set. A bound parent packet resolves this
ambiguity by supplying the actual inherited set, subject to hash verification.

For a studied receipt, reconstruct candidates (packet bytes, review bytes,
seven output byte strings) satisfying the recorded parent and output hashes,
then intersect with successful-path constraints: train partition, valid review,
context identity, matching isolated exam and final model. Hashes alone do not
produce their byte preimages, identify an unavailable base checkpoint, or recover
training gradients. The summary's prior_state_sha256 constrains a candidate
prior state; the actual saved base remains needed to inspect it. Many executions
can share projected facts, while wall_seconds prevents asserting identical full
summary bytes across reruns.

For an evaluated_candidate receipt, a compatible history must contain baseline
and candidate checkpoints with the reported identities, the same two assessment
identities and four bound grades (new_before,new_after,old_before,old_after).
Given gain g and new_after a, recover new_before=a-g subject to integer score
bounds. Recommendation alone leaves every valid score quadruple satisfying
(g>0 and old_after>=old_before), or its negation, compatible. It does not recover
questions, choices, knowledge changes or a unique training path. Full bound grade
artifacts narrow this relation without identifying causal credit for improvement.

Soundness target: every reconstructed candidate must satisfy the selected
forward guards when supplied with its artifacts. Completeness target is only the
stated set-union projection, whose interval gives all compatible previous sets.
No unique recovery claim applies to hashes, recommendations or learned models.
The cycle receipt teacher_calls=1 is a report constant; finish itself makes no
teacher call and this field alone cannot reconstruct an actual paid-call history.
Missing artifact recovery and numerical stability of model reconstruction remain
unmodeled. These converse relations are separate from model.md#nonbehavior's U\R.

## invariants

### Invariants and boundaries

Subject and revision: [manifest](manifest.json). Scope: successful serial paths;
source-inspected contracts, not an exhaustive proof of the surrounding system.

- Frozen IDs grow by union across a bound study chain. Generated proposals
  exclude them; validation of subsequent review exercises is in
  [lib.rs](../lib.rs), exercised by next_review_cannot_train_on_parent_assessment_sources.
- Exercise feedback covers parent exercises in order, with exact source,
  question and expected answer identity. exercise_feedback checks retained
  fingerprints before/after answering. It reports support independently of
  correctness; priorities treat either a wrong answer or missing support as weak.
- Existing dictionary entries are refreshed before the next prompt; six-field
  records and unique headwords are required. Preserving old meanings is enforced
  by review validation, not by priority ordering.
- Study output verification requires exactly teaching.lesson, examination.lesson,
  model.json, exercise-before.json, study-context.json, exam.json and summary.json.
  exam-process.stderr is retained but is outside that seven-file hash contract.
- Learning continues from the baseline checkpoint when finishing a candidate.
  Assessments bind question identities, learners and rows before comparison.
  Equal counts alone do not permit comparing different question sets.
- Fresh directories and create-new writes preserve partial failure evidence.
  Their presence is not a success marker. finish can reuse complete validated
  artifacts, but an existing incomplete learned directory is not retrained.
- Study/prepare/finish make no teacher review call; improve has one review call
  site. No automatic retry loop exists here. Receipt teacher_calls is declared
  metadata, not independently audited transport accounting.

Not guaranteed: deterministic wall-clock summaries, teacher factual truth,
semantic validity of distractors, full lexical uniqueness in a supplied context,
transactional filesystem persistence, arbitrary concurrent safety or held-out
quality improvement. The manifest records the missing AST explicitly.

## nonbehavior

### Behavior complement: absent study transitions

Type: behavior_complement. Subject: [study.rs](../study.rs), manifest SHA.
Status: partial inner-bound specification, supported by source inspection;
not a machine-checked proof. Evidence: [tests and source anchors](model.md#tests).

Universe U contains tuples (operation, initial artifacts/state, external outcomes,
finite artifact/call trace, return result) for study_with_base, prepare_next,
finish_improve and improve_with_base. Fix a serial, stable-filesystem invocation,
valid Rust execution and the dependency contracts named in model.md#approximant.
R is actual Rust call behavior within this universe. The operator targets U\R.
The following union C is an intended inner bound C subset U\R; it does not claim
that every excluded trace is listed. In particular, an invalid input followed
by an error is in R, not in C.

C includes traces with these contradictory successful transitions:

- study succeeds from an evaluation-routed packet or a noncurrent/nonstaged
  parent, or overwrites an already existing destination directory.
- study emits context whose frozen set differs from the union of inherited
  frozen IDs and this review's nonexercise IDs; generated clozes use a frozen
  source; feedback enumerates assessment items as exercises.
- study emits more than sixteen generated clozes, or emits a cloze without three
  distinct canonical distractors selected from the same-role raw candidate pool.
- study reaches a successful receipt after an isolated exam reports another
  model, nonzero lesson reads/updates, or fails AssessmentContract binding.
- prepare_next succeeds after a required output byte/hash mismatch, a changed
  context binding, or fewer than three nonfrozen source sentences.
- finish succeeds with a candidate lineage other than the baseline model, a
  foreign cached assessment, or a changed existing cycle receipt.
- study or prepare_next directly invokes review; finish invokes review; improve
  invokes its review step twice in the same trace. None of these paths exists.
- a successful finish report sets published=true, or sets recommendation=true
  when the bound same-new-assessment gain is nonpositive or old score regresses.

These are missing behaviors, not merely a list of negative test cases: their
complement relation is over complete guarded call traces and observed outputs.
The forbidden metadata transitions are source-local; score exclusions also
assume AssessmentContract and compare execute as read in assessment.rs.

Unknown/outside bound: semantic correctness of a teacher answer; global absence
of label leakage through arbitrary source text; malicious simultaneous writers;
all possible IO failures. Validation does not recompute feedback booleans or
priority hit counts from the saved model, and count-plus-membership checking of
lexical priorities alone is not a proof of unique coverage. These stronger
properties must not be smuggled into C. A receipt hash check is consistency
under trusted artifacts, not authentication against replacement of the receipt
and every referenced file. Partial output directories are allowed behavior.

## reuse-cost

### Specialized model: reuse cost and failure stages

Type: reuse_cost. Subject: study.rs at manifest SHA. Domain: the serial trace
model, fixed parent/material and dependency behavior. Observation: work executed,
artifact stage and first failure, not numerical throughput. Operator maps a
trace's reuse flags to symbolic work terms. Status: partial analytic model;
source inspection only, no benchmark or measured speedup. Evidence: retained
branch in study_with_base and existence branches in finish_improve; see model.md#tests.

Let V cover validation/hashing/IO, E exercise feedback, L training, P proposals,
X process launch plus assessment, and K lesson-identity lookup. A study costs
approximately V+K+2E+P+X+(1-r)L, with r=1 exactly when retained_lesson returns
Some(number). Reuse does not skip feedback, context generation, saving, hashing,
or the isolated exam. It keeps the matched number and reports zero new frames,
changed weights and exercise evaluations for training. This is not a promise
that the entire command performs no evaluations.

For finish, let cL,cR,cB indicate validated existing learned artifacts, regression
exam and candidate-before exam. Cost is approximately
V+(1-cL)Study_candidate+(1-cR)X_old+(1-cB)X_new+Bind4+Compare.
Existence is only the branch condition; verification remains required afterward.
A bad cached result produces error rather than automatically paying to replace
it. A matching cycle receipt still incurs validation and binding on reentry.

With N raw eligible clozes, row count t_s per sentence and source text length
b_s, candidate discovery includes O(sum_s(t_s^2+t_s*b_s)) naive scans.
Repeated role-pool construction costs up to O(N^2 log(N+1)) map work plus string
canonicalization. Sixteen is an output cap, not a discovery bound: all raw
candidates are built first, and many unsuccessful candidates can scan the pool.
Memory includes source rows, O(N) raw strings and O(N) pool keys. Priority cost
includes repeated weak-feedback question tokenization and source membership
checks for each word, then O(W log(W+1)) sorting. Hash/serialization cost scales
with artifact bytes. Dependency learning/answering costs are deliberately opaque.

Failure-stage projection: before destination creation, no new study tree is
written; after material writes, feedback/training failure leaves material;
after checkpoint/context writes, subprocess/binding failure leaves those files
and possibly stderr/exam output; summary precedes receipt. No rollback transition
is present. In improve, a failed review/stage prevents finish; reusing finish
avoids another review call but requires valid staged inputs. A crash during
create-new writes can leave unusable partial files, not a resumable transaction.

Prospective checks: instrument per-stage times and raw-pool cardinality before
proposing caching; compare reuse versus fresh training for identical material
and base with separate output directories. Expect training term elimination,
not a fixed total speedup. These experiments have not been run for this sidecar.

## History

### 2026-10-29 · descriptant

Mova turns exercise feedback into source-grounded proposals, prepares a bounded
review, and evaluates retained learning with isolated question-only assessments.
This file describes the combiner and retains useful modeling observations.
Epoch 1, 2026-10-29: source inspection at db9905694; exact Rust SHA in manifest.
No Rust behavior changed and no new learning-quality or timing result is claimed.

Epoch 2, 2026-10-29: [catalog](model.md#catalog) expands the family to 21 models,
retaining both required baseline types first and the earlier symbolic reuse cost.
The combiner can now project provenance, reservation capacity, feedback identity
and state preservation, priority ranking, cold assessment, artifact closure,
lineage, failure prefixes, cache validity, retained reuse, dictionary extensions,
literal clozes, distractor equivalence, truncation ambiguity, prompt admission,
score thresholds and call accounting. These are correlated source views; their
intersection is a candidate consistency check, not independent verification.
The score-threshold variant is explicitly hypothetical and changes no policy.

#### Set and coverage correspondence, 2026-10-29

The family now has 23 models. A finite converse oracle confirms frozen-source
ambiguity and exact compatible-parent sets over three IDs. A separate admission
oracle found that equal priority count plus membership did not establish coverage:
one headword could replace another. The context gate now requires distinct
headwords, with metadata still checked against the packet. Its negative case
failed before the fix. Source identity is updated in the manifest.

### 2026-10-29 · epoch-memory

2026-10-29: five fragments add twenty explicit models. The bounded priority oracle initially expected a short token to survive the shared tokenizer. Its failure corrected the incidence model; production semantics were preserved. The proposal oracle independently checks output cardinality and ordered distractors across all 64 source subsets, in addition to removal-before-pooling equality. Full process recovery and semantic natural-language quality remain untested.

### 2026-10-29 · document

Epoch 2, 2026-10-29. Subject: [study.rs](../study.rs), SHA and revision identity in
[manifest.json](manifest.json). The three earlier models remain separate artifacts.
These eighteen specializations expose different observations of the same code;
they are not independent proofs. Domains below are single serial calls or
explicitly bounded chains, with stable input files and dependency contracts
from model.md#approximant. Relations are partial source-derived models unless marked
as hypotheses. Checks named here are existing tests; proposed oracles are not run
records. Historical integrated test evidence is in [tests](model.md#tests).

### 2026-10-29 · frozen-union-preimage

Type: reverse_reconstruction. Status: bounded_rust_correspondence.
Domain: source-ID sets P (parent), A (non-exercise review) and observed frozen O.
Forward F(P,A)=P union A. Converse B_A(O) is empty if A is not a subset of O;
otherwise B_A(O)={P | O minus A subset P subset O}. Its cardinality on a finite
universe is 2^|A| when A subset O; freezing loses which parent supplied shared IDs.
Assume exact source-ID equality and the phase projection in frozen; renamed or
aliased identities are outside this relation. The three-ID Rust oracle tests
64 forward cases and all 512 inverse memberships, including duplicated/reordered
raw inputs and exercise-only IDs. Evidence: frozen_union_preimage_matches_all_small_source_sets,
executed 2026-10-29. Finite correspondence is not a general machine proof.

### 2026-10-29 · lexical-priority-bijection

Type: input_bijection. Status: bounded_rust_correspondence.
Domain: valid context metadata with unique packet headwords W. For priority list L,
admission requires |L|=|W|, every headword in W, no repeated headword, and matching
known/source fields. Thus headwords(L) is a permutation of W. Ordering, weak-hit
counts and prose reasons are not recomputed by this gate. The behavior complement
contains duplicated, omitted or foreign headwords under these assumptions.
The two-word/one-foreign oracle enumerates nine pairs (two accepted permutations)
and rejects two metadata mutations. The duplicate case failed against the old
count-plus-membership code, then passed after adding uniqueness. Evidence:
lexical_priority_admission_is_a_bijection_not_just_a_count, executed 2026-10-29.
The extra ordered set uses O(n) space and O(n log n) time within an existing
O(n squared) membership loop; no measured speedup is claimed.

### 2026-10-29 · tests

Evidence collected 2026-10-29: read study.rs and its six inline unit tests,
assessment.rs comparison policy, and the existing study-orchestrator registry
entry. The manifest records the actual source SHA-256. Tests below were inspected,
not executed in this documentation-only task; no teacher or training run occurred.

Integration check, 2026-10-29: cargo test --release -p mova-enrich --lib
passed all 16 tests, including the named study and assessment checks. No teacher
request, corpus training or full exam course was run for this modeling pass.

Catalog expansion check, 2026-10-29: mova-modelcheck groupants accepted the
six sidecars in this checkout with no diagnostics (525 missing sidecars remain
visible backlog). mova-modelcheck changed against 772c98aec reported no changed
Rust subjects and no missing contracts. git diff --check passed. These checks
validate inventory/source bindings, not the semantics of the 18 new models.
The family now has 21 entries; each new ID names a heading in model.md#catalog.
Existing Rust tests were inspected and their prior integration result retained;
no Rust test rerun, training, teacher request or benchmark accompanied this
sidecar-only expansion. Proposed fault, quota and boundary oracles are unrun.

#### Executed bounded model checks, 2026-10-29

frozen_union_preimage_matches_all_small_source_sets checks 64 forward cases and
512 inverse memberships over three source IDs, plus exact preimage cardinalities,
canonical ordering, duplicates and ignored exercise-only IDs.

lexical_priority_admission_is_a_bijection_not_just_a_count enumerates nine
two-entry lists from two proposed and one foreign headword: exactly two valid
permutations. It also rejects known/source metadata changes. The old code failed
on duplicate pair (0,0), accepting it; the uniqueness fix makes the test pass.

The integrated mova-enrich library suite passed all 23 tests. These finite oracles
exercise actual functions, without live teaching, training or GPU execution.

The general set relation now also has a separately checked Lean theorem,
MovaFrozenUnion.union_preimage in formal/FrozenUnion.lean, with an axiom audit
and rejected false-formula control (2026-10-29). This strengthens the specification;
Rust correspondence remains the separately executed finite-domain experiment.

#### Nested fragment correspondence, 2026-10-29

The file retains 23 models; five nested fragments add four models each (20).
priority_aggregation_matches_independent_incidence_oracle executes 128 cases:
64 correctness/support masks times two understood settings. A hand-specified
incidence matrix checks weak counts, lexical ties and reason selection, including
short-token exclusion, repeated source IDs and source/token overlap. The first
attempt failed because its matrix incorrectly included three-byte fox in token
matching. Inspection of the shared tokenizer corrected the model, not production.

proposal_blocking_equals_removing_sources_before_pooling checks all 64 subsets
of six source sentences. It compares blocked input with physical removal and
independently predicts exact counts and the ordered first-three distractors.
No semantic natural-language answer quality is inferred from this fixture.

Executed: cargo test -p mova-enrich --lib passed 25 tests. A temporary production
mutation changing weak feedback OR to AND made the incidence oracle fail;
original source was restored. The focused study suite was rerun after restoration.
This negative control establishes sensitivity to one real aggregation defect,
not mutation completeness. No teacher, GPU, training or whole-repository suite
was run. Separate external run logs retain the initial model error, positive
suite, negative control and restored focused suite.

The restored focused study suite passed all 10 tests (15 other library tests
filtered out). Groupant validation reports no study or fragment diagnostics;
the two ancestor manifests still require the integration owner's source-hash
refresh. This leaf work does not claim a clean global inventory before that
refresh. All five fragment ranges are source-bound and nested inside their
parent range, with exact H2 artifact references and explicit AST gaps.

### 2026-10-29 · invariants

- Context lexical priorities contain each unique packet headword exactly once;
  known/source metadata agrees. Ordering and computed hit counts remain outside
  this gate. The 2026-10-29 bounded bijection test guards duplicate substitution.
