# Seeds: treebanks/en/gentle

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# GENTLE — what these data are and which layers exist

**Gist.** Genre Tests for Linguistic Evaluation — an out-of-domain test annotated with the same multilayer pipeline as GUM. 26 documents of eight unusual genres, ≈ 2100–2400 words each:
- dictionary entries;
- live esports commentary;
- legal documents;
- medical notes;
- poetry;
- mathematical proofs;
- course syllabi;
- threat letters.

In total 1334 sentences and 17,799 words. Test only. License CC BY-NC-SA 4.0.

**Conditions and exceptions.**
- The annotation is as in GUM: XPOS and lemmas manual, UPOS and FEATS from PTB and the graph, dependencies manual in UD. The GUM customs are inherited completely (`gum-family.md`).
- The genre is visible in `# newdoc id` (*GENTLE_poetry_…*). The genres differ a lot in annotation:
  - syllabi: `dep` 31, X 149;
  - dictionary: `dep` 23, X 64;
  - poetry: `dep` 0, X 1.
- X — mostly item numbers *1.*, *2.* with XPOS `LS` and relation `discourse` (230), as in EWT since 2.14.
- `Style=Arch` (29, poetry) and `Abbr=Yes` (90) — more often than elsewhere.

**Examples.** Poetry: `Style=Arch` on *thy* (7), *quoth* (5), *thee* (4), *hath* (3), *o'er* (3). Syllabus: *1.* — X/LS, `discourse`.

**In UD.** Layers — as in GUM: lemmas, XPOS (without `$`, `ADD`, `AFX`, `NFP`), FEATS, DEPS, MISC with Entity/Discourse. MWT — 180.

Our annotator on GENTLE: UPOS 88.47, XPOS 86.76, UFeats 87.30, LAS 67.94, MLAS 48.18 (`en/README.md`). By LAS this is the third result from the bottom, after CTeTex (61.49) and ATIS (63.01). The main cause is the genres, not the customs: the GUM customs affect hundreds of tokens, not thousands (`gum-family.md`).

**Sources.** README `UD_English-GENTLE`; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_gentle/index.md`; Aoyama et al. 2023, `2023.law-1.17` (papers/a/ao/aoyama-2023-gentle-genre-diverse-multilayer-challenge); digest the UD survey digest «english-banks», §1.11.

<a id="gum-family"></a>

# GENTLE — GUM customs in a small treebank

**Gist.** GENTLE was annotated with the GUM pipeline, so the differences from EWT are the same:
- the number of *you* and agreement with it;
- `Degree=Pos` on invariable adverbs;
- *such* without `Degree`;
- `obl` in fragments;
- `dep` where EWT has a specific label;
- XPOS without web tags.

The seeds with explanations are in `../gum/seeds.md`. Here — the GENTLE numbers.

**Conditions and exceptions.** Because of the genres, the share of some customs is larger than in GUM:
- `dep` — 96 per 17,799 words (0.54%; GUM 0.07%, EWT 0.002%). Of them on NOUN 52, PROPN 17, NUM 16. Most are in syllabi (31) and the dictionary (23): *t*, *s*, *webex*, *ipa*, *hours*, *email*, *thesaurus*.
- `obl` on a noun root without a copula — 11 of 19: a larger share than in GUM (56 of 207).

**Examples.** See `../gum/seeds.md`: the mechanism is the same.

**In UD.**

| rule | GENTLE | GUM | EWT |
|---|---:|---:|---:|
| `tb.atis.you-number`: *you* with Number | 153 / 153 | 2461 / 2461 | 0 / 2768 |
| `tb.gum.you-plur`: verb with *you* in the plural | 7 / 46 | 63 / 882 | 6 / 765 |
| `tb.en.adv-degree`: invariable adverb with `Degree` | 88 / 287 | 1146 / 4668 | 0 / 5076 |
| `tb.en.adj-degree`: ADJ without `Degree` (all — *such*) | 11 / 1228 | 145 / 16 637 | 0 / 16 789 |
| `tb.en.obl-fragment` | 11 / 19 | 56 / 207 | 12 / 196 |
| `dep` (`grep`) | 96 | 184 | 5 |
| `tb.gum.num-dep`: `dep` on NUM/X/SYM | 16 / 16 | 24 / 24 | 2 / 2 |
| `tb.gum.you-dep` | 2 / 2 | 56 / 56 | 0 / 0 |
| `tb.en.imp-mood`: infinitive root without `Mood=Imp` | 23 / 207 | 64 / 2472 | 38 / 3066 |

The remaining general counters on GENTLE are zeros or ones. So the GUM converter fits without changes.

**Sources.** README `UD_English-GENTLE`; `../gum/seeds.md` (you-number, adv-degree, obl-fragment, dep, xpos-web); Aoyama et al. 2023, `2023.law-1.17`.
