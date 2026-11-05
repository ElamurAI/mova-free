# annot-groupant-transfers

## descriptant

# annot-groupant-transfers

## epoch-memory

This harness captures both a valid graph-property transfer and its rejected API generalization. [3,0,4,3] distinguishes first unreachable token from first cycle member; empty and self-head cases distinguish the two gates. The 64/65-token and 132-token fixtures reach dispatch and budget boundaries. Twelve common-domain vectors and four boundary-policy shapes passed in four focused runs. The wrong reverse-edge adapter failed a named assertion and was restored exactly.

## model

# annot-groupant-transfers

## forward

Two supplied-input test observations construct lawful annotation tokens and converter words from shared head vectors, invoke actual ann::check and en::convert::tree_gate, and compare accepted trees or minimum root-unreachable-token projections with reversed-head BFS. Separate fixtures assert the intentionally different empty, self-head, count/form and ordered error policies. Expanded inputs exercise the hybrid dispatch without changing these named test relations.

## invariant

The harness keeps operation tags and compares cross-gate outputs only after establishing shared representation and preflight assumptions. An unreachable tail can be the first reported token without being a cycle vertex. Boundary differences are asserted explicitly rather than normalized into false equivalence.

## cost

For fixture vectors h, total harness cost is sum_h of construction plus both gates plus the BFS observation. Production cycle-phase costs are O(n_h^2) with constant auxiliary walk space for n_h<=64, and O(n_h) with up to O(n_h) BFS storage after the <=4n_h budget for larger vectors. Fixture enumeration, diagnostics and source-included test modules add separate compile/test-run scope; no timing claim.

## composition

Test-only join: one supplied head vector produces two lawful record representations and an independent reachability observation. Calls target the real annotation and conversion gates; neither production gate calls the other. On admitted common-domain fixtures, compare acceptance or first root-unreachable token. On boundary fixtures, assert the distinct exact Result observations. Source-local imported unit tests are separate from the two groupant_ tests.

## checks

["groupant_common_tree_and_rejected_cycle_only_projection","groupant_distinct_empty_self_head_and_error_policies","Run only groupant_ external tests when distinguishing them from included source-local tests"]

## limitations

Finite checks do not establish full annotation normalization or converter action semantics. Installed fixtures include 12 shared vectors (eight accepted, four cycle cases), lengths 64/65 and a 132-token budget-exhaustion case. Exact root error strings and stable one-based indexing are part of the checked observation. Paired syntax is byte-identical; named tests passed twice in debug and twice in release.

## catalog

# Typed transfer relations

Source-derived models; native evidence pending.

## excluded-fixture-projections

Type: behavior_complement.

For the finite supplied-call universe, the modeled excluded observations include accepting a root-disconnected lawful vector, reporting token 3 instead of token 1 for [3,0,4,3], and claiming both gates share empty/self-head outcomes. The first test samples common-domain exclusions and the second samples boundary exclusions; passing them is not a complement model for every parser input.

## compatible-fixture-inputs

Type: reverse_reconstruction.

Given a projected accepted/rejected result, retain all lawful fixture vectors and token fields producing it under both actual gates. A rejected first token does not identify the rest of the cycle or lexical fields. Given a boundary error, retain the operation tag and earlier predicates; no unique input is recovered.

## root-projection

Type: first_error_projection.

The graph observation checks reversed-head reachability from virtual root 0 and selects the smallest unreachable token. It is compared with both gates only after their local preflight obligations are satisfied. Small and expanded vectors use the same observation relation.

Named observation: groupant_common_tree_and_rejected_cycle_only_projection.

## separate-preflights

Type: boundary_non_equivalence.

Empty converter input succeeds while annotation rejects Roots(0); self-head behavior differs; annotation count/form checks do not map to universal converter checks. Exact tagged observations constrain a rejected full-API replacement.

Named observation: groupant_distinct_empty_self_head_and_error_policies.

## hybrid-observation-scope

Type: resource_dispatch_obligation.

Expanded fixtures must exercise n<=64 walk behavior, larger cheap completion and budget-exhausted BFS while retaining exact results. Length alone does not prove the budget was exhausted; shape and source control flow are needed. The named projection test provides outcomes, and source-local checks cover dispatch internals.

Named observation: groupant_common_tree_and_rejected_cycle_only_projection.

## evidence

# annot-groupant-transfers evidence

## checks (evidence)

See models/mova.groupant/model.md#transfer-evidence, transfer-benchmark.md and transfer-validation.md for the combined receipts. Source revisions are bound by this manifest and paired partial syntax. Finite checks do not establish universal equivalence, floating-point optimality, parser quality or whole-system speed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 finite transfer integration evidence.

### 2026-10-30 · checks (evidence)

2026-10-30: 2 focused checks passed twice in debug and twice in release with identical normalized observations. Two named groupant_ observations only; imported ann tests are counted separately. Twelve shared vectors (eight valid, four cyclic) include 64/65-length cases and the 132-token budget-exhaustion case. Four policy shapes distinguish empty, self-head, range and count/form behavior. A reversed-edge adapter mutant failed; exact restoration passed.
