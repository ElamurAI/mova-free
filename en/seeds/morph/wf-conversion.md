# Conversion: one word — different parts of speech (a walk, to bottle)

**Gist.** In English a word easily moves to another part of speech without any suffix: *a walk* (from *to walk*), *to bottle* (from *a bottle*), *to clean* (from *clean*), *to up the price* (from *up*). Jespersen calls this the "unchanged word" or a derivative with a zero suffix. After the shift the word takes on **all** the formal marks of the new class: a noun — article and plural (*three walks*), a verb — *-s, -ed, -ing* (*bottles, bottled, bottling*). So in English the part of speech is determined by syntax and inflection in the particular sentence, not by the dictionary.

**Conditions and exceptions.**
- Directions: noun → verb (*to bottle, to pigeon-hole, to short-circuit*); verb → noun (*have a look, give a push, three tries*); adjective → verb (*to calm, to dry, to empty*, even *to best, to worst*); adverb → verb (*to down, to out*); proper name → noun or verb (*a Plato, to boycott*).
- Some pairs are distinguished by stress (noun — on the first syllable, verb — on the second): *'record/re'cord, 'object/ob'ject, 'present/pre'sent, 'import/im'port, 'insult/in'sult*; others by consonant voicing (*use* [s]/*use* [z], *house*, *belief/believe, proof/prove, bath/bathe, advice/advise*) or by vowel (*food/feed, blood/bleed, full/fill, sing/song*).
- Verbs are formed from the **singular** of the noun (exception *to dice*).
- The lemma follows the new part of speech: *walks* as a noun → lemma *walk* (NOUN), as a verb → *walk* (VERB); *bottled* → *bottle* (VERB).
- An article points to a noun: a word with a definite or indefinite article cannot be a finite or base verb form (VB, VBP, VBZ, VBD). Participles with an article do occur (*the following, the attached*), but these are borderline cases annotated in EWT as VBG/VBN.

**Examples.** *Let's take **a walk*** (NOUN). *They **walk** daily* (VERB). *She **bottled** the wine* (VERB, lemma *bottle*). *Prices **upped*** (VERB, lemma *up*).

**In UD.** UPOS and XPOS by role in the sentence; the lemma is the base form of the corresponding part of speech; features as in the new class.

**Sources.** Jespersen MEG VI 6.1₂ (text-5, pp. 100–101: "unchanged word", zero suffix, classes distinguished by the paradigm *love–loves*), 6.8₃ (p. 118: verbs from the singular and from compounds), 6.9₁–6.9₃ (pp. 125–127: from adjectives and adverbs), 7.1₁–7.2₆ (pp. 128–135: nouns from verbs), 11.2–11.9, 12.1–12.5 (pp. 202–222: pairs with alternation of vowel, voicing, stress); Sweet NEG I §105–106 (text-1, pp. 68–69: conversion is only new inflection; a converted word takes all the formal marks of the class), §164 (p. 90: *a Plato, to boycott*), §887 (p. 315: stress pairs), §914–915 (p. 323: *a breakdown, a drawback*); Whitney §98–99 (text-1, pp. 59–60), §225d (p. 119: *to head, to hand, to face, to witness*).

```rule
rule: en.morph.article-not-on-finite-verb
what: an article does not attach to a finite or base verb form (VB, VBP, VBZ, VBD)
match: w[upos=VERB|AUX, xpos=VB|VBP|VBZ|VBD]; d[feats.PronType=Art, rel=det, head=w]
require: not w[upos=VERB|AUX]
severity: error
source: Sweet NEG I §105–106 (converted word takes the formal marks of its class); Jespersen MEG VI 6.1₂; Santorini 1990, §4.1, p. 19 (NN vs VBG)
```
