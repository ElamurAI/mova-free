# who, whom, whose, which: case and person

**Gist.** A relative pronoun takes its case from its role in the clause (Curme): *who* — subject, *whom* — object or word after a preposition, *whose* — possessive. *Who/whom* are for persons, *which* for things, *that* for both (Poutsma XXXIX §9–12).
**Conditions and exceptions.** In colloquial speech *who* instead of *whom* is the norm (*the man who I saw*, Curme). *Whom* as subject (*the man whom I think is guilty*) is hypercorrection. *Whose* is also used of things (*a house whose roof leaks*).
**Examples.** *the boy whom I trusted*; *the boy whose knife was lost*; *the boy to whom I gave it*.
**In UD.** *who, whom* — PRON WP, `PronType=Rel` (in questions `Int`); *whose* — PRON WP$, `Poss=Yes`, relation `nmod:poss` (EWT 2.18: 13 of 13); *which* — PRON WDT or DET (*which book*). *Whom* as `nsubj` — not once.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIX §4, §9–12 (pp. 279, 286–290 = book pp. 959, 966–970); Curme 1931, Ch. XIV §23 II 7–8 "Personality and Form", "Case of Relative" (pp. 228–231).

```rule
rule: en.nominal.whom-not-subject
what: whom is objective case; as a subject it is hypercorrection or a relation error
match: w[form=whom, !feats.Typo]
require: not w[rel~nsubj]
severity: warn
source: Poutsma GLME IV Ch. XXXIX §4; Curme 1931 §23 II 8
```

```rule
rule: en.nominal.whose-poss
what: whose is possessive WP$ with Poss=Yes, usually nmod:poss
match: w[upos=PRON, form=whose, !feats.Typo]
require: w[xpos=WP$, feats.Poss=Yes, rel=nmod:poss]
severity: warn
source: Poutsma GLME IV Ch. XXXIX §4 b; Curme 1931 §23 II 8
```
