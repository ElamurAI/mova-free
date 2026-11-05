# wasi-memory-tests

## descriptant

### wasi-memory-tests

Nine bounded tests exercise private Ctx helpers with synthetic Vfs and byte memory; six legacy tests cover args/open/escape/readdir/time/limits, three wave31 tests add late faults, memory/poll/descriptors and tiny binary Host run. Helpers do not route every WASI linker export.

## approximant

### Forward relation

Nine bounded tests exercise private Ctx helpers with synthetic Vfs and byte memory; six legacy tests cover args/open/escape/readdir/time/limits, three wave31 tests add late faults, memory/poll/descriptors and tiny binary Host run. Helpers do not route every WASI linker export.

Scope: `wasi-memory-tests`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

Nine bounded tests exercise private Ctx helpers with synthetic Vfs and byte memory; six legacy tests cover args/open/escape/readdir/time/limits, three wave31 tests add late faults, memory/poll/descriptors and tiny binary Host run. Helpers do not route every WASI linker export.

The invariant is conditional on the admitted transition: A passing finite suite excludes the particular failed assertions in its named fixtures, not untested guest or resource behavior.

Cost: Fixed fixtures<=8192 bytes; finite Vfs tiny files, two tiny wasm bodies, no host filesystem/corpus.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## composition

### Composition

Nine bounded tests exercise private Ctx helpers with synthetic Vfs and byte memory; six legacy tests cover args/open/escape/readdir/time/limits, three wave31 tests add late faults, memory/poll/descriptors and tiny binary Host run. Helpers do not route every WASI linker export.

Immediate fragments partition selected mechanisms; their byte ranges refer to this single source and reuse its syntax projection. Memory helper success is a precondition for ABI transport, not proof that all linker wrappers dispatch correctly. WASI path/read/write helpers call real Vfs; mlab remains an external reference. Remaining top-level definitions and test bodies stay covered only by the parent source model.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fixture_coordinates

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## vfs_join

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## engine_scope

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fault_witness

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

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: A passing finite suite excludes the particular failed assertions in its named fixtures, not untested guest or resource behavior.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Passing assertions constrain observed fixtures only; many alternate implementations agree on these finite cases. Test names/output cannot reconstruct full state or show complete preview1 compliance.

### Undated · fixture_coordinates

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: open helper copies path at1000 and writes fd at900, requiring sufficiently large buffer; helper itself can panic on small memory.

### Undated · vfs_join

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: open/read/write tests call production Ctx methods which call real Vfs; read-only, EXCL, holes, append and limits observable.

### Undated · engine_scope

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Tiny engine fixture has no imports; verifies run outcome/fuel and unreachable, not generated guest ABI imports.

### Undated · fault_witness

Subject: `wasi-host/src/tests.rs` at SHA-256 `74685e5440b3b7df7f40518395e62d6cd68a60e80ff71e87b772f020c80bec8d`, scope `wasi-memory-tests`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Late faults retain clock, stdout, created file and allocated descriptor; tests explicitly reject transactional overclaims.
