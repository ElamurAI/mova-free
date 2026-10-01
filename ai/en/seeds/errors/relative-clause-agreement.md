# The relative-clause verb agrees with the antecedent

**Gist.** When the relative pronoun (*who, which, that*) is the subject of the relative clause, the verb takes the number of the noun the clause is attached to: *people who live*, *a man who lives*. A mismatch means an error in the text or that the relative clause was attached to the wrong noun. A typical ambiguous case is *the servant of the actress who…*: which of the two heads? Agreement across a relative clause is the hardest for language models among agreement tests.

**Conditions and exceptions.**
- *one of the best men that have ever lived*: according to Fowler, the antecedent is *men*, so the verb is plural; singular *has* is often used, but it is a "constant error".
- Collective nouns (*the Taliban want*) agree by meaning.

**Examples.**
- ✗ *the people who lives here* → ✓ *live*.
- ✓ *He is one of the best men that have ever lived.*

**In UD.** acl:relcl(n, v), nsubj(v, who/which/that) with PronType=Rel. The number of n is compared with the XPOS of v.

**Check against gold.** EWT 2.18: plural antecedent + VBZ — 338/1; singular antecedent + VBP — 601/17.

**Sources.** Fowler MEU 1926, NUMBER (pp. 401–402: *one of the best men that have ever lived*); `2025.depling-1.4` (Garcia et al.: relative clauses with attractors are the hardest); `2020.tacl-1.25` (BLiMP: attractor in a relative clause); `2025.law-1.16` (ICLE-RC: annotation of relative clauses in learner texts).

```rule
rule: en.errors.relcl-plural-antecedent
what: plural antecedent, the relative pronoun is the subject, but the relative-clause verb is VBZ
match: n[feats.Number=Plur]; v[rel=acl:relcl, head=n]; w[rel~nsubj, head=v, feats.PronType=Rel]
require: not v[xpos=VBZ]
severity: warn
source: Fowler MEU 1926, NUMBER (pp. 401–402); 2025.depling-1.4; 2020.tacl-1.25
```

```rule
rule: en.errors.relcl-singular-antecedent
what: singular noun antecedent, the relative pronoun is the subject, but the relative-clause verb is VBP
match: n[upos=NOUN|PROPN, feats.Number=Sing]; v[rel=acl:relcl, head=n]; w[rel~nsubj, head=v, feats.PronType=Rel]
require: not v[xpos=VBP]
unless: exists c[rel=conj, head=n]; n[lemma=lot|lots|number|majority|minority|rest|remainder|half|third|quarter|percent|percentage|proportion|fraction|portion|part|share|bunch|couple|pair|variety|plenty|range|group|set|series|kind|sort|type|handful|dozen|host|mass|pile|ton|load|heap|total|wealth]; n[lemma=family|team|government|committee|army|party|board|company|council|court|crew|staff|panel|body|crowd|public|admiralty|aristocracy|multitude|assembly|congress|class|choir|gentry|navy|cavalry|infantry|police|poultry|race]
severity: warn
source: Fowler MEU 1926, NUMBER (pp. 401–402); 2025.depling-1.4; exceptions: coordinated antecedent (the husband and wife who run); quantity and collective nouns — as in errors/singular-subject-plural-verb.md
```
