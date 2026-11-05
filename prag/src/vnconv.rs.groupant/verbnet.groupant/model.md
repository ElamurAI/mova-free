# prag-vnconv-verbnet

## forward
Fold events through (classes,stack,restriction,target,syntax/semantic/example flags,frame buffers). Opening a class appends its ID and active parent. MEMBER normalizes its name. Closing FRAME appends primary, syntax, adjacent-deduplicated predicates and first example text to active class then clears buffers. render emits classes in opening order, each immediately followed by accumulated frames; counts equal vector lengths. verbnet_xml runs this same fold and renderer.

## invariant
For balanced supported input, nested subclass ownership is stack-local; parent frames still follow parent row even when subclass occurred earlier in XML. Semantic dedup removes only consecutive duplicates. Parser does not synthesize closes for selfclosing VNCLASS/FRAME and is not a validator. Empty input gives empty rows and zero counts.

## cost
O(E+R) outside restriction-expression copying and lemma/cell normalization, O(E+B+R) retained events/classes/output. Depth stack O(D). Extraction preserves directory streaming and does not collect all raw files.

## behavior-complement
Pairs (xml,y) with incorrect class/member/frame totals, swapped parent/class order, globally deduplicated nonadjacent predicates, or second example replacing first text are excluded for balanced supported fixtures. Arbitrary malformed XML has no conformant-XML interpretation promised. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
A row tuple forgets original nesting wrappers, comments, extra examples and duplicate adjacent predicate events. Compatible antecedents preserve class opening order and parent IDs but may interleave subclass openings and parent frame events differently, since rendering regroups by class. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Ordering projection distinguishes opening order from event order: a child frame can occur before its parent's frame in XML while parent row/frame precede child row/frame in TSV. The exact four-line literal test witnesses this contract.

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
