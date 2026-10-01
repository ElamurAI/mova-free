# A determiner stands before its noun

**Gist.** An article, demonstrative or quantifier (*the, a, this, every, no, some*) in English stands before the noun it specifies. Only numerals, adjectives and noun modifiers can come between them. The determiner itself is always a separate function word, DET.
**Conditions and exceptions.** *All, both, each* after a noun or pronoun (*the boys all left*, *we both agree*, *see you all*) are no longer determiners but "floating" quantifiers; UD annotates them differently (e.g. `nmod:unmarked`). Numerals and possessives are not `det` but `nummod` and `nmod:poss`.
**Examples.** *the old house*; *every other day*; *no such thing*; ✗ *house the*.
**In UD.** A determiner is UPOS DET, relation `det` (subtype `det:predet` for *all the*, *such a*) to the head noun, always to its left. Check on EWT 2.18: 19 371 `det`, all to the left and all DET.
**Sources.** Poutsma GLME vol. 1, Ch. VIII §150 (p. 554): the article, pronoun and numeral stand before adjectives; https://universaldependencies.org/en/dep/det.html, `det-predet.md`, `nmod-unmarked.md` (v: *See you all*).

```rule
rule: en.nominal.det-before-head
what: a determiner (det, det:predet) stands before its head
match: n[]; d[rel~det, head=n]
require: d[before=n]
severity: error
source: Poutsma GLME I Ch. VIII §150; UD en det, det:predet
```

```rule
rule: en.nominal.det-is-det
what: the det relation carries only DET (numeral — nummod, possessive — nmod:poss)
match: d[rel~det]
require: d[upos=DET]
severity: error
source: UD en det, nummod, nmod:poss
```
