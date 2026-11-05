# prag-vnconv-xml

## forward
Scan UTF-8 text at ASCII tag/quote boundaries. Emit Open(name, ordered attributes,selfclosing), Close(trimmed name), Text(trimmed unescape(text)); skip comments, declarations and processing instructions. Attribute lookup returns the first matching key. Replace lt,gt,quot,apos,amp once in that order. cell collapses Unicode whitespace and replaces '|'; lemma_of lowercases, changes underscores to spaces, and splits a final recognized particle. Sum folds bytes with wrapping FNV-1a.

## invariant
Quoted '>' does not close a tag. No XML validation, namespaces, numeric entity decoding, CDATA or DTD interpretation. Unterminated comments disappear; unterminated quoted tag attributes are retained to end. Duplicate attributes are first-wins. Literal '&amp;lt;' becomes '&lt;', not '<'. All supplied strings are valid Rust UTF-8.

## cost
O(B) scanning and a constant five replacement passes; O(B) event/string storage. Attribute lookup is O(A); filename sorting O(F log F) is unexecuted. checksum O(B), O(1) accumulator.

## behavior-complement
For supported tag fixtures, candidate events that split at quoted '>', expose skipped comment text, recursively decode '&amp;lt;', or normalize numeric '&#65;' to A belong to U minus the modeled relation. They are excluded behavior, not claims of XML compliance. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
The event sequence admits alternate whitespace, comments and entity spellings. A cell output admits arbitrary runs of original whitespace and both '/' or '|' at a replaced separator. lemma 'bring out' admits 'BRING-OUT', 'bring_out' and 'bring out'; inverse is not unique. A checksum fiber includes boundary partitions of the same concatenated bytes. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Encoding-boundary relation: tag slicing advances over bytes until ASCII delimiters, while WordNet source-target tokens require independently checked UTF-8 ranges. These are different byte-boundary mechanisms; no claim that the reader accepts invalid UTF-8 bytes.

## composition
Children share the exact source and intermediate event/word identities. XML feeds the same class fold used by production; restriction expressions route through the active target. WordNet maps and index offsets must agree before selection and word-scoped frames. Reverse composition retains those intermediates jointly, not independently chosen marginals. Overlapping costs are counted once.

## checks
Seven vnconv::wave15_models tests exercise tiny original in-memory strings. Named old-code Unicode panic witness, compiled semantic-buffer mutation, exact restoration, twice debug and twice release with identical normalized outcomes; syntax generated twice. See root receipts.

## epoch-memory

Prospective review predicted unchanged valid XML/WordNet rows and no speed gain. A single checked range adds O(1) validation to pointer parsing. Public verbnet_xml and private nominal_rows expose existing production algorithms with no duplicate parser. Old pointer slice failed at byte 2 of a euro sign. Positive fixtures preserve quoted delimiters, literal escape behavior, nesting, source reset, first-sense semantics and index order. Partial syntax and source hashes identify this epoch, not semantic truth.

## limitations
This is a small tag reader, not general XML. No corpus files, live parser/teacher/GPU, converter jobs or filesystem writes executed by tests. PropBank/full NomBank conversion, external error paths and attribution truth remain unexecuted. Malformed huge counts, XML structural validation, universal Unicode linguistic handling and full lexicon correctness remain outside the finite domain. No speed, error-cap or full-system proof claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective review predicted unchanged valid XML/WordNet rows and no speed gain. A single checked range adds O(1) validation to pointer parsing. Public verbnet_xml and private nominal_rows expose existing production algorithms with no duplicate parser. Old pointer slice failed at byte 2 of a euro sign. Positive fixtures preserve quoted delimiters, literal escape behavior, nesting, source reset, first-sense semantics and index order. Partial syntax and source hashes identify this epoch, not semantic truth.
