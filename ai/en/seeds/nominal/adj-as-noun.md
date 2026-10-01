# Adjective instead of a noun: the rich, the wounded, the English

**Gist.** An adjective can name a class of people or an abstraction without a noun: *the rich, the poor, the wounded, the unknown, the English*. Poutsma calls this partial substantivation: the word takes no plural or genitive *-s*, and means all such people. Full substantivation yields a true noun with all its properties — plural, article *a*, numeral: *a native — natives, valuables, sweets, the blacks*.
**Conditions and exceptions.** Nationality names ending in a sibilant (*the English, the French, the Swiss, the Chinese*) are partially substantivized; those in *-an* (*Americans, Germans*) are full nouns. A partially substantivized adjective takes a plural verb: *The rich are different.*
**Examples.** *The poor get poorer.*; *the best of friends*; *the natives* (already a noun).
**In UD.** A partially substantivized one stays ADJ (JJ, `Degree`): it takes `det` and the role of the phrase (`nsubj`, `obj`, `nmod`…), but has no `Number` (the UD 2.18 registry does not allow `Number` for ADJ) and no `nummod`. If there is a plural *-s* or a numeral, it is a NOUN. In EWT *English* as a language name is PROPN.
**Sources.** Poutsma GLME vol. 3, Ch. XXIX §1, §13–15 (pp. 385, 407–410 = book pp. 365, 387–390); Curme 1931 §57 "Substantive Function of Adjectives" (p. 518); registry `en/data/ud-registry-en.tsv`.

```rule
rule: en.nominal.adj-no-number
what: a substantivized adjective (the rich) stays ADJ without Number; with number it is already a noun
match: a[upos=ADJ]
require: not a[feats.Number]
severity: error
source: Poutsma GLME III Ch. XXIX §1, §14; UD 2.18 registry (ADJ without Number)
```

```rule
rule: en.nominal.adj-no-nummod
what: a numeral on an adjective-as-noun (three whites) is a sign of a full noun (NOUN)
match: a[upos=ADJ]
require: none c[rel=nummod, head=a]
severity: warn
source: Poutsma GLME III Ch. XXIX §1 (full substantivation)
```
