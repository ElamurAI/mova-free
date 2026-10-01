# Relative that is a pronoun; no preposition precedes it

**Gist.** *That* in a relative clause (*the book that I read*) is a relative pronoun, like *which/who*: it takes the place of the subject or object of the clause. Unlike *who/which*, *that* cannot be preceded by a preposition: *the house in which I live*, but *the house that I live in* (preposition at the end), not *in that I live*.
**Conditions and exceptions.** Poutsma (XXXIX §34): *that* is always at the start of the clause, so it is impossible where the relative word must be preceded by a preposition, *all/both* or a participle. *That* introduces mostly restrictive clauses; in non-restrictive ones (after a comma) — *who/which*, and old *that* there is an archaism (§16–17). CGEL considers *that* a subordinator, UD a pronoun.
**Examples.** ✓ *the man that I met*; ✓ *the house that Jack built*; ✓ *the tool that I work with*; ✗ *the tool with that I work*.
**In UD.** PRON, XPOS WDT, `PronType=Rel`; the relation is its role in the clause (`nsubj` 379, `obj` 134, `nsubj:pass` 63, `obl` with a stranded preposition…), not `mark` (EWT 2.18: 1 exception). A preposition (`case`) on relative *that* is only to the right (30 of 30), while on *which/whom* it is mostly to the left.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIX §12, §16–17, §34 (pp. 290, 296, 324 = book pp. 970, 976, 1004); Poutsma GLME vol. 1, preface (text-1.md: *that* as a relative pronoun); https://universaldependencies.org/en/dep/acl-relcl.html (Relativizers, note 1); CGELBank (*that* — Sdr).

```rule
rule: en.nominal.relative-that-not-mark
what: that at the start of a relative clause is a pronoun (PRON) with a role in the clause, not mark
match: v[rel=acl:relcl]; t[lemma=that, head=v, before=v]
require: not t[rel=mark]; t[upos=PRON]
severity: error
source: Poutsma GLME IV Ch. XXXIX §12, §34; UD en acl:relcl (Relativizers); 2023.udw-1.7 (v2.11 relatives); UD _en/dep/mark.md
```

```rule
rule: en.nominal.relative-that-no-preposition
what: relative that is never preceded by a preposition (the house in which I live / that I live in)
match: t[lemma=that, feats.PronType=Rel]; c[rel=case, head=t]
require: c[after=t]
severity: error
source: Poutsma GLME IV Ch. XXXIX §34
```
