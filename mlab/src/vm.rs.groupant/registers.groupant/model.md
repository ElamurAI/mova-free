# mlab-vm-registers

## descriptant

### mlab-vm-registers

Register representation and value/matrix transport.

## approximant

### Scoped forward relation

Real scalar doubles pack to F; canonical logical bits pack L; all else V; U converts to None.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

Logical -0 does not pack as L(false): only bits==0 does.

#### Cost

Scalar packing/unpacking O(1); shared Value conversion may clone matrix payload. Borrowed matrix view avoids clone, scalar view allocates one cell.

## models

# Scoped typed relations

## behavior_complement

U excludes any Value result. Noncanonical logical bits exclude L packing; complex/character/multielement matrices exclude F packing.

## reverse_reconstruction

F(x) admits scalar double value x; V retains nonpacked values; allocation ownership and clone history cannot be reconstructed.

## canonical_bits

Logical -0 does not pack as L(false): only bits==0 does.

## undefined_value

U has no value and is not an empty matrix at this boundary.

## ownership

into_value tries Rc unwrap, otherwise clones shared payload.

## matrix_view

mat_cow borrows V matrices but constructs scalar matrices for F/L.


## tests

### Evidence scope

Parent selected tests: wave30_bounded_state_and_fallback, wave30_parse_runtime_and_output_state, wave30_synthetic_gate_fail_fast, vm_workspace_is_visible_after_run. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [2447,5611); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
