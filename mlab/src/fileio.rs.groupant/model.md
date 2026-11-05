# mlab-fileio

## descriptant

### mlab-fileio

CSV and JSON parsers, inferred table columns and matrix/value serialization without filesystem effects.

## approximant

### Forward relation

CSV scans Unicode chars after leading BOMs, tracks line/column, decodes quoted fields and validates table width before inference. Numeric inference precedes boolean/date/text and quoted-present forces text. JSON parser preserves object member sequence; table conversion unions first-seen keys, picks first duplicate and fills missing null. Writers emit canonical bytes.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

CSV coordinates are one-based Unicode-scalar columns and decoded line counts. Matrix output uses row order over column-major storage. Parsing allocates values but performs no host/VFS effects.

#### Failure ordering

CSV syntax errors precede table width errors; quoted unterminated error reports field origin, unexpected postquote reports current location. JSON parser is permissive f64 grammar rather than strict RFC number validator; recursion/allocation unbounded.

#### Cost

CSV O(B+cells) time/storage. JSON character parsing O(B), error position rescans prefix; key union and per-column row lookups can require O(rows*keys^2). json_text copies child strings at each nesting level: cumulative work is the sum of serialized subtree sizes, bounded by O(depth * output bytes), plus table/cell work. A depth-D singleton-array chain can require O(D^2) copying for O(D) output; recursion depth is unchecked.

## composition

### Composition

CSV scans Unicode chars after leading BOMs, tracks line/column, decodes quoted fields and validates table width before inference. Numeric inference precedes boolean/date/text and quoted-present forces text. JSON parser preserves object member sequence; table conversion unions first-seen keys, picks first duplicate and fills missing null. Writers emit canonical bytes.

Numeric/date NaN table cells serialize empty CSV; write_matrix emits literal NaN through num_text. Numeric-class nonfinite JSON values become null, while logical-class values use nonzero truth, so logical NaN becomes true. Booleans with missing values become numeric; dates encode day values. Empty table CSV is newline and JSON is [] newline; read_matrix skips at most one nonnumeric header.

#### csv-parser

Strip leading BOM chars, skip blank LF/CRLF lines; each field records starting line/col, quoted strings decode doubled quotes and embedded LF before delimiter/end handling.

Boundary: Field coordinates are Unicode scalar indices, one-based; each record retains start line and end column.

#### json-table

Require array of objects, collect keys in first-encounter order, sanitize/dedupe names; each column uses first matching member per row or Null and ordered type tests.

Boundary: Output column order follows first key encounter; all output columns have one element per input object.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-fileio

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Quoted numeric CSV fields exclude numeric inference for their column; entirely missing column excludes text inference. J::Num and numeric-class nonfinite JSON serialization excludes numeric literals and becomes null; logical-class values instead use x!=0, so logical NaN serializes true. Successful parse_json excludes trailing nonwhitespace.

## reverse_reconstruction

Canonical tables admit many CSV antecedents differing quoting, headers sanitized/deduped and numeric spelling. JSON table preimages include absent versus null and later duplicate keys. Vector JSON cannot reconstruct row versus column orientation or complex imaginary data.

## state_invariant

CSV coordinates are one-based Unicode-scalar columns and decoded line counts. Matrix output uses row order over column-major storage. Parsing allocates values but performs no host/VFS effects.

## failure_precedence

CSV syntax errors precede table width errors; quoted unterminated error reports field origin, unexpected postquote reports current location. JSON parser is permissive f64 grammar rather than strict RFC number validator; recursion/allocation unbounded.

## representation_transport

Numeric/date NaN table cells serialize empty CSV; write_matrix emits literal NaN through num_text. Numeric-class nonfinite JSON values become null, while logical-class values use nonzero truth, so logical NaN becomes true. Booleans with missing values become numeric; dates encode day values. Empty table CSV is newline and JSON is [] newline; read_matrix skips at most one nonnumeric header.

## resource_cost

CSV O(B+cells) time/storage. JSON character parsing O(B), error position rescans prefix; key union and per-column row lookups can require O(rows*keys^2). json_text copies child strings at each nesting level: cumulative work is the sum of serialized subtree sizes, bounded by O(depth * output bytes), plus table/cell work. A depth-D singleton-array chain can require O(D^2) copying for O(D) output; recursion depth is unchecked.

## tests

### Evidence and limitations

Scope: CSV coordinates are one-based Unicode-scalar columns and decoded line counts. Matrix output uses row order over column-major storage. Parsing allocates values but performs no host/VFS effects.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

#### Cross-review refinement — 2026-10-31

Table CSV encodes numeric/date NaN as empty fields, whereas matrix CSV emits NaN text. Logical NaN becomes JSON true by nonzero conversion; numeric nonfinite values become null. Recursive JSON rendering repeatedly copies child text, so cumulative subtree sizes, rather than final output alone, determine work. CRLF/LF ambiguity is limited to record separators; quoted payload retains CR.

### 2026-10-31 · tests

Selected named outcomes: fileio::wave30_tests::csv_inference_and_json_loss. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "fileio::wave30_tests::csv_inference_and_json_loss": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
