# Determiner and noun — same number

**Gist.** *this/that* and *a/an* combine with the singular, *these/those* with the plural: *this book, these books*. BLiMP tests this as a separate category of 8 paradigms, including with an adjective in between and with irregular plurals (*man/men*). In learner texts these are DET and NOUN:NUM errors, together about 15 % of edits. For annotation, a mismatch means an error in the text or a wrong Number feature on the noun.

**Conditions and exceptions.**
- The indefinite article with a quantity phrase: *a few days*, *a good few*, *a full 20 minutes*, *an estimated 850 people*, *a further 45*.
- *those* as a pronoun (*those who*) is not `det`.
- The lemma of *those* is *that*, so the rule checks the form.

**Examples.**
- ✗ *this times*
- ✗ *these question*
- ✗ *a monthly breakfasts*
- ✗ *an SAP identification numbers*
- ✓ *a few weeks*

**In UD.** det(n, this|that) — n is not Number=Plur; det(n, these|those) — n is not Sing; det(n, a) with Plur — only with a quantity modifier.

**Check against gold.** EWT 2.18:
- this/that + plural — 1094/3;
- these/those + singular — 280/2;
- a + plural — 116/8. Among the violations there are errors in the gold too: *a species* with Number=Plur, *an old soldiers' home* with det on *soldiers*.

**Sources.** `2020.tacl-1.25` (BLiMP: DET-NOUN AGR); `P17-1074` (ERRANT: DET, NOUN:NUM); `W19-4406` (table 4: DET 11.25 %, NOUN:NUM 4.05 % of W&I train edits); Fowler MEU 1926, A, AN §2 (combinations with *few*, p. 13).

The rule is `en.nominal.dem-number` in `nominal/dem-number.md`.

```rule
rule: en.errors.a-singular
what: indefinite article with a plural without a quantity word (a few, a good, a full…)
match: n[feats.Number=Plur]; d[rel=det, lemma=a, head=n]
require: exists q[head=n, lemma=few|good|full|estimate|estimated|further|extra|additional|mere|whopping|great|total]
severity: warn
source: P17-1074 (DET, NOUN:NUM); Fowler MEU 1926, A, AN §2 (p. 13); Poutsma GLME III Ch. XXVI §17; Poutsma GLME IV Ch. XL §58–62
```
