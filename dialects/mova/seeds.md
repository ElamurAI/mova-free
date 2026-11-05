# Seeds: dialects/mova

The seeds of this dialect, one section per seed. A section's anchor is the seed name: `seeds.md#clausal-predicate`.

<a id="clausal-predicate"></a>

# Clausal predicate with a copula — `nsubj:outer` — proposal

**Proposal: accept** the analysis of the 2.10 amendment and EWT. Do not take the old analysis from `_en/specific-syntax.md:46–51`, where the copula is the head and the clause is `ccomp` (`dialects/ud-2.18/seeds.md#clausal-predicate`).

**What exactly in `mova`.** *The problem is that these sentences are difficult*: the head is *difficult*, *is* is `cop`, *problem* is `nsubj:outer`, *sentences* is `nsubj`. For a clausal subject of the outer clause — `csubj:outer`.

**Why.**
- **Matches the standard and the data:** EWT 2.18 has 257 `nsubj:outer`/`csubj:outer` and not a single instance of the old analysis. The 7 cases of *be* with `ccomp` are quotative *be like* and *there are hints … that*.
- **More consistent.** The copula stays `cop` in all sentences, with no "transitive copula" exception.
- **Lossless:** export is identity. The old analysis, if it is ever needed for UD v1 or SD conversions, is derived deterministically from `nsubj:outer` + `cop`.

**Check.** `en.verbal.outer-subject-needs-cop` (`en/seeds/verbal/outer-subject.md`), `en.verbal.cop-verb-head-outer` (`en/seeds/verbal/copula-predicate-head.md`); counters `ud218.outer-subject`, `ud218.be-ccomp-head`.

**Origin.** `dialects/ud-2.18/seeds.md#clausal-predicate`; `2.18:docs/changes.md#multiple-subjects`.

Mova decides.

<a id="depictives"></a>

# Depictives — `advcl` — proposal

**Proposal: accept** `advcl` for optional depictives, as in the 2.10 amendment, `_en/dep/advcl.md` and the EWT data. Do not take the outdated `acl` and `advmod` from `_en/specific-syntax.md` (`dialects/ud-2.18/seeds.md#depictives`).

**What exactly in `mova`.** *She entered the room **sad***, *came back **dead***, *came into office **obsessed** with Iraq* — the depictive adjective is `advcl` of the verb. The depictive's predicand (*she*), if visible, goes into EUD, not into the basic tree.

**Why.**
- **One answer instead of three.** `specific-syntax.md` contradicts itself: line 612 says `acl`, line 878 says `advmod`.
- **Matches the data and the standard:** EWT 2.18 has 22 depictives with `advcl`, 0 with `acl`, 0 with `advmod`.
- **More informative than `advmod`.** `advcl` shows that this is secondary predication, not manner.
- **Lossless:** export to 2.18 and to `ud-next` is identity.

**Effect on seeds.** Do not take the examples from `specific-syntax.md:878–890` (*unable*, *assured*) into seeds: in EWT they are annotated differently, as `parataxis` and VERB `advcl`.

**Origin.** `dialects/ud-2.18/seeds.md#depictives`; `2.18:docs/changes.md#optional-depictives`.

Mova decides.

<a id="discourse-adverbs"></a>

# `discourse` only for interjections, particles, numbering and emoticons — proposal

**Proposal: accept** the new definition of `discourse` (`dialects/ud-next/seeds.md#discourse-scope`). Do not introduce MISC marks for the 9 EWT adverbs.

**What exactly in `mova`.**
- ADV is never `discourse`, but `advmod`: *though*, *so*, *also*, *maybe*, *btw*;
- a prepositional phrase is never `discourse`, but `obl` (*in other words*);
- `discourse` remains for INTJ (*oh*, *well*, *like*, *um*, *yes*), item numbering (NUM, *1.*, *(a)*), emoticons (SYM), *thanks*, *ps*.

**Why.**
- **More consistent.** The criterion is the word class, visible without interpreting pragmatics. In EWT *though* is `advmod` 42 times and `discourse` 3; *actually* is `advmod` 83 times out of 83. In GUM *so* is `discourse` 57 times and `advmod` 622.
- **Lossless into the standard.** The 2.18 guideline already excluded adverbs ("non-adverbial discourse markers", `dialects/ud-2.18/seeds.md#discourse-adverbs`). So `advmod` is the correct export both in 2.18 and in `ud-next`.
- **What we pay.** The reverse run of EWT `mova → 2.18` diverges from gold in 9 tokens (train 4, dev 1, test 4), in GUM — 78. These are divergences from gold errors, not from the guideline, so it is not worth preserving them with a MISC mark. In the reverse-run report they should be shown as a separate line.

**Evidence from the data** (EWT 2.18): `discourse` — 1066: INTJ 775, SYM 123, NUM 113, NOUN 27, VERB 12, ADV 9, PROPN 3, ADJ 3, X 1. Prepositional phrases with `discourse` — 0.

**Converters.**
- EWT, GUM → `mova`: rule `mova.ud-next.discourse-adv` (`dialects/ud-next/convert.md`);
- `mova` → 2.18 / `ud-next`: no change;
- DEPS: in EWT `N:discourse` will remain. An action for DEPS or an EUD rebuild is needed (see `convert.md`).

**Check.** Gates `udnext.discourse-not-adv` and `udnext.discourse-not-pp` (error).

**Origin.** `dialects/ud-next/seeds.md#discourse-scope`, `dialects/ud-2.18/seeds.md#discourse-adverbs`.

Mova decides.

<a id="feat-layer-ids"></a>

# Feature layer identifiers — only `[a-z]+` — proposal

**Proposal: accept.** Name `mova`'s own layered features, if any appear (for example, for agreement with several participants in other languages), with lowercase Latin letters only, no digits.

**Why.**
- the post-2.18 guideline requires exactly this (`docs/_u-overview/feat-layers.md:80`);
- the validator also lets digits through (`utils.py:60`, `\[[a-z0-9]+\]`), but Old Georgian has already renamed `sauf2` → `dsauf`. So digits will eventually become an error;
- for English nothing changes now: there are no layers;
- no loss.

**Origin.** `dialects/ud-next/seeds.md#feat-layer-ids`.

Mova decides.

<a id="features-context"></a>

# Features from context, form wins; `Exponence` — proposal

**Proposal:**
- **accept** the principle of clarification No. 18: feature values come from context, and in a conflict the form wins;
- **wait** on `Exponence` in MISC.

**What exactly in `mova`.**
- keep the "FEATS from the subject" normalization (`en::ud::agreement`, `docs/dialect.md`) and give it a boundary: where the form is unambiguous, agreement does not override it. VBZ is always `Sing|3`. VBP is never `Sing|3`: *the team **are*** → `Plur|3`. *Was* is always `Sing`;
- put gates `udnext.vbz-3sg` and `udnext.vbp-not-3sg` on the `en` output and on silver data.

**Why accept.**
- **This is already EWT practice:**
  - VBD and VBP get `Number`/`Person` from the subject (VBD `Sing|3` 3499, VBP `Plur|3` 2515…);
  - *you* and *it* get `Case` by position;
  - VBZ `Sing|3` — 5655 of 5656 (the exception is a token with empty FEATS);
  - there is no VBP `Sing|3` in the basic tree.
- **Lossless:** export to 2.18 changes nothing.
- **What it guards against:** the subject normalization must not override the form under semantic agreement (*the team are*, *a number of people are*). Otherwise it would produce a feature that is not in the standard.

**Why wait on `Exponence`.**
- There is no value set. Clarification No. 18 says directly: "We are not prepared to standardize the set of values without further community experimentation". There is only a pilot in French (`Exponence[Number]=Inherent`).
- #1233 is still open: "lexical" features on *a/each/every* and on modals.
- For English one split would be useful: `Number`/`Person` on VBD/VBP — from context, on VBZ — from the form. But it is computable from XPOS, so there is no need to write it into MISC until there is a standard.

**Check.** `udnext.vbz-3sg`, `udnext.vbp-not-3sg` (`dialects/ud-next/seeds.md#features-context`). On EWT 2.18: 1 and 0 violations.

**Origin.** `dialects/ud-next/seeds.md#features-context`; `docs/changes.md#morphosyntactic-features`; #1233.

Mova decides.

<a id="imperative-do"></a>

# Main verb with imperative *do* — `Mood=Imp|VerbForm=Fin` — proposal

**Proposal: accept** the EWT custom, not the rule of `_en/feat/VerbForm.md` (`dialects/ud-2.18/seeds.md#imperative-do`). Align the 8 EWT exceptions with `VerbForm=Inf` with the EWT → `mova` converter.

**What exactly in `mova`.** In an imperative sentence with *do* (*Don't worry*, *Do not hesitate*, *DO NOT GO HERE*) both verbs — *do* and the main VB — have `Mood=Imp|VerbForm=Fin`.

**Why.**
- **More consistent.** The main verb of an imperative sentence becomes `Imp` regardless of whether *do* is present: *Go!* and *Don't go!* are annotated the same way. By the letter of `VerbForm.md`, *go* in *Don't go* would be `Inf`, and the mood feature would sit only on the function word.
- **More informative.** The sentence type (directive) is visible on the lexical verb, which is the one taking part in the fact. For RAG and `kg` this is a clause-level feature (the UD survey digest «future», section 4, item 4).
- **Matches most of the gold data:** 98 of 106.
- **Lossless into the standard.** The literal `VerbForm.md` form is obtained deterministically: a VB with `aux` *do* that has `Mood=Imp` → `VerbForm=Inf`, remove `Mood`. Export to "EWT 2.18" diverges from gold only in the 8 exceptions.

**Evidence from the data** (EWT 2.18, engine): `Mood=Imp|VerbForm=Fin` — 98, `VerbForm=Inf` — 8 (*Don't cling*, *Don't go!*, *don't let that fool you*, *Do not go there*…). Separately: *Don't* with `Mood=Ind` in imperative sentences — 7; these are EWT errors (report `verbal.md`).

**Rule for the EWT → `mova` converter** (proposal for `treebanks/en/ewt/convert.md`, the `convert` language from `docs/dialect.md`):

```convert
rule: ewt.imperative-do-main
what: main VB with imperative do — Mood=Imp|VerbForm=Fin, as in 98 of 106 EWT cases
from: ewt
to: mova
match: v[xpos=VB, feats.VerbForm=Inf]; d[lemma=do, rel=aux, head=v, feats.Mood=Imp]
set: v[feats+=VerbForm=Fin; feats+=Mood=Imp]
source: EWT 2.18; dialects/mova/seeds.md#imperative-do
```

Checked with the engine (`en convert-check`, 26.09): on EWT 2.18 the rule changes exactly 8 words, FEATS `VerbForm=Inf` → `Mood=Imp|VerbForm=Fin`. There is no reverse rule: the 8 exceptions are not visible from the tree, so the reverse-run report has a separate line for these 8 tokens.

**Check.** Seed `en/seeds/verbal/imperative.md`: `en.verbal.imperative-do-main-verb` (warn) follows EWT, `vb-aux-infinitive` excludes imperative *do*. Counter `ud218.imperative-do-inf`.

**Origin.** `dialects/ud-2.18/seeds.md#imperative-do`; `2.18:docs/_en/feat/VerbForm.md:12`, `:21`.

Mova decides.

<a id="iobj-sole"></a>

# `iobj` without `obj` and with `xcomp` — proposal

**Proposal: accept** the 2.12 "Sole iobj" amendment and the EWT custom, not the text of `_en/dep/iobj.md` and `_en/specific-syntax.md` (`dialects/ud-2.18/seeds.md#iobj-sole`).

**What exactly in `mova`.**
- `iobj` can be the only internal argument (*remind me*, *ask Bush*);
- `iobj` can stand next to `xcomp` when the verb takes an addressee or recipient: *ask/tell/allow/convince/urge/persuade/cause/trust/teach/remind/warn **X** to do*;
- with *let, make, get, have, keep, find, want, help* + `xcomp` — `obj`;
- the boundary between these two classes is set by the lexicon (`en/seeds/lexicon/verb-iobj-licensors.md`, VerbNet), not by examples;
- *permit* and *recommend* have both variants in EWT. `mova` takes whichever the lexicon says once its rule matures, and until then — as in gold.

**Why.**
- **Matches the universal guideline.** The 2.12 amendment is of type AMENDMENT and cancels the condition on which the `_en` text rests.
- **More informative.** `iobj` with *ask X to* distinguishes the addressee from the "causee" `obj` with *make X do*. Merging into `obj`, as `_en` says, would lose this difference.
- **Lossless.** Export to EWT 2.18 is identity. If the literal `_en` text is ever needed (with `xcomp` — only `obj`), the `iobj` → `obj` transition is deterministic. The reverse transition goes through the lexicon.

**Evidence from the data** (EWT 2.18, engine):
- `iobj` — 795; without `obj`/`ccomp` — 243; with `xcomp` — 93;
- the same verb class with `obj` + `xcomp` — 39 (*encourage*, *enable*, *order*, *invite*, *force*, *require*);
- `iobj` is present in GUM (482), LinES (101), ParTUT (33), GENTLE (31).

**Origin.** `dialects/ud-2.18/seeds.md#iobj-sole`; `2.18:docs/changes.md#sole-iobj`; report `$MOVA_DATA/runs/bones-2026-09-25/verbal.md`.

Mova decides.

<a id="modal-mood"></a>

# Modal MD without `Mood` — proposal

**Proposal: wait** for the decision on #1118 and #1155. Until then keep the EWT custom: MD — `VerbForm=Fin` without `Mood`, `Number` and `Person`.

**Options and why not now.**
- **`Mood=Pot`/`Nec`/`Cnd` in FEATS**, as the universal `Mood.md` advises and as auxiliary functions are recorded in `data.json` (*can* `Mood=Pot`, *must* `Mood=Nec`, *would* `Mood=Cnd`):
  - informative;
  - but the en FEATS registry allows only `Imp|Ind|Sub`, so export to 2.18 and to `ud-next` would be invalid;
  - #1118 leans toward treating this as lexical meaning, not a paradigm cell.
- **`Mood=Ind` on modals:** would remove the validator warning, but as a claim of "indicative mood" for *might*, *would* it is false.
- **Remove `VerbForm=Fin`** (#1233: an "inherent" feature) — also undecided.

**Why this does no harm.** The modal meaning in `mova` is visible from the lemma anyway: the auxiliary registry has the function of each of the 16 lemmas. A clause-level feature ("possibility", "necessity") for RAG can be derived from the lemma without writing it into FEATS.

**Evidence from the data.** EWT 2.18: MD with `VerbForm=Fin` without `Mood` — 4049. The same in GUM (2956), GENTLE, PUD and LittlePrince. The validator gives a `verbform-fin-without-mood` warning for each such token, but not an error.

**When to come back.** When #1118 is closed or when `_en/feat/Mood.md` changes in the snapshot. The snapshot-vs-release comparison should be repeated before 01.11.2026.

**Origin.** `dialects/ud-2.18/seeds.md#modal-mood`; #1118, #1155, #1233.

Mova decides.

<a id="no-effect"></a>

# Changes with no effect on `mova` annotation — proposals

None of these changes touches English annotation. The decision is short, and the explanation is in the source seed.

| change | proposal | reason |
|---|---|---|
| [`expl:pass`, `expl:pv` — universal pages](../ud-next/seeds.md#expl-pass-pv-docs) | accept as is | documentation only; English has no such subtypes. One thing is useful for the language-independent layer: this is a common subtype of three language groups |
| [validator data](../ud-next/seeds.md#validator-data) | do nothing; repeat the comparison before 2.19 | no change for en; registry = 2.18 = snapshot of 24.09 |
| [`conllu_convert_uposf_to_xpos.pl`, treebank script](../ud-next/seeds.md#tools-scripts) | do not take into the pipeline | a Perl tool with `Lingua::Interset`, while Mova's mechanics are Rust. As a one-off external cross-check of UPOS+FEATS ↔ PTB pairs (`en::penn`) — only on Mova's say-so. Script (`Latn`) — a line in the `treebanks/` cards |
| [list of English treebanks](../ud-next/seeds.md#en-treebank-list) | accept as is | documentation |
| [other languages and the site](../ud-next/seeds.md#other-languages) | accept as is | not English. For the future language-independent layer, note `flat:redup` (hy, axm) and `expl:rel` (egy) |
| [v1 labels in `specific-syntax.md`](../ud-2.18/seeds.md#v1-labels) | not applicable | `mova` is v2; take examples from this file only with `dobj` → `obj` etc. replaced |

Mova decides.

<a id="spoken-subtypes"></a>

# Subtypes for spoken language (`discourse:filler`, `discourse:tag`, `conj:reform`) — proposal

**Proposal: wait.**

**What this is.** After 2.18, French documents three subtypes for spoken treebanks (`dialects/ud-next/seeds.md#fr-spoken-subtypes`). Universally, these constructions are being discussed by the UniDive spoken-language working group: #1290 (parataxis, tag questions, backchannels), #1280, #1273, #1289. Its fork of the guidelines has not yet been merged into UD.

**Why not now.**
- It would be more informative. The filler *um* is not the interjection *oh*, and a tag question is not ordinary parataxis. For spoken English (ESLSpok, the spoken part of GUM, CHILDES) these are different things.
- It could also be converted to the standard without loss: drop the subtype (`discourse:filler` → `discourse`). The reverse only via a lexicon of fillers.
- But there is no universal decision yet. English gold data do not have these subtypes, so there is nothing to train or measure on. Our own names before agreement with UniDive would only complicate compatibility (the UD survey digest «future», section 4, item 8).
- They are not in the en registry, so export would have to drop them.

**When to come back.** When UniDive merges the spoken-language guidelines into UD or when Mova takes on spoken English treebanks.

**Origin.** `dialects/ud-next/seeds.md#fr-spoken-subtypes`; #1290, #1280, #1273, #1289.

Mova decides.

<a id="subjunctive"></a>

# Subjunctive — `Mood=Sub` — proposal

**Proposal: accept** `Mood=Sub`, as in EWT, GUM and ParTUT, not the note in `_en/feat/Mood.md` about Inf and Ind (`dialects/ud-2.18/seeds.md#subjunctive`).

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

**Origin.** `dialects/ud-2.18/seeds.md#subjunctive`; `2.18:docs/_en/feat/Mood.md:40`, `2.18:docs/_en/feat/Tense.md:15`.

Mova decides.

<a id="to-phrase-obl"></a>

# A *to* phrase on a verb — `obl` — proposal

**Proposal: accept** `obl`, as in EWT and in the universal `obl`/`nmod`. The text of `_en/dep/iobj.md` about `nmod` for *to me* is a UD v1 leftover (`dialects/ud-2.18/seeds.md#iobj-to-phrase`).

**Why.**
- in EWT 2.18 a *to* phrase on a verb is `obl` 1285 times and `nmod` never;
- the 2.18 validator checks the `nmod`/`obl` boundary (`changes.md` No. 17, error `obl-should-be-nmod`), and the data agree with it;
- no loss: export is identity.

**Effect on seeds.** If examples from `_en/dep/iobj.md` get into the `en/seeds` seeds, *to me* there is `obl`.

**Origin.** `dialects/ud-2.18/seeds.md#iobj-to-phrase`.

Mova decides.

<a id="wh-case"></a>

# *who, whom, whose*: case and the lemma *who* — proposal

**Proposal: accept** `ud-next` (`dialects/ud-next/seeds.md#en-wh-case`).

**What exactly in `mova`.**
- lemma *who* for *whom*, *whoever* for *whomever*;
- *who*/*whoever*: `Case=Nom` or `Case=Acc` by syntactic function (rules `mova.ud-next.who-*` in `dialects/ud-next/convert.md`); *whom*/*whomever* — always `Acc`, because the form wins;
- *whose* with a noun (`nmod:poss`) — `Case=Gen|Poss=Yes`; standalone — `Poss=Yes`.

**Why.**
- **More informative.** Case shows the role of *who* in a relative clause or question: *the man **who** I saw* (`Acc`) versus *the man **who** saw me* (`Nom`). For `annot` and RAG this is the role of the "gap" in the relative clause, one of the most valuable clause-level features (the UD survey digest «future», section 4, item 4).
- **More consistent.**
  - Personal pronouns in EWT already have `Case` by position (*you* `Nom` 2096 / `Acc` 651).
  - *Who* pronouns are the only pronouns with case but without `Case`.
  - GUM, GENTLE, LinES and ParTUT already write the lemma *who* for *whom*; EWT, PUD and LittlePrince do not.
- **Lossless into the 2.18 standard.** The reverse rules are deterministic (`ud-next.who-case-drop`, `ud-next.whose-case-drop`, `ud-next.whom-lemma`): drop `Case` and restore the lemma from the form. A round trip of EWT 2.18 gives identity: 24 lemmas, 460 features.
- **The registry** already allows `PRON Case=Nom|Acc|Gen` and `Poss=Yes`. Export to 2.18 and to `ud-next` is valid.

**Evidence from the data** (EWT 2.18, engine):
- *whom* with lemma *whom* — 23, *whomever* — 1;
- *who* pronouns without `Case` — 447 of 447;
- *whose* without `Gen` — 13, all with a noun;
- expected values after the rules: `Acc` 37, `Nom` 410, `Gen` 13 (`dialects/ud-next/convert.md`).

**Risks.**
- `Case` for *who* is taken from the basic function, but in 2 free relatives and in one predicative case — from the position inside the subordinate clause (edge cases are described in `convert.md`).
- Whether EWT 2.19 will do this is not visible from the data (there is no `dev` in the snapshot). If it does not, export to "EWT 2.19" will go through the same reverse rules.

**Check.**
- Gates `udnext.whom-who-acc`, `udnext.who-case`, `udnext.whose-dependent` (error);
- hints `udnext.who-object-acc`, `udnext.who-subject-nom`, `udnext.whose-independent` (warn);
- the existing `en.morph.whom-acc` (`en/seeds/morph/pron-wh.md`).

**Origin.** `dialects/ud-next/seeds.md#en-wh-case`, `dialects/ud-2.18/seeds.md#wh-pronouns`, #517.

Mova decides.

<a id="xpos-not-empty"></a>

# XPOS is never empty — proposal

**Proposal: accept** the validator 0.2.8 rule (`empty-string-in-xpos`) as a CoNLL-U writer gate for all export targets, not only `ud-next`.

**What exactly in `mova`.** An unknown XPOS is written as `_`; an empty string is never written. This mainly concerns future languages with string XPOS (`docs/dialect.md`, section "Training on the metadialect"). In `en`, XPOS is always a PTB tag.

**Why.**
- the 2.19 validator will reject a file with an empty XPOS as a level 2 error;
- for 2.18 `_` is also correct, so there is no loss;
- right now there is no empty XPOS anywhere: EWT, 13 other English 2.18 treebanks and `corpus/mova-*-en.conllu` — 0 lines (grep).

**Check.** This cannot be expressed in the seed rule language: it does not see raw columns. The gate is in the writer code, with a negative control: a line with an empty XPOS must produce an error.

**Origin.** `dialects/ud-next/seeds.md#xpos-not-empty`.

Mova decides.
