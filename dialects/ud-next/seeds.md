# Seeds: dialects/ud-next

The seeds of this dialect, one section per seed. A section's anchor is the seed name: `seeds.md#discourse-scope`.

<a id="discourse-scope"></a>

# discourse: boundary by word class (universal, with an English example)

**Gist.** After 2.18 the definition of `discourse` was rewritten. Now these are discourse expressions with grammatical properties that distinguish them from ordinary syntagmatic relations. The central cases are interjections, particles, item numbering, emoticons. Each language decides itself what belongs here, and for English it is said explicitly: adverbs (*actually*) and prepositional phrases (*in other words*) are not `discourse` but `advmod` and `obl`, even when they work as discourse markers.

**Level.** Universal, `_u-dep/discourse.md`. The English `_en/dep/discourse.md` only refers to this page, so for en this is a direct guideline change.

**Before (2.18).** `2.18:docs/_u-dep/discourse.md:8–14`: interjections and other discourse particles, «not clearly linked to the structure of the sentence, except in an expressive way». For English — «non-adverbial discourse markers (*well*, *like*, but not *you know* or *actually*)» and item numbering.

**Now (snapshot 24.09.2026).** `docs/_u-dep/discourse.md`:
- `:8–10` — expressions «with grammatical properties that distinguish them from expressions appearing in typical syntagmatic relations (like advmod, obl, advcl, parataxis, vocative, etc.)»;
- `:11–19` — a list:
  - interjections: expressive (*oh*, *Welcome*), backchannel (*yes*, *uh-huh*), fillers (*um*);
  - particles that cannot be classed as adverbs or prepositions: interrogative, vocative;
  - item numbering;
  - emoji and emoticons with an extra-syntactic expressive function;
- `:25–31` — «`discourse` is reserved as a syntactic function for constructions signaling discourse meaning in grammatically distinctive ways … In English, for example, we exclude from `discourse` any items from the lexical class of adverbs (_actually_) and prepositional phrases (_in other words_); these are simply [advmod]() and [obl]()».

The examples *well*, *like*, *you know* were removed from the text.

**Evidence.** `diff` `2.18:docs/_u-dep/discourse.md` ↔ `docs/_u-dep/discourse.md`. There is no entry in `changes.md`: it counts as a minor clarification («Many minor clarifications are not listed»). There is no separate issue in the dump.

**What it means for English annotation.** The criterion is now word class, not meaning:
- ADV with `discourse` → `advmod`. EWT 2.18 has 9 of them (train 4, dev 1, test 4), GUM — 78 (*so* 57);
- a prepositional phrase with `discourse` → `obl`. In EWT 0: *in other words* — `obl` 5 of 5;
- emoticons with `discourse` are now named in the guideline: EWT has SYM with `discourse` 123 (`:)` 59, `:(` 9, `:-)` 8…);
- interjections (INTJ 775) and numbering (NUM 113) — unchanged;
- *like* and *well* as INTJ with `discourse` (27 and 58 in EWT) remain: they are interjections, not adverbs.

**Registry.** Unchanged: `discourse` is allowed; the registry does not check UPOS × relation pairs.

**Sources.** `docs/_u-dep/discourse.md`, `2.18:docs/_u-dep/discourse.md`, `docs/_en/dep/discourse.md`; `dialects/ud-2.18/seeds.md#discourse-adverbs`.

```rule
rule: udnext.discourse-not-adv
what: discourse is not for adverbs — in English ADV stays advmod
match: d[rel=discourse, upos=ADV]
require: not d[rel=discourse]
severity: error
source: UD docs/_u-dep/discourse.md:25–31 (snapshot 24.09.2026)
```

```rule
rule: udnext.discourse-not-pp
what: discourse is not for prepositional phrases — they are obl
match: d[rel=discourse]; c[rel=case, head=d]
require: not d[rel=discourse]
severity: error
source: UD docs/_u-dep/discourse.md:28–31 (snapshot 24.09.2026)
```

<a id="en-treebank-list"></a>

# The English page: 13 active treebanks instead of 10 (English, documentation)

**Gist.** The main English page now counts 13 active treebanks: CHILDES, CTeTex and LittlePrince, released in 2.17–2.18, were added.

**Level.** English, documentation. Does not affect annotation.

**Before (2.18).** `2.18:docs/_en/index.md:50`: «There are [ten](https://universaldependencies.org/treebanks/en-comparison.html) active English UD treebanks».

**Now (snapshot 24.09.2026).** `docs/_en/index.md:50–61`: «thirteen»; English-CHILDES, English-CTeTex, English-LittlePrince were added to the list.

**Evidence.** `diff` of the two files; PR #1297 «Update number and list of active English UD treebanks» (closed 01.09.2026).

**What it means for English annotation.** Nothing. The 2.18 release has 14 English folders: Pronouns is added to the 13 active ones (see `$MOVA_DATA/raw/multi-ud/SOURCES.md`). Treebank facts are in `treebanks/cards.tsv`.

**Registry.** Not applicable.

**Sources.** `docs/_en/index.md`; #1297.

<a id="en-wh-case"></a>

# *who, whom, whose*: case and the lemma *who* (English)

**Gist.** After 2.18 the English `PRON.md` closed two TODOs. *Who* pronouns got the `Case` feature: *who* — `Nom` or `Acc` by context, *whom* — `Acc` with lemma *who*, dependent *whose* — `Case=Gen|Poss=Yes`, standalone *whose* — only `Poss=Yes`. Likewise *whoever*, *whomever* (lemma *whoever*) and the new form *whosever*.

**Level.** English, `_en/pos/PRON.md`. A guideline, not the validator: the feature registry for en did not change.

**Before (2.18).** `2.18:docs/_en/pos/PRON.md:48` — «[PronType]() is the only feature except where shown below». TODO at `:66` (*whom(ever)*: case and lemma *who*) and `:68` (*whose*: `Poss=Yes`, possibly `Case=Gen`). Details — `dialects/ud-2.18/seeds.md#wh-pronouns`.

**Now (snapshot 24.09.2026).**
- `docs/_en/pos/PRON.md:48`: «`Case` is marked only for _who_ and variants»;
- `:50–54` — a table:

  | | `Case=Nom` | `Case=Acc` | dependent possessive: `Case=Gen`, `Poss=Yes` | independent possessive: `Poss=Yes` |
  |---|---|---|---|---|
  | `PronType=Int` or `Rel` | who | whom [who], who | whose | whose |
  | `PronType=Int` or `Rel` | whoever, *whosoever* | whomever [whoever], whoever | whosever | — |
  | XPOS | WP | WP | WP$ | WP$ |

- `:56`: «_who_ and _whoever_ may bear either value of `Case`, depending on context. The variant _whosoever_ receives `Style=Form`»;
- the TODOs about *whom* and *whose* were removed; TODOs remain about *whatsoever*, *whatever* as DET and exclamative *what* (`:68–72`).

**Evidence.** `diff` `2.18:docs/_en/pos/PRON.md` ↔ `docs/_en/pos/PRON.md`: lines 48–69. Context — #517 «Lemmas of English personal pronouns» (closed 19.11.2025, milestone v2.18): «I am not sure why *whom*, *whomever* … aren't normalized to nominative». There is no separate issue about the change itself in the dump. Treebank branches are not in the snapshot, so whether EWT `dev` has already reworked this is unknown.

**What it means for English annotation.**
- lemma *who* for all forms, as in GUM, GENTLE, LinES and ParTUT. EWT 2.18 has lemma *whom* (23);
- case in FEATS for 447 EWT tokens: *who* 422 (one of them a typo with lemma *whoever*), *whom* 23, *whoever* 1, *whomever* 1;
- on *whose* (13) — `Case=Gen`: all 13 in EWT are `nmod:poss`, i.e. dependent;
- `Case` for *who* is taken from the syntactic function. This agrees with the 2.19 clarification on features from context (`features-context.md`). The form *whom* gives `Acc` even in subject position, because the form weighs more.

**Registry.** No new pairs needed: `feat PRON Case=Nom|Acc|Gen` and `feat PRON Poss=Yes` are already in `en/data/ud-registry-en.tsv`. Only `Style=Form` for *whosoever* is missing, but that was so in 2.18 too, and EWT has no such form.

**Sources.** `docs/_en/pos/PRON.md`, `2.18:docs/_en/pos/PRON.md`; #517, #1009 (independent possessives); seed `en/seeds/morph/pron-wh.md` (`en.morph.whom-acc` is already written according to this guideline).

EWT 2.18 does not conform to these rules, and that is the negative control: the error rules give 24 (*whom*) + 423 (*who* without `Case`) + 13 (*whose*) violations, the warn rules 7 + 6 + 396. After the `mova → ud-next` converter (`../convert.md`) — 0, checked with the engine on 26.09.

```rule
rule: udnext.whom-who-acc
what: whom/whomever — lemma who/whoever and Case=Acc
match: w[upos=PRON, form=whom|whomever]
require: w[lemma=who|whoever, feats.Case=Acc]
severity: error
source: UD docs/_en/pos/PRON.md:52–53 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-case
what: who/whoever always has Case — Nom or Acc by context
match: w[upos=PRON, form=who|whoever]
require: w[feats.Case=Nom|Acc]
severity: error
source: UD docs/_en/pos/PRON.md:48, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-object-acc
what: who/whoever as an object — Case=Acc
match: w[upos=PRON, form=who|whoever, rel=obj|iobj]
require: w[feats.Case=Acc]
severity: warn
source: UD docs/_en/pos/PRON.md:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-oblique-acc
what: who/whoever as a prepositional object (who … to, who … with) — Case=Acc; the head of a free relative is left alone
match: w[upos=PRON, form=who|whoever, rel=obl|obl:agent|nmod]
require: w[feats.Case=Acc]
unless: exists r[rel=acl:relcl, head=w]
severity: warn
source: UD docs/_en/pos/PRON.md:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-subject-nom
what: who/whoever as a subject — Case=Nom
match: w[upos=PRON, form=who|whoever, rel~nsubj]
require: w[feats.Case=Nom]
severity: warn
source: UD docs/_en/pos/PRON.md:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.whose-dependent
what: whose with a noun — Case=Gen and Poss=Yes
match: w[upos=PRON, form=whose, rel=nmod:poss]
require: w[feats.Case=Gen, feats.Poss=Yes]
severity: error
source: UD docs/_en/pos/PRON.md:52 (snapshot 24.09.2026)
```

```rule
rule: udnext.whose-independent
what: standalone whose — Poss=Yes without Case
match: w[upos=PRON, form=whose, rel!=nmod:poss]
require: w[feats.Poss=Yes, !feats.Case]
severity: warn
source: UD docs/_en/pos/PRON.md:52 (snapshot 24.09.2026)
```

<a id="expl-pass-pv-docs"></a>

# `expl:pass` and `expl:pv`: pages changed from Czech to universal (documentation)

**Gist.** The universal pages `expl:pass` (reflexive passive) and `expl:pv` (reflexive verbs) used to describe Czech: they had links to `cs-feat/Reflex`, the PDT label `AuxR` and history from UD 1.2 and 2.0. Now they describe Slavic, Romance and Germanic languages, link to the universal `Reflex` and have Czech examples with an English translation and a parallel analysis. The annotation rule did not change.

**Level.** Universal, documentation.

**Before (2.18).** `2.18:docs/_u-dep/expl-pass.md:8–12`: «Reflexive pronouns (see the feature [cs-feat/Reflex]()) are used in various constructions in Czech … In PDT, their relation to the verb is labeled `AuxR`». `2.18:docs/_u-dep/expl-pv.md:8–16` — likewise, with the history of the labels `auxpass:reflex` and `compound:reflex`.

**Now (snapshot 24.09.2026).** `docs/_u-dep/expl-pass.md:8–11`: «used in various constructions in Slavic, Romance and Germanic languages, including so-called _reflexive passive,_ which in UD uses the relation subtype `expl:pass`». `docs/_u-dep/expl-pv.md:8–15`: «inherently reflexive (also called pronominal verbs in some grammatical traditions)».

**Evidence.** `diff` of both files.

**What it means for English annotation.** Nothing: the en registry has no `expl:pass` and `expl:pv`, English has only `expl`. For `mova` as a language-independent layer one thing is useful: these subtypes are described as common to three language groups, not as a Czech custom.

**Registry.** Not applicable.

**Sources.** `docs/_u-dep/expl-pass.md`, `docs/_u-dep/expl-pv.md`.

<a id="feat-layer-ids"></a>

# Feature layer identifiers: from advice to rule (universal)

**Gist.** Layered features such as `Number[psor]` have a layer identifier. Lowercase Latin letters in it used to be only recommended; now they are required. The trigger seems to have been Old Georgian: after 2.18 its layers `sauf2` and `stack2` were renamed to `dsauf` and `stackb`. The validator still lets digits through.

**Level.** Universal, `_u-overview/feat-layers.md`. A guideline; the validator was not changed.

**Before (2.18).** `2.18:docs/_u-overview/feat-layers.md:80`: «We recommend that the layer identifiers consist of lowercase English letters».

**Now (snapshot 24.09.2026).**
- `docs/_u-overview/feat-layers.md:80`: «The layer identifiers consist of lowercase English letters». The other change in the file is a removed trailing space on line 261;
- `docs/_oge/feat/`: new pages `Case-dsauf.md`, `Number-dsauf.md`, `Case-stackb.md`, `Case-sauf.md`, `Number-sauf.md`. The text still has the old names: `Case-stackb.md:14` — `Case[stack2]`, `Number-dsauf.md:15` — `Number[sauf2]`. About the pages themselves — PRs #1258–#1266 (June 2026);
- validator: `tools/udtools/src/udtools/utils.py:60`, layer — `\[[a-z0-9]+\]`, i.e. digits are allowed. `tools/data/feats.json` has no layers with digits.

**Evidence.** `diff` of `feat-layers.md`; the listing of `_oge/feat/`; `utils.py`. In parallel the site renamed pages: `Number[io].md` → `Number-io.md` (`_ka`, `_oge`). This is infrastructure, not annotation.

**What it means for English annotation.** Nothing: EWT and the en registry have no layered features. For `mova` — a constraint on future own layers: the identifier is only `[a-z]+`, no digits, even though the validator would accept digits.

**Registry.** Not applicable.

**Sources.** `docs/_u-overview/feat-layers.md`, `2.18:docs/_u-overview/feat-layers.md`, `docs/_oge/feat/*`, `tools/udtools/src/udtools/utils.py`.

<a id="features-context"></a>

# Features from context and `Exponence` in MISC (universal, 2.19)

**Gist.** A new clarification for 2.19 — No. 18, CLARIFICATION, June 2026:
1. Feature values are taken from context when the form itself does not show them, but words of this class usually express the feature. Examples are syncretism and agreement.
2. If the form and the context give different values, as with notional agreement, the form wins.
3. How exactly a feature is expressed — by inflection, context or lemma — a treebank may indicate in MISC with the `Exponence` attribute, for example `Exponence[Number]=Inherent`. The value set is not standardized.
4. The validator will not change.

**Level.** Universal; `changes.md`, type CLARIFICATION, in force from 2.19.

**Before (2.18).** The general rule `docs/_u-overview/morphology.md:112`: a feature value may depend «on the context of the given sentence where the word is used». There was nothing about conflicts between form and context or about `Exponence`.

**Now (snapshot 24.09.2026).** `docs/changes.md:27` (table row «18 | 2026-Jun | 2.19 | CLARIFICATION | Morphosyntactic Features») and `:46–67`:
- «**Context should be used to disambiguate the feature value when the wordform itself is underspecified with respect to the feature, but that feature is characteristically expressed by words in the same class.**»;
- «(If wordform and context suggest conflicting values of a feature, e.g. with notional agreement, the wordform-based feature is to be preferred.) This was affirmed unanimously by the Core Group.»;
- «We recommend `Exponence` as the name for the MISC attribute, as already piloted in French … We are not prepared to standardize the set of values»;
- «There are currently no plans for validator changes arising from these clarifications.»

**Evidence.** `diff` `2.18:docs/changes.md` ↔ `docs/changes.md`. Discussions:
- #985 — `Exponence`, not in the dump;
- #1233 «When do we need to instantiate a (lexical) morphosyntactic feature?» (Kahane, open, updated 07.07.2026). The examples there are from EWT and GUM: modals with an "inherent" `VerbForm=Fin`; *a, each, every* without `Number`, although demonstratives have it;
- the paper Kahane et al. 2025, `2025.tlt-1.18`.

**What it means for English annotation.** EWT 2.18 already annotates this way, i.e. the clarification legitimizes existing practice:
- a finite verb takes `Number`/`Person` from the subject where the form does not show them. VBD: 3499 `Sing|3`, 1262 `Sing|1`, 1120 `Plur|3`…; VBP: 2515 `Plur|3`, 2440 `Sing|1`, 1192 `Sing|2`…;
- `Case` on *you* and *it* — from position: *you* `Nom` 2096 / `Acc` 651, *it* `Nom` 1492 / `Acc` 784;
- the form wins. VBZ has `Sing|3` 5655 times of 5656; the exception is the token *s* with empty FEATS (newsgroup-groups.google.com_AgeingMonkeys_37131d1864a0b950_ENG_20051114_080100-0001). *Was* is always `Sing`. There is no VBP with `Sing|3` in the basic tree. There is only the EUD empty node 16.1 *have* in answers-20111108082831AAco5PI_ans-0002; the engine does not read empty nodes;
- EWT has no `Exponence`. #1233 remains open: whether to put "lexical" features on *a/each/every* (`Number`) and on modals (`VerbForm`, `Mood`, see `dialects/ud-2.18/seeds.md#modal-mood`).

For `mova` this confirms our own "FEATS from the subject" normalization (`en::ud::agreement`, `docs/dialect.md`). At the same time it sets a boundary for it: where the form is unambiguous, agreement does not override it (*the team **are*** → `Plur`).

**Registry.** Unchanged (`Exponence` lives in MISC; the FEATS registry does not cover it).

**Sources.** `docs/changes.md#morphosyntactic-features`; #1233; `2025.tlt-1.18`; the UD survey digest «future», section 2.2 («`Exponence` (from 2.19, per #985)»).

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

<a id="fr-spoken-subtypes"></a>

# French subtypes for spoken language (another language, a borrowing candidate)

**Gist.** After 2.18, French documents three subtypes for spoken treebanks:
- `discourse:filler` — a filled pause (*euh* ≈ *um*);
- `discourse:tag` — a tag question (*…, hein ?*, *…, non ?*, *tu t'en souviens ?*);
- `conj:reform` — a reformulation of an already completed utterance: *une maison en banlieue, la banlieue parisienne*. An incomplete utterance is, as before, `reparandum`.

**Level.** Language-specific (fr). For English this is a candidate for borrowing into `mova`. A related open universal discussion is #1290 (the UniDive spoken-language working group: parataxis, tag questions, backchannels).

**Now (snapshot 24.09.2026).** New files `docs/_fr/dep/discourse-filler.md`, `docs/_fr/dep/discourse-tag.md`, `docs/_fr/dep/conj-reform.md` (not in 2.18). Changed `docs/_fr/dep/dislocated-mod.md` (only a typo in an example).

**Evidence.** `diff -rq`, «Only in docs/_fr/dep». In `tools/data/deprels.json` the permitted relations for fr changed (see `validator-data.md`).

**What it means for English annotation.**
- EWT has no such distinction. Fillers (*um*, *uh*) are INTJ with `discourse`, together with *oh*, *yes*, *wow*: INTJ with `discourse` — 775. A tag question is rare in EWT: grep found one, *…, aren't they?* (train), and it is `parataxis`;
- for spoken English treebanks (ESLSpok, the spoken part of GUM, CHILDES) the subtypes would make it possible to distinguish a filler from an interjection and a tag from a parenthetical clause;
- the en registry has no such subtypes, so they must be dropped on export to UD. A subtype can be dropped without loss: `discourse:filler` → `discourse`.

**Registry.** Not allowed for en; allowed for fr in the snapshot.

**Sources.** `docs/_fr/dep/*.md`; #1290 (`grew.fr/spoken-language-guidelines/workgroups/spoken-data/specific_syntax.html`), #1280 (co-constructions by several speakers), #1273 (segmentation of spoken language).

<a id="other-languages"></a>

# Changes in other languages and the site (summary)

**Gist.** The remaining changes after 2.18 concern individual languages or site infrastructure. They do not affect English annotation, so they are collected in one seed. The comparison ignored the «Interlanguage links updated» lines: a script updates this timestamp in every language-guideline file (4680 files, among them 90 `_en` files).

**Level.** Language-specific and infrastructure.

**Languages** (`diff -rq`: files changed / added / removed):

| language | changes | gist |
|---|---|---|
| hy, hyw, axm (Armenian, Western Armenian, Middle Armenian) | 87 / 1 / 1; 12 / 93 / 0; 86 / 1 / 1 | gloss translations; `compound:redup` → `flat:redup` (new `dep/flat-redup.md`); full hyw guidelines (dep 50, feat 27, pos 16) |
| oge (Old Georgian) | 5 / 15 / 7 | layers `Case[sauf]`, `Case[dsauf]`, `Case[stack]`, `Case[stackb]`, `Number[sauf]`, `Number[dsauf]`; `compound:lvc`; `Polarity`, `Typo`; pages `X[y].md` → `X-y.md` |
| ka (Georgian) | 0 / 6 / 6 | only renaming of layered-feature pages `Number[io].md` → `Number-io.md` |
| lt (Lithuanian) | 8 / 0 / 0 | changes to the pages `acl:relcl`, `advcl`, `ccomp`, `conj` (new examples), `dep`, `reparandum`, `xcomp` and `index.md` |
| el (Modern Greek) | 6 / 0 / 0 | `compound` (reciprocal *ο ένας τον άλλον*), `dislocated`, `expl`, `PronType`, `VerbForm` (gerunds in *-οντας*), `VERB` |
| fr (French) | 1 / 3 / 0 | `discourse:filler`, `discourse:tag`, `conj:reform` — separate seed `fr-spoken-subtypes.md` |
| es (Spanish) | 1 / 1 / 0 | new `dep/dislocated.md`; updated `index.md` |
| egy (Egyptian) | 0 / 2 / 0 | `expl:rel` — an expletive pronoun in a relative clause, agreeing with the antecedent; `AdvType` |
| ru (Russian) | 1 / 0 / 0 | `ccomp` — new examples |
| ps (Pashto) | 3 / 2 / 1 | `Case`, description of the verbal system, feature table |
| ta (Tamil) | 1 / 6 / 0 | reference pages: features, relations, preprocessing, MWE |
| dar, pal, pi, bgs, mjl | new `dep`/`feat`/`pos` folders or `index.md` | first language guidelines |
| sah, hsb, nds, xpg | 1 each | minor changes (`Tense`, a `dep:alt` example, `index.md`) |
| new languages | `index.md` | ce, dta, eve, evn, gld, kfx, ks, mjg, mn, mnc, mvf, neg, oac, peh, sce, txg, ude, ulc, xal |

**Site and general pages** (do not affect annotation):
- `introduction.md` — rewritten introduction and a K'iche' example;
- `index.md`: release 2.18, a link to UD on Hugging Face, language-diversity charts;
- `download.md`: 2.19 — 15.11.2026, data freeze — 01.11.2026;
- `events.md`: UDW 2027 at SyntaxFest in Prague;
- `tools.md`: two new EUD tools — UD Enhanced Graph Visualiser and EUD Annotation for Portuguese (PR #1302);
- `infrastructure/*` — how the site and the release are built;
- `_data/*.yaml`, `_includes/*`, `_layouts/*`, `css`, `lib`, `flags`, `img` — site generation (#1277: accordions in plain JS);
- `languages.md` and `survey-scripts.md` — see `tools-scripts.md`.

**For Mova.** None of these changes alters English annotation. Potentially interesting for the future language-independent layer: reduplication as `flat:redup` rather than `compound:redup` (hy, axm), and `expl:rel` (egy).

**Sources.** `diff -rq -I 'Interlanguage links updated'` between `markdown-source/` of the 2.18 archive and `docs/` of the snapshot: 245 changed files, 186 only in the snapshot, 25 only in 2.18.

<a id="tools-scripts"></a>

# New tools: UPOS+FEATS → XPOS and treebank scripts

**Gist.** After 2.18 two scripts by Zeman appeared in `tools`, and two new pages on the site:
- `conllu_convert_uposf_to_xpos.pl` computes XPOS in a given tagset from UPOS and FEATS via Interset. It is the inverse of `conll_convert_tags_to_uposf.pl`;
- `survey_scripts.pl` determines which script (ISO 15924) each treebank is written in. The result is `docs/survey-scripts.md`;
- `docs/languages.md` — a list of languages with script codes.

This closed #1032 «How to document writing system used in a treebank?» (closed 07.06.2026, milestone v2.19).

**Level.** Universal, tools and metadata. Does not affect annotation.

**Evidence.**
- `diff -rq`: `tools/conllu_convert_uposf_to_xpos.pl` and `tools/survey_scripts.pl` exist only in the snapshot;
- `tools/README.md`, section «conllu_convert_uposf_to_xpos.pl»: «takes UPOS and FEATS, and based on them it computes XPOS in a specific tagset … The target tagset is a tagset identifier known to Interset; the default value is `cs::pdtc`»;
- `docs/survey-scripts.md`: «automatically generated list of scripts that occur in the UD data».

Also in `tools`: `package_ud_release.sh` copies via `rsync` without `__pycache__`; `survey_language_families.pl` was updated; the snapshot has no `compat/`, `conllu-formconvert.py`, `conllu-w2t.py`, `create_iso_639_3_symlinks.py`.

**What it means for English annotation.** Directly — nothing. For Mova:
- Interset has an `en::penn` table. So `conllu_convert_uposf_to_xpos.pl -t en::penn` gives an independent check of UPOS+FEATS ↔ PTB pairs in our `en`, and also XPOS for treebanks without it. Using it requires Perl and `Lingua::Interset`. That is an external tool, so only on Mova's say-so;
- the script (`Latn`) will become a metadata field, and for `treebanks/cards.tsv` one more column.

**Registry.** Not applicable.

**Sources.** `tools/conllu_convert_uposf_to_xpos.pl`, `tools/survey_scripts.pl`, `tools/README.md`, `docs/survey-scripts.md`, `docs/languages.md`; #1032.

<a id="validator-data"></a>

# Validator data `tools/data/*.json`: English unchanged

**Gist.** From 2.18 to the snapshot, the validator data — permitted features, relations, auxiliaries and tokens with spaces — changed for 48 languages. 18 languages are new, 7 more got entries they did not have before (auxiliaries or documentation), 23 changed. English is not among them. Our registry `en/data/ud-registry-en.tsv` matches both 2.18 and the snapshot.

**Level.** Universal (validator infrastructure). Language entries are submitted via web forms `quest.ms.mff.cuni.cz/udvalidator/…`, and the system generates the JSON.

**Comparison.** Each JSON was expanded into "path → value" lines and compared with `diff`: `2.18:tools/data/*.json` ↔ `tools/data/*.json`.

| file | languages in 2.18 → in the snapshot | new languages | changed languages | en |
|---|---:|---:|---|---|
| `data.json` (auxiliaries) | 197 → 214 | 17 | ar, axm, cs, el, tn, uk | unchanged |
| `deprels.json` | 269 → 287 | 18 | axm, bgs, dar, egy, es, fa, fr, hy, hyw, ko, lij, mn, oge, pal, pi, uk | unchanged |
| `feats.json` | 269 → 287 | 18 | axm, bgs, ca, cs, dar, egy, el, ess, hi, hy, hyw, kbc, ko, lij, mn, oge, pal, pi, pro, uk, ur | unchanged |
| `docdeps.json`, `docfeats.json` | 269 → 287 | 18 | 9 and 8 languages; universal `gdocs` and `deviations` unchanged | unchanged |
| `tospace.json` | 23 → 23 | 0 | ltg, lv, nl | no entry |
| `edeprels.json`, `udeprels.json`, `upos.json` | — | — | unchanged | — |

New languages: ce, dta, eve, evn, gld, kfx, ks, mjg, mnc, mvf, neg, oac, peh, sce, txg, ude, ulc, xal. ce, evn, kfx, ks, oac, txg, ulc have no auxiliaries. Languages that already existed but got auxiliaries for the first time (`data.json`): dar, mn, pal, pi, pro, sc. Got language documentation (`ldocs`) for the first time: bgs, dar, pal, pi.

**Our registry.** The English registry was rebuilt from both JSON states: `features/en/*/byupos/*/*` pairs with a non-zero value, `deprels/en/*` with `permitted=1`, `auxiliaries/en/*` with functions. Result:
- 2.18 and the snapshot are identical for en;
- `en/data/ud-registry-en.tsv` equals both: 251 `feat`, 58 `deprel`, 16 `aux`, auxiliary functions match.

So the registry is the 2.18 state, and also the state of the 24.09 snapshot. It cannot yet be called the 2.19 state: en entries may change before the data freeze on 01.11.2026 (`docs/download.md`: release 2.19 — 15.11.2026). The comparison should be repeated before 2.19.

**What it means for English annotation.** Nothing new. As in 2.18, the registry does not accept `Style=Form`, which `_en/pos/PRON.md` gives to *whosoever* (only `Arch, Coll, Expr, Slng, Vrnc` are allowed). The en `feats.json` also has `ExtPos` with `evalues: ["AUX"]`, but the validator accepts only values from `uvalues`, `lvalues` and `unused_*` and does not take `evalues` (`tools/udtools/src/udtools/level4.py:190`), and `_en/feat/ExtPos.md` does not describe the value `AUX`. So `ExtPos=AUX` is not allowed — just as in our registry.

**In passing, for `uk`.** Ukrainian in the snapshot has:
- a new auxiliary *maty* "to have" (`Tense=Fut`, historical spelling, *robyty-mu*);
- `expl:pv` allowed;
- the layer `Animacy[gram]`;
- new `ExtPos` values.

**Sources.** `tools/data/*.json`, `2.18:tools/data/*.json`, `tools/data/README.md`; `en/src/udreg.rs`.

<a id="xpos-not-empty"></a>

# XPOS cannot be an empty string (validator)

**Gist.** The validator checks something new at level 2: the XPOS column is never an empty string. An unspecified XPOS is written as `_`.

**Level.** Universal; validator (`udtools` 0.2.7 → 0.2.8). This change is not in `changes.md`: it is format, not a guideline.

**Before (2.18).** `2.18:tools/udtools/src/udtools/level2.py` — there is no check for empty XPOS. `udtools/pyproject.toml`: `version = "0.2.7"`.

**Now (snapshot 24.09.2026).**
- `tools/udtools/src/udtools/level2.py:247–273`, `check_xpos_format`: `if cols[XPOS] == None or cols[XPOS] == ''` → Error, level 2, testclass MORPHO, testid `empty-string-in-xpos`, «Unspecified XPOS must be encoded as an underscore ('_'), the field must not be empty.»;
- `tools/udtools/src/udtools/validator.py:286` — the check is called for every token line;
- `tools/udtools/pyproject.toml:7` and `__init__.py`: `0.2.8`.

Other validator changes are only logging stubs under `###!!!`, docstrings and a corrected link in a `data.py` message. `level3.py`–`level6.py` and `validate.py` did not change in substance. The tests were rewritten for the new package.

**Evidence.** `diff -r ud-tools-v2.18/udtools tools/udtools`.

**What it means for English annotation.** There is no empty XPOS in the EWT data, in the other 13 English 2.18 treebanks, or in `corpus/mova-*-en.conllu`: checked with grep, 0 lines. `en` writes PTB tags. The change matters for exporting `mova` into other languages where XPOS is a string and may be unknown: write `_`, not empty.

**Registry.** Not applicable: this is a column format. The gate must be in the CoNLL-U writer. It cannot be expressed in the seed rule language: that language does not see raw columns.

**Sources.** `tools/udtools/src/udtools/level2.py`, `validator.py`, `pyproject.toml`; `2.18:tools/udtools/…`.
