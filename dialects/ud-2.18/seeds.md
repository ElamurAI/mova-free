# Seeds: dialects/ud-2.18

The seeds of this dialect, one section per seed. A section's anchor is the seed name: `seeds.md#clausal-predicate`.

<a id="clausal-predicate"></a>

# Clausal predicate with a copula: ccomp in the text, nsubj:outer in the data

**Gist.** In *The problem is that these sentences are difficult* the predicate is a whole clause. `specific-syntax.md` describes the old analysis: the head is *is*, the clause is `ccomp`. The 2.10 amendment and `_en/dep/cop.md` make the predicate of the inner clause the head, and mark the subject of the outer clause `nsubj:outer`. EWT follows the amendment.

**Guideline.**
- `2.18:docs/_en/specific-syntax.md:46–51`: «exceptionally, the copular verb is treated as the predicate, and the clause is given attached to it, with the `ccomp` label», example `ccomp(is, difficult)`;
- `2.18:docs/_en/dep/cop.md:62–79`: `nsubj:outer(tried, problem)`;
- `2.18:docs/changes.md` No. 5 (05.2022, 2.10, AMENDMENT, VALIDATOR) «Multiple Subjects»: the old exception is cancelled.

**EWT 2.18 data** (engine):
- `nsubj:outer` and `csubj:outer` — 257. Only 3 of them are on a head without `cop`: *with conditions as they are*, the ellipsis *probably to get*, *the first thing I do is change*;
- *be* as a head with `ccomp` and a subject — 7. None is the old analysis: these are quotative *be like* (*I was like …*, 5) and *there are hints … that …* (2).

**Where the discrepancy comes from.** The "Copulas" section of `specific-syntax.md` was not updated after 2.10.

**Sources.** `2.18:docs/_en/specific-syntax.md`, `2.18:docs/_en/dep/cop.md`, `2.18:docs/_en/dep/nsubj-outer.md`, `2.18:docs/changes.md#multiple-subjects`; seeds `en/seeds/verbal/outer-subject.md`, `en/seeds/verbal/copula-predicate-head.md`; report `verbal.md`, discrepancy 5.

```rule
rule: ud218.outer-subject
what: nsubj:outer / csubj:outer — the clausal-predicate analysis of the 2.10 amendment
match: p[]; s[rel=nsubj:outer|csubj:outer, head=p]
require: exists c[rel=cop, head=p]
severity: warn
source: UD 2.18 docs/changes.md#multiple-subjects; docs/_en/dep/cop.md:62
```

```rule
rule: ud218.be-ccomp-head
what: be — head with ccomp and a subject (the old analysis of specific-syntax.md:46)
match: b[lemma=be]; x[rel=ccomp, head=b]; s[rel~nsubj, head=b]
require: not b[lemma=be]
severity: warn
source: UD 2.18 docs/_en/specific-syntax.md:46–51
```

<a id="depictives"></a>

# Depictives: three answers in the guidelines, advcl in the data

**Gist.** An optional depictive is an adjective describing a participant during the action (*She entered the room sad*). `specific-syntax.md` gives two outdated answers for it: `acl` and `advmod`. The 2.10 amendment and `_en/dep/advcl.md` say `advcl`. EWT follows the amendment.

**Guideline.**
- `2.18:docs/_en/specific-syntax.md:612`: «_Depictives_ are also represented with the `acl` relation»;
- `2.18:docs/_en/specific-syntax.md:876–878`: «[<!> May be subject to change] Depictives … should be analyzed using the `advmod` relation». Examples: `advmod(stuttering, unable)`, `advmod(rest, assured)`;
- `2.18:docs/_en/dep/advcl.md:49–60` and `2.18:docs/changes.md` No. 4 (05.2022, 2.10, AMENDMENT) «Optional Depictives»: `advcl(entered, sad)`;
- `2.18:docs/_en/dep/acl.md:54`: `acl` «no longer used for optional depictives».

**EWT 2.18 data** (engine):
- ADJ as `advcl` on a VERB — 208. Of these, without `mark`, `cop` and a subject, i.e. depictives — 22: *came back **dead***, *came into office **obsessed** with Iraq*, *died in its sleep **aged** 21*, *feed them … **uncut***, *put it **alone** in a breeding tank*;
- ADJ as `acl` on a VERB — 0;
- ADJ as `advmod` on a VERB — 11, and none is a depictive: *strong-arm*, *effective immediately*, *Worse for Israel*, *Right "mouse" click*;
- the guideline's own examples are annotated differently in EWT: *unable* — `parataxis`, and *assured* (*rest assured*) — a VERB with `advcl`.

**Where the discrepancy comes from.** The `specific-syntax.md` sections were written before the 2.10 amendment and not updated. Line 612 even contradicts line 878 of the same file.

**Sources.** `2.18:docs/_en/specific-syntax.md`, `2.18:docs/_en/dep/advcl.md`, `2.18:docs/_en/dep/acl.md`, `2.18:docs/changes.md#optional-depictives`, `2.18:docs/_u-dep/xcomp.md:202`; report `$MOVA_DATA/runs/bones-2026-09-25/verbal.md`, discrepancy 4.

```rule
rule: ud218.depictive-advcl
what: ADJ depictive as advcl on a verb without mark, cop and subject (2.10 amendment)
match: v[upos=VERB]; a[upos=ADJ, rel=advcl, head=v]
require: exists c[rel=mark|cop|nsubj|nsubj:pass|csubj, head=a]
severity: warn
source: UD 2.18 docs/changes.md#optional-depictives; docs/_en/dep/advcl.md:49
```

```rule
rule: ud218.depictive-advmod
what: ADJ as advmod on a verb — the analysis of specific-syntax.md:878
match: v[upos=VERB]; a[upos=ADJ, rel=advmod, head=v]
require: not a[rel=advmod]
severity: warn
source: UD 2.18 docs/_en/specific-syntax.md:878
```

```rule
rule: ud218.depictive-acl
what: ADJ as acl on a verb — the analysis of specific-syntax.md:612
match: v[upos=VERB]; a[upos=ADJ, rel=acl, head=v]
require: not a[rel=acl]
severity: warn
source: UD 2.18 docs/_en/specific-syntax.md:612
```

<a id="discourse-adverbs"></a>

# discourse on adverbs

**Gist.** Already the 2.18 guideline admits into English `discourse` only non-adverbial discourse markers: *well*, *like*, but not *actually*. EWT has 9 adverbs with `discourse`, and GUM has 78. After 2.18 the exception for adverbs is written out explicitly: `dialects/ud-next/seeds.md#discourse-scope`.

**2.18 guideline.** `_en/dep/discourse.md` refers to the universal page. There, `2.18:docs/_u-dep/discourse.md:11–12`: «interjections (*oh*, *uh-huh*, *Welcome*), fillers (*um*, *ah*), and non-adverbial discourse markers (*well*, *like*, but not *you know* or *actually*)».

**EWT 2.18 data** (engine and grep):
- `discourse` in total — 1066: INTJ 775, SYM 123 (emoticons), NUM 113 (item numbering), NOUN 27 (*thanks*, *ps*), VERB 12 (*mean*, *idk*), **ADV 9**, PROPN 3, ADJ 3, X 1;
- ADV with `discourse` (train 4, dev 1, test 4): *though* ×3, *So* ×2, *Also*, *Maybe*, *btw*, *FTW*. For *though* this is an exception: ADV *though* is `advmod` 42 times and `discourse` 3;
- *actually* — `advmod` 83 times, `discourse` never; *in other words* — `obl` 5 times.

**Other 2.18 treebanks.** GUM: ADV with `discourse` — 78, of which *so* 57, *anyways* 6, *now* 5. At the same time *so* with `advmod` — 622. LinES — 15, ParTUT — 3, LittlePrince — 3, GENTLE — 1.

**Where the discrepancy comes from.** The boundary between a marker and an adverb is drawn by meaning, not by word class. After 2.18 the guideline explicitly gave priority to word class.

**Sources.** `2.18:docs/_u-dep/discourse.md`, `2.18:docs/_en/dep/discourse.md`.

```rule
rule: ud218.discourse-adv
what: ADV with discourse — 2.18 admits only non-adverbial markers
match: d[rel=discourse, upos=ADV]
require: not d[upos=ADV]
severity: warn
source: UD 2.18 docs/_u-dep/discourse.md:11–12
```

<a id="imperative-do"></a>

# Main verb with imperative *do*: Inf in the text, Imp in the data

**Gist.** By the rule of `VerbForm.md`, a VB with an auxiliary has `VerbForm=Inf`. In *Don't go* there is an auxiliary *do*, so *go* should be Inf. In 98 cases of 106 EWT gives the main verb `Mood=Imp|VerbForm=Fin`, just like *Do* itself.

**Guideline.**
- `2.18:docs/_en/feat/VerbForm.md:12`: VB has `Fin` «if they don't have an auxiliary or modal verb attached to it»;
- `2.18:docs/_en/feat/VerbForm.md:21`: VB has `Inf` «if they have an auxiliary or modal verb or the inifinitval _to_ attached to it».

There is no exception for imperative *do*.

**EWT 2.18 data** (engine):
- VB with `aux` *do* that has `Mood=Imp` — 106;
- of them `Mood=Imp|VerbForm=Fin` — 98 (*Don't worry*, *Do not hesitate*, *DO NOT GO HERE!!!*);
- `VerbForm=Inf` — 8. These are the same constructions: *Don't cling* (newsgroup-groups.google.com_magicworld_04c89d43ff4fd6ea_ENG_20050104_152000-0065), *Don't go!* (reviews-263630-0009), *don't let that fool you* (reviews-280663-0008), *Do not go there* (reviews-020992-0012) and others.

Separately: *Don't* with `Mood=Ind` in imperative sentences — 7 (report `verbal.md`, `vb-aux-infinitive`). These are errors on *do* itself, not on the main verb.

**Where the discrepancy comes from.** The text describes a rule by form (an auxiliary is present → Inf). EWT annotates by sentence type: the whole sentence is imperative, so the main verb is too. The 8 cases with Inf are a data inconsistency.

**Sources.** `2.18:docs/_en/feat/VerbForm.md`; seed `en/seeds/verbal/imperative.md` (`en.verbal.imperative-do-main-verb`); report `$MOVA_DATA/runs/bones-2026-09-25/verbal.md`, discrepancy 2.

"Fired" — all VB with imperative *do*; "violations" — those that are not Inf, as the text requires.

```rule
rule: ud218.imperative-do-inf
what: VB with imperative do — VerbForm.md requires Inf, EWT uses Mood=Imp|VerbForm=Fin
match: v[xpos=VB]; d[lemma=do, rel=aux, head=v, feats.Mood=Imp]
require: v[feats.VerbForm=Inf]
severity: warn
source: UD 2.18 docs/_en/feat/VerbForm.md:12, :21
```

<a id="iobj-sole"></a>

# Indirect object without a direct one and with xcomp

**Gist.** The English guideline still says that `iobj` occurs only next to `obj` or `ccomp`, and never with `xcomp`. The universal 2.12 amendment removed this condition, and EWT 2.18 follows the amendment.

**Guideline.**
- `2.18:docs/_en/dep/iobj.md:10`: «It occurs only when there is a `obj` or `ccomp` in the clause.»
- `2.18:docs/_en/specific-syntax.md:141`: «Only when another internal argument is present can `iobj` occur.»
- `2.18:docs/_en/specific-syntax.md:157`: with `xcomp` the nominal object is «uniformly labeled `obj`, and never `iobj`».
- The opposite — `2.18:docs/changes.md` No. 7 (01.2023, 2.12, AMENDMENT) «Sole `iobj`»: the condition of an obligatory `obj` is «hereby removed» (#916).

**EWT 2.18 data** (train+dev+test, 16,622 sentences, engine, rules below):
- `iobj` in total — 795;
- without `obj` and `ccomp` — 243 (31%). Of them 93 are with `xcomp`, the remaining 150 have no other internal argument at all (*reminds me*, *asked Bush*);
- with `xcomp` the custom is lexical:
  - `iobj` — 93: *ask* 25, *allow* 21, *tell* 14, *convince* 7, *urge* 6, *cause* 6, *trust* 3, *persuade* 3, *teach*, *remind*, *warn*, *instruct*…;
  - `obj` — 39 with verbs of the same class: *encourage* 8, *enable* 8, *order* 5, *invite* 5, *force* 4, *require* 4, *beg*;
  - *permit* and *recommend* occur both with `iobj` and with `obj`;
  - ordinary `obj` + `xcomp` (*let, make, get, have, keep, find, want, help*) — 931.

**Where the discrepancy comes from.** The `_en` pages were not updated after the 2.12 amendment. The EWT data match the universal guideline, not the English one.

**Other 2.18 treebanks.** `iobj` is present in all manually annotated ones: GUM 482, LinES 101, ParTUT 33, GENTLE 31.

**Sources.** `2.18:docs/_en/dep/iobj.md`; `2.18:docs/_en/specific-syntax.md`; `2.18:docs/changes.md#sole-iobj`; seeds `en/seeds/lexicon/verb-iobj-licensors.md`, `en/seeds/verbal/indirect-object.md`; report `$MOVA_DATA/runs/bones-2026-09-25/verbal.md`, discrepancy 1.

Counters: "fired" — all `iobj`, "violations" — those that contradict the `_en` text.

```rule
rule: ud218.iobj-sole
what: iobj without obj and ccomp — _en forbids it, the 2.12 amendment allows it
match: v[]; i[rel=iobj, head=v]
require: exists o[rel=obj|ccomp, head=v]
severity: warn
source: UD 2.18 docs/_en/dep/iobj.md:10; docs/changes.md#sole-iobj (2.12)
```

```rule
rule: ud218.iobj-xcomp
what: iobj together with xcomp — specific-syntax.md requires obj
match: v[]; i[rel=iobj, head=v]
require: none x[rel=xcomp, head=v]
severity: warn
source: UD 2.18 docs/_en/specific-syntax.md:157
```

```rule
rule: ud218.iobj-alone
what: iobj without obj, ccomp and xcomp — the only internal argument
match: v[]; i[rel=iobj, head=v]
require: exists o[rel=obj|ccomp|xcomp, head=v]
severity: warn
source: UD 2.18 docs/_en/dep/iobj.md:10; docs/changes.md#sole-iobj (2.12)
```

<a id="iobj-to-phrase"></a>

# A *to* phrase on a verb: nmod in the text, obl in the data

**Gist.** The `iobj` page says that *to me* in *she gave it to me* attaches as `nmod`. This is a UD v1 leftover. In v2 a nominal with a preposition on a verb is `obl`, and EWT writes only `obl`.

**Guideline.** `2.18:docs/_en/dep/iobj.md:17–19`: «the _to me_ part is attached as [nmod]() although semantically it corresponds to the dative». The universal `obl` and `nmod` (rewritten in 2.18, `changes.md` No. 17): a dependent of a verb is `obl`, a dependent of a noun is `nmod`.

**EWT 2.18 data** (engine):
- a phrase with `case` *to* on a VERB: `obl` — 1285, `nmod` — 0.

**Where the discrepancy comes from.** The page text has not been updated since v1.

**Sources.** `2.18:docs/_en/dep/iobj.md`; `2.18:docs/changes.md` No. 17 «`nmod` and `obl`».

```rule
rule: ud218.to-phrase-obl
what: to phrase on a verb as obl — iobj.md says nmod
match: v[upos=VERB]; n[rel~obl, head=v]; c[lemma=to, rel=case, head=n]
require: n[rel~nmod]
severity: warn
source: UD 2.18 docs/_en/dep/iobj.md:18
```

```rule
rule: ud218.to-phrase-nmod
what: to phrase on a verb as nmod — as iobj.md says
match: v[upos=VERB]; n[rel~nmod, head=v]; c[lemma=to, rel=case, head=n]
require: n[rel~obl]
severity: warn
source: UD 2.18 docs/_en/dep/iobj.md:18
```

<a id="modal-mood"></a>

# Modal MD: VerbForm=Fin without Mood

**Gist.** According to `Mood.md`, mood is a feature of finite forms, and according to `VerbForm.md`, modal MD are finite. EWT gives modals only `VerbForm=Fin`, without `Mood`. The validator issues a `verbform-fin-without-mood` warning for each such token. What to do is being discussed in #1118 (open, 45 comments).

**Guideline.**
- `2.18:docs/_en/feat/Mood.md:8`: «`Mood` is a feature of [finite](https://universaldependencies.org/en/feat/VerbForm.html) [verbs](https://universaldependencies.org/en/pos/VERB.html)»;
- `2.18:docs/_en/feat/VerbForm.md:12`: «Rule of thumb: if it has non-empty [Mood](https://universaldependencies.org/en/feat/Mood.html), it is finite. … modals with the PTB tag `MD` have this feature»;
- the en FEATS registry allows only `Mood=Imp|Ind|Sub`. In the auxiliary registry (`tools/data/data.json`) the function of modals is recorded as mood: *can, could, may, might, dare* — `Mood=Pot`; *must, should, ought, need* — `Mood=Nec`; *would* — `Mood=Cnd`; *will* — `Tense=Fut`; *shall* — `Mood=Nec,Tense=Fut` (likewise in `en/data/ud-registry-en.tsv`, `aux` lines).

**EWT 2.18 data.** Counted by the engine (rule below). MD with `VerbForm=Fin` without `Mood` — 4049: `VerbForm=Fin` — 4026, another 23 with `Typo`, `Abbr` or `Style=Arch`. One MD has empty FEATS. In GUM (2956), GENTLE, PUD and LittlePrince modals are likewise without `Mood`.

**Validator.** `tools/udtools/src/udtools/level3.py:110–113`: `if node.feats['VerbForm'] == 'Fin' and node.feats['Mood'] == ''` → Warning `verbform-fin-without-mood`. The comment there refers to #1155 («complaints about this warning»). It is a warning, not an error, so it does not block the release.

**Open question.** #1118 (nschneid): should modals get `Mood=Pot`/`Nec`, as the universal `Mood.md` advises? What then about *will*: `Mood=Irr` or `Tense=Fut`? The author leans toward treating this as lexical meaning, not a paradigm cell. #1233 (Kahane) cites modals with `VerbForm=Fin` as an example of a "lexical" feature. No decision yet.

**Sources.** `2.18:docs/_en/feat/Mood.md`, `2.18:docs/_en/feat/VerbForm.md`; `2.18:tools/udtools/src/udtools/level3.py` (unchanged in the snapshot); issues #1118, #1155, #1233.

```rule
rule: ud218.modal-no-mood
what: modal MD with VerbForm=Fin without Mood — Mood.md: every finite form has mood
match: m[xpos=MD, feats.VerbForm=Fin]
require: m[feats.Mood]
severity: warn
source: UD 2.18 docs/_en/feat/Mood.md:8; docs/_en/feat/VerbForm.md:12; tools level3.py verbform-fin-without-mood
```

<a id="subjunctive"></a>

# Subjunctive: Mood.md requires Inf and Ind, EWT writes Mood=Sub

**Gist.** A note in `Mood.md` says that the subjunctive is not recognized automatically. So the present subjunctive is annotated as an infinitive, and the past one as past indicative. EWT 2.18, however, has 57 `Mood=Sub`, and `Tense.md` itself mentions the subjunctive on VB. Some present subjunctives were left with Inf.

**Guideline.**
- `2.18:docs/_en/feat/Mood.md:40`: «we currently also mark present subjunctive verbs as [infinitives](https://universaldependencies.org/en/feat/VerbForm.html) and past subjunctive verbs as past indicative verbs»;
- `2.18:docs/_en/feat/Tense.md:15`: «[Subjunctives](https://universaldependencies.org/en/feat/Mood.html) with the PTB tag `VB` also have this feature» (`Tense=Pres`);
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

**Sources.** `2.18:docs/_en/feat/Mood.md`, `2.18:docs/_en/feat/Tense.md`; seed `en/seeds/verbal/subjunctive.md`; report `$MOVA_DATA/runs/bones-2026-09-25/verbal.md`, discrepancy 3.

```rule
rule: ud218.subjunctive-present
what: VB with Mood=Sub — Mood.md requires Inf
match: v[xpos=VB, feats.Mood=Sub]
require: v[feats.VerbForm=Inf]
severity: warn
source: UD 2.18 docs/_en/feat/Mood.md:40
```

```rule
rule: ud218.subjunctive-past
what: VBD with Mood=Sub — Mood.md requires past indicative
match: v[xpos=VBD, feats.Mood=Sub]
require: v[feats.Mood=Ind]
severity: warn
source: UD 2.18 docs/_en/feat/Mood.md:40
```

```rule
rule: ud218.subjunctive-as-inf
what: VB Inf with its own subject, without aux and to — a subjunctive candidate annotated as Inf
match: v[xpos=VB, feats.VerbForm=Inf]; s[rel~nsubj, head=v]
require: exists a[rel~aux, head=v] or exists m[xpos=TO, head=v]
severity: warn
source: UD 2.18 docs/_en/feat/Mood.md:40; docs/_en/feat/Tense.md:15
```

<a id="v1-labels"></a>

# UD v1 labels in the examples of `specific-syntax.md`

**Gist.** The examples in `specific-syntax.md` are still written with UD v1 labels: `dobj`, `nsubjpass`, `auxpass`, `csubjpass`, `neg`, `mwe`. They do not exist in v2, the validator rejects them, so there are 0 of them in the EWT data. The analyses themselves are correct; only the label names are outdated.

**Guideline.** In the `sdparse` blocks of `2.18:docs/_en/specific-syntax.md` there are 36 lines with v1 labels (grep): `dobj` 20, `mwe` 5, `neg` 4, `auxpass` 3, `nsubjpass` 3, `csubjpass` 1. The text also has v1 names, for example `:96` (*nsubjpass*, *csubjpass*).

**v1 → v2 mapping** (`docs/_v2/summary.md:74–100`):
- `dobj` → `obj`;
- `nsubjpass` → `nsubj:pass`, `csubjpass` → `csubj:pass`;
- `auxpass` → `aux:pass`;
- `neg` → `advmod` (in English — with `Polarity=Neg` on *not*);
- `mwe` → `fixed`.

**EWT 2.18 data.** Not a single v1 label (the registry `en/data/ud-registry-en.tsv` does not have them; the `en` reader does not know them).

**What this means for Mova.** Take examples from `specific-syntax.md` into seeds only with this replacement. They cannot serve as a corpus for checking.

**Sources.** `2.18:docs/_en/specific-syntax.md`; `docs/_v2/summary.md`; `docs/workgroups/v1_to_v2.md`.

<a id="wh-pronouns"></a>

# The pronouns *who, whom, whose* in 2.18: TODO in the guideline

**Gist.** In 2.18 `PRON.md` does not yet have a decision on the case of *who* pronouns: there are two TODOs. The first is to mark the case of *whom(ever)* and lemmatize *whom* as *who*. The second is to add *whose* `Poss=Yes` and possibly `Case=Gen`. The EWT 2.18 data already partly fulfil the second TODO (`Poss=Yes`), but not the first. After 2.18 the guideline was completed: `dialects/ud-next/seeds.md#en-wh-case`.

**2.18 guideline.**
- `2.18:docs/_en/pos/PRON.md:48`: «[PronType]() is the only feature except where shown below»;
- `:52`: *who, whom* — WP; *whoever, whosoever, whomever* — WP; *whose* — WP$;
- `:66`: «TODO: tag *whom(ever)* for case; lemmatize *whom* as *who* and *whomever* as *whoever*»;
- `:68`: «TODO: add Poss=Yes and possibly Case=Gen for *whose*»;
- `:60`: *whosoever* has `Style=Form`. But in the validator registry for en the `Style` values are only `Arch, Coll, Expr, Slng, Vrnc`, so the validator will not accept `Style=Form`. EWT has no *whosoever*.

**EWT 2.18 data** (engine and grep):
- *who*: 421 (Rel 361, Int 60), without `Case`;
- *whom*: 23 with lemma *whom* (Rel 20, Int 3), without `Case`; *whomever*: 1 with lemma *whomever*;
- *whose*: 13, all `Poss=Yes|PronType=Rel`, without `Case=Gen`;
- in total *who, whom, whoever, whomever* without `Case` — 447 of 447.

**Other 2.18 treebanks.** The lemma *who* for *whom* is already written by GUM (16), GENTLE (6), LinES (15) and ParTUT (4). EWT, PUD and LittlePrince keep the lemma *whom*. No English treebank writes `Case` on *who* pronouns.

**Where the discrepancy comes from.** The 2.18 guideline acknowledged an open question (TODO, see #517 «Lemmas of English personal pronouns», closed 19.11.2025). EWT 2.18 reflects the state before the decision.

**Sources.** `2.18:docs/_en/pos/PRON.md`; `tools/data/feats.json` (en, Style); issue #517; seeds `en/seeds/morph/pron-wh.md` (`en.morph.whom-acc`), `en/seeds/nominal/who-whom-whose.md`; report `$MOVA_DATA/runs/bones-2026-09-25/morph.md`.

```rule
rule: ud218.whom-lemma
what: whom/whomever with its own lemma — the 2.18 TODO advises the lemma who/whoever
match: w[upos=PRON, form=whom|whomever]
require: w[lemma=who|whoever]
severity: warn
source: UD 2.18 docs/_en/pos/PRON.md:66
```

```rule
rule: ud218.who-case
what: who pronoun without Case — 2.18 TODO
match: w[upos=PRON, form=who|whom|whoever|whomever]
require: w[feats.Case]
severity: warn
source: UD 2.18 docs/_en/pos/PRON.md:66
```

```rule
rule: ud218.whose-poss
what: whose with Poss=Yes — 2.18 TODO, already present in the data
match: w[upos=PRON, form=whose]
require: w[feats.Poss=Yes]
severity: warn
source: UD 2.18 docs/_en/pos/PRON.md:68
```
