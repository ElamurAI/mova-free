# wasi-mlab-tests

## descriptant

### wasi-mlab-tests

Integration harness binds external suite texts, checks wasm freshness against mlab/src and vfs/src, caches Host and defines native/guest comparisons. Wave31 executes only two added synthetic tests: newest_mtime isolated tree and actual CLI with tiny modules/import/export/journal. Legacy mlab guest/frozen tests remain excluded.

## approximant

### Forward relation

Integration harness binds external suite texts, checks wasm freshness against mlab/src and vfs/src, caches Host and defines native/guest comparisons. Wave31 executes only two added synthetic tests: newest_mtime isolated tree and actual CLI with tiny modules/import/export/journal. Legacy mlab guest/frozen tests remain excluded.

Scope: `wasi-mlab-tests`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

Integration harness binds external suite texts, checks wasm freshness against mlab/src and vfs/src, caches Host and defines native/guest comparisons. Wave31 executes only two added synthetic tests: newest_mtime isolated tree and actual CLI with tiny modules/import/export/journal. Legacy mlab guest/frozen tests remain excluded.

The invariant is conditional on the admitted transition: Helper-only wave31 results exclude neither frozen-suite divergences nor stale wasm, since those paths are not run.

Cost: Recursive metadata O(entries+depth stack); full tests have external native/wasm costs not executed. Helper test2 files/one subdir.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## composition

### Composition

Integration harness binds external suite texts, checks wasm freshness against mlab/src and vfs/src, caches Host and defines native/guest comparisons. Wave31 executes only two added synthetic tests: newest_mtime isolated tree and actual CLI with tiny modules/import/export/journal. Legacy mlab guest/frozen tests remain excluded.

Immediate fragments partition selected mechanisms; their byte ranges refer to this single source and reuse its syntax projection. Memory helper success is a precondition for ABI transport, not proof that all linker wrappers dispatch correctly. WASI path/read/write helpers call real Vfs; mlab remains an external reference. Remaining top-level definitions and test bodies stay covered only by the parent source model.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## freshness_projection

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## frozen_exclusion

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## host_cache

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## test_effect_boundary

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: wave31_newest_mtime_isolated_tree, wave31_cli_tiny_guest_isolated_effects.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Helper-only wave31 results exclude neither frozen-suite divergences nor stale wasm, since those paths are not run.

### Undated · reverse_reconstruction

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Passing freshness means artifact mtime>=maximum leaf mtime in two trees, not reproducible build identity. Output equality permits differing states; la18 divergence is explicitly expected by legacy suite.

### Undated · freshness_projection

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Only mlab/src and vfs/src checked, not Cargo config/dependencies; timestamp ordering is weaker than source hashing.

### Undated · frozen_exclusion

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Legacy fixed250/249 and25-file scenarios not executed; no confirmation of those expectations this epoch.

### Undated · host_cache

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: OnceLock keeps one Host across guest calls, while each call supplies fresh Vfs unless explicitly passed; engine run creates fresh instance.

### Undated · test_effect_boundary

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-mlab-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Wave31 creates/writes/stats/deletes isolated temporary trees and invokes wasi-run on tiny modules only. No wasm_path, host(), native(), guest() or suite execution.
