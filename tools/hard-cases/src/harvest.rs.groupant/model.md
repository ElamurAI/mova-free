# mova-hard-cases-harvest

## Descriptant

Validated Fish replies of the lane's waves -> learn/annotations.jsonl and per track/split CoNLL-U and pragmatics TSV, review-only silver. Source: `tools/hard-cases/src/harvest.rs`.

## Forward approximant

for wave in order, pager root, job in sorted order: accept when outer and inner receipts are validated_review_only (review_only, not training-authorized), reply binds task id, each target tree has the target id and its context text -> first annotation per target (minus held-out texts) -> write(annotations, track/split files, manifest).

## Invariant

Only ledger targets with status queued are kept; one annotation per target; files are rewritten whole and atomically; nothing is gold or training-authorized.

## Cost

O(jobs + reply bytes).

## Composition

Reads fish-pages/fish-swarm job layouts (pages/<intent>/run/jobs/<job>/); feeds witness, eel and judge. Parent hard-cases.

## behavior_complement

Does not judge annotation correctness, merge two replies or repair trees; quarantined, uncertain or unbound jobs are skipped with their reason counted. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each annotation keeps its wave, job, receipt hash, source id and key, so the reply and the case can be found again; the other replies for the same target are not kept in learn/.

## Tests

harvest::tests: a bound validated reply counts; red: a quarantined job and a tree whose text is not its target's; learn files split by track and split. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

harvest::tests (2026-11-04): a bound validated reply counts; red: a quarantined job and a tree whose text is not its target's; learn files split by track and split. Finite fixtures and live samples; not a proof.
