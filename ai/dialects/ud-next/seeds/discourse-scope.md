# discourse: boundary by word class (universal, with an English example)

**Gist.** After 2.18 the definition of `discourse` was rewritten. Now these are discourse expressions with grammatical properties that distinguish them from ordinary syntagmatic relations. The central cases are interjections, particles, item numbering, emoticons. Each language decides itself what belongs here, and for English it is said explicitly: adverbs (*actually*) and prepositional phrases (*in other words*) are not `discourse` but `advmod` and `obl`, even when they work as discourse markers.

**Level.** Universal, `_u-dep/discourse.md`. The English `_en/dep/discourse.md` only refers to this page, so for en this is a direct guideline change.

**Before (2.18).** `2.18:https://universaldependencies.org/u/dep/discourse.html:8–14`: interjections and other discourse particles, «not clearly linked to the structure of the sentence, except in an expressive way». For English — «non-adverbial discourse markers (*well*, *like*, but not *you know* or *actually*)» and item numbering.

**Now (snapshot 24.09.2026).** https://universaldependencies.org/u/dep/discourse.html:
- `:8–10` — expressions «with grammatical properties that distinguish them from expressions appearing in typical syntagmatic relations (like advmod, obl, advcl, parataxis, vocative, etc.)»;
- `:11–19` — a list:
  - interjections: expressive (*oh*, *Welcome*), backchannel (*yes*, *uh-huh*), fillers (*um*);
  - particles that cannot be classed as adverbs or prepositions: interrogative, vocative;
  - item numbering;
  - emoji and emoticons with an extra-syntactic expressive function;
- `:25–31` — «`discourse` is reserved as a syntactic function for constructions signaling discourse meaning in grammatically distinctive ways … In English, for example, we exclude from `discourse` any items from the lexical class of adverbs (_actually_) and prepositional phrases (_in other words_); these are simply [advmod]() and [obl]()».

The examples *well*, *like*, *you know* were removed from the text.

**Evidence.** `diff` `2.18:https://universaldependencies.org/u/dep/discourse.html ↔ https://universaldependencies.org/u/dep/discourse.html. There is no entry in `changes.md`: it counts as a minor clarification («Many minor clarifications are not listed»). There is no separate issue in the dump.

**What it means for English annotation.** The criterion is now word class, not meaning:
- ADV with `discourse` → `advmod`. EWT 2.18 has 9 of them (train 4, dev 1, test 4), GUM — 78 (*so* 57);
- a prepositional phrase with `discourse` → `obl`. In EWT 0: *in other words* — `obl` 5 of 5;
- emoticons with `discourse` are now named in the guideline: EWT has SYM with `discourse` 123 (`:)` 59, `:(` 9, `:-)` 8…);
- interjections (INTJ 775) and numbering (NUM 113) — unchanged;
- *like* and *well* as INTJ with `discourse` (27 and 58 in EWT) remain: they are interjections, not adverbs.

**Registry.** Unchanged: `discourse` is allowed; the registry does not check UPOS × relation pairs.

**Sources.** https://universaldependencies.org/u/dep/discourse.html, `2.18:https://universaldependencies.org/u/dep/discourse.html, https://universaldependencies.org/en/dep/discourse.html; `dialects/ud-2.18/seeds/discourse-adverbs.md`.

```rule
rule: udnext.discourse-not-adv
what: discourse is not for adverbs — in English ADV stays advmod
match: d[rel=discourse, upos=ADV]
require: not d[rel=discourse]
severity: error
source: https://universaldependencies.org/u/dep/discourse.html:25–31 (snapshot 24.09.2026)
```

```rule
rule: udnext.discourse-not-pp
what: discourse is not for prepositional phrases — they are obl
match: d[rel=discourse]; c[rel=case, head=d]
require: not d[rel=discourse]
severity: error
source: https://universaldependencies.org/u/dep/discourse.html:28–31 (snapshot 24.09.2026)
```
