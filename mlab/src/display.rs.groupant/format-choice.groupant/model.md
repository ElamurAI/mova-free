# mlab-display-format-choice

## descriptant

### mlab-display-format-choice

Accumulate finite maxabs/minabs and all_int; all-int branch chooses integer unless digits>15, otherwise fixed versus exponent depends on rounded digit extrema.

## approximant

### Forward relation

Accumulate finite maxabs/minabs and all_int; all-int branch chooses integer unless digits>15, otherwise fixed versus exponent depends on rounded digit extrema.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Nonfinite strings require width at least four in integer/fixed branches; exponent width expands for large exponents.

#### Failure ordering

No Result errors; exact zero excluded from minabs update, which can remain infinity.

#### Cost

O(number of real values) scans and bounded float formatting per extrema.

## models

# Typed models — mlab-display-format-choice

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Nonfinite elements alone exclude making all_int false; integer values with more than 15 decimal digits exclude integer format.

## reverse_reconstruction

Same RealFmt admits many value collections with equal threshold category and width; extrema and order generally unrecoverable.

## state_invariant

Nonfinite strings require width at least four in integer/fixed branches; exponent width expands for large exponents.

## failure_precedence

No Result errors; exact zero excluded from minabs update, which can remain infinity.

## representation_transport

Exponent threshold uses dmax>=prec or dmin<=-2, after round_sig; common decimal precision is max endpoint requirements.

## resource_cost

O(number of real values) scans and bounded float formatting per extrema.

## tests

### Evidence and limitations

Parent source checks: display::tests::scalars_short, display::tests::matrix_rows. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
