# prag-data

## model

# prag-data

## forward
Source tags and Item records feed deterministic seeded selection and make_items batching, with JSON-lines persistence and external DuckDB acquisition. This epoch checks pure hash, source/serde and batching observations; database process execution, external input selection and file persistence remain unexecuted.

## invariant
Commercial source provenance must survive record construction. Valid tale windows refer to existing indices and nonempty paragraphs; windows should be selected in document order. Batch maximum is soft for an indivisible oversized window.

## cost
Hash linear in UTF-8 bytes. make_items linear in emitted records and copied text/context. Selection includes sorting and repeated passes, and DB acquisition has external unbounded cost; no measured throughput claim.

## behavior-complement
For supplied valid windows, dropping or reordering selected sentences, splitting one window over batches, or assigning Tale provenance to Tatoeba rows is excluded. This partial complement does not certify SQL, license metadata truth or file error handling. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
Items identify selected text and source but omit unused input rows and most selection history. Batch numbers alone admit many window partitions; a seeded hash cannot uniquely recover its input string. Reverse composition retains matching source tags and copied keys. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
The first item of each window may carry up to two same-document earlier input rows, including rows not selected for annotation. That context is not an additional training sentence.

## composition
Children share source identity and dictionary/sample or record indices. Parent order and domains constrain joins; leaf observations alone do not establish external orchestration. Overlapping scopes must not double-count costs.

## checks
Seven wave14_models tests: four evaluation and three data; supplied inputs only. Debug twice and release twice; compiled production mutation per source must fail a named assertion. Syntax generated twice identically. Receipts summarize final evidence.

## epoch-memory

First coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.

## limitations
Partial syntax projection, opaque macros, no semantic AST or universal proof. No live parser, DB subprocess, external corpus, teacher, GPU or training job. External IO, report formatting, selection corner cases, malformed Set/Train and empty tale windows remain outside executed scope. No performance claim.

## receipts

### Wave14 evidence

Seven focused tests pass twice debug and twice release with identical normalized named outcomes; restored debug passes. Old-code rejection witness failed on unequal lengths. Compiled eval feedback mutant returned [1,1,1] instead of [1,0,0]; compiled data zero-size batch mutant returned [0,0,1] instead of [0,1,2]. Both restored byte-for-byte before positives.

SHA-256 identities (source, old witness, owned mutation, debug, release):

Syntax projections repeat byte-for-byte. Finite observations only; no external parser, corpus, DB process, teacher or GPU.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.

### Undated · receipts

b3146b6af3ca37c5a64f82c60970bea0420e5f7f62cf38642c305e09b8ce4015
c67caaddfdeb5b325547cbd3eaaf574047979857dc30c6f1b0513e61054b5671
2c6064452b7bab3938918a43a7d282bc13e5797eb4ded82f44a4040598cbf5cc
eb8b6b5db811f8656064935a03b0d543c412bd01bbd4fc386178b583be684ef6
251f06c0cffce8c1be52a77c0cf5595ef22090393252b8df059eef803014045d
