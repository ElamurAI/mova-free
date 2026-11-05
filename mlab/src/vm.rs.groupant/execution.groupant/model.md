# mlab-vm-execution

## descriptant

### mlab-vm-execution

Workspace transfer, pooled calls, arithmetic/index slow paths and dispatcher.

## approximant

### Scoped forward relation

Import current frame to U/F/L/V registers; execute code; synchronize tree fallbacks and dyn calls; export all script variables even after returned error.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

exec truncates end_stack on Err; finish_call restores frame/depth and clears pooled buffers.

#### Cost

Executed instructions plus delegated kernels, sync-list size, matrix COW and pooled frames. Loops unbounded; arithmetic uses f64 exceptional values and indices use dimension products.

## models

# Scoped typed relations

## behavior_complement

Out-of-bounds or nonintegral matrix coordinates exclude fast indexed write; shared Rc aliases exclude mutating the other alias. A fallback error excludes continuing dispatch but not its already synchronized writes.

## reverse_reconstruction

Final workspace does not reveal instruction count, prior overwritten registers or COW ownership. Stats constrain counts but not execution trace.

## error_restore

exec truncates end_stack on Err; finish_call restores frame/depth and clears pooled buffers.

## copy_on_write

asg_fast validates in-bounds double matrix then Rc::make_mut writes one cell.

## fallback_flow

Break/continue use patched fallback targets; Return ends execution; errors propagate after sync_in.

## fault_scope

vm:add affects nonconstant binary scalar AddBug, not every addition fast path.


## tests

### Evidence scope

Parent selected tests: wave30_bounded_state_and_fallback, wave30_parse_runtime_and_output_state, wave30_synthetic_gate_fail_fast, vm_workspace_is_visible_after_run. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [45698,88727); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
