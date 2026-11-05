# mova-whale-world-export-tests

## Descriptant

Tests of the texts export on a fixture: a batch log with an early batch, a passed and a stopped paper, one learner state with a passing and a later failing round and a terminal knowledge item, and a dossier holding the original, a CC BY-SA page, an earlier description, a consultant draft and a Mova draft equal to the description. Source: `tools/whale-world/src/export/tests.rs`.

## Forward approximant

fixture -> passed (from_batch 13, control 12) ; export -> files -> asserts on citations, body, shadows -> export again -> no writes, same bytes -> fresh repository -> same bytes ; edit article and seo.json, change upstream -> export -> kept and reported twice -> undo edit -> upstream written ; foreign file -> kept ; every file free of held sentences ; ref_mentions and seal/intact with controls.

## Invariant

No model and no network; each test uses its own temporary directory.

## Cost

About 20 ms.

## Composition

Parent: mova-whale-world-export; cfg(test) only; uses the dossier test helpers (`cfg`, `tmp`).

## behavior_complement

Does not exercise the library (no refs.json, no library works), topic-corpus provenance, pod items or paragraph mode; not a proof. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Not applicable beyond the fixtures, which are rebuilt in each run.
