# mova-nautilus-layer

## model

# mova-nautilus-layer

## Descriptant

A dictionary layer from annotated units, natively: content lemmas (NOUN/VERB/ADJ/ADV) counted, top N by count, documents, lemma; part of speech and meaning from the pupils' dictionary; a counted sense (uses, documents, UPOS share, classes, co-occurring words); up to three examples with source ids; the school-layer TSV with a draft line; stats and parse_rows. Source: `tools/nautilus/src/layer.rs`.

## Forward approximant

units -> count (BTreeMap per lemma: uses, documents, UPOS, classes, co-occurrence, unit order) -> sort (count desc, documents desc, lemma) -> take top -> Row{word, base pos/meaning or UPOS name, counted sense, examples (new document first, 20..220 chars, source order), draft note} -> render (HEAD, draft line, rows).

## Invariant

Same units, base and source give the same bytes; rows <= top; every example ends with its unit id; parse_rows(render(rows)) == rows for tab-free rows.

## Cost

O(tokens log lemmas) counting plus O(L log L) sorting.

## Composition

Parent: `nautilus`. Parent: nautilus. Used by animal::make, work::DictJob and mova-turtle::make. State: Pure functions over in-memory units.

## behavior_complement

Does not write senses, disambiguate meanings, lemmatize itself, read proper nouns, function words or tokens with digits, or call any model; the meaning is copied, not checked against the source. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a layer one recovers the selected lemmas, their counts, documents, classes, top co-occurring words and chosen example ids; not the units that were not chosen, nor the annotation of each token. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.
