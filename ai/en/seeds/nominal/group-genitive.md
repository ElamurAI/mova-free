# Group genitive: 's stands at the end of the group but belongs to its head

**Gist.** In compound names and phrases *'s* is placed after the last word of the whole group: *the King of England's power, my father-in-law's house, Julius Caesar's death, someone else's car*. But the possessor is the head of the group (*King, Caesar, someone*), not the last word. Curme explains: *King's of England* would be taken as a plural, so the *-s* moved to the end.
**Conditions and exceptions.** Joint possession — one *'s* at the end (*John and Mary's house*), separate possession — on each (*John's and Mary's books*). Before ~1500 people said *the King's property of England* (Curme).
**Examples.** ✓ *the King of England's*; ✓ *John Smith's car*; ✗ *the King's of England*.
**In UD.** `case('s)` attaches to the head of the possessor group: in *John Smith's* — to *John* (*Smith* is `flat` from *John*), in *someone else's* — to *someone*. So *'s* does not hang on a `flat` or `compound` part of a name (EWT 2.18: 0 and 1 of 842). In coordination EWT attaches *'s* to the last conjunct (`conj`, 21 times).
**Sources.** Poutsma GLME vol. 3, Ch. XXIV §3–4 (pp. 53–55 = book pp. 33–35); Curme 1931 §10 II 1 d "Group Genitive" (pp. 77–78); https://universaldependencies.org/en/dep/nmod-poss.html.

```rule
rule: en.nominal.group-genitive
what: 's attaches to the head of the possessor group, not to a flat or compound part of a name
match: f[]; s[xpos=POS, head=f]
require: not f[rel~flat]; not f[rel=compound]
severity: error
source: Poutsma GLME III Ch. XXIV §3; Curme 1931 §10 II 1 d; UD en nmod:poss
```
