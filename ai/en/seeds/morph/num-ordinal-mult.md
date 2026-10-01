# Ordinal and multiplicative numerals: first, fifth, once, twice

**Gist.** Ordinal numerals, except the first three (*first, second, third*), are formed with the suffix *-th*: *fourth, fifth, sixth, … twentieth, hundredth*. In compounds only the last word becomes ordinal: *twenty-first, a hundred and second*. In digits — *1st, 2nd, 3rd, 4th*. Multiplicative numeral adverbs are *once, twice*, bookish *thrice*; beyond that periphrastically: *three times*.

**Conditions and exceptions.**
- Spelling: *five → fifth, twelve → twelfth* (v → f), *eight → eighth* (one *t*), *nine → ninth* (no *e*), *twenty → twentieth* (y → ie).
- *Second* is ambiguous: ordinal (*the second day*) and a noun for the unit of time (*a second*), which has no `NumType`.
- Ordinals act as adjectives (*the third book*: ADJ), adverbs (*first, we…*: ADV) and in dates as nouns (*July 3rd*: NOUN) — in all cases `NumType=Ord`.
- Fractions: *a half, a third, two thirds* — mostly NOUN with `NumType=Frac`.
- *Once* meaning "as soon as" (*once you finish*) is the conjunction SCONJ, without `NumType`.

**Examples.** *the **fifth** chapter; she came **first**; July **4th**; I called **twice**; **once** a week.*

**In UD.** Ordinals: ADJ (JJ), ADV (RB) or NOUN with `NumType=Ord` (and `NumForm=Word` or `Combi` for *4th*). *Once, twice*: ADV, RB, `NumType=Mult`. Cardinals: NUM, CD, `NumType=Card`.

**Sources.** Sweet NEG I §1170–1176 (text-1, p. 392–394: *first, second, third; fifth, eighth, ninth, twelfth; -ieth*; only the last member of a compound becomes ordinal; fractions), §1504 (text-1, p. 462: *once, twice, thrice*); Whitney §215–218 (text-1, p. 112–113); Santorini 1990, §2, p. 1 (ordinals — JJ); https://universaldependencies.org/en/feat/NumType.html (Card, Ord, Mult: *once, twice*, Frac); Jespersen MEG VI 24.4₁–24.4₄ (text-5, p. 455–457: *fifth, twelfth, eighth, -tieth*; *two thirds*), 18.1₁ (p. 319–320: *once, twice, thrice* with adverbial *-s*); Kruisinga II.3 §§1693, 1706–1710 (text-4, p. 69, 74–75); Mätzner I, p. 288–289 (text-1, p. 306–307: *fifth, eighth, ninth, twelfth, twentieth; twenty-first*).

```rule
rule: en.morph.ordinal-form
what: NumType=Ord — only a word in -th, -st, -nd, -rd acting as ADJ, ADV or NOUN
match: n[feats.NumType=Ord, !feats.Typo]
require: n[suffix=th|st|nd|rd, upos=ADJ|ADV|NOUN]
severity: warn
source: UD en feat/NumType (Ord: ADJ, ADV, NOUN); Sweet NEG I §1170–1175
```

```rule
rule: en.morph.ordinal-words
what: first, third, fifth… as ADJ or ADV — NumType=Ord
match: n[upos=ADJ|ADV, form=first|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|twentieth|thirtieth|hundredth|thousandth|millionth, !feats.Typo]
require: n[feats.NumType=Ord]
severity: warn
source: UD en feat/NumType (Ord); Sweet NEG I §1170–1175
```

```rule
rule: en.morph.multiplicative-once-twice
what: once, twice, thrice as an adverb — NumType=Mult
match: n[upos=ADV, form=once|twice|thrice, !feats.Typo]
require: n[feats.NumType=Mult]
severity: warn
source: UD en feat/NumType (Mult: once, twice); Sweet NEG I §1504
```

```rule
rule: en.morph.cardinal-numtype
what: a cardinal numeral tagged CD — NumType Card (or Frac for fractions in digits)
match: n[upos=NUM, xpos=CD]
require: n[feats.NumType=Card|Frac]
severity: error
source: UD en feat/NumType (Card: CD)
```
