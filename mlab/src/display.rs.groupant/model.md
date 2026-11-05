# mlab-display

## descriptant

### mlab-display

Octave-style textual display of values, real/complex common formats and function expression unparse.

## approximant

### Forward relation

Choose real common format from finite extrema/integrality and precision 5 or 16; render column-major cells as rows, split blocks by floor(80/cell_width). Named output adds labels/blank lines; disp omits numeric empty values and prints char rows.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Cells use index col*rows+row. All matrix cells share format widths computed across data. Nonfinite names are NaN/Inf/-Inf.

#### Failure ordering

Public Mat dimensions/storage and scalar index must be valid; inconsistent lengths can panic. Width and formatting allocation are unchecked; no I/O performed here.

#### Cost

O(elements + emitted characters), O(elements + text) cell buffering; recursive unparse may repeatedly concatenate subtree strings. No measured speed or general relative numeric-error guarantee for text.

## composition

### Composition

Choose real common format from finite extrema/integrality and precision 5 or 16; render column-major cells as rows, split blocks by floor(80/cell_width). Named output adds labels/blank lines; disp omits numeric empty values and prints char rows.

Logical values use nonzero predicate; char values map codepoints. Precision controls significant digits, but exact integer branch remains integer up to 15 digits.

#### format-choice

Accumulate finite maxabs/minabs and all_int; all-int branch chooses integer unless digits>15, otherwise fixed versus exponent depends on rounded digit extrema.

Boundary: Nonfinite strings require width at least four in integer/fixed branches; exponent width expands for large exponents.

#### matrix-layout

Map storage j*r+i into row vectors with common field width, then partition columns into blocks of max(1,80/width).

Boundary: Every row visits columns in increasing order. Last two-column block uses and rather than through header.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-display

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Numeric zero formatting excludes a printed negative sign; scalar complex imaginary -0 permits minus while matrix complex comparison excludes it. Empty numeric disp excludes a newline, empty char disp includes one.

## reverse_reconstruction

Printed rounded values admit all floating inputs sharing chosen format and rounding bins; labels do not recover types uniquely. unparse loses closure captures and is not a precedence-repairing inverse parser.

## state_invariant

Cells use index col*rows+row. All matrix cells share format widths computed across data. Nonfinite names are NaN/Inf/-Inf.

## failure_precedence

Public Mat dimensions/storage and scalar index must be valid; inconsistent lengths can panic. Width and formatting allocation are unchecked; no I/O performed here.

## representation_transport

Logical values use nonzero predicate; char values map codepoints. Precision controls significant digits, but exact integer branch remains integer up to 15 digits.

## resource_cost

O(elements + emitted characters), O(elements + text) cell buffering; recursive unparse may repeatedly concatenate subtree strings. No measured speed or general relative numeric-error guarantee for text.

## tests

### Evidence and limitations

Scope: Cells use index col*rows+row. All matrix cells share format widths computed across data. Nonfinite names are NaN/Inf/-Inf.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

### 2026-10-31 · tests

Selected named outcomes: display::tests::scalars_short, display::tests::matrix_rows. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "display::tests::scalars_short": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
