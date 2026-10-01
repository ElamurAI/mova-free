# Negative prefixes: un-, in-/im-/il-/ir-, dis-, non-

**Gist.** Negative prefixes turn an adjective (less often a noun or verb) into a word with the opposite meaning: *happy → unhappy, possible → impossible, honest → dishonest, smoker → non-smoker*. The prefix does not change the part of speech: *unhappy* is just as much an adjective. The negation here is **lexical**, so in UD such words do not have the feature `Polarity=Neg`: only grammatical words get it (*not, n't, nor, no* as an interjection).

**Conditions and exceptions.**
- *Un-* is native and the most productive: with adjectives and adverbs (*unkind, unfortunately*), with nouns derived from them (*unkindness, untruth*), with participles (*unfinished, unwilling, unheard-of*). With verbs *un-* has another meaning — "reverse the action": *untie, undo, unlock, unpack* (so *unlocked* is ambiguous).
- *In-* goes with Latin stems and assimilates: *il-* before *l* (*illegal*), *im-* before *b, m, p* (*imbalance, immoral, impossible*), *ir-* before *r* (*irregular*). Pairs: *unable/inability, unjust/injustice*.
- Lexicalized: *infamous, invaluable* ("priceless", not "not valuable"), *impertinent, indifferent*.
- *Dis-* (*dishonest, disagree, disappear*), *non-* (*nonsense, non-smoker, non-profit*), Greek *a-* (*amoral, atypical*), *mis-* "wrongly" (mostly with verbs: *misread*).
- A prefix written separately or with a hyphen is sometimes split off by EWT as a separate token with XPOS AFX (*non profit, mid 90s*); UPOS is then inconsistent (X, ADJ, ADV).
- The lemma keeps the prefix: *unhappy → unhappy*, not *happy*.

**Examples.** *an **unhappy** child; it's **impossible**; a **dishonest** man; **non-profit** organisations.*

**In UD.** A word with a prefix is an ordinary ADJ/NOUN/VERB/ADV with its own lemma; it has no `Polarity`.

**Sources.** https://universaldependencies.org/en/feat/Polarity.html ("Neither do negative prefixes (on adjectives: wise – unwise, probable – improbable)"), https://universaldependencies.org/en/pos/X.html (AFX); Jespersen MEG VI 26.1₂–26.1₇ (text-5, pp. 481–485: *un-* with adjectives, adverbs, participles; *unable/inability*), 26.2₁, 26.2₄ (pp. 488–490: assimilation *il-, im-, ir-*; *invaluable*), 26.3₃, 26.4₁–26.4₄, 26.5₁, 26.6₃ (pp. 491–500: *non-*, privative *un-*, *dis-*, *mis-*); Sweet NEG I §1584, §1587 (text-1, pp. 485–486), §1652 (p. 504: *in-* only with longer foreign words), §1642, §1658 (pp. 502, 505); Whitney §100, §193d (text-1, pp. 60, 103).

```rule
rule: en.morph.no-polarity-on-lexical-words
what: nouns, adjectives, verbs, adverbs have no Polarity (negative prefixes are lexical)
match: w[upos=ADJ|ADV|NOUN|PROPN|VERB]
require: w[!feats.Polarity]
severity: error
source: UD en feat/Polarity (negative prefixes do not receive the feature); Jespersen MEG VI 26.1–26.2
```
