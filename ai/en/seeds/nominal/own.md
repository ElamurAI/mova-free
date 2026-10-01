# own — only after a possessive

**Gist.** The adjective *own* intensifies possession and stands only after a possessive pronoun or genitive: *my own house, John's own idea, a room of one's own*. Without a possessive *own* is not used (*an own house* ✗ — a calque of German *ein eigenes Haus*).
**Conditions and exceptions.** The set sports term *own goal* is a compound noun. *On my own, of their own, a room of one's own* — *own* without a noun (Poutsma: a partly substantivized adjective), but it still has a possessive.
**Examples.** ✓ *her own car*; ✓ *their very own*; ✓ *on my own*; ✗ *an own car*.
**In UD.** *own* is ADJ JJ `amod`; its head has `nmod:poss` (EWT 2.18: 82 of 82). Without a noun (*on my own*) *own* is an ADJ head (`obl`, `nmod`) with its own `nmod:poss` (14 of 14).
**Sources.** Poutsma GLME vol. 3, Ch. XXIV §38–39 (p. 108 = book p. 88); Ch. XXIX §28 (p. 445); Poutsma GLME vol. 4, Ch. XXXIII §18–20 (p. 135 = book p. 815).

```rule
rule: en.nominal.own-needs-possessor
what: own as a modifier requires a possessive on the same noun
match: n[]; o[lemma=own, upos=ADJ, rel=amod, head=n]
require: exists p[rel=nmod:poss, head=n]
severity: warn
source: Poutsma GLME III Ch. XXIV §38; Poutsma GLME IV Ch. XXXIII §18
```

```rule
rule: en.nominal.own-alone-needs-possessor
what: own without a noun (on my own) has its own possessive
match: o[lemma=own, upos=ADJ, rel=obl|nmod|obj|nsubj]
require: exists p[rel=nmod:poss, head=o]
severity: warn
source: Poutsma GLME III Ch. XXIX §28
```
