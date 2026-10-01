# much, many and countability

**Gist.** *many* combines with plural countable nouns (*many books*), *much* with singular uncountable ones (*much time*). Uncountable nouns have no plural: *information, advice, furniture, equipment, knowledge, homework, luggage, evidence, feedback, software* (✗ *informations*). ERRANT assigns such errors to NOUN:INFL (*informations → information*) and NOUN:NUM. They are typical of speakers whose language treats these words as countable: Ukrainian, where the words for *advice, furniture, information* are countable plurals.

**Conditions and exceptions.**
- *much* as an adverb (*much bigger*, *not much*) is not covered by the rule: only `amod`/`det` on a noun is taken.
- *many a man* is singular (literary).
- Words with an established plural in particular senses are not in the uncountable list: *accommodations*, *behaviors*, *damages*, *researches*, *staffs*.

**Examples.**
- ✗ *this has caused much difficulties*
- ✗ *much monies*
- ✗ *storage for your luggages*

**Check against gold.** EWT 2.18: much + plural — 4 (all errors in the text); many + singular — 1; uncountables in the plural — 1 (*luggages*).

**Sources.** `P17-1074` (table 2: NOUN:INFL — *informations → information*; NOUN:NUM — countable and uncountable); `W19-4406` (table 4: NOUN:NUM 4.05 %, NOUN:INFL 0.12 % of W&I train edits).

```rule
rule: en.errors.much-plural
what: much with a plural noun — with a plural, many is used
match: n[upos=NOUN, feats.Number=Plur]; d[lemma=much, head=n, rel=amod|det]
require: not d[lemma=much]
severity: warn
source: P17-1074 (NOUN:NUM); W19-4406
```

```rule
rule: en.errors.many-singular
what: many with a singular noun — with an uncountable, much is used
match: n[upos=NOUN, feats.Number=Sing]; d[lemma=many, head=n, rel=amod|det]
require: not d[lemma=many]
severity: warn
source: P17-1074 (NOUN:NUM); W19-4406
```

The rule is `en.nominal.mass-no-plural` in `nominal/mass-nouns.md`.
