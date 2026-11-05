# qagen-main-tests

## forward
Pure helper fixtures test missing-value detection, last-wins duplicate compatibility and allowed/rejected flags across command variants.

## invariant
No CLI dispatch, filesystem reads, model loading, teacher or database calls occur in these tests.

## behavior-complement
These tests do not establish all positional arities or typed numeric limits, nor behavior of external CLI effects.

## reverse-reconstruction
Equal helper results constrain tested argument sequences only. Failure of unknown-key tests points to whitelist acceptance but not uniquely a production cause.

## cost
Constant short argument vectors and ordered maps.

## composition
Production opts/validate_options are called directly; mutations alter real conditionals and are required to fail named assertions.

## missing-witness
--jobs --max-calls 1 must error; old parser consumed --max-calls as jobs value.

## compatibility-witness
dir --jobs 2 --jobs 3 retains dir and jobs=3.

## whitelist-witness
Four rejected command/key pairs and four allowed pairs exercise per-command distinctions.

## mutation-evidence
Removing following-option detection or disabling whitelist rejection fails named tests.

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
