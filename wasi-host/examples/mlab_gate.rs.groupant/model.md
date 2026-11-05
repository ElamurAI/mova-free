# wasi-mlab-gate

## descriptant

### wasi-mlab-gate

Example reads fixed mlab.wasm, constructs Host then for each .txt path parses scenarios; each runs native capture with fresh Vfs and guest with fresh Vfs/merged stderr, appends textual trap, compares output strings, accumulates wall times and divergence list. Main not executed in wave31.

## approximant

### Forward relation

Example reads fixed mlab.wasm, constructs Host then for each .txt path parses scenarios; each runs native capture with fresh Vfs and guest with fresh Vfs/merged stderr, appends textual trap, compares output strings, accumulates wall times and divergence list. Main not executed in wave31.

Scope: `wasi-mlab-gate`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

Example reads fixed mlab.wasm, constructs Host then for each .txt path parses scenarios; each runs native capture with fresh Vfs and guest with fresh Vfs/merged stderr, appends textual trap, compares output strings, accumulates wall times and divergence list. Main not executed in wave31.

The invariant is conditional on the admitted transition: With no differences and successful traversal, final status1 is excluded; matching output does not exclude nonzero guest exit or different Vfs.

Cost: O(suite bytes+outputs+diff text)+sum native and guest run costs, unbounded scenario workload. No measured speed ratio.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## composition

### Composition

Example reads fixed mlab.wasm, constructs Host then for each .txt path parses scenarios; each runs native capture with fresh Vfs and guest with fresh Vfs/merged stderr, appends textual trap, compares output strings, accumulates wall times and divergence list. Main not executed in wave31.

Immediate fragments partition selected mechanisms; their byte ranges refer to this single source and reuse its syntax projection. Memory helper success is a precondition for ABI transport, not proof that all linker wrappers dispatch correctly. WASI path/read/write helpers call real Vfs; mlab remains an external reference. Remaining top-level definitions and test bodies stay covered only by the parent source model.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## argument_selection

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## external_reference

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## comparison_projection

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## empty_suite

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

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: With no differences and successful traversal, final status1 is excluded; matching output does not exclude nonzero guest exit or different Vfs.

### Undated · reverse_reconstruction

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: same/total and summed times admit arbitrarily many code/output histories; output equality does not recover state equivalence or successful nonzero-exit status.

### Undated · argument_selection

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: vm enabled by any --vm, fuel uses first --fuel next value parse; only args ending .txt are suite paths.

### Undated · external_reference

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: mlab Interp and suite parser are external references, not owned runtime implementation; native/guest equality is a gate hypothesis.

### Undated · comparison_projection

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Guest stdout decoded lossily and concatenated trap; nonzero proc_exit without trap may compare equal because exit_code is ignored.

### Undated · empty_suite

Subject: `wasi-host/examples/mlab_gate.rs` at SHA-256 `a7947069ab507ffd5ad7b71ae9146bca16e08b827af78bc5961993c788b699b8`, scope `wasi-mlab-gate`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Even zero selected txt files still reads/builds Host; afterward0/0 and exit0 if no differences.
