# A relative clause has a subject — its own or the relative pronoun

**Gist.** The relative pronoun can be omitted (Curme: "asyndetic relative clause") when it is an object (*the man [whom] I saw*), the object of a stranded preposition (*the house [that] I live in*) or a predicative (*the man [that] he is*). When it is the subject, omission is impossible in the standard language: *the man who came*, not *the man came*.
**Conditions and exceptions.** Colloquially and in dialects the subject pronoun drops after *there is / here is* (*There's a man wants to see you*, Poutsma XXXIX §29). In a clause with *there*, the subject role is taken by *there* to the ear (*the problems there are*, §28 d). So a finite relative clause on a noun has `nsubj` (the relative pronoun or its own subject) or `expl` *there*.
**Examples.** ✓ *the book I read* (`nsubj` = I); ✓ *the man who came* (`nsubj` = who); ~ *There's a man wants you* (colloquial).
**In UD.** A finite `acl:relcl` on NOUN/PROPN without `nsubj`/`expl` — in EWT 2.18 only 2 times out of 990, both text glitches.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIX §27–29 (pp. 313–316 = book pp. 993–996); Curme 1931, Ch. XIV §23 II 10 "Asyndetic Relative Construction" (pp. 233–235); https://universaldependencies.org/en/dep/acl-relcl.html (reduced RC).

```rule
rule: en.nominal.finite-relcl-subject
what: a finite relative clause on a noun has a subject (relative pronoun, own subject) or there
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, feats.VerbForm=Fin, head=n]
require: exists s[head=v, rel=nsubj|nsubj:pass|nsubj:outer|csubj|csubj:pass|expl]
severity: warn
source: Poutsma GLME IV Ch. XXXIX §27–29; Curme 1931 §23 II 10
```
