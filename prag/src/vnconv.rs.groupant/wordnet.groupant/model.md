# prag-vnconv-wordnet

## forward
nominals reads index.noun, data.noun, data.verb and calls nominal_rows unchanged after checksum accumulation. The pure function builds offset maps, selects noun rows in index line order, derives verb choice, scopes frames to the selected word, sorts/deduplicates frame IDs and accumulates counts by event file. The wrapper adds attribution/header and writes one file.

## invariant
First noun sense must map to EVENT_FILES. Verb linkage uses first eligible sense with any surviving candidate, maximum spelling prefix within that sense and first pointer on ties. Object flag is based only on word-applicable frames. Map ordering does not reorder index output.

## cost
O(W log S + sum candidate scans + sum F log F), plus string copying and map storage O(W). malformed huge counts and arithmetic overflow in index skip are outside bounded fixture domain. IO and checksum additional O(B).

## behavior-complement
Candidate rows from a non-event first noun sense, a later better sense overriding an earlier linked sense, or frames belonging only to another verb word are excluded in the valid bounded domain. Revision: this source hash. Observation: pure supplied-input outputs. This is a partial source-derived exclusion model with finite witness evidence, not a universal proof.

## reverse-reconstruction
Nominal rows omit unused senses/pointers, exact noun offset, rejected candidate spelling, glosses and input ordering of frames. All maps/index histories yielding the selected tuple remain compatible; offset and frame evidence constrain but do not uniquely recover them. The target is the full compatible fiber under forward and its stated domain, without unique recovery or an implemented complete solver.

## specialization
Transitivity is a membership observation in the explicit TRANSITIVE_FRAMES set, not a semantic guarantee that the noun takes an object in every sentence. Tests use only original tiny supplied strings.

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
