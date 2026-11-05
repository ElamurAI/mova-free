# en-conllu-document

## descriptant

### en-conllu-document

Doc::read reads UTF-8 then calls parse with path context. parse tracks leading blanks, per-sentence blank counts, final newline and exact raw comments/ten columns; CR is rejected and comments after rows fail. After complete lexical scanning, refresh each sentence, then derive global layer flags. text joins retained lines/tabs/LF without consulting derived tokens.

## approximant

### Forward and cost

Doc::read reads UTF-8 then calls parse with path context. parse tracks leading blanks, per-sentence blank counts, final newline and exact raw comments/ten columns; CR is rejected and comments after rows fail. After complete lexical scanning, refresh each sentence, then derive global layer flags. text joins retained lines/tabs/LF without consulting derived tokens.

Let B be input bytes, R rows, W syntactic words and F feature parse work. read costs O(B+F) plus file-wide token counters and O(B+W) retained projection storage. Doc parse/text and refresh are linear in traversed bytes/rows plus gram parsing and cloning; exact enum parser costs are delegated. Atomic refresh adds an O(R) temporary index vector alongside old derived views. No dimension cap or fallible allocation interface; usize counters/length arithmetic have no separate overflow certificate.

No executable approximate parser or supported 1/2/5/10% semantic profile exists. Exact round-trip and failure effects are categorical obligations.

## invariant

### Scoped invariant

For accepted unedited text, serialization excludes any byte change, including blank runs and missing final newline. Strict grammar is partial CoNLL-U validation: ranges/empty nodes and HEAD remain largely uninterpreted. Public edits may create states outside the parse image, so text is not a validator.

The pair parse/text is a left inverse on accepted input strings. Exact raw data separates strings that the Sentence projection conflates. Arbitrary mutated Doc states can serialize identically despite different derived views and can serialize invalid grammar.

Finite f64 arithmetic is not used. Counters and indexes are usize; no declared resource ceiling, allocator recovery, stream rollback or concurrency guarantee. Public fields can break postconditions after return.

## models

## behavior_complement

For accepted unedited text, serialization excludes any byte change, including blank runs and missing final newline. Strict grammar is partial CoNLL-U validation: ranges/empty nodes and HEAD remain largely uninterpreted. Public edits may create states outside the parse image, so text is not a validator.

## reverse_reconstruction

The pair parse/text is a left inverse on accepted input strings. Exact raw data separates strings that the Sentence projection conflates. Arbitrary mutated Doc states can serialize identically despite different derived views and can serialize invalid grammar.

## round_trip

Split only one final LF marker, count remaining blank lines, store exact raw columns/comments, then join with LF and restore final marker. Synthetic fixture includes leading/interior/trailing blanks, empty string, no final LF, ranges and decimal rows.

## strict_lexical_layer

CR, comment after a row and column count !=10 are lexical errors with line context; otherwise all rows are collected before semantic refresh begins. Consecutive IDs/known relations are semantic requirements, not full graph validity.

## comment_only_zero

Comment-only nonblank runs are FullSentence entries with zero words, unlike read. Global W0 makes has_feats/lemmas/forms true for those entries. Wholly empty input has no entries and final_newline=false.

## raw_vs_derived

text uses comments/rows/blank metadata exclusively. Editing derived sent or words alone cannot change output; raw invalid HEAD can round-trip while tree_gate later rejects it.

## tests

### Bounded evidence

Four conllu::full_tests tests twice debug/twice release with --skip full_round_trip_all_english_treebanks. Two compiled repair mutants fail wave26_training_range_state_is_sentence_local and wave26_refresh_error_preserves_derived_view; exact source restoration then four-test pass. Parent receipt.json binds external logs and source. No external corpus coverage or full English suite is claimed.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: root baseline witnesses rejected incomplete-range cross-sentence leakage and partial words publication after refresh failure. Repairs preserve permissive policies and raw data. Local validation uses four bounded synthetic tests; all-treebank test skipped. No corpus, timing, model or database operation.
