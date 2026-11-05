# peta-store

## descriptant

### Peta store epoch

Review found two actual boundary mismatches: parser files may omit #text, and ID-only joins can select witnesses from another teacher revision. Text metadata now requires exact ID/token binding, and witnesses require a committed batch containing the selected teacher tree/prag hashes. Dev text aliases are excluded at export without deleting evidence. These are integrity improvements, not gold adjudication.

Quarantines retain accepted IDs lost and explicit recovery; no blind replay. Changed eligibility requires deliberate new policy/store. Partial source syntax and seventeen focused test executions provide bounded evidence.

## model

# Peta store models

Source-derived partial models; immutable trusted snapshots, cooperating SQLite callers and ordinary storage semantics are assumptions. No formal proof, numerical surrogate or gold-annotation claim.

## forward

Content-bound reviewed seed and durable receipts enter SQLite WAL. Immediate transactions append whole batch/version/selection joins or roll back and record terminal quarantine separately. Exact known receipts are no-ops; changed bytes under a key refuse. Teacher pairs and actual parser/agreement witnesses retain separate roles. First committed eligible whole pairs stay selected; conflicting later revisions remain rejected evidence.

Eligibility requires current verified school explain/fact, historical training_allowed=false retained, no protected ID/exact text or operator quarantine, valid integer tree and known nonreserved targets. Dev iff sha256(lesson)[0] mod 10=0. Export drops train exact-text aliases of dev without deleting DB records. Other valid pairs can be stored ineligible. Create-new exports bind a consistent read snapshot, teacher pairs and actual three-parser agreement inputs. Missing raw #text is supplied only after exact ID/token-form join; predictions never synthesized.

## composition

Receipt/config/source/queue pins -> strict pairs -> serialized SQLite append -> stable selection -> consistent read snapshot -> same-selected-teacher-batch witness join -> train/dev text exclusion -> actual treeagree inputs. Trainer owns replay, real agreement, CUDA, diagnostics and activation. Store invokes neither teacher nor GPU and does not rewrite historical file DB. Child context-exclusion files have their own campaign hashes, distinct from primary protected metadata.

## invariant

One immutable receipt SHA per batch key. Selected key resolves to eligible version with exact split; reusing formerly ineligible key refuses. Version key binds source/ID/tree/prag hashes. Export has at most one whole selected pair per ID, excludes dev aliases from train and never overwrites a directory. Manifest marks completion; partial directory is incomplete.

## cost

Strong phase/current/heldout/source/queue hash/index scans each ingest: O(input bytes) time and O(index metadata) memory. Known receipts rehash bytes but do not reparse artifacts. New versions incur indexed SQLite joins/fsync, no per-batch gigabyte legacy snapshot. Status/export scan selection/relevant witnesses. No safe hash cache or measured end-to-end gain claimed.

## behavior_complement

Domain: fixed trusted snapshots and normal SQLite/I/O. Observe eligible selection/export, not rejection metadata. Invalid phase/source/tree/prag, protected ID/text, quarantine or unknown targets cannot become eligible. Whole-batch failure can quarantine otherwise valid accepted IDs; lost-ID counts and recovery instructions preserve this gap. Counterexample: checked receipt is not linguistic gold. Evidence: rejection, missing-pair, malformed and ineligible-reuse controls. Partial excluded-transition model; no arbitrary-crash proof.

## reverse_reconstruction

An exported selected pair admits every retained committed receipt/version with its exact tree/prag key plus same-revision witnesses. Two receipts can yield the same output: inverse is a fiber. Export alone cannot reconstruct hidden teacher computation or alternate history; DB retains provenance separately. Evidence: multi-batch, conflict and immutable-export controls. Arbitrary corruption recovery absent.

## witness_revision_fiber

Type: join_preimage. For selected hashes (t,p), usable witness w requires a committed batch b containing w and teacher v with v.id=w.id, v.tree_sha=t, v.prag_sha=p. Restrict actual three open-parser sources and exact token forms. Counterexample: matching ID in a conflicting VERB-teacher batch cannot supply a witness for selected NOUN-teacher revision. Text-header adaptation changes metadata only. Evidence: unrelated_teacher_batch_cannot_supply_witnesses and no-text fixtures; partial relation, no parser-equivalence proof.

## transaction_error_partition

Type: error_partition over ingestion trace. Precommit failure rolls back all version/selection writes; separate quarantine retains error and accepted IDs lost. Commit exposes complete joins; repeat no-op, mutation refuses. Counterexample to malformed receipt blocking all work: unchanged malformed records quarantine once, later good jobs progress. Evidence: concurrent writers, missing-pair and malformed controls. SQLite/storage crash semantics assumed; power-loss injection absent.

## training_set_preimage

Type: reverse_reconstruction. With fixed selection/split, protected H and dev text set X, K=eligible_train minus denials and text aliases in X. Many excluded histories yield K; K alone cannot reveal historical checkpoint exposure. Counterexample to prefix-only eligibility: changed teaching text or exam phase fails. Evidence: cross-lesson text alias, phase/question/target controls. Safe replay and assessment provenance remain downstream responsibilities.

## tests

### Native checks

Offline locked standalone harness compiles actual store.rs and directly reused prag/src/schema.rs with at most two build jobs. Ten store tests cover multi-batch/idempotence/frozen export, actual no-text parser metadata, missing pairs, phase/question/heldout exact-text/target rejection, conflicts, concurrent writers, changed receipts/policy, same-selected-teacher witness lineage, unrelated-source generation stability, malformed-receipt terminal progress, ineligible same-key denial and cross-lesson train/dev text alias exclusion. Seven reused schema checks cover categorical transport.

## History

### 2026-11-01 · descriptant

2026-11-01: parallel annotation needs appendable serialized commits, retaining the reviewed seed in a separate SQLite WAL store. Whole-pair selection and immutable exports distinguish new evidence from source-model authority.

### 2026-11-01 · tests

All tests use local temporary SQLite DBs and fixtures, no teacher/live DB/CUDA/assessment labels. Debug and release receipts are external under $MOVA_BIG/runs/peta-store-2026-11-01. These do not establish linguistic gold, arbitrary crash durability or historical model exposure; parent integration owns real data/GPU checks.
