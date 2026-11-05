# world-schooltools-editing-restore-selection

## forward

Resolve snapshot copy by parent-basename/file-basename. Read saved then current lines split on newline. Refuse unequal line counts, then refuse any zero or oversized requested index before writing. Copy selected saved lines in list order, count only actual differences, write whole joined buffer, report changes.

## invariant

On preflight refusal the current file is unchanged. Successful restoration preserves all unselected lines and exact saved selected bytes including CR. Duplicate selected indices can increase requested count but not changed count twice.

## cost

O(saved bytes+current bytes+indices+output bytes), O(saved/current bytes) memory; filesystem latency/atomicity unmeasured.

## composition

line_numbers normally sorts/deduplicates CLI indices, but public restore_lines admits duplicates. snapshot supplies copy naming. Count/index validation precedes the replacement loop and final fs::write; write failures may partially affect disk, outside preflight invariant.

## behavior-complement

Exclude a preflight-refused call writing any prefix of valid requested lines; exclude indexing zero or above line count. Exclude changing an unselected line on successful I/O. No exclusion of partial writes on filesystem failure is claimed.

## reverse-reconstruction

Given saved file, index list and successful final current file, compatible pre-files agree with final bytes outside selected lines and have equal split-line cardinality; selected earlier bytes are unconstrained. Changed count additionally constrains how many selected distinct lines differed.

## preflight_order

Read saved before current; line-count mismatch precedes index validation; any invalid index anywhere in list prevents the whole restore. Valid-first/invalid-later is not a prefix commit.

## duplicate_accounting

After the first restoration of x its buffer line equals saved[x]; a second x contributes zero to changed, while report denominator remains input-list length.

## newline_cardinality

split('\n') retains trailing empty row: final newline affects cardinality and can make otherwise similar logical lines incompatible. CR is part of each restored row.

## filesystem_boundary

Ordinary fs::write is not atomic or crash durable. Fixture success and preflight non-effects concern uniquely created temporary paths only; live lesson path resolution remains unexecuted.

## checks

Exact world library selections: schooltools::wave34_models::restore_preflight_and_duplicate_selection. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
