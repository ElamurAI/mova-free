# world-poly-extraction

## descriptant

# world-poly-extraction

## Forward

Logical source region "/// Pure extraction" through before "#[cfg(test)]". Select first exact heading after first START and first exact end after that heading; end before the selected start is not a candidate. Repeated headings may cut at the second occurrence. A selected heading is removed; nested headings are ordinary paragraph strings. Oracle: repeated and whitespace-sensitive headings. Paragraph projection splits on blank lines and joins all split_whitespace words with one space. Preimage permits arbitrary runs of blank lines and intra-paragraph whitespace; it does not recover line boundaries. Whitespace-only selection is an error. Order is read UTF-8, remove leading BOMs, locate START, locate END, check ordering, select heading/end, reject empty paragraphs, acquire annotator, encode/write. Earlier errors exclude later effects. Output write is not atomic; no rollback claim after acquisition. Oracle: sentinel unchanged on reversed markers. Parent composition retains shared text/clock/intermediate values. Full-file wrapper and test gaps remain in the root.

## Invariant

Respect source order, retain ambiguity of lossy text projections, and keep observed finite correspondence distinct from universal semantics. Pure functions have no modeled filesystem effects. Error paths before annotator acquisition do not write the output.

## Composition

Compose only through equal intermediate token sequences, paragraphs, quantities, event lists and explicit clock inputs. Parent roots cover every logical source region, including imports/constants and native tests; local fragments isolate useful operations and may overlap. The hierarchy is not a complete call graph. Same numeric answer does not imply same text or event history.

## Cost

Let N be total input bytes, L the line count, B selected bytes, T token characters and Bout output bytes. Whole-call time is O(N+L+B+T+Bout), excluding opaque annotation and I/O latency. Whole-call retained memory is O(N+L+B+Bout) plus token and annotator storage; N accounts for prep retaining the full raw input even when selection is short. Costs are symbolic operation/allocation counts, not measured speedups.

## Limits

Partial syn-2 syntax is source-bound; no macro expansion, semantic AST, name resolution or verified dependency behavior. No automatic reverse solver or production surrogate exists. Poly annotator acquisition/tokenization, detokenization, output encoding and filesystem failures beyond early rejection remain conditional. Trigger talk::civil/now/tz_offset, state::root, global nerves graph, manual data and stdout are dependency boundaries. No activate/Task/drop/fire/episodes/expectation/explain calls are made by the new bounded tests. Extremely large counts, nonfinite floating values, signed timestamp overflow and concurrent log append/read remain outside tested domains. Declared event effects are not executed.

## Epoch memory

## catalog

# world-poly-extraction

Domain: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects.

All relations below are source-derived partial models, conditional on successful allocation and the stated input domain. U is the product of admitted inputs and observations (returns, errors, and explicit effects). R is the declared forward relation. Complement entries are candidate inner bounds of U minus R, not complements certified from an approximate emulator. Converse relations preserve every compatible antecedent; no unique original text or history is claimed.

## nonbehavior

Exclude successful selection without START/END or matching heading, end-before-start marker slicing, and empty paragraph results. On these errors no parser/output effects occur.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## antecedents

P maps back to all books with first matching markers/headings selecting lines whose whitespace projection equals P; retain arbitrary text outside the selected interval. The relation is many-to-one.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## heading_selection

Select first exact heading after first START and first exact end after that heading; end before the selected start is not a candidate. Repeated headings may cut at the second occurrence. A selected heading is removed; nested headings are ordinary paragraph strings. Oracle: repeated and whitespace-sensitive headings.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## whitespace_loss

Paragraph projection splits on blank lines and joins all split_whitespace words with one space. Preimage permits arbitrary runs of blank lines and intra-paragraph whitespace; it does not recover line boundaries. Whitespace-only selection is an error.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## failure_order

Order is read UTF-8, remove leading BOMs, locate START, locate END, check ordering, select heading/end, reject empty paragraphs, acquire annotator, encode/write. Earlier errors exclude later effects. Output write is not atomic; no rollback claim after acquisition. Oracle: sentinel unchanged on reversed markers.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## tests

### Bounded native evidence

Commands: cargo test -p world --lib bounded_models -- --test-threads=1; repeat with --release. New native modules contain five tests per source, including a compiled wrong-model discriminator. Independent finite tables cover partition, headings, malformed extraction, units, Gregorian dates, explicit UTC civil time and event ordering; enumeration covers 1,555 token sequences through length four and 85 unit quantities. The poly public rejection fixture creates a fresh scratch directory and preserves a sentinel output. No live hot-state access or parser acquisition is exercised.

External mutation controls remove the marker-order guard and replace the Gregorian century predicate, then compile and run the corresponding native tests; both must fail, after which originals are restored. Debug/release logs are normalized to sorted bounded test outcomes and compared twice per profile. Syntax projection is generated twice and compared byte-for-byte. Detailed receipts and fingerprints: evidence.json at the file root. These are finite observations, not exhaustive Rust or I/O proofs.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective: punctuation continuation should preserve a total token partition; malformed Gutenberg ordering should return Err before the parser. The old slice admitted a reversed-bound panic; the repair adds the ordering guard and isolates pure extraction without changing public signatures. Calendar century exceptions and expectation upper medians are predicted separately from natural-language semantics. Native tests and mutation controls calibrate these claims; see model.md#tests. Formal proofs and general corpus quality remain absent.
