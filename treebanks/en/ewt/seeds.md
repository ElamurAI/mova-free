# Seeds: treebanks/en/ewt

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# EWT — what these data are and which layers exist

**Gist.** The English Web Treebank on top of LDC2012T13: 16,622 sentences and 254,820 words from five web genres — blogs, newsgroups, e-mail, reviews, Yahoo! Answers. The LDC PTB trees were automatically converted to Stanford Dependencies, then manually corrected to UD. Dependencies were mostly annotated by one person, part by two, with agreement ≈ 96%. XPOS are the manual LDC tags. UPOS was derived from XPOS and the graph, lemmas and FEATS from CoreNLP; all of this has been selectively corrected over the years. EWT is the base of `mova` (`docs/dialect.md`): UD 2.18 with EWT conventions.

**Conditions and exceptions.**
- Split: train 12,544 / dev 2001 / test 2077 sentences.
- License CC BY-SA 4.0.
- Enhanced UD (DEPS) — automatic, with the Schuster & Manning 2016 converter, without manual checking. Reduced relatives were added in 2.14.
- The treebank changes its customs every year: `:outer`, `advcl:relcl`, `obl:agent`, `ExtPos`, `:unmarked`, `nmod:desc`, `flat` for place names. This is the chronology along which other treebanks got "stuck" at different versions (`../seeds-overview.md`).

**Examples.** `email-enronsent30_02-0005` *You have now decided to set your sights on a position or situation that could give you greater prestige…* — an e-mail from a mailing list, a typical EWT text.

**In UD.** Layers — all of them:

| layer | state |
|---|---|
| lemmas | «_» only in 177 words |
| XPOS | PTB + LDC web tags: `ADD` 475, `NFP` 499, `AFX` 75, `GW` 345. `XX` is in the tag list but does not occur in 2.18 (`../gum/seeds.md#xpos-web`) |
| FEATS | «_» in 79,476 words: punctuation and some function words |
| DEPS | enhanced; 43 empty nodes |
| MISC | `SpaceAfter` 30,961; STREUSLE in reviews (`Supersense`, `MWE*`, `PRel`); `Cxn`/`CxnElt` (UCxn, relative types); EWT's own attributes: `TemporalNPAdjunct` 1077, `FlatType` 425, `Promoted` 328, `Superlocation` 170; `CorrectForm` 1429 with `Typo=Yes` 1431 |
| MWT | 3327 |

Our annotator on the EWT test: UPOS 94.74, XPOS 94.40, UFeats 95.66, Lemmas 96.78, LAS 80.24, MLAS 70.75 (`en/README.md`).

**Sources.** README `UD_English-EWT` (Introduction, Structure, MISC Annotations, Known Issues, Changelog); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_ewt/index.md`; Silveira et al. 2014, `L14-1067`; Zeldes & Schneider 2023, `2023.udw-1.7`; digest the UD survey digest «english-banks», §1.4.

<a id="errors"></a>

# EWT — known annotation errors

**Gist.** EWT is the base of `mova`, so the converter does not fix its errors: otherwise the round trip EWT → `mova` → EWT would stop being identity. Errors are caught by seed rules (`en/seeds`, error level) and tagger suspicions. Fixes are a separate layer of edits, not a dialect.

**Conditions and exceptions.** Numbers — `en expert-check en/seeds` on all parts of EWT 2.18: error-level violations out of matches.

**Examples and numbers.**
- **`compound` to the right of the head** (`en.nominal.compound-head-final`) — 23 of 8690: `email-enronsent21_02-0030` *EY4096.1 PERFORMANCE*.
- **Infinitive VB under *Do*, while *Do* is not aux** (`en.verbal.vb-aux-infinitive`) — 7 of 3888: `email-enronsent08_02-0045` *Do … take*.
- **aux on a non-finite head** (`en.verbal.aux-head-nonfinite`) — 6: *could … missed*.
- ***what* as `det:predet`** (`en.nominal.predet-xpos`, `det-is-det`) — 5: `reviews-287360-0002` *What …*.
- **Imperative with inconsistent features** (`en.verbal.imperative-form`, `imperative-aux-do-only`) — 2 + 2: `reviews-326439-0002` *TRY*.
- **Free relatives annotated as interrogatives** — a known problem from the README (Known Issues), not caught by a rule.
- **General counters** (`../seeds-overview.md`) give small residues on EWT:
  - `tb.en.imp-mood` — 38 of 3066;
  - `tb.en.propn-number` — 475 of 16,562 PROPN without Number;
  - `tb.en.obl-fragment` — 12 of 196;
  - `tb.en.num-feats` — 15 of 5051;
  - `tb.en.pron-lemma` — 9 of 5039.
  
  Some of them are edge cases rather than errors.
- **`Typo=Yes`** — 1431, of which 1429 have `CorrectForm` in MISC. These are not annotation errors but marked text errors; about 200 more were added in 2.18.

**In UD.** Of the 213 error-level rules in `en/seeds`, only 16 have violations on EWT train, 1–17 cases per rule. Almost all of them are errors of EWT itself (`docs/english.md`, "Self-check on gold data"). Under "examples are not truth" these are tagger and parser suspicions, not a reason to change `mova`.

**Sources.** README `UD_English-EWT`, Known Issues («Many free relatives are incorrectly analyzed as interrogative»), v2.18 («Mark about 200 tokens as typos»); `docs/english.md`; `en/README.md`, "Examples are not truth".

<a id="mova-diff"></a>

# EWT and `mova`: identical today, except for the MISC trace

**Gist.** `mova` = UD 2.18 with EWT conventions plus three normalizations (`docs/dialect.md`). On the EWT 2.18 data each of them is empty:
1. `det:poss` → `nmod:poss`: EWT has no `det:poss` (0; `nmod:poss` — 4466).
2. `:tmod`/`:npmod` → `:unmarked`: there are no old subtypes (0). `nmod:unmarked` — 1360, `obl:unmarked` — 1244.
3. Number/Person of verbs from the subject: EWT already writes it this way. All 19,126 finite verbs with Tense have Number and Person (`tb.en.fin-agree` 0 / 19,126). MD has only `VerbForm=Fin` (`tb.en.modal-fin` 0 / 4048).

These normalizations are needed not for EWT but for Opus annotation and `annot` silver data, where `det:poss` and `:tmod` occur, and for the other treebanks.

**Conditions and exceptions.**
- **MISC is part of the dialect.** `TemporalNPAdjunct=Yes` (1077) is the only thing distinguishing a former `:tmod` from a former `:npmod`. The `mova` → treebank converter with old subtypes (ESLSpok, CHILDES, LittlePrince) has to guess by lemma without it (`../eslspok/convert.md`). So `mova` must keep the EWT MISC, and the `en` annotator should eventually predict `TemporalNPAdjunct`.
- Likewise `Superlocation=Yes` (170) distinguishes *Dallas, **Texas*** from other `nmod:unmarked`, and `FlatType` (425) distinguishes *Chapter 1*, phone numbers, file names from other `flat`.
- **`ud-next`.** Changes after 2.18 that will go into 2.19 are not yet visible in the EWT data. The guideline snapshot has clarification No. 18 (`docs/changes.md`): a feature underspecified by the form is taken from context. It legitimizes exactly the EWT custom (Number/Person on VBP/VBD from the subject), so `mova` already has it too.

**Examples.**
- *Dallas, Texas* — *Texas*: `nmod:unmarked` + `Superlocation=Yes`.
- *every day* as an adverbial — `obl:unmarked` + `TemporalNPAdjunct=Yes`.

**In UD.** The counters of all general rules in `../seeds-overview.md` on EWT are zeros or small residues (`errors.md`). This is also a check of the rules themselves: a counter rule that gives many "violations" on EWT is wrong for `mova`.

**Sources.** `docs/dialect.md` (the `mova` dialect, own normalizations); EWT README v2.15 (`TemporalNPAdjunct`), v2.17 (`Superlocation`, `FlatType`), section «Idiosyncratic MISC Attributes»; UD `docs/changes.md` No. 18 (2.19).
