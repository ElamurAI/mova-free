# Reciprocal pronouns each other, one another

**Gist.** *Each other* and *one another* denote a mutual action of several participants: *They love each other.* It is an inseparable combination that behaves like a single object pronoun.
**Conditions and exceptions.** A plural subject or several subjects are needed. The genitive is *each other's* (*each other's houses*). Historically *each* was the subject and *other* the object (*They each loved the other*; Poutsma XL §37), but now both words stand together and act as one.
**Examples.** *We help each other.*; *They looked at one another.*; *each other's names*.
**In UD.** The first word is the head with `ExtPos=PRON|PronType=Rcp` (*each* — DET DT, *one* — PRON CD); the second (*other* ADJ JJ, *another* DET) is `fixed`. The whole combination takes the relation of its role (`obj`, `obl`, `nmod:poss`). EWT 2.18: *each other* — 17 times, *one another* — 2, all like this.
**Sources.** Poutsma GLME vol. 4, Ch. XL §37–39 (pp. 387–389 = book pp. 1067–1069); https://universaldependencies.org/en/feat/PronType.html (Rcp), https://universaldependencies.org/en/feat/ExtPos.html, https://universaldependencies.org/u/dep/fixed.html.

```rule
rule: en.nominal.reciprocal-fixed
what: each other / one another — the first word has PronType=Rcp and ExtPos=PRON, the second is fixed
match: e[lemma=each|one]; o[lemma=other|another, rel=fixed, head=e]
require: e[feats.PronType=Rcp, feats.ExtPos=PRON]
severity: error
source: Poutsma GLME IV Ch. XL §37; UD en PronType=Rcp, fixed
```
