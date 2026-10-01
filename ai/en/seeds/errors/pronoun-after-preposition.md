# After a preposition — accusative: between you and me

**Gist.** A pronoun after a preposition is in the accusative (*for me*, *with them*). The same applies to the second member of a coordinated phrase: *between you and me*. The hypercorrection ✗ *between you and I* is an old error of educated native speakers: Fowler mentions it as "not uncommon" together with *let you & I try*. ERRANT classifies it as PRON.

**Conditions and exceptions.** The rule has two parts:
- a nominative pronoun with its own preposition;
- a pronoun conjunct in a phrase with a preposition, since the preposition hangs on the first conjunct.

The rule also catches a wrong Case=Nom on *it* in the EWT gold.

**Examples.**
- ✗ *between you and I* → case(you, between), conj(you, I), I: Case=Nom.
- ✓ *between you and me*.

**Check against gold.** EWT 2.18:
- 7 — Case=Nom on *it* under a preposition (feature errors in the gold);
- 1 — *between you and I*.

**Sources.** Fowler MEU 1926, ME (p. 358: "between you & I, let you & I try, are not uncommon"); UD `_en/feat/Case.md`; `P17-1074` (PRON).

```rule
rule: en.errors.nominative-after-preposition
what: a nominative personal pronoun has its own preposition — after a preposition the accusative is used
match: p[upos=PRON, feats.Case=Nom, feats.PronType=Prs]; c[rel=case, upos=ADP, head=p]
require: not p[feats.Case=Nom]
severity: warn
source: UD _en/feat/Case.md; P17-1074 (PRON)
```

```rule
rule: en.errors.nominative-conjunct-after-preposition
what: a nominative pronoun is a conjunct of a phrase with a preposition (between you and I)
match: q[]; c[rel=case, upos=ADP, head=q, before=q]; p[upos=PRON, feats.Case=Nom, feats.PronType=Prs, rel=conj, head=q]
require: not p[feats.Case=Nom]
severity: warn
source: Fowler MEU 1926, ME (p. 358); P17-1074 (PRON)
```
