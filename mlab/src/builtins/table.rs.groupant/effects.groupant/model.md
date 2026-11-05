# mlab-builtins-table-effects

## descriptant

### mlab-builtins-table-effects

read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

## approximant

### effects: forward relation

read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

Domain: Well-shaped values and finite practical windows; effects delegated to it.fs. Physical filesystem not exercised.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### effects: invariant and cost

writetable serializes before write, so serialization/options failure excludes write; backend partial writes remain backend contract.

datenum with one matrix >=3 columns uses Y/M/D only; scalar components add hour/minute/second fraction.

Scope-specific cost: O(bytes read/written + parser/serializer work) and backend I/O cost; dates/vector string conversion O(number of cells + rendered bytes).

## models

## behavior_complement

Spreadsheet readtable excludes attempted read; readmatrix empty delimiter may follow successful read. No host FS isolation guaranteed by this module itself.

## reverse_reconstruction

Read output admits every file representation decoded to same table; write result empty does not reconstruct previous filesystem contents.

## effect_order

writetable serializes before write, so serialization/options failure excludes write; backend partial writes remain backend contract.

## date_adapter

datenum with one matrix >=3 columns uses Y/M/D only; scalar components add hour/minute/second fraction.

## branch_relation

read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

## resource_cost

O(bytes read/written + parser/serializer work) and backend I/O cost; dates/vector string conversion O(number of cells + rendered bytes).

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Spreadsheet readtable excludes attempted read; readmatrix empty delimiter may follow successful read. No host FS isolation guaranteed by this module itself. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [26346,33011); includes no claim beyond this relation. Source hash f1b0959092cfcdd3cbd171a48d252ce4c1e652b1e56ed030c160e7667d8cbb54.
