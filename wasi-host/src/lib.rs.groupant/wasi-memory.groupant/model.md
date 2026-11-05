# wasi-memory

## descriptant

### wasi-memory

range accepts [ptr,ptr+len) only with checked end<=memory length. off checks u32 overflow. iovs rejects n>1024 before coordinates, then reads n pairs of LE u32; get_str validates UTF8 after copying. filestat emits64 bytes with fixed offsets.

## approximant

### Forward relation

range accepts [ptr,ptr+len) only with checked end<=memory length. off checks u32 overflow. iovs rejects n>1024 before coordinates, then reads n pairs of LE u32; get_str validates UTF8 after copying. filestat emits64 bytes with fixed offsets.

Scope: `wasi-memory`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

range accepts [ptr,ptr+len) only with checked end<=memory length. off checks u32 overflow. iovs rejects n>1024 before coordinates, then reads n pairs of LE u32; get_str validates UTF8 after copying. filestat emits64 bytes with fixed offsets.

The invariant is conditional on the admitted transition: Successful range excludes end>memory length; decoded LE scalar excludes different values for identical selected bytes. n>1024 excludes successful iovec decode irrespective of ptr.

Cost: O(n) iovec table and allocation, O(bytes) string/copy; scalar accesses O(1).

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## zero_coordinates

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## fault_precedence

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## byte_layout

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## integer_boundary

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::args_and_prestat, tests::wave31_memory_poll_and_descriptor_boundaries.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful range excludes end>memory length; decoded LE scalar excludes different values for identical selected bytes. n>1024 excludes successful iovec decode irrespective of ptr.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful scalar loads recover the selected bytes uniquely, but all other bytes and original provenance remain arbitrary. A FAULT permits many bad coordinates; ILSEQ implies an in-range invalid UTF8 slice.

### Undated · zero_coordinates

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: range(len,0) succeeds while range(len+1,0) faults; zero iovs reads no table bytes.

### Undated · fault_precedence

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: n>1024 yields INVAL even when ptr=u32::MAX; malformed table coordinates yield FAULT before file lookup.

### Undated · byte_layout

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: filestat dev zero, ino at8, type16, nlink24=1, size32, atim40=mtime, mtim48, ctim56.

### Undated · integer_boundary

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-memory`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: off(u32::MAX,1) faults; range uses usize checked addition; put casts slice length to u32, so giant host slices are outside tested domain.
