# Possessive pronouns my, your, his, her, its, our, their

**Gist.** The prenominal possessive pronouns (Poutsma: "conjoint") — *my, your, his, her, its, our, their* — are the genitive of the personal pronouns. They stand only before a noun and do not combine with an article (*a friend of mine*, not *a my friend*).
**Conditions and exceptions.** *His* is both prenominal and independent (*The car is his*). *Her* is a homonym: possessive (*her book*, PRP$) and objective (*I saw her*, PRP). *Its* ≠ *it's*. A possessive before a gerund is its subject (*I object to his coming*).
**Examples.** *my book*; *their house*; *I object to his coming.*
**In UD.** PRON, XPOS PRP$, `Case=Gen|Poss=Yes|PronType=Prs` + Person/Number/Gender; relation `nmod:poss`, with a gerund `nsubj`. The lemma is the form itself (*my, your, his, her, its, our, their*). EWT 2.18: 3 673 PRP$ in `nmod:poss`.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIII §1–2, §7 (p. 102 = book p. 782); Poutsma GLME vol. 4, Ch. XXXII §2 (p. 25); https://universaldependencies.org/en/feat/Poss.html, `dep/nmod-poss.md`.

```rule
rule: en.nominal.prp-poss-feats
what: PRP$ is a possessive personal pronoun: PRON, Case=Gen, Poss=Yes, PronType=Prs
match: p[xpos=PRP$]
require: p[upos=PRON, feats.Case=Gen, feats.Poss=Yes, feats.PronType=Prs]
severity: error
source: Poutsma GLME IV Ch. XXXIII §1; UD en Poss, Case
```

```rule
rule: en.nominal.prp-poss-rel
what: PRP$ and WP$ attach as nmod:poss (or gerund subject, or conjunct); det:poss is not used in English
match: p[xpos=PRP$|WP$]
require: p[rel=nmod:poss|nsubj|nsubj:pass|conj]
severity: warn
source: Poutsma GLME IV Ch. XXXIII §7; UD en nmod:poss; UD _en/dep/nmod-poss.md (det:poss in Opus output)
```
