# en-conllu-refresh

## descriptant

### en-conllu-refresh

FullSentence methods index rows through public words and may panic for invalid callers; label uses sent_id or source line. refresh creates a local Sentence preserving old has_* flags, scans comments then rows, skips range/decimal rows, validates consecutive word IDs and relations, builds local words indices, then assigns both derived views only on success.

## approximant

### Forward and cost

FullSentence methods index rows through public words and may panic for invalid callers; label uses sent_id or source line. refresh creates a local Sentence preserving old has_* flags, scans comments then rows, skips range/decimal rows, validates consecutive word IDs and relations, builds local words indices, then assigns both derived views only on success.

Let B be input bytes, R rows, W syntactic words and F feature parse work. read costs O(B+F) plus file-wide token counters and O(B+W) retained projection storage. Doc parse/text and refresh are linear in traversed bytes/rows plus gram parsing and cloning; exact enum parser costs are delegated. Atomic refresh adds an O(R) temporary index vector alongside old derived views. No dimension cap or fallible allocation interface; usize counters/length arithmetic have no separate overflow certificate.

No executable approximate parser or supported 1/2/5/10% semantic profile exists. Exact round-trip and failure effects are categorical obligations.

## invariant

### Scoped invariant

On returned refresh error, words and sent are unchanged. Rows/comments and other public fields remain whatever the caller edited; old indices may no longer be usable if the caller removed/reordered rows. Atomicity covers derived view replacement, not full object consistency or allocation failure.

Compatible preimages of derived tokens include arbitrary preserved unknown features, DEPS/MISC and skipped node rows; old derived views can coexist with edited invalid raw rows after failure. There is no recovery of prior raw data or transaction log.

Finite f64 arithmetic is not used. Counters and indexes are usize; no declared resource ceiling, allocator recovery, stream rollback or concurrency guarantee. Public fields can break postconditions after return.

## models

## behavior_complement

On returned refresh error, words and sent are unchanged. Rows/comments and other public fields remain whatever the caller edited; old indices may no longer be usable if the caller removed/reordered rows. Atomicity covers derived view replacement, not full object consistency or allocation failure.

## reverse_reconstruction

Compatible preimages of derived tokens include arbitrary preserved unknown features, DEPS/MISC and skipped node rows; old derived views can coexist with edited invalid raw rows after failure. There is no recovery of prior raw data or transaction log.

## atomic_commit

All fallible ID/relation checks precede self.words=words and self.sent=s; temporary state is discarded on error. Explicit arithmetic or allocation recovery is absent.

## index_witness

Successful words[k] points to the original row of the kth consecutive integer word, ID k+1; token and index counts agree. Public mutability can invalidate this invariant after return.

## spacing_and_layers

Range state is local to each refresh call. Same range spacing recurrence as read, but IDs are consecutive. has_* flags are copied from the old Sentence even if edited rows change annotation coverage.

## rejection_order

Comments determine last ID/text. For each syntactic word, ID parsing/consecutiveness fails before relation parsing; unknown relation falls back to base subtype. HEAD malformed->0 remains permissive.

## tests

### Bounded evidence

Four conllu::full_tests tests twice debug/twice release with --skip full_round_trip_all_english_treebanks. Two compiled repair mutants fail wave26_training_range_state_is_sentence_local and wave26_refresh_error_preserves_derived_view; exact source restoration then four-test pass. Parent receipt.json binds external logs and source. No external corpus coverage or full English suite is claimed.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: root baseline witnesses rejected incomplete-range cross-sentence leakage and partial words publication after refresh failure. Repairs preserve permissive policies and raw data. Local validation uses four bounded synthetic tests; all-treebank test skipped. No corpus, timing, model or database operation.
