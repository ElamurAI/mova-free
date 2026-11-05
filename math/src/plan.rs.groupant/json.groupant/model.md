# math-plan-gate-json

## descriptant

### Source-bound description

Scan chars for opening brace, balance nested braces respecting string escapes, emit completed object slice; incomplete trailing object omitted. parse_plans serde-decodes each emitted object independently.

## approximant

### Forward relation

Scan chars for opening brace, balance nested braces respecting string escapes, emit completed object slice; incomplete trailing object omitted. parse_plans serde-decodes each emitted object independently.

Domain: Unicode text; coordinates are char offsets internal to collected Vec, not raw byte offsets.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Unclosed object excluded from outputs; braces inside quoted string do not close object; malformed completed JSON yields Err rather than omission in parse_plans.

Ordering: Extract balanced candidates before serde validation; malformed early opening brace can absorb later text.

Cost: O(chars+emitted bytes) scanner storage/time plus serde parse, no explicit depth/size bound.

## composition

### Composition and epoch memory

solve parsers reuse json_objects but silently skip invalid objects, unlike parse_plans Err retention.

Source scope bytes[27846,29335). Finite checks not proof; no meaningful percentage surrogate for categorical gate. Not every branch in this fragment executed.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Unclosed object excluded from outputs; braces inside quoted string do not close object; malformed completed JSON yields Err rather than omission in parse_plans.

## reverse_reconstruction

Object sequence admits arbitrary surrounding prose and incomplete suffix; serialized objects retain raw spelling but Plan defaults collapse absent fields.

## failure_precedence

Extract balanced candidates before serde validation; malformed early opening brace can absorb later text.

## representation_transport

solve parsers reuse json_objects but silently skip invalid objects, unlike parse_plans Err retention.

## resource_boundary

O(chars+emitted bytes) scanner storage/time plus serde parse, no explicit depth/size bound.

## domain_boundary

Unicode text; coordinates are char offsets internal to collected Vec, not raw byte offsets.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --lib plan::tests`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Source scope bytes[27846,29335). Finite checks not proof; no meaningful percentage surrogate for categorical gate. Not every branch in this fragment executed.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Source scope bytes[27846,29335). Finite checks not proof; no meaningful percentage surrogate for categorical gate. Not every branch in this fragment executed.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
