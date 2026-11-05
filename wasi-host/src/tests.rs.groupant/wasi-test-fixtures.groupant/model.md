# wasi-test-fixtures

## descriptant

### wasi-test-fixtures

ctx creates /d, two files and argv prog,arg1; put_str raw-copies at given coordinate; open writes path at1000 and invokes production path_open with root fd3/follow1 and output900.

## approximant

### Forward relation

ctx creates /d, two files and argv prog,arg1; put_str raw-copies at given coordinate; open writes path at1000 and invokes production path_open with root fd3/follow1 and output900.

Scope: `wasi-test-fixtures`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

ctx creates /d, two files and argv prog,arg1; put_str raw-copies at given coordinate; open writes path at1000 and invokes production path_open with root fd3/follow1 and output900.

The invariant is conditional on the admitted transition: Successful helper open cannot return an fd unrelated to output word900; bounded helper does not perform host directory IO.

Cost: O(path+file bytes) tiny fixture; helper raw slices require bounds as caller premise.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fixture_contents

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## pointer_assumption

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## root_adapter

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## error_propagation

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::args_and_prestat, tests::open_read_seek_write.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful helper open cannot return an fd unrelated to output word900; bounded helper does not perform host directory IO.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: fd result determines produced word at900 on success, not path or prior descriptor state; test helper inputs are fixture-defined.

### Undated · fixture_contents

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: a.txt=hello world,b.txt empty; argv nul size10. This is known test data, not arbitrary external files.

### Undated · pointer_assumption

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: put_str uses unchecked slice indexing and can panic unlike production get_str; test input lengths ensure bounds.

### Undated · root_adapter

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: open passes root3 and follow1, so does not cover every dirfd/follow combination.

### Undated · error_propagation

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-test-fixtures`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: path_open error propagated before get_u32 output; successful helper reads exact fd from memory.
