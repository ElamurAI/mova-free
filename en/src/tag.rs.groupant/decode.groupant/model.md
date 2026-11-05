# en-tag-decode

## descriptant

### en-tag-decode
Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.

## approximant

### Scoped forward relation
Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
No executable percentage approximant attached.

## invariant

### Scoped assumptions and costs
Empty tagging empty; empty training has NaNs/no support and nonempty inference may panic. Deserialize and Soft multiplier have no validation.
Train O(T+NT^3+suffix work); decode O(n*K^3) time/O(n*K^2), NT fixed; unchecked counters.

## composition

### Parent composition
Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent combines this fragment with the other named source fragments and helpers outside their ranges. No coverage claim for unscoped code.

## models

## behavior_complement
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: Empty tagging empty; empty training has NaNs/no support and nonempty inference may panic. Deserialize and Soft multiplier have no validation.

## reverse_reconstruction
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: A sequence has all words/parameters for which its score maximizes under strict-greater first-candidate ties; aggregated training counts lose original sentence ordering.

## interpolation_boundary
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: Deleted interpolation chooses q3 on ties then q2; no counts makes lambda division0/0 and NaN transition cells.

## suffix_backoff
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: Suffix smoothing (q+theta*p)/(1+theta) stops at first missing suffix, cap flag distinguishes uppercase first scalar.

## emission_precedence
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: Known exact/lowercase lexical entries return before dictionary; hints only modify unknown probabilities; hard cannot add unseen classes.

## viterbi_ties
Within this fragment's byte scope: Emission candidate Cartesian products define pair-state dynamic programming. Predecessor maximum uses strict greater-than, terminal BOS completes score, stored predecessor indices reconstruct tags. Empty words return before tables; empty candidate lists are unsupported.
Parent relation constraining compatible states: Viterbi sums transition+emission and final BOS; strict > retains first finite maximizing predecessor; candidate emptiness is not validated.


## tests

### Bounded parent evidence
See ../model.md#tests. Actual check: tag::wave27_tests::tiny_known_emissions_and_empty_boundary; shared source evidence, not independent exhaustive fragment testing.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-scoped partial model; parent tests provide finite shared evidence.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): source rebound to e2175f4ca53b; byte range 9930..12561 → 10432..13063. The scope text is unchanged (moved only).
