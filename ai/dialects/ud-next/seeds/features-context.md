# Features from context and `Exponence` in MISC (universal, 2.19)

**Gist.** A new clarification for 2.19 — No. 18, CLARIFICATION, June 2026:
1. Feature values are taken from context when the form itself does not show them, but words of this class usually express the feature. Examples are syncretism and agreement.
2. If the form and the context give different values, as with notional agreement, the form wins.
3. How exactly a feature is expressed — by inflection, context or lemma — a treebank may indicate in MISC with the `Exponence` attribute, for example `Exponence[Number]=Inherent`. The value set is not standardized.
4. The validator will not change.

**Level.** Universal; `changes.md`, type CLARIFICATION, in force from 2.19.

**Before (2.18).** The general rule https://universaldependencies.org/u/overview/morphology.html:112`: a feature value may depend «on the context of the given sentence where the word is used». There was nothing about conflicts between form and context or about `Exponence`.

**Now (snapshot 24.09.2026).** UD docs/changes.md:27` (table row «18 | 2026-Jun | 2.19 | CLARIFICATION | Morphosyntactic Features») and `:46–67`:
- «**Context should be used to disambiguate the feature value when the wordform itself is underspecified with respect to the feature, but that feature is characteristically expressed by words in the same class.**»;
- «(If wordform and context suggest conflicting values of a feature, e.g. with notional agreement, the wordform-based feature is to be preferred.) This was affirmed unanimously by the Core Group.»;
- «We recommend `Exponence` as the name for the MISC attribute, as already piloted in French … We are not prepared to standardize the set of values»;
- «There are currently no plans for validator changes arising from these clarifications.»

**Evidence.** `diff` `2.18:UD docs/changes.md` ↔ UD docs/changes.md`. Discussions:
- #985 — `Exponence`, not in the dump;
- #1233 «When do we need to instantiate a (lexical) morphosyntactic feature?» (Kahane, open, updated 07.07.2026). The examples there are from EWT and GUM: modals with an "inherent" `VerbForm=Fin`; *a, each, every* without `Number`, although demonstratives have it;
- the paper Kahane et al. 2025, `2025.tlt-1.18`.

**What it means for English annotation.** EWT 2.18 already annotates this way, i.e. the clarification legitimizes existing practice:
- a finite verb takes `Number`/`Person` from the subject where the form does not show them. VBD: 3499 `Sing|3`, 1262 `Sing|1`, 1120 `Plur|3`…; VBP: 2515 `Plur|3`, 2440 `Sing|1`, 1192 `Sing|2`…;
- `Case` on *you* and *it* — from position: *you* `Nom` 2096 / `Acc` 651, *it* `Nom` 1492 / `Acc` 784;
- the form wins. VBZ has `Sing|3` 5655 times of 5656; the exception is the token *s* with empty FEATS (newsgroup-groups.google.com_AgeingMonkeys_37131d1864a0b950_ENG_20051114_080100-0001). *Was* is always `Sing`. There is no VBP with `Sing|3` in the basic tree. There is only the EUD empty node 16.1 *have* in answers-20111108082831AAco5PI_ans-0002; the engine does not read empty nodes;
- EWT has no `Exponence`. #1233 remains open: whether to put "lexical" features on *a/each/every* (`Number`) and on modals (`VerbForm`, `Mood`, see `dialects/ud-2.18/seeds/modal-mood.md`).

For `mova` this confirms our own "FEATS from the subject" normalization (`en::ud::agreement`, `train/dialects-and-converters.md`). At the same time it sets a boundary for it: where the form is unambiguous, agreement does not override it (*the team **are*** → `Plur`).

**Registry.** Unchanged (`Exponence` lives in MISC; the FEATS registry does not cover it).

**Sources.** UD docs/changes.md#morphosyntactic-features`; #1233; `2025.tlt-1.18`, section 2.2 («`Exponence` (from 2.19, per #985)»).

```rule
rule: udnext.vbp-not-3sg
what: VBP is never 3rd person singular — in a conflict of form and context the form wins
match: v[xpos=VBP]
require: not v[feats.Number=Sing, feats.Person=3]
severity: error
source: UD docs/changes.md:53–56 (No. 18, 2.19)
```

```rule
rule: udnext.vbz-3sg
what: VBZ — always 3rd person singular, even with notional agreement
match: v[xpos=VBZ, !feats.Typo]
require: v[feats.Number=Sing, feats.Person=3]
severity: error
source: UD docs/changes.md:53–56 (No. 18, 2.19)
```
