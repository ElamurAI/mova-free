# mova-whale-world-tests

## Descriptant

End-to-end tests of dossier -> world -> query on three small documents (two GEC papers, one unrelated, a CC0 page): byte-identical builds, licence of world texts, GEC entity and definition, explanations, citations resolving to documents and sentences, related and neighbourhood order, neighbourhood content count, byte cut; incremental epochs (old shards untouched, texts.jsonl prefix kept, world(1) unchanged, plain draft read via the heading fallback) and a changed file refused with nothing written. Source: `tools/whale-world/src/tests.rs`.

## Forward approximant

build twice -> compare every file -> queries -> asserts; add epoch-1 text -> add(1) nothing -> add(2) one -> compare files -> views -> change a file -> add(3) refused, store unchanged.

## Invariant

Model-dependent tests skip without WORLD_EN_MODEL.

## Cost

About 1 s with the model.

## Composition

Parent: whale-world; cfg(test) only.

## behavior_complement

Does not test the library graph edges or real corpora (fixtures have no library); not a proof. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Not applicable beyond the fixtures, which are rebuilt in each run.
