# Adjectives in a-: afraid, asleep, alive — predicative only

**Gist.** Adjectives with the old prefix *a-* (once a preposition: *on sleep* → *asleep*) — *afraid, asleep, awake, alive, alike, alone, aware, ablaze, ashamed, akin* — almost never stand before a noun: *The child is asleep*, but not *an asleep child*. Before a noun other words are used: *a sleeping child, a live fish, a lone rider, a frightened man*.
**Conditions and exceptions.** After the noun they are possible: *the only man alive, the bravest man alive*. With an intensifier they occasionally occur attributively (*a wide-awake child*). Poutsma adds *ill* "sick" to the predicative ones.
**Examples.** ✓ *The baby is asleep.*; ✓ *the greatest poet alive*; ✗ *an afraid dog*.
**In UD.** Such an ADJ as `amod` before a noun is suspicious. EWT 2.18: not once (2 `amod` — after the noun).
**Sources.** Poutsma GLME vol. 3, Ch. XXVIII §8 b (p. 381 = book p. 361; after Onions, Advanced English Syntax §25).

```rule
rule: en.nominal.a-adj-not-prenominal
what: afraid/asleep/awake/alive/alike/alone/aware/ashamed… do not stand before a noun as amod
match: n[]; a[lemma=afraid|asleep|awake|alive|alike|alone|aware|ashamed|ablaze|aghast|ajar|akin|averse, rel=amod, head=n]
require: a[after=n]
severity: warn
source: Poutsma GLME III Ch. XXVIII §8 b
```
