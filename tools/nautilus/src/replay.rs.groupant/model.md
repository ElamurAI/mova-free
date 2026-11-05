# mova-nautilus-replay

## model

# mova-nautilus-replay

## Descriptant

Replay: measure the loop on copies with earlier real answers. earlier(roots) reads tasks/*.md and answers/*.txt, the dictionary name of each question (both templates), answers by (name, word); run copies each configured layer's asked rows to OUT/layers, runs work::Loop with a Replayer terminal (draft: `new: <old sense>`; layer: the asked fields from the old columns, pick new when a sense field is written) and one store OUT/senses.jsonl, and reports questions, bytes, words per question, native picks (and their agreement with the earlier teacher's text), junk, thin, numbered/same/new/restates. Source: `tools/nautilus/src/replay.rs`.

## Forward approximant

roots -> earlier questions and answers -> per layer: subset copy -> Loop steps until idle (min_gap 0, operator word 'replay', no ledger) -> proposals -> counts -> JSON report.

## Invariant

Reads the terminal roots and the configured files only; writes only under OUT; the same inputs give the same report and store bytes (checked twice).

## Cost

One loop run per layer over the copied rows; O(rows x examples) planning.

## Composition

Parent: `nautilus`. Uses animal::Config, work::Loop, teacher::Config, improve::teacher_fields, senses.

## behavior_complement

Never sends anything to a teacher or terminal; never writes the live folders or the layer files; the agreement figure is a lexical proxy, not gold. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The report recovers per-layer counts; OUT/state holds every replayed journal and proposal. The earlier answers are not reconstructed from it.

## tests

### Checks

Finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · Invariant

Reads the terminal roots and the configured files only; writes only under OUT; the same inputs give the same report and store bytes (checked twice on 2026-11-04).

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; prospective: on new runs the native share stays near the replay's 10% and restatements near 10% of written senses.

### 2026-11-04 · tests

2026-11-04: dictionary_names_of_both_templates; two replay runs on a snapshot of six instances' 129 questions gave byte-identical reports and stores.

### Undated · tests

Command: cargo test --release --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin.
