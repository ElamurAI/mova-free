# prag-data-batching

## forward
Maintain (batch,cur). Before each whole window of W sentences increment batch and reset cur iff cur>0 and cur+W>max. Copy paragraphs in order with pos/plen; attach preceding two same-doc input texts to first index only. After nonempty tales increment batch once, then Tatoeba k gets batch+k/max(tat_batch,1).

## invariant
Windows remain indivisible; first oversized window exceeds max by design. Tatoeba zero batch size normalizes to one. Context is restricted to previous two input indices and same document, not arbitrary earlier sentences. Domain excludes empty windows/paragraphs and invalid indices.

## cost
O(E+T) record operations plus copied text bytes and at most two context sentences per window; output storage proportional to emitted bytes. Integer overflow on impossible-size aggregates remains outside tested domain.

## behavior-complement
Splitting a window to enforce max, crossing document boundary for context, attaching context to every paragraph head, or sharing final tale batch with Tatoeba is excluded. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
Output records preserve paragraph positions and batch labels but do not uniquely identify window grouping: adjacent paragraphs within a batch may come from one or multiple windows, especially with empty context. Unselected input rows and selection seed remain ambiguous. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Literal fixture checks exact batches [0,0,0,1,1,2,2,3], same-document context clipping, paragraph metadata and zero Tatoeba batch normalization.

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
