# cycle-reviewed

## descriptant

### Reviewed DB selection

#### Descriptant

Prospective requirement: source-weight votes can revert explicit corrections or mix old tags with new heads. Persist full reviewed tree/prag snapshots with separate user authorization and train/dev decisions. Require exact content-bound joins and recover all DB files together.

Synthetic native evidence demonstrates an old Opus-weight vote winning without policy; the same history preserves the complete correction after two agrees with policy. Joined sentence IDs and newdoc/version preambles retain per-sentence ownership. Rejection, repeat, split and interrupted rollback run on temp files only. Models and syntax are partial; no semantic/training-quality claim.

Final admission review found legacy block parsing could discard orphan fragments. A strict pre-scan now excludes them; the parent must revalidate staged promotion against the updated executable.

## approximant

# Forward relation

Validate unique teaching-only ID pairs, structure/tokenization/text, previous/current/final/prag/phase SHA256 and explicit train/dev partition. Original phase training_allowed remains false. Decisions bind the snapshot transaction name. Snapshot all DB files, inventory hashes, stage seven managed outputs, journal PREPARED -> APPLYING -> COMMITTED around individual renames. Identical repeat never appends. Rollback verifies before hashes and restores all files under the quiescent premise.

## Composition

cycle CLI -> promote/rollback; db update/update_prag/agree -> policy -> full-tree overlay after ordinary voting; training_file and trainer fresh selection -> training_allow -> train partition minus caller heldout IDs. Directly include the existing prag schema to reuse its closed-set parser. No teacher/GPU call or semantic adjudication.

Headerless token/comment fragments and malformed sent_id headers now reject before legacy block parsing, preventing unpaired fragments from silently disappearing.

## catalog

# Typed observations

## behavior_complement

Domain: quiescent, finite, operator-owned input files, ordinary I/O semantics and normal return. Observe managed DB-file bytes and the promotion result separately from lock/staging metadata. Let U contain proposed input/output transitions; R contains actual successful promotions. If gate g_i is false (phase/header/hash/structure/tokenization, school prefix, heldout/quarantine flag, unique pairs or split), the modeled inner exclusion is {(x, committed-output): not g_i(x)} subset U minus R. This is conditional on the actual validators and trusted metadata; a forged internally consistent phase record is outside independent provenance assurance.

Before APPLYING, admission/staging failure does not change managed DB data, although incomplete transaction directories can remain. After APPLYING, crashes can leave a prefix of seven renames and the retained lock blocks cooperating normal callers. Counterexample to the overbroad claim 'error means no filesystem changes': staging artifacts and partial application. Oracle: malformed/hash/phase/question/split controls in reviewed_selection_transaction_and_training_gates and malformed_orphan_fragments_fail. Evidence: executed bounded controls; power-loss/storage durability is not proved.

## reverse_reconstruction

Domain: a transaction with a complete, hash-verified before inventory B and quiescent rollback. Observe the complete map of relative filenames to bytes, excluding retained transaction receipts. The byte reconstruction is rollback_B(state)=B; ownership/permission metadata, external model state, intent and linguistic correctness are outside the observation. Current trees alone have many compatible prior histories, so no unique inverse of ordinary promotion is asserted.

Counterexample: equal current trees with two distinct version histories cannot identify the earlier history without B. A post-promotion legitimate write is also removed by point-in-time rollback. Oracle: interrupted-lock synthetic rollback and external six-file inventory plus streaming byte comparisons. Evidence: both finite checks passed; arbitrary crash injection remains absent.

## selected_vote_override

Type: partial_exclusion specialized to whole-revision selection. Domain: validated policy S=(S_tree,S_prag), current C already matching S, fixed history H, and a successful agreement that retains all selected IDs I. Observe complete selected blocks, not just dependency heads. Let A_H be legacy voting and O_S the whole-tree overlay. Then projection_I(O_S(A_H(C)))=S_tree; pragmatic projection_I remains S_prag. The composed model excludes a successful selected hybrid with new heads but old tags/lemmas/features.

Assumptions: cooperating callers, unchanged selected text, no missing selected IDs and successful I/O. This is an identity of the modeled projection, not equivalence of all DB states. Counterexample to omitting O_S: a stronger old Opus version wins; copying heads alone can retain incompatible tags. Oracle: the native negative control first observes old voting win, then checks whole blocks after two agrees. Campaign evidence: all 3,416 selected full-tree/prag hashes survive staged agreement. Status: partial source derivation with synthetic and bounded real-data evidence.

## train_partition_preimage

Type: reverse_reconstruction specialized to training eligibility. Domain: one reviewed primary DB with all selected IDs present, no secondary DB, explicit train set T, exclusion E, and successful training_file. Observe output ID set K, ignoring file order. Forward model K=T minus E. Given K and E, compatible train sets form the fiber {T: T minus E=K}; excluded IDs' membership is unrecoverable from K alone. In particular, many distinct policies with differences inside E produce the same output.

Assumptions: policy integrity and ID-based exclusions. Equal-text aliases and historical checkpoint exposure require independent filtering; this relation says nothing about semantic equivalence or safe replay. Counterexample to 'all selected IDs train': selected dev IDs are outside T. Counterexample to unique inversion: add one excluded ID to T. Oracle: actual training_file native train/dev/heldout controls, plus campaign API output exactly 3,081 train IDs and zero of 335 dev IDs. Status: partial source-derived set model and finite evidence, not a provenance theorem.

## transaction_prefix_partition

Type: error_partition over application trace positions. Domain: one quiescent seven-file application after complete staging; failures occur between completed individual renames, without concurrent modification. Let f_1,...,f_7 be FILES order. At trace position k in 0..7, S_k(f_i)=new_i for i<=k and old_i otherwise. These are distinct trace positions, not necessarily distinct byte states when old_i=new_i. APPLYING admits all such prefixes; COMMITTED follows completed application and policy validation.

The complement of cross-file atomicity includes 1<=k<=6. Counterexample to 'APPLYING means no files changed': k=1 already replaced current.conllu. Counterexample to recovering k from equal bytes: an unchanged pragmatic row/file can hide a completed rename. Recovery uses the complete before snapshot, not guessed k. Oracle: journal/rename source order and the synthetic retained-lock rollback fixture. Status: partial trace abstraction; every prefix, true power loss and filesystem/controller behavior have not been experimentally explored.

## tests

### Checks and limits

reviewed::tests::reviewed_selection_transaction_and_training_gates: synthetic promotion/rollback, actual old-vote negative control, full tags/heads across two agrees, duplicate IDs, stale previous/tree/prag hashes, wrong header, denied decision, forged question/nonteaching phase/heldout/authorization, no-blank boundaries, train/dev/heldout exclusion, conflicting update/prag rejection, exact repeat, retained lock on simulated interrupted commit, byte restoration and removal of later artifacts. joined_preambles_belong_to_following_sentence binds newdoc/version/source preambles to the second sentence. Existing pragmatic schema tests exhaust closed categorical transport.

Run cycle tests with --test-threads=1 because existing fixtures change process environment. Synthetic interruption is not actual power-loss injection or storage-controller proof. No canonical DB/model/teacher/GPU/corpus/heldout evaluation.

malformed_orphan_fragments_fail rejects a structurally plausible headerless token after an otherwise valid tree, an orphan comment fragment and malformed sent_id syntax, while retaining a legitimate comment preamble.

## evidence

### Evidence

The initial seventeen cycle debug tests passed and release source build completed offline/locked with two jobs. A later explicit joined-preamble regression test records the parent review finding. Final rerun/release results and model-check receipts are in the external DB policy result report. Bounded observations do not establish arbitrary filesystem or semantic correctness.

## invariant-and-cost

### Invariant and cost

Selected full trees and pragmatic rows remain content-bound to a durable decision batch. agree restores entire selected trees, including tags/features, after ordinary source-name votes. Conflicting update/prag calls fail before effects. Training admits only the explicit train partition, excluding caller heldout IDs.

Quiescent callers and exclusive promotion lock required. Seven same-filesystem renames are individually atomic; there is no cross-file atomicity. Snapshots and journal provide explicit recovery. SHA256 binds bytes, not operator signatures or phase truth. One batch per fresh DB; exact repeats idempotent.

For B database bytes, S selected sentences and W tokens: O(B + S log S + sum W^2), full-file memory copies and snapshot/staging I/O. Inventory hashing streams files. No throughput bound measured. No justified numeric percentage surrogate for categorical admission/effects.

## History

### 2026-11-01 · descriptant

Heading `#### Epoch 2026-11-01` stays in descriptant as `#### Descriptant`.

### 2026-11-01 · document

Subject: the manifest-bound `reviewed.rs`, composed with `db.rs` and the cycle CLI. These source-derived relations are partial models, not formal proofs. Synthetic native tests and the bounded 2026-11-01 campaign are finite evidence. No numerical approximation/error percentage is claimed.

### 2026-11-01 · evidence

Heading `### Evidence — 2026-11-01` stays in evidence as `### Evidence`.

Bounded application evidence, 2026-11-01: staged agreement retained all 3,416 selected tree/prag hashes; repeated promotion did not append; rollback restored six DB files byte-for-byte. Live import retained 791,385 tree IDs; all 787,969 nonselected normalized blocks and 780,137 nonselected pragmatic rows were unchanged. Existing policy/training APIs admitted exactly 3,081 train and no 335 dev IDs. These are finite integrity observations, not gold annotation judgments. External run storage retains the independent checkers, hashes and receipts; no private data belongs here.

### 2026-11-05 · tests

2026-11-05: the training-gate assertions call training_file with db::Heldout::default() and Heldout::of_ids, the same id exclusion as before.
