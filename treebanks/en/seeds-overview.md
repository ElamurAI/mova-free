# English UD 2.18 treebanks — comparative seed

**Gist.** UD 2.18 has 13 English treebanks, and they are annotated in different dialects:
- EWT — the base of `mova`: UD 2.18 with EWT conventions;
- GUM, GENTLE and GUMReddit — almost EWT, but with their own customs;
- the rest are "stuck" on different EWT/UD versions or brought customs from other schemes: PTB tags (ATIS), Italian TUT (ParTUT), FDG (LinES), Google-SD and CoreNLP (PUD), Stanza (LittlePrince), UD 1 (part of CHILDES), UD 2.0 (ESLSpok).

Merged as is, they teach the model contradictions: the same imperative, the same *by* phrase, the same compound noun get different annotation in different treebanks. Below are a table of treebanks × customs with numbers, a list of contradictions, and what training on `mova` through converters will give.

Each treebank's seeds are in `<treebank>/seeds/`, converters in `<treebank>/convert.md`. PTB is separate (`ptb/`): there are no data, only tagging customs and the constituents → dependencies path.

## How it was counted

- **Counter rules.** The general `tb.en.*` at the end of this seed and each treebank's own `tb.<treebank>.*` in the treebank seeds. Run: `en expert-check treebanks/en <treebank files>`. The engine collects all rules from the folder. The notation "*v*/*n*": *n* — matches (`fired`), *v* — violations of the `mova`/EWT norm. Numbers are over all parts of the treebank together.
- **Negative control.** Each rule is also run on EWT. There it must give 0 or a small residue: errors of EWT itself (`ewt/seeds/errors.md`). A rule that is red on EWT is wrong for `mova`.
- **`grep`/`cut`** — for what the rules do not see: layers, labels and features with a comma (`PronType=Int,Rel`), XPOS outside PTB.
- "—" — the rule has no matches: the layer or forms are missing, or the phenomenon is absent from the treebank.

## 1. Layers and origin

| treebank | sentences / words | license | how annotated | lemmas | XPOS | FEATS | DEPS | MWT | for `mova` |
|---|---|---|---|---|---|---|---|---:|---|
| ATIS | 5432 / 61 879 | CC BY-SA 4.0 | manually; UPOS and FEATS — by rules from hidden PTB tags | ✓ | — | from PTB, without agreement | — | 406 | training after the converter |
| CHILDES | 48 183 / 302 740 | CC BY-SA 4.0 | 3 sources, harmonized; 11 446 sentences `gold_annotation = False` | auto + correction | auto, `?`/`!` | — | ✓ | 12 905 | training with a weight |
| CTeTex | 276 / 9273 | CC BY-SA 4.0 | manually, UD | ✓ (since 2.16) | — | only Number, Tense | — | 0 | test |
| ESLSpok | 2320 / 21 312 | CC BY-SA 4.0 | manually, UD 2.0 | — | PTB 1990 | — | — | 0 | training (already in `en`) |
| **EWT** | 16 622 / 254 820 | CC BY-SA 4.0 | PTB → SD → UD, manually | CoreNLP + correction | PTB + web tags | auto + correction | auto | 3327 | base |
| GENTLE | 1334 / 17 799 | CC BY-NC-SA 4.0 | as GUM | ✓ | PTB without web tags | ✓ | ✓ | 180 | test |
| GUM | 14 353 / 256 739 | CC BY-NC-SA 4.0 | manually; SD → UD, since v6 directly UD | ✓ | PTB without web tags | ✓ | ✓ | 4455 | test and research (NC) |
| GUMReddit | 895 / 16 364 | CC BY 4.0 (annotations) | as GUM | no text | ✓ | ✓ | ✓ | 406 | after `get_text.py` |
| LinES | 5696 / 106 307 | CC BY-NC-SA 4.0 | conversion from non-UD + review | ✓ | own morphological | from XPOS | — | 1170 | test and research (NC) |
| LittlePrince | 500 / 6852 | CC BY-SA 4.0 | Stanza + student correction | ✓ | PTB (Stanza) | ✓ | — | 0 | test |
| ParTUT | 2090 / 49 478 | CC BY-NC-SA 4.0 | TUT → USD → UD | ✓ | TUT (Ital.) | ✓, without `Case`, `Voice` | — | 445 | test and research (NC) |
| Pronouns | 285 / 1705 | CC BY-SA 4.0 | manually, constructed | ✓ | PTB | ✓, without `Case` | — | 65 | test |
| PUD | 1000 / 21 180 | CC BY-SA 3.0 | Google → UD, manual syntax correction | CoreNLP | PTB | CoreNLP | auto | 129 | test |

README metadata do not always match the data:
- LittlePrince says "XPOS: not available", but XPOS is on all words;
- ESLSpok — "Features: manual native", but there are no FEATS;
- LinES — "Features: not available", but FEATS are present;
- in PUD the changelog ends at 2.6, but the data have `:unmarked` (2.15) and `nmod:desc` (2.16).

## 2. Customs: labels and trees

| treebank | `:tmod`/`:npmod` | *by* without `obl:agent` | *when…* as `mark` | relative *that* — `mark` | N N not `compound` | `obl` in a fragment | *to* preposition PART | passive without `nsubj:pass` | conjunction not SCONJ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ATIS | 1263/1263 | 12/12 | 3/183 | 293/463 | 51/1945 | 22/30 | 1/4216 | 10/10 | 284/284 |
| CHILDES | 241/241 | — | 676/3308 | 61/271 | 395/3650 | 49/106 | 24/1104 | 22/263 | 11/1430 |
| CTeTex | — | — | 36/47 | 0/20 | 314/726 | 0/1 | 4/98 | 3/107 | 1/55 |
| ESLSpok | 151/151 | — | 41/103 | 2/15 | 4/237 | 7/20 | 224/224 | 1/35 | 0/160 |
| **EWT** | — | 16/411 | 0/1049 | 1/909 | 19/5376 | 12/196 | 0/2029 | 0/1405 | 0/2419 |
| GENTLE | — | 7/35 | 0/38 | 2/64 | 11/414 | 11/19 | 0/152 | 2/94 | 0/121 |
| GUM | — | 17/497 | 0/1203 | 6/1263 | 19/4017 | 56/207 | 0/1943 | 3/1689 | 0/2212 |
| GUMReddit | — | — | — | — | 2/256 | 2/6 | — | 0/71 | — |
| LinES | — | 152/166 | 256/524 | 13/408 | 21/1716 | 5/31 | 0/991 | 24/766 | 0/942 |
| LittlePrince | 9/9 | 3/11 | 18/43 | 1/34 | 2/11 | 0/4 | 1/64 | 1/28 | 0/66 |
| ParTUT | — | 141/157 | 72/155 | 6/291 | 862/974 | 0/11 | 0/543 | 1/453 | 0/460 |
| Pronouns | — | — | — | — | — | — | 0/10 | 25/25 | 0/25 |
| PUD | — | 66/75 | 4/69 | 0/113 | 11/414 | 0/21 | 0/192 | 2/240 | 0/118 |

Largest discrepancies:
- **ATIS:** `:tmod` as semantic "time", even with a preposition (1710 of 2114); relative *that* as `mark` (293); conjunctions as ADP (284).
- **ParTUT:** `nmod` in compound nouns (862).
- **CTeTex:** the same `nmod`, half of the time (314).
- ***by* agent without `obl:agent`:** LinES (152), ParTUT (141), PUD (66).
- **Subordinate *when* as `mark`:** CHILDES (676), LinES (256).
- **GUM:** `obl` in fragments (56).
- **ESLSpok:** *to* preposition — PART (224).

Three more customs are not counted by a rule — `grep`:
- `dep`: GUM 184, GENTLE 96, EWT 5 (`gum/seeds/dep.md`: *you guys*, photo captions, numbers);
- `flat:foreign`: ParTUT 88, ESLSpok 14, EWT 0;
- `acl:relcl` on a participle without a subject: ATIS 662, EWT 13 (`atis/seeds/acl-relcl-participle.md`).

## 3. Customs: features and lemmas

| treebank | imperative not `Imp` | finite without Number | MD not only `Fin` | without `Voice=Pass` | pronoun without `Case` | possessive without `Gen` | NUM without `NumForm` | `Degree` on an invariable ADV | *you* with Number | pronoun lemmas |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ATIS | 1709/2599 | 1654/2903 | 783/783 | 9/9 | 2/2640 | — | 1191/1191 | 132/132 | 223/223 | 0/1261 |
| CHILDES | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | 1940/6594 |
| CTeTex | only Number, Tense | only Number, Tense | 246/246 | only Number, Tense | only Number, Tense | only Number, Tense | 317/317 | 0/35 | 0/3 | — |
| ESLSpok | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no FEATS | no lemmas |
| **EWT** | 38/3066 | 0/19126 | 0/4048 | 0/1643 | 0/14842 | 16/3689 | 15/5051 | 0/5076 | 0/2768 | 9/5039 |
| GENTLE | 23/207 | 0/948 | 0/184 | 0/117 | 0/672 | 2/235 | 0/386 | 88/287 | 153/153 | 0/285 |
| GUM | 64/2472 | 0/19346 | 0/2945 | 0/1908 | 0/13099 | 26/3438 | 2/4626 | 1146/4668 | 2461/2461 | 0/4039 |
| GUMReddit | 6/185 | 0/1547 | — | 0/95 | — | 1/267 | 0/107 | 502/907 RB\*\* | 210/210\*\* | no lemmas |
| LinES | 28/813 | 6607/9608 | 1295/1295 | 5/842 | 811/6554 | 257/1886 | 691/691 | 1/2463 | 0/811 | 8/2489 |
| LittlePrince | 1/81 | 317/625 | 0/93 | 0/32 | 0/570 | 110/122 | 142/142 | 3/168 | 10/84 | 15/201 |
| ParTUT | 7/306 | 2302/3689 | 690/690 | 472/472 | 1153/1153 | DET\* | 817/832 | 0/495 | 0/131 | 31/129 |
| Pronouns | — | 90/315 | 0/5 | 20/20 | 40/40 | — | 5/5 | 0/5 | — | 0/228 |
| PUD | 4/74 | 940/1569 | 0/164 | 272/272 | 0/474 | 260/260 | 0/464 | 0/234 | 0/15 | 0/222 |

\* ParTUT: possessives are DET, so the PRON rule does not see them. The custom is in the `tb.en.poss-pron` column: 615 of 617 are not PRON.
\*\* GUMReddit without text: the feature variants of the rules are counted (`gumreddit/seeds/gum-family.md`).

Largest discrepancies:
- **Agreement by form:** Number/Person only where they are visible in the form. LinES 6607, ParTUT 2302, ATIS 1654, PUD 940, LittlePrince 317 — together ≈ 11 800 finite verbs without the features that EWT sets from the subject.
- **Modals with `Mood`/`Tense`/`Person`:** LinES 1295, ParTUT 690.
- **Without `Voice=Pass`:** ParTUT 472, PUD 272.
- **Without `Case`:** ParTUT 1153, LinES 811 (*you*).
- **Without `NumForm`:** ATIS, LinES, ParTUT, LittlePrince.
- **`Degree` on invariable adverbs:** GUM 1146.
- **Number on *you*:** GUM 2461, ATIS 223, GENTLE 153.

## 4. Which version each treebank is "stuck" on

The chronology of EWT customs (EWT README, Changelog) is the axis on which one can see what each treebank took and what it did not:

| EWT version | custom | lacking it |
|---|---|---|
| 1.2 | *this/that* as a noun — PRON; conjunctions — SCONJ | ATIS (DET, ADP), CHILDES (304 DET) |
| 2.7–2.8 | MWT for contractions; ADJ/VERB in names instead of PROPN | ESLSpok, LittlePrince, CTeTex (no MWT); ATIS (*new* — PROPN) |
| 2.10 | «X, so Y» — `parataxis` | ESLSpok (*so* — CCONJ `cc`, 60) |
| 2.11 | `:outer`, `advcl:relcl`; possessive lemmas (*his* → *his*); WH adverbs — `advmod` | CHILDES, LinES, ParTUT, ESLSpok, LittlePrince, CTeTex |
| 2.12 | sole `iobj` allowed | — (all have it) |
| 2.13 | `obl:agent`, `Voice=Pass`; no `flat:foreign`; DET and NUM features | LinES, ParTUT, PUD (`obl:agent`); ParTUT, PUD, Pronouns (`Voice`); ESLSpok, ParTUT (`flat:foreign`) |
| 2.14 | numbering markers — `discourse`; Ger versus Part; `Mood=Sub` | CTeTex (`nummod`); LinES, ATIS (no `Ger`) |
| 2.15 | `:tmod`/`:npmod` → `:unmarked`; `ExtPos`; `Polarity=Neg` | ATIS (`nmod:tmod`), ESLSpok, CHILDES (`:npmod`), LittlePrince |
| 2.16 | `nmod:desc`; dates; numbered entities | ATIS, CHILDES, CTeTex, ESLSpok, LittlePrince, Pronouns |
| 2.17 | `flat` for place names; *you guys* — `nmod:unmarked`; *there* — `PronType=Dem`; `Case` on *it/you* | GUM (*you guys* — `dep`), LittlePrince (*there*), LinES and ParTUT (`Case`) |
| 2.18–2.19 | `Typo`; features from context legitimized (`docs/changes.md` No. 18) | all with agreement by form |

Mass edits by the UD maintainers gave `:unmarked` and `nmod:desc` even to those that lack other changes: LinES, ParTUT, PUD. So a treebank's "version" is not one number but a set.

## 5. Why merging without converters is contradictory

A model trained on the union sees two norms for the same phenomenon. Which one wins will be decided by frequency, not by grammar:

| phenomenon | EWT / `mova` norm | other norm | where and how many |
|---|---|---|---|
| imperative *show me…* | `Mood=Imp\|VerbForm=Fin` (1499 VB) | `VerbForm=Inf` | ATIS 1709 |
| *they go*, *went* | Number/Person from the subject (19 126) | without Number/Person | LinES, ParTUT, ATIS, PUD, LittlePrince ≈ 11 800 |
| *on mondays* | `nmod` + `case` | `nmod:tmod` + `case` | ATIS 1022 |
| temporal NP | `:unmarked` + `TemporalNPAdjunct` | `:tmod`/`:npmod` | ATIS 1263, CHILDES 241, ESLSpok 151 |
| *governed by X* | `obl:agent` (376) | `obl` | LinES 152, ParTUT 141, PUD 66 |
| subordinate *when* | ADV `advmod` (0 `mark`) | SCONJ `mark` | CHILDES 676, LinES 256, ParTUT 72, ESLSpok 41 |
| *law firm* | `compound` (4750) | `nmod` | ParTUT 797, CTeTex 307 |
| *a flight that arrives* | *that* PRON `nsubj` | *that* ADP `mark` | ATIS 293, CHILDES 61 |
| *flights leaving at 5* | `acl` | `acl:relcl` | ATIS 662 |
| *you guys* | *you* — head, *guys* — `nmod:unmarked` | *guys* — head, *you* — `dep` | GUM 56 |
| *go to school* | *to* ADP/IN | PART/TO | ESLSpok 224 |
| *his* (possessive) | PRON, lemma *his*, `Case=Gen` | lemma *he* / DET | LinES 136, LittlePrince 39; ParTUT 615 DET |
| *your* (possessive) | lemma *your* | lemma *you* | CHILDES 1093 |
| *you* | without Number | `Number=Sing/Plur` | GUM 2461 |
| *really* | without `Degree` | `Degree=Pos` | GUM 1146 |

Part of the "foreign composition" that does not help training comes precisely from these customs, not from genre. `en/README.md`: CHILDES (+35k sentences) gave EWT LAS −0.2, while GUM (+11k) gave +0.8. CHILDES has hundreds of contradictory labels (WH-`mark`, relative `mark`, DET pronouns), GUM has dozens per hundreds of thousands of words.

## 6. What training on `mova` will give

1. **Consistent labels and features.** After the converters every phenomenon in the table above has one norm. The model will stop splitting probability between two annotations, and each doubling of the data will give a gain as for data of the same composition: +0.8 tagging, +2.5–3 LAS (`en/README.md`, "How much more data gives").
2. **More data for weak spots.** The license allows training on ATIS (4274 train sentences) and CHILDES (34 732). ATIS is user queries, our worst genre: LAS 63.01, UPOS 74.50. After the converter these are the same queries in the EWT norm, i.e. a corpus close to real RAG queries. CHILDES is dialogue and questions: questions there are ≈ 45% of the number of declaratives, in GUM — 9% (Yang et al. 2025).
3. **Missing layers do not train.** The converter does not invent lemmas for ESLSpok, FEATS for CHILDES and ESLSpok, XPOS for ATIS, CTeTex, LinES, ParTUT. Such sentences train only the layers that exist, as `en::ud` already does (`has_lemmas`, `has_feats`, `tagged`).
4. **Honest evaluation on tests.** The reverse converter `mova` → treebank makes it possible to measure `en` in the treebank's dialect. UFeats on LinES (83.44), PUD (87.55), LittlePrince (85.90), Pronouns (69.56), ParTUT (78.90) and MLAS on ATIS (22.53) and CTeTex (13.26) are currently lowered by the difference in norms. After the reverse converter the real errors become visible.
5. **NC treebanks — test and research only.** GUM, GENTLE, LinES, ParTUT. Their converters are needed for evaluation and for the two roads to the goal (`docs/dialect.md`), not for training commercial models.
6. **Candidates for borrowing into `mova`.** Customs more informative than EWT that convert to EWT without loss:
   - the number of *you* from GUM (`gum/seeds/you-number.md`);
   - singular *they/theirs* from Pronouns (`pronouns/convert.md`).

   Decide in `dialects/mova/seeds/`.
7. **Negative control:**
   - training on converted data must not worsen EWT test;
   - a converter with a deliberate error must give divergences in the reverse run and worsen the test. For example, `atis.imperative` without the subject condition would make 890 verbs with a subject imperative.

## 7. Conventions for `convert.md`

The language is `docs/dialect.md`, "Converter rule language". There is no engine yet, so the drafts rest on the following assumptions. They must be checked against how the engine is built.

- **Fields:** `rule`, `what`, `from`, `to`, `match`, `set`, `source` and the optional `require:`.
- **`require:` — an application condition.** Actions are performed only on matches where all `require` items hold; within an item, at least one ` or ` alternative. The semantics are the same as in seeds, only reversed: there an unmet `require` is a violation, here it is a skip. `unless:` is not used in converters.
- **`set:`** — actions separated by `;`, each in its own brackets: `v[feats+=VerbForm=Fin]; v[feats+=Mood=Imp]`.
- **`feats+=X=V`** replaces the existing value of X rather than adding a second one.
- **Order.** The rules of a `from → to` pair run in turn, and each next one sees the result of the previous: this is in the specification. So agreement comes after relative pronouns, removing Number from *you* after agreement, etc.
- **```` ```convert-todo ````** — a rule that lacks an engine capability. The engine skips such blocks, and when the capability appears, the block becomes ```` ```convert ````.
- **Matches** in `convert.md` were counted by `en expert-check`: the `match` and `require` of each rule were carried over into a seed rule. For `tb → mova` the count is on the treebank, for `mova → tb` on EWT as a stand-in for `mova`. For rules that rely on previous ones, the number on the source data is lower than the real one, and this is marked.

**What the `convert` language lacks** (from concrete rules):

| need | where | how many |
|---|---|---|
| a comma in a value: `PronType=Int,Rel`, `xpos=,` | ATIS `int-rel`; GUM ellipsis | 1793 + 94 |
| string XPOS — tags outside PTB and `xpos=_` | LinES, ParTUT, CHILDES `?`/`!`, PTB `-LCB-`; clear XPOS for ATIS and CTeTex | LinES 106 307, ParTUT 49 478, CHILDES 9652 |
| a condition on MISC: `misc.K=V` | reverse `:tmod`/`:npmod` by `TemporalNPAdjunct`: ESLSpok, CHILDES, LittlePrince, ATIS | 1077 on EWT |
| copy a relation: `rel=@v` | head swap: *you guys* (GUM 56), *new york* (ATIS 1861) | ≈ 1900 |
| clear a column: `feats=_`, `misc=_`, `lemma=_` | `mova` → ESLSpok, CHILDES | all tokens |
| feature → MISC: `misc+=K=@feats.X` | LittlePrince `NumForm` | 157 |
| token actions: MWT, merging and splitting | ESLSpok (contractions without MWT), PTB (hyphens) | ≈ 540 + 852 |
| a condition on a sentence comment (`# child_name`) | CHILDES: pronoun lemmas by subcorpus | ≈ 1400 |

## Sources

- READMEs of all 13 UD 2.18 treebanks (`$MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-*/README.md`, LinES — LICENSE only) and the pages `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_*/index.md`.
- UD `docs/changes.md` (No. 7, 13, 14, 15, 17, 18) and `docs/_en/`.
- Digest the UD survey digest «english-banks».
- Zeldes & Schneider 2023, `2023.udw-1.7`; Peng & Zeldes 2018, `W18-4918`; Kyle et al. 2022, `2022.bea-1.7`; Cesur et al. 2024, `2024.bucc-1.11`; Yang et al. 2025, `2025.udw-1.6`; Hassert et al. 2021, `2021.udw-1.5`; Aoyama et al. 2023, `2023.law-1.17`; Zeman et al. 2017, `K17-3001`.
- `en` results by treebank — `en/README.md`, "Full UD annotation".

## General counter rules

```rule
rule: tb.en.imp-mood
what: imperative — VERB root without subject, aux and mark: Mood=Imp|VerbForm=Fin
match: v[upos=VERB, head=0, feats.VerbForm=Inf|Fin, !feats.Tense]
require: v[feats.Mood=Imp] or exists c[rel~nsubj, head=v] or exists c[rel~csubj, head=v] or exists c[rel=expl, head=v] or exists c[rel~aux, head=v] or exists c[rel=mark, head=v]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.fin-agree
what: a finite verb with tense has Number and Person
match: v[feats.VerbForm=Fin, feats.Tense]
require: v[feats.Number]; v[feats.Person]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.fin-mood
what: a finite verb with tense has Mood
match: v[feats.VerbForm=Fin, feats.Tense]
require: v[feats.Mood]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.modal-fin
what: a modal verb — only VerbForm=Fin
match: m[upos=AUX, lemma=will|would|can|could|may|might|shall|should|must]
require: m[feats.VerbForm=Fin]; not m[feats.Tense]; not m[feats.Mood]; not m[feats.Person]
source: EWT 2.18 (MD)
severity: warn
```
```rule
rule: tb.en.part-voice
what: a participle with aux:pass has Voice=Pass
match: v[feats.VerbForm=Part, feats.Tense=Past]; a[rel=aux:pass, head=v]
require: v[feats.Voice=Pass]
source: EWT 2.13
severity: warn
```
```rule
rule: tb.en.obl-agent
what: a by phrase on a past participle without have — obl:agent
match: v[feats.VerbForm=Part, feats.Tense=Past]; o[rel=obl|obl:agent, head=v]; b[form=by, rel=case, head=o]
require: o[rel=obl:agent] or exists c[rel=aux, head=v, lemma=have]
source: EWT 2.13
severity: warn
```
```rule
rule: tb.en.tmod-npmod
what: old subtypes :tmod/:npmod (before UD 2.15)
match: t[rel=obl:tmod|obl:npmod|nmod:tmod|nmod:npmod]
require: t[rel=obl:unmarked]
source: UD 2.15, docs#1028
severity: warn
```
```rule
rule: tb.en.pron-case
what: a personal pronoun has Case
match: p[upos=PRON, feats.PronType=Prs, !feats.Poss, !feats.Reflex, form=i|me|he|him|she|her|we|us|they|them|it|you]
require: p[feats.Case]
source: EWT 2.17
severity: warn
```
```rule
rule: tb.en.poss-gen
what: a possessive determiner pronoun (nmod:poss) has Case=Gen
match: p[upos=PRON, feats.Poss=Yes, rel=nmod:poss]
require: p[feats.Case=Gen]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.poss-pron
what: possessive modifier my/your/his… — PRON
match: p[form=my|your|his|its|our|their, rel=nmod:poss]
require: p[upos=PRON]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.pron-lemma
what: the lemma of a personal pronoun is the nominative; of a possessive, the possessive itself (EWT 2.11)
match: p[upos=PRON, form=me|him|us|them|my|his|our|their|mine|ours|theirs|yours|your]
require: p[lemma=i|he|we|they|my|his|our|their|your] 
source: EWT 2.11, docs#517
severity: warn
```
```rule
rule: tb.en.art-feats
what: an article has PronType=Art and Definite
match: d[form=a|an|the, upos=DET]
require: d[feats.PronType=Art]; d[feats.Definite]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.an-lemma
what: the lemma of an is a
match: d[form=an, upos=DET]
require: d[lemma=a]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.not-polarity
what: not/n't has Polarity=Neg
match: n[form=not|n't, upos=PART]
require: n[feats.Polarity=Neg]
source: EWT 2.15
severity: warn
```
```rule
rule: tb.en.adj-degree
what: an adjective has Degree
match: a[upos=ADJ]
require: a[feats.Degree]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.num-feats
what: a numeral has NumType and NumForm
match: n[upos=NUM]
require: n[feats.NumType]; n[feats.NumForm]
source: EWT 2.13
severity: warn
```
```rule
rule: tb.en.noun-number
what: a common noun has Number
match: n[upos=NOUN]
require: n[feats.Number]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.expl-there
what: expletive there — PRON with PronType=Dem
match: t[form=there, rel=expl]
require: t[upos=PRON]; t[feats.PronType=Dem]
source: EWT 2.17
severity: warn
```
```rule
rule: tb.en.rel-that
what: relative that as subject/object — PRON
match: v[rel=acl:relcl]; t[form=that, rel=nsubj|nsubj:pass|obj|obl, head=v, before=v]
require: t[upos=PRON]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.rel-prontype
what: a relative pronoun has PronType=Rel
match: v[rel=acl:relcl]; t[form=that|which|who|whom, upos=PRON, rel=nsubj|nsubj:pass|obj|obl, head=v, before=v]
require: t[feats.PronType=Rel]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.dem-pron
what: demonstrative this/that as a noun — PRON
match: t[form=this|that|these|those, rel=nsubj|nsubj:pass|obj|iobj|obl|root|conj]
require: t[upos=PRON]
source: EWT 1.2
severity: warn
```
```rule
rule: tb.en.to-part
what: infinitival to — PART
match: t[form=to, rel=mark]
require: t[upos=PART]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.cop-be
what: the copula — only be
match: c[rel=cop]
require: c[lemma=be]
source: UD en cop
severity: warn
```
```rule
rule: tb.en.sole-iobj
what: iobj without obj — allowed since UD 2.12
match: v[]; i[rel=iobj, head=v]
require: exists c[rel=obj, head=v]
source: UD 2.12, Sole iobj
severity: warn
```
```rule
rule: tb.en.fixed-extpos
what: the head of fixed has ExtPos
match: h[]; f[rel=fixed, head=h, next=h]
require: h[feats.ExtPos]
source: UD 2.16, ExtPos
severity: warn
```
```rule
rule: tb.en.part-tense
what: a participle has Tense
match: v[feats.VerbForm=Part]
require: v[feats.Tense]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.pass-subj
what: subject with aux:pass — nsubj:pass
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj|nsubj:pass, head=v]
require: s[rel=nsubj:pass]
source: UD en nsubj:pass
severity: warn
```
```rule
rule: tb.en.poss-s
what: possessive 's — PART with case
match: s[form='s|'|’s, xpos=POS]
require: s[upos=PART]; s[rel=case]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.to-case
what: prepositional to (case) — ADP
match: t[form=to, rel=case]
require: t[upos=ADP]
source: EWT 2.18; UD en pos/ADP
severity: warn
```
```rule
rule: tb.en.compound-prt
what: phrasal verb particle — compound:prt
match: v[upos=VERB]; p[upos=ADP|ADV, rel=compound|compound:prt, head=v]
require: p[rel=compound:prt]
source: UD en compound:prt
severity: warn
```
```rule
rule: tb.en.nn-compound
what: a noun immediately before a head noun — compound
match: n[upos=NOUN]; h[upos=NOUN, next=n]
require: not n[head=h] or n[rel=compound|nmod:poss]
source: UD en compound
severity: warn
```
```rule
rule: tb.en.obl-fragment
what: obl on a noun root without a copula (fragment)
match: h[upos=NOUN|PROPN|PRON|NUM, head=0]; o[rel~obl, head=h]
require: exists c[rel=cop, head=h]
source: UD 2.18, nmod and obl (obl-should-be-nmod)
severity: warn
```
```rule
rule: tb.en.adv-degree
what: invariable adverbs without Degree
match: a[upos=ADV, lemma=really|back|still|again|too|therefore|down|above|here|there|now|then|just|so|very|also|even|only|up|out|away|together|yet|ever]
require: not a[feats.Degree]
source: EWT 2.18 (Degree only on well, far, soon, long, more…)
severity: warn
```
```rule
rule: tb.en.sconj-mark
what: a subordinating conjunction — SCONJ
match: m[rel=mark, form=that|if|because|whether|although|though|unless]
require: m[upos=SCONJ]
source: EWT 1.2; UD en pos/SCONJ
severity: warn
```
```rule
rule: tb.en.propn-number
what: a proper noun has Number
match: n[upos=PROPN]
require: n[feats.Number]
source: EWT 2.18
severity: warn
```
```rule
rule: tb.en.wh-advmod
what: when/where/how/why in a subordinate clause — ADV advmod, not mark (EWT 2.11)
match: w[form=when|where|how|why|whenever|wherever]
require: not w[rel=mark]
source: EWT 2.11 (#88)
severity: warn
```
