# prag-eval-training

## forward
train_rows clones rows in idx order, replacing previous-act with gold[prev[i]].act or start. fit_set dispatches six model::fit calls using ys[k], Field::ALL[k].classes(), the same x, keys, dictionary and seed.

## invariant
Input rows remain unchanged. Valid domain requires six aligned label vectors, valid labels, dictionary-compatible encoded rows and keys aligned with x; malformed training shapes are not newly validated.

## cost
Row preparation O(selected*S) cloning plus string bytes. Training is sum_k T(kind,N,S,C_k), with six retained models and learner-specific temporary storage.

## behavior-complement
A prepared row borrowing its own act rather than its predecessor, mutation of input rows, or swapping field labels/names is excluded in this domain. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
Observed prepared start may arise from no predecessor or absent predecessor gold. Original previous-act slot is overwritten and unrecoverable; compatible source rows differ freely in that slot. A fitted model generally has many compatible training histories. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Field-routing oracle supplies constant last-class labels separately for all six fields and checks all three actual learner kinds on a two-row fixture.

## composition
Children share source identity and dictionary/sample or record indices. Parent order and domains constrain joins; leaf observations alone do not establish external orchestration. Overlapping scopes must not double-count costs.

## checks
Seven wave14_models tests: four evaluation and three data; supplied inputs only. Debug twice and release twice; compiled production mutation per source must fail a named assertion. Syntax generated twice identically. Receipts summarize final evidence.

## epoch-memory

First coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.

## limitations
Partial syntax projection, opaque macros, no semantic AST or universal proof. No live parser, DB subprocess, external corpus, teacher, GPU or training job. External IO, report formatting, selection corner cases, malformed Set/Train and empty tale windows remain outside executed scope. No performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.
