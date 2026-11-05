# mova-nautilus-sources

## model

# mova-nautilus-sources

## Descriptant

The sources as annotated units: command lessons (every explain/fact/question/option, id commands:<lesson>:<field>:<n>), the command corpus JSONL prompts, both annotated by en.bin; the Wikipedia corpus's ast.conllu sentences with sent_id and the article class. Source: `tools/nautilus/src/sources.rs`.

## Forward approximant

lesson text -> (id, doc, text) per field and option (stars stripped); corpus JSONL -> (id, doc, text) per line with text; Annotator tokenize+annotate -> Unit words (form, lemma, UPOS); CoNLL-U -> Unit per sentence (multiword ranges and empty nodes skipped); files in name order.

## Invariant

Directory order is sorted; ids are unique per source; nothing is written.

## Cost

O(bytes) reading plus one annotation per commands unit.

## Composition

Parent: `nautilus`. Parent: nautilus. State: Reads files only.

## behavior_complement

Does not read lesson headers as text, deduplicate prompts, repair trees or read articles without ast.conllu. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Unit ids recover the lesson, field, question and option, or the corpus file and line, or the Wikipedia sentence. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool)

courses_raw reads school course folders: only explain, fact and exercise lines (LEARN_FIELDS); a course folder with a held-out name (test, exam, control, heldout, held-out) and a lesson file named held-out/heldout are never opened; lessons whose header declares CC BY-SA are reported. Red controls: test/exam lines read; held-out folder read.

#### Epoch 2026-11-04 (pool)

courses_raw reads school course folders: only explain, fact and exercise lines (LEARN_FIELDS); a course folder with a held-out name (test, exam, control, heldout, held-out) and a lesson file named held-out/heldout are never opened; lessons whose header declares CC BY-SA are reported. Red controls: test/exam lines read; held-out folder read.

#### Epoch 2026-11-04 (tier 1)

wikipedia also reads the large corpus's tier 1 when `<corpus>/tier1/index.tsv` exists: index rows in title order (title, shard, offset, length, …), each record one zstd frame at that offset, its `conllu` and `class` read as before. Doc ids are the title with `_` and `%2F` (the folders' slug). Red control: a short index row fails (`tier1_records_are_read`).

#### Epoch 2026-11-04 (no assessment)

The command course reads only what is taught, like the course source: lesson_units uses LEARN_FIELDS (explain, fact,
exercise and an exercise's options); test and exam lines and their options are never units, and a lesson file named
held-out or heldout is not opened. The five-field list that read assessment is gone. Measured 2026-11-04: the
commands draft (top 1000) held 985 examples from test/exam lines before and 0 after; 145 of 1000 words changed.
Red controls: assessment fields read again; held-out lesson file opened (each red, restored).

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

Pool epoch (2026-11-04): cargo test --release with the 0.4 en.bin, 26 unit + 2 English tests, two identical runs; red controls in the pool, improve and sources tests (each red, restored). Finite fixtures, not a proof.

2026-11-04 (no assessment): cargo test --release --locked (0.4 en.bin), tools/nautilus 38 unit + 2 English. sources::tests: lesson_lines_become_units (taught lines only; zorblax/quuxite of test and exam never units), the_command_course_reads_only_what_is_taught (test and exam lines and a held-out lesson file never read). Red controls: assessment fields read (two tests red), held-out file filter removed (red); each restored.

### 2026-11-05 · school data outside the repository

Doc comment: the command course and school courses are courses/<course> of the school data.
