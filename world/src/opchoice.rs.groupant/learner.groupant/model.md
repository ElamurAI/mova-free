# world-opchoice-learner

Book loading, bagged train, incremental Learner, persistence, ranked explanations and CLI.

## forward

add retains gold-reachable examples and interns first-observed names. run advances each step, chooses first max predicted and first max gold, then updates occurrences iff predicted value differs. weights catches up lazy durations and divides by max(step,1). top lexically orders tied expressions and deduplicates answers.

## invariant

Interned vectors align for internally constructed Learner. run trusts valid prepared ranges. Empty learner exports empty weights. Large score/history arithmetic and invalid caller ranges are outside safe bounded model.

## cost

Prepared memory O(sum C*F); each epoch scans candidate features and updates two occurrence lists on error. Export O(W). Bagging multiplies learner work; all integer sums require bounded intermediates.

## composition

Candidate enumeration feeds features and first-max scoring. Learner interns the same feature occurrences, preserves candidate order and updates toward the first highest-scoring gold-valued candidate. Text extraction and lexical environment must be fixed. top uses lexical expression ties unlike choose; parent ranking cannot silently substitute one for the other.

## epoch-memory

I128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.

## checks

See ../model.md#tests and native opchoice::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

Gold-unreachable items cannot enter prepared via add; correct-value predictions do not update weights. Invalid manually supplied run ranges can panic, so total safety is not claimed.

## reverse-reconstruction

Averaged weights admit many prepared sequences, step counts and updates. Truncation and dropped zero coordinates erase information. Complete prepared features and step trace are needed for replay.

## average-boundary

Independent dense history accumulates pre-update weights for two examples with duplicate IDs over epochs 0..6. Final raw weights alone do not determine average; test checks both.

## ranking-projection

top loses same-value alternatives and orders equal scores lexically, while answer chooses first enumeration maximum. Reverse ranking relation retains discarded expressions and unobserved feature explanations.

## History

### 2026-10-30 · epoch-memory

2026-10-30: i128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.
