# Independent possessives: mine, yours, hers, ours, theirs

**Gist.** Independent possessive pronouns (Poutsma: "absolute") replace a whole "possessive + noun" group: *This book is mine* (= my book), *Yours is better.* They do not stand before a noun.
**Conditions and exceptions.** After a noun with an article, demonstrative or numeral, possession is expressed with *of* + the independent form: *a friend of mine, that dog of yours, two books of his* (double genitive). *His* and *its* have one form for both roles. Archaic *mine eyes* is the old prenominal form before a vowel.
**Examples.** *Yours is better.*; *a friend of mine*; *The choice is theirs.*
**In UD.** PRON, XPOS PRP, `Poss=Yes|PronType=Prs` + Person/Number, **without** `Case`; the lemma is the prenominal form (*mine* → *my*, *yours* → *your*). The relation follows the role in the phrase or sentence: `nsubj`, `obj`, `nmod` (with `case` *of*), predicate; never `nmod:poss`.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIII §1, §21–23 (pp. 102, 139–140 = book pp. 782, 819–820); Curme 1931 §57 5 "Substantive Forms of Possessive Adjectives" (pp. 528–529); https://universaldependencies.org/en/feat/Poss.html.

```rule
rule: en.nominal.abs-poss-feats
what: mine/yours/hers/ours/theirs — PRON PRP with Poss=Yes and without Case
match: p[upos=PRON, !feats.Typo, form=mine|yours|hers|ours|theirs]
require: p[xpos=PRP, feats.Poss=Yes, !feats.Case]
severity: error
source: Poutsma GLME IV Ch. XXXIII §1, §21; UD en Poss
```

```rule
rule: en.nominal.abs-poss-not-det
what: an independent possessive does not attach to a noun as nmod:poss
match: p[upos=PRON, !feats.Typo, form=mine|yours|hers|ours|theirs]
require: not p[rel=nmod:poss]
severity: error
source: Poutsma GLME IV Ch. XXXIII §21–23
```
