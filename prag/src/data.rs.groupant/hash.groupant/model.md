# prag-data-hash

## forward
Start h=FNV_offset xor seed*golden_ratio modulo 2^64. For each UTF-8 byte apply (h xor byte)*FNV_prime modulo 2^64, then splitmix xor-shift/multiply finalization.

## invariant
Arithmetic wraps identically in debug and release. Hash consumes bytes, so Unicode normalization is not performed. Hash is deterministic but not cryptographic and not collision-free.

## cost
O(B) time, O(1) extra space for B UTF-8 bytes.

## behavior-complement
Debug overflow panic, Unicode scalar iteration in place of byte iteration, or ignoring seed is excluded for this exact wrapping function. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
For a fixed seed and unrestricted strings, a 64-bit result has potentially many antecedents. The final xor shifts and odd multipliers are invertible modulo 2^64, but they recover only the pre-finalization accumulator, not string length or bytes. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Independent u128 multiplication with explicit modulo reduction checks six byte strings at seeds 0,1,MAX. Same hash constrains an accumulator equivalence class, not identity.

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
