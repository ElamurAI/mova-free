# CHILDES — known annotation errors

**Gist.** XPOS in CHILDES was assigned by software without checking. Lemmas and UPOS were corrected only partly. So most errors are in the tags. Dependencies were annotated manually; errors there are isolated. The converter does not fix them.

**Examples and numbers** (all parts together):
- **XPOS `POS` on the verb *'s*** — 79: UPOS AUX, relation `cop` 52, `aux` 20, `root` 7. That is, *'s* = *is* has the possessive tag. The rule `tb.en.poss-s` gives 101 of 798: another 16 *'s* PART with `nmod:poss` and 13 *s* PART in `reparandum` or `obj`.
- **XPOS not in PTB**, besides `?` and `!` — 25 tokens: `ADV` 3, `adv` 1, `DET` 1, `INTJ` 1, `PP` 2, `TP` 1, `VV` 2, `remember` 1, `remembered` 1 and `_` 12 (on X fragments *a, k, stairs…*).
- **PUNCT with XPOS `NNP`/`NN`/`HYPH`** — 5.
- **Lowercase *i* as a lemma** — 53.
- **A garbage comment line** `# chi l d` — once in the file.
- **A noun before a head noun not `compound`** (`tb.en.nn-compound`) — 395 of 3650: `amod` 130, `nmod` 90, `nsubj` 75, `reparandum` 37 and others. Some are child speech with repetitions and broken-off phrases, some are errors.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`) — 49 of 106. Fragments, as in GUM (`../../gum/seeds/obl-fragment.md`).
- **A sole `iobj` without `obj`** — 414 of 963. Not an error: allowed since UD 2.12.

**In UD.** For merging, CHILDES XPOS is unreliable. `en` already skips sentences with tags outside PTB (`Sentence::tagged()`), but *'s*/`POS`-on-AUX pass as valid tags. They should be passed through tagger suspicions rather than trusted.

**Sources.** README `UD_English-CHILDES` («XPOS: automatic»; «Lemmas, UPOS: automatic with corrections»); Yang et al. 2025, `2025.udw-1.6`, sec. 3 (≈ 8000 edits, most often UPOS versus relation).
