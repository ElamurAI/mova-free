# prag-eval

## model

# prag-eval

## forward
Evaluation composes supplied training rows, one classifier per Field::ALL entry, ordered predecessor feedback and metric/report projections. The executable domain here is aligned supplied rows and valid six-model sets. run and label additionally load external annotation models and files, enforce source licenses and produce reports; these orchestration paths are not executed.

## invariant
Dictionary coordinates, field ordering and sample indices must agree at every join. Scores now rejects unequal lengths and out-of-domain classes before computing any summary. Reporting time is an observation, not deterministic state.

## cost
For N rows, S slots, C classes: scores remains O(N*C) time and O(C) temporary space; rejection adds O(N) time. Fit cost is sum of six child fits. Ordered prediction costs O(N*S) encoding plus six predictions per selected index. No speedup claimed.

## behavior-complement
Within valid supplied-input traces, an output at an unselected key, use of an unprocessed predecessor prediction, or a metric accepting invalid class indices is excluded. External report failures and classifier-internal behavior are outside this partial complement. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
A score pair leaves confusion counts and sentence identities ambiguous; many gold/pred sequences yield (1,1). Set observations retain kind and six field order but do not recover training keys or learned inactive weights. Reverse composition must retain the same Dict across fitting and sequence prediction. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Hold supplied classifiers fixed and vary idx order: previous-state availability can change predictions. This is a sequence-order intervention, not a new training result.

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

### 2026-11-04 · document

#### epoch 2026-11-04 (Labeler)
`label` is now `Labeler::train` (the perceptron fitted once on the silver run, features read with a supplied annotator) followed by `Labeler::label` over (sent_id, text) pairs; the rows, their order and the header are the old `label`'s. A sentence's context is only its own document (`roles::doc_key`), so labelling documents one list at a time gives the rows labelling them together gives: the large Wikipedia corpus labels article by article with one trained model. Measured: the per-article rows of «Accra» and «Aden» equal the Wikipedia school's whole-corpus `prag.tsv` byte for byte (2026-11-04). The child scopes (training, sequence, scores) lie before the change and are unchanged.

### Undated · receipts

c6a0e3ae0e424ffd6e5883502ce7e55d38d61e53b1184b6160f4236371fecd2e
c67caaddfdeb5b325547cbd3eaaf574047979857dc30c6f1b0513e61054b5671
722abc8a8796e5bf5f2e98c6b37e1c16a9e86b4af3de147255df52b39c1ae206
eb8b6b5db811f8656064935a03b0d543c412bd01bbd4fc386178b583be684ef6
251f06c0cffce8c1be52a77c0cf5595ef22090393252b8df059eef803014045d
