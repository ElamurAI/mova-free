# ud-next — changes after UD 2.18 that will go into 2.19

The `ud-next` layer of the `mova` dialect ([docs/dialect.md](../../docs/dialect.md)). With it, future 2.19 changes can already be used.

**Sources.**
- Release 2.18 (15.05.2026): `markdown-source/` from `$MOVA_DATA/raw/multi-ud/ud-documentation-v2.18.tgz` and `$MOVA_DATA/raw/multi-ud/ud-tools-v2.18.tgz`.
- A snapshot of the `docs` and `tools` repositories from 24.09.2026 (shallow clone, no history): `$MOVA_DATA/raw/ud-docs/{docs,tools}`. The `docs` tracker — `$MOVA_DATA/raw/ud-docs/issues/`: issues and PRs, bodies only, without comments.
- There are no treebank branches in the snapshot: whether EWT `dev` already follows the new guidelines is not visible from these data.

**How the layer is compared.**
- `diff -r -I 'Interlanguage links updated'`: the ignored line is a timestamp that a script rewrites in every language-guideline file;
- `tools/data/*.json` is expanded into "path → value" lines and compared with `diff`;
- `udtools` and `validate.py` are compared selectively;
- EWT 2.18 is counted with the `en expert-check` engine and counter rules from the seeds, some by grep.

**Path notation.** `docs/…` and `tools/…` — the snapshot. `2.18:docs/…` and `2.18:tools/…` — the 2.18 archives. `#N` — an issue or PR in `docs`.

## Summary

- **Changes after 2.18, in groups.**
  - Universal: clarification No. 18 on features and `Exponence`; a new definition of `discourse`; a new validator check; feature layers; `expl:*` pages; validator data.
  - English: case of *who* pronouns; the treebank list.
  - Tools.
  - Other languages and the site, one of them on the French subtypes as a borrowing candidate.
- **Two change English annotation:**
  - *who/whom/whose* get `Case`, *whom* gets the lemma *who*;
  - adverbs are no longer `discourse`.
  Two more only legitimize or protect what EWT already does: features from context (No. 18) and non-empty XPOS.
- **Registry.** `en/data/ud-registry-en.tsv` matches the validator data of both 2.18 and the snapshot: nothing changed for en (`seeds/validator-data.md`). It cannot be called the 2.19 state before the 2.19 data freeze and release.
- **The converter** `mova` ⇄ `ud-next` is in `convert.md`, run with the real engine `en convert` / `en convert-check`. The reverse direction restores everything except `discourse` on adverbs.

## Changes

| seed | level | type | what changed | for English annotation | in the registry |
|---|---|---|---|---|---|
| [en-wh-case](seeds.md#en-wh-case) | en | guideline `_en/pos/PRON.md` | *who* `Case=Nom\|Acc` by context, *whom* → lemma *who* + `Acc`, *whose* `Case=Gen` with a noun; 2.18 TODO closed | yes: lemma and FEATS | yes: `PRON Case=*`, `Poss=Yes` |
| [discourse-scope](seeds.md#discourse-scope) | universal (en — explicitly) | guideline `_u-dep/discourse.md` | `discourse` — by grammatical features; ADV → `advmod`, prepositional phrase → `obl`; emoticons — `discourse` | yes: DEPREL | yes |
| [features-context](seeds.md#features-context) | universal | CLARIFICATION No. 18, 2.19 | feature value — from context, but the form wins; `Exponence` in MISC | no: EWT already does this (VBD/VBP get `Number`/`Person` from the subject; *you*/*it* `Case` by position) | not applicable |
| [xpos-not-empty](seeds.md#xpos-not-empty) | universal | validator `udtools` 0.2.8 | error `empty-string-in-xpos` | export gate | not applicable |
| [feat-layer-ids](seeds.md#feat-layer-ids) | universal | guideline | a layer identifier — lowercase Latin letters only, now a rule | no | not applicable |
| [expl-pass-pv-docs](seeds.md#expl-pass-pv-docs) | universal | documentation | the `expl:pass`/`expl:pv` pages changed from Czech to universal | no | not present for en |
| [validator-data](seeds.md#validator-data) | universal | validator data | new languages and changed entries; en unchanged | no | = 2.18 = snapshot |
| [tools-scripts](seeds.md#tools-scripts) | universal | tools | `conllu_convert_uposf_to_xpos.pl`, `survey_scripts.pl`, pages on script (#1032) | no | — |
| [en-treebank-list](seeds.md#en-treebank-list) | en | documentation | active English treebanks (#1297) | no | — |
| [fr-spoken-subtypes](seeds.md#fr-spoken-subtypes) | fr | guideline | `discourse:filler`, `discourse:tag`, `conj:reform` | borrowing candidate | not present for en |
| [other-languages](seeds.md#other-languages) | other languages, site | various | Armenian (`flat:redup`), oge, ka, lt, el, es, egy, ru, ps, ta…; new languages; site | no | — |

The rules in the seeds `en-wh-case`, `discourse-scope` and `features-context` check annotation for conformity with `ud-next`. EWT 2.18 as released is their negative control (the error rules must fire on it); EWT after the `mova → ud-next` converter and a manual example with correct annotation must give no violations:

```
target/release/en expert-check dialects/ud-next/seeds \
  $MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-EWT/en_ewt-ud-{train,dev,test}.conllu
```

Proposals for `mova` on each change are in `dialects/mova/`.

## Open discussions where decisions are expected before 2.19

**About the milestone.** Most issues with the `v2.19` milestone were carried over from `v2.18` in bulk, so the milestone by itself promises nothing. Below are those with activity after the 2.18 release that touch English or universal rules. The tracker dump contains no decisions, because it has no comments.

| # | topic | what it touches in English (EWT 2.18) | for `mova` |
|---|---|---|---|
| #1118 | `Mood` for modal AUX in English (`Pot`/`Nec`? *will* — `Irr` or `Tense=Fut`?) | MD with `VerbForm=Fin` without `Mood`; warning `verbform-fin-without-mood` (`dialects/ud-2.18/seeds.md#modal-mood`) | wait |
| #1155 | restrict this warning to VERB | the same MD (AUX) | wait |
| #1233 | when to put "lexical" features (examples from EWT and GUM) | modals with `VerbForm=Fin`; *a/each/every/another/both* without `Number`; `Exponence` — partly answered by No. 18 | wait |
| #1252 | `ExtPos` in FEATS or in MISC; PROPN boundaries | EWT writes `ExtPos`; moving it to MISC would change the registry and `Feat` in `en` | wait |
| #1176 | dependents of a nominalized adjective — `nmod`, not `obl` («decided in a Core Group conversation») | ADJ in nominal position without `cop`, with `nmod` or `obl` (*several of these children*) | wait for the text; the candidate rule `obl → nmod` is simple |
| #1274 | *on what*: `case` or `mark` | a preposition as `mark` before a wh-clause, or as `case` on a wh-word | wait |
| #1248 | *both X and Y*, *tanto … como* | EWT: *both* — CCONJ `cc:preconj`, otherwise DET or ADV; *and* — CCONJ | wait |
| #1254 | participles in adjective position: ADJ or VERB | in EWT the boundary is drawn by the PTB tag: VBN and VBG are always VERB or AUX, lexicalized *-ed* forms have JJ and ADJ | wait |
| #776 | `Lang=` for code-switching | EWT: `Foreign=Yes` tokens; no `flat:foreign` | wait |
| #1273, #1280, #1289, #1290, #1300, #1282 | UniDive, spoken language: segmentation, co-constructions of several speakers, tokenization and morphology, parataxis, tag questions, backchannels, metadata | ESLSpok, the spoken part of GUM, CHILDES; see `seeds/fr-spoken-subtypes.md` | wait; the guideline fork is not merged yet |
| #1291 | one language — different scripts | not applicable | — |

**English issues in milestone v2.19 without recent activity** (label `lang:English`):
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
- #1192 relative *that*: PRON or SCONJ;
- #1208 days and months: PROPN or NOUN;
- #1024 clausal `appos`;
- #1066 subjects with non-clausal relations;
- #1064 `nmod:poss` and `det:poss`: a proposal to forbid `det:poss` in the validator. Matches our normalization `det:poss` → `nmod:poss`.

**Closed in milestone v2.19:**
- #1032 — treebank script, see `tools-scripts.md`;
- #1250 — Thai verbal clauses without conjunctions;
- #1270, #1279, #1286, #1287, #1299, #1301 — the site and Middle Persian.

Outside the milestone, PR #1297 (the list of English treebanks, `en-treebank-list.md`) and #1302 (EUD tools) were closed. Neither of them changes English annotation.
