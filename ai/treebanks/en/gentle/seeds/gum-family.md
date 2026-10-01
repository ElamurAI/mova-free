# GENTLE — GUM customs in a small treebank

**Gist.** GENTLE was annotated with the GUM pipeline, so the differences from EWT are the same:
- the number of *you* and agreement with it;
- `Degree=Pos` on invariable adverbs;
- *such* without `Degree`;
- `obl` in fragments;
- `dep` where EWT has a specific label;
- XPOS without web tags.

The seeds with explanations are in `../../gum/seeds/`. Here — the GENTLE numbers.

**Conditions and exceptions.** Because of the genres, the share of some customs is larger than in GUM:
- `dep` — 96 per 17,799 words (0.54%; GUM 0.07%, EWT 0.002%). Of them on NOUN 52, PROPN 17, NUM 16. Most are in syllabi (31) and the dictionary (23): *t*, *s*, *webex*, *ipa*, *hours*, *email*, *thesaurus*.
- `obl` on a noun root without a copula — 11 of 19: a larger share than in GUM (56 of 207).

**Examples.** See `../../gum/seeds/*.md`: the mechanism is the same.

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

**Sources.** README `UD_English-GENTLE`; `../../gum/seeds/` (you-number, adv-degree, obl-fragment, dep, xpos-web); Aoyama et al. 2023, `2023.law-1.17`.
