# Explicit source-composition CLI

## domain
Finite UTF-8 argv with exact snapshot/check/stage arities, public input paths and external manifest/output. Library validation and ordinary stdout/IO success are premises.
## forward
Dispatch snapshot to save_snapshot; check loads the bound external manifest/snapshot, prints structured compatibility and exits3 for drift; stage loads then composes a fresh tree and prints its receipt. Unsupported argv returns usage error. Commands never invoke a shell or build subprocess.
## invariant
No CLI command accepts hooks or arbitrary execution argv. Snapshot output is outside source; check is read-only; stage effects are delegated to the bounded composer. Fixed Cargo argv are receipt data only.
## cost
O(argument bytes+serialized report) plus delegated bounded library inventory/composition. Large reports are bounded by the library file/path limits; no independent timing claim.
## behavior-complement
Within exact-arity dispatch, no unsupported command yields a successful composition. A drift check cannot report exit0. CLI does not turn a stored pipeline role name into an executable operation. Effectful library errors remain explicit.
## reverse-reconstruction
A successful report constrains the selected command and bound input observations, but does not recover filesystem paths from deterministic receipts, which intentionally omit absolute roots. Exit3 identifies base drift, not the exact changed file without the report; error exit alone has multiple causes.
## dispatch-partition
Argv tag and length select snapshot/check/stage/error. Observation comprises return code and stdout record; equal output-tree commitments across distinct output paths are allowed. Counterexample to location recovery: stage identical inputs into two fresh sibling paths; receipt bytes remain equal.
## checks
Native example CLI check/stage runs and repeated receipts exercise transport; twelve library fixtures supply scoped delegate evidence. Unsupported manifest hooks are rejected by schema validation. No arbitrary-extension build compatibility is inferred.
## epoch-memory

CLI is a small transport over the separate library. Cargo recipes remain unexecuted unless an operator independently selects them.

## History

### 2026-11-01 · epoch-memory

2026-11-01: CLI is a small transport over the separate library. Cargo recipes remain unexecuted unless an operator independently selects them.
