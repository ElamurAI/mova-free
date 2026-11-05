# wasi-cli-effects

## descriptant

### wasi-cli-effects

Construct limited Vfs, sequentially import each real->virtual pair, derive basename argv and append suffix, apply seed/mem/echo, run; exports gated on code0+no trap. Journal/stat occur after export loop.

## approximant

### Forward relation

Construct limited Vfs, sequentially import each real->virtual pair, derive basename argv and append suffix, apply seed/mem/echo, run; exports gated on code0+no trap. Journal/stat occur after export loop.

Scope: `wasi-cli-effects`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

Construct limited Vfs, sequentially import each real->virtual pair, derive basename argv and append suffix, apply seed/mem/echo, run; exports gated on code0+no trap. Journal/stat occur after export loop.

The invariant is conditional on the admitted transition: Once import fails, guest execution and exports are absent; once export fails, later exports/journal/stats are absent. Earlier exports need not be undone.

Cost: O(import+export traversal bytes)+Host run; journal serializes all records; stats scans distinct calls.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## import_failure

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## basename_argv

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## export_order

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## status_clamp

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.


## tests

### Checks and limits

Named actual-source tests run twice per debug/release; exact commands and numeric statuses retained externally. Test success is finite evidence only.

Exact names: wave31_cli_tiny_guest_isolated_effects.

wave31_cli_tiny_guest_isolated_effects invokes actual wasi-run with one tiny empty guest and one unreachable guest, root-local import/export/journal and two usage failures.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review recorded before edits. Source review found late output FAULT effects and weaker-than-capability rights metadata; these are modeled as current behavior rather than silently repaired. Only bounded new test blocks were added; production code remains unchanged. Actual finite results and exact restoration receipts live under $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. No full mlab guest, frozen suite, GPU or corpus run. This node retains its scoped observations for future refinement.

### 2026-10-31 · tests

Five compiled production/helper-source controls: append offset, virtual clock advance, trap status, CLI export gate and newest-mtime max. Each compiles, fails its selected assertion, restores exact original bytes and passes again. These controls cover their target relations, not every node. No assertion-only mutants. See external source-test-map.json, native-commands.json, verify.rs and numeric/raw receipts in $MOVA_DATA/runs/octopus-wave31-wasi-2026-10-31. The example main and frozen mlab paths are never executed.

### Undated · behavior_complement

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Once import fails, guest execution and exports are absent; once export fails, later exports/journal/stats are absent. Earlier exports need not be undone.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Successful exit with journal absent admits no request or failed journal write. Export tree loses guest descriptor/call history.

### Undated · import_failure

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: First import failure returns2 before guest, preserving any earlier host reads but no exports.

### Undated · basename_argv

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: argv0 is module basename or fallback module; directory path erased from guest argv0.

### Undated · export_order

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Exports are sequential and no rollback; first write failure returns2, later exports and journal skipped.

### Undated · status_clamp

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli-effects`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Returned code is clamp(out.exit_code,0,255); trap normally134 but printed trap alone is not return code logic.
