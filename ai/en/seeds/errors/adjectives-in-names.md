# An adjective in a name — amod, not compound

**Gist.** Since UD 2.8, adjectives and verbs in proper names have their own part of speech (ADJ, VERB), although XPOS stays NNP. For example, in *United States* the word United is ADJ with amod(States, United). An adjective as a `compound` dependent is a legacy of the automatic PTB conversion, where everything inside a noun phrase was treated as a compound. Because of this, a model trained on GUM gives `amod` in such places, and one trained on EWT gives `compound` (*Islamist officers, Baathist saboteurs*).

**Conditions and exceptions.** Lexicalized pairs that EWT kept as `compound`: *top notch*, *green tea*, *quarter end*.

**Examples.**
- *the United States* → amod(States, United).
- *Central Iowa* → amod(Iowa, Central).
- *Islamist officers* → amod(officers, Islamist).

**In UD.** An adjective modifying a noun is `amod` regardless of capitalization and the NNP tag. The lemma of an adjective in a name is capitalized (since 2.8).

**Check against gold.** EWT 2.18: ADJ as `compound` of a noun — 14; GUM — 57 (such pairs are more frequent there).

**Sources.** `2023.udw-1.7` (v2.8: ADJ and VERB in names instead of PROPN; appendix: *Islamist officers*); english-banks §1.4, §2 (proper names); UD `_en/dep/flat.md` (example *Natural Resources Conservation Service* → amod + compound).

```rule
rule: en.errors.adj-not-compound
what: an adjective as compound of a noun — should be amod (since UD 2.8, also in names)
match: h[upos=NOUN|PROPN]; d[rel=compound, upos=ADJ, head=h]
require: not d[upos=ADJ]
severity: warn
source: 2023.udw-1.7 (v2.8; Islamist officers); UD _en/dep/flat.md (Natural Resources → amod)
```
