# mlab-vm-compiler

## descriptant

### mlab-vm-compiler

AST classification, register assignment, bytecode emission and jump patching.

## approximant

### Scoped forward relation

Collect assigned variables/params/outputs, classify statements, emit native ops or Fb; loops patch exits/continues; sorted functions determine call indices.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

Try/switch/commands and frame-sensitive expressions become Fallback.

#### Cost

AST visits and emitted code/tables plus linear string dedup and sorted function names. Nested temporary register demand bounded only by assertion; usize-to-u16/u32/u8 casts need admitted bounds.

## models

# Scoped typed relations

## behavior_complement

Try/switch/commands exclude native statement compilation. Right operands with possible effects exclude direct reuse of left variable registers; unsupported expressions remain tree fallback.

## reverse_reconstruction

Bytecode admits multiple ASTs: parenthesization and equivalent constant forms may be lost; fallback retains original Stmt and its sync list.

## fallback_selection

Try/switch/commands and frame-sensitive expressions become Fallback.

## order_guard

left operand direct register reuse is allowed only if right expression is simple.

## range_compilation

For range emits ForRange/ForNext, other iterables ForGen; empty handling is runtime.

## register_budget

tmp asserts top<NO; fixed vars and u8 index casts lack a full overflow guarantee.


## tests

### Evidence scope

Parent selected tests: wave30_bounded_state_and_fallback, wave30_parse_runtime_and_output_state, wave30_synthetic_gate_fail_fast, vm_workspace_is_visible_after_run. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [10660,45698); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
