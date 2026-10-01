# GUMReddit — GUM customs measured without words

**Gist.** GUMReddit customs are the same as in GUM (`../../gum/seeds/`). But most counter rules rely on the form or lemma (*you*, *such*, *what about*), and GUMReddit does not have them. So here there is only what is visible from UPOS, XPOS, FEATS and the tree.

**Conditions and exceptions.**
- **The number of *you*:** a 2nd person personal pronoun (PRON, `Person=2`, `PronType=Prs`, neither possessive nor reflexive) has Number in 210 of 210 cases. In GUM — 2468 of 2468, in EWT — 4 of 2767. By features: `Number=Sing` 203, `Plur` 6.
- **`Degree` on RB:** 502 of 907 ADV/RB have `Degree`. GUM — 5359 of 10,228, EWT — 482 of 10,980.
- **`obl` on a noun root without a copula:** 2 of 6.
- **`dep`:** 12 — PRON 6, NUM 3, NOUN 2, SYM 1. PRON here is most likely *you* in *you guys*, as in GUM (`../../gum/seeds/dep.md`).
- **XPOS** without `ADD`, `NFP`, `AFX`, as in GUM (`../../gum/seeds/xpos-web.md`).

**Examples.** Without text there are no examples. The mechanism is as in `../../gum/seeds/*.md`.

**In UD.** The rules in the text are feature-based variants of the GUM rules:
- `p[upos=PRON, feats.Person=2, feats.PronType=Prs, !feats.Poss, !feats.Reflex]` → `not p[feats.Number]`;
- `a[upos=ADV, xpos=RB]` → `not a[feats.Degree]`.

They are also counted on EWT, so they serve as a negative control: EWT 4 and 482, GUMReddit 210 and 502.

**Sources.** README `UD_English-GUMReddit`; `../../gum/seeds/`.
