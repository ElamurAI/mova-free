# Subjunctive: Mood.md requires Inf and Ind, EWT writes Mood=Sub

**Gist.** A note in `Mood.md` says that the subjunctive is not recognized automatically. So the present subjunctive is annotated as an infinitive, and the past one as past indicative. EWT 2.18, however, has 57 `Mood=Sub`, and `Tense.md` itself mentions the subjunctive on VB. Some present subjunctives were left with Inf.

**Guideline.**
- `2.18:https://universaldependencies.org/en/feat/Mood.html:40`: «we currently also mark present subjunctive verbs as [infinitives](VerbForm) and past subjunctive verbs as past indicative verbs»;
- `2.18:https://universaldependencies.org/en/feat/Tense.html:15`: «[Subjunctives](Mood) with the PTB tag `VB` also have this feature» (`Tense=Pres`);
- validator registry: `Mood=Sub` is allowed for AUX and VERB (`tools/data/feats.json`, en; `en/data/ud-registry-en.tsv`).

**EWT 2.18 data** (engine; train 48, dev 3, test 6):
- VB with `Mood=Sub|…|Tense=Pres|VerbForm=Fin` — 49 (VERB 37, AUX 12). Lemma *be* — 11, then *go* 4, *take*, *have*, *get*, *come* 2 each. Person and number come from the subject. Example: *people who are suggesting that we **go** out and fight them* (weblog-blogspot.com_healingiraq_20040409053012_ENG_20040409_053012-0008);
- VBD *were* with `Mood=Sub|Tense=Past` — 8: *If i **were** you* (answers-20111108091921AAaLK4e_ans-0050), *were I to purchase* (reviews-211797-0003). EWT has not a single *were* with a 1st or 3rd person singular subject and `Mood=Ind`;
- VB with `VerbForm=Inf`, with its own subject, without `aux` and without *to* — 54. At least 5 of them are present subjunctives annotated as Inf:
  - *encouraged that the parties **reach*** (weblog-blogspot.com_dakbangla_20050311135387_ENG_20050311_135387-0231);
  - *only fair that the FBI **leak*** (…-0233);
  - *My suggestion is that Global Counterparty **use*** (email-enronsent41_01-0074);
  - *that he bench **test** every part* (reviews-086839-0012);
  - *possible that you … **mark*** (email-enronsent30_02-0030).
  The rest are pseudo-clefts (*All you have to do is **sign***), questions without an auxiliary (*Anyone **know**…?*), *I better **pass***.

**Other 2.18 treebanks:** `Mood=Sub` is present in GUM (27), ParTUT (18), LinES (5). In GENTLE, PUD and LittlePrince — 0.

**Where the discrepancy comes from.** The note in `Mood.md` was not updated when EWT started annotating the subjunctive. The data are not fully annotated: some present subjunctives remained Inf.

**Sources.** `2.18:https://universaldependencies.org/en/feat/Mood.html, `2.18:https://universaldependencies.org/en/feat/Tense.html; seed `en/seeds/verbal/subjunctive.md`; report `data/runs/bones-2026-09-25/verbal.md`, discrepancy 3.

```rule
rule: ud218.subjunctive-present
what: VB with Mood=Sub — Mood.md requires Inf
match: v[xpos=VB, feats.Mood=Sub]
require: v[feats.VerbForm=Inf]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/feat/Mood.html:40
```

```rule
rule: ud218.subjunctive-past
what: VBD with Mood=Sub — Mood.md requires past indicative
match: v[xpos=VBD, feats.Mood=Sub]
require: v[feats.Mood=Ind]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/feat/Mood.html:40
```

```rule
rule: ud218.subjunctive-as-inf
what: VB Inf with its own subject, without aux and to — a subjunctive candidate annotated as Inf
match: v[xpos=VB, feats.VerbForm=Inf]; s[rel~nsubj, head=v]
require: exists a[rel~aux, head=v] or exists m[xpos=TO, head=v]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/feat/Mood.html:40; https://universaldependencies.org/en/feat/Tense.html:15
```
