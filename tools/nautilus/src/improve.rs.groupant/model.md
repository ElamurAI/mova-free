# mova-nautilus-improve

## model

# mova-nautilus-improve

## Descriptant

The `layer` source: improve an existing dictionary layer in place through review-only proposals. Reads the layer file (records, judaism-senses, emode-senses or mystic-senses), finds weak rows natively, asks the teacher only for weak rows with evidence, ten per question, and turns the answer into proposal lines in the builder's own format. Source: `tools/nautilus/src/improve.rs`.

## Forward approximant

parse: line -> its exact tab columns (render = join, byte-identical). weakness(format,row,base): records meaning<40 or sense<80 or examples<3; judaism no primary record and no own pos/meaning, or >=2 examples and neither sense nor interpretation; emode an empty modern/meaning/difference; mystic sense<80 or no primary record and no meaning. askable = weak and (an example, or a sense for records/mystic). LayerJob.next(seen) = the first ten askable rows of part `part` (FNV(word) mod parts) whose words are in no seen key `<layer>:w1+w2...`. validate: lines with exactly the format's fields, each `-` or 2..1200 chars without date or control characters; >= half of the asked words well formed, else rejected; proposal = the changed lines, rebuilt in the target columns (examples, kind, forms, tests and note from the row; pos/meaning empty when the base has the word, as the builder writes); none changed: rejected as kept.

## Invariant

The layer file is opened read-only; a proposal line has exactly the builder's columns (records 6, judaism 6, emode 9, mystic 4); a teacher field never replaces a corpus quotation; the same file and seen set give the same item.

## Cost

One read and parse of the layer file per next/open call (20 MB primary layer: about 0.1 s); one question of at most 12000 bytes per ten rows.

## Composition

Parent: `nautilus`. Implements work::Job for the loop and the pool; animal::Config::job builds it; pool.rs counts its open items.

## behavior_complement

Never writes the layer file or the builder's senses files; never asks a thin row (fewer than two corpus examples) or a row without evidence; never writes examples for corpus-quoted formats; never admits. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A proposal line and the row it came from recover which fields the teacher changed (the others are the row's); the item key recovers the asked words. No unique inverse is claimed: an all-`-` answer and no answer both leave the row as it is.

## tests

### Checks

improve::tests: rows_read_back_byte_for_byte (every row of the repository's emode-senses, judaism-senses, mystic-senses, mystic-school and primary-school.v2 renders back to its bytes), weak_rows_and_builder_lines (emode 9 columns, judaism empty pos/meaning with a base record, records appended examples; red: wrong field count, a date, an unchanged line, a word not asked), cc_by_sa_headers.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source (the Nautilus work pool); no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (senses)

Batches of `batch` asked rows (native and junk-free rows ride along); askable needs a non-junk word and min_uses examples that use the word (Mova's lemmas, boilerplate excluded: 2 for judaism/emode); asked_fields = only the weak fields; plan_row = senses::plan with the base's numbered senses, the store's fixed sense and native picks only when every asked field is sense-like; task shows senses, fixed sense, two excerpts and an `answer:` line; validate_planned: word, pick, exactly the asked fields; absent lines rejected; a sense field restating a numbered sense dropped and marked; `# pick` lines appended. Red controls: absent gate removed; restatement gate removed.

### 2026-11-04 · tests

Heading `### Checks (2026-11-04)` stays in tests as `### Checks`.

Command: `cargo test --release` of the crate (tools/nautilus with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin). Finite fixtures; no terminal, teacher or model was called; not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass. 2026-11-04 (senses): senses_picks_restatements_and_absent_lines (asked fields [2,3], one shared lemma not enough, restated sense field dropped and marked, absent line rejected, two shared lemmas = native pick, ag and boilerplate archive not askable); weak_rows_and_builder_lines with the pick column. Red: absent gate, restatement gate, prefix matching.

### 2026-11-05 · school data outside the repository

rows_read_back_byte_for_byte reads the layer files from the school data ($MOVA_SCHOOL_DATA or $MOVA_DATA/school, dictionaries/); absent files are skipped as before.
