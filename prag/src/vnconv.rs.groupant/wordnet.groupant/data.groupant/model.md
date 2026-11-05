# prag-vnconv-wordnet-data

## forward
wn_data tokenizes non-indented lines before ' | ', parses offsets and hexadecimal word counts, then word/lex-id pairs, pointers and optional verb frames. Store Syn by offset in BTreeMap; duplicate offset is last-wins. Source/target pointer indices parse two-byte hexadecimal windows, defaulting invalid/unavailable windows to zero.

## invariant
Word strings are lowercase with suffix marker after '(' removed. Invalid offset/word count skips row. Missing count/frame numeric fields mostly default zero. Checked get now makes malformed Unicode source-target ranges follow the existing invalid ASCII zero fallback; no validation or rejection semantics added.

## cost
O(W log S) map insertion plus tokenization and word/pointer/frame copies; O(W) maps. Finite nonpathological counts assumed; loops driven by untrusted huge word counts remain an explicit limitation.

## behavior-complement
For source-target token '€1', a UTF-8 slicing panic is excluded after the witnessed repair; (src,tgt)=(0,0) is the specified fallback. Valid '0102' maps to (1,2); swapped coordinates are excluded. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
Syn omits glosses, lexical IDs and spelling case; multiple lines normalize to the same record and duplicate earlier offsets disappear. Default zero cannot distinguish absent, malformed ASCII or malformed Unicode pointer components. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Failure-boundary specialization: the old source fails malformed_unicode_wordnet_pointer_does_not_panic at byte index 2 within '€'. The repair changes only the first unsafe slice to checked get; destination parsing already used get.

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
