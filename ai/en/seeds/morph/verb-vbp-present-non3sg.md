# Present tense without -s: VBP

**Gist.** In the English present tense a verb has only two forms: with the ending *-s* for the third person singular (*she works*) and the bare base for all other persons (*I work, you work, we work, they work*). The PTB tag for the bare base in the present is VBP. So VBP is always a finite present indicative verb, but **not** 3rd person singular.

**Conditions and exceptions.**
- The same bare base can also be an infinitive, imperative and subjunctive (*to work, Work!, that he work*) — then the tag is VB, not VBP (see seed `verb-vb-base-form`).
- The verb *be* has separate present forms: *am* (1st person singular), *are* (the rest, except 3rd singular), *is* (3rd singular, VBZ). *Am* and *are* (and also *'m*, *'re*) are VBP.
- Modal verbs (*can, must, will*) have no endings, but they are MD, not VBP.
- Non-standard text has *he don't*, *it taste good*: the tag follows the form (VBP), and the features follow the subject. This is an error of the text, not of the annotation; the rule below does not catch such cases, since it looks only at the features of the word itself.

**Examples.** *I **know**. They **are** here. We **have** time. I**'m** late.*

**In UD.** UPOS VERB or AUX; XPOS VBP; FEATS `Mood=Ind|Tense=Pres|VerbForm=Fin` plus `Number` and `Person` from the subject: *I know* — `Number=Sing|Person=1`, *you know* — `Number=Sing|Person=2` or `Plur`, *they know* — `Number=Plur|Person=3`. The combination `Person=3|Number=Sing` is impossible in VBP: such a subject requires *-s* (VBZ).

**Sources.** https://universaldependencies.org/en/feat/Tense.html (Pres: all VBP and VBZ), `feat/VerbForm.md` (Fin: VBZ, VBD, VBP, MD), `feat/Person.md`, `feat/Number.md`; Santorini 1990, §2, p. 5 (VBP) and §4.1, p. 21 (VB/VBP test: substitute a 3rd person subject — if *-s* appears, it is VBP); Sweet NEG I §1290 (text-1, p. 423: the base — present tense, except 3rd person singular); Whitney §229–230 (text-1, pp. 120–121).

```rule
rule: en.morph.vbp-feats
what: VBP — finite present indicative verb
match: v[xpos=VBP, upos=VERB|AUX]
require: v[feats.Tense=Pres, feats.VerbForm=Fin, feats.Mood=Ind]
severity: error
source: UD en feat/Tense, feat/VerbForm, feat/Mood; Santorini 1990, VBP
```

```rule
rule: en.morph.vbp-not-3sg
what: VBP is never 3rd person singular — that has the -s form (VBZ)
match: v[xpos=VBP, upos=VERB|AUX]
require: not v[feats.Person=3, feats.Number=Sing]
severity: error
source: UD en feat/Person (Person=3 for verbs — VBZ); Santorini 1990, VBP vs VBZ
```
