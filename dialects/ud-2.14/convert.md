# Converter mova ↔ ud-2.14

One difference so far: preposition-less nominal adverbials and modifiers. Before UD 2.15, English split them into `:tmod` (time) and `:npmod` (the rest). Since 2.15 this is one subtype `:unmarked` (`obl:unmarked`, `nmod:unmarked`), as in `mova`.

- **ud-2.14 → mova** — a simple merge: four old labels become two, `mova` loses nothing.
- **mova → ud-2.14** — lossy: the choice between tmod and npmod is reconstructed by rules. Rule order matters: each rule sees the state after the previous ones.
  1. **Measure → npmod.** A measure noun immediately before its ADJ/ADV head (*65 years old*, *two days ago*) or immediately before the preposition of its head (*five days before the funeral*).
  2. **Time lexemes → tmod:**
     - days, months, *today*, *tonight*, parts of the day, time units, seasons, holidays, *time*/*times*, *a.m.*/*p.m.*;
     - by lemma; if there are no lemmas (ESLSpok) — by form.
  3. **Year or number in a date → tmod** (*October 1963*, *8, 1963*).
  4. **The rest → npmod.**

Where the rules come from — the English UD documentation: `_en/dep/obl-tmod.md`, `obl-npmod.md`, `nmod-tmod.md`, `nmod-npmod.md`, `nmod-unmarked.md`. Where the documentation is silent — the custom of EWT 2.14, the reference treebank of English UD:
- frequency (*20 times*, *once a week*, *twice a day*) — tmod;
- time zones — mostly npmod.

The rules do not touch DEPS. In a treebank with an enhanced graph, DEPS stay in the source dialect.

## Check: ud-2.14 → mova → ud-2.14 (`en convert-check`, 26.09.2026)

```
en convert-check dialects/ud-2.14 ud-2.14 <files>
```

| data | old labels | forward changed words | restored exactly | reverse divergences |
|---|---:|---:|---:|---|
| ESLSpok 2.18, train + dev + test — the check from the assignment | 151 | 151, DEPREL only | 137 (90.7%) | npmod → tmod ×13, tmod → npmod ×1 |
| EWT r2.14 train — errors were analysed on it | 1475 | 1475, DEPREL only | 1366 (92.6%) | tmod → npmod ×89, npmod → tmod ×20 |
| EWT r2.14 dev + test — held out, rules were not tuned on them | 323 | 323, DEPREL only | 303 (93.8%) | tmod → npmod ×14, npmod → tmod ×6 |

EWT r2.14 was taken from GitHub (tag `r2.14` of the `UD_English-EWT` repo) for checking only. In all runs UPOS, XPOS, FEATS, LEMMA, HEAD, DEPS and MISC have 0 divergences. Comments, multiword tokens and empty nodes — byte for byte.

**mova → ud-2.14 → mova** gives the same file byte for byte on all 2.18 treebanks without old labels: EWT, GUM, GENTLE, GUMReddit, LinES, ParTUT, PUD, CTeTex, Pronouns. ATIS, CHILDES, ESLSpok and LittlePrince still have old labels, so they are not in `mova`, and a divergence there is expected.

**Contribution of the rules** (restored exactly):

| variant | ESLSpok | EWT 2.14 train | EWT 2.14 dev + test |
|---|---:|---:|---:|
| time lexemes only, as in the assignment | 78.1% | 83.3% | 87.6% |
| + measure and dates — **this file** | **90.7%** | **92.6%** | **93.8%** |
| + "per each" → npmod (UD `nmod-npmod.md` (i), *$5 a share*) | 96.0% | 91.2% | 91.6% |

By rule on EWT 2.14 (train / dev + test — how many words the rule changed and what share of them match gold):
- measure before ADJ/ADV: 215 words, 95.8% / 55 words, 92.7%;
- time lexemes by lemma, obl: 97.2% / 96.9%; nmod: 98.0% / 96.9%;
- dates: 18 of 18 / 3 of 3;
- "the rest → npmod": obl 89.6% / 88.1%, nmod 66.9% / 79.2% — the main losses are here.

**ESLSpok — treebank customs, not versions.** 12 of the 14 divergences are frequency and "per each":
- *once a week*, *twice a month*, *six days a week*;
- *many times*, *five times*.

EWT 2.14 annotates such expressions with a time unit as tmod (*times* in train — 34 of 42), ESLSpok as npmod. The "per each" rule would raise ESLSpok to 96%, but would lower the reference EWT. So it belongs to the converter of that treebank (`treebanks/en/eslspok/convert.md`), not of the version. Two more divergences:
- *one day last week* — npmod in ESLSpok, although it is time;
- *the end of the year* — time by meaning, but not a lexeme.

**What cannot be guessed on EWT:**
- *the rest of the day*, *all my life*, *most of the time* — time by meaning, not by lexeme;
- time zones — EWT itself is inconsistent: *PDT* as NOUN — tmod, *CST*, *EDT* as PROPN — npmod;
- duration on a verb (*stayed a week*) — occurs both ways.

## Rules

```convert
rule: ud-2.14.obl-unmarked
what: obl:tmod, obl:npmod → obl:unmarked — since UD 2.15 one subtype for a preposition-less adverbial
from: ud-2.14
to: mova
match: t[rel=obl:tmod|obl:npmod]
set: t[rel=obl:unmarked]
source: UD _en/dep/obl-unmarked.md (History: before 2.15 — obl:tmod and obl:npmod)
```

```convert
rule: ud-2.14.nmod-unmarked
what: nmod:tmod, nmod:npmod → nmod:unmarked
from: ud-2.14
to: mova
match: t[rel=nmod:tmod|nmod:npmod]
set: t[rel=nmod:unmarked]
source: UD _en/dep/nmod-unmarked.md (History: before 2.15 — nmod:tmod and nmod:npmod)
```

Measure — a noun immediately before its ADJ/ADV head (*65 years old*, *two days ago*, *a bit more*).

```convert
rule: ud-2.14.obl-npmod-measure
what: measure before ADJ/ADV → obl:npmod, even if it is a time unit
from: mova
to: ud-2.14
match: t[rel=obl:unmarked, head=h, prev=h]; h[upos=ADJ|ADV]
set: t[rel=obl:npmod]
source: UD _en/dep/obl-npmod.md (i): «a measure phrase … head of an adjectival/adverbial … phrase»
```

```convert
rule: ud-2.14.nmod-npmod-measure
what: the same for nmod
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, head=h, prev=h]; h[upos=ADJ|ADV]
set: t[rel=nmod:npmod]
source: UD _en/dep/obl-npmod.md (i)
```

Measure before a prepositional phrase — the noun is immediately followed by the preposition of its head (*five days before the funeral*).

```convert
rule: ud-2.14.obl-npmod-measure-pp
what: measure before a prepositional phrase → obl:npmod
from: mova
to: ud-2.14
match: t[rel=obl:unmarked, head=h]; h[]; c[rel=case, head=h, next=t]
set: t[rel=obl:npmod]
source: UD _en/dep/obl-npmod.md (i): «… or prepositional phrase»
```

```convert
rule: ud-2.14.nmod-npmod-measure-pp
what: the same for nmod (five days before the funeral → nmod(funeral, days))
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, head=h]; h[]; c[rel=case, head=h, next=t]
set: t[rel=nmod:npmod]
source: UD _en/dep/obl-npmod.md (i); en/seeds/errors/tmod-npmod-unmarked.md (example)
```

Time lexemes — by lemma. *Time* as part of a time-zone name (*Pacific Time*, PROPN) — npmod, as in EWT.

```convert
rule: ud-2.14.obl-tmod-lemma
what: time lexeme → obl:tmod
from: mova
to: ud-2.14
match: t[rel=obl:unmarked, lemma=monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekday|weekend|weeknight|january|february|march|april|may|june|july|august|september|october|november|december|today|tonight|tomorrow|yesterday|someday|sometime|anytime|morning|afternoon|evening|night|noon|midnight|midday|dawn|dusk|daytime|nighttime|a.m.|p.m.|am|pm|second|minute|hour|day|week|fortnight|month|quarter|year|decade|century|millennium|moment|instant|time|while|period|season|semester|spring|summer|autumn|fall|winter|christmas|easter|thanksgiving|halloween|holiday|vacation|everyday|lifetime|era]
unless: t[upos=PROPN, lemma=time]
set: t[rel=obl:tmod]
source: UD _en/dep/obl-tmod.md («if the modifier is specifying a time»); _en/dep/nmod-unmarked.md, Dates (8:00 a.m.); EWT r2.14 (frequency — tmod)
```

```convert
rule: ud-2.14.nmod-tmod-lemma
what: time lexeme → nmod:tmod (any day this week, 8:00 a.m.)
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, lemma=monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekday|weekend|weeknight|january|february|march|april|may|june|july|august|september|october|november|december|today|tonight|tomorrow|yesterday|someday|sometime|anytime|morning|afternoon|evening|night|noon|midnight|midday|dawn|dusk|daytime|nighttime|a.m.|p.m.|am|pm|second|minute|hour|day|week|fortnight|month|quarter|year|decade|century|millennium|moment|instant|time|while|period|season|semester|spring|summer|autumn|fall|winter|christmas|easter|thanksgiving|halloween|holiday|vacation|everyday|lifetime|era]
unless: t[upos=PROPN, lemma=time]
set: t[rel=nmod:tmod]
source: UD _en/dep/nmod-tmod.md; _en/dep/nmod-unmarked.md (i), Dates
```

Without lemmas (ESLSpok: LEMMA is «_») — the same lexemes by form, with plurals.

```convert
rule: ud-2.14.obl-tmod-form
what: time lexeme by form, when there are no lemmas → obl:tmod
from: mova
to: ud-2.14
match: t[rel=obl:unmarked, lemma=_, form=monday|tuesday|wednesday|thursday|friday|saturday|sunday|mondays|tuesdays|wednesdays|thursdays|fridays|saturdays|sundays|weekday|weekdays|weekend|weekends|weeknight|weeknights|january|february|march|april|may|june|july|august|september|october|november|december|today|tonight|tomorrow|yesterday|someday|sometime|anytime|morning|mornings|afternoon|afternoons|evening|evenings|night|nights|noon|midnight|midday|dawn|dusk|daytime|nighttime|a.m.|p.m.|am|pm|second|seconds|minute|minutes|hour|hours|day|days|week|weeks|fortnight|fortnights|month|months|quarter|quarters|year|years|decade|decades|century|centuries|millennium|millennia|moment|moments|instant|time|times|while|period|periods|season|seasons|semester|semesters|spring|springs|summer|summers|autumn|autumns|fall|winter|winters|christmas|easter|thanksgiving|halloween|holiday|holidays|vacation|vacations|everyday|lifetime|era|eras]
set: t[rel=obl:tmod]
source: as ud-2.14.obl-tmod-lemma; forms — plurals of the same lexemes
```

```convert
rule: ud-2.14.nmod-tmod-form
what: time lexeme by form, when there are no lemmas → nmod:tmod
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, lemma=_, form=monday|tuesday|wednesday|thursday|friday|saturday|sunday|mondays|tuesdays|wednesdays|thursdays|fridays|saturdays|sundays|weekday|weekdays|weekend|weekends|weeknight|weeknights|january|february|march|april|may|june|july|august|september|october|november|december|today|tonight|tomorrow|yesterday|someday|sometime|anytime|morning|mornings|afternoon|afternoons|evening|evenings|night|nights|noon|midnight|midday|dawn|dusk|daytime|nighttime|a.m.|p.m.|am|pm|second|seconds|minute|minutes|hour|hours|day|days|week|weeks|fortnight|fortnights|month|months|quarter|quarters|year|years|decade|decades|century|centuries|millennium|millennia|moment|moments|instant|time|times|while|period|periods|season|seasons|semester|semesters|spring|springs|summer|summers|autumn|autumns|fall|winter|winters|christmas|easter|thanksgiving|halloween|holiday|holidays|vacation|vacations|everyday|lifetime|era|eras]
set: t[rel=nmod:tmod]
source: as ud-2.14.nmod-tmod-lemma
```

Date: a number with a month or day name (*October 1963*) or with another number of the date (*8, 1963*).

```convert
rule: ud-2.14.nmod-tmod-date
what: year or date number with a month, weekday or another number → nmod:tmod
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, upos=NUM, head=h]; h[form=january|february|march|april|may|june|july|august|september|october|november|december|jan.|feb.|mar.|apr.|jun.|jul.|aug.|sep.|sept.|oct.|nov.|dec.|monday|tuesday|wednesday|thursday|friday|saturday|sunday]
set: t[rel=nmod:tmod]
source: UD _en/dep/nmod-unmarked.md, Dates: nmod:unmarked(October, 1963), nmod:unmarked(8, 1963) — temporal before 2.15
```

```convert
rule: ud-2.14.nmod-tmod-date-num
what: number with a date number (8, 1963) → nmod:tmod
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked, upos=NUM, head=h]; h[upos=NUM]
set: t[rel=nmod:tmod]
source: UD _en/dep/nmod-unmarked.md, Dates
```

The rest — npmod.

```convert
rule: ud-2.14.obl-npmod
what: remaining obl:unmarked → obl:npmod
from: mova
to: ud-2.14
match: t[rel=obl:unmarked]
set: t[rel=obl:npmod]
source: UD _en/dep/obl-npmod.md; _en/dep/obl-unmarked.md (History)
```

```convert
rule: ud-2.14.nmod-npmod
what: remaining nmod:unmarked → nmod:npmod
from: mova
to: ud-2.14
match: t[rel=nmod:unmarked]
set: t[rel=nmod:npmod]
source: UD _en/dep/nmod-npmod.md; _en/dep/nmod-unmarked.md (History)
```
