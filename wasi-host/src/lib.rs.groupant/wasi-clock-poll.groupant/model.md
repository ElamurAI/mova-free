# wasi-clock-poll

## descriptant

### wasi-clock-poll

String sizes write count then total nul-terminated bytes; strings_get writes pointer then bytes then terminator. clock validates id<=3, reads now and advances before put. random validates range before consuming ceil(len/8) splitmix words. poll validates1..4096 subscriptions, prioritizes fd events; otherwise advances minimum timer and emits every minimum tie.

## approximant

### Forward relation

String sizes write count then total nul-terminated bytes; strings_get writes pointer then bytes then terminator. clock validates id<=3, reads now and advances before put. random validates range before consuming ceil(len/8) splitmix words. poll validates1..4096 subscriptions, prioritizes fd events; otherwise advances minimum timer and emits every minimum tie.

Scope: `wasi-clock-poll`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

String sizes write count then total nul-terminated bytes; strings_get writes pointer then bytes then terminator. clock validates id<=3, reads now and advances before put. random validates range before consuming ceil(len/8) splitmix words. poll validates1..4096 subscriptions, prioritizes fd events; otherwise advances minimum timer and emits every minimum tie.

The invariant is conditional on the admitted transition: A valid timer-only poll cannot omit one of the minimum ties when output storage fits; fd-ready poll cannot advance timer clock. Invalid random range cannot consume RNG state.

Cost: O(argument bytes+count), O(random len) allocated len+8; poll O(nsubs) space/time plus O(events) writes.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## clock_coordinates

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## timer_ties

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## random_zero

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## partial_output

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: tests::clock_is_virtual_and_random_is_seeded, tests::wave31_memory_poll_and_descriptor_boundaries, tests::wave31_late_faults_keep_prior_effects.

Memory tests bypass linker ABI; tiny empty/unreachable guests exercise engine outcomes only. The CLI test uses isolated import/export/journal fixtures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

Independent review narrowed reverse reconstruction: emitted event bytes carry userdata/type, not timestamps. The minimum timer duration is recoverable only with before/after clock observations, timer-only execution and nonsaturating advancement. Event bytes alone leave durations ambiguous.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: A valid timer-only poll cannot omit one of the minimum ties when output storage fits; fd-ready poll cannot advance timer clock. Invalid random range cannot consume RNG state.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Event bytes recover emitted userdata and event type, but contain no timer timestamp or duration. For timer-only execution, observing both before and after virtual clock values recovers the minimum relative duration only when clock advancement does not saturate; larger timer durations remain ambiguous. Descriptor-ready events do not establish any timer duration. Random output truncates the final64-bit word, hiding remaining bytes.

### Undated · clock_coordinates

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: id0 returns epoch time;1..3 return now-EPOCH_NS. Invalid id returns INVAL without advance; valid id with bad out advances then FAULT.

### Undated · timer_ties

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Equal minimum durations all emit events in input order; any type1/2 subscription suppresses timer advancement.

### Undated · random_zero

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: len0 consumes no generator state, provided zero-length range valid; invalid range consumes no state.

### Undated · partial_output

Subject: `wasi-host/src/lib.rs` at SHA-256 `91213ce9038107912c2dd707c3a6639a2d723cdfceb02981a166a6853800248e`, scope `wasi-clock-poll`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: String pointer cells and prior events can be written before a later memory fault; no all-or-none output promise.
