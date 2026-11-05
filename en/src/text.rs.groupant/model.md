# en-text

## descriptant

### en-text
Ordered detokenizer, chrF, LCS, adjacency count and Unicode title

## approximant

### Forward relation
One-based detok original adjacency uses space_after before punctuation heuristics. chrF strips Unicode whitespace, averages common available orders1..6, returns 5pr/(4p+r), zero with no orders. LCS two-row recurrence: cur[0]=0, cur[j+1] from prev[j], prev[j+1], cur[j]; rows swap after each left item; either side empty returns 0 before allocation.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
One-based detok original adjacency uses space_after before punctuation heuristics. chrF strips Unicode whitespace, averages common available orders1..6, returns 5pr/(4p+r), zero with no orders. LCS two-row recurrence: cur[0]=0, cur[j+1] from prev[j], prev[j+1], cur[j]; rows swap after each left item; either side empty returns 0 before allocation.
Empty/empty chrF zero; invalid detok indices panic; adjacency usize::MAX+1 may overflow; no permutation validation.
Detok O(output bytes); chrF expected O(6(H+R)) with i32 counts; LCS O(A*B) comparisons in row-major order, exactly two B+1-word rows for nonempty inputs (zero allocations when a side is empty); previously A fresh rows.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: One-based detok original adjacency uses space_after before punctuation heuristics. chrF strips Unicode whitespace, averages common available orders1..6, returns 5pr/(4p+r), zero with no orders. LCS two-row recurrence: cur[0]=0, cur[j+1] from prev[j], prev[j+1], cur[j]; rows swap after each left item; either side empty returns 0 before allocation.
Reverse relation: Metrics have many antecedent sequences; chrF ignores whitespace and LCS discards witnesses; title may expand first character.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
Empty/empty chrF zero; invalid detok indices panic; adjacency usize::MAX+1 may overflow; no permutation validation.

## reverse_reconstruction
Metrics have many antecedent sequences; chrF ignores whitespace and LCS discards witnesses; title may expand first character.

## coordinate_domain
Order indices are 1-based; original successor pair uses space_after even for punctuation, arbitrary reorder falls back to rules.

## chrf_normalization
Only ngram orders present on both sides contribute; multiplicities clipped by decrementing reference i32 counts; zero denominator path returns0.

## lcs_recurrence
D(i,j)=D(i-1,j-1)+1 for equal symbols else max(D(i-1,j),D(i,j-1)); no subsequence witness stored. Two reused rows swap per left item; cur[0] is reset and every cell is overwritten before it is read. Histogram bound Σ_t min(count_a(t),count_b(t)) excludes larger outputs but is not generally equal to the LCS ([1,2]/[2,1]: bound 2, LCS 1). Elements are tokens or characters as given, never raw UTF-8 bytes on the caller's behalf.

## unicode_case
Title uppercases first scalar with potential expansion and copies remaining suffix; ASCII spacing rules do not generalize to every language.


## tests

### Actual source checks
Named checks: text::wave27_tests::metrics_and_adjacency_priority and text::wave27_tests::lcs_rows_empty_shapes_and_oracle (empty shapes, 400 seeded u8 cases against a brute-force subsequence oracle, characters versus raw UTF-8 bytes). Command: cargo test --offline --locked -p en --lib text::wave27_tests -- --test-threads=1. Two runs: 2 passed, 0 failed, exit 0.
Integration: tools/correspondence/tests/lcs_bridges.rs calls the actual public API against both algorithm LCS implementations, the catalogue rows and a logging asymmetric PartialEq schedule probe; two debug and two release runs green; mutation control red (exit 101) then restored by hash.
Allocation counts (separate counting-allocator build) match the two-row model on eight shapes. Timings are a scoped trade-off with one measured regression (10x1000); see evidence.json and docs/correspondences.md. Unknown support, allocation faults and exhaustive Unicode behavior remain unexecuted gaps. The wave27 mutation controls for hash/lexgen remain historical.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

Epoch 2026-10-31 (wave37): `lcs` reuses two rolling rows instead of allocating one fresh row per left item; empty shapes return before allocation. Comparison order and output are unchanged. Typed forward rows `lcs.ordinary`/`lcs.algorithm`/`lcs.bitparallel` in `models/correspondences.json` bind this source to the algorithm LCS groupants; the histogram upper bound stays a partial excluded relation in tests and `docs/correspondences.md`, not a catalogue complement. External bench evidence: octopus-wave37-fable-2026-10-31.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
