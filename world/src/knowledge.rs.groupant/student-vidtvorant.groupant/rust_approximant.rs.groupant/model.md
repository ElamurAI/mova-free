# school-student-vidtvorant-rust

File model of `rust_approximant.rs`, the executable code of the student's vidtvorant. The vidtvorant itself (subject,
domain, observations, error and performance profile, conformance) is described in [../model.md](../model.md); this
file models the code.

## forward

`words` = `knowledge::words` (alphanumeric split, length > 1, stop and negation words out, `mentions::stem`).
`Memory::read_line` appends a line (lesson, word set, negation flag, numbers) and indexes its words.
`predict(mem, lesson, question, options, params)`: tail cut; question words; gap flag selects the gate; anchors by
shared-word counts (current lesson first, strongest only, ten); targets (anchor/next/linked); per option own lines;
features `[asked, echo, missing, neg-mismatch, route×4, current]`; score `w·f`; structured and ≥ gate to count; best
and best-current per option; lesson-first pool; argmax (score, current) with earlier index on ties; subset/extra
rule outside lesson-first; unique counted number override. `replay` reads a course lesson by lesson and calls back
per traced question. `fit`: averaged integer perceptron over the training fold, updating only on supported
disagreements. `TimeModel::fit`: least squares on ln ns with ridge 1e-6, Gauss–Jordan with partial pivoting.

## invariant

Deterministic: ordered maps and sets only, no randomness, no clock in prediction or fit. `predict` never panics on
the declared inputs (empty question words or no options return the guess). Weights change only by
`φ(student choice) − φ(predicted choice)`. The time model's coefficients are finite when the design matrix has full
rank; a zero pivot leaves the coefficient 0.

## cost

`read_line`: O(w log V). `predict`: O(P_q + Σ_options (P_o + T·w_o)) for posting-list sizes P, targets T ≤ 30 and
option words w_o. `fit`: epochs × replay. `TimeModel::fit`: O(n·7² + 7³). Measured: 40–48 µs median per
prediction on one P-core thread.

## behavior-complement

The code does not parse, tag, look up WordNet, read the kind base or rules, touch the file system, the network,
the student's state or any production path; it does not certify a bound; it does not predict exercises or learning.

## reverse-reconstruction

A returned prediction is compatible with every question/option/memory triple with the same word bags, gap flag,
negation flags, numbers and line-to-lesson map; fitted weights are compatible with a cone of weight vectors.

## tests

Six unit tests, run with `cargo test --release -p mova-approx --example student_vidtvorant` (the runner includes
this file): stemming and stop words; supported option wins and an unread topic guesses first; the support gates
(and the no-gap gate only without a gap); current lesson first; fit moves only on disagreement and only inside the
training fold; time model recovers a planted law to 0.1%.

## epoch-memory

First version; the module is included by path from the runner, so it compiles only with it.

## History

### 2026-11-04 · cost

`read_line`: O(w log V). `predict`: O(P_q + Σ_options (P_o + T·w_o)) for posting-list sizes P, targets T ≤ 30 and
option words w_o. `fit`: epochs × replay. `TimeModel::fit`: O(n·7² + 7³). Measured 2026-11-04: 40–48 µs median per
prediction on one P-core thread.

### 2026-11-04 · epoch-memory

2026-11-04: first version; the module is included by path from the runner, so it compiles only with it.
