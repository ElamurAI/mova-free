# wasi-gate-scenarios

## descriptant

### wasi-gate-scenarios

For each parsed scenario instantiate native Vfs and interpreter, then guest argv mlab,-e,code,--fs,real plus optional --vm; merge stderr, append trap, compare n==g, increment total/times and same or diffs.

## approximant

### Forward relation

For each parsed scenario instantiate native Vfs and interpreter, then guest argv mlab,-e,code,--fs,real plus optional --vm; merge stderr, append trap, compare n==g, increment total/times and same or diffs.

Scope: `wasi-gate-scenarios`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

For each parsed scenario instantiate native Vfs and interpreter, then guest argv mlab,-e,code,--fs,real plus optional --vm; merge stderr, append trap, compare n==g, increment total/times and same or diffs.

The invariant is conditional on the admitted transition: For one completed scenario, same and diff cannot both increment; guest stdout equality with native is insufficient to exclude differing stderr before merge settings.

Cost: Sum scenario parsing and native/guest costs; captures O(output bytes), diffs retained O(total differing output).

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fresh_vfs

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## trap_text

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## counter_relation

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## timing_scope

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Source review and compilation only for this scope; no runtime branch-coverage claim.

Exact names: .

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: For one completed scenario, same and diff cannot both increment; guest stdout equality with native is insufficient to exclude differing stderr before merge settings.

### Undated · reverse_reconstruction

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: A diff string preserves scenario id and debug outputs but not Vfs state, fuel or guest exit. Equal output admits different states and errors excluded by comparison.

### Undated · fresh_vfs

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: No scenario shares Vfs; guest real means WASI guest API mapping, not host real filesystem.

### Undated · trap_text

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Trap Some contributes literal <trap: text>; its formatting can affect equality independent of stdout.

### Undated · counter_relation

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: total increments once per scenario, same iff n==g; diffs.len=total-same absent panic.

### Undated · timing_scope

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-gate-scenarios`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Native duration includes interpreter creation/run/output extraction; guest duration includes host.run only; timing comparison has differing setup scope.
