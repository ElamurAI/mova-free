# Possessive pronouns: dependent and independent

**Gist.** A possessive pronoun has two forms. The dependent (adnominal) one stands before a noun: *my book, your car, her idea*. The independent (absolute) one replaces a whole noun phrase: *the book is **mine**, **yours** is better*. The independent form is built from the dependent one: by adding *-s* (*yours, hers, ours, theirs*) or *-n* (*mine*, archaic *thine*). *His* and *its* have one form for both roles.

**Conditions and exceptions.**
- *Her* is both possessive (*her book*, PRP$) and objective (*I saw her*, PRP); the position before a noun tells them apart.
- *His* can be dependent (*his book*, PRP$) and independent (*the book is his*, PRP without case).
- *Its* (possessive) ≠ *it's* (= *it is/has*): the latter is tokenized as *it* + *'s*.
- Independent forms are written without an apostrophe: *yours*, not *your's*.
- *Whose* is a possessive interrogative/relative pronoun (WP$), see `pron-wh`.

**Examples.** ***My** car is older than **yours**. The idea was **hers**. The cat licked **its** paw.*

**In UD.**
- Dependent form: XPOS PRP$; `Case=Gen|Poss=Yes|PronType=Prs` with person, number and gender; lemma — the form itself (*my, your, her, its, our, their*).
- Independent form: XPOS PRP; `Poss=Yes|PronType=Prs` **without** `Case`; lemma — the dependent form (*mine → my, yours → your, hers → her, ours → our, theirs → their*).
- The relation of the dependent form to the noun is `nmod:poss`.

**Sources.** https://universaldependencies.org/en/feat/Poss.html, `feat/Case.md` (Gen only for dependent forms), https://universaldependencies.org/en/pos/PRON.html (table: independent possessive [my] etc.); Santorini 1990, §2, p. 4 (PRP$: *my, your, his, her, its, one’s, our, their*) and §4.1, p. 20 (*That’s hers* — PRP); Sweet NEG I §1102–1103 (text-1, p. 375–376: conjoint *my, thy, his, its, her, our, your, their* and absolute *mine, thine, his, hers, ours, yours, theirs*; *mine host*), §1022 (p. 352: *its, hers, yours* without apostrophe, but *one's*); Jespersen MEG II 16.21₁–16.21₂ (text-2, p. 431–433: *my/mine*), VI 20.3 (text-5, p. 361–362: *-n* in *mine, none*; dialectal *hisn, hern*), VI 16.1₃ (p. 270: *her's* — not standard); Whitney §165, §205–207 (text-1, p. 89, 108–109: *a friend of ours*); Mätzner I, p. 296–297 (text-1, p. 314–315: *its* since Shakespeare's time).

```rule
rule: en.morph.prp-dollar-feats
what: PRP$ — dependent possessive pronoun: Case=Gen, Poss=Yes
match: p[xpos=PRP$]
require: p[upos=PRON, feats.Case=Gen, feats.Poss=Yes, feats.PronType=Prs]
severity: error
source: UD en feat/Poss, feat/Case (Gen); Santorini 1990, PRP$
```

```rule
rule: en.morph.poss-independent
what: mine, yours, hers, ours, theirs — independent: PRP, Poss=Yes without Case, lemma is the dependent form
match: p[upos=PRON, form=mine|yours|hers|ours|theirs, !feats.Typo]
require: p[xpos=PRP, feats.Poss=Yes, !feats.Case, lemma=my|your|her|our|their]
severity: error
source: UD en pos/PRON (independent possessive), feat/Case (no Case for mine)
```

```rule
rule: en.morph.poss-her-dependent
what: her tagged PRP$ — possessive, lemma her
match: p[upos=PRON, xpos=PRP$, form=her, !feats.Typo]
require: p[lemma=her, feats.Case=Gen, feats.Poss=Yes]
severity: error
source: UD en pos/PRON, feat/Case (her ambiguous Acc/Gen)
```
