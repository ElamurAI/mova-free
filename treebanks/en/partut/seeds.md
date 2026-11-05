# Seeds: treebanks/en/partut

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# ParTUT — what these data are and which layers exist

**Gist.** The English part of the parallel Italian-French-English treebank ParTUT (Turin): 2090 sentences and 49,478 words. Sources:
- Creative Commons licenses;
- DGT-TM, JRC-Acquis, the Universal Declaration of Human Rights — law;
- Europarl;
- Facebook pages;
- Project Syndicate;
- Wikipedia;
- WIT3 (talks).

The annotation was originally done in the TUT scheme — dependencies grown out of Italian grammar. From there it was converted to Universal Stanford Dependencies, then to UD, with manual corrections. All columns are marked "converted with corrections". No changes since 2.5.

**Conditions and exceptions.**
- Split: train 1781 / dev 156 / test 153 sentences. The English part is split the same way as the Italian one, to preserve parallelism.
- License CC BY-NC-SA 4.0: for `mova` test and research only.
- `# sent_id` and `# text` are present, `# newdoc id` is not.

**Examples.** `…Shakespeare's property…` — *Shakespeare's* is split as an MWT: *Shakespeare* (`nmod:poss`) + *'s* (PART `case`); *property* — `nmod` → the next noun (`nmod-compound.md`).

**In UD.**

| layer | state |
|---|---|
| XPOS | the Italian TUT/ISST set: `S`, `E`, `V`, `A`, `RD`, `FF`, `SP`, `AP`…; «_» — 73 (`xpos-tut.md`) |
| FEATS | present, without `Case`, `Voice`, `NumForm`, `Reflex` (`feats.md`) |
| lemmas | present |
| DEPS | absent |
| MISC | «_» in 43,590 of 49,478: only `SpaceAfter` |
| MWT | 445, of which 353 are possessive *X's* |

Our annotator on the ParTUT test: UPOS 92.12, UFeats 78.90, LAS 77.67, MLAS 44.45 (`en/README.md`). XPOS is not measured: the tags are not PTB.

**Sources.** README `UD_English-ParTUT`; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_partut/index.md`; Sanguinetti & Bosco 2014 (PartTUT; conversion to USD, CLiC-it 2014; TLT-13); Bosco et al. 2013, `W13-2308`; digest the UD survey digest «english-banks», §1.11.

<a id="errors"></a>

# ParTUT — known annotation errors

**Gist.** ParTUT is an automatic conversion from TUT via USD with manual corrections. The last corrections were in 2.5. So alongside the Italian customs there are traces of the conversion: lemmas, labels and features that match neither EWT nor ParTUT itself. The converter does not fix them.

**Examples and numbers.**
- **Pronoun lemmas in lowercase and in the oblique case:** *me* → *i* (9, lowercase), *us* → *us* (28), *them* → *them* (1), *themselves* → *they* (5), *our/ours* → *us* (60 DET + 2 PRON). EWT: *I*, *we*, *they*, *themselves*, *our*.
- **An adjacent noun as `obj`** of the head noun — 15 (`nmod-compound.md`): a noun does not take `obj`.
- **Possessive PRON among DET** — 2 of 617: an inconsistency in the custom (`structures.md`).
- **`Number` on PROPN** — only 82 of 2230: some proper nouns have Number, the rest do not.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 1 of 453.
- **`obl` on a noun without a copula** — 0 of 11. Here ParTUT already agrees with UD 2.18.

**In UD.** For merging, ParTUT's main problem is not errors but customs: `nmod` in compound nouns (862), features without `Case` and `Voice`, possessive DET. The errors (≈ 120) are within the noise.

**Sources.** README `UD_English-ParTUT` (changelog: last corrections — 2.5); rules in `../seeds-overview.md`.

<a id="feats"></a>

# ParTUT — FEATS without `Case`, `Voice`, `NumForm`; modals with `Person=3|Tense`; *are* with Number without Person

**Gist.** ParTUT FEATS were converted from TUT's Italian morphology. So the feature set is Italian in spirit:
- **No `Case` anywhere.** The pronouns *I, he, they, them* have only `Number|Person|PronType`: 1153 of 1153 personal pronouns without `Case`.
- **No `Voice`.** 472 of 472 participles with `aux:pass` without `Voice=Pass`.
- **No `NumForm`** (817 of 832 NUM).
- **PROPN without Number** — 2148 of 2230.
- **Modals** — `Mood=Ind|Person=3|Tense=Pres|VerbForm=Fin` (*shall, can, will, may*): 690 of 690. In EWT MD has only `VerbForm=Fin`.
- **Verb agreement — halfway.** *is, has* — `Number=Sing|Person=3`. *are* (222) and *have* (125) — `Number=Plur` without `Person`. The past tense has no Number and Person, except *was/were*.

**Conditions and exceptions.**
- Finite verbs with Tense without Number/Person — 2302 of 3689 (`tb.en.fin-agree`).
- *not* has `Polarity=Neg` (235 of 235) — here ParTUT is already like EWT 2.15.
- `Mood=Sub` — 18: more than in GUM (27 on a five times larger volume). Probably inherited from the Italian `congiuntivo`.

**Examples.**
- `en_partut-ud-72` *You have requested a debate…* — *You*: `Person=2|PronType=Prs`, without `Case`; *have*: without Person.
- `en_partut-ud-7` *The work … is provided under the terms…* — *provided*: `Tense=Past|VerbForm=Part`, without `Voice=Pass`.

**In UD.**

| rule | ParTUT | EWT |
|---|---:|---:|
| `tb.en.pron-case` | 1153 / 1153 | 0 / 14 842 |
| `tb.en.part-voice` | 472 / 472 | 0 / 1643 |
| `tb.en.num-feats` | 817 / 832 | 15 / 5051 |
| `tb.en.propn-number` | 2148 / 2230 | 475 / 16 562 |
| `tb.en.modal-fin` | 690 / 690 | 0 / 4048 |
| `tb.en.fin-agree` | 2302 / 3689 | 0 / 19 126 |

**Sources.** README `UD_English-ParTUT` («Features: converted with corrections»); UD `docs/_en/feat/Case.md`, `feat/Voice.md`, `pos/AUX_.md`; UD `docs/changes.md` No. 18 (Number/Person from context).

<a id="nmod-compound"></a>

# ParTUT — a noun before a noun — `nmod`, not `compound`

**Gist.** Italian has no compounds like *law firm*: a noun modifier always has a preposition (*studio di avvocati*). The TUT scheme from which ParTUT was converted annotates such a modifier as `nmod`, and the English part kept it that way. So:
- *law firm*, *guidance programmes*, *quota penalties* — the left noun is `nmod` → the right one, without `case`;
- `compound` in ParTUT — 124 per 49,478 words (0.25%). In EWT — 9047 per 254,820 (3.6%).

EWT and `mova`: a noun before a head noun is `compound`.

**Conditions and exceptions.**
- Adjacent NOUN, the left one depends on the right one:
  - `nmod` 797;
  - `compound` 61;
  - `nmod:desc` 45;
  - `obj` 15;
  - `amod` 4.

  In EWT: `compound` 4750, `nmod:unmarked` 16, `nmod:poss` 15.
- `nmod:unmarked` in ParTUT — 194: some such modifiers already have the subtype, but not all.
- For proper nouns ParTUT uses `flat` (441) and `flat:foreign` (88).

**Examples.**
- `en_partut-ud-2` *Creative Commons Corporation is not a law firm…* — *law*: `nmod` → *firm*. In EWT — `compound`.
- `en_partut-ud-85` *The Cunha report on multiannual guidance programmes…* — *guidance*: `nmod` → *programmes*.

**In UD.**

| rule | ParTUT | CTeTex | EWT |
|---|---:|---:|---:|
| `tb.en.nn-compound`: adjacent noun modifier not `compound` | 862 / 974 | 314 / 726 | 19 / 5376 |

For LAS without subtypes, `nmod` versus `compound` are different labels. Merging ParTUT without a converter will teach the parser to put `nmod` on *law firm*.

**Sources.** Sanguinetti & Bosco 2014 (conversion of ParTUT to USD, CLiC-it 2014); Bosco et al. 2013, `W13-2308` (Italian treebanks → Stanford Dependencies); UD `docs/_en/dep/compound.md`; README `UD_English-ParTUT`.

<a id="structures"></a>

# ParTUT — possessives as DET, *by* without `obl:agent`, `flat:foreign`, *when* as `mark`

**Gist.** From the TUT scheme and early UD 2, ParTUT kept customs that EWT no longer has:
1. **Possessives *his, their, its, our, your*** — DET with XPOS `AP` ("possessive adjective") in `nmod:poss`, without `Case` and `Person`. Lemma *our* → *us*, *your* → *you*. In EWT — PRON with `Case=Gen|Person|Poss=Yes`, the lemma is the form itself (2.11).
2. **The passive agent *by X*** — plain `obl`: not a single `obl:agent`.
3. **Foreign names** — `flat:foreign` (88): *Cousin Pons*, *Cousine Bette*, *Les Parents Pauvres*, *ad hoc*. EWT removed `flat:foreign` in 2.13.
4. **Subordinate *when*** — SCONJ `mark` (65), *why* — SCONJ `mark` (4). EWT since 2.11 — ADV `advmod`.

**Conditions and exceptions.**
- Possessive DET in `nmod:poss`: *his* 243, *their* 116, *its* 101, *our* 60, *your* 50. Only 2 of them are PRON.
- No new subtypes: `advcl:relcl`, `nsubj:outer`, `cc:preconj`, `goeswith`, `list`, `reparandum` — 0. `det:predet` is present (17), `nmod:desc` is present (142), `nmod:unmarked` is present (194). So ParTUT received part of the 2.15–2.16 changes through mass edits.
- `obl:unmarked` — only 1: for temporal NPs ParTUT uses `nmod:unmarked` or plain `obl`.

**Examples.**
- `en_partut-ud-465` *…you will be responsible … determined by … provider* — *provider*: `obl`, not `obl:agent`.
- *his* in `nmod:poss`: DET/`AP`, FEATS `Poss=Yes|PronType=Prs`.

**In UD.**

| rule | ParTUT | EWT |
|---|---:|---:|
| `tb.en.poss-pron`: *my, your, his…* in `nmod:poss` not PRON | 615 / 617 | 1 / 3468 |
| `tb.en.obl-agent`: *by* agent not `obl:agent` | 141 / 157 | 16 / 411 |
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 72 / 155 | 0 / 1049 |
| `tb.en.pron-lemma`: pronoun lemmas not per EWT 2.11 | 31 / 129 | 9 / 5039 |
| `flat:foreign` (`grep`) | 88 | 0 |

**Sources.** EWT README v2.11 (possessive lemmas, #88), v2.13 (`obl:agent`, no `flat:foreign`); UD `docs/_en/pos/PRON.md`, `dep/obl-agent.md`; `docs/changes.md` No. 13 (foreign expressions); README `UD_English-ParTUT`.

<a id="xpos-tut"></a>

# ParTUT — XPOS from the Italian TUT/ISST set

**Gist.** ParTUT XPOS are Italian-grammar tags shared by all three languages of the treebank: `S` — noun, `E` — preposition, `V` — verb, `A` — adjective, `RD` — definite article, `RI` — indefinite, `SP` — proper noun, `FF`/`FS`/`FB` — punctuation, `VA`/`VM` — auxiliary/modal, `AP` — possessive adjective, `PE`/`PR`/`PD` — pronouns, `CS`/`CC` — conjunctions. A PTB tag cannot be recovered from it: `V` does not distinguish VB, VBD, VBZ, VBP; `S` — NN and NNS. In `mova` XPOS = PTB, so ParTUT XPOS is a separate layer.

**Conditions and exceptions.**
- The most frequent UPOS → XPOS pairs:
  - NOUN → `S` 10,571, ADP → `E` 5980, VERB → `V` 4722, ADJ → `A` 3740;
  - DET → `RD` 2958 / `RI` 973 / `AP` 634 / `DD` 444 / `DI` 232;
  - PUNCT → `FF` 2836 / `FS` 1959 / `FB` 797;
  - PROPN → `SP` 2230;
  - AUX → `VA` 973 / `VM` 758 / `V` 707: the copula *be* has had `V` rather than `VA` since 2.1.
- 73 words with XPOS «_».
- The CoNLL-U reader in `en` does not know these tags, so ParTUT does not train the PTB tagger (`Sentence::tagged()`).

**Examples.** *Shakespeare* — PROPN/`SP`; *'s* — PART/`PART`; *property* — NOUN/`S`.

**In UD.** Neither `ParTUT → mova` nor `mova → ParTUT` can be translated by rules:
- a PTB tag can still be derived from `V` + FEATS: `Tense=Past|VerbForm=Fin` → VBD etc.;
- back, `S`, `V`, `A` follow unambiguously from UPOS;
- `RD`/`RI`/`AP`/`DD`/`DI` — from `PronType` and `Definite`.

But the engine will not accept a tag outside PTB either in a condition or in an action: string XPOS is needed (`convert.md`).

**Sources.** README `UD_English-ParTUT`, changelog v2.1 («changed xpos of copulas (from VA to V)»); Bosco et al. 2000, `L00-1168` (TUT); digest the UD survey digest «english-banks».
