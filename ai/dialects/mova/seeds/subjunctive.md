# Subjunctive — `Mood=Sub` — proposal

**Proposal: accept** `Mood=Sub`, as in EWT, GUM and ParTUT, not the note in `_en/feat/Mood.md` about Inf and Ind (`dialects/ud-2.18/seeds/subjunctive.md`).

**What exactly in `mova`.**
- present subjunctive, a VB without an auxiliary in a clause with its own subject after *suggest/insist/recommend that*, *it is vital that* etc.: `Mood=Sub|Tense=Pres|VerbForm=Fin`, person and number from the subject, as in EWT;
- past subjunctive *were* with a singular subject (*if I were*, *were I to*): `Mood=Sub|Tense=Past`;
- do not automatically fix examples that EWT left with Inf (at least 5). The rule `ud218.subjunctive-as-inf` only suggests candidates for edit mode.

**Why.**
- **More informative.** The subjunctive is a clause-level feature: demand, unreal condition. Exactly such features enrich a document for RAG. By the letter of `Mood.md` they would disappear into `Inf` and `Ind`.
- **More consistent than the letter of the text.** The note itself admits that this is a forced simplification ("no reliable way of identifying subjunctive verbs in an automatic way"). `_en/feat/Tense.md:15` already mentions the subjunctive on VB. The validator allows `Mood=Sub` for AUX and VERB.
- **Lossless into the standard.** The literal `Mood.md` form is obtained deterministically: VB `Sub` → `VerbForm=Inf`, remove `Mood`, `Tense`, `Number`, `Person`; VBD `Sub` → `Mood=Ind`. Export to EWT 2.18 is identity.
- **Caveat.** The reverse transition from `Inf` to `Sub` is never deterministic. So `Sub` must be on the `mova` side, not on the export side.

**Evidence from the data:**
- EWT 2.18: `Mood=Sub` 57 (VB 49, *were* 8); *were* with 1st or 3rd person `Sing` and `Mood=Ind` — 0;
- VB `Inf` with a subject, without `aux` and *to* — 54, of which at least 5 are subjunctives;
- GUM — 27, ParTUT — 18, LinES — 5.

**Check.** `en.verbal.subjunctive-present-feats` (`en/seeds/verbal/subjunctive.md`); counters `ud218.subjunctive-*`.

**Origin.** `dialects/ud-2.18/seeds/subjunctive.md`; `2.18:https://universaldependencies.org/en/feat/Mood.html:40`, `2.18:https://universaldependencies.org/en/feat/Tense.html:15`.

Mova decides.
