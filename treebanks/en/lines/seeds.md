# Seeds: treebanks/en/lines

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# LinES — what these data are and which layers exist

**Gist.** The English half of the parallel English-Swedish treebank LinES: 5696 sentences and 106,307 words. Three parts:
- fiction (the majority);
- online software help;
- Europarl.

Each sentence has a Swedish counterpart with the same index (`# parallel_id`) in UD_Swedish-LinES. The treebank was first annotated in a different, non-UD dependency scheme. It was then automatically converted to UD 1 and partly reviewed (Ahrenberg 2015), and in 2017 to UD 2 with another review. Lemmas exist since 2.1. Development happens outside the UD repository: errors must be fixed in the source or the converter.

**Conditions and exceptions.**
- Split: train 3457 / dev 1118 / test 1121 sentences.
- License CC BY-NC-SA 4.0. The 2.18 release has no README, only LICENSE.txt; the description is on the treebank page.
- The page metadata say «Features: not available», but the data do have FEATS. They are derived from LinES's own XPOS (`xpos-lines.md`), and the customs here are its own (`verb-feats.md`, `nominal-feats.md`).

**Examples.** The fiction part, dialogues: *you* — 823, `vocative` 104, `dislocated` 131 (EWT — 7).

**In UD.**

| layer | state |
|---|---|
| XPOS | LinES's own morphological set, 159 values; «_» in 22,538 words — function-word ADP, ADV, CCONJ, SCONJ, PART (`xpos-lines.md`) |
| FEATS | present; «_» in 35,574 words |
| lemmas | present; «_» — 1 |
| DEPS | absent |
| MISC | almost empty: «_» in 93,567 of 106,307 |
| MWT | 1170 |

Our annotator on the LinES test: UPOS 93.13, UFeats 83.44, LAS 76.05, MLAS 52.12 (`en/README.md`). XPOS is not measured: the tags are not PTB. UFeats is 12 points lower than on EWT — these are the customs from `verb-feats.md` and `nominal-feats.md`.

**Sources.** `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_lines/index.md` (description, table of annotation sources); Ahrenberg 2007, `W07-2441`; Ahrenberg 2015, `W15-2103` (papers/a/ah/ahrenberg-2015-converting-english-swedish-parallel-treebank, abstract only); digest the UD survey digest «english-banks», §1.11.

<a id="errors"></a>

# LinES — known annotation errors

**Gist.** LinES was obtained by automatic conversion from a non-UD scheme with partial review. So alongside the customs there are conversion errors: features that contradict the form itself, and inconsistent lemmas. The converter into `mova` does not fix them. They are candidates for suspicions and edits.

**Examples and numbers.**
- **Possessive *her*, *its* with personal-pronoun features**: in `nmod:poss` *her* has `Case=Acc` without `Poss=Yes` (32), *its* — `Case=Nom` without `Poss=Yes` (22). Correct is `Case=Gen|Poss=Yes`.
- **NOUN without `Number`** — 469, although the XPOS `SG-NOM`/`PL-NOM` sets the number (*garden*, *smile*, *sir*, *jews*).
- **Two norms of possessive lemmas** in one treebank: *his* → *his* 670 and *his* → *he* 136; *my* → *my* 364 and *my* → *I* 45; *our* → *we* 10; *myself* → *I* 6.
- **`Polarity=Neg` on *not*** is missing in 98 of 872.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 24 of 766.
- **Infinitive root without a subject and without `Mood=Imp`** (`tb.en.imp-mood`): 28 of 813. Probably imperatives, which LinES annotates as `Mood=Imp` elsewhere (130).
- **`dislocated` — 131**, 18 times more than in EWT (`old-structures.md`). Some are probably `nsubj`/`obj` under fronting, but without checking this is only a suspicion.

**In UD.** LinES is better merged only after the converter: otherwise the model will learn two lemma norms and "features only from form". The feature errors (54) do not affect parsing.

**Sources.** `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_lines/index.md` («automatically converted to UD, with some manual corrections of the conversion»); Ahrenberg 2015, `W15-2103` (abstract).

<a id="nominal-feats"></a>

# LinES — pronouns and nominal features: *you* without Case, possessives with old lemmas, no `NumForm` and partly no `Polarity`

**Gist.** The features and lemmas of LinES nouns and pronouns follow old EWT rules (before 2.11–2.15) and have gaps:
- *you* has no `Case` (811 of 811). EWT sets `Nom`/`Acc` by role.
- Possessives: 257 of 1886 `nmod:poss` pronouns lack `Case=Gen`. Among them *his* with lemma *he* (136), *my* → *I* (45), *their* → *they* (37). EWT since 2.11 (docs#517): lemma *his* → *his*, *my* → *my*, `Case=Gen`.
- NUM without `NumForm` — 691 of 691. EWT since 2.13 sets `NumForm=Digit|Word|Roman`.
- *not/n't* without `Polarity=Neg` — 98 of 872. EWT since 2.15 sets `Polarity=Neg` everywhere.
- `fixed` heads without `ExtPos` — 63 of 293.
- NOUN without `Number` — 469, although the XPOS `SG-NOM`/`PL-NOM` shows the number: *garden* 17, *smile* 15, *sir* 15, *jews* 15.
- *an* with lemma *an* — 31 of 357.
- Relative pronouns in `acl:relcl` without `PronType=Rel` — 314 of 513: *that* 258 (no PronType), *which* 36, *who* 17, *whom* 3 (with `PronType=Int`).

**Conditions and exceptions.**
- The other possessives already follow the new rule: *his* → *his* 670, *my* → *my* 364. LinES mixes two norms.
- *her*/*its* in `nmod:poss` have `Case=Acc`/`Case=Nom` and lack `Poss=Yes` — 32 and 22 times. These are feature errors, not a custom (`errors.md`).

**Examples.** `en_lines-ud-dev-doc3-3538` *…a middle-aged woman whose face is so hot…* — *whose*: `nmod:poss` without `Case=Gen`. `en_lines-ud-dev-doc9-5290` *…that did not seem wholly natural…* — *not*: without `Polarity=Neg`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.pron-case`: personal pronoun without `Case` | 811 / 6554 | 0 / 14 842 |
| `tb.en.poss-gen`: `nmod:poss` pronoun without `Case=Gen` | 257 / 1886 | 16 / 3689 |
| `tb.en.num-feats`: NUM without `NumType`/`NumForm` | 691 / 691 | 15 / 5051 |
| `tb.en.not-polarity` | 98 / 872 | 0 / 2077 |
| `tb.en.fixed-extpos` | 63 / 293 | 0 / 586 |
| `tb.en.noun-number`: NOUN without `Number` | 469 / 18 955 | 0 / 43 084 |
| `tb.en.rel-prontype`: relative without `PronType=Rel` | 314 / 513 | 1 / 1238 |
| `tb.en.pron-lemma` | 8 / 2489 | 9 / 5039 |

`tb.en.pron-lemma` is small because the rule accepts any lemma from its list, and *he* is in it too: *his* → *he* passes. The mixing of lemmas is visible with `grep`.

**Sources.** EWT README v2.11 (docs#517: pronoun lemmas and features), v2.13 (NumForm), v2.15 (Polarity, ExtPos); UD `docs/_en/pos/PRON.md`, `feat/Case.md`; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_lines/index.md`.

<a id="old-structures"></a>

# LinES — structures before EWT 2.11–2.16: *when* as `mark`, *by* without `obl:agent`, no `cc:preconj` and `det:predet`

**Gist.** LinES was updated in UD selectively. It already has `:unmarked` (2.15) and `nmod:desc` (2.16): `obl:unmarked` 229, `nmod:unmarked` 162, `nmod:desc` 142. But some of EWT's older English customs are missing:
1. **Subordinate *when/where/how/why*** — SCONJ `mark` in 256 of 524 cases. EWT since 2.11 (#88) — ADV `advmod`, 0 of 1049.
2. **The passive agent *by X*** — plain `obl`, `obl:agent` only 4. EWT since 2.13 uses `obl:agent`: in LinES 152 of 166 such *by* phrases lack it.
3. **No `cc:preconj`:** *both/either/neither … and/or* — CCONJ `cc` (34). In EWT — `cc:preconj` (98).
4. **No `det:predet`:** *all the* — `det` (68), *such a* — `amod` (18), *half* — `amod` (3). In EWT *all the* and *such a* — `det:predet` (226).
5. **Not a single `advcl:relcl`, `goeswith`, `list`.**

**Conditions and exceptions.**
- `nsubj:outer` is present (23), `csubj:outer` — 1: LinES adopted this 2.11 change.
- `dislocated` — 131, in EWT — 7. Fiction has more fronted constituents, but an 18-fold difference looks like a broader interpretation of the label: PRON 60, NOUN 44.
- `acl` — 1070 versus `acl:relcl` 958. Among participles, `acl:relcl` without subject and `mark` — 34 of 280, almost as in EWT (13 of 527).

**Examples.**
- `en_lines-ud-dev-doc1-3177` *When a user connects to the SQL Server database…* — *When*: SCONJ `mark`.
- `en_lines-ud-dev-doc1-3204` *…a standards-based protocol that is governed by the World Wide Web Consortium (W3C).* — *Consortium*: `obl`, not `obl:agent`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 256 / 524 | 0 / 1049 |
| `tb.en.obl-agent`: *by* agent on a passive participle not `obl:agent` | 152 / 166 | 16 / 411 |
| `tb.lines.preconj`: *both/either/neither* CCONJ — not `cc:preconj` | 34 / 34 | 2 / 100 |
| `tb.lines.predet`: *all/such/half/both* before an article or possessive — not `det:predet` | 90 / 90 | 22 / 204 |

**Sources.** EWT README v2.11 (#88, `:outer`), v2.13 (`obl:agent`, #290), v2.16 (*such* as predet, docs#1114); UD `docs/_en/dep/cc-preconj.md`, `dep/det-predet.md`, `dep/obl-agent.md`.

```rule
rule: tb.lines.preconj
what: both/either/neither before coordination — cc:preconj (LinES: cc)
match: p[form=both|either|neither, upos=CCONJ]
require: p[rel=cc:preconj]
severity: warn
source: UD en cc:preconj; counter
```

```rule
rule: tb.lines.predet
what: all/such/half before an article or possessive — det:predet
match: p[form=all|such|half|both]; d[upos=DET|PRON, rel=det|nmod:poss, next=p]
require: p[rel=det:predet]
severity: warn
source: UD en det:predet; EWT 2.16; counter
```

<a id="verb-feats"></a>

# LinES — verb features by form: Number/Person only where visible; modals with `Mood`; no gerund

**Gist.** LinES verb FEATS are derived from the morphological XPOS (`xpos-lines.md`). So they describe the form, not agreement:
- Number and Person are present only where the form shows them: *-s* in 3sg (`Pres` + `Sing|3`), *am*, *was/were*;
- VBP (*they go*) and the past tense of ordinary verbs (*went*) — without Number and Person;
- modals — `Mood=Ind|VerbForm=Fin`, not only `VerbForm=Fin` as in EWT;
- *-ing* forms — always `Tense=Pres|VerbForm=Part`; there is not a single `VerbForm=Ger`.

EWT and `mova` set Number/Person on every finite verb from the subject. A modal there has only `VerbForm=Fin`, and since 2.14 the gerund is separated from the participle.

**Conditions and exceptions.**
- Finite verbs with Tense — 9608, of which 6607 without Number:
  - present without Number — 1609 (all VBP);
  - past without Number — 4996.
- Past with Number — 1360, and all of them are *be* (*was/were*).
- Present non-3sg with Number — 52: *am* 41, *'m* 9 and 2 more.
- 1295 modals, all with `Mood=Ind`. 42 of them also have `Tense=Past` (*could, would*), 5 — `Tense=Pres`.
- *-ing* with `Tense=Pres|VerbForm=Part` — 1973:
  - `advcl` 558;
  - `acl` 303;
  - `root` 299;
  - `conj` 276;
  - `xcomp` 187;
  - `ccomp` 96.

  Some of them would be `VerbForm=Ger` in EWT (EWT: `acl`, `xcomp`, `csubj`…).

**Examples.**
- `en_lines-ud-dev-doc1-3178` *…the account name and password were validated…* — *were*: `Number=Plur|Person=3`. And *verifies* in the same sentence — `Sing|3`: a form with *-s*.
- `en_lines-ud-dev-doc1-3179` *…the user must…* — *must*: `Mood=Ind|VerbForm=Fin`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.fin-agree`: finite with Tense without Number/Person | 6607 / 9608 | 0 / 19 126 |
| `tb.atis.vbp-bare`: present without Number | 1609 / 1609 | 0 / 0 |
| `tb.en.modal-fin`: modal not only `VerbForm=Fin` | 1295 / 1295 | 0 / 4048 |
| `VerbForm=Ger` (`grep`) | 0 | 1230 |
| `tb.en.part-voice`: participle with `aux:pass` without `Voice=Pass` | 5 / 842 | 0 / 1643 |

LinES shares this "feature only from form" custom with PUD, LittlePrince, ATIS and partly ParTUT (`../seeds-overview.md`). UD 2.19 (`docs/changes.md` No. 18) clarifies: a feature underspecified by the form is taken from context. So the EWT custom is normative, and the LinES custom is incomplete.

**Sources.** `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_lines/index.md`; UD `docs/changes.md` No. 18; UD `docs/_en/feat/VerbForm.md`, `feat/Mood.md`; EWT README v2.14 (Ger versus Part, #305).

<a id="xpos-lines"></a>

# LinES — XPOS: own morphological tags, not PTB

**Gist.** LinES XPOS describes not the part of speech but the morphological form: `SG-NOM`, `PL-NOM`, `PAST`, `INF`, `ING`, `PERS-P3SG-NOM`, `P3SG-GEN`, `DEF`, `IND-SG`, `NEG`. The part of speech is carried by UPOS. Function-word ADP, ADV, CCONJ, SCONJ, PART have «_». `mova` keeps XPOS = PTB with EWT web tags, so for `mova` LinES XPOS is a different layer, not a variant of the same one.

**Conditions and exceptions.**
- 159 values. The most frequent:
  - «_» 22,538;
  - `SG-NOM` 16,645;
  - `DEF` 5838;
  - `Comma` 5693, `Period` 5130 — punctuation names in words;
  - `PAST` 5387;
  - `PL-NOM` 4824;
  - `INF` 3296.
- UPOS → XPOS correspondence:
  - NOUN → `SG-NOM` 13,909 / `PL-NOM` 4744;
  - DET → `DEF` 5838 / `IND-SG` 2919;
  - VERB → `PAST` 3909 / `INF` 2929 / `ING` 1883 / `PRES` 1547 / `PASS` 1338 / `PERF` 755;
  - AUX → `PAST-AUX` 1381 / `PRES-AUX` 1321 / `PAST` 1476 / `PRES` 833.
- A PTB tag can be recovered from this only partly. `SG-NOM` on NOUN is NN, on PROPN NNP. `PAST` — VBD. `PERF` and `PASS` — VBN. `ING` — VBG. `INF` — VB. But `PRES` does not distinguish VBP from VBZ, and «_» on ADP does not say whether it is IN or TO.
- The CoNLL-U reader in `en` rejects such tags as unknown (`Tag::parse` → none), so LinES does not go into PTB tagger training (`Sentence::tagged()`).

**Examples.** `PRON → PERS-P3SG-NOM` (*he, she, it*) 1485, `PART → NEG` (*not*) 870, `PUNCT → Quote` 527.

**In UD.** The converter into `mova` can set a PTB tag where the correspondence is unambiguous: NN, NNS, NNP, VBD, VBN, VBG, VB, DT. VBP/VBZ need Number and Person, and IN/TO/RP/RB need the lemma and role. So `mova` XPOS for LinES will be incomplete. Back, `mova` → LinES, the map is unambiguous for content words, because LinES XPOS is derived from UPOS and FEATS (`convert.md`).

**Sources.** `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_lines/index.md` («XPOS: annotated manually»); Ahrenberg 2007, `W07-2441`; `en/src/gram.rs` (the list of PTB tags in `en`).
