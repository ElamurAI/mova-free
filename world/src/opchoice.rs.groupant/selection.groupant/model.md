# world-opchoice-selection

Weights, score, choose, margin and SURE before book loading.

## forward

Score sums each feature occurrence with absent weights zero; choose replaces only on strict improvement. margin finds first top-score value and minimum top-minus-score among other values; empty returns 0 and no different value returns 1000.

## invariant

choose None iff candidates empty under valid indices. Nonempty margin is nonnegative if subtraction representable. SURE=8 is historical empirical threshold, not a correctness probability.

## cost

Choose O(C*F*log W), plus generated features; margin retains O(C) score/value pairs and scans linearly.

## composition

Candidate enumeration feeds features and first-max scoring. Learner interns the same feature occurrences, preserves candidate order and updates toward the first highest-scoring gold-valued candidate. Text extraction and lexical environment must be fixed. top uses lexical expression ties unlike choose; parent ranking cannot silently substitute one for the other.

## epoch-memory

I128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.

## checks

See ../model.md#tests and native opchoice::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

Fixed candidate-order universe excludes later equal-score winners in choose. Positive margin does not certify gold correctness; that property is outside the relation.

## reverse-reconstruction

Winner j imposes strict earlier and non-strict later linear score inequalities. Observed margin constrains only differently valued candidates; same-value score variation and many weight vectors remain possible.

## zero-weight-choice

All-zero weights select first candidate. A larger division weight selects 6/2=3; equal addition/division weights restore first addition. Empty slice returns None.

## score-kernel

For multiplicity vector m, score s admits all bounded integer w satisfying dot(m,w)=s. Absent feature coordinates are unconstrained; duplicate features change coefficients.

## History

### 2026-10-30 · epoch-memory

2026-10-30: i128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.
