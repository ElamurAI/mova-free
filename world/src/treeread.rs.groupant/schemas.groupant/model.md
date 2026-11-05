# Ordered quantitative schemas

## forward
Scope: same_thing, answer and answer_reading. Thing compatibility is nonempty equality, trailing-s removal or substring inclusion. Select first applicable branch in order comparison, part/whole, total, rate, each, left, change. Total sums plain quantities; each uses two values and positive exact divisor; left starts with first equality and applies gains/losses, ignoring other effects.

## invariant
In bounded safe arithmetic, total is additive and each has no rounding. Negative left balance refuses. Branch priority is observable: rate multiplication wins over each division. Comparison and whole exclude question quantities but other branches may include them.

## cost
Scans are linear in quantity count apart from tag matching with nested string comparisons; reason construction scales with rendered operands. No dynamic-programming or training work is introduced.

## behavior-complement
Each cannot return for zero small operand or nondivisibility when no earlier branch matches. Left cannot answer with only one eligible quantity. Empty reading/default question yields None. These restrictions are conditioned on prior-branch exclusion.

## reverse-reconstruction
Within ordered two-value domain 0..8 with total=true and no rates/comparisons, scalar eight has nine antecedents. Reason strings further restrict operands but not unused Q fields. None includes both missing evidence and blocked arithmetic; do not identify a unique cause from None.

## boundary
Rust structural Q equality excludes all equal copies in some filters; first-match/tie behavior depends on order. Overflow is outside the bounded domain. Substring unit compatibility is deliberately coarse and admits pineapple/apple.

## composition
The schema child consumes actual ordered reading plus original text for part/whole cues. Numerical branch correctness is insufficient to establish correct parsing, units, holders or story meaning.

## checks
81 total oracle cases, nine-member sum fiber, 169 each pairs plus rate-priority fixture, and 343 gain/loss ledgers exercise production answer_reading. Total sum-to-product mutation must fail the sum oracle.

## epoch-memory

Independent finite unit-count/divisibility/balance observations. Refactoring exposes supplied reading only; public answer retains read. No general arithmetic overflow or full task-quality proof, and no approximate replacement.

## History

### 2026-10-30 · epoch-memory

2026-10-30: independent finite unit-count/divisibility/balance observations. Refactoring exposes supplied reading only; public answer retains read. No general arithmetic overflow or full task-quality proof, and no approximate replacement.
