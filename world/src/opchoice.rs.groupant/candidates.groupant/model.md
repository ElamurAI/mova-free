# world-opchoice-candidates

Op, Cand, question/features and candidate enumeration.

## forward

Arithmetic is exact representable i64 Add/Sub/Mul/Div with nonnegative subtraction and integral division gates. Traverse x,y,OPS and append each first step then z,OPS second steps. Features combine arithmetic identity with external context.

## invariant

Each retained intermediate is representable, even when hypothetical later cancellation could yield an in-range final result. Mention indices are distinct; commutative order is canonical.

## cost

O(n^2) first-step work plus O(n^3) second-step work restricted to n=3..5; O(C) candidate space. Feature cost depends on context lengths/cache lookups; kind cache can grow by unique word/POS.

## composition

Candidate enumeration feeds features and first-max scoring. Learner interns the same feature occurrences, preserves candidate order and updates toward the first highest-scoring gold-valued candidate. Text extraction and lexical environment must be fixed. top uses lexical expression ties unlike choose; parent ranking cannot silently substitute one for the other.

## epoch-memory

I128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.

## checks

See ../model.md#tests and native opchoice::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

In candidate-output universe, second steps at n=6, reversed commutative duplicates and overflowing intermediates are absent. Full final-expression representability alone is insufficient.

## reverse-reconstruction

For fixed mentions and value v, retain all enumerated canonical expressions yielding v. Add/Mul reverse domain excludes redundant reversed index order; numeric values cannot recover lexical context.

## intermediate-domain

Invalid first operation omits both the first candidate and all extensions. The independent oracle evaluates in i128 then converts each intermediate separately.

## arity-window

Adding an unrelated sixth mention removes all two-step candidates. Parent mention extraction must preserve count; distractors can reduce reachability rather than only add choices.

## History

### 2026-10-30 · epoch-memory

2026-10-30: i128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.
