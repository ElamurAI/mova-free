# A noun modifier before the head noun: compound

**Gist.** English readily puts a noun before a noun as a modifier: *phone book, oil price, shoe shop, college friend*. The head is the last word, the modifier comes before it: *a phone book* is a book, not a phone. Poutsma: such a noun is a "substitute for an adjective" the language lacks (*an iron bedstead, Ceylon tea, a Gladstone bag*), and it is not apposition.
**Conditions and exceptions.** Chains have internal structure: *[oil price] futures*. Proper names with the generic word at the end (*Wall Street, Stanford University, Mirror Lake*) are also `compound` with a right head; the reverse order (*Lake Mead, Mount Everest, Hotel California*) is `flat`. Names with a preposition (*Bank of America*) are ordinary syntax (`nmod`).
**Examples.** *phone book* — `compound(book, phone)`; *Wall Street* — `compound(Street, Wall)`; *Lake Mead* — `flat(Lake, Mead)`.
**In UD.** `compound` stands to the left of the noun head. EWT 2.18: to the right — 23 times out of 8 690, and these are mostly glitches in the annotation itself (*Brothers Grimm* should be `flat`, *Invercargill, New Zealand* — `nmod:unmarked`). The subtype `compound:prt` (verb particle) is a different matter.
**Sources.** https://universaldependencies.org/en/dep/compound.html, `nmod-desc.md` (Place names); Poutsma GLME vol. 3, Ch. XXII §1–3 (p. 21 = book p. 1); Poutsma GLME vol. 1, Ch. IV §8 (p. 292).

```rule
rule: en.nominal.compound-head-final
what: a compound dependent stands before its noun head
match: n[upos=NOUN|PROPN]; c[rel=compound, head=n]
require: c[before=n]
severity: error
source: UD en compound; Poutsma GLME III Ch. XXII §1–3; 2023.udw-1.7 (Sri Lanka: compound EWT / flat GUM); 2020.udw-1.8
```
