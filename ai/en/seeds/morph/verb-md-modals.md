# Modal verbs: can, may, must, shall, will…

**Gist.** Modal verbs (*can/could, may/might, shall/should, will/would, must, ought*) are defective: they have no *-s* ending in the 3rd person (*he can*, not *he cans*), no infinitive, no *-ing* forms or participles (*to can, canning* are impossible), and always come first in the verb group. Historically *could, might, should, would* are past tense, but in the modern language they are separate words with their own meaning (*you should go* is not past).

**Conditions and exceptions.**
- The PTB tag for all modals is MD; in UD — AUX. *Could, would, should, might* have their **own** lemma (*could → could*), not *can*.
- *Dare* and *need* can be modal (*he need not go*) and lexical (*he needs to go*); *ought* requires *to* (*ought to go*), but is also MD.
- The infinitive after a modal is bare (VB, `VerbForm=Inf`): *can **go***.
- Contracted forms: *'ll, wo* → *will*; *'d* → *would*; *ca* → *can*; *sha* → *shall* (see `verb-clitics`).
- Substitutes for missing forms are periphrastic: *be able to* (instead of *to can*), *have to* (instead of *must* in the past).

**Examples.** *She **can** swim. It **might** rain. You **should** go. We **must** leave. They **ought** to know.*

**In UD.** UPOS AUX; XPOS MD; FEATS — only `VerbForm=Fin` (no tense, mood, person or number); the lemma is the form itself, except contractions.

**Sources.** https://universaldependencies.org/en/pos/AUX_.html (modal auxiliaries: `VerbForm=Fin` as their only feature), `feat/VerbForm.md` (Fin: MD); Santorini 1990, §2, p. 3 (MD: *can, could, dare, may, might, must, ought, shall, should, will, would*), §4.1, p. 17 (*be, do, have* are never MD); Sweet NEG I §1477–1487 (text-1, pp. 450–455: preterite-presents; *must, ought* — former preterites; *need, dare*); Jespersen MEG VI 3.8–3.9 (text-5, p. 38: *shall, will, may, can, must* without *-s*; *he need do it*), 4.8₃, 4.9₂ (pp. 58–60: no infinitive or participle); Kruisinga II.1 §§19, 23 (text-2, pp. 52–55); Whitney §276–278 (text-1, pp. 134–135).

```rule
rule: en.morph.md-feats
what: modal verb — AUX with only VerbForm=Fin
match: m[xpos=MD, upos=AUX|VERB]
require: m[upos=AUX, feats.VerbForm=Fin, !feats.Tense, !feats.Mood, !feats.Person, !feats.Number]
severity: error
source: UD en pos/AUX (modals: VerbForm=Fin only); Santorini 1990, MD
```

```rule
rule: en.morph.modal-is-md
what: auxiliary with a modal lemma — tag MD
match: m[upos=AUX, lemma=can|could|may|might|must|shall|should|will|would|ought, !feats.Typo]
require: m[xpos=MD]
severity: error
source: UD en pos/AUX; Santorini 1990, MD
```
