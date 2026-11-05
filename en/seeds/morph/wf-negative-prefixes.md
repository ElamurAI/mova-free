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

**Sources.** Aronoff 1976, *Word Formation in Generative Grammar* (blocking: an existing word blocks a synonymous derivative); UD `docs/_en/feat/Polarity.md` ("Neither do negative prefixes (on adjectives: wise – unwise, probable – improbable)"), `docs/_en/pos/X.md` (AFX); Jespersen MEG VI 26.1₂–26.1₇ (text-5, pp. 481–485: *un-* with adjectives, adverbs, participles; *unable/inability*), 26.2₁, 26.2₄ (pp. 488–490: assimilation *il-, im-, ir-*; *invaluable*), 26.3₃, 26.4₁–26.4₄, 26.5₁, 26.6₃ (pp. 491–500: *non-*, privative *un-*, *dis-*, *mis-*); Sweet NEG I §1584, §1587 (text-1, pp. 485–486), §1652 (p. 504: *in-* only with longer foreign words), §1642, §1658 (pp. 502, 505); Whitney §100, §193d (text-1, pp. 60, 103).

```rule
rule: en.morph.no-polarity-on-lexical-words
what: nouns, adjectives, verbs, adverbs have no Polarity (negative prefixes are lexical)
match: w[upos=ADJ|ADV|NOUN|PROPN|VERB]
require: w[!feats.Polarity]
severity: error
source: UD en feat/Polarity (negative prefixes do not receive the feature); Jespersen MEG VI 26.1–26.2
```

```derive
# Derivation data for `en::derive` (en/README.md, "Derived words"). One prefix per line:
# prefix <affix> <classes of the stem> <shortest stem> [block=<rival prefixes>] [except=<stem endings>]
#        [start=<first letters of the stem>] [nostart=<first letters it may not have>] | <gloss>
# Classes: N noun, V verb, A adjective, R adverb. A prefix keeps the stem's tags (it does not change the part of speech).
# block=: a rival negative prefix that already makes a dictionary word with this stem blocks this one (impossible ⇒ no
# *unpossible*, dishonest ⇒ no *unhonest*: Aronoff 1976, blocking). except=: stem endings this prefix does not take
# (in- goes with adjectives, not with adverbs in -ly: *in- + visibly* is invisible + -ly). start=/nostart=: in-
# assimilates to im- before b, m, p, il- before l, ir- before r (*inpossible*, *inlegal* are misspellings). part=: the
# tag a participle stem takes (un- + participle is an adjective, *unchanged*, *unwilling*: JJ in 55 of 61 un…ed forms of
# UD EWT 2.18 train; the rest are mostly reversative *uncovered*).
prefix un   A,V,R  4 block=in,im,il,ir,dis part=JJ | not; reverse the action
prefix in   A      5 block=un,dis,im,il,ir except=ly nostart=b,m,p,l,r | not
prefix im   A      5 block=un,dis,in except=ly start=b,m,p | not (before b, m, p)
prefix il   A      5 block=un,dis except=ly start=l | not (before l)
prefix ir   A      5 block=un,dis except=ly start=r | not (before r)
prefix dis  V,A,N  4 block=un,in,im,il,ir | not; undo
prefix non  N,A    4 | not
prefix mis  V,N    4 | wrongly
```
