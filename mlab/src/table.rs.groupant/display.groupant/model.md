# mlab-table-display

## descriptant

### mlab-table-display

cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

## approximant

### display: forward relation

cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

Domain: Well-shaped public Table/Mat/StrArr, in-range zero-based internal indices, bounded i64 civil arithmetic. Malformed direct structs may panic.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### display: invariant and cost

All cells generated before selection, so elision reduces output not source-format work.

Names and shown widths determine pad; trailing spaces stripped, four-space indent and three-space separators.

Scope-specific cost: O(all cell rendering bytes) memory/time before selecting displayed rows; output only head10+tail5 above30 rows.

## models

## behavior_complement

Elided output excludes reconstruction of hidden rows; nonfinite Date displays NaT and string None displays <missing>. No ANSI width guarantee.

## reverse_reconstruction

Display string admits many matrices differing beyond precision, elided middle rows or real literal <missing> text.

## display_cost

All cells generated before selection, so elision reduces output not source-format work.

## layout_transport

Names and shown widths determine pad; trailing spaces stripped, four-space indent and three-space separators.

## branch_relation

cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

## resource_cost

O(all cell rendering bytes) memory/time before selecting displayed rows; output only head10+tail5 above30 rows.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Elided output excludes reconstruction of hidden rows; nonfinite Date displays NaT and string None displays <missing>. No ANSI width guarantee. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [15045,18136); includes no claim beyond this relation. Source hash 7af3a65aabe33226d39f741909fd08d74c3b3e43390417c64e617a10704ac26b.
