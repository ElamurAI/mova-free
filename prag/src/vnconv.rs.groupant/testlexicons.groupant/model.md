# prag-vnconv-testlexicons

## forward
PropBank folds predicate/roleset/role/alias/example events, emits rolesets on closes and collects unique class links/prepositions. NomBank folds NOMLEX sexps for NOM/NOMING/NOMLIKE and noun rolesets. sexps uses character tokens with lists, quoted escapes and semicolon comments. positions maps DET-POSS->poss, N-N-MOD->nn, PP PVAL->lowercase strings, preserving first occurrence. convert labels and writes their tables only under test.

## invariant
Only supplied sexps/positions are executed. Corpus file traversal, PropBank roleset conversion, NomBank full rows and routing IO are source-read descriptions, not behavioral coverage. Position uniqueness is first occurrence; unknown heads contribute nothing. File errors may leave partial outputs.

## cost
sexps O(B) scalar storage/tokenization; positions O(P^2) worst-case linear duplicate searches plus string bytes. Full converter cost includes per-file event materialization, linear duplicate searches and output copies; filesystem latency is not modeled.

## behavior-complement
For the supplied NOMLEX position relation, output missing poss/nn, duplicate of, uppercase OF, or sorted rather than encounter ordering is excluded. For full test-lexicon conversion this remains a partial source-derived hypothesis with no executed corpus evidence. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
Position output loses unknown heads, duplicate occurrences, capitalization and all other property-list fields. Many sexps share the same positions. TSV reverse candidates must retain roleset/alias join identity but cannot recover ignored source annotations. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
License-destination projection keeps permissive and test-only output arguments distinct. This is a source inspection of call destinations, not an executed filesystem or legal provenance guarantee.

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
