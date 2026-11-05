# coref-tests

## descriptant

### coref-tests

Hand-authored synthetic CoNLL-U stories exercising actual resolver, format, document and evaluation APIs.

State: In-memory fixtures plus resolver/sys observations; no corpus or teacher access.

Finite fixtures; no model accuracy/corpus assertions, property exhaustiveness, performance or allocation-failure evidence.

## approximant

### Forward model

row maps short fixture columns to ten-column CoNLL-U; sent adds comments and ids; story builds three-sentence title/appos/reflexive/relative chain. Assertions compare externally specified chain texts/reasons, speaker partitions, stripped-gold output, read/write spans, it readings and strict-before-notional choice. Added wave27 checks cover document failure precedence/quotes/retree, nested Entity/head tie, coordination visibility and event/empty-eval behavior.

Domain: acyclic, in-range document views from checked parsing unless explicitly discussing permissive adapters; stable environment during each call; finite addressable allocations. Public vectors can be malformed and panic.

No runtime substitute or numerical error budget is established.

## invariant

### Invariant and cost

Assertions call actual production APIs. Test-generated expected chain/span/feature values are independent explicit constants, not copied resolver code. Fixtures require default environment with COREF_X/COREF_MD/PRAG_OFF unset.

Bounded constant fixtures, maximum story3 sentences; costs inherit production routines at tiny N/M. Empty eval executes all configurations with zero documents. No performance inference.

Integer counters/lengths use Rust native arithmetic with no separate overflow certificate. Allocation exhaustion, panics and I/O failures are not experimentally characterized.

## composition

### Source-fragment composition

Fragments project exact byte ranges of this source; shared types/imports outside ranges remain prerequisites. Models compose through actual caller/callee state, not an error-budget sum.

- fixtures: bytes [321,3967); Synthetic fixture grammar and observation helpers.
- contracts: bytes [3967,23859); Actual production assertions for chains, pronouns, representation and new bounded boundaries.

Parent relation: row maps short fixture columns to ten-column CoNLL-U; sent adds comments and ids; story builds three-sentence title/appos/reflexive/relative chain. Assertions compare externally specified chain texts/reasons, speaker partitions, stripped-gold output, read/write spans, it readings and strict-before-notional choice. Added wave27 checks cover document failure precedence/quotes/retree, nested Entity/head tie, coordination visibility and event/empty-eval behavior.

Integration: checked document tree -> mention spans/features -> ordered resolver partition -> eval visibility -> CorefUD rows or existing score module. Source-specific exceptions and projection losses remain in model.md#models.

## models

# Typed models

## behavior_complement

A passing suite excludes the specific observed wrong chains, head/span order, gold leak, quote reset and visibility failures on these fixtures. It does not exclude untested language errors, arbitrary large inputs or environmental configurations.

## reverse_reconstruction

For the vector of passing outcomes, compatible implementations are all programs agreeing on these bounded assertions; infinitely many disagree elsewhere. For serialized fixtures, row's lowercase lemma and omitted default MISC collapse antecedent short-column strings.

## fixture_transport

Short fixture columns are trusted: missing mandatory columns panic in helper. Lemma defaults lowercase form with explicit replacements for rain/happen/break/be.

## oracle_independence

story chain expectations specify title-based masculine inference, principle B across relative-pronoun cluster and sieve identities; tests do not simulate merge.

## negative_sensitivity

Compiled mutants target quote reset, rightmost matching close, weather, coordination number, visibility and principle B; failure records and restoration live outside repository.

## evidence_scope

Two debug and two release named outcome sets are compared; runtime control receipts distinguish compiler success from failing assertions. Test count is measured, not inferred from declarations.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- All tests in this source (score module tests excluded)

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: Finite fixtures; no model accuracy/corpus assertions, property exhaustiveness, performance or allocation-failure evidence.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Fixture source executed by its assertions; no independent production mutant of test helpers.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

Strict scoped Clippy: exit101 on four pre-existing production lints (corefud unnecessary_sort_by, doc if_same_then_else and clone_on_copy, mention collapsible_if). Those source files are byte-identical to the base. No unrelated lint refactor was made.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective analysis preceded test additions. Production behavior is unchanged. A registry entry records partial source-bound modeling, not verification.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
