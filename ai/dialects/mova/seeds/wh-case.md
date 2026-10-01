# *who, whom, whose*: case and the lemma *who* — proposal

**Proposal: accept** `ud-next` (`dialects/ud-next/seeds/en-wh-case.md`).

**What exactly in `mova`.**
- lemma *who* for *whom*, *whoever* for *whomever*;
- *who*/*whoever*: `Case=Nom` or `Case=Acc` by syntactic function (rules `mova.ud-next.who-*` in `dialects/ud-next/convert.md`); *whom*/*whomever* — always `Acc`, because the form wins;
- *whose* with a noun (`nmod:poss`) — `Case=Gen|Poss=Yes`; standalone — `Poss=Yes`.

**Why.**
- **More informative.** Case shows the role of *who* in a relative clause or question: *the man **who** I saw* (`Acc`) versus *the man **who** saw me* (`Nom`). For `annot` and RAG this is the role of the "gap" in the relative clause, one of the most valuable clause-level features (, section 4, item 4).
- **More consistent.**
  - Personal pronouns in EWT already have `Case` by position (*you* `Nom` 2096 / `Acc` 651).
  - *Who* pronouns are the only pronouns with case but without `Case`.
  - GUM, GENTLE, LinES and ParTUT already write the lemma *who* for *whom*; EWT, PUD and LittlePrince do not.
- **Lossless into the 2.18 standard.** The reverse rules are deterministic (`ud-next.who-case-drop`, `ud-next.whose-case-drop`, `ud-next.whom-lemma`): drop `Case` and restore the lemma from the form. A round trip of EWT 2.18 gives identity: 24 lemmas, 460 features.
- **The registry** already allows `PRON Case=Nom|Acc|Gen` and `Poss=Yes`. Export to 2.18 and to `ud-next` is valid.

**Evidence from the data** (EWT 2.18, engine):
- *whom* with lemma *whom* — 23, *whomever* — 1;
- *who* pronouns without `Case` — 447 of 447;
- *whose* without `Gen` — 13, all with a noun;
- expected values after the rules: `Acc` 37, `Nom` 410, `Gen` 13 (`dialects/ud-next/convert.md`).

**Risks.**
- `Case` for *who* is taken from the basic function, but in 2 free relatives and in one predicative case — from the position inside the subordinate clause (edge cases are described in `convert.md`).
- Whether EWT 2.19 will do this is not visible from the data (there is no `dev` in the snapshot). If it does not, export to "EWT 2.19" will go through the same reverse rules.

**Check.**
- Gates `udnext.whom-who-acc`, `udnext.who-case`, `udnext.whose-dependent` (error);
- hints `udnext.who-object-acc`, `udnext.who-subject-nom`, `udnext.whose-independent` (warn);
- the existing `en.morph.whom-acc` (`en/seeds/morph/pron-wh.md`).

**Origin.** `dialects/ud-next/seeds/en-wh-case.md`, `dialects/ud-2.18/seeds/wh-pronouns.md`, #517.

Mova decides.
