# en-conllu

## descriptant

### en-conllu

Three deliberately different views: read projects a UTF-8 file permissively to syntactic-word Sentence values; FullSentence retains raw rows/comments plus explicitly refreshed derived words/sent; Doc parses a stricter LF/column/comment grammar and serializes retained raw data exactly when unedited. No function establishes a dependency tree. Tokens keep mapped gram enums and parsed known features; serde views omit raw unknown fields.

Format reference: Universal Dependencies, CoNLL-U Format, https://universaldependencies.org/format.html. Current implementation accepts a documented subset/superset by entry point; the format reference is not evidence of full conformance.

Structural validation: a source-only inventory sandbox containing this exact source and complete sidecar passed mova-modelcheck groupants (one file groupant and three nested fragments). Global registry/ancestor wiring remains the integration owner's check.

## approximant

### Forward and cost

Three deliberately different views: read projects a UTF-8 file permissively to syntactic-word Sentence values; FullSentence retains raw rows/comments plus explicitly refreshed derived words/sent; Doc parses a stricter LF/column/comment grammar and serializes retained raw data exactly when unedited. No function establishes a dependency tree. Tokens keep mapped gram enums and parsed known features; serde views omit raw unknown fields.

Let B be input bytes, R rows, W syntactic words and F feature parse work. read costs O(B+F) plus file-wide token counters and O(B+W) retained projection storage. Doc parse/text and refresh are linear in traversed bytes/rows plus gram parsing and cloning; exact enum parser costs are delegated. Atomic refresh adds an O(R) temporary index vector alongside old derived views. No dimension cap or fallible allocation interface; usize counters/length arithmetic have no separate overflow certificate.

No executable approximate parser or supported 1/2/5/10% semantic profile exists. Exact round-trip and failure effects are categorical obligations.

## invariant

### Scoped invariant

The accepted Doc domain excludes CR, non-ten-column rows, comments after rows, nonconsecutive syntactic-word IDs and unknown word relations. It does not exclude malformed range IDs, malformed decimal node fields, invalid raw heads, multiple roots, cycles or out-of-range heads. read admits still more lexical forms. A failed refresh excludes changed derived words/sent but does not undo caller raw edits.

Doc::parse(t)?.text()==t uniquely recovers accepted original bytes from an unedited parsed Doc; there is no converse for arbitrary publicly mutable Doc structures. Sentence projections have many antecedents: different DEPS/MISC, comments, unknown features, range/empty rows and malformed heads mapping to zero can share one view. Existing sent/words alone cannot reconstruct raw rows.

Finite f64 arithmetic is not used. Counters and indexes are usize; no declared resource ceiling, allocator recovery, stream rollback or concurrency guarantee. Public fields can break postconditions after return.

## composition

### Composition and boundaries

Training read and Doc are alternative input policies, not equivalent parsers. Both use gram mappings but Doc routes raw rows through refresh before whole-file layer detection. refresh's atomic commit preserves previous derived views on error; serialization still exposes retained raw caller edits. Raw HEAD validation through tree_gate is a downstream obligation, never implied by head=0 fallback. Byte preservation belongs to Doc's accepted unedited text fiber only.

Source types/Row accessors and embedded tests remain parent-modeled; three fragments intentionally focus on training read, FullSentence refresh/accessors and Doc parse/text. Arbitrary public-field mutation, allocation/I/O fault injection and external corpus behavior remain gaps. No composition-wide approximation accuracy follows from local correctness.

## models

## behavior_complement

The accepted Doc domain excludes CR, non-ten-column rows, comments after rows, nonconsecutive syntactic-word IDs and unknown word relations. It does not exclude malformed range IDs, malformed decimal node fields, invalid raw heads, multiple roots, cycles or out-of-range heads. read admits still more lexical forms. A failed refresh excludes changed derived words/sent but does not undo caller raw edits.

## reverse_reconstruction

Doc::parse(t)?.text()==t uniquely recovers accepted original bytes from an unedited parsed Doc; there is no converse for arbitrary publicly mutable Doc structures. Sentence projections have many antecedents: different DEPS/MISC, comments, unknown features, range/empty rows and malformed heads mapping to zero can share one view. Existing sent/words alone cannot reconstruct raw rows.

## file_layers

Layers are computed over all returned syntactic words: nonempty parsed known FEATS, lemma != underscore, form != underscore; present iff count*20>=W, inclusive 5%. read emits no sentence at W0; comment-only Doc sentences exist and all three flags become true because 0>=0. Refresh preserves existing flags rather than recomputing a file-wide decision.

## head_gate_boundary

Both read and refresh parse raw HEAD using usize.parse().unwrap_or(0), so malformed raw strings can look like roots in the derived view. Doc keeps the raw string losslessly. Consumers needing trees must validate raw heads through tree_gate; Doc acceptance and a derived zero are not a certificate.

## failure_precedence

read opens/streams the file, skips short and range/decimal lines before relation parsing. Doc performs the whole lexical scan before refreshing sentences, so a later lexical error can precede an earlier semantic ID/relation error. Refresh checks each word ID before that word relation. I/O errors propagate, vector allocation may abort; no callbacks are invoked.

## finite_checks

Four bounded tests cover synthetic byte round trips, lexical/semantic rejections and two repaired state transitions. Embedded full-treebank test is explicitly skipped. Passing finite fixtures neither proves the parser for every UTF-8 string nor observes external corpora.

## tests

### Bounded evidence

Four conllu::full_tests tests twice debug/twice release with --skip full_round_trip_all_english_treebanks. Two compiled repair mutants fail wave26_training_range_state_is_sentence_local and wave26_refresh_error_preserves_derived_view; exact source restoration then four-test pass. Parent receipt.json binds external logs and source. No external corpus coverage or full English suite is claimed.

## prospective

### CoNLL-U state model before repair

Root owns en/src/conllu.rs and its new adjacent groupant. It is an unregistered producer of FullSentence consumed by conversion. Model three scopes: permissive training-file projection; row-to-derived-view refresh; strict lossless Doc parse/render. Doc acceptance does not establish a valid dependency tree: raw heads still pass through tree_gate.

Two source-derived failure hypotheses: read keeps a multiword range accumulator across blank sentence boundaries when a range is incomplete, leaking spacing into the next sentence; refresh clears and partially rebuilds self.words before later validation can fail while self.sent remains old. Create witnesses against the old source before accepting repairs. Fix by resetting range state at every blank boundary and by constructing both derived views locally, committing words/sent only after successful validation. Raw rows/comments remain caller-owned edits on failure; this is atomic derived-view replacement, not full sentence rollback.

The reader remains permissive about short lines, unknown features, malformed numeric heads and extra columns; the strict Doc parser keeps its separate grammar and round-trip promise. Preserve error ordering and raw serialization. Empty and comment-only documents, multiword/empty nodes and global layer thresholds remain explicit. read cost is input text plus feature parsing; refresh costs row/token parsing and cloning, with an extra temporary word-index vector to preserve the old view until commit. This correctness tradeoff needs a small synthetic timing observation; no speed gain is predicted.

New integration observations will join raw Doc rows to tree_gate and the functional-graph decomposition on one-based token heads with virtual vertex zero. Distinct API policies and failure effects stay tagged. Source syntax remains partial; no corpus or external data is read. Synthetic temporary files are owned by tests and removed by guards.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: root baseline witnesses rejected incomplete-range cross-sentence leakage and partial words publication after refresh failure. Repairs preserve permissive policies and raw data. Local validation uses four bounded synthetic tests; all-treebank test skipped. No corpus, timing, model or database operation.

### 2026-10-30 · prospective

Heading `### CoNLL-U state model before repair — 2026-10-30` stays in prospective as `### CoNLL-U state model before repair`.
