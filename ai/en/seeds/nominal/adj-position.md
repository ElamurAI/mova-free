# Adjective position: before the noun; after it only with a reason

**Gist.** An attributive adjective in English stands before the noun (Curme: "adherent"). It stands after the noun (Curme: "appositive") when it has its own dependents (*a man proud of his son*, *a plan so stupid that…*), when there are several adjectives joined by a conjunction (*a laugh musical but malicious*), and after pronouns in *-thing/-body* (*something new*). Such a postposed adjective is like a reduced relative clause.
**Conditions and exceptions.** Fixed postposition (often from French): *court martial, Poet Laureate, Postmaster General, President elect, the sum total, from time immemorial, God Almighty*. Adjectives in *-able/-ible* after a superlative or *only*: *the best style possible, the only person available*. Native English ones: *the amount due, the people involved/concerned/present, five years old*. An adjective with a dependent **before** the noun is possible only in the construction *so/too/as + adjective + a + noun* (a separate seed) or with hyphens (*an easy-to-read book*).
**Examples.** ✓ *a proud man*; ✓ *a man proud of his son*; ✓ *the people involved*; ✗ *a proud of his son man*.
**In UD.** `amod` (ADJ) is usually to the left of the head: EWT 2.18 — 11 043 left, 269 right. To the right of a NOUN — 162 times; 133 of them have their own dependents, the other 29 are fixed postpositives (*available, involved, old, possible, due, left*…).
**Sources.** Curme 1931 §10 I 1 and §10 I 1 a (pp. 63–66); Poutsma GLME vol. 3, Ch. XXVIII §6 (p. 379 = book p. 359: *the present poet laureate*); https://universaldependencies.org/en/dep/amod.html.

```rule
rule: en.nominal.postposed-adj-has-deps
what: an adjective after a noun usually has its own dependents (proud of his son) or is a fixed postpositive (available, involved)
match: n[upos=NOUN]; a[upos=ADJ, rel=amod, head=n, after=n, lemma!=involved|concerned|present|due|left|old|dead|dear|elect|simple|total|martial|politic|laureate|general|immemorial|incarnate|almighty|minor, lemma.suffix!=able|ible]
require: exists c[head=a]
severity: warn
source: Curme 1931 §10 I 1, §10 I 1 a (fixed postpositives: elect, simple, total, martial, politic, laureate, general, immemorial, incarnate, almighty, minor; native: due, dead, dear); seed: involved, concerned, present, left, old, -able/-ible (available, possible)
```
