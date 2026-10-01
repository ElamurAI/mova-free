# GUM — `Degree=Pos` on invariable adverbs, and *such* without Degree

**Gist.** Two opposite small details with one consequence: GUM and EWT UFeats diverge on frequent words.
1. GUM puts `Degree=Pos` on adverbs that have no degrees of comparison: *really, back, still, again, too, therefore, down, above*. EWT puts `Degree` only on adverbs with degrees: *well, far, soon, long, hard, early, late, little, close, high, fast, badly, low* (all ADV lemmas with `Degree=Pos` in EWT) and on RBR/RBS.
2. *such* as ADJ has no `Degree` in GUM; in EWT it has `Degree=Pos`.

**Conditions and exceptions.**
- ADV with `Degree`: GUM 6033 of 12,206, EWT 997 of 12,595.
- Of the list of 24 invariable adverbs (*really, back, still, again, too, here, there, now, then, just, so, very…*), `Degree` is on:
  - GUM — 1146 of 4668;
  - GENTLE — 88 of 287;
  - EWT — 0 of 5076.
- GUM does not put `Degree=Pos` on *so, just, very, also, even, only* — the same words as EWT. The boundary between "has degrees" and "does not" is different in GUM, not absent.
- *such* ADJ without `Degree` — 145 (all ADJ without Degree in GUM). In EWT *such* ADJ — 88, all with `Degree=Pos`.

**Examples.**
- *really* RB: GUM — `Degree=Pos` 234 times; EWT — without Degree (196 of 199).
- `GUM_academic_exposure-4` *However, it is not enough to have attained such native-like levels.* — *such*: ADJ, FEATS «_»; in EWT — `Degree=Pos`.

**In UD.**

| rule | GUM | EWT | GENTLE |
|---|---:|---:|---:|
| `tb.en.adv-degree`: invariable adverb with `Degree` | 1146 / 4668 | 0 / 5076 | 88 / 287 |
| `tb.en.adj-degree`: ADJ without `Degree` (here it is *such*) | 145 / 16 637 | 0 / 16 789 | 11 / 1228 |

**Sources.** https://universaldependencies.org/en/feat/Degree.html; EWT 2.18 (data: ADV with `Degree=Pos` — only 13 lemmas); README `UD_English-GUM` (FEATS from PTB and the graph, «Many updates to UPOS and FEATS consistency with EWT», 2022-10-21).
