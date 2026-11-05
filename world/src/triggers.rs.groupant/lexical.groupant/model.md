# world-triggers-lexical

## descriptant

# world-triggers-lexical

## Forward

Logical source region "const NUMBER_WORDS" through before "/// What fires". quantities emits units in text order, recognizes only exact singular/plural spelling after lowercasing, and attaches only the immediately preceding number. Unit table index and multiplier remain coupled. number_word lowercases, rejects a/an and numeric digits, recursively adds hyphen-separated known words. Internal number accepts comma-stripped f64 or table entries including a/an. quantities splits hyphens, so twenty-four hours uses four. No full compositional English number parser is claimed. show_duration chooses largest table unit <= seconds, else nanosecond; formats near-integers exactly else two decimals, singular only when displayed num equals '1'. Multiple seconds values can share text, including a rounded singular; no unique numeric inverse. Parent composition retains shared text/clock/intermediate values. Full-file wrapper and test gaps remain in the root.

## Invariant

Respect source order, retain ambiguity of lossy text projections, and keep observed finite correspondence distinct from universal semantics. Pure functions have no modeled filesystem effects. Error paths before annotator acquisition do not write the output.

## Composition

Compose only through equal intermediate token sequences, paragraphs, quantities, event lists and explicit clock inputs. Parent roots cover every logical source region, including imports/constants and native tests; local fragments isolate useful operations and may overlap. The hierarchy is not a complete call graph. Same numeric answer does not imply same text or event history.

## Cost

Tokenization O(text bytes); quantity lookup O(words * units); recall O(EVENTS * words * hooks + lived scan); expectations O(events + sum group gap-sort costs); log reads O(file bytes), overlap O(manual bytes + candidates log candidates). Costs are symbolic operation/allocation counts, not measured speedups.

## Limits

Partial syn-2 syntax is source-bound; no macro expansion, semantic AST, name resolution or verified dependency behavior. No automatic reverse solver or production surrogate exists. Poly annotator acquisition/tokenization, detokenization, output encoding and filesystem failures beyond early rejection remain conditional. Trigger talk::civil/now/tz_offset, state::root, global nerves graph, manual data and stdout are dependency boundaries. No activate/Task/drop/fire/episodes/expectation/explain calls are made by the new bounded tests. Extremely large counts, nonfinite floating values, signed timestamp overflow and concurrent log append/read remain outside tested domains. Declared event effects are not executed.

## Epoch memory

## catalog

# world-triggers-lexical

Domain: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only.

All relations below are source-derived partial models, conditional on successful allocation and the stated input domain. U is the product of admitted inputs and observations (returns, errors, and explicit effects). R is the declared forward relation. Complement entries are candidate inner bounds of U minus R, not complements certified from an approximate emulator. Converse relations preserve every compatible antecedent; no unique original text or history is claimed.

## nonbehavior

For lexical helper inputs exclude numeric digit strings and a/an from number_word; exclude unknown unit strings from quantities. Internal number differs and admits a/an.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## antecedents

Given quantities retain every string whose token words carry the same recognized preceding numbers and unit indices, including arbitrary ignored words elsewhere. Given rendered duration retain all seconds in the same formatting fiber.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## unit_lookup

quantities emits units in text order, recognizes only exact singular/plural spelling after lowercasing, and attaches only the immediately preceding number. Unit table index and multiplier remain coupled.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## number_grammar

number_word lowercases, rejects a/an and numeric digits, recursively adds hyphen-separated known words. Internal number accepts comma-stripped f64 or table entries including a/an. quantities splits hyphens, so twenty-four hours uses four. No full compositional English number parser is claimed.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## duration_render

show_duration chooses largest table unit <= seconds, else nanosecond; formats near-integers exactly else two decimals, singular only when displayed num equals '1'. Multiple seconds values can share text, including a rounded singular; no unique numeric inverse.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## tests

### Bounded native evidence

Commands: cargo test -p world --lib bounded_models -- --test-threads=1; repeat with --release. New native modules contain five tests per source, including a compiled wrong-model discriminator. Independent finite tables cover partition, headings, malformed extraction, units, Gregorian dates, explicit UTC civil time and event ordering; enumeration covers 1,555 token sequences through length four and 85 unit quantities. The poly public rejection fixture creates a fresh scratch directory and preserves a sentinel output. No live hot-state access or parser acquisition is exercised.

External mutation controls remove the marker-order guard and replace the Gregorian century predicate, then compile and run the corresponding native tests; both must fail, after which originals are restored. Debug/release logs are normalized to sorted bounded test outcomes and compared twice per profile. Syntax projection is generated twice and compared byte-for-byte. Detailed receipts and fingerprints: evidence.json at the file root. These are finite observations, not exhaustive Rust or I/O proofs.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective: punctuation continuation should preserve a total token partition; malformed Gutenberg ordering should return Err before the parser. The old slice admitted a reversed-bound panic; the repair adds the ordering guard and isolates pure extraction without changing public signatures. Calendar century exceptions and expectation upper medians are predicted separately from natural-language semantics. Native tests and mutation controls calibrate these claims; see model.md#tests. Formal proofs and general corpus quality remain absent.
