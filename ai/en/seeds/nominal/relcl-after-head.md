# A relative modifier stands after the noun

**Gist.** A relative clause (*the man who came, the book I read*) stands after the noun it modifies and contains a "gap" corresponding to that noun: *the book [that I read ___]*. Poutsma divides such clauses into restrictive (no comma, they pick out the item) and non-restrictive (after a comma, they add a fact); Curme into "restrictive" and "descriptive".
**Conditions and exceptions.** The head is usually a noun or pronoun (*those who, something that, all that*), sometimes an adjective used as a noun (*the best that…*). A clause referring to the whole sentence (*…, which was a bad idea*) is `advcl:relcl`, not `acl:relcl`.
**Examples.** *the book that I read*; *people who care*; *everything you said*.
**In UD.** The main word of the clause is `acl:relcl` to the noun, always to its right (EWT 2.18: 2 341 of 2 341). The relative pronoun takes its role in the clause (`nsubj`, `obj`, `obl`…). The head is NOUN, PRON, PROPN, rarely DET, ADJ, NUM, SYM.
**Sources.** Poutsma GLME vol. 2, Ch. XVI §1 (p. 112); Poutsma GLME vol. 4, Ch. XXXIX §6 (p. 285 = book p. 965); Curme 1931, Ch. XIV (§23 II 6 "Descriptive and Restrictive"; p. 223); https://universaldependencies.org/en/dep/acl-relcl.html.

```rule
rule: en.nominal.relcl-after-head
what: a relative clause (acl:relcl) stands after its noun
match: n[]; v[rel=acl:relcl, head=n]
require: v[after=n]
severity: error
source: Poutsma GLME II Ch. XVI §1; UD en acl:relcl
```

```rule
rule: en.nominal.relcl-head-nominal
what: acl:relcl attaches to a nominal head (NOUN, PROPN, PRON, NUM, DET, substantivized ADJ, SYM)
match: n[]; v[rel=acl:relcl, head=n]
require: n[upos=NOUN|PROPN|PRON|NUM|DET|ADJ|SYM]
severity: warn
source: Poutsma GLME IV Ch. XXXIX §6; UD en acl:relcl
```
