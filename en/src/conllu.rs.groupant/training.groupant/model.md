# en-conllu-training

## descriptant

### en-conllu-training

Training-file projection streams BufRead::lines (LF/CRLF accepted by line handling), ignores short rows (<10 columns), keeps first ten of extra columns, skips rows containing range or decimal ID markers, accepts nonconsecutive/malformed word IDs, maps unknown tags/UPOS to None and drops unknown feature pairs. Unknown word relation fails, with base-before-colon fallback. Last sent_id/text comment wins, even after words.

## approximant

### Forward and cost

Training-file projection streams BufRead::lines (LF/CRLF accepted by line handling), ignores short rows (<10 columns), keeps first ten of extra columns, skips rows containing range or decimal ID markers, accepts nonconsecutive/malformed word IDs, maps unknown tags/UPOS to None and drops unknown feature pairs. Unknown word relation fails, with base-before-colon fallback. Last sent_id/text comment wins, even after words.

Let B be input bytes, R rows, W syntactic words and F feature parse work. read costs O(B+F) plus file-wide token counters and O(B+W) retained projection storage. Doc parse/text and refresh are linear in traversed bytes/rows plus gram parsing and cloning; exact enum parser costs are delegated. Atomic refresh adds an O(R) temporary index vector alongside old derived views. No dimension cap or fallible allocation interface; usize counters/length arithmetic have no separate overflow certificate.

No executable approximate parser or supported 1/2/5/10% semantic profile exists. Exact round-trip and failure effects are categorical obligations.

## invariant

### Scoped invariant

Each blank separator resets range_end=0 and range_space=true, including incomplete ranges. A returned token cannot inherit range spacing from a prior sentence. read emits only nonempty token sentences. Comment-only metadata can remain in cur across blank separators until later tokens, an unchanged policy gap; this repair resets range state only.

The many-to-one projection admits changes to DEPS, ignored extra columns, unknown features and skipped rows without changing tokens, subject to spacing/comment/layer effects. Permissive acceptance cannot be inverted to a canonical valid CoNLL-U source.

Finite f64 arithmetic is not used. Counters and indexes are usize; no declared resource ceiling, allocator recovery, stream rollback or concurrency guarantee. Public fields can break postconditions after return.

## models

## behavior_complement

Each blank separator resets range_end=0 and range_space=true, including incomplete ranges. A returned token cannot inherit range spacing from a prior sentence. read emits only nonempty token sentences. Comment-only metadata can remain in cur across blank separators until later tokens, an unchanged policy gap; this repair resets range state only.

## reverse_reconstruction

The many-to-one projection admits changes to DEPS, ignored extra columns, unknown features and skipped rows without changing tokens, subject to spacing/comment/layer effects. Permissive acceptance cannot be inverted to a canonical valid CoNLL-U source.

## range_boundary

Range line end b sets range_end (invalid end->0), SpaceAfter substring controls range_space. IDs before end suppress spaces; equal end receives range-space and clears end. Blank unconditionally resets both. No range completeness or overlap proof.

## projection_loss

Raw DEPS, full MISC, range/empty nodes, unknown features and most comments are absent in Sentence. Relation subtype falls back to the base only if exact enum parse fails.

## metadata_and_empty

Token-free cur is not pushed or taken on blanks; comment metadata may carry to later tokens. Empty or comment-only file yields no sentences. EOF emits remaining tokens only.

## layer_threshold

One whole-file pass counts mapped-known FEATS and raw non-underscore lemma/form. Each emitted sentence gets the same >=5% flags; feature annotations dropped by gram parsing do not count.

## tests

### Bounded evidence

Four conllu::full_tests tests twice debug/twice release with --skip full_round_trip_all_english_treebanks. Two compiled repair mutants fail wave26_training_range_state_is_sentence_local and wave26_refresh_error_preserves_derived_view; exact source restoration then four-test pass. Parent receipt.json binds external logs and source. No external corpus coverage or full English suite is claimed.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: root baseline witnesses rejected incomplete-range cross-sentence leakage and partial words publication after refresh failure. Repairs preserve permissive policies and raw data. Local validation uses four bounded synthetic tests; all-treebank test skipped. No corpus, timing, model or database operation.
