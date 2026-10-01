# Demonstratives this/these, that/those

**Gist.** Demonstratives are the only English determiner words that inflect for number: *this* → *these* (near), *that* → *those* (far). They stand before a noun (*this book*) or replace a noun phrase on their own (*I like this*). The lemma of the plural is the singular: *these → this, those → that*.

**Conditions and exceptions.**
- Before a noun — DET, relation `det`; standalone — PRON. The PTB tag is DT in both cases.
- *That* has three more roles without demonstrative meaning: relative pronoun (*the book **that** I read*: PRON, WDT, `PronType=Rel`, no number), conjunction (*I think **that**…*: SCONJ, IN) and degree adverb (*not **that** big*: ADV, RB). Only demonstrative *that* has the `Number` and `PronType=Dem` features.
- The adverbs *here, there* are also `PronType=Dem`, as is expletive *there* (EX); they have no number.
- Colloquial *them* in the role of *those* (*them apples*) — DET with `Number=Plur|PronType=Dem|Style=Vrnc`.

**Examples.** ***These** books are mine. I prefer **that**. **Those** who wait…*

**In UD.** UPOS DET or PRON; XPOS DT; `PronType=Dem`; `Number=Sing` (*this, that*) or `Number=Plur` (*these, those*); lemma *this* or *that*.

**Sources.** https://universaldependencies.org/en/pos/DET.html (lexeme table), https://universaldependencies.org/en/pos/PRON.html (Demonstrative pronouns: *these [this], those [that]*), `feat/PronType.md` (Dem; *that* only as demonstrative); Santorini 1990, §2, p. 2 and §4.1, p. 8 (DT also without a noun: *I can’t stand this/DT*); Sweet NEG I §1128–1130 (text-1, p. 381: *this–these, that–those*), §177 (p. 95: the only agreement of an adjectival word with a noun), §125 (p. 78: *these kind of things*); Whitney §150, §166–168 (text-1, p. 84–85, 89–90: pronoun vs "pronominal adjective", i.e. PRON vs DET); Jespersen MEG II 2.22, 16.3₁ (text-2, p. 49, 438: nonstandard *them stairs*).

```rule
rule: en.morph.dem-plural
what: these, those — plural, demonstrative, DT
match: d[upos=DET|PRON, form=these|those, !feats.Typo]
require: d[xpos=DT, feats.Number=Plur, feats.PronType=Dem]
severity: error
source: UD en pos/DET, pos/PRON, feat/PronType (Dem)
```

```rule
rule: en.morph.dem-these-lemma
what: the lemma of these is this
match: d[upos=DET|PRON, form=these, !feats.Typo]
require: d[lemma=this]
severity: error
source: UD en pos/PRON (these [this])
```

```rule
rule: en.morph.dem-those-lemma
what: the lemma of those is that
match: d[upos=DET|PRON, form=those, !feats.Typo]
require: d[lemma=that]
severity: error
source: UD en pos/PRON (those [that])
```

```rule
rule: en.morph.dem-this-sing
what: this as a determiner or pronoun — singular, demonstrative
match: d[upos=DET|PRON, form=this, !feats.Typo]
require: d[feats.Number=Sing, feats.PronType=Dem]
severity: error
source: UD en pos/DET (this, that: Number=Sing|PronType=Dem)
```

```rule
rule: en.morph.dem-that-det
what: that before a noun — singular, demonstrative
match: d[upos=DET, form=that, !feats.Typo]
require: d[feats.Number=Sing, feats.PronType=Dem]
severity: error
source: UD en pos/DET, feat/PronType (Dem)
```
