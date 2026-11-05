# wasi-engine

## descriptant

### wasi-engine

new configures fuel only when Some, validates Module and binds WASI. run creates Store with Ctx limits, sets optional fuel, instantiates/starts module then calls typed _start. Return success0, i32 exit preserved without trap, other error134 with text; fuel used=ceiling-remaining.

## approximant

### Forward relation

new configures fuel only when Some, validates Module and binds WASI. run creates Store with Ctx limits, sets optional fuel, instantiates/starts module then calls typed _start. Return success0, i32 exit preserved without trap, other error134 with text; fuel used=ceiling-remaining.

Scope: `wasi-engine`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

new configures fuel only when Some, validates Module and binds WASI. run creates Store with Ctx limits, sets optional fuel, instantiates/starts module then calls typed _start. Return success0, i32 exit preserved without trap, other error134 with text; fuel used=ceiling-remaining.

The invariant is conditional on the admitted transition: A non-exit engine error cannot yield trap=None; a normal return cannot yield134. Named allowed imports do not exclude signature mismatches.

Cost: Module-dependent validation and interpreter cost; optional fuel constrains instructions only, Store memory separate; per-run instance allocation.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fresh_context

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## import_name_filter

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## entry_requirement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fuel_observation

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::wave31_tiny_engine_outcomes.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: A non-exit engine error cannot yield trap=None; a normal return cannot yield134. Named allowed imports do not exclude signature mismatches.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful exit0 can arise from normal return or proc_exit(0); trap134 admits validation-compatible modules failing at instantiate/start/call. Missing imports list cannot recover type mismatches.

### Undated · fresh_context

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Host shares compiled module/linker but each run gets fresh Store and supplied Ctx, not previous memory.

### Undated · import_name_filter

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: missing_imports tests namespace/name membership only; known name with wrong signature still fails linking.

### Undated · entry_requirement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Valid module lacking compatible exported _start becomes run error134, not Host::new error.

### Undated · fuel_observation

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-engine`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: None produces no fuel_used; Some reports consumed instructions even on trap, not elapsed time or all host work.
