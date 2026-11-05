# Seeds: treebanks/en/pud

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# PUD — what these data are and which layers exist

**Gist.** The English part of Parallel Universal Dependencies for CoNLL 2017: 1000 sentences (21,180 words) from news (`n…`) and Wikipedia (`w…`). 750 sentences are originally English (`…01…`). 250 were translated from German, French, Italian and Spanish (`02`–`05`); from English they were further translated into other languages. Each sentence has counterparts in the PUD of other languages (`# parallel_id`). Annotation history:
1. Google annotated it following "universal" guidelines based on Stanford Dependencies.
2. Martin Popel automatically converted it to UD v2 (Udapi, `ud.Google2ud`).
3. Schuster, Reddy and Manning manually corrected UPOS and syntax.
4. Lemmas and FEATS were assigned by CoreNLP, without checking.

Test only. License CC BY-SA 3.0.

**Conditions and exceptions.**
- The README changelog ends at 2.6 ("No changes"). Yet the 2.18 data have subtypes and features of later versions: `nmod:unmarked` 111, `obl:unmarked` 37, `nmod:desc` 41, `advcl:relcl` 1, `ExtPos` 76, `PronType=Dem` on *there* (35 of 35). So the treebank received mass edits from the UD maintainers that are not recorded in the README.
- DEPS — enhanced UD, automatic since 2.2, unchecked. 7 empty nodes.
- 5 sentences still contain annotator notes `# Checktree: …` (*Not sure about this one*, *Ellipsis*), and one more `# notes`.

**Examples.** `n01002042` *The new spending is fueled by Clinton’s large bank account.*; `n01085008` *Fast forward to 2016 and this is increasingly worthy of attention.*

**In UD.**

| layer | state |
|---|---|
| XPOS | PTB, manual; no `ADD`, `LS`, `NFP` |
| FEATS | CoreNLP: agreement by form, no `Voice`, no `Case=Gen` (`feats-corenlp.md`) |
| lemmas | CoreNLP, unchecked |
| DEPS | present |
| MWT | 129 |

Our annotator on PUD: UPOS 94.42, XPOS 94.07, UFeats 87.55, LAS 77.02, MLAS 58.04 (`en/README.md`). UPOS and XPOS are almost as on EWT, UFeats is 8 points lower — CoreNLP customs.

**Sources.** README `UD_English-PUD` (from Google's README); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_pud/index.md`; Zeman et al. 2017, `K17-3001` (papers/z/ze/zeman-2017-conll-2017-shared-task-multilingual); McDonald et al. 2013 (Google universal treebanks, ACL 2013).

<a id="errors"></a>

# PUD — known annotation errors

**Gist.** PUD syntax and UPOS were corrected manually, so there are few errors there. Lemmas and FEATS are unchecked CoreNLP output. Their errors are mostly inherited from the model (`feats-corenlp.md`), not individual faults. The converter does not fix errors.

**Examples and numbers.**
- **Annotator notes in the data** — 5 lines `# Checktree: …` and one `# notes`. Doubtful sentences that were never resolved: *Not sure about this one*, *Ellipsis*, *how to represet "to record what would be another remarkable success"*.
- **Infinitive root without a subject** (`tb.en.imp-mood`) — 4: `n01085008` *Fast forward to 2016…* — an imperative with `VerbForm=Inf` without `Mood=Imp`.
- **PROPN without Number** (`tb.en.propn-number`) — 11 of 1719; **NOUN without Number** — 5.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 2 of 240.
- **Enhanced UD** was added automatically in 2.2 and not checked (README).

**In UD.** PUD is useful as an independent test of UPOS, XPOS and trees: they were corrected manually. UFeats and lemmas on it measure agreement with CoreNLP 2017, not with EWT 2.18, until there is a reverse converter (`convert.md`).

**Sources.** README `UD_English-PUD`; Zeman et al. 2017, `K17-3001`, sec. 2.4 (PUD: Udapi conversion, manual corrections); rules in `../seeds-overview.md`.

<a id="feats-corenlp"></a>

# PUD — FEATS from CoreNLP: agreement by form, no `Voice`, no `Case=Gen`, no `Mood=Imp`; *by* without `obl:agent`

**Gist.** Lemmas and FEATS in PUD were assigned by CoreNLP (≈ 2017) and have not been checked since. So they reflect EWT of those years, not 2.18:
- **Number/Person only where visible in the form:** VBZ, *am*, *was/were*. 940 of 1569 finite verbs with Tense lack them.
- **No `Voice=Pass`:** 272 of 272 participles with `aux:pass` lack it. EWT introduced it more widely in 2.13.
- **No `Case=Gen` on possessives:** 260 of 260 in `nmod:poss`. EWT added it in 2.11.
- **Not a single `Mood=Imp`:** 74 infinitive roots have `VerbForm=Inf`. Only 4 of them are real imperatives without a subject (*Fast forward to 2016…*). News and Wiki have few imperatives.
- **`Polarity=Pos`, `Style`, `Abbr=Yes`** — isolated or 0.

The syntax was corrected manually in 2017, so the old norm is there too: the ***by* agent** is `obl`, not `obl:agent`, in 66 of 75.

**Conditions and exceptions.**
- Mass edits by the maintainers (`data.md`) added `ExtPos`, `PronType=Dem` on *there*, `:unmarked`, `nmod:desc`, `Polarity=Neg` on *not* (74 of 74). But not agreement, `Voice` and `Case=Gen`.
- Subordinate *when* are mostly already ADV `advmod` (25). `mark` — only 4 (`tb.en.wh-advmod` 4 of 69).

**Examples.**
- `n01002042` *The new spending is fueled by Clinton’s large bank account.* — *fueled*: without `Voice=Pass`; *account*: `obl`, not `obl:agent`.
- `n01002017` *…in a break from his past rhetoric…* — *his*: without `Case=Gen`.
- `n01001011` *…wrote…* — `Mood=Ind|Tense=Past|VerbForm=Fin`, without Number/Person.

**In UD.**

| rule | PUD | EWT |
|---|---:|---:|
| `tb.en.fin-agree` | 940 / 1569 | 0 / 19 126 |
| `tb.en.part-voice` | 272 / 272 | 0 / 1643 |
| `tb.en.poss-gen` | 260 / 260 | 16 / 3689 |
| `tb.en.obl-agent` | 66 / 75 | 16 / 411 |
| `tb.en.imp-mood` | 4 / 74 | 38 / 3066 |
| `tb.en.wh-advmod` | 4 / 69 | 0 / 1049 |

**Sources.** README `UD_English-PUD` («Morphological features and lemmata were added automatically using Stanford CoreNLP»); EWT README v2.11 (Case), v2.13 (`Voice`, `obl:agent`); UD `docs/changes.md` No. 18.
