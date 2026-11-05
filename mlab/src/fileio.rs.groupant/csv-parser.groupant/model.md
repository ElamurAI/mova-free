# mlab-fileio-csv-parser

## descriptant

### mlab-fileio-csv-parser

Strip leading BOM chars, skip blank LF/CRLF lines; each field records starting line/col, quoted strings decode doubled quotes and embedded LF before delimiter/end handling.

## approximant

### Forward relation

Strip leading BOM chars, skip blank LF/CRLF lines; each field records starting line/col, quoted strings decode doubled quotes and embedded LF before delimiter/end handling.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Field coordinates are Unicode scalar indices, one-based; each record retains start line and end column.

#### Failure ordering

Closing quote followed by a non-delimiter/non-newline/non-CR fails immediately; lone CR behavior differs between quoted and unquoted paths.

#### Cost

O(input characters + copied field bytes), O(input characters + field storage).

## models

# Typed models — mlab-fileio-csv-parser

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Unterminated quoted input excludes a Record result; quotes in unquoted fields are retained rather than rejected.

## reverse_reconstruction

Field text plus quoted flag admits CRLF/LF record-separator and leading-BOM variations; CR inside quoted payload is retained and is not interchangeable with LF. Coordinate metadata narrows but does not uniquely recover separators or doubled-quote encoding.

## state_invariant

Field coordinates are Unicode scalar indices, one-based; each record retains start line and end column.

## failure_precedence

Closing quote followed by a non-delimiter/non-newline/non-CR fails immediately; lone CR behavior differs between quoted and unquoted paths.

## representation_transport

Empty lines skipped, trailing delimiter creates empty last field; text is Vec<char>, so indices are not UTF-8 bytes.

## resource_cost

O(input characters + copied field bytes), O(input characters + field storage).

## tests

### Evidence and limitations

Parent source checks: fileio::wave30_tests::csv_inference_and_json_loss. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
