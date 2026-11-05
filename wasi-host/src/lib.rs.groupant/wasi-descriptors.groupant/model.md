# wasi-descriptors

## descriptant

### wasi-descriptors

read decodes table, reads/copies Vfs or stdin and advances implicit offset before guest writes. write gathers all input bytes before descriptor lookup, writes/echoes then stores count. Explicit positional IO leaves file offset. seek computes i128 base+delta in0..i64::MAX then updates before output. readdir materializes dot/dotdot/list and copies a possibly truncated prefix from cookie.

## approximant

### Forward relation

read decodes table, reads/copies Vfs or stdin and advances implicit offset before guest writes. write gathers all input bytes before descriptor lookup, writes/echoes then stores count. Explicit positional IO leaves file offset. seek computes i128 base+delta in0..i64::MAX then updates before output. readdir materializes dot/dotdot/list and copies a possibly truncated prefix from cookie.

Scope: `wasi-descriptors`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

read decodes table, reads/copies Vfs or stdin and advances implicit offset before guest writes. write gathers all input bytes before descriptor lookup, writes/echoes then stores count. Explicit positional IO leaves file offset. seek computes i128 base+delta in0..i64::MAX then updates before output. readdir materializes dot/dotdot/list and copies a possibly truncated prefix from cookie.

The invariant is conditional on the admitted transition: Successful read-only file descriptor writes are excluded; explicit positional write cannot alter fd position. Output FAULT does not exclude earlier write/emission.

Cost: O(iovs+payload) allocation/copy plus Vfs read/write; directory listing O(entries+names), output prefix construction may exceed len by one full entry.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## append_position

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## rights_and_kind

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## seek_range

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## cookie_prefix

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::open_read_seek_write, tests::readdir_with_small_buffer_and_cookies, tests::limits_reach_the_guest_as_errno, tests::wave31_late_faults_keep_prior_effects.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful read-only file descriptor writes are excluded; explicit positional write cannot alter fd position. Output FAULT does not exclude earlier write/emission.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Observed bytes and count constrain a prefix at old offset; EOF makes many offsets compatible. Appended file state admits many prior contents and writes unless original size and input bytes are fixed.

### Undated · append_position

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Explicit pos_at overrides append; implicit append uses current file size. Non-positional write updates fd position by payload length.

### Undated · rights_and_kind

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: read/write booleans cached in Kind control access; stdout/stderr reject reads; directories map ISDIR and other nonfile seek maps SPIPE.

### Undated · seek_range

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Negative/new>i64::MAX returns INVAL before set_pos; invalid output pointer can fail after valid position change.

### Undated · cookie_prefix

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-descriptors`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Cookie skips entries, . and .. count; truncated entry allowed, used==len suggests retry but does not itself prove remaining entries.
