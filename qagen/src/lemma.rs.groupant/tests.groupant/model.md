# qagen-lemma-tests

## forward
Stub morphology checks threshold and inference cases; 64 answer-content masks times 16 sentence masks compare cover to nested-loop oracle; traced callbacks verify preflight and order.

## invariant
1024 finite oracle cases cover repeated forms, empty pool/content and membership combinations. Real-model test is explicitly skipped in every executed library suite.

## behavior-complement
Finite masks do not certify model accuracy, lexical semantics, hash implementation or enormous f64 lengths.

## reverse-reconstruction
Same finite results permit alternative implementations agreeing on the tested masks; failing tie witness distinguishes first versus last but not every tie bug.

## cost
1024 tiny cases with six answer and up to four sentence words; constant trace fixtures.

## composition
Independent nested membership oracle checks production cover. Traced Lemmas joins anchor validation to externally observable callback history.

## oracle
Expected coverage comes from nested any comparisons, independent of HashSet production representation.

## tie-witness
[2,1] with equal sentence coverage must select 2; [2,1,2] alone fails to discriminate first from last.

## callback-witness
Invalid [1,0] yields no callback; valid zero-content case records answer,two,one,two exactly.

## mutation-evidence
Compiled denominator/hit and tie/validation mutations fail; restored source passes both profiles twice.

## checks
Synthetic embedded Rust tests, 1024 independent coverage cases, preflight sentinel artifacts, callback trace and compiled branch/arithmetic controls; debug and release twice, model-loading test explicitly skipped. See parent ../model.md#evidence for observed outcomes, not proof.

## epoch-memory

Prospective model was saved before repair. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.

## references
Original project implementation and rule v2: docs/context.md, qagen/src/gate.rs and qagen/src/run.rs. Model conventions: docs/groupants.md, docs/mathematical-modeling.md and docs/mathematical-modeling.md. No external solution code copied.

## limitations
Finite local tests and partial syntax projections, no semantic AST or universal proof. Allocation failure and callback panic are outside successful-return model; filesystem races and write failures can interrupt publication. No live model/teacher/database/corpus used. No useful certified numeric surrogate is attached to these categorical/effectful interfaces; no invented error budget, substitution or speed claim.

## History

### 2026-10-30 · epoch-memory

Prospective model was saved before repair on 2026-10-30. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.
