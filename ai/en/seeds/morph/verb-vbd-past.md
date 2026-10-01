# Past tense: VBD

**Gist.** The simple past of an English verb is one form for all persons and numbers: *I/you/he/they worked*. Regular verbs form it with the ending *-ed*, irregular ones by vowel change or otherwise (*went, sang, kept*). The PTB tag is VBD. The only verb that distinguishes number in the past is *be*: *was* (1st and 3rd person singular) and *were* (the rest).

**Conditions and exceptions.**
- For many verbs the past form coincides with the past participle (*worked, kept, made*); then the tag is chosen by function: without an auxiliary — VBD, after *have* or in the passive — VBN (see `verb-vbn-participle`).
- *Were* in an unreal condition (*if I were rich*) is the past subjunctive: the same tag VBD, but `Mood=Sub`.
- Contracted *'d* is *had* (VBD, lemma *have*) or *would* (MD); see `verb-clitics`.
- In non-standard text the past form stands in place of the participle (*I have went*); EWT then puts VBN by function.

**Examples.** *She **went** home. We **were** late. I**'d** already left* ('d = had). *If I **were** you…*

**In UD.** UPOS VERB or AUX; XPOS VBD; FEATS `Mood=Ind|Tense=Past|VerbForm=Fin` (or `Mood=Sub` for unreal *were*) and, as in EWT 2.18, `Number` and `Person` from the subject (*he wanted* — `Number=Sing|Person=3`). The lemma is the infinitive (*went → go, was → be*).

**Sources.** https://universaldependencies.org/en/feat/Tense.html (Past: all VBD and VBN), `feat/VerbForm.md` (Fin: VBZ, VBD, VBP), `feat/Mood.md` (Sub: *If I were rich*), `feat/Number.md` (*he wanted a cat* — Sing); Santorini 1990, §2, p. 5 (VBD, including conditional *If I were rich*); Sweet NEG I §1290 (text-1, p. 423: *-ed* — both preterite and participle); Jespersen MEG VI 5.6 (text-5, p. 91: *were* — the only preterite distinguishing number); Kruisinga II.1 §28 (text-2: one form in *-ed* — both VBD and VBN; syntax decides).

```rule
rule: en.morph.vbd-feats
what: VBD — finite past-tense verb (indicative or subjunctive)
match: v[xpos=VBD, upos=VERB|AUX]
require: v[feats.Tense=Past, feats.VerbForm=Fin, feats.Mood=Ind|Sub]
severity: error
source: UD en feat/Tense, feat/VerbForm, feat/Mood; Santorini 1990, VBD
```

```rule
rule: en.morph.vbd-agreement-feats
what: VBD in EWT has number and person from the subject
match: v[xpos=VBD, upos=VERB|AUX]
require: v[feats.Number, feats.Person]
severity: warn
source: UD en feat/Number (verbs with a singular subject: he wanted); EWT 2.18 practice
```
