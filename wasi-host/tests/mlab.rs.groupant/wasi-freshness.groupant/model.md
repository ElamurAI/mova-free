# wasi-freshness

## descriptant

### wasi-freshness

newest_mtime recursively computes max of leaf modified times starting UNIX_EPOCH; directories contribute recursive leaves, not directory timestamp. read_dir errors unwrap; entry errors are flattened away. wasm_path checks metadata and two source trees then returns artifact path.

## approximant

### Forward relation

newest_mtime recursively computes max of leaf modified times starting UNIX_EPOCH; directories contribute recursive leaves, not directory timestamp. read_dir errors unwrap; entry errors are flattened away. wasm_path checks metadata and two source trees then returns artifact path.

Scope: `wasi-freshness`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

newest_mtime recursively computes max of leaf modified times starting UNIX_EPOCH; directories contribute recursive leaves, not directory timestamp. read_dir errors unwrap; entry errors are flattened away. wasm_path checks metadata and two source trees then returns artifact path.

The invariant is conditional on the admitted transition: With readable leaves and no pre-epoch timestamps, returned maximum cannot be below an observed leaf time. Empty directory excludes any non-epoch result.

Cost: O(entries) metadata calls and depth stack; follows is_dir semantics so cycles/resource errors not bounded by helper itself.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## empty_tree

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## timestamp_ties

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## error_partition

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## isolated_oracle

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: wave31_newest_mtime_isolated_tree.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: With readable leaves and no pre-epoch timestamps, returned maximum cannot be below an observed leaf time. Empty directory excludes any non-epoch result.

### Undated · reverse_reconstruction

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Maximum mtime constrains all observed leaves<=max and some attaining max unless empty; directory layout, file bytes and ignored entry errors are not recoverable.

### Undated · empty_tree

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Empty tree returns UNIX_EPOCH regardless directory modified time.

### Undated · timestamp_ties

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Equal newest file times share max; no unique newest path returned.

### Undated · error_partition

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: read_dir/metadata/modified panic; individual read_dir entry errors dropped by flatten, so not complete error reporting.

### Undated · isolated_oracle

Subject: `wasi-host/tests/mlab.rs` at SHA-256 `e3b35b1a315b6d995fd3cb8e590209d9f57141b89d9def132cd733f31d8f2db8`, scope `wasi-freshness`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Two explicitly written files yield oracle max(metadata(a),metadata(sub/b)); cleanup after caught assertion panic.
