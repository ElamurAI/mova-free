# mova-nautilus-senses

## model

# mova-nautilus-senses

## Descriptant

Senses: which of the pupils' numbered senses a layer's examples show, natively first. tokens/lemmas (en::dict analyses, comparatives only from UD, archaic verb endings), find (lemma match, never prefixes), junk (< 3 letters, scan abbreviations), boilerplate examples, uses, excerpt (~160 chars around the word, no id), parse (numbered senses), content lemmas, score (shared lemmas, quoted phrases x3), native_pick (one sense and >= 1 shared, or best >= 2 and >= next + 2), restates (Dice >= 0.5 and >= 3 shared), absent, Pick (numbers, new, same, -), Store (senses.jsonl: first numbered/new pick fixes a word), plan/WordPlan (Junk, Thin, Native, Ask; block; native_line), pick_line, records. Source: `tools/nautilus/src/senses.rs`.

## Forward approximant

plan(word, examples, min_uses, senses, fixed, native_ok): junk(word) -> Junk; uses = examples using word by lemma, not boilerplate; |uses| < min_uses -> Thin; !native_ok or no senses -> Ask; native_pick -> Native(pick, sense text, evidence; agrees/differs with fixed) else Ask. Store.fixed = first record per word with a numbered or new pick.

## Invariant

Deterministic: BTreeSet/BTreeMap, ties by sense number; a native pick shares >= 1 lemma with its sense; a restated text shares >= 3 lemmas and Dice >= 0.5 with its sense; the store is append-only.

## Cost

O(tokens x analyses) per example; one store read per plan batch; appends one line per pick.

## Composition

Parent: `nautilus`. Used by work::DictJob, improve::LayerJob, teacher::dictionary_task, replay.

## behavior_complement

Never matches a word by prefix; never asks junk or thin words; never writes a layer or admits; a native pick needs the stated overlap; the store records are review-only (admitted false). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A pick line or store record recovers word, pick, sense text, layer, by and evidence; the examples' shared lemmas are named in the evidence. Unique recovery of the examples is not claimed.

## tests

### Checks

Finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; prospective: on new runs the native share stays near the replay's 10% and restatements near 10% of written senses.

### 2026-11-04 · tests

2026-11-04: words_are_matched_on_lemmas_not_prefixes (red control: a prefix match takes «aged» for «ag»), junk_and_boilerplate, senses_scores_picks_and_restatements, picks_parse_and_excerpts_cut_around_the_word, the_store_fixes_a_word_once. Red: prefix match, junk length, restatement gate (each red, restored).

### Undated · tests

Command: cargo test --release --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin.
