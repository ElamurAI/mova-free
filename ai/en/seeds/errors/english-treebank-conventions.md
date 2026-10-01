# Convention differences between the English UD treebanks

**Gist.** UD 2.18 has thirteen English treebanks, and they are not the same. The differences are of two kinds: some treebanks lack some columns, others annotate the same phenomena differently. A rule built on EWT may give false alarms on another treebank. The error is then not in the data but in the convention. Therefore: (1) the expert system checks annotation in the EWT 2.18 style; (2) on other treebanks, violations must first be checked against this list.

**Missing columns.**
- ESLSpok has no lemmas. Rules with `lemma=` give nothing but false alarms there: `cop-be` — 651 of 651.
- CHILDES has no FEATS.
- ATIS, CTeTex, LittlePrince have no XPOS.
- CTeTex, ESLSpok, LittlePrince have no multiword tokens.

**Obsolete labels.** ATIS, ESLSpok, CHILDES, LittlePrince still have `:tmod`/`:npmod` (ESLSpok — 151), while EWT, GUM, GENTLE, LinES, ParTUT and PUD have moved to `:unmarked` and `:desc` (see `tmod-npmod-unmarked.md`).

**EWT vs GUM.**

| phenomenon | EWT | GUM |
|---|---|---|
| *Sri Lanka, Hong Kong* | `compound`, head on the right | `flat`, head on the left |
| *Page 3* before 2.11 | `nummod` | `dep` |
| *Marvel Consultants, Inc.* | head *Inc.* | head *Consultants* (since 2.15 — `nmod:desc`) |
| ADJ in names (*Islamist officers*) | often `compound` | `amod` |
| prepositional phrase on a fragment root (*Good morning to all*) | `nmod` (15 exceptions) | `obl` (47 on the root) |
| `dep` | 5 | 184 |
| `iobj` outside the double-object class | 1 | 17 (*presented me*) |
| `orphan` outside a conjunct | 0 | 6 (more aggressive ellipsis since 2.10) |

**Alignment across versions.**
- 2.7–2.8: MWT for clitics; hyphen as a separate token (HYPH); ADJ and VERB in names.
- 2.10: proper names — from `flat` to transparent syntax.
- 2.11: `nsubj:outer`, relative clauses, clefts, lemma policy for possessives.
- 2.15: `:unmarked`, `nmod:desc`.

A joint EWT+GUM model in 2.11 trails a single-corpus model by only 0.32 LAS. So few differences are left, and most parser errors are genuine attachment errors (obl/nmod).

**For our engine.** A violation count on GUM noticeably higher than on EWT signals a convention, not a rule error. In the seeds of this section such places are marked in "Check against gold".

**Sources.** `2023.udw-1.7` (Zeldes, Schneider: §3.1–3.2, appendix); `2021.udw-1.14` (Schneider, Zeldes) §1.4–1.5, §2 ("Divergences within UD English"); measurement with `en expert-check` on EWT, GUM and ESLSpok 2.18 (26.09.2026).
