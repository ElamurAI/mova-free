# prag-vnconv

## model

# prag-vnconv

## forward
For ordered supplied documents D, V(D)=render(fold append_verbnet over D). A directory wrapper sorts XML filenames, accumulates raw UTF-8 checksum, and invokes exactly these functions. WordNet nominal rows N(index,noun,verb) project first eligible derivational senses. convert routes VerbNet to out and PropBank/NomBank to test; nominals writes permissive rows to out. Only the pure supplied-input paths are executed here.

## invariant
Class parent indices refer to the active per-document stack; no stack or partial frame crosses documents. Class rows precede their own frames. External license declarations and checksum provenance are preserved text, not independently certified provenance. File writes are sequential, not transactional.

## cost
Let B be total supplied bytes, E events, R emitted row bytes, and W WordNet tokens. XML scanning O(B), materialized events O(B), retained classes and rows O(B+R); repeated string joins may copy intermediate restriction strings. WordNet uses ordered maps O(W log S) plus candidate scans and frame sorting. External IO is additional and unmeasured.

## behavior-complement
Universe U consists of supplied input/candidate row tuples with representable counts. The partial complement includes pairs (D,y) where y has a child parent from a previous document, a frame before its class row, or a nominal from a first non-event sense. These are concrete mismatches to the projected relation; unrestricted XML validity and external write traces remain unknown. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
V^-1(y)={D:V(D)=y} and N^-1(y)={(i,n,v):N(i,n,v)=y}. Preserve all candidate documents, intermediate events and lexical maps satisfying the projection. Comments, ignored fields, unused senses, whitespace and alternative entity spellings create nonunique fibers. The FNV accumulator cannot reconstruct document boundaries. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Source-boundary intervention: append an unfinished class/SYNTAX document followed by a clean document. The second class has parent '-' and only its own syntax; class accumulation persists, parser state does not.

## composition
Children share the exact source and intermediate event/word identities. XML feeds the same class fold used by production; restriction expressions route through the active target. WordNet maps and index offsets must agree before selection and word-scoped frames. Reverse composition retains those intermediates jointly, not independently chosen marginals. Overlapping costs are counted once.

## checks
Seven vnconv::wave15_models tests exercise tiny original in-memory strings. Named old-code Unicode panic witness, compiled semantic-buffer mutation, exact restoration, twice debug and twice release with identical normalized outcomes; syntax generated twice. See root receipts.

## epoch-memory

Prospective review predicted unchanged valid XML/WordNet rows and no speed gain. A single checked range adds O(1) validation to pointer parsing. Public verbnet_xml and private nominal_rows expose existing production algorithms with no duplicate parser. Old pointer slice failed at byte 2 of a euro sign. Positive fixtures preserve quoted delimiters, literal escape behavior, nesting, source reset, first-sense semantics and index order. Partial syntax and source hashes identify this epoch, not semantic truth.

## limitations
This is a small tag reader, not general XML. No corpus files, live parser/teacher/GPU, converter jobs or filesystem writes executed by tests. PropBank/full NomBank conversion, external error paths and attribution truth remain unexecuted. Malformed huge counts, XML structural validation, universal Unicode linguistic handling and full lexicon correctness remain outside the finite domain. No speed, error-cap or full-system proof claim.

## receipts

### Wave15 conversion evidence

Seven named vnconv::wave15_models tests pass twice debug and twice release with identical normalized identities/results. Original tiny supplied-input strings only. The old source failed malformed_unicode_wordnet_pointer_does_not_panic at a UTF-8 boundary. A compiled production mutation replacing adjacent semantic deduplication with sem.clear failed verbnet_nested_restrictions_and_row_order; production bytes restored exactly before the four positive runs.

Two syntax projections match byte for byte. Two complete-registry structural reports match byte for byte; changed-source registration passes using an external temporary registry, without editing shared parents. An initial leaf-only registry report rejected unrelated sidecars because the inventory scans the whole repository; the complete scaffold resolves ownership. No production changes arose from that harness correction.

#### Limits

Finite observations and source-bound partial syntax, not a general XML parser proof. No actual corpus files, conversion jobs, live parser, teacher, GPU or external writes. PropBank/full NomBank and filesystem branches remain unexecuted. Seven nested fragments describe the supported joins and gaps. No optimization, throughput or error-cap claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective review predicted unchanged valid XML/WordNet rows and no speed gain. A single checked range adds O(1) validation to pointer parsing. Public verbnet_xml and private nominal_rows expose existing production algorithms with no duplicate parser. Old pointer slice failed at byte 2 of a euro sign. Positive fixtures preserve quoted delimiters, literal escape behavior, nesting, source reset, first-sense semantics and index order. Partial syntax and source hashes identify this epoch, not semantic truth.

### Undated · receipts

#### SHA-256 receipts

- old.log: c7672df9193a4cc761f6640347f19f4e345f7863b2290ee96b53200ecc8e2350
- mutant.log: 03b5cc7bb7196524482609c9f5bfd919762d7fa83b389eff3b0f6d70b25f03c0
- debug-1.log: 686a7979fd36705b767fd9ced7f9362bfd9fedcadc7d9c5c41ce41ef0337377e
- debug-2.log: e9ee8debe85c3dbf67a532a74609a350ca3106168b5ec6257479b96773b27600
- release-1.log: 392debfb5c0b460ec885dd6f845ad54c39a8e2d66d66028d9d78522fe3a5cdff
- release-2.log: 19c83e32ba4f9a23217ff46527b49cb9646e150bbce6ea7fb3291f702c703fa1
- normalized.txt: b0d073cb9301c57889d46672112ec58b90985f5c15eef6fab8437ed57c0eb591
- syntax-1.json: e1d4852214a39f969795de836509650affc133484407ef51f0f52fa96fbeefe3
- syntax-2.json: e1d4852214a39f969795de836509650affc133484407ef51f0f52fa96fbeefe3
- full-groupants-1.json: a1ef0e211a9084b7267e2f88c8dfbf0d9de20b295109527e11076db3ad06fe01
- full-groupants-2.json: a1ef0e211a9084b7267e2f88c8dfbf0d9de20b295109527e11076db3ad06fe01
- changed.json: 4bd3f15538a2c69c3006da3e4149154fe7d18ce27cd99950ce4bee6621247b5e
- vnconv.rs: b5e9e5a2a3fc9359403585f2a0b1ffacc4ecb71a563d252d1ea334a76695ae52
