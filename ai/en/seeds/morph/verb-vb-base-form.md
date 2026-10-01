# The verb base: VB — infinitive, imperative, subjunctive

**Gist.** The bare verb base (*work, go, be*) is the dictionary form. It has the tag VB in three roles: infinitive (*to go, can go, will go*), imperative (*Go!*) and present subjunctive (*I suggest that he go*). In the present indicative the same form gets the tag VBP instead.

**Conditions and exceptions.**
- Infinitive: after *to*, after modals (*can, must, will*), after *do* in questions and negations (*did you go?*), after *let, make, see* (*let him go*).
- Imperative: no subject, at the start of the sentence (*Read the book!*); also *Don't go*, where *do* is VB too.
- Subjunctive: after *suggest, demand, insist that*, in formulas (*God save the Queen*); there is a subject, but no *-s* (*that he **go***).
- *Be* in the infinitive and subjunctive is *be* (*that he **be** told*); *were* in an unreal condition is VBD (see `verb-vbd-past`).

**Examples.** *I want to **leave**. You must **try**. **Make** a sandwich! I suggest that he **see** a doctor.*

**In UD.** UPOS VERB or AUX; XPOS VB; FEATS:
- infinitive — only `VerbForm=Inf`, without tense, mood, person, number;
- imperative — `Mood=Imp|VerbForm=Fin`;
- subjunctive — `Mood=Sub|Tense=Pres|VerbForm=Fin` and `Number`/`Person` from the subject.

EWT 2.18 annotates exactly so (a few dozen subjunctives with `Mood=Sub`), although the guideline text `Mood.md` still says the subjunctive is not distinguished automatically.

**Sources.** https://universaldependencies.org/en/feat/VerbForm.html (Fin: VB without an auxiliary; Inf: VB with an auxiliary, modal or *to*), `feat/Mood.md` (Imp, Sub), `feat/Tense.md` (Pres: VB subjunctives); Santorini 1990, §2, p. 5 (VB: imperative, infinitive, subjunctive), §4.1, p. 21 (VB/VBP); Sweet NEG I §1290 (text-1, p. 423: the base — present, subjunctive, imperative, infinitive); Whitney §234 (text-1, p. 122: subjunctive "almost lost").

```rule
rule: en.morph.vb-verbform
what: VB — either infinitive or finite form (imperative, subjunctive)
match: v[xpos=VB, upos=VERB|AUX]
require: v[feats.VerbForm=Inf|Fin]
severity: error
source: UD en feat/VerbForm (Fin, Inf for VB); Santorini 1990, VB
```

```rule
rule: en.morph.vb-finite-mood
what: finite VB — only imperative or subjunctive
match: v[xpos=VB, feats.VerbForm=Fin]
require: v[feats.Mood=Imp|Sub]
severity: error
source: UD en feat/Mood (Imp, Sub), feat/VerbForm
```

```rule
rule: en.morph.vb-infinitive-bare
what: the infinitive has no tense, mood, person or number
match: v[xpos=VB, feats.VerbForm=Inf]
require: v[!feats.Tense, !feats.Mood, !feats.Person, !feats.Number]
severity: error
source: UD en feat/VerbForm (Inf); EWT 2.18 practice
```
