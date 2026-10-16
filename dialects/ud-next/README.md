# ud-next — changes after UD 2.18 that will go into 2.19

The `ud-next` layer of the `mova` dialect (`docs/dialect.md`). Design note: future 2.19 changes can already be used.

**What was compared.**
- Release 2.18 (15.05.2026): `markdown-source/` from `$MOVA_DATA/raw/multi-ud/ud-documentation-v2.18.tgz` and `$MOVA_DATA/raw/multi-ud/ud-tools-v2.18.tgz`.
- A snapshot of the `docs` and `tools` repositories from 24.09.2026 (shallow clone, no history): `$MOVA_DATA/raw/ud-docs/{docs,tools}`. The `docs` tracker — `$MOVA_DATA/raw/ud-docs/issues/`: 1000 issues and PRs (#296–#1305), bodies only, without comments.
- There are no treebank branches in the snapshot: whether EWT `dev` already follows the new guidelines is not visible from these data.

**How it was compared.**
- `diff -r -I 'Interlanguage links updated'`: without this timestamp, which a script rewrites in every language-guideline file, 4680 files would look changed;
- `tools/data/*.json` was expanded into "path → value" lines and compared with `diff`;
- `udtools` and `validate.py` were compared selectively;
- numbers for EWT 2.18 — the `en expert-check` engine with counter rules from the seeds, some by grep.

**Path notation.** `docs/…` and `tools/…` — the snapshot. `2.18:docs/…` and `2.18:tools/…` — the 2.18 archives. `#N` — an issue or PR in `docs`.

## In brief

- **Changes after 2.18 — 11 groups.**
  - Universal, 6: clarification No. 18 on features and `Exponence`; a new definition of `discourse`; a new validator check; feature layers; `expl:*` pages; validator data.
  - English, 2: case of *who* pronouns; the treebank list.
  - Tools — 1.
  - Other languages and the site — 2 seeds, one of them on the French subtypes as a borrowing candidate.
- **Two change English annotation:**
  - *who/whom/whose* get `Case`, *whom* gets the lemma *who*: 24 lemmas and 460 features in EWT 2.18;
  - adverbs are no longer `discourse`: 9 EWT tokens.
  Two more only legitimize or protect what EWT already does: features from context (No. 18) and non-empty XPOS.
- **Registry.** `en/data/ud-registry-en.tsv` matches the validator data of both 2.18 and the snapshot: nothing changed for en (`seeds/validator-data.md`). This is the state of 2.18 and also of the 24.09 snapshot. It cannot yet be called the 2.19 state: the data freeze is 01.11.2026, the release is 15.11.2026.
- **The converter** `mova` ⇄ `ud-next` is in `convert.md`, run with the real engine `en convert` / `en convert-check`. The forward one changes 469 words on EWT, the reverse one restores everything except 9 `discourse` on adverbs.

## Changes

| seed | level | type | what changed | for English annotation | EWT 2.18 | in the registry |
|---|---|---|---|---|---|---|
| [en-wh-case](seeds/en-wh-case.md) | en | guideline `_en/pos/PRON.md` | *who* `Case=Nom\|Acc` by context, *whom* → lemma *who* + `Acc`, *whose* `Case=Gen` with a noun; 2.18 TODO closed | yes: lemma and FEATS | *whom* with lemma *whom* 23 (+1 *whomever*); `Case` on *who* pronouns 0 of 447; *whose* without `Gen` 13 | yes: `PRON Case=*`, `Poss=Yes` |
| [discourse-scope](seeds/discourse-scope.md) | universal (en — explicitly) | guideline `_u-dep/discourse.md` | `discourse` — by grammatical features; ADV → `advmod`, prepositional phrase → `obl`; emoticons — `discourse` | yes: DEPREL | ADV with `discourse` 9 (GUM 78); emoticons 123 | yes |
| [features-context](seeds/features-context.md) | universal | CLARIFICATION No. 18, 2.19 | feature value — from context, but the form wins; `Exponence` in MISC | no: EWT already does this | VBD/VBP get `Number`/`Person` from the subject; *you*/*it* `Case` by position; `Exponence` 0 | not applicable |
| [xpos-not-empty](seeds/xpos-not-empty.md) | universal | validator `udtools` 0.2.8 | error `empty-string-in-xpos` | export gate | empty XPOS 0 | not applicable |
| [feat-layer-ids](seeds/feat-layer-ids.md) | universal | guideline | a layer identifier — lowercase Latin letters only, now a rule | no | no layers | not applicable |
| [expl-pass-pv-docs](seeds/expl-pass-pv-docs.md) | universal | documentation | the `expl:pass`/`expl:pv` pages changed from Czech to universal | no | — | not present for en |
| [validator-data](seeds/validator-data.md) | universal | validator data | 48 languages: 18 new, 7 got new entries, 23 changed; en unchanged | no | — | = 2.18 = snapshot |
| [tools-scripts](seeds/tools-scripts.md) | universal | tools | `conllu_convert_uposf_to_xpos.pl`, `survey_scripts.pl`, pages on script (#1032) | no | — | — |
| [en-treebank-list](seeds/en-treebank-list.md) | en | documentation | 13 active treebanks (#1297) | no | — | — |
| [fr-spoken-subtypes](seeds/fr-spoken-subtypes.md) | fr | guideline | `discourse:filler`, `discourse:tag`, `conj:reform` | borrowing candidate | — | not present for en |
| [other-languages](seeds/other-languages.md) | other languages, site | various | Armenian (`flat:redup`), oge, ka, lt, el, es, egy, ru, ps, ta…; new languages; site | no | — | — |

The rules in the seeds `en-wh-case`, `discourse-scope` and `features-context` check annotation for conformity with `ud-next`. On EWT 2.18 the error rules give 24 + 423 + 13 + 9 violations, and this is the negative control. On EWT after the `mova → ud-next` converter and on a manual example with correct annotation — 0:

```
target/release/en expert-check dialects/ud-next/seeds \
  $MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-EWT/en_ewt-ud-{train,dev,test}.conllu
```

Proposals for `mova` on each change are in `dialects/mova/`.

## Open discussions where decisions are expected before 2.19

**About the milestone.** 129 issues have the `v2.19` milestone: 8 closed, 121 open. For 73 of the open ones the last update is 22.05.2026, and they were created before 2.18. This is a mass carry-over from `v2.18`, so the milestone by itself promises nothing. Activity after 22.05 (or creation after the 2.18 release) — in 47. Below are those of them that touch English or universal rules. The dump contains no decisions, because it has no comments.

| # | topic | activity | what it touches in English (EWT 2.18) | for `mova` |
|---|---|---|---|---|
| #1118 | `Mood` for modal AUX in English (`Pot`/`Nec`? *will* — `Irr` or `Tense=Fut`?) | 45 comments, 27.05 | MD with `VerbForm=Fin` without `Mood` — 4049; warning `verbform-fin-without-mood` (`dialects/ud-2.18/seeds/modal-mood.md`) | wait |
| #1155 | restrict this warning to VERB | 11, 22.05 | the same 4049 MD (AUX) | wait |
| #1233 | when to put "lexical" features (examples from EWT and GUM) | 20, 07.07 | modals with `VerbForm=Fin`; *a/each/every/another/both* without `Number` (5348 *a*); `Exponence` — partly answered by No. 18 | wait |
| #1252 | `ExtPos` in FEATS or in MISC; PROPN boundaries | 32, 11.06 | EWT writes `ExtPos` (7 values in the registry); moving it to MISC would change the registry and `Feat` in `en` | wait |
| #1176 | dependents of a nominalized adjective — `nmod`, not `obl` («decided in a Core Group conversation») | 0, 22.05 | ADJ in nominal position without `cop`: with `nmod` 83, with `obl` 53 (*several of these children*) | wait for the text; the candidate rule `obl → nmod` is simple |
| #1274 | *on what*: `case` or `mark` | 7, 10.07 | by a rough counter, a preposition as `mark` before a wh-clause: EWT 69, GUM 17; as `case` on a wh-word: EWT 92, GUM 246 | wait |
| #1248 | *both X and Y*, *tanto … como* | 9, 25.05 | EWT: *both* — CCONJ `cc:preconj` 58, the rest DET or ADV; *and* — CCONJ | wait |
| #1254 | participles in adjective position: ADJ or VERB | 5, 10.06 | in EWT the boundary is drawn by the PTB tag: VBN and VBG are always VERB or AUX (VBN with `amod` — 585), lexicalized *-ed* forms have JJ and ADJ (739) | wait |
| #776 | `Lang=` for code-switching | 26, 24.05 | EWT: `Foreign=Yes` — 61 tokens; `flat:foreign` — 0 | wait |
| #1273, #1280, #1289, #1290, #1300, #1282 | UniDive, spoken language: segmentation, co-constructions of several speakers, tokenization and morphology, parataxis, tag questions, backchannels, metadata | 07–09.2026 | ESLSpok, the spoken part of GUM, CHILDES; see `seeds/fr-spoken-subtypes.md` | wait; the guideline fork is not merged yet |
| #1291 | one language — different scripts | 1, 13.08 | not applicable | — |

**English issues in milestone v2.19 without activity after 22.05** (16 with the `lang:English` label, activity only in #1118 and #1274):
- #308 *tough*;
- #455 dates, times, addresses;
- #647 `aux:pass` for *get*;
- #658 coordination of auxiliaries;
- #864 reported speech;
- #876 EUD;
- #923 *be easy to VERB*;
- #1009 independent possessives;
- #1040 names and numbers;
- #1070 *Espresso*;
- #1086 *everyone at X*;
- #1091 *all too*;
- #1099 internet slang;
- #1176 (above).

Related universal ones:
- #1192 relative *that*: PRON or SCONJ, 14 comments;
- #1208 days and months: PROPN or NOUN, 68 comments;
- #1024 clausal `appos`, 36;
- #1066 subjects with non-clausal relations, 35;
- #1064 `nmod:poss` and `det:poss`: a proposal to forbid `det:poss` in the validator. Matches our normalization `det:poss` → `nmod:poss`.

**Closed in milestone v2.19** (8):
- #1032 — treebank script, see `tools-scripts.md`;
- #1250 — Thai verbal clauses without conjunctions;
- #1270, #1279, #1286, #1287, #1299, #1301 — the site and Middle Persian.

Outside the milestone, PR #1297 (the list of English treebanks, `en-treebank-list.md`) and #1302 (EUD tools) were closed. Neither of them changes English annotation.
