# algorithm-boyer-moore-horspool-tests

## descriptant

### Combiner and epoch memory

Combiner inputs are raw Rust, its partial syntax projection, forward/invariant models, the ordered typed family and this memory. Output candidates are local recurrence, tie or shift changes, filtered by invariant and oracle agreement. It is a descriptive interface, not an implemented autonomous model generator.

Epoch evidence is retained in model.md#tests. Finite agreement narrows uncertainty but does not prove arbitrary input correctness. No useful faster executable approximant is supplied for these exact observations; categorical errors have no accepted percentage budget.

## approximant

### Forward model and invariant

Accept precisely when every finite fixture assertion holds. Preserve the fixture generator, oracle and production calls as distinct observers. No mutation of caller inputs is requested.

R. Nigel Horspool, Practical fast searching in strings, Software: Practice and Experience 10(6), 1980, doi:10.1002/spe.4380100608. Original Rust implementation.

## models

# Typed relations

Revision: source hash in manifest. Observation: complete single call, lawful Eq and successful allocation unless explicitly scoped otherwise. Model direction is an exact-target relational specification; correspondence to Rust is bounded evidence, not a proof. Each section names its distinct observation, fixed quantities and falsifiable constraint.

## behavior_complement

Universe: finite fixture executions with assertion outcomes. Excluded accepted runs disagree with the independent oracle, omit required boundary assertions or accept the compiled production mutant. A green run only constrains executed cases; untested inputs remain unknown.

## reverse_reconstruction

Given an assertion transcript, compatible production implementations are all programs producing those same fixture observations. The actual source is one antecedent, not uniquely determined. Fiber tests reconstruct compatible inputs under the finite oracle, retaining multiplicity.

## finite_domain

The binary word generator enumerates exactly sum(2^k,k=0..max) words including empty. Search compares max text 7 and pattern 4; substring compares both max 5; OSA both max 4. Exhaustiveness is only within these domains, not all bytes, elements or lengths.

## oracle_independence

Search scans candidate slices without hashes/shifts; substring searches lengths descending then starts ascending without a DP row; OSA recursively enumerates consumed prefixes without rolling storage. Shared problem definitions remain a possible specification error; hand witnesses address selected boundaries.

## mutation_sensitivity

A production source mutation is compiled against unchanged assertions: search acceptance/shift, substring tie comparison, or OSA transposition removal. At least one named assertion must fail; compile errors do not count. Restoring exact bytes binds subsequent evidence to the original source.

## tests

Independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under (see History). Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### Native checks

## evidence

### Validation evidence

Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed. Focused rustfmt --check passed for all eight owned Rust sources. Eight partial syntax projections were generated twice by mova-modelcheck syntax and compared byte-for-byte.

Four production mutations compiled and failed assertions: remove Rabin-Karp byte verification (actual wrapping collision), replace Horspool shift by full pattern length (overlap loss), reverse substring tie preference, disable OSA transposition. The exact source bytes were restored after each run. Compiler failures were not accepted as negative controls.

Independent bounded comparisons: each search checks 255 binary texts times 31 patterns (7905 pairs); substring checks 63 squared (3969 pairs); OSA checks 31 squared (961 pairs). Additional fibers, boundary and unit fixtures are separate. OSA uses recursive script enumeration, substring uses descending-length slice comparisons, and search uses direct candidate slices.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: Independent bounded oracles should agree with production, and a targeted compiled mutant should fail.

### 2026-10-30 · tests

2026-10-30: independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/. Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### 2026-10-30 · evidence

2026-10-30, CPU, native Rust. Four newly implemented algorithms contribute 14 tests. With temporary local library exports, the crate passed 279 unit tests and one doctest in debug twice and release twice. Sorting the 280 passed-test records produced byte-identical normalized results across all four runs. Timings and compiler banners are excluded; raw logs remain external.

Artifacts: $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/{debug1,debug2,release1,release2}.log and .normalized, controls.log, mutant-*.log, clippy.log and *-syntax2.json. controls.rs is the native external mutation/normalization driver. These finite checks neither establish a universal proof nor benchmark throughput. No live API, GPU, teacher or bus execution occurred.
