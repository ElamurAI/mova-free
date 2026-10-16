# `mova` — Mova's dialect

Design note: Mova has its own dialect based on the latest UD at any given time — so future 2.19 changes can already be used — and also based on alternative UD dialects, whose changes are pulled into the dialect when they are judged good.

**What `mova` is** (details in `docs/dialect.md`):
- The internal dialect into which all treebanks and representations are converted: UD versions, the customs of individual treebanks, SUD, SD, classic PTB conversions. The `en` annotator learns on it. From it we convert to any target, and quality is measured on each treebank's gold data through the reverse converter.
- **Base** — UD 2.18 with EWT conventions (registry `en/data/ud-registry-en.tsv`).
- **Layers on top of the base:**
  - `ud-next` — changes after 2.18 that will go into 2.19 (`dialects/ud-next/`);
  - borrowings from alternative dialects;
  - own normalizations: `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked`, FEATS from the subject.
- **Selection rule:** take whatever is more consistent and more informative and can be converted to the standard without loss. A lossy target is only an export target, never the base.

This folder holds the decisions on `ud-next` changes and on discrepancies between the 2.18 guidelines and the EWT data, each as a separate seed in `seeds/`. **Approved by Mova on 26.09.2026:** all proposals in the table — as proposed. Accepted: 11; "wait" — watched until the 2.19 data (freeze 01.11.2026); "do not accept" — no. Principle: changes from other dialects are pulled into this one when they are judged good.

## Decisions

| topic | origin | decision | why, briefly | effect on export |
|---|---|---|---|---|
| [wh-case](seeds/wh-case.md) | ud-next, `_en/pos/PRON.md` | **accept** | case shows the role of *who* in the clause; lemma *who* — as in GUM, GENTLE, LinES, ParTUT | in 2.18 — drop `Case`, lemma from form; identity on EWT |
| [discourse-adverbs](seeds/discourse-adverbs.md) | ud-next `_u-dep/discourse.md`; 2.18 discrepancy | **accept** | the boundary is by word class; 2.18 already excluded ADV | 9 EWT tokens (GUM — 78) will diverge from gold, but not from the guideline |
| [features-context](seeds/features-context.md) | ud-next, `changes.md` No. 18 | **accept** the principle; **wait** on `Exponence` | EWT already does this; form wins over semantic agreement; `Exponence` values do not exist yet | no change |
| [xpos-not-empty](seeds/xpos-not-empty.md) | ud-next, validator 0.2.8 | **accept** | otherwise a 2.19 validator error | writer gate |
| [feat-layer-ids](seeds/feat-layer-ids.md) | ud-next, `feat-layers.md` | **accept** | a layer is only `[a-z]+` | does not apply to en |
| [spoken-subtypes](seeds/spoken-subtypes.md) | ud-next, fr; #1290 | **wait** | no universal decision and no English gold data | the subtype is dropped without loss |
| [no-effect](seeds/no-effect.md) | ud-next: `expl:*`, validator data, tools, treebank list, other languages; 2.18: v1 labels | **accept as is / not applicable**; do not take `uposf→xpos` into the pipeline | no effect on English annotation | — |
| [iobj-sole](seeds/iobj-sole.md) | 2.18 discrepancy | **accept** the 2.12 amendment, as EWT does | `iobj` with *ask/tell/allow X to* is more informative than `obj`; the lexicon decides the boundary | identity |
| [to-phrase-obl](seeds/to-phrase-obl.md) | 2.18 discrepancy | **accept** `obl` | 1285 : 0 in EWT; `nmod` in the text is a v1 leftover | identity |
| [imperative-do](seeds/imperative-do.md) | 2.18 discrepancy | **accept** the EWT custom (`Imp\|Fin`) | the same for *Go!* and *Don't go!*; 98 of 106 | 8 EWT exceptions will diverge; the letter of `VerbForm.md` is derived deterministically |
| [subjunctive](seeds/subjunctive.md) | 2.18 discrepancy | **accept** `Mood=Sub` | a clause feature; 57 in EWT, present in GUM and ParTUT | identity; the letter of `Mood.md` is derived deterministically |
| [modal-mood](seeds/modal-mood.md) | 2.18 discrepancy; #1118 | **wait** | `Mood=Pot/Nec` is outside the en registry; no decision | no change |
| [depictives](seeds/depictives.md) | 2.18 discrepancy | **accept** `advcl` | one answer instead of three; 22 in EWT | identity |
| [clausal-predicate](seeds/clausal-predicate.md) | 2.18 discrepancy | **accept** `nsubj:outer` | 257 in EWT, 0 of the old analysis | identity |

**Summary:**
- accept — 11;
- "as is" or "not applicable" — 5 (in `no-effect`);
- wait — 3: `Exponence`, spoken-language subtypes, `Mood` of modals;
- do not accept — 1: the `uposf→xpos` script in the pipeline.

**Only 3 proposals change `mova` annotation compared with EWT 2.18:**
- *who* pronouns: 24 lemmas, 460 features;
- adverbs with `discourse`: 9 tokens;
- imperatives with `Inf`: 8 tokens.

The rest confirm that `mova` follows the EWT data where the `_en` text is out of date.

Rules for the first two are in `dialects/ud-next/convert.md`. They were run with the `en convert` engine on EWT 2.18: the forward one changes 469 words, the reverse one restores everything except 9 `discourse`. The rule for the third is in `seeds/imperative-do.md`, for `treebanks/en/ewt/convert.md`; the engine changes exactly 8 words with it.

**Outside the decisions, under watch.** #1176: dependents of a nominalized adjective are `nmod`, not `obl`. The Core Group decision has been declared but not yet written into the guidelines. In EWT 2.18, ADJ in nominal position has `nmod` 83 times and `obl` 53. When it lands in `docs/`, it will be one more `ud-next` change with a simple rule `obl → nmod`. Other open discussions are in `dialects/ud-next/README.md`.
