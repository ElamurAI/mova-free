# fish-annotation-api

## model

# Shared typed Fish interface
## forward
Domain: Rust clients importing this library. Public annotation and execute modules expose the same Task/Profile types, native evaluate/render functions and review-only executor used by the binary. The library root reexports module paths; it performs no model invocation or runtime initialization.
## behavior_complement
Merely importing the library cannot launch a consultant, train a model or mutate a database. No separately duplicated Task layout or serializer is introduced at this root. Counterexample to interpreting import safety as client safety: a client may explicitly call execute::run, whose side effects have their own contract.
## reverse_reconstruction
An imported module path identifies the available public API surface, not a unique calling program or task. Many callers and serialized task instances share it. No inversion of arbitrary downstream behavior is claimed.
## invariant
Binary preparation and execution use one public annotation module. Canonical Task/context serialization belongs to those actual Rust types. This file contains declarations only.
## cost
Compile-time module exposure; no root-level runtime work. Delegated evaluator and executor costs belong to their source models.
## composition
Public main -> annotation_review_worker::{annotation,execute}; private or public clients can import the actual typed contract without copying source or routing Sonnet through a legacy Codex shim. A shared type does not prove the caller's source provenance or held-out exclusion.
## descriptant
This small root exposes reusable mechanics while deployment paths, queues and credentials remain external. Focused crate compilation and fake-CLI tests exercise the declared wiring; no global API compatibility theorem is claimed.

## tests

14 library tests plus the preserved legacy guard test pass offline/locked with two jobs. New fake-CLI checks cover exact stdin and safe/medium argv, strict schema, stale task hash, changed source, missing target, canonical labels, new reply-context findings, unresolved retention, declared-evaluation/deadline/oversize nonlaunch, Unknown review with not_teaching retained, durable malformed/timeout outcomes and replay refusal. Exact100000-byte and multibyte-overflow tests remain green. No live model, training, DB or corpus assessment was invoked. These are finite checks, not semantic accuracy, power-loss durability or exactly-once proofs.

Protocol followup:17 library tests plus the legacy guard pass offline/locked j2. Bound Claude JSON Schema and exact schema-plus-stdin100000-byte budget, strict structured/result envelope branches, UTF8-byte/cardinality controls, raw/typed profile pins, reaped plus actual ESRCH process-group observation, and provider/error-envelope partition are checked. Known-closed inner invalid_response is distinct from transport_error/child_failed; no replay, semantic/gold/training promotion or historical pilot artifact mutation. Fake executables only; deployment pilots separate.

## History

### 2026-11-02 · tests

2026-11-02: 14 library tests plus the preserved legacy guard test pass offline/locked with two jobs. New fake-CLI checks cover exact stdin and safe/medium argv, strict schema, stale task hash, changed source, missing target, canonical labels, new reply-context findings, unresolved retention, declared-evaluation/deadline/oversize nonlaunch, Unknown review with not_teaching retained, durable malformed/timeout outcomes and replay refusal. Exact100000-byte and multibyte-overflow tests remain green. No live model, training, DB or corpus assessment was invoked. These are finite checks, not semantic accuracy, power-loss durability or exactly-once proofs.

2026-11-02 protocol followup:17 library tests plus the legacy guard pass offline/locked j2. Bound Claude JSON Schema and exact schema-plus-stdin100000-byte budget, strict structured/result envelope branches, UTF8-byte/cardinality controls, raw/typed profile pins, reaped plus actual ESRCH process-group observation, and provider/error-envelope partition are checked. Known-closed inner invalid_response is distinct from transport_error/child_failed; no replay, semantic/gold/training promotion or historical pilot artifact mutation. Fake executables only; deployment pilots separate.
