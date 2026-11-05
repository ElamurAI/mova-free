# mova-seaspider-answer

## model

# mova-seaspider-answer

## Descriptant

Sea Spider's English answers realized deterministically from structured results (robots decisions, queued jobs, status, store counts, page summaries, refusals). Source: `tools/seaspider/src/answer.rs`.

## Forward approximant

robots: decision allowed/deferred/disallowed and source parsed/allow_all/unreachable -> 'Yes, R allows T to fetch U (rule ...)', 'Yes, R is unavailable (HTTP 404) ...', 'No, R disallows U for T (rule ...)', 'No, I could not read R (...) ...', plus basis (just read / read at time). fetch_queued: count, list and position. outcome: already stored / fetched / HTTP>=400 'I could not get U because the server answered HTTP s' / disallowed with rule / deferred / error. status: running job with done of total, waiting count, last finished job in past tense with its non-fetched URLs. stats, page (title, links, redirect, truncation), report (actions' answers or the refusal reason with the review candidate prefix). utc via civil-from-days; thousands separators; title from <title> with basic entities.

## Invariant

Every sentence states only fields of its input; number agreement follows the counts; a failure is never phrased as success.

## Cost

O(size of the result JSON); title scan O(min(body, 256 KiB)).

## Composition

Parent: `seaspider`. Pure functions over JSON results and deputy reports; uses mova_octopus::realize.

## behavior_complement

Not done: paraphrase variety, other languages, summarising page text beyond its title, explanations of why a parse failed beyond the deputy's reason. Counterexample: a fetched page with HTTP 404 is phrased 'I could not get ... HTTP 404', not 'I fetched'. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From an answer one recovers the decision kind, counts, URLs, rule and status it names; the full structured result is not recoverable from the text, which is why the JSON travels alongside. No unique inverse is claimed.

## tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs).

Negative controls: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site; not a proof.

Cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs)

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: New caller_refused: «I did not act on this request because C is not among the callers I answer (…).»

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04).

Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site (2026-11-04); not a proof.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04)
