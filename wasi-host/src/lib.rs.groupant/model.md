# wasi-runtime

## descriptant

### wasi-runtime

Host validates a wasm module, registers preview1 functions, then creates a fresh Store per run. Ctx transports guest byte coordinates through descriptor/path checks to Vfs; captures streams, virtual time, seeded randomness and call counts. Macro wrappers require exported memory before counting. proc_exit is separate and preserves its code; other traps return134. Name-only missing_imports is weaker than successful linking.

## approximant

### Forward relation

Host validates a wasm module, registers preview1 functions, then creates a fresh Store per run. Ctx transports guest byte coordinates through descriptor/path checks to Vfs; captures streams, virtual time, seeded randomness and call counts. Macro wrappers require exported memory before counting. proc_exit is separate and preserves its code; other traps return134. Name-only missing_imports is weaker than successful linking.

Scope: `wasi-runtime`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

Host validates a wasm module, registers preview1 functions, then creates a fresh Store per run. Ctx transports guest byte coordinates through descriptor/path checks to Vfs; captures streams, virtual time, seeded randomness and call counts. Macro wrappers require exported memory before counting. proc_exit is separate and preserves its code; other traps return134. Name-only missing_imports is weaker than successful linking.

The invariant is conditional on the admitted transition: Within validated memory and available resources, a normal successful _start return cannot yield a trap; a socket wrapper with memory cannot perform network IO. FAULT does not exclude preceding Vfs/clock effects.

Cost: Validation O(module bytes) plus engine-specific validation; run interpreter work up to optional fuel, with separate host work and allocations. fd allocation O(slots); iov O(n+bytes)+Vfs; random O(len); poll O(subscriptions); readdir O(entries+names).

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## composition

### Composition

Host validates a wasm module, registers preview1 functions, then creates a fresh Store per run. Ctx transports guest byte coordinates through descriptor/path checks to Vfs; captures streams, virtual time, seeded randomness and call counts. Macro wrappers require exported memory before counting. proc_exit is separate and preserves its code; other traps return134. Name-only missing_imports is weaker than successful linking.

Immediate fragments partition selected mechanisms; their byte ranges refer to this single source and reuse its syntax projection. Memory helper success is a precondition for ABI transport, not proof that all linker wrappers dispatch correctly. WASI path/read/write helpers call real Vfs; mlab remains an external reference. Remaining top-level definitions and test bodies stay covered only by the parent source model.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## memory_transport

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## failure_order

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## capability_boundary

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## engine_resource

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::args_and_prestat, tests::clock_is_virtual_and_random_is_seeded, tests::escape_and_absolute_paths_are_not_capable, tests::limits_reach_the_guest_as_errno, tests::open_read_seek_write, tests::readdir_with_small_buffer_and_cookies, tests::wave31_late_faults_keep_prior_effects, tests::wave31_memory_poll_and_descriptor_boundaries, tests::wave31_tiny_engine_outcomes.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Within validated memory and available resources, a normal successful _start return cannot yield a trap; a socket wrapper with memory cannot perform network IO. FAULT does not exclude preceding Vfs/clock effects.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: An Outcome admits every module/context pair producing its code, trap, buffers and final Vfs. Output capture loses discarded stderr/echo failures and prior descriptor offsets; exit0 alone neither identifies a module nor implies filesystem equivalence.

### Undated · memory_transport

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Guest pointers are u32 and words little endian; checked range/off reject invalid access. Signed ABI arguments cast to unsigned coordinates; this is no signed-input sanitizer.

### Undated · failure_order

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Late output FAULT may follow file creation, stream emission, descriptor updates or clock advancement. No rollback contract.

### Undated · capability_boundary

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: at rejects absolute/empty paths after UTF8 and descriptor checks; relative traversal resolution delegates to Vfs and journal. Rights metadata attenuation does not change Kind::File read/write booleans.

### Undated · engine_resource

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-runtime`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Fuel limits engine instructions, not total host allocation/time; memory StoreLimits and max_fds are separate. No percentage surrogate for traps/effects.
