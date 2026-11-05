# mlab-suite-run-gate

## descriptant

### mlab-suite-run-gate

Fresh runtime construction and fail-fast output equivalence.

## approximant

### Scoped forward relation

Construct fresh interpreter for each run; gate runs tree then VM from new captures and stops first differing output.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

N run_fs scenarios invoke mk_fs N times, including empty programs.

#### Cost

Sum of scenario execution plus fresh interpreter/FS creation; gate performs two runs per visited scenario and stops at first mismatch; no time limit.

## models

# Scoped typed relations

## behavior_complement

First output mismatch excludes evaluating remaining scenarios. Equal engine output does not exclude shared erroneous builtin behavior; run_fs excludes default workspace sharing.

## reverse_reconstruction

Same outputs can come from shared faulty builtin; first mismatch restricts observed prefix only.

## factory_count

N run_fs scenarios invoke mk_fs N times, including empty programs.

## fault_transport

Same fault string supplied to tree and VM; vm:add intentionally affects only VM.

## prefix_stats

Mismatch scenario contributes totals and static/dynamic counts before break.

## empty_gate

Empty input returns all-zero default report.


## tests

### Evidence scope

Parent selected tests: wave30_synthetic_suite_boundaries, wave30_synthetic_gate_fail_fast. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [3525,6795); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
