# mlab-builtins-table-joins

## descriptant

### mlab-builtins-table-joins

right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

## approximant

### joins: forward relation

right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

Domain: Well-shaped values and finite practical windows; effects delegated to it.fs. Physical filesystem not exercised.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### joins: invariant and cost

innerjoin matches count sums left multiplicity*right multiplicity for each key; strict join enforces right<=1 globally.

Duplicate nonmissing right keys fail before first unmatched left; missing right rows never enter map.

Scope-specific cost: Expected O(nr*k + nl*k + m) hash match construction; innerjoin adds O(m log m*k) stable ordering and O(m*w) result materialization. Duplicates may make m=nl*nr.

## models

## behavior_complement

No NaN/None key matches. join excludes partial unmatched output and does not guarantee valid unique final names after suffix collisions.

## reverse_reconstruction

Joined values admit multiple original unmatched rows and duplicate multiplicities; explicit source indices absent, so inverse is a relation. strict join admits right rows unused by left.

## multiplicity

innerjoin matches count sums left multiplicity*right multiplicity for each key; strict join enforces right<=1 globally.

## failure_precedence

Duplicate nonmissing right keys fail before first unmatched left; missing right rows never enter map.

## branch_relation

right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

## resource_cost

Expected O(nr*k + nl*k + m) hash match construction; innerjoin adds O(m log m*k) stable ordering and O(m*w) result materialization. Duplicates may make m=nl*nr.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. No NaN/None key matches. join excludes partial unmatched output and does not guarantee valid unique final names after suffix collisions. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [18733,22422); includes no claim beyond this relation. Source hash f1b0959092cfcdd3cbd171a48d252ce4c1e652b1e56ed030c160e7667d8cbb54.
