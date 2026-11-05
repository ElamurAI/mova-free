# command-school-corpus

## model

# Command school corpus and exam — source-bound model

## Descriptant

`commands.rs` is the start of the command school: an anonymized corpus of commands sent to Crab and an exam that
replays reviewed commands through Mova's understanding in plan mode (nothing executed). As in the primary school, a
command an arbitrary Mova-based animal does not understand signals a gap in the learning algorithm, not data to
memorize. Lessons and a command dictionary follow the primary-school pipeline later.

## Forward

anonymize: whitespace tokens whose punctuation-trimmed core is an absolute or home path → `<path>`, contains `://` →
`<url>`, looks like an e-mail → `<email>`, is a ≥12-char hex/dash run with ≥4 digits or a ≥5-digit number → `<id>`;
everything else (words, role names, small numbers) unchanged. record: append `{text, priority, observed, origin}`
(JSONL, 0600). exam: for each entry with `expected`, compare `reading(review(text))` (plan capabilities in order, or
refusal stage) and count by kind; unreviewed entries are only counted.

## Invariant

anonymize is idempotent; the exam never executes a capability (it calls review only); observed readings never become
expectations automatically — `expected` is written by review.

## Cost

Linear in prompt length for anonymize; one native parse per examined entry.

## Composition

The `crab` binary records every ask (optional corpus path on `serve`) and exposes `crab exam` and `crab audit`.

## behavior_complement

Not anonymized: names of people or organisations written as plain words, short numbers, relative paths; the audit only
checks idempotence of these rules, not semantic privacy. Not done: lesson generation, dictionary, training.

## reverse_reconstruction

From an anonymized text the original paths, ids, e-mails and URLs are not recoverable (many-to-one); the reading
recovers which configured capability or refusal stage the understanding produced, not the execution outcome.

## checks

### Checks

anonymize keeps English and hides path/id/e-mail/URL, idempotent; audit refuses raw paths. Live: 21 reviewed seed commands examined twice with identical output, 21/21 after correcting one refusal-stage expectation.

## History

### 2026-11-03 · checks

Heading `### Checks (2026-11-03)` stays in checks as `### Checks`.

### 2026-11-04 · document

#### Epoch 2026-11-04: Executed failures, review loop and generated exercises

`Reading::ExecutedFailed{capabilities}` marks an understood plan whose execution failed; it is never examined as a refusal and cannot be a reviewed expectation. `review_queue` lists observed texts without a reviewed expectation; `promote` admits reviewer decisions only when observed, privacy_review=passed, anonymized, a reading (plan/refusal) and not yet reviewed — never training. `exercises` derives determiner variants and same-shaped other-entity variants (same handler and parameters except the target; a headed name only within its configured group) from reviewed single-capability plans. `group_candidates` groups legacy and deduplicated review files read-only.

### 2026-11-04 · checks

2026-11-04: unit tests for determiner toggling, promote gate (privacy, observation, execution outcome, duplicates), review queue and exercises with a negative control; live: seed 52/53 (the one failure is a reviewed refusal now understood: "Is the documenter running?"), 123 generated exercises from the seed all pass.

### 2026-11-05 · document

#### Epoch 2026-11-05

Test fixture only: the worker_status capabilities of the exercise fixture are declared `observes` (see the deputy's question rule); generated exercises are unchanged.
