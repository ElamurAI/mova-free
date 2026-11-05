# prag-eval-sequence

## forward
Fold idx left-to-right over an initially empty map. Encode rows[i], overwrite previous-act coordinate using emitted predecessor class[1] or start, predict all six fields, insert/replace out[i]. Caller order is authoritative, not automatically topologically sorted.

## invariant
Only already-emitted keys may supply feedback. Missing or future predecessors use start. Duplicate indices overwrite their earlier map result. Caller rows and prev are read-only. Domain requires valid row indices, six classifiers and valid act class.

## cost
O(K*S + sum of 6K prediction costs), O(U*prediction-size) map space for U unique selected indices; input order retained only during execution.

## behavior-complement
Future-key feedback, implicit sorting, filling unselected keys, or changing non-previous feature coordinates is excluded. These assertions concern the wrapper and do not constrain arbitrary user-supplied Classifier internals. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
An output map loses processing order and overwritten duplicate results; compatible histories retain dictionary, predecessor graph and classifier transitions. An observed start-based prediction cannot distinguish absent predecessor from not-yet-visited predecessor. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
With a supplied act classifier keyed only on the overwritten slot, order [0,1,2] yields [1,0,0], reversed order yields all ones; singleton [2] also yields one.

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
