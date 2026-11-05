# wasi-cli

## descriptant

### wasi-cli

CLI parses leading options, reads and validates module, rejects unknown imports, imports host directories to bounded Vfs, runs with echo, conditionally exports, optionally journals/stats, then clamps exit to0..255.

## approximant

### Forward relation

CLI parses leading options, reads and validates module, rejects unknown imports, imports host directories to bounded Vfs, runs with echo, conditionally exports, optionally journals/stats, then clamps exit to0..255.

Scope: `wasi-cli`. State and transitions follow the authoritative source, including partial effects before errors. No executable approximate replacement is attached: categorical errno/effects and module execution lack a useful validated percentage metric; future numerical profiles may only use1/2/5/10% caps after domain/reference evidence.

## invariant

### Invariant and cost

CLI parses leading options, reads and validates module, rejects unknown imports, imports host directories to bounded Vfs, runs with echo, conditionally exports, optionally journals/stats, then clamps exit to0..255.

The invariant is conditional on the admitted transition: A trapped guest cannot trigger export through the normal gate; missing option value cannot run a module. A journal-write error does not necessarily exclude exit0.

Cost: O(args bytes+module bytes+import/export bytes)+Vfs and engine costs; host IO unbounded by guest fuel.

No wall-time gain, universal resource bound or atomic rollback is inferred. External Vfs/wasmi/native-mlab work remains separately charged.

## composition

### Composition

CLI parses leading options, reads and validates module, rejects unknown imports, imports host directories to bounded Vfs, runs with echo, conditionally exports, optionally journals/stats, then clamps exit to0..255.

Immediate fragments partition selected mechanisms; their byte ranges refer to this single source and reuse its syntax projection. Memory helper success is a precondition for ABI transport, not proof that all linker wrappers dispatch correctly. WASI path/read/write helpers call real Vfs; mlab remains an external reference. Remaining top-level definitions and test bodies stay covered only by the parent source model.

## models

# Typed model catalog

## behavior_complement

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## reverse_reconstruction

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## option_boundary

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## export_gate

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## journal_failure

Status: partial source-derived relation; complement is a scoped exclusion hypothesis, reverse preserves all compatible antecedents described rather than a chosen inverse. Assumptions: production branch order, declared coordinate/resource premises; no completeness claim for untested inputs. Evidence: model.md#tests distinguishes actual execution from source review; finite observations can falsify but not prove the relation.

## memory_cast

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

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: A trapped guest cannot trigger export through the normal gate; missing option value cannot run a module. A journal-write error does not necessarily exclude exit0.

### Undated · reverse_reconstruction

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Exit2 admits usage/module/import/export failures. Final exit cannot reconstruct options or filesystem history; journal error can coexist with successful guest exit.

### Undated · option_boundary

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Only leading -- arguments are host options; suffix after module becomes guest argv. stats consumes one token; other flags require value.

### Undated · export_gate

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Export occurs iff guest code0 and no trap; first export error returns2 before journal/stats, prior exports may persist.

### Undated · journal_failure

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: Journal write errors print but do not alter final exit; no atomic export+journal transaction.

### Undated · memory_cast

Subject: `wasi-host/src/main.rs` at SHA-256 `2200229297d1dde6d9f8477a577a7320c854d0db4fb3ea450b457a9858107122`, scope `wasi-cli`. Domain: the source transition and bounded fixtures described in model.md#approximant; readable valid host resources where required. Observation: returned values, byte coordinates, state/effect order, not total-system semantics. Operator/relation: parse_size returns integer cast to usize; architecture width matters. Seed/fuel parse u64; default mem1GiB.
