# Possessive 's is a separate function token

**Gist.** The possessive (genitive) case of an English noun is formed with the clitic *'s* (*John's, men's*) or with a bare apostrophe in plurals in *-s* (*the boys'*) and in old names in *-s* (*Jesus', Socrates'*). In the modern language only this "s-genitive" and the *of*-construction are alive (Curme).
**Conditions and exceptions.** *It's* = *it is*, not a possessive; *its* is a pronoun without an apostrophe. Names ending in a sibilant: *Jones's* and *Jones'* — spelling varies. Once *John his book* (the his-genitive, Curme) lived alongside — today it is vernacular.
**Examples.** *the boy's hat*; *the boys' hats*; *children's shoes*; *Mrs. Adams's wrapper* (Curme).
**In UD.** *'s* or *'* is a separate token: PART, XPOS POS, lemma `'s`, relation `case` to the head of the possessor phrase, to its right. The possessor is `nmod:poss` to what is possessed. EWT 2.18: 842 POS — all PART `case`.
**Sources.** Poutsma GLME vol. 3, Ch. XXIV §1–2 (pp. 50–53 = book pp. 30–33); Curme 1931 §10 II 1 (pp. 70–73); https://universaldependencies.org/en/dep/nmod-poss.html; Santorini 1990 (PTB), POS.

```rule
rule: en.nominal.pos-clitic
what: possessive 's / ' (POS) — PART with lemma 's, relation case, after its possessor
match: p[]; s[xpos=POS, head=p]
require: s[upos=PART, lemma='s, rel=case, after=p]
severity: error
source: Poutsma GLME III Ch. XXIV §1; Curme 1931 §10 II 1; UD en nmod:poss
```
