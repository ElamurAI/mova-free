# prag-vnconv-wordnet-selection

## forward
For each index line, reject punctuated/digit lemmas; collect listed offsets. Gate first offset by event lexfile. Search eligible senses in listed order; in each choose + pointers to verbs with source 0 or matching noun word, target through saturating_sub(1), no underscore and deverbal predicate true. Maximize common Unicode-scalar prefix, strictly greater so ties retain first pointer. Stop at first sense with a winner. Select frames with word 0 or target, classify and sort/dedup before writing.

## invariant
Domain: bounded parse counts and offsets, valid supplied UTF-8. Same-word verbs survive deverbal. The heuristic compares scalar-prefix counts with byte lengths in its verbalisation threshold; no linguistic or Unicode-normalization claim. Target zero currently selects first word by saturating subtraction and is not rejected.

## cost
Candidate work proportional to visited senses, pointer counts and spelling lengths; frame sorting O(F log F). Duplicate index lines remain duplicate output rows. Output/map storage grows with retained words and emitted rows.

## behavior-complement
Given fixed valid index/maps, choosing a lower-prefix candidate within the same sense, continuing to a later eligible sense after a winner, or including nonmatching word frames lies outside the forward relation. Selection is not a claim about true nominal semantics. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
Every index/data triple giving the exact rows is compatible, including added rejected pointers and later senses. An empty result admits no first event sense, no suitable link, rejected verbalisation or missing target data; these failure causes remain ambiguous. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
The two nominal fixtures separately witness first-linked-sense precedence, within-sense prefix ranking, first-sense event gate, source-word mismatch, verbalisation exclusion, transitive frame scoping and frame sorting/dedup.

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
