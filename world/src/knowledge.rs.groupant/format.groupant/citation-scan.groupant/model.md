# Citation scanner and unread-suffix state

Subject: `world/src/knowledge.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

State (out,rest) scans the next '(' and first subsequent ')'. For a recognized reference, append trim_end(prefix) and advance beyond ')'; otherwise append prefix+'(' and advance only one delimiter. An unmatched '(' terminates scanning and copies the whole remaining suffix. This deliberately does not parse balanced parentheses.

## invariant

Each completed iteration consumes at least one byte of rest at an ASCII boundary; UTF-8 boundaries are preserved. Output never expands beyond input bytes. Nonreference nesting may still expose a later inner reference.

## cost

For B bytes, O(B) output space; conservative O(B^2) scanning/tokenization in nested nonreference inputs because suffixes may be revisited. No measured speedup.

## behavior-complement

Type: behavior_complement. Over UTF-8 string/output pairs, exclude deleting a closed group whose last token lacks ':' or whose numeric tail contains a letter, unless a separately scanned nested group qualifies. Exclude inventing output bytes. These are inspected inner bounds, not a balanced-parenthesis grammar claim.

## reverse-reconstruction

Type: reverse_reconstruction. Given output y, retain all strings whose scanner trace emits y: optional qualifying reference insertions can erase adjacent trailing whitespace, while nonreference groups survive except independently recognized inner spans. Empty output admits empty input and reference-only inputs. The original citations and whitespace cannot be uniquely recovered.

## recognizer

Type: lexical_admissibility. Closed inner text needs >=2 whitespace words; every name token starts uppercase or ASCII digit; last token contains ':' and only digits, ':', '-', ','. This does not validate books or verse numbers. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## suffix-progress

Type: termination_measure. The unread byte length strictly decreases on each iteration; an unmatched opening exits without deleting rest. Observe trace positions rather than count of balanced pairs. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## nested-parentheses

Type: delimiter_projection. First-closing-delimiter selection plus one-byte fallback allows an inner valid reference to be stripped from an invalid outer group; outer structure is not a protected span. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## whitespace-loss

Type: information_loss. A successful deletion trims the immediately preceding prefix's trailing whitespace, while following suffix whitespace survives. Antecedents differing in removed citation/preceding whitespace share output. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

parse_fields invokes this for explain; fact_lines invokes it only on fallback. Invalid field/question errors can stop the parent before later calls. Citation cost is already inside the format fold.

## checks

`knowledge::wave34_tests::citation_scan_preserves_nonreferences_and_utf8_suffixes` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.
