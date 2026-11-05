# mlab-fileio-json-table

## descriptant

### mlab-fileio-json-table

Require array of objects, collect keys in first-encounter order, sanitize/dedupe names; each column uses first matching member per row or Null and ordered type tests.

## approximant

### Forward relation

Require array of objects, collect keys in first-encounter order, sanitize/dedupe names; each column uses first matching member per row or Null and ordered type tests.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Output column order follows first key encounter; all output columns have one element per input object.

#### Failure ordering

Wrong root or nonobject member fails before column building. Nested objects/arrays fall back to JSON text in heterogeneous columns.

#### Cost

Key discovery linear search and per-column per-row lookup give O(rows*keys^2) worst-case work plus rendered nested text.

## models

# Typed models — mlab-fileio-json-table

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

A later duplicate key excludes overriding the first value. Missing booleans exclude logical-class column and yield numeric NaN instead.

## reverse_reconstruction

Every output column admits absent or explicit null antecedents; sanitized names and mixed-value text further erase original keys/types.

## state_invariant

Output column order follows first key encounter; all output columns have one element per input object.

## failure_precedence

Wrong root or nonobject member fails before column building. Nested objects/arrays fall back to JSON text in heterogeneous columns.

## representation_transport

All-null column is numeric by vacuous all; dates require all present strings parse_iso. Numeric nonfinite values remain until writer null projection.

## resource_cost

Key discovery linear search and per-column per-row lookup give O(rows*keys^2) worst-case work plus rendered nested text.

## tests

### Evidence and limitations

Parent source checks: fileio::wave30_tests::csv_inference_and_json_loss. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
