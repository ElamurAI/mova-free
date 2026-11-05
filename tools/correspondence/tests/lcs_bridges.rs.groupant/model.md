# correspondence-lcs-bridges

## descriptant

### LCS correspondence bridges

Six focused tests tie the production `en::text::lcs` to the two algorithm LCS implementations, an exhaustive oracle, one shared typed contract in the catalogue and the histogram upper bound as a partial exclusion. The comparison-schedule probe protects an API obligation that byte equality alone cannot see. See model.md for forward behavior, typed views, controls and epoch memory.

## model

# LCS correspondence probe model

## Forward
Compiled production sources plus the embedded catalogue. Exhaustive byte pairs: alphabet 2 through length 5 (63² pairs) call `en::text::lcs`, `longest_common_subsequence::length`, `bit_parallel_lcs::length` and a brute-force subsequence oracle; alphabet 3 through length 4 (121² pairs) call `en::text::lcs`, `longest_common_subsequence::length` and the oracle only, without bit-parallel. All lengths must agree. Seven fixtures bind rows `lcs.ordinary`, `lcs.algorithm`, `lcs.bitparallel`; `mova_correspondence::compare` must report `finite_coincidence` between the ordinary row and each algorithm row.

## Invariant
All three rows carry one identical contract: listed byte fixtures, symmetric byte equality, length-only observation, shorter side at most 64, successful returns. Implementation schedules and costs live in the source groupants. Finite success never promotes to universal equivalence.

## Cost
63² pairs with three library calls plus the oracle, 121² pairs with two library calls plus the oracle (at most 2⁵ masks each); fixtures are constant; catalogue JSON is reparsed per helper call. No timing is measured here.

## behavior_complement
Within the fixture universe, failure is a length disagreement, a wrong label, an absent output label, a wrong indexed pair, a schedule trace that is not a-major with the left receiver, or a bit-parallel `Ok` for a 65-byte short side. Untested inputs are unsupported, not excluded.

## reverse_reconstruction
Success admits any source satisfying these traces and tables; `(L,|a|,|b|)` has many antecedents ([1,2]/[2,1] and [1,3]/[1,4] both give 1). Hashes narrow identity only.

## histogram_exclusion
`Σ_t min(count_a(t),count_b(t))` is an independently computed upper bound: on the 7×4 rectangle it excludes 10 pairs, all disjoint from the three forward tables. It is a partial exclusion: `[1,2]/[2,1]→2` survives the bound while the actual value is 1, so the bound is not generally equal to the LCS and must not be substituted for the value; the catalogue holds no complement row for it.

## comparison_schedule
An asymmetric logging `PartialEq` records `(x.id, y.id, receiver side)`. `en::text::lcs` must produce exactly |a|·|b| probes in a-major order with the left element as receiver; the swapped call yields only right-side receivers and, under the asymmetric equality, zero. This obligation is specific to the ordinary API and is outside the shared length-only contract.

## unit_boundary
`"éa"` versus `"aé"`: one as characters, two as raw UTF-8 bytes. Bit-parallel accepts a 64-byte short side and rejects 65 with `ShortSideTooLong`; the two other APIs agree at 65.

## Composition
Production calls → length labels → indexed relation assertions → library compare on identical contracts. Correct lengths alone cannot certify label wiring; the negative controls swap distinct actual outputs (indices 0 and 2), truncate the domain, and append one assumption to show `incompatible_contract`.

## Epoch memory and checks

Wave37. First debug run was red: the histogram test asserted 15 excluded pairs while the independent count is 10 (3+3+2+1+0+0+1); the assertion was corrected and the log overwritten by the rerun. A first negative control swapped two equal outputs (indices 2 and 3, both 1) and could not fail; it now swaps indices 0 and 2. Final: six tests, two debug and two release runs, exit 0 each, sorted named outcomes identical. Source mutation control (`prev[j] + 1` → `prev[j]` in `en::text::lcs`): compiled, five of six tests failed, exit 101; exact restoration verified by hash and a green rerun. Logs: external (see History).

## History

### 2026-10-31 · Epoch memory and checks

2026-10-31 wave37. First debug run was red: the histogram test asserted 15 excluded pairs while the independent count is 10 (3+3+2+1+0+0+1); the assertion was corrected and the log overwritten by the rerun. A first negative control swapped two equal outputs (indices 2 and 3, both 1) and could not fail; it now swaps indices 0 and 2. Final: six tests, two debug and two release runs, exit 0 each, sorted named outcomes identical. Source mutation control (`prev[j] + 1` → `prev[j]` in `en::text::lcs`): compiled, five of six tests failed, exit 101; exact restoration verified by hash and a green rerun. Logs: external octopus-wave37-fable-2026-10-31.
