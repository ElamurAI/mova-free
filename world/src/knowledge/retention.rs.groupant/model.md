# retained-student

## descriptant

### Retained learning groupant

Mova retains a course's graph and role weights in an explicit checkpoint, then
grades question-only material from restored state. This file owns the checkpoint
boundary, teaching/assessment split, guarded training and closed-book exam wrapper.
It does not establish semantic generalization or the quality of the parser and
inference engine.

The forward model is in model.md#approximant; invariants and prospective costs are in
model.md#invariants-cost. The first typed obernant, model.md#nonbehavior, specifies excluded
successful transitions within an explicit one-call universe. The second,
model.md#complementer, reconstructs compatible antecedents as sets. model.md#persistence and
model.md#identities add models useful for interpreting retained state and assessment
reuse. model.md#catalog adds sixteen specialized models of compatibility, provenance,
ordering, write failures, reuse, scoring and reconstruction. All twenty models
are partial source-derived artifacts tied to manifest.json.

#### Combining operation

For this leaf, the intended combiner takes raw Rust (AST pending), forward
transition projection, typed relations, invariants and evidence. It checks that
each exclusion/reconstruction uses the same payload and call boundaries, records
counterexamples, and proposes narrower assumptions or revised models. Parent
self-study composition must additionally bind learner/assessment identities,
fresh-process execution and old-knowledge regression. A correct checkpoint leaf
alone does not establish those interactions. No autonomous combiner was run.

## approximant

### Forward approximant: retained learning

Subject/revision: retention.rs at manifest.json source identity. Status: partial
source-derived specification. Observation horizon: one completed public call;
process isolation additionally requires the CLI test path and trusted execution.
Assume terminating dependencies, compatible serialization/runtime, valid allocated
memory and ordinary filesystem semantics. Parser and inference are opaque.

Write S=(P,C,I), where P=(schema,course,ledger,frames,same,texts,weights,parts,isa,per)
is payload(), C denotes omitted transient state and I derived lookup indexes.
J(P) below means the validations actually performed by load, not all imaginable
semantic consistency conditions. H is SHA-256 of serde_json serialization.

- new(course): default student, read_kinds(), lesson_focus=false, empty ledger.
- fingerprint(S)=H(P). save creates a new path exclusively, writes an envelope
  (P,H(P)) and syncs the file. It does not replace an existing path.
- load: deserialize; require schema, checksum, weight-layout length, unique text
  keys and parts/per keys; text lessons must be in the ledger; every frame source
  must exist and quote/cause/next indices must be in range. Kind relation sources
  must be KINDS_SRC or a text key. Rebuild indexes and codes from frames in order,
  with light verbs excluded from by_verb; set lesson_focus=false.
- train(L,p): reject labels, absent teaching evidence, foreign course and a
  ledger/practice mismatch. Let d=H(course,L.n,explain,facts,exercises).
  Allowed cases: absent ledger entry and p=false, or identical digest and p=true.
  Preparse teaching/exercise inputs; in the new case read teaching and insert d.
  Set lesson_focus=false, call learn_lesson(teaching,2), report frame delta,
  changed weight positions, exercise evaluations and ledger size.
- exam(L): require same course, no explain/facts/exercises, and nonempty tests or
  exams. Save H(P), preparse questions/options, answer deterministic shuffled
  options, then compare choice with gold. Rows contain diagnosis and witness.
  Require final H(P)=initial H(P). Return counts, identities, elapsed time and
  zero lesson-read/exercise-update declarations.
- retained_lesson(L): same-course gate, scan ledger in ascending key order;
  recompute material digest using each stored number; return first match.
  next_lesson_number gives maximum stored number plus one, checked for overflow
  and the reserved KINDS_SRC value; it does not search for a vacant lower slot.
- split_lesson validates material then filters original lines by field name:
  headers go to both, teaching/exercise lines to train, test/exam lines to exam.
  Other lines are dropped; emitted lines receive a newline.

Training syntax validation (positive lesson number/nonempty course) belongs to
parse_material and CLI callers; train itself is public and does not repeat it.
Direct mutation through public student is possible. In particular exam reports
lesson_focus=false but does not set that field: neutral focus is an entry
assumption furnished by new/load/train, not enforced by every public call.

Error effects are partial: preparse may alter transient caches; failed writes
can leave a partial new file; split writes two files sequentially with no rollback.
No global atomicity, answer accuracy or exercise-improvement claim follows.

Evidence: inspected implementation and in-file tests; model.md#tests separates available
checks from execution. Dependencies and successful full exams remain unmodeled.

## composition

# Retention fragment tree

## composition (composition)

The file retains its twenty typed models. Six source-bound fragments add thirty
models: checkpoint contains canonical, validation and restore; material and exam
are its siblings. All bind the current whole-file Rust SHA in manifest.json. The ranges
are partial and intentionally leave training, split/CLI and identity helpers
modeled only by the file-level artifacts. Called helpers can lie outside a
fragment interval; their behavior is an explicit dependency, not claimed syntax
coverage.

For valid persisted p, checkpoint restoration is V(p) followed by D(p), with
canonical observation C(p)=P(D(p)). The restore fragment assumes initialization
from the validation prefix. P and D retain the same course and ledger used by
the material matcher; matching substitutes each retained key into the teaching
digest. Reuse is a query and does not itself perform training or allocation.

A closed-book path is Load(e)=s followed by Exam(s,l)=r. Its relation is
R(e,l,r) iff there exists s with Rload(e,s) and Rexam(s,l,r), preserving course
compatibility and question-only gates. Calling exam directly after public student
mutation does not inherit the load-time false-focus guarantee. The exam report's
literal false field is not independent evidence of that guarantee.

The parent behavior complement is U minus this coupled relation, with U ranging
over envelope, lesson, intermediate state and observable result. Only source-gate
exclusions are established as an inner bound. It is not the union of child
complements. Reverse reconstruction retains every compatible envelope and
intermediate state; fingerprints do not identify a unique antecedent. Restore
canonicalization erases vector ordering and live transient fields.

No costs are added twice for the overlapping checkpoint parent/children.
End-to-end cost includes disk/JSON/hash, validation, index rebuilding, then parser
and answer costs if an exam succeeds. Material lookup is a separate branch,
with repeated hashing proportional to examined ledger candidates.

## epoch-memory

## catalog

# Specialized retention obernants

## schema-layout-boundary

Domain: deserialized envelopes and current start_weights(). Observation: load
return or first gate error. Relation: acceptance requires schema=SCHEMA,
hash(serialized payload)=stored digest, and weights.len=start_weights().len,
in that order. Unknown schema with a valid checksum still lies outside success.
Assume deserialization succeeded; equal length is not proof of compatible weight
meaning across code versions. Oracle: vary schema or vector length and recompute
the checksum to isolate the relevant gate. Evidence: load; existing
tamper_and_unsupported_schema_are_rejected covers only part of this boundary.

## provenance-reconstruction

Domain: accepted payloads; observation: graph source keys. Relation: every text
key (n,line) requires n in the ledger; every frame src requires an exact text key;
every parts/isa/per source requires a text key or KINDS_SRC as its first component.
The compatible antecedent set includes any text/ledger values satisfying these
constraints and the remaining load gates. Source claims need not be true or
uniquely reconstructible; digest values are not checked against original lessons.
Oracle: remove one required source or ledger key and rehash, expecting rejection;
substitute text without breaking structure to expose the authenticity limit.
Evidence: load's source_ok and provenance gates; source-derived, no new execution.

## edge-topology-boundary

Domain: a deserialized, otherwise admissible F-frame payload. Observation: load
success and rebuilt edge indexes. The excluded transition set contains every
successful load with quote/cause/next target j>=F. This is an inner complement;
cycles, self-edges and multiple quote owners are not excluded by this predicate.
Frame::link is outside this global-index domain. Assume valid usize encoding.
Oracle: out-of-range next must fail despite a recomputed checksum; a cyclic graph
requires a separate downstream analysis. Evidence:
structurally_invalid_graph_with_valid_hash_is_rejected and the load edge loop.

## canonical-payload-fiber

Domain: accepted payloads differing in texts/parts/per vector order, with unique
keys and correctly recomputed checksums. Observation: payload() after load.
Relation: the preimage of a canonical payload includes permutations of these
vectors because collection into BTreeMap followed by iteration orders their keys.
Frame order and list-valued contents are not quotiented out. Assume reconstruction
helpers preserve persisted coordinates. Incoming envelope hashes can differ while
restored fingerprints agree. Oracle: permute each vector, rehash, load and compare
canonical fingerprints. Evidence: payload/load collection operations; proposed
oracle now implemented by oracle_canonical_payload_fiber_preserves_values_not_vector_order:
eight distinct wire payloads converge to one exact canonical JSON payload and
fingerprint; a changed text value stays distinguishable. This refines
model.md#persistence with an explicit bounded input fiber.

## index-order-reconstruction

Domain: accepted ordered frames. Observation: by_ent/by_verb/by_line, prev/effect,
sayer and code lengths after load. Relation: fold frames by increasing index,
appending each matching occurrence; exclude empty/light verbs from by_verb;
reverse next into prev and cause into effect; sayer[q] is the greatest frame index
whose quote contains q. Repeated entity occurrences need not deduplicate.
Assume ents(), LIGHT and mcode retain their pinned semantics; codes themselves
are opaque. Oracle: two quote owners select the later index, and permuting frames
with consistently remapped edges changes ordered index lists. Evidence: load's
rebuild loop; roundtrip test checks selected indexes, not all these cases.

## checkpoint-write-effects

Domain: save(S,path), including I/O failures. Observation: return tag and pathname
contents. Relation: serialize/hash before opening; create_new failure writes
nothing through this call; after successful creation, failed write_all may leave
an empty or partial file, and failed sync_all may leave complete bytes without
the promised completed sync. Success follows all three stages. No rollback or
rename transition exists here. Assume standard I/O semantics; directory durability
and concurrent filesystem interference are outside the guarantee.
Oracle: fault injection at open/write/sync distinguishes these outcomes; retry
against a leftover path must hit exclusive creation. Evidence: save sequence;
roundtrip test covers existing-path rejection only.

## split-write-effects

Domain: main_args split with validated material and distinct initially absent
output names. Observation: both paths and the command result. Relation: complete
train write precedes exam write; failure of the second write can coexist with a
complete first artifact and absent/partial second artifact. Initial exists checks
do not make the pair transactional; create_new still controls each actual open.
Assume ordinary I/O; no directory transaction or rollback is implemented.
Oracle: induce failure only on the second output and inspect the first.
Evidence: main_args split and write_new; source-derived, oracle not executed.
This concerns two-artifact commitment, distinct from a single save's write phases.

## split-input-fiber

Domain: strings accepted by parse_material. Observation: split_lesson's two
strings. Relation: headers are copied to both outputs; teaching/exercise lines
only to train; test/exam lines only to exam; unrecognized lines are erased and
every retained line gains a newline. Compatible inputs preserve each output's
within-stream order and shared header sequence, but can interleave private lines
differently and add ignored lines if parse_material still accepts the result.
Assume actual parser acceptance, not arbitrary ignored-line admissibility.
Oracle: compare final-newline variants and safe cross-stream interleavings.
Evidence: split loop and split_separates_training_from_exam_labels; no complete
inverse parser is supplied.

## practice-transition-boundary

Domain: train(S,L,practice) after label/evidence/course gates. Observation: ledger
and whether read is reached. Let d=material_digest(L,L.n). Admissible cases are
missing key with practice=false, or ledger[L.n]=d with practice=true. Success
outside these cases is excluded. The first case reads and inserts d; the second
skips read/insertion but still calls learn_lesson(teaching,2).
Assume preparse succeeds and dependencies terminate; this does not make practice
idempotent or ensure no frames change inside the learner. Oracle: a four-case
ledger/practice matrix plus changed digest. Evidence: train and
duplicate_or_changed_lessons_fail_before_any_parsing; runtime learning unmeasured.

## lesson-reuse-selection

Domain: same-course lesson and ordered ledger. Observation: retained_lesson.
Define M={n in ledger | material_digest(L,n)=ledger[n]}. Result is Some(min M)
when M is nonempty, else None. Caller L.n and assessment fields are erased before
each comparison; changing teaching/exercises can change M. Assume successful
serialization; matching hashes mean bytes only under a collision assumption.
Oracle: construct matches at two numbers to require the smaller; changing only
placeholder/test fields must preserve the result. Evidence: retained_lesson loop
and retained_material_is_found_across_placeholder_ids_and_assessment_changes.
The executable oracle_reuse_fiber_selects_smallest_of_multiple_antecedents creates
three matching ledger entries from independent explicit tuples; four placeholder
numbers and assessment changes select 3, then deletions select 7 and 19.
Changed facts and another course are outside this observed identity fiber.

## lesson-number-frontier

Domain: any representable ledger key set. Observation: next_lesson_number result.
Operator: checked successor of max(keys), taking max(empty)=0; reject overflow or
candidate=KINDS_SRC. Holes below the maximum remain unused. This models allocation
frontier, not material identity or the parser's positive-number gate.
Assume unchanged usize width and KINDS_SRC; callers must handle the error rather
than infer a free slot exists. Oracle: sparse keys, empty ledger, maximum usize
and predecessor of KINDS_SRC. Evidence: next_lesson_number; existing reuse test
checks ordinary successor only. oracle_allocation_frontier_ignores_holes_and_rejects_boundaries
adds empty, zero, sparse, maximum-adjacent, reserved and overflow cases against
literal expected results and separate error classes.

## exam-score-passivity

Domain: exam-admissible lessons and retained states. Observation: payload hashes,
answer input and scoring rows. Relation: successful exam requires equal before/
after fingerprints; answer receives question and shuffled option strings before
choice is compared with gold position. Gold text participates in options, so
changing gold text can change answers; no blanket label-independence is claimed.
The gold index is used for diagnosis/why after selection, not passed to answer.
Assume collision resistance for payload equality, opaque deterministic answer
semantics and termination; transient caches are outside the projection.
Oracle: capture identical question/options and confirm selection precedes scoring,
and compare payload fingerprints across a successful exam. Evidence: exam call
order; existing rejection test never reaches scoring. Successful passivity here
is source-derived, not newly measured.

## assessment-presentation-order

Domain: exam-admissible finite test/exam vectors. Observation: ordered rows and
option presentations. Operator: concatenate test rows then exam rows, enumerate
each from zero, and shuffle with (((n as u64)<<20)|((i as u64)<<1)|1) XOR extra,
where extra is 0 for tests and 0xE8A3 for exams. Row index resets for exams.
Assume pinned shuffled implementation; the bitwise seed formula is not an
injective encoding of arbitrary usize pairs. Ordered option content and seed,
not item count alone, determine presentation. Oracle: enumerate two rows of each
kind and compare row keys/seed inputs; swap distractors to test identity change.
Evidence: exam loop and assessment identity test; no new shuffle measurement.

## diagnosis-partition

Domain: answer results with valid choice and gold-option indices. Observation:
row diagnosis. Ordered predicates partition outcomes: correct if choice=gold;
otherwise count_rule_error if count is present; otherwise missing_correct_support
if gold best is absent; otherwise question_not_understood if understood=false;
otherwise supported_selection_error. Earlier predicates mask later ones.
Assume answer result indexing is valid; these are implementation categories,
not certified root causes. Oracle: a case with count present, no gold support
and understood=false must choose count_rule_error. Evidence: exam diagnosis
chain; proposed branch-table oracle unexecuted.

## training-report-fiber

Domain: successful train calls and their before/after states. Observation:
(new_frames,changed_weights,exercise_evaluations,practice,lessons). Relation:
compatible antecedents satisfy the train gates, frame-length difference, unequal
paired weight count over zip(old,new), learner evaluated count and final ledger
length. This is a constraint preimage; arbitrary satisfying tuples are not
asserted reachable without the opaque learner relation.
Assume nonshrinking frame count for the reported subtraction and ordinary bounded
arithmetic; zip compares only overlapping coordinates if weight lengths change.
Counts lose coordinate identities, values and exercise history. Oracle: compare
two different equal-count weight edits to witness ambiguity; zero changed weights
does not establish unchanged frames. Evidence: train report construction.
No reverse executor, learning-quality result or unique recovery is claimed.

## exam-focus-boundary

Domain: reachable new/load/train outputs and states edited through public student.
Observation: lesson_focus on exam entry and its report declaration. Relation:
new/load set false, and successful train sets false before learning; preservation
through learning assumes that dependency does not change it. exam does not reset
focus yet emits lesson_focus=false unconditionally on success. Therefore the
reported declaration implies actual neutral focus only with an entry/preservation
assumption; public mutation can break that implication.
Oracle: set the public field true before an admissible exam and compare actual
entry flag with the report, conditional on successful parser/answer dependencies.
Evidence: constructors, train assignment and exam JSON; roundtrip test checks
loaded focus only. No successful counterexample run is claimed.

## tests

### Existing checks and remaining evidence

Subject/revision: manifest.json. The original documentation epoch inspected
references only. The later bounded oracle epoch below records Rust execution;
no Lean builds, parser sessions, training or benchmarks are credited.
All unit names below are in ../retention.rs, module knowledge::retention::tests.

| Check | Evidence scope |
|---|---|
| split_separates_training_from_exam_labels | Export field separation and separate material validation |
| state_roundtrip_retains_weights_aliases_and_kind_graph_without_caches | Hand-built payload fingerprint, selected indexes, empty caches, exclusive save |
| tamper_and_unsupported_schema_are_rejected | Digest mismatch and unsupported schema |
| structurally_invalid_graph_with_valid_hash_is_rejected | Out-of-range next rejected despite recomputed checksum |
| exam_rejects_teaching_and_does_not_change_state | Early rejection and unchanged fingerprint; does not run a successful exam |
| duplicate_or_changed_lessons_fail_before_any_parsing | Duplicate/stale/practice/course guard examples |
| retained_material_is_found_across_placeholder_ids_and_assessment_changes | Reuse lookup and teaching/assessment partition examples |
| assessment_identity_models_scored_inputs_and_ignores_unread_metadata | Selected identity mutations and metadata invariance |

../../../../world/examples/retention_probe.rs compares live/cold reports after
removing seconds, then evaluates a blank baseline. It is an existing executable
probe, not a unit proof or evidence that this epoch ran it. Independent CLI
train/test invocations are needed to establish the fresh-process protocol.

../../../../formal/Retention.lean contains restored_answer, exam_preserves_state,
amortized_saving and repeat_cost_derivative. The encode/decode identity is an
explicit premise; these theorems do not prove Rust serialization or answer quality.

At the first documentation epoch, unexecuted checks included duplicate quote-owner rebuilding; successful exam after
direct public focus mutation;
fault injection between split outputs and during save; all provenance gates;
successful cold exam invariance; old-knowledge regression after practice.
These are gaps, not silently credited coverage. AST extraction, reverse solver,
full behavior-complement correspondence and sidecar semantic validation remain open.

## audit

### Checkpoint trust and reconstruction audit

#### behavior-complement

Type: behavior_complement; partial source-derived inner bound.
Let U be decoded-envelope/return pairs and R the load transition relation.
Let g1..gk be the actual ordered acceptance predicates. The inner bound includes
(e,Ok) whenever some gi fails, even when the checksum has been recomputed.
The complementary successful-save exclusion is an existing target under ordinary
exclusive-create semantics. Successful load with bytes changed at its input path
is also excluded under the no-concurrent-writer assumption. Negative fixtures
sample these rules; they do not define the whole complement.

#### reverse-reconstruction

Type: reverse_reconstruction; partial relational specification.
For a failed gate occurrence i, the fiber is {e | all j<i: gj(e), not gi(e)}
with later predicates unconstrained. An observed error string generally denotes
a union of these fibers, since loop occurrences can share the same message. The tests witness duplicate-source before missing-ledger and
frame-source before invalid-link outcomes; they do not enumerate every pair.
For successful restored state s, preserve all accepted e with D(e.payload)=s.
Payload normalization P(D(p)) canonicalizes unique texts/parts/per keys.
Hence checksum(e.payload) may differ from fingerprint(s); no unique wire
ordering, live cache, focus, exercise history or trustworthy author is recovered.
For q->i alone, quote-owner antecedents require i to quote q and every later
frame not to quote q. Earlier quoting frames remain free.

#### validation-versus-authentication

The checksum binds the deserialized payload serialization, not its author's
identity or literal JSON bytes. Ledger strings are not parsed as SHA-256 digests
and are not checked against source texts. A correctly resealed replacement text
with an arbitrary ledger string is accepted. Its reuse lookup returns None in
the explicit fixture, since the arbitrary string differs from the independently
computed material digest. This is a trust boundary, not a new schema defect.
Save serializes public mutable student fields without the load gates: it can
successfully write an out-of-range edge that load subsequently rejects.

The malformed-null envelope checks deserialization separately; the wrong-checksum
fixture reaches exactly the checksum error. The resealed matrix exercises schema,
weight layout, duplicate text, ledger membership, frame provenance, each of
quote/cause/next range, duplicate parts/per, and kind provenance. These are finite
representatives, not arbitrary malformed-JSON or denial-of-service coverage.

#### provenance-and-topology

For each of parts/isa/per, src is accepted when src.0=KINDS_SRC or texts[src]
exists. For frame provenance, only texts[src] suffices; the sentinel supplies no
special exemption. The 3-by-4 table samples actual text, absent line, absent
lesson, and sentinel, with literal retained-source observations after success.
Accepted quote self-links and next cycles demonstrate that range validation
does not certify acyclicity or causal truth. The schema and code stay unchanged.

#### ordered-index-projection

The entity index records occurrences, including repeated IDs from an entity's
coordination list and repeated roles. Its lists are not sets. The literal
fixture expects alice->[0,0,0,1], basket->[1], bob->[0]. Empty/light verbs add no
verb entry; line membership remains [0,1]. Repeated quote entries and a later
owner yield sayer[1]=1. Removing earlier quotes preserves that owner map while
changing the payload fingerprint, witnessing a non-singleton index preimage.
Codes are checked only for cardinality; numeric-code semantic equivalence remains
outside this oracle. Empty caches establish cold reconstruction of these fields,
not equality of all hidden or future runtime state.

#### cost-and-composition

Sealing/hashing cost scales with serialized bytes. Validation includes ordered
map insertion and scans of graph sources and links; rebuilding scales with frame
entity occurrences and links plus the actual mcode helper cost. No constant
mcode or summed overlapping-fragment cost is assumed. This audit strengthens
checkpoint validation/restore children and their common payload observation.
Reuse is still a separate read-only query, and exam passivity still means the
persisted projection rather than all transient caches. Successful examination
and full-state inverse laws have not been tested here.

#### epoch-memory

Prospective prediction before runs: all existing twelve retention checks plus
four new synthetic tests should pass without changing production code. In
particular, v1 should accept the stated weakly trusted artifacts while rejecting
structural defects even after resealing. Measured outcomes belong to model.md#tests.
The audit found coverage gaps and clarifies trust claims; it found no production
defect warranting an unrequested v1 compatibility change.

## complementer

### Second obernant: reverse reconstruction

Type: reverse_reconstruction. Subject/revision: manifest.json. Status: partial
relational specification; no reverse executor. Domain and horizon: one successful
load/save/train/exam call over the values in model.md#approximant. Observation scope
must say whether it contains payload bytes, a digest only, or a report.
Assume the pinned serializer and dependency versions; ordinary I/O; restrict to
states satisfying the listed load gates when discussing restored payloads.

Let pi(S)=P and D(P) be load's deterministic rebuilding of omitted indexes and
default transient state. For a payload observation p, define

    B_payload(p) = { S | pi(S)=p }.
    B_load(s) = { envelope e | gates(e) and D(e.payload)=s }.

Neither is a unique history. A valid checkpoint retains graph/weights/ledger but
not the sequence of exercises or cache history that produced them. Save/load is
a candidate section/retraction on canonical valid payloads, not an inverse on
full live student states. model.md#persistence specifies normalization limits.

For retained_lesson(L)=Some(n), compatible inputs satisfy equal course,
H(course,n,explain,facts,exercises)=ledger[n] and no smaller matching ledger key.
The caller's placeholder L.n and assessment fields remain unconstrained by this
observation. None means no ledger digest matches, under successful course check.
Digest observations give equivalence classes, not recovered teaching text.

For a successful training report (new_frames,changed_weights,evaluations,...),
B_report is all (S,L,practice) satisfying the forward gates and producing those
aggregates. Changed-weight count alone leaves both changed coordinates and values
ambiguous. Zero frame delta does not establish absence of learning. Even final
payload plus counts does not identify the pre-training weights: learning is not
assumed injective.

For an exam report, candidate antecedents must satisfy the state fingerprint,
assessment fingerprint, row order and answer/diagnosis/witness relations under
the actual answer function. Defining this intersection is a necessary constraint
model, not a practical solver for arbitrary SHA-256 preimages. Equal score alone
admits many distinct questions, states and selected answers. Elapsed seconds do
not provide a stable reverse discriminator.

Evidence: roundtrip, material-lookup and identity tests in retention.rs;
formal/Retention.lean restored_answer assumes its encode/decode identity and
does not establish it for the full Rust student. Completeness and stable recovery
are only claimed for the definitions as relations; no finite enumerator, unique
reconstruction, collision proof or reconstruction of parser outputs is supplied.

## identities

### Specialized obernant: identity partitions

Type: identity_partition. Subject/revision: manifest.json.
Status: partial exact dependency projection, with hash limitations.
Domain: serializable Lesson values; observation: one fingerprint/digest lookup.
Operator: partition lessons by selected tuple equality, then apply SHA-256.

    A(L)=(course,n,tests,exams)
    T_n(L)=(course,n,explain,facts,exercises)
    assessment_fingerprint(L)=H(A(L))
    material_digest(L,n)=H(T_n(L))

A preserves labels and distractor order as well as seed-bearing lesson number.
T ignores all tests/exams and uses its explicit n argument. Both discard filename,
title, level and stars. A also discards teaching; T also discards the caller's
placeholder number when retained_lesson substitutes a ledger key. The domains
are field projections, not semantic equivalence of natural-language statements.

With identical tuple bytes the digest is necessarily identical. Distinct tuples
are predicted to differ under the usual collision assumption, not a theorem of
injectivity. Neither model recovers tuple contents from the hash.
retained_lesson returns the smallest matching stored number, rather than proving
the content exists at only one ledger number.

Practical use: a title change should not force regrading identity; changing option
order or n can change shuffled presentation and must bind a new assessment.
Changing an exercise changes teaching identity even if explanations stay fixed.
A question revision does not by itself force rereading retained teaching.
Equal item counts establish neither assessment identity nor question equality.
Identity does not include inference-engine version, so comparisons need an
external code/environment binding in addition to the state and assessment hashes.

Evidence: retained_material_is_found_across_placeholder_ids_and_assessment_changes
and assessment_identity_models_scored_inputs_and_ignores_unread_metadata in
../retention.rs. No exhaustive collision search or semantic identity check.

## invariants-cost

### Invariants and cost approximations

Subject/revision: manifest.json. Status: partial source review; no new benchmarks.
Assume valid initial states, unchanged dependencies, ordinary successful I/O and
bounded string/key comparison costs when using the asymptotic shorthand below.

Load invariants are its explicit gates: ledger-backed source lines, frame-backed
source provenance, bounded quote/cause/next indices, weight-vector layout and
unique map-backed vector keys; kind sources allow KINDS_SRC. Frame::link is local
metadata and is deliberately not checked as a global index. No DAG, consistency
of statements, authenticity of teaching or positive lesson number invariant is
established for arbitrary checkpoint contents.

Successful exam preserves the payload hash; collision resistance strengthens
this to payload identity. Caches are outside it. Neutral focus is preserved by
new/load/train and must be an exam entry precondition for directly edited students.
Training rejects stale teaching versions for an existing number. Practice avoids
read() and ledger insertion but performs exercise learning; the exact effect of
learn_lesson is delegated. Save uses exclusive creation, not a transactional
rename; a failed save or second split write may leave artifacts.

Let B be serialized payload bytes, F frames, E entity occurrences including
conjoined IDs, K quote/cause/next edges, V distinct indexed keys, T text keys,
R kind relations, W weights, L ledger entries and M serialized teaching bytes.
- fingerprint/save: O(B) serialization/hash and space O(B), with additional cloned
  payload storage; save adds O(B) writes plus unmodeled sync latency.
- load: O(B) parse/hash, O(T log T + R log R) collection/validation, frame provenance
  O(F log T), and roughly O((F+E+K) log(V+F)) index work plus mcode cost. String
  lengths and BTree key comparisons may dominate this shorthand.
- retained_lesson worst case: L serializations/hashes, O(L*M) for fixed-width n;
  early match shortens the scan. next_lesson_number follows the ordered map's
  maximum-key lookup plus checked arithmetic.
- exam: two full payload fingerprints, question/option preparse, Q answers and
  witnesses, row serialization; O(B)+C_parse+sum(C_answer+C_why)+output bytes.
  No claim that it is constant in retained graph size.
- train: preparse + optional read + learn_lesson with two configured passes,
  payload-independent weight comparison O(W); dependency costs remain symbolic.

formal/Retention.lean states repeat cost initial + r*(load+evaluate) and saving
(r-1)*replay-r*load-save versus repeated replay. This predicts a benefit only
when that expression is positive for measured costs; it proves algebra under
assumptions, not a runtime speedup. Source-reviewed costs are prospective.
Evidence: implementation, model.md#tests and that Lean file; no timing or test result
was produced by adding this sidecar.

## nonbehavior

### First obernant: behavior complement

Type: behavior_complement. Subject/revision: manifest.json. Status: partial
source-derived inner bound, conditional on the assumptions below; no machine proof.
Domain: finite, well-formed Rust Lesson/Retained values and filesystem inputs;
observation is one call's return tag, durable payload projection and reported
identities. Let U contain all candidate (input,prestate,return,postpayload)
tuples on this domain, including both success and error outcomes. Let R be
actual terminating transitions of the pinned code and trusted dependencies.
This is a transition complement, not unbounded trace reachability.

Define BadTrain as labels present OR teaching empty OR course mismatch OR
(existing number AND (digest differs OR practice=false)) OR
(absent number AND practice=true).
Define BadExam as course mismatch OR teaching/exercises present OR questions empty.
Define BadLoad as any failure of the explicit schema/hash/layout/provenance/
edge/duplicate checks in model.md#approximant, after successful deserialization.

The following set C is an inner approximation to U minus R:
1. A train tuple with BadTrain and an Ok result.
2. An exam tuple with BadExam and an Ok result.
3. A load tuple with BadLoad and an Ok result.
4. A completed successful exam tuple whose before/after payload hashes differ.
5. A successful save tuple opening an already existing path under ordinary
   create_new semantics and without external filesystem races altering the premise.
6. A successful retained_lesson tuple returning a stored number whose recomputed
   digest differs, or returning a later match when an earlier key matches.

This supplies membership rules for absent behavior across the declared domain;
negative tests witness selected members but do not define C. It is deliberately
not the complement of an approximate parser or answer model. Unknown transitions
stay outside C. Rejections need not identify every defect: first failing gate wins.

Assumptions: normal termination, no adversarial dependency or filesystem behavior;
hash equality is used literally in rule 4. Stronger absence of changed payload
with unchanged hash additionally assumes collision resistance. No exclusion of
transient-cache mutations is asserted. The public student can be changed by a
caller, so neutral exam focus is not unconditional. Corrupt but self-consistently
rehashed content passing the listed gates is not proven unreachable.

Evidence: source guard order and named tests in model.md#tests, especially structural
invalidity with a valid hash. No exhaustive domain checker, formal Rust
correspondence or newly executed negative control accompanies this model.

## persistence

### Specialized obernant: persistence and information loss

Type: persistence_information_loss. Subject/revision: manifest.json.
Status: partial source-derived projection specification. Domain: valid retained
states whose payloads meet load gates, with unchanged implementation/dependencies.
Observation horizon: save followed by load, no external modifications. Operator:
N(S)=D(pi(S)). Observed output: restored payload and derived indexes.

The persisted coordinates are exactly payload(): course, lesson digest ledger,
frames, alias map, source texts, role weights, parts/isa/per relations and the
fingerprint of the dictionary store the state learned with (empty in older
states). Load refuses a non-empty fingerprint that differs from the process's
store; an empty one adopts the current store. meanings_layer projects the isa and
alias relations (lesson-provenanced) into a derived dictionary layer from the
payload alone; it reads no lesson and forgets frame contents and weights.
Neither cached parses/reads nor the live lookup indexes are serialized.
Define S ~ T iff pi(S)=pi(T). Every equivalence class forgets differences in
omitted state; N chooses a reconstructed representative. N is not a historical
undo: caches and earlier exercise trajectory are unrecoverable from the payload.

On payloads emitted by save from valid states, map-backed vector fields have
canonical key order. Assuming rebuild helpers do not mutate persisted fields,
pi(N(S))=pi(S) and N is idempotent up to the observed projection. This is a
conditional model, not full student equality or a proved answer equivalence.
For hand-authored accepted envelopes, texts/parts/per vectors may be out of key
order; load collects them into ordered maps. A later payload() canonicalizes
their order, so the incoming payload checksum need not equal fingerprint() of
the loaded state. A valid envelope checksum is not itself a canonical state ID.

Reconstruction folds frames in order: by_ent records entity occurrences,
by_line groups frame sources, prev/effect reverse next/cause edges, sayer inserts
quote owners (later insertions overwrite earlier owners), and mcode builds codes.
Load range-checks edges but does not prove acyclicity, unique quote ownership or
semantic agreement of aliases. Arbitrary direct student edits may violate the
preconditions and save does not validate them before writing.

Practical implication: compare payload fingerprints for canonical retained state,
and compare answers/witnesses under neutral focus to test semantic restoration.
The existing retention_probe removes seconds before live/cold comparison; it
also measures a blank baseline. Those are proposed applicable checks here, not
newly measured evidence. A fingerprint alone neither authenticates an author nor
proves the artifact learned its contents legitimately.

Evidence: payload(), load() and state_roundtrip_retains_weights_aliases_and_kind_graph_without_caches
in ../retention.rs; ../../../../world/examples/retention_probe.rs; the conditional
restored_answer theorem in ../../../../formal/Retention.lean.

## wave34-evidence

### Wave34 knowledge evidence

Ten new scoped groupants across two existing Rust sources, six source-specific typed views each. Five inline helper oracles added to knowledge.rs; retention uses five existing synthetic tests. Original source bytes preserved exactly; only the cfg(test) module is appended.

Ten exact named world::lib tests pass twice debug and twice release (40 positive invocations). Sorted target-qualified outcomes agree across all four profiles/trials. Three production mutants compile and are rejected by their named runtime tests: citation predicate inversion and preparse cardinality bypass fail direct assertions; removing the kind sentinel exemption causes a Result::unwrap panic on the unexpected provenance Err at retention.rs:500. The latter is a compiled behavior rejection via runtime panic, not a direct assertion rejection. Cross-review supersedes the original all-three-assertions wording while preserving the raw receipts. Each is restored byte-for-byte and its selected test passes again. Compile failures and assertion-only edits: zero. The initial citation pilot also passed.

Syntax for both sources generated twice and byte-compared. Current source hashes match syntax and every descendant manifest; shared syntax is not duplicated in children. Structural report has only two expected ancestor bindings for knowledge.rs (mova and self-study), for integration to refresh. No production repair or optimization was made.

No corpus, actual lesson, teacher, network, model loading, training, frozen suites or successful exam was run. Tests use literal strings, synthetic callbacks or hand-built graphs and temporary files. Lesson replay is not credited as retained knowledge. No observed-error percentage surrogate is justified; parser/inference behavior, teaching truth, cold-exam quality, I/O fault recovery and full semantic correspondence remain gaps. Existing models and epoch memories are retained as historical evidence.

## History

### 2026-10-29 · descriptant

#### Epoch 2026-10-29

Source reviewed at db990569444b5e4af84174c16e8227645a44a755; SHA in manifest.
Existing hand-built tests support payload roundtrip and selected rejection
boundaries. No new test execution, Lean verification or timing evidence here.

Useful observations retained: payload equality excludes transient caches;
load canonicalizes map-backed vectors; checksum validity is not authorship;
public student mutation makes neutral focus conditional; report counters alone
do not prove learned improvement; failed file writes are not rolled back.
The existing exam rejection test never reaches successful question evaluation.
These limits motivate future targeted checks rather than a completeness claim.

The central retained-student registry entry already exists and remains unchanged.
No AST is fabricated, no full inverse is claimed, and no performance result is
inferred from the conditional algebra in formal/Retention.lean.

#### Bounded oracle epoch 2026-10-29

Four proposed relations now have executable bounded witnesses in retention.rs:
canonical payload preimages, lost live indexes/focus, smallest compatible lesson
antecedent, and allocation frontier. Predictions, fixture bounds, independent
expectations and negative controls are preserved in model.md#tests. The eight existing
retention tests remain in the focused suite. No production semantics changed.

The first draft of the information-loss fixture assumed default focus was false;
the actual default is true. An initial fixture assertion caught this, and the
second antecedent now sets false explicitly. This was a test setup correction,
not a production bug. The oracle distinguishes omitted state before checking
that loading selects the neutral, reconstructed representative.

Persisted-value equality is checked directly alongside SHA-256. Eight wire
payload hashes can correspond to one canonical state. Matching material at
several ledger keys is an actual non-singleton fiber; lookup chooses its minimum.
Allocation uses the maximum and deliberately leaves holes. These observations
narrow model ambiguity without claiming semantic learning or full inversion.

### 2026-10-29 · epoch-memory

2026-10-29: six fragments expose checkpoint order, lossy canonicalization,
validation gates, index rebuilding, ordered reuse, and question-only evaluation.
The existing twelve tests are bounded hand-built fixtures. The named exam test
rejects teaching before parsing; it does not establish a successful fresh-process
exam. No new production Rust, teacher call, training run, GPU work, full semantic
AST or general proof is included. Existing historical observations remain in the
file's descriptant and tests.

### 2026-10-29 · document

Subject: ../retention.rs, revision and SHA-256 in manifest.json. These sixteen
models extend the four existing models; manifest order keeps the two required
baseline types first. Each heading is its manifest ID; types and scopes are there.
All entries are partial specifications, not Lean proofs. Four bounded executable
oracles added in the 2026-10-29 oracle epoch are recorded below and in model.md#tests;
the remaining entries retain source-derived status. Unless narrowed below, the
horizon is one terminating call, with the pinned serializer/dependencies, valid
Rust values and ordinary
filesystem behavior. Parser, learner and answer internals remain opaque.
Proposed oracles remain future checks unless an executed oracle is named below.
Negative cases are witnesses for relations defined here, not the models themselves.

### 2026-10-29 · tests

Integration check, 2026-10-29: cargo test --release -p world --lib
knowledge::retention passed all eight selected tests. This includes the existing
exam rejection case; successful cold-exam correspondence still needs its own
coverage and is not inferred from the rejection test's name.

#### Bounded oracle epoch, 2026-10-29

Prediction before execution: permutations of the three map-backed vectors form
one canonical payload fiber; stale lookup indexes and focus lie outside the
persisted projection; reuse selects the least matching key; allocation follows
the maximum key even when lower holes exist. Production semantics are unchanged.

| Executable oracle (same module) | Independent observation |
|---|---|
| oracle_canonical_payload_fiber_preserves_values_not_vector_order | Entire 2 x 2 x 2 vector-order product: eight wire digests, one exact JSON payload/fingerprint; changing source text remains visible |
| oracle_checkpoint_fiber_erases_stale_indexes_and_focus | Two unequal live states share a checkpoint projection; literal entity/verb/edge index maps and false focus are reconstructed; a second load is idempotent in the observed projection |
| oracle_reuse_fiber_selects_smallest_of_multiple_antecedents | Ledger digests constructed from explicit tuples, without material_digest; matches at 3/7/19 select 3 across four placeholders, then 7/19 after deletions; facts/course changes distinguish inputs |
| oracle_allocation_frontier_ignores_holes_and_rejects_boundaries | Five literal successful frontiers, plus distinct reserved-number and overflow errors; sparse holes are not allocation targets |

The load helper only seals and writes the candidate envelope, invokes real load,
and removes the temporary file. It supplies no reconstruction logic. Tests compare
full canonical JSON as well as hashes; they do not infer unique preimages from a
hash. The index oracle uses explicit expected maps, not a second rebuild loop.
All fixtures are hand-built; no lesson parsing or learner execution is involved.

Negative controls temporarily mutate four independent production sites in an
isolated worktree: clear loaded per relations; restore focus as true; traverse
the lesson ledger backwards; allocate after the minimum key. Running the four
oracle tests together must report all four FAILED (not compilation failure).
The mutations affect distinct fixtures/observations and the source is restored
before the final positive runs. This is sensitivity evidence for these specific
wrong implementations, not mutation-complete coverage.

Reproduce positive checks with:
`cargo test --release -p world --lib knowledge::retention -- --test-threads=1`
using a separate CARGO_TARGET_DIR. The source hash in manifest.json binds the
unmutated version. Broad school accuracy, successful cold exams, GPU behavior,
I/O fault recovery and all unsampled relations remain unmeasured.

Measured result, 2026-10-29: all 12 focused release tests passed before mutation;
the four oracle tests each failed under their respective simultaneous independent
mutation; after exact source restoration, all 12 passed twice with identical
ordered test outcomes. The final runs reported 0.01 and 0.00 seconds of test time;
this is fixture execution time, not a performance benchmark. Existing unrelated
compiler warnings remain. rustfmt --check and git diff --check passed.

mova-modelcheck changed against base d46b1842a reports the changed Rust file
registered with no missing changed files. Groupant validation reports only the
expected stale retention source bindings in the parent self-study/root manifests;
the integration owner refreshes those centrally. No leaf diagnostic was reported.

#### Nested fragment epoch, 2026-10-29

The production Rust SHA-256 remains
3e9f4683e3f0be15924b4c968b3654a7d90583903cbe4cc27b737c85329fa9ed.
Six source-range fragments add thirty typed models while retaining the original
twenty. Each new family lists behavior complement first and reverse reconstruction
second, then three specialized relations. The checkpoint node composes three
children; file-level composition binds checkpoint, material and exam.

Executed on CPU with a separate target directory and two build jobs:
the twelve focused release tests passed twice, with identical ordered outcomes
(0.00 and 0.01 seconds fixture time). Those timings are not a performance claim.
No mutations were run in this decomposition epoch; the historical controls above
remain scoped to their earlier run. Existing compiler warnings remain.

The groupant inventory accepted all six new fragments with no diagnostics.
It still reports 520 Rust files missing sidecars: this decomposition increases
fragment/model counts, not file coverage. Source ranges and artifact sections
were validated structurally, without semantic proof. Successful examination,
fresh-process retention, quote-owner multiplicity, all provenance/error partitions
and I/O fault injection remain gaps. No live teacher, GPU, lesson training,
school benchmark or Lean execution was performed.

### 2026-10-29 · complementer

Bounded execution, 2026-10-29: four oracles listed in model.md#tests now exercise the
canonical checkpoint preimage, omitted-state fiber, multiple matching lesson
antecedents and allocation boundary. Literal index/frontier expectations and
explicit independently constructed lesson tuples keep these checks separate from
the corresponding production loops. They supply finite witnesses and rejected
mutants, without extending the relations to unobserved states.

### 2026-10-29 · identities

Bounded execution, 2026-10-29: oracle_reuse_fiber_selects_smallest_of_multiple_antecedents
constructs three matching ledger entries from explicit serialized tuples, without
calling material_digest. Four placeholder values with changed title/test/exam
fields select the smallest entry; removing entries advances the selected witness.
Changed facts yield None and changed course is rejected. This witnesses multiple
compatible teaching antecedents without assuming a unique historical lesson.

### 2026-10-29 · persistence

Bounded execution, 2026-10-29: oracle_canonical_payload_fiber_preserves_values_not_vector_order
checks every ordering of three two-entry vectors (eight distinct accepted payload
digests), exact canonical JSON equality, and a value-changing counterexample.
oracle_checkpoint_fiber_erases_stale_indexes_and_focus constructs unequal live
states with equal projections, checks literal rebuilt entity/verb/edge maps and
neutral focus, then repeats load to check projected idempotence. These establish
finite witnesses of ambiguity and normalization, not full-state inversion.
Mutation controls and limits are recorded in model.md#tests.

### 2026-10-30 · descriptant

#### Checkpoint audit, 2026-10-30

Four synthetic oracles exercise resealed validation partitions, all three kind
provenance fields, repeated entity occurrences and last quote ownership, and
the distinct serializer/integrity/authentication boundaries. See model.md#audit for
the relations and model.md#tests for measured evidence. No production change was
justified: accepting cycles and unchecked ledger strings is existing v1 scope.
Current source-bound partial syntax was generated twice; existing fragment
ranges retain identical production bytes. The projection forgets transient
state and wire vector order, so no full-memory inverse is claimed.

2026-11-04 (add-on modules): the payload gains `licence` (skipped when empty, so states of an unmarked store keep
their bytes and fingerprints): the share-alike or non-commercial module licences of the dictionary store the state
learned with. `save` and the `meanings` export call `dictstore::Store::guard_derived_write` (no such output inside a
Git work tree or under a built-in file name), and the exported layer's header carries `derived_header` instead of
«ours» when such a module is loaded. The 20 retention tests pass unchanged; the mark and guard have their red
controls in the dictstore tests (the process-wide store cannot hold a module inside these tests).

### 2026-10-30 · document

#### audit-epoch-2026-10-30

The production checkpoint/material/exam byte ranges are unchanged; four tests
are appended outside those ranges. Current root syntax is generated twice and
byte-compared, without expanded macros or semantic AST claims. model.md#audit adds
finite evidence for coupled validation and reconstruction, without adding new
model types or changing the twenty file-level model identities. Load validation
plus reconstruction implies canonical payload normalization, not authenticity
or full live-state inversion. The existing eight wire-order witnesses remain
part of each focused test run.

### 2026-10-30 · tests

#### Trust-boundary audit, 2026-10-30

The four added oracles are:

| Check | Independent oracle and boundary |
|---|---|
| oracle_resealed_rejections_partition_validation_gates | Eleven single-defect envelopes with correct hashes, exact error strings, two paired-defect order witnesses and byte-preserving load |
| oracle_provenance_and_topology_have_distinct_trust_boundaries | Literal 3-by-4 kind-source table, frame sentinel rejection, accepted self/cyclic edges with literal reverse maps |
| oracle_restore_occurrences_and_quote_ownership_are_ordered | Literal occurrence lists, omitted light/empty verbs, line grouping, last quote owner and unequal-payload owner fiber |
| oracle_checkpoint_integrity_is_not_authentication_or_validation | Existing bytes survive save rejection, resealed unauthenticated testimony/ledger accepted, malformed shape/hash rejected, invalid public graph saved then rejected by load |

All call real serialization/loading and synthetic retained graphs. The rejection
helper seals a candidate and observes the returned error; it implements no
validation oracle. Success checks use literal graph fields/maps and exact file
bytes in addition to hashes. Arbitrary digest strings and cycles are accepted
trust-boundary witnesses, not new production features. The old eight-order
canonical-vector oracle and independent reuse-tuple oracle also remain in every
focused run. See model.md#audit for the complement-first/preimage-second relations.

The audit corrects two model ambiguities: compatible live projections satisfy
P(s)=C(p), where C=P composed with D, for accepted noncanonical wire p; a direct
save cannot emit that p. Error-message preimages are unions of failing gate
occurrences with the same message. Neither checksum nor error text determines
a unique history.

CPU results: all 16 focused tests passed in debug twice and release twice, then
passed twice in each profile again after a compiled negative control was removed.
Ordered per-test outcomes were compared across all eight runs; elapsed times
are excluded from the equality claim. The mutant changed quote ownership from
last writer to first writer. The new ordered-index oracle failed its literal
sayer-map assertion after compilation; original source bytes were restored.
This is one sensitivity witness, not comprehensive mutation coverage.

Reproduction uses cargo test -p world --lib knowledge::retention
with --test-threads=1 and a separate target directory; add --release for release. The external run directory is
$MOVA_DATA/runs/octopus-next-retention-2026-10-30. It holds raw logs, results.json,
the mutation runner, syntax copies, changed/groupant reports and registry-entry.json.
No live parser, teacher, training, school benchmark, successful cold exam, GPU
or Lean execution was performed.

Current syntax was generated twice with mova-modelcheck syntax and compared
byte-for-byte. All six existing fragment intervals were compared to the base
production bytes; appended tests leave those intervals unchanged. The manifest
binds the current whole source and partial syntax. This does not establish
expanded macro or semantic AST coverage. Groupant validation reports only the
expected stale retention bindings in the shared mova/self-study parents, which
the integration owner refreshes. Changed-file registration, rustfmt and diff
whitespace checks passed. Successful exam passivity, full-state equivalence,
numeric-code semantics, general malformed input and I/O fault recovery remain
unmeasured.

Scoped Clippy (-p world --lib --tests --no-deps) reached both library targets
but failed on existing clippy::modulo_one at world/src/idioms.rs:157 and
clippy::never_loop at world/src/know.rs:298. Each appears once per target.
The only retention diagnostics were the existing type_complexity warnings at
lines 48 and 50, again once per target; no new-test diagnostic was reported.
This is not a clean whole-world Clippy result.

External evidence SHA-256:
- results.json: 7a2d37113bb02acc0cda5849b8443db1ecedd340a7780b8df28429314b3447a6
- mutant.log: 3e76e4dab3ac31814fd46f79b7816d2edfe14057e45b7a6aabcd28720b463ec9

These hashes bind this finite execution record, not a general correctness proof.

### 2026-10-30 · audit

Subject/revision: manifest.json; observation epoch 2026-10-30.
Domain: finite decoded v1 envelopes and synthetic retained states, fixed Rust
serializer/runtime dependencies, ordinary filesystem operations without races.
The four new oracles call actual save/load against temporary files. Their
literal expected errors and maps do not duplicate the reconstruction algorithm.
No parser, teacher, training, successful exam or inference-quality claim follows.

### 2026-10-31 · wave34-evidence

Evidence SHA-256: native-receipts.json a3330bf15cd1b7901bac4a7fd4d7522a4c6b72bf02f08e17e07b541416acc496; mutation-receipts.json bc6c2f106735fa5f515618efc2138f08a363693fc0233db417836f0cfbda1151; production-delta.json f688ac9cb00e9d64eccac438b3463ce60cffd297bfd86b882b0b0c92c98500a1. External raw receipts are in $MOVA_DATA/runs/octopus-wave34-knowledge-2026-10-31.
