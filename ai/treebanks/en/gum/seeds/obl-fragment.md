# GUM — `obl` in fragments without a verb

**Gist.** In verbless fragments (*What about you?*, *Good morning to all.*, *Order out of chaos.*) GUM attaches the prepositional phrase to the noun root as `obl`, as if it were a predicate. EWT uses `nmod` in such fragments, because the head is a noun and there is no copula. UD 2.18 clarified the `nmod`/`obl` boundary, and the validator now catches `obl-should-be-nmod` (UD docs/changes.md` No. 17).

**Conditions and exceptions.**
- We count `obl` on a NOUN/PROPN/PRON/NUM root without `cop`: GUM 56 of 207, EWT 12 of 196.
- On all noun heads without a copula, not only roots: GUM 105 of 335, EWT 17 of 291.
- The clearest marker is *What about X?*: in GUM X is always `obl` (13 of 13), in EWT always `nmod` (7 of 7).
- The GUM family: GENTLE 11 of 19, GUMReddit 2 of 6. CHILDES also has many such cases: 49 of 106.

**Examples.**
- `GUM_podcast_bangladesh-13` *What about you?* — *you*: `obl` → *What*.
- `GUM_court_negligence-1` *Good morning to all.* — *all*: `obl` → *morning*.
- `GUM_essay_system-37` *Order out of chaos.* — *chaos*: `obl` → *Order*.

**In UD.**

| rule | GUM | EWT | GENTLE | CHILDES |
|---|---:|---:|---:|---:|
| `tb.en.obl-fragment`: `obl` on a noun root without `cop` | 56 / 207 | 12 / 196 | 11 / 19 | 49 / 106 |
| `tb.gum.what-about`: *What about X* — X not `nmod` | 13 / 13 | 0 / 7 | — | — |

For LAS without subtypes, `obl` versus `nmod` are different labels, hence an error. Zeldes & Schneider 2023 call the `nmod`/`obl` confusion the most frequent error of cross-corpus EWT↔GUM parsing, although most of it is PP attachment, not custom.

**Sources.** UD docs/changes.md` No. 17 (`nmod` and `obl`, 2.18); https://universaldependencies.org/u/dep/obl.html, https://universaldependencies.org/u/dep/nmod.html; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 and appendix (confusion matrices).

```rule
rule: tb.gum.what-about
what: What about X? — in GUM X as obl, in EWT as nmod
match: h[form=what, head=0]; o[head=h, after=h]; c[form=about, rel=case, head=o]
require: o[rel=nmod]
severity: warn
source: EWT 2.18; UD 2.18 nmod/obl; counter
```
