# mlab-table-dates

## descriptant

### mlab-table-dates

Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

## approximant

### dates: forward relation

Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

Domain: Well-shaped public Table/Mat/StrArr, in-range zero-based internal indices, bounded i64 civil arithmetic. Malformed direct structs may panic.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### dates: invariant and cost

days_from_civil/civil_from_days form inverse on valid supported calendar dates; ISO parser additionally requires4-digit year layout.

Time values are floating parsed, not range checked; timezone suffix unsupported; date range checked before calendar roundtrip.

Scope-specific cost: O(1) calendar arithmetic plus O(input bytes + format chars) parsing/formatting; no overflow certificate for extreme i64.

## models

## behavior_complement

Invalid February30 excludes Some date; hour24 is not excluded. Formatting is second-resolution and cannot preserve arbitrary fractional day bits.

## reverse_reconstruction

For datestr with a time-bearing pattern, an output admits datenum inputs rounding to the shown second, including midnight carry. iso_date first tests exact integrality: date-only output admits integral inputs only; nonintegral inputs may round to midnight but retain the datetime pattern. Valid civil roundtrip is unique only within the nonoverflow calendar domain.

## calendar_transport

days_from_civil/civil_from_days form inverse on valid supported calendar dates; ISO parser additionally requires4-digit year layout.

## clock_boundary

Time values are floating parsed, not range checked; timezone suffix unsupported; date range checked before calendar roundtrip.

## branch_relation

Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

## resource_cost

O(1) calendar arithmetic plus O(input bytes + format chars) parsing/formatting; no overflow certificate for extreme i64.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Invalid February30 excludes Some date; hour24 is not excluded. Formatting is second-resolution and cannot preserve arbitrary fractional day bits. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [325,4747); includes no claim beyond this relation. Source hash 7af3a65aabe33226d39f741909fd08d74c3b3e43390417c64e617a10704ac26b.
