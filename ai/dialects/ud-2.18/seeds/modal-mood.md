# Modal MD: VerbForm=Fin without Mood

**Gist.** According to `Mood.md`, mood is a feature of finite forms, and according to `VerbForm.md`, modal MD are finite. EWT gives modals only `VerbForm=Fin`, without `Mood`. The validator issues a `verbform-fin-without-mood` warning for each such token. What to do is being discussed in #1118 (open, 45 comments).

**Guideline.**
- `2.18:https://universaldependencies.org/en/feat/Mood.html:8`: «`Mood` is a feature of [finite](VerbForm) [verbs](en-pos/Verb)»;
- `2.18:https://universaldependencies.org/en/feat/VerbForm.html:12`: «Rule of thumb: if it has non-empty [Mood](Mood), it is finite. … modals with the PTB tag `MD` have this feature»;
- the en FEATS registry allows only `Mood=Imp|Ind|Sub`. In the auxiliary registry (`tools/data/data.json`) the function of modals is recorded as mood: *can, could, may, might, dare* — `Mood=Pot`; *must, should, ought, need* — `Mood=Nec`; *would* — `Mood=Cnd`; *will* — `Tense=Fut`; *shall* — `Mood=Nec,Tense=Fut` (likewise in `en/data/ud-registry-en.tsv`, `aux` lines).

**EWT 2.18 data.** Counted by the engine (rule below). MD with `VerbForm=Fin` without `Mood` — 4049: `VerbForm=Fin` — 4026, another 23 with `Typo`, `Abbr` or `Style=Arch`. One MD has empty FEATS. In GUM (2956), GENTLE, PUD and LittlePrince modals are likewise without `Mood`.

**Validator.** `tools/udtools/src/udtools/level3.py:110–113`: `if node.feats['VerbForm'] == 'Fin' and node.feats['Mood'] == ''` → Warning `verbform-fin-without-mood`. The comment there refers to #1155 («complaints about this warning»). It is a warning, not an error, so it does not block the release.

**Open question.** #1118 (nschneid): should modals get `Mood=Pot`/`Nec`, as the universal `Mood.md` advises? What then about *will*: `Mood=Irr` or `Tense=Fut`? The author leans toward treating this as lexical meaning, not a paradigm cell. #1233 (Kahane) cites modals with `VerbForm=Fin` as an example of a "lexical" feature. No decision yet.

**Sources.** `2.18:https://universaldependencies.org/en/feat/Mood.html, `2.18:https://universaldependencies.org/en/feat/VerbForm.html; `2.18:tools/udtools/src/udtools/level3.py` (unchanged in the snapshot); issues #1118, #1155, #1233.

```rule
rule: ud218.modal-no-mood
what: modal MD with VerbForm=Fin without Mood — Mood.md: every finite form has mood
match: m[xpos=MD, feats.VerbForm=Fin]
require: m[feats.Mood]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/feat/Mood.html:8; https://universaldependencies.org/en/feat/VerbForm.html:12; tools level3.py verbform-fin-without-mood
```
