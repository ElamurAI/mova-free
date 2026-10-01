# Possessives — nmod:poss, always before the head

**Gist.** In English UD possessive pronouns (*my, your, his, their, whose*) and the possessive case (*John's*) are `nmod:poss`. A possessive pronoun is PRON with Poss=Yes. The possessive always stands before the possessed. English UD does not use the label `det:poss` (it exists in other languages), but LLMs produce it: Opus on high gave `det:poss` in 3 of 20 calibration sentences.

**Conditions and exceptions.**
- A possessive as a conjunct (*his and her*) is `conj`.
- Independent *mine, hers* have other roles (nsubj, obj).
- *a friend of mine* is `nmod` with case *of*.

**Examples.**
- *my office* → nmod:poss(office, my).
- *the president's office* → nmod:poss(office, president), case(president, 's).
- *whose car* → nmod:poss(car, whose).

**Check against gold.** EWT 2.18:
- PRP$ or WP$ not as `nmod:poss` — 16 of 3705 (conjuncts, fragments);
- `nmod:poss` after the head — 0 of 4466.

GUM — 29 and 1.

**Sources.** UD `_en/dep/nmod-poss.md`, `_en/dep/nmod.md`, `_en/feat/Poss.md`; english-banks §1.3 (SD poss → nmod:poss), §2 (PRP$ → PRON with Poss=Yes) (measurement 25.09.2026: det:poss instead of nmod:poss).

The rule is `en.nominal.prp-poss-rel` in `nominal/poss-pronouns.md`.

The rule is `en.nominal.poss-before-head` in `nominal/nmod-poss.md`.
