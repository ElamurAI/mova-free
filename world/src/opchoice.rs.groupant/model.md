# world-opchoice

## model

# world-opchoice

Integer candidate grammar, environment-dependent features, first-max selection, averaged learner and weight publication.

## forward

Enumerate distinct ordered mention pairs, canonicalize Add/Mul ordering, and retain representable results. At 3..5 mentions append valid second operations using distinct third indices. Score feature occurrences and choose first strict maximum. Learner retains gold-reachable examples and averages gold-minus-predicted updates.

## invariant

Valid distinct candidate indices; no second steps outside n=3..5. Op rejects unrepresentable arithmetic, negative-result subtraction and nonintegral division. Score, margin and learner require no i64 overflow. Fixed external flags/dictionaries are required for text determinism.

## cost

One-step count <=3*n*(n-1), multiplied by <=1+4*(n-2) only at n=3..5. C candidates, F features, W weights: score/choose O(C*F*log W), excluding external lookup cost. Training stores O(sum C*F), scans O(epochs*sum C*F); top sorts O(C log C). No throughput claim.

## composition

Candidate enumeration feeds features and first-max scoring. Learner interns the same feature occurrences, preserves candidate order and updates toward the first highest-scoring gold-valued candidate. Text extraction and lexical environment must be fixed. top uses lexical expression ties unlike choose; parent ranking cannot silently substitute one for the other.

## epoch-memory

I128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.

## checks

See model.md#tests and native opchoice::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

Universe: mention arrays, fixed feature context, bounded weights and candidate/choice results. Invalid/reused indices, invalid division, unrepresentable arithmetic and later tied choice winners are absent. This inner absence relation does not exclude scoring overflow outside bounds.

## reverse-reconstruction

For fixed ordered candidates and features, winner j admits weight vectors with score(j)>score(i) for earlier i and >= for later i. Answer-only observation unions these regions over candidates of that value. Different expressions and training histories remain compatible.

## integer-domain

Checked arithmetic returns None on overflow; subtraction also requires x>=y; division requires representable remainder zero and quotient. i128 oracle covers 29 values per operand and four operators: 3364 comparisons.

## candidate-cardinality

Canonical commutative pairs contribute at most n(n-1), directed Sub/Div at most 2n(n-1). Each first step extends by at most 4(n-2) only in n=3..5. A sixth mention disables second steps.

## answer-fibers

For fixed mentions, fiber(v) retains every candidate tuple evaluating to v. Seven n=0..6 fixtures compare full structural tuple sets and 21 value-fiber counts each; no answer-level deduplication is inferred.

## feature-multiplicity

features is a Vec, not set: repeated feature names contribute their weights repeatedly and receive repeated training updates. The score multiset fixture tests 3+3-2=4 with a missing feature contributing zero.

## tie-order

choose uses first maximum in enumeration. margin uses the same winner value; top sorts tied expressions lexically before deduplication. top(1) can differ from answer at equal scores; this behavior is retained.

## lazy-average

At T steps, export trunc(sum of pre-update weights over t=1..T / max(T,1)), dropping zero coordinates. Correct predictions still advance time. Dense synthetic history verifies this convention for epochs 0..6.

## weight-publication

current returns a cached Arc snapshot; set replaces the snapshot for later calls, preserving earlier Arc contents. save delegates atomic write; load tolerates absent/malformed lines. These persistence/concurrency behaviors are source models, not executed evidence.

## feature-context

MOVA_TREEF, MOVA_NOMEANS, MOVA_NOCLASSES, cached MOVA_NOKINDS, dictionaries and mention extraction affect text features. Feature observations cannot recover these hidden inputs. Tests disable lexical enrichments at process startup and use synthetic mentions.

## tests

### Bounded evidence

Run with MOVA_NOMEANS=1 MOVA_NOKINDS=1 MOVA_NOCLASSES=1 and cargo test -p world --lib wave8_models -- --test-threads=1; add --release for release. Synthetic mentions and prepared learner state avoid annotation, corpus, teacher and live state. The two existing quant::tests also pass.

Coverage: 3364 arithmetic comparisons using independent i128 math over -12..12 plus MIN, MIN+1, MAX-1, MAX; seven candidate fixtures n=0..6 and 147 value-fiber counts; empty, zero-weight, division-preferred and tied choices; feature multiplicity; seven dense averaging histories epochs 0..6 with repeated feature IDs.

Pre-fix arithmetic oracle failed with add-overflow panic. Compiled controls changed checked addition to subtraction and strict best-score comparison to >=; arithmetic_domain_oracle and first_tie_and_weighted_selection each failed, then source was restored.

Limits: finite CPU evidence does not prove all grammar/feature behavior, learner overflow safety, lexical correctness, persistent storage, teacher transport, budget adherence or full task accuracy. No speedup or numerical surrogate claim. Partial syntax projections repeat identically; structural checks validate bindings and references, not semantic correctness.

#### Artifact identities

Structural validation: an isolated Git-visible copy of this file and its complete sidecar tree passed mova-modelcheck groupants twice with identical reports and no diagnostics. Each root declares ten typed views and three fragments with four views each (22 views per file). Whole-repository registry and ancestor integration are delegated to the parent task; isolated success does not validate those inter-file bindings. Restored production sources passed all ten native tests after compiled controls.

## History

### 2026-10-30 · epoch-memory

2026-10-30: i128 oracle exposed unchecked arithmetic overflow. Checked operations now omit unrepresentable candidates, including MIN/-1 division; score and training overflow remain bounded-domain assumptions. Dense learner history and first-tie tests preserve existing behavior.

### 2026-10-30 · tests

2026-10-30. Five native tests in opchoice::wave8_models; both files total ten.

Four focused runs (twice debug, twice release) pass all ten; sorted test execution uses --test-threads=1. Normalized named-result lines match byte-for-byte, SHA-256 dba383da848a3393a0766618919041d307b687a2a8b88d65b833bd9f77aa633d. Raw durations are excluded from identity comparison.

Source SHA-256: 770d2eff5a128ca9eb808c6360dd33bb79ef1296f86256ff077a630eccd1862e.

Compiled arithmetic control raw-log SHA-256: 58d1302ffe21437b87471e54809fc602e8a08000890002c02bdb5935065d39c5.

Compiled tie control raw-log SHA-256: 67d26930687965fc80e480ac579bd93ad1414a89e5324bd7181fc1e112e4caa1.
