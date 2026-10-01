# Singular in -s: news, mathematics, measles

**Gist.** A few nouns end in *-s* but are grammatically singular: they agree with *is, has, this*. These are *news*, names of sciences and activities in *-ics* (*mathematics, physics, linguistics, economics, politics, athletics*), names of diseases (*measles, mumps*) and games (*billiards, darts*). In English, number is determined by agreement, not by final *-s*.

**Conditions and exceptions.**
- *News* was once plural ("new things"), now only singular: *the news **is** good*. The lemma is *news*, not *new*.
- *Mathematics* follows the Greek neuter plural; *politics, economics, ethics* can be plural meaning "views, practice": *his politics are…*. EWT annotates *politics, economics* as NNS with `Number=Ptan`, *mathematics* — NN, `Sing`.
- *Ethics* as the plural of *ethic* ("work ethics") — an ordinary plural with lemma *ethic*.
- *Series, species, means* — zero plural (see `noun-plural-zero`), not this type.
- *Linguistics is my favorite subject* — an example from the UD guidelines.
- The lemma is the whole form with *-s*: *news, mathematics, physics, measles*.

**Examples.** *The **news** is on. **Linguistics** is my favorite subject. **Measles** is contagious.*

**In UD.** NOUN; *news* — NN, `Number=Sing`; sciences in *-ics* — NN `Sing` or NNS `Ptan` by agreement; lemma = form.

**Sources.** https://universaldependencies.org/en/feat/Number.html (*the news*, *linguistics is my favorite subject* — Sing; "linguistics … singular, … none of these is a pluralia tantum"); Sweet NEG I §998 (text-1, p. 343: *news* — a plural that became singular), §1724 (p. 521: *mathematics* on the model of the Greek plural); Whitney §129 (text-1, p. 74: *news* and *means* "properly plural", now singular; sciences in *-ics* — singular); Santorini 1990, §4.1, pp. 17–18 (*Linguistics/NN is*; *Mechanics/NN is* vs *The mechanics/NNS are*); Jespersen MEG II 5.76–5.78₁ (text-2, p. 187–191: *measles* — singular; sciences in *-ics* tend to the singular; *news* — singular); Kruisinga II.2 §808 (text-3, p. 49), II.3 §§2136–2141 (text-4, p. 321–324).

```rule
rule: en.morph.news-singular
what: news — a singular noun, lemma news
match: n[upos=NOUN, form=news, !feats.Typo]
require: n[xpos=NN, feats.Number=Sing, lemma=news]
severity: error
source: UD en feat/Number (the news — Sing); Sweet NEG I §998; Whitney §129
```

```rule
rule: en.morph.ics-lemma
what: names of sciences and diseases in -s — lemma with -s (mathematics, not mathematic)
match: n[upos=NOUN, form=mathematics|physics|linguistics|economics|politics|athletics|gymnastics|phonetics|genetics|electronics|measles|mumps|billiards, !feats.Typo]
require: n[lemma=mathematics|physics|linguistics|economics|politics|athletics|gymnastics|phonetics|genetics|electronics|measles|mumps|billiards]
severity: error
source: UD en feat/Number; Sweet NEG I §1724; Whitney §129; EWT 2.18 practice
```
