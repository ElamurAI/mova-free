# mova-nautilus-english-tests

## model

# mova-nautilus-english-tests

## Descriptant

Nautilus in English with the 0.4 model: six prompts select capabilities, five refusals stay refusals with review candidates, and real answers from a draft of a one-lesson fixture course. Source: `tools/nautilus/tests/english.rs`.

## Forward approximant

deputy.example.json with the model -> review each prompt -> compare plans/stages; answer q1..q5 -> English text and the layer file.

## Invariant

Deterministic for a fixed model.

## Cost

One model load per test.

## Composition

Parent: `nautilus`. Parent: nautilus. State: Temporary folders.

## behavior_complement

Skips without MOVA_OCTOPUS_TEST_MODEL; no network or teacher. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The temporary ledger recovers each request. No unique inverse is claimed.

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

### 2026-11-05 · document

#### Epoch 2026-11-05

"Draft the Wikipedia dictionary." was pinned as a refusal because the 0.4 parse read «Draft» as a noun; 0.5 reads it as the verb and the frame repair covers the noun reading, so it is a positive. New controls: "Draft the dictionary.", "Do not draft the Wikipedia dictionary." and four questions about drafting that must never plan a draft.
