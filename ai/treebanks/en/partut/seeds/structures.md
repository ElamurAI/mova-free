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

**Sources.** EWT README v2.11 (possessive lemmas, #88), v2.13 (`obl:agent`, no `flat:foreign`); https://universaldependencies.org/en/pos/PRON.html, `dep/obl-agent.md`; UD docs/changes.md` No. 13 (foreign expressions); README `UD_English-ParTUT`.
