# prag-vnconv-verbnet-restrictions

## forward
Stack elements are (is_or,items). open pushes; item appends Value concatenated with type to top, or overwrites done if stack empty. close pops: an empty group contributes nothing; join with '|' for or else '&'; if nested and multiple items, parent receives parentheses; top-level completed groups combine with '&'. take clears stack and consumes done.

## invariant
No logical simplification or precedence normalization. Parentheses are inserted only for nested multi-item groups. Closing an empty stack is a no-op. Literal signs and types are retained; incomplete pending groups are discarded by take.

## cost
O(K+L) stack operations plus copying expression strings at each ancestor; worst-case O(D*L) copied characters for depth D and final expression length L. O(D+L) live expression storage excluding events; no measured optimization.

## behavior-complement
Given the nested conjunction/disjunction fixture, flattened '+animate&+human|-solid' or reordered signed leaves are excluded candidates. Empty-group punctuation and a restriction leaking into a later target also violate the modeled take transition. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
The same expression can come from nested singleton groups or no singleton nesting, and omitted/empty groups leave no output. Reverse reconstruction retains all grouping stacks whose closes yield the string; precedence alone cannot identify original XML. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Restriction-target routing sends completed strings only to active role/NP/PREP; PREP without a value renders '{restriction}', while a value renders '{value}[restriction]'. Core stack logic is tested through a real role and syntax NP.

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
