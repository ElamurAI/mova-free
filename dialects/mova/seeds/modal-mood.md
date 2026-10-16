# Modal MD without `Mood` — proposal

**Proposal: wait** for the decision on #1118 and #1155. Until then keep the EWT custom: MD — `VerbForm=Fin` without `Mood`, `Number` and `Person`.

**Options and why not now.**
- **`Mood=Pot`/`Nec`/`Cnd` in FEATS**, as the universal `Mood.md` advises and as auxiliary functions are recorded in `data.json` (*can* `Mood=Pot`, *must* `Mood=Nec`, *would* `Mood=Cnd`):
  - informative;
  - but the en FEATS registry allows only `Imp|Ind|Sub`, so export to 2.18 and to `ud-next` would be invalid;
  - #1118 leans toward treating this as lexical meaning, not a paradigm cell.
- **`Mood=Ind` on modals:** would remove the validator warning, but as a claim of "indicative mood" for *might*, *would* it is false.
- **Remove `VerbForm=Fin`** (#1233: an "inherent" feature) — also undecided.

**Why this does no harm.** The modal meaning in `mova` is visible from the lemma anyway: the auxiliary registry has the function of each of the 16 lemmas. A clause-level feature ("possibility", "necessity") for RAG can be derived from the lemma without writing it into FEATS.

**Evidence from the data.** EWT 2.18: MD with `VerbForm=Fin` without `Mood` — 4049. The same in GUM (2956), GENTLE, PUD and LittlePrince. The validator gives a `verbform-fin-without-mood` warning for each such token, but not an error.

**When to come back.** When #1118 is closed or when `_en/feat/Mood.md` changes in the snapshot. The snapshot-vs-release comparison should be repeated before 01.11.2026.

**Origin.** `dialects/ud-2.18/seeds/modal-mood.md`; #1118, #1155, #1233.

Mova decides.
