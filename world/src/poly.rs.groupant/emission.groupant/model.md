# world-poly-emission

## descriptant

# world-poly-emission

## Forward

Logical source region "/// Cut out" through before "/// Pure extraction". Conditioned on tokenizer/detokenizer, each nonempty detokenized sentence yields paragraph index pi+1, globally increasing sentence ID and tabs replaced by spaces. words counts tokens containing an alphanumeric character; diff counts paragraphs not reproduced verbatim. Header carries raw path and selections. No guarantee that arbitrary heading text is automatically isolated without blank lines. Let N be total input bytes, L the line count and B selected bytes. Extraction takes O(N+L+B) time: text.lines scans the entire input, including excluded text. Its auxiliary memory is O(L+B); the whole prep call also retains the raw input, so extraction-stage retained memory is O(N+L+B). Sentence scan takes O(n+sum token bytes); output adds O(Bout) time/storage, plus opaque annotator and token storage costs. These bounds assume finite valid UTF-8 and successful allocation; no measured timing prediction. prep reads exactly the named input before pure extraction; annotation is acquired only after nonempty extraction; output write follows full formatting. Annotator acquisition and filesystem effects are opaque. Native test covers only the early-error no-write boundary. Parent composition retains shared text/clock/intermediate values. Full-file wrapper and test gaps remain in the root.

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

# world-poly-emission

Domain: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects.

All relations below are source-derived partial models, conditional on successful allocation and the stated input domain. U is the product of admitted inputs and observations (returns, errors, and explicit effects). R is the declared forward relation. Complement entries are candidate inner bounds of U minus R, not complements certified from an approximate emulator. Converse relations preserve every compatible antecedent; no unique original text or history is claimed.

## nonbehavior

On successful output exclude sentence IDs that skip an emitted row or reset per paragraph. Do not infer this constraint if filesystem writing fails.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## antecedents

Given TSV rows and summary retain all books, annotator token sequences and detokenization results matching the rows, word count and diff. Header constrains path/selection but cannot recover original whitespace.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## emission

Conditioned on tokenizer/detokenizer, each nonempty detokenized sentence yields paragraph index pi+1, globally increasing sentence ID and tabs replaced by spaces. words counts tokens containing an alphanumeric character; diff counts paragraphs not reproduced verbatim. Header carries raw path and selections. No guarantee that arbitrary heading text is automatically isolated without blank lines.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## resource

Let N be total input bytes, L the line count and B selected bytes. Extraction takes O(N+L+B) time: text.lines scans the entire input, including excluded text. Its auxiliary memory is O(L+B); the whole prep call also retains the raw input, so extraction-stage retained memory is O(N+L+B). Sentence scan takes O(n+sum token bytes); output adds O(Bout) time/storage, plus opaque annotator and token storage costs. These bounds assume finite valid UTF-8 and successful allocation; no measured timing prediction.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## dependencies

prep reads exactly the named input before pure extraction; annotation is acquired only after nonempty extraction; output write follows full formatting. Annotator acquisition and filesystem effects are opaque. Native test covers only the early-error no-write boundary.

Scope: Finite UTF-8 Gutenberg books with exact line markers/headings; finite Tok arrays; explicit named input/output paths. Observations are ranges, extracted paragraphs, errors and conditional TSV effects. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## tests

### Bounded native evidence

Commands: cargo test -p world --lib bounded_models -- --test-threads=1; repeat with --release. New native modules contain five tests per source, including a compiled wrong-model discriminator. Independent finite tables cover partition, headings, malformed extraction, units, Gregorian dates, explicit UTC civil time and event ordering; enumeration covers 1,555 token sequences through length four and 85 unit quantities. The poly public rejection fixture creates a fresh scratch directory and preserves a sentinel output. No live hot-state access or parser acquisition is exercised.

External mutation controls remove the marker-order guard and replace the Gregorian century predicate, then compile and run the corresponding native tests; both must fail, after which originals are restored. Debug/release logs are normalized to sorted bounded test outcomes and compared twice per profile. Syntax projection is generated twice and compared byte-for-byte. Detailed receipts and fingerprints: evidence.json at the file root. These are finite observations, not exhaustive Rust or I/O proofs.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective: punctuation continuation should preserve a total token partition; malformed Gutenberg ordering should return Err before the parser. The old slice admitted a reversed-bound panic; the repair adds the ordering guard and isolates pure extraction without changing public signatures. Calendar century exceptions and expectation upper medians are predicted separately from natural-language semantics. Native tests and mutation controls calibrate these claims; see model.md#tests. Formal proofs and general corpus quality remain absent.
