# Existential there is a subject pronoun without locative meaning

**Gist.** In existential sentences (*There is a cat on the roof*) *there* is a formal subject without the meaning "in that place"; the real subject (*a cat*) stands after the verb. Poutsma: unstressed *there* "takes over the subject role to the ear".
**Conditions and exceptions.** It differs from the locative adverb *there* ("in that place"): it is unstressed and does not answer the question "where?". Both can occur in one sentence: *There's a dog there.*
**Examples.** *There are three options.*; *There seems to be a problem.*
**In UD.** Existential: PRON, XPOS EX, `PronType=Dem`, relation `expl`; the noun subject is `nsubj`. Locative *there* is ADV RB `advmod`. EWT 2.18: 466 EX (including the typos *their/they*) — all PRON `expl`.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIX §28 d (p. 314 = book p. 994); https://universaldependencies.org/en/dep/expl.html; Santorini 1990 (PTB), EX.

```rule
rule: en.nominal.ex-there
what: existential there (EX) is PRON with PronType=Dem, relation expl
match: t[xpos=EX]
require: t[upos=PRON, feats.PronType=Dem, rel=expl]
severity: error
source: Poutsma GLME IV Ch. XXXIX §28 d; UD en expl; UD _en/dep/expl.md (There is a ghost in the room)
```
