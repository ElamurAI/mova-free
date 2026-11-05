# world-triggers

## descriptant

# world-triggers

## Forward

Constants establish Trigger/TIME_UNITS/FREQ_UNITS/EVENTS. Event APIs append/read and summarize logs. Number/word/quantity helpers feed show_duration and fire_at. fire_at emits duration, cycles, clock, calendar, then recall in fixed order. fire samples dependencies; explain uses graph/fallback ranking; main_args prints. Existing tests and new bounded native tests are explicit evidence regions.

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

# world-triggers

Domain: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only.

All relations below are source-derived partial models, conditional on successful allocation and the stated input domain. U is the product of admitted inputs and observations (returns, errors, and explicit effects). R is the declared forward relation. Complement entries are candidate inner bounds of U minus R, not complements certified from an approximate emulator. Converse relations preserve every compatible antecedent; no unique original text or history is claimed.

## nonbehavior

For admitted finite pure inputs exclude GHz as an alias for gigahertz, century 1900 reported leap, absent event histories fabricated as recall, and expectation successors message/task. No claim about arbitrary nonfinite values or overflow.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## antecedents

Given fired messages retain all (text,now,tz,lived) tuples with matching ordered trigger observations. Text case and punctuation, unused events and multiple same-total quantity decompositions are ambiguous; formatted rounding creates additional fibers.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## units

Time quantities carry seconds multiplier; frequencies carry cycles/second. First numbered frequency times first numbered duration yields cycles; without duration the period is reciprocal frequency. Target conversion uses first unnumbered duration unit and sum of all numbered durations. Mean month/year seconds are fixed constants, not civil interval lengths.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## number_grammar

number_word lowercases, rejects a/an and numeric digits, recursively adds hyphen-separated known words. Internal number accepts comma-stripped f64 or table entries including a/an. quantities splits hyphens, so twenty-four hours uses four. No full compositional English number parser is claimed.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## calendar

First numeric word in 1583..9999 is inspected only when whole-word leap or year occurs. Gregorian leap predicate is divisible by 4 except centuries unless divisible by 400. Clock fires on now/today or substring what time; local seconds = now as i64 + 3600*tz. Test domain keeps arithmetic in range.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## event_statistics

expect_after considers adjacent original pairs whose first event matches and second is neither message nor task; it does not reconnect across excluded events. Each group has floor(1000*n/N) and upper median of saturating time gaps. Sort descending probability then lexical event. Frequencies may sum below 1000.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## event_recall

Recall uses first supplied matching event, fixed EVENTS order, lowercased alphanumeric words with optional trailing s, and saturating age. Future timestamp yields zero age; nonpositive elapsed duration omits took. All histories differing after first matched event remain compatible.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## io_clock

activate recognizes EVENTS, gets now, sanitizes tab/newline, truncates detail to 200 Unicode characters and best-effort appends; Task drop invokes task activation with Instant elapsed. episodes reads/reverses/truncates physical rows, not timestamp sorting. fire performs per-event reads then clock/timezone reads, so its input is not an atomic snapshot. This typed effect relation is unexecuted by bounded tests.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## explanation

explain first accepts nonempty nerves graph results, otherwise chooses manual by trigger world and substring hooks, scores overlap, filters byte length <160, sorts descending count then ascending byte length then lexical text and takes k. main_args --expect short-circuits to log-based expectation; ordinary CLI joins words, fires then explains. Graph/state/manual/stdout dependencies are unmodeled.

Scope: Finite English texts, finite positive unit quantities, year domain 1583..9999, in-range explicit u64 clock/i64 timezone, bounded event histories; named external log/graph/manual effects are conditional only. Fixed code and dependency contracts; evidence in model.md#tests. This is a conditional source relation, not a general proof. The explicit native fixtures are falsifiable observations; untested I/O schedules remain gaps.

## tests

### Bounded native evidence

Commands: cargo test -p world --lib bounded_models -- --test-threads=1; repeat with --release. New native modules contain five tests per source, including a compiled wrong-model discriminator. Independent finite tables cover partition, headings, malformed extraction, units, Gregorian dates, explicit UTC civil time and event ordering; enumeration covers 1,555 token sequences through length four and 85 unit quantities. The poly public rejection fixture creates a fresh scratch directory and preserves a sentinel output. No live hot-state access or parser acquisition is exercised.

External mutation controls remove the marker-order guard and replace the Gregorian century predicate, then compile and run the corresponding native tests; both must fail, after which originals are restored. Debug/release logs are normalized to sorted bounded test outcomes and compared twice per profile. Syntax projection is generated twice and compared byte-for-byte. Detailed receipts and fingerprints: evidence.json at the file root. These are finite observations, not exhaustive Rust or I/O proofs.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective: punctuation continuation should preserve a total token partition; malformed Gutenberg ordering should return Err before the parser. The old slice admitted a reversed-bound panic; the repair adds the ordering guard and isolates pure extraction without changing public signatures. Calendar century exceptions and expectation upper medians are predicted separately from natural-language semantics. Native tests and mutation controls calibrate these claims; see model.md#tests. Formal proofs and general corpus quality remain absent.
