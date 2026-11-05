# roundtrip-derived-piece-codec-line-script

## descriptant

### Description and epoch memory

Bottom-up LCS lengths feed greedy keep/insert/drop. apply removes indexed source lines then inserts stored lines at output positions; script_bits charges drop positions plus stored-line bytes/positions.

## approximant

### Forward relation

Bottom-up LCS lengths feed greedy keep/insert/drop. apply removes indexed source lines then inserts stored lines at output positions; script_bits charges drop positions plus stored-line bytes/positions.

Domain: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

No executable percentage surrogate is supplied: these categorical codec/effect observations have no justified lossy metric. Future caps are 1/2/5/10% only after defining metric, zero behavior and differential reference.

## catalog

# Source-bound typed models

Each section is a partial source-derived relation on the bound snapshot and declared domain; finite checks are falsification evidence, not a universal proof. Observation horizon is one call unless history is explicitly named.

## behavior_complement

Generated script never reorders retained lines; equal-match path keeps before edit; ties prefer insertion.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## reverse_reconstruction

Fixed a and script imply b uniquely. Given b only, deleted source lines are unconstrained; empty target admits all a paired with dropping every line.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## newline_units

split_inclusive preserves final newline absence; a and a-newline are distinct units.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## insertion_tie

Equal DP alternatives choose put at output j before dropping input.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## drop_coordinates

drop indexes original source lines, put indexes final output lines.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## cost_domain

LCS max<=min(n,m) must fit u32; bit costs are symbolic, not actual serialized storage.

Domain and held state: Same model/lexicon/environment at encode/decode; valid references and 10-column patches; LCS dimensions/counts fit usize/u32 and memory. Tests execute pure scripts/tree validation/reference/damage, not model-backed encoder.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.


## tests

### Actual checks

- `blocks::site_code::wave32::script_fibers_and_reference_precedence`
- `blocks::site_code::tests::line_scripts_give_the_target_back`
- `blocks::site_code::tests::a_tree_needs_known_relations_and_a_root`

The listed parent/source tests exercise only their named mechanisms; other model statements are source inspection. Sixteen selected actual roundtrip library tests passed twice debug and twice release (64 exact-name executions), with identical sorted test outcomes. Eight compiled production-behavior mutants, one per source, failed assertions; exact originals were restored and the selected check passed. No assertion-only mutants or surviving compiled mutants. An initial mutation selector was ambiguous and stopped before writing that source; corrected selector retained separately.

## invariant-and-cost

### Invariants and cost

Generated script never reorders retained lines; equal-match path keeps before edit; ties prefer insertion.

Fixed a and script imply b uniquely. Given b only, deleted source lines are unconstrained; empty target admits all a paired with dropping every line.

#### Cost

O(nm) u32 matrix, O(n+m) traceback, apply O(n log n+output). Allocation can fail for unbounded n,m.

Integer additions/products and allocation sizes must fit native types. Floating costs require finite values; no throughput or all-input guarantee follows from synthetic tests.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained externally before edits. Existing production bytes preserved; only bounded inline tests appended to six owned files. Models distinguish byte reconstruction with residual from semantic understanding and persistent learning. Finite tests and compiled mutants support selected boundaries only. Source inspection identified noninjective projections and explicit admissibility restrictions; no bug repair or performance claim.

Combiner input is this source snapshot, syntax projection, models, costs and retained receipts. Proposed next epoch: target the stated unexecuted branches with bounded witnesses before expanding any claim. AST remains partial, macros opaque; no name resolution or proof.

### 2026-10-31 · tests

Raw logs, numeric statuses, original/restored hashes, prospective.md, native-commands.json and syntax pairs: `$MOVA_DATA/runs/octopus-wave32-rt_blocks-2026-10-31`. No live model/corpus/site/learning workflow or full pipeline was executed.
