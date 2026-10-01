# The prop-word one: the big one, the red ones

**Gist.** So that an adjective or determiner can stand without a noun, English puts *one/ones* after it: *the red one, a big one, these ones, the ones I like*. This is not the numeral "one" but a substitute for a countable noun (Poutsma: "prop-word").
**Conditions and exceptions.** Only for countables: of milk — *the white*, not *the white one*. After a possessive it is usually not used (*my one* is colloquial). Curme considers *one* here rather a suffix that substantivizes the adjective; for annotation it is a noun.
**Examples.** *I want the blue one.*; *Which ones?*; *the one on the left*.
**In UD.** NOUN (NN *one*, NNS *ones*), `Number=Sing|Plur`; it has `det`/`amod` and takes the role of the whole phrase. The numeral *one* (NUM) has no `det`/`amod` (EWT 2.18: 0 of 425); *ones* is always NOUN (41 of 41).
**Sources.** Poutsma GLME vol. 4, Ch. XLIII §1–3 (pp. 592–594 = book pp. 1272–1274); Curme 1931 §57 1 "Use of the Suffix One" (pp. 518–520).

```rule
rule: en.nominal.num-one-no-det
what: the numeral one (NUM) has no det or amod — with them it is the prop-word (NOUN)
match: o[upos=NUM, lemma=one]
require: none c[rel~det, head=o]; none c[rel=amod, head=o]
severity: warn
source: Poutsma GLME IV Ch. XLIII §3; Curme 1931 §57 1
```

```rule
rule: en.nominal.ones-noun
what: ones is always a plural prop-word noun
match: o[form=ones, !feats.Typo]
require: o[upos=NOUN, feats.Number=Plur]
severity: error
source: Poutsma GLME IV Ch. XLIII §3; Curme 1931 §57 1
```
