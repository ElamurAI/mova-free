# Subjunctive mood

**Gist.** The subjunctive presents an action not as a fact but as a demand, supposition or wish. The present subjunctive is the form without endings even in the third person: *I suggest that he **see** a doctor*, *It is vital that she **be** present*. The past is *were* for all persons: *If I **were** rich…*. So *he see*, *I were* are not agreement errors if they are subjunctive.

**Conditions and exceptions.** The subjunctive after verbs of demand and advice (*suggest, insist, ask that, recommend*) is "mandative"; in conditional and concessive clauses (*if, though, whether*) it has mostly been displaced by the indicative. Brown advised against the subjunctive where the reader would take it for an agreement error (Rule XIV, Note X).

**Examples.** *…suggesting that we **go** out and fight them.* — *Whether such an offer **were** accepted…* — *If it were not so, I would have told you.*

**In UD.** EWT 2.18 marks the subjunctive: `VB` → `Mood=Sub|VerbForm=Fin|Tense=Pres` (with person and number), *were* → `Mood=Sub|Tense=Past`. (An outdated note in `Mood.md` says the subjunctive is not distinguished — EWT 2.18 data already distinguish it.) `VB` with `VerbForm=Fin` is either imperative or subjunctive (rule — `morph/verb-vb-base-form.md`, `en.morph.vb-finite-mood`).

**Sources.** https://universaldependencies.org/en/feat/Mood.html (Sub: *I suggest that he see a doctor*; *If I were rich*); https://universaldependencies.org/en/feat/Tense.html (Subjunctives with VB — Tense=Pres); Brown 1851, Rule XIV, Notes IX–X; Reed & Kellogg, Higher Lessons, Lesson 131 (Subjunctive Mode); Jespersen, MEG IV, ch. IX–X "Imaginative use of Tenses" (text-4).

```rule
rule: en.verbal.subjunctive-present-feats
what: present subjunctive (VB) — Fin and Tense=Pres
match: v[xpos=VB, feats.Mood=Sub]
require: v[feats.VerbForm=Fin, feats.Tense=Pres]
severity: error
source: https://universaldependencies.org/en/feat/Tense.html (Subjunctives with VB have Tense=Pres)
```
