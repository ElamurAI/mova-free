# mlab-display-matrix-layout

## descriptant

### mlab-display-matrix-layout

Map storage j*r+i into row vectors with common field width, then partition columns into blocks of max(1,80/width).

## approximant

### Forward relation

Map storage j*r+i into row vectors with common field width, then partition columns into blocks of max(1,80/width).

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Every row visits columns in increasing order. Last two-column block uses and rather than through header.

#### Failure ordering

Invalid Mat storage can panic during indexing; output buffering has no fallible allocation API.

#### Cost

O(rows*cols + output bytes) time and buffered cells; no streaming interface.

## models

# Typed models — mlab-display-matrix-layout

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Matrix complex -0 excludes minus separator because only <0 is checked. Block headers are absent when all columns fit.

## reverse_reconstruction

Aligned row strings admit matrices of compatible shape and quantized scalar cells; common width and block headers constrain shape but not original floats.

## state_invariant

Every row visits columns in increasing order. Last two-column block uses and rather than through header.

## failure_precedence

Invalid Mat storage can panic during indexing; output buffering has no fallible allocation API.

## representation_transport

Logical width is 3, real width is 2+field width, complex adds signs/imaginary field; terminal width is nominal characters.

## resource_cost

O(rows*cols + output bytes) time and buffered cells; no streaming interface.

## tests

### Evidence and limitations

Parent source checks: display::tests::scalars_short, display::tests::matrix_rows. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
