# Attributive only: main, mere, sheer, utter, chief

**Gist.** Some adjectives are used only with a noun and are never predicative: *main, mere, sheer, utter, chief, principal, lone, former, latter, very* (*the very man*), as well as relational adjectives that replace a genitive or an adverb (*a daily paper, a wooden house, a medical school*). Poutsma: nobody says *The journals are daily* — it is *The journals appear daily*.
**Conditions and exceptions.** *The main thing is…* — *main* is on the noun, not a predicate. *Former, latter* without a noun are substantivized (*the latter is…*) — this is not predicative use. *Live* "in real time" can be predicative (*The show is live*), so it is not on the list.
**Examples.** ✓ *the main road*; ✓ *sheer luck*; ✗ *The road is main.*; ✗ *His luck was sheer.*
**In UD.** Such an ADJ with a `cop` child is suspicious. EWT 2.18: *main* 38, *former* 46, *mere* 3, *sheer* 2 — none with `cop`.
**Sources.** Poutsma GLME vol. 3, Ch. XXVIII §7 (p. 380 = book p. 360).

```rule
rule: en.nominal.attributive-only-adj
what: main/mere/sheer/utter/chief/principal/lone are never predicates with a copula
match: a[upos=ADJ, lemma=main|mere|sheer|utter|chief|principal|lone]
require: none c[rel=cop, head=a]
severity: warn
source: Poutsma GLME III Ch. XXVIII §7
```
