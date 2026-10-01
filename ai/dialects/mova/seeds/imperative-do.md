# Main verb with imperative *do* — `Mood=Imp|VerbForm=Fin` — proposal

**Proposal: accept** the EWT custom, not the rule of `_en/feat/VerbForm.md` (`dialects/ud-2.18/seeds/imperative-do.md`). Align the 8 EWT exceptions with `VerbForm=Inf` with the EWT → `mova` converter.

**What exactly in `mova`.** In an imperative sentence with *do* (*Don't worry*, *Do not hesitate*, *DO NOT GO HERE*) both verbs — *do* and the main VB — have `Mood=Imp|VerbForm=Fin`.

**Why.**
- **More consistent.** The main verb of an imperative sentence becomes `Imp` regardless of whether *do* is present: *Go!* and *Don't go!* are annotated the same way. By the letter of `VerbForm.md`, *go* in *Don't go* would be `Inf`, and the mood feature would sit only on the function word.
- **More informative.** The sentence type (directive) is visible on the lexical verb, which is the one taking part in the fact. For RAG and `kg` this is a clause-level feature (, section 4, item 4).
- **Matches most of the gold data:** 98 of 106.
- **Lossless into the standard.** The literal `VerbForm.md` form is obtained deterministically: a VB with `aux` *do* that has `Mood=Imp` → `VerbForm=Inf`, remove `Mood`. Export to "EWT 2.18" diverges from gold only in the 8 exceptions.

**Evidence from the data** (EWT 2.18, engine): `Mood=Imp|VerbForm=Fin` — 98, `VerbForm=Inf` — 8 (*Don't cling*, *Don't go!*, *don't let that fool you*, *Do not go there*…). Separately: *Don't* with `Mood=Ind` in imperative sentences — 7; these are EWT errors (report `verbal.md`).

**Rule for the EWT → `mova` converter** (proposal for `treebanks/en/ewt/convert.md`, the `convert` language from `train/dialects-and-converters.md`):

```convert
rule: ewt.imperative-do-main
what: main VB with imperative do — Mood=Imp|VerbForm=Fin, as in 98 of 106 EWT cases
from: ewt
to: mova
match: v[xpos=VB, feats.VerbForm=Inf]; d[lemma=do, rel=aux, head=v, feats.Mood=Imp]
set: v[feats+=VerbForm=Fin; feats+=Mood=Imp]
source: EWT 2.18; dialects/mova/seeds/imperative-do.md
```

Checked with the engine (`en convert-check`, 26.09): on EWT 2.18 the rule changes exactly 8 words, FEATS `VerbForm=Inf` → `Mood=Imp|VerbForm=Fin`. There is no reverse rule: the 8 exceptions are not visible from the tree, so the reverse-run report has a separate line for these 8 tokens.

**Check.** Seed `en/seeds/verbal/imperative.md`: `en.verbal.imperative-do-main-verb` (warn) follows EWT, `vb-aux-infinitive` excludes imperative *do*. Counter `ud218.imperative-do-inf`.

**Origin.** `dialects/ud-2.18/seeds/imperative-do.md`; `2.18:https://universaldependencies.org/en/feat/VerbForm.html:12`, `:21`.

Mova decides.
