# LinES — pronouns and nominal features: *you* without Case, possessives with old lemmas, no `NumForm` and partly no `Polarity`

**Gist.** The features and lemmas of LinES nouns and pronouns follow old EWT rules (before 2.11–2.15) and have gaps:
- *you* has no `Case` (811 of 811). EWT sets `Nom`/`Acc` by role.
- Possessives: 257 of 1886 `nmod:poss` pronouns lack `Case=Gen`. Among them *his* with lemma *he* (136), *my* → *I* (45), *their* → *they* (37). EWT since 2.11 (docs#517): lemma *his* → *his*, *my* → *my*, `Case=Gen`.
- NUM without `NumForm` — 691 of 691. EWT since 2.13 sets `NumForm=Digit|Word|Roman`.
- *not/n't* without `Polarity=Neg` — 98 of 872. EWT since 2.15 sets `Polarity=Neg` everywhere.
- `fixed` heads without `ExtPos` — 63 of 293.
- NOUN without `Number` — 469, although the XPOS `SG-NOM`/`PL-NOM` shows the number: *garden* 17, *smile* 15, *sir* 15, *jews* 15.
- *an* with lemma *an* — 31 of 357.
- Relative pronouns in `acl:relcl` without `PronType=Rel` — 314 of 513: *that* 258 (no PronType), *which* 36, *who* 17, *whom* 3 (with `PronType=Int`).

**Conditions and exceptions.**
- The other possessives already follow the new rule: *his* → *his* 670, *my* → *my* 364. LinES mixes two norms.
- *her*/*its* in `nmod:poss` have `Case=Acc`/`Case=Nom` and lack `Poss=Yes` — 32 and 22 times. These are feature errors, not a custom (`errors.md`).

**Examples.** `en_lines-ud-dev-doc3-3538` *…a middle-aged woman whose face is so hot…* — *whose*: `nmod:poss` without `Case=Gen`. `en_lines-ud-dev-doc9-5290` *…that did not seem wholly natural…* — *not*: without `Polarity=Neg`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.pron-case`: personal pronoun without `Case` | 811 / 6554 | 0 / 14 842 |
| `tb.en.poss-gen`: `nmod:poss` pronoun without `Case=Gen` | 257 / 1886 | 16 / 3689 |
| `tb.en.num-feats`: NUM without `NumType`/`NumForm` | 691 / 691 | 15 / 5051 |
| `tb.en.not-polarity` | 98 / 872 | 0 / 2077 |
| `tb.en.fixed-extpos` | 63 / 293 | 0 / 586 |
| `tb.en.noun-number`: NOUN without `Number` | 469 / 18 955 | 0 / 43 084 |
| `tb.en.rel-prontype`: relative without `PronType=Rel` | 314 / 513 | 1 / 1238 |
| `tb.en.pron-lemma` | 8 / 2489 | 9 / 5039 |

`tb.en.pron-lemma` is small because the rule accepts any lemma from its list, and *he* is in it too: *his* → *he* passes. The mixing of lemmas is visible with `grep`.

**Sources.** EWT README v2.11 (docs#517: pronoun lemmas and features), v2.13 (NumForm), v2.15 (Polarity, ExtPos); https://universaldependencies.org/en/pos/PRON.html, `feat/Case.md`; `data/raw/ud-docs/docs/treebanks/en_lines/index.md`.
