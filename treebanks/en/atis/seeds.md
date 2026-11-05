# Seeds: treebanks/en/atis

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# ATIS — what these data are and which layers exist

**Gist.** Transcripts of spoken queries to an automatic flight-information service (ATIS, Hemphill et al. 1990): 5432 sentences, 61,879 words, all lowercase and without punctuation. The annotation is manual, but UPOS and FEATS were derived from PTB tags that were not included in the release. The workflow (2024.bucc-1.11, §3):
- PTB tags were assigned automatically in StarDust, and annotators corrected them;
- rules converted the PTB tags to UPOS and, according to the README, also to FEATS («converted from manual»);
- dependencies were annotated manually, directly in UD, by three linguists.

**Conditions and exceptions.**
- Split: train 4274 / dev 572 / test 586 sentences. The README says 4224 for train, 50 fewer than in the file.
- Annotator agreement for the English part (table 3): head and label — 82, head — 91, label — 86.
- The parallel Turkish treebank has the same `sent_id` (`# parallel_id`, since UD 2.16).

**Examples.** `0003.dev` *show me round trip flights from chicago to detroit leaving next tuesday and returning the day after*; `0002.dev` *i want a flight from memphis to seattle that arrives no later than 3 pm*.

**In UD.** Layers (all parts together, `cut -f` over the CoNLL-U columns):

| layer | state |
|---|---|
| XPOS | absent: «_» in 61,879 of 61,879 words |
| DEPS, MISC | absent: all «_», even `SpaceAfter` |
| PUNCT, SCONJ, X | 0 tokens; subordinating conjunctions have UPOS ADP (`pos-feats-from-ptb.md`) |
| lemmas | manual; unlike the forms, proper nouns are capitalized: 14,399 of 14,775 PROPN (*chicago* → *Chicago*), also *I* (2387) and day names (*Wednesday* 267) |
| FEATS | «_» in 16,276 words; the rest from the PTB tag, see `verb-feats.md` and `pos-feats-from-ptb.md` |
| MWT | 406: *i'd* 235, *what's* 88, *i'm* 46, *what're* 5, *let's* 5 |
| `# text` | lowercase, no punctuation, like a transcript |

Our annotator trained on EWT + ESLSpok (`en/README.md`, "Full UD annotation") gives on the ATIS test UPOS 74.50, UFeats 74.39, LAS 63.01, MLAS 22.53. This is the worst result among the treebanks. Part of the gap is genre: lowercase, no punctuation. The rest is the customs from the seeds in this folder.

**Sources.** README `UD_English-Atis` (metadata: «XPOS: not available», «Features: converted from manual»); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_atis/index.md`; Cesur et al. 2024, `2024.bucc-1.11` (papers/c/ce/cesur-2024-building-annotated-parallel-corpora-atis), §3 and table 3; digest the UD survey digest «english-banks», §1.11.

<a id="acl-relcl-participle"></a>

# ATIS — participial phrase after a noun — `acl:relcl`, not `acl`

**Gist.** *flights leaving on wednesday* — a participle without a subject and without a relative word. EWT and `mova` give it `acl`, because there is no relative clause here. ATIS almost always uses `acl:relcl`: 662 times of 672. Hence the label proportion in ATIS: `acl:relcl` — 1177, `acl` — 43.

**Conditions and exceptions.**
- The rule counts only `acl:relcl` with `VerbForm=Part` that have no subject or `mark`. Full relative clauses (*a flight that arrives…*) are not included.
- EWT keeps such reduced phrases as `acl` and since 2.14 adds an enhanced edge and `Cxn=rc-red-…` in MISC to them.

**Examples.**
- `0007.dev` *…flights from new york to montreal canada leaving on wednesday*: *leaving* — `acl:relcl` → *flights*.
- `0003.dev` *round trip flights … leaving next tuesday and returning the day after* — *leaving* likewise.

**In UD.**

| rule | ATIS | EWT | GUM | LinES |
|---|---:|---:|---:|---:|
| `tb.atis.relcl-participle`: `acl:relcl` on a participle without subject and `mark` | 662 / 672 | 13 / 527 | 38 / 615 | 34 / 280 |

**Sources.** UD `docs/_en/dep/acl.md` (participial modifiers), `dep/acl-relcl.md`; EWT README v2.14 (#392: enhanced for reduced relatives, `Cxn=rc-red-…`).

```rule
rule: tb.atis.relcl-participle
what: participle as acl:relcl without subject and mark — in EWT this is acl
match: v[rel=acl:relcl, feats.VerbForm=Part]
require: exists c[rel~nsubj, head=v] or exists c[rel=mark, head=v]
severity: warn
source: UD en acl, acl:relcl; counter
```

<a id="errors"></a>

# ATIS — known annotation errors

**Gist.** Besides the customs that distinguish ATIS from EWT (the other seeds in this folder), ATIS has errors that contradict the UD guidelines themselves. The converter does not fix them, because an error is not a dialect. They are candidates for tagger and parser suspicions (`en/README.md`, "Examples are not truth") and for edits.

**Conditions and exceptions.** Counters — the general rules of `../seeds-overview.md` and the rules in this folder, violations / matches, all parts together.

**Examples and numbers.**
- **Passive subject is `nsubj`; there is not a single `nsubj:pass`.** `tb.en.pass-subj` — 10 of 10 passives with `aux:pass`: `1185.train` *what kind of aircraft is used on…* — *kind* `nsubj`. There is no `Voice=Pass` either (`verb-feats.md`).
- **A verb with UPOS NOUN.** `0539.dev` *flight will start from boston* — *start*: NOUN, head, has `aux` *will* and `obl` *boston*. `0220.train` *what cities does continental service* — *service*: NOUN, although it has `aux` *does*.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`): 22 of 30. Some are query fragments where EWT would give `nmod`: `0254.train` *flights from oakland to san francisco on january twenty first 1992*. Some are a consequence of the false NOUN above.
- **A preposition as `amod`** — 3 times: `0003.dev` *…returning the day after*: *after* — ADP, `amod` → *day*. In EWT postposed *after, before* are ADV `advmod`: *before* 41, *after* 5.
- **Relative *that* has three different UPOS in the same `nsubj` role**: ADP 42, DET 29, PRON 23 (`relative-that-mark.md`).
- **Demonstrative *this/that* with `PronType=Art`** — 67 (`pos-feats-from-ptb.md`).
- **The README does not match the data:** train 4224 sentences in the README, 4274 in the file.

**In UD.** Of these errors only `nsubj` instead of `nsubj:pass` is systematic: it is on all 10 passives. It can be converted by a rule, and in `convert.md` it is listed as a custom. The rest are isolated. For training, such sentences are better passed through suspicions than fixed by the converter.

**Sources.** UD `docs/_en/dep/nsubj-pass.md`, `docs/_u/overview/…` (obl and nmod, UD 2.18: `obl-should-be-nmod`); `docs/changes.md` No. 17; README `UD_English-Atis`.

<a id="names-flat"></a>

# ATIS — compound names: the head is the first word, the rest `flat`

**Gist.** ATIS annotates names of cities, airports and airlines the same way: the first word is the head, the following ones are `flat` to it (*san* ← *francisco*, *new* ← *york*). *New* in *new york* is PROPN and the head. EWT distinguishes name types:
- person names — `flat` with a left head;
- structured names — `compound` with a right head (*San* → *Francisco*);
- *New* in *New York* since 2.8 — ADJ `amod`;
- since 2.17 names like *Mount Fuji*, *Fort Worth* — `flat` (#595).

**Conditions and exceptions.**
- Of 3373 pairs of adjacent PROPN in ATIS, 2829 are a left head with `flat`, only 27 are `compound` to the right.
- In EWT, of 3741 pairs: `flat` — 1500, `compound` — 1326. In GUM, of 3664 pairs: 1437 and 1298.
- The prefixes *new, san, los, salt, las, st., fort* before PROPN: in ATIS 1861 of 1865 are neither `amod` nor `compound`. In EWT — 67 of 191.
- `compound` exists in ATIS (2377), but mostly between common nouns: *round trip*, *ground transportation*, *twenty one*.

**Examples.**
- `0003.dev` *…from chicago to detroit…* — single-word names, no difference.
- `0007.dev` *could i have listings of flights from new york to montreal canada…* — *new*: PROPN (lemma *New*), head; *york* — `flat` → *new*. In EWT *New* is ADJ `amod` → *York*. *canada* here is `appos` → *montreal*; since 2.17 EWT puts `nmod:unmarked` + `Superlocation=Yes` on such places (#588).

**In UD.** The rule `tb.atis.name-flat` below counts PROPN pairs with a left `flat` head: ATIS 2829 / 3373, EWT 1500 / 3741, GUM 1437 / 3664. Merging ATIS with EWT without a converter will teach the parser that *new* in *new york* is a PROPN head.

**Sources.** EWT README v2.8 (ADJ/VERB in proper names), v2.15 (#81: foreign names `compound` → `flat`), v2.17 (#595 `flat` for place names, #588 `Superlocation`); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 (*Sri Lanka*: `compound` to the right in EWT, `flat` to the left in GUM); UD `docs/_en/dep/flat.md`, `dep/compound.md`.

```rule
rule: tb.atis.name-flat
what: two adjacent PROPN — left head with flat (ATIS annotates all compound names this way)
match: a[upos=PROPN]; b[upos=PROPN, next=a]
require: not b[rel=flat, head=a]
severity: warn
source: UD en flat, compound; counter
```

<a id="pos-feats-from-ptb"></a>

# ATIS — function words: UPOS and FEATS from the PTB tag without context

**Gist.** The ATIS rules convert the PTB tag to UPOS and FEATS regardless of the word's role:
- DT → `PronType=Art` for all determiners, even demonstrative *this, that*;
- WDT, WP, WRB → `PronType=Int,Rel`, without splitting into interrogative and relative;
- PRP$ → DET;
- IN → ADP, even when the word is a subordinating conjunction (*if, that*).

In EWT and `mova` UPOS and features depend on role: *that* as a conjunction is SCONJ, *which* in a relative clause is `PronType=Rel`, *my* is PRON with `Poss=Yes|Case=Gen`.

**Conditions and exceptions.**
- Articles have `PronType=Art` but no `Definite`: 3341 of 3341.
- *not* — ADV without `Polarity=Neg` (4 of 4). In EWT it is PART with `Polarity=Neg`.
- NUM has only `NumType=Card`; there is no `NumForm` anywhere (1191 of 1191).
- Adverbs have `Degree=Pos` where EWT does not put it: *now, back, then, here* — 132 of 132 from the list of invariable ones.
- `fixed` heads have no `ExtPos`: 22 of 22.
- *what* as a determiner — DET with `PronType=Int,Rel`, 532 times.

**Examples.**
- `0083.dev` *a flight that goes from tampa…*: *that* — ADP, `mark` (on relatives — `relative-that-mark.md`).
- `0057.test` *please list me the flights and their cost…*: *their* — DET.
- *this/that/these/those* with `PronType=Art` — 67 times: *that* 40, *this* 18, *these* 5, *those* 4.

**In UD.** Counters — the general rules from `../seeds-overview.md`, violations / matches:

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.art-feats`: article without `PronType=Art` or `Definite` | 3341 / 3341 | 0 / 16 365 |
| `tb.en.sconj-mark`: *that, if, because…* in `mark` not SCONJ | 284 / 284 | 0 / 2419 |
| `tb.en.poss-pron`: *my, your, their…* in `nmod:poss` not PRON | 28 / 28 | 1 / 3468 |
| `tb.en.dem-pron`: *this, that…* as a noun not PRON | 90 / 125 | 0 / 1531 |
| `tb.en.num-feats`: NUM without `NumType` or `NumForm` | 1191 / 1191 | 15 / 5051 |
| `tb.en.adv-degree`: invariable adverb with `Degree` | 132 / 132 | 0 / 5076 |
| `tb.en.fixed-extpos`: `fixed` head without `ExtPos` | 22 / 22 | 0 / 586 |

`PronType=Int,Rel` is on 1793 words (`grep -c 'PronType=Int,Rel'`); EWT has no such pair. The rule `tb.atis.int-rel` below catches them without a comma in the value, because the rule language does not accept a comma.

**Sources.** Cesur et al. 2024, `2024.bucc-1.11`, §3 («the Penn POS-tags were automatically converted into UD-style tags… by a rule-based algorithm»); UD `docs/_en/pos/SCONJ.md`, `pos/DET.md`, `pos/PRON.md`, `feat/PronType.md`, `feat/Definite.md`; EWT README v2.15 (`Polarity=Neg` on *not*, `ExtPos`), v2.13 (DET and NUM features).

```rule
rule: tb.atis.int-rel
what: ATIS — PronType=Int,Rel (the feature is present, but is none of the simple values)
match: w[feats.PronType, feats.PronType!=Int|Rel|Prs|Art|Dem|Ind|Neg|Tot|Rcp|Emp]
require: w[feats.PronType=Int|Rel]
severity: warn
source: README UD_English-Atis; counter
```

```rule
rule: tb.atis.dem-art
what: ATIS — demonstrative this/that/these/those with PronType=Art (DT → Art)
match: d[form=this|that|these|those, feats.PronType=Art]
require: d[feats.PronType=Dem]
severity: warn
source: UD en feat/PronType (Dem); counter
```

<a id="relative-that-mark"></a>

# ATIS — relative *that/which* is most often annotated as the conjunction `mark`

**Gist.** In a relative clause *a flight that arrives…* ATIS most often attaches *that* to the verb as `mark` with UPOS ADP. The relative-clause verb is then left without a subject. EWT and `mova` treat relative *that* as a pronoun: PRON with `PronType=Rel` in the role of subject, object or obl inside the clause. In EWT `mark` occurs here only once in 909 cases.

**Conditions and exceptions.**
- The remaining relative *that/which* in ATIS are `nsubj` with three different UPOS:
  - *which* PRON — 35;
  - *that* ADP — 42;
  - *that* DET — 29;
  - *that* PRON — 23.

  Here ATIS itself is inconsistent.
- *which* is also sometimes `mark` with ADP — 18 times.

**Examples.**
- `0083.dev` *i would like to find a flight that goes from tampa to montreal…* — *that*: ADP, `mark` → *goes*.
- `0002.dev` *…a flight from memphis to seattle that arrives no later than 3 pm* — *that* is not PRON.
- `0015.dev` *ground transportation that i could get in boston* — in EWT *that* would be `obj` of *get*.

**In UD.**

| rule | ATIS | EWT | CHILDES |
|---|---:|---:|---:|
| `tb.atis.relcl-mark`: relative *that/which* before an `acl:relcl` verb — `mark` | 293 / 463 | 1 / 909 | 61 / 271 |
| `tb.en.rel-that`: relative *that* as subject or object not PRON | 86 / 111 | 0 / 595 | 3 / 201 |
| `tb.en.rel-prontype`: relative pronoun without `PronType=Rel` | 67 / 67 | 1 / 1238 | 242 / 242 |

In ATIS `rel-prontype` is always violated: it has `PronType=Int,Rel` (`pos-feats-from-ptb.md`). CHILDES has no FEATS at all.

**Sources.** UD `docs/_en/dep/acl-relcl.md`, `docs/_en/pos/PRON.md` (relative pronouns); EWT README v2.11 (relatives), v2.14 (`Cxn=rc-…`: relative clause types); Cesur et al. 2024, `2024.bucc-1.11`, §3 (IN → ADP by rule).

```rule
rule: tb.atis.relcl-mark
what: relative that/which annotated as mark, not as a pronoun argument
match: v[rel=acl:relcl]; t[form=that|which, head=v, before=v]
require: not t[rel=mark]
severity: warn
source: UD en acl:relcl; EWT 2.18; counter
```

<a id="tmod"></a>

# ATIS — `:tmod` and `:unmarked` mean "time", even with a preposition

**Gist.** In UD `:tmod`, and since 2.15 `:unmarked`, is a nominal without a preposition (docs#1028). ATIS uses these subtypes as a semantic "temporal modifier" label, so 81% of them have `case`:
- `nmod:tmod` on a noun — 1263, with a preposition 1022: *flights **on** mondays*;
- `obl:unmarked` on a verb — 851, with a preposition 688: *leave **in** the morning*.

ATIS did the 2.15 renaming only for `obl`: there is not a single `nmod:unmarked`, and `nmod:tmod` remained. In EWT `:unmarked` never has a preposition (0 of 2604). Temporal phrases with a preposition there are plain `obl`/`nmod`: 178 of 179 on the words *morning, pm, monday…*. EWT keeps the temporal meaning of an NP without a preposition in MISC as `TemporalNPAdjunct=Yes`.

**Conditions and exceptions.**
- ATIS is inconsistent here too: of 319 `obl`/`nmod` on time words, 210 have a preposition and remained without a subtype.
- ATIS has no `npmod` (measures, *a lot*): all 1263 `nmod:tmod` are temporal.
- MISC is empty, there is no `TemporalNPAdjunct`.

**Examples.**
- `0008.dev` *list the tower air flights on mondays*: *mondays* — `nmod:tmod` → *flights*, with `case` *on*.
- `0003.dev` *…leaving next tuesday…*: *tuesday* — `obl:unmarked` without a preposition, as in EWT.

**In UD.**

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 1263 / 1263 | 0 / 0 |
| `tb.atis.tmod-case`: `:tmod`/`:unmarked` with a preposition | 1710 / 2114 | 0 / 2604 |

For LAS without subtypes (`tools/eval.py`) this difference does not matter; for LAS with subtypes it does.

**Sources.** UD `docs/_en/dep/obl-unmarked.md`, `dep/nmod-unmarked.md` («a modifier structured as an NP without case marking»); docs#1028; EWT README v2.15 (`TemporalNPAdjunct=Yes`); digest the UD survey digest «english-banks», §2 («ATIS… still have tmod/npmod»).

```rule
rule: tb.atis.tmod-case
what: :tmod/:unmarked (NP without a preposition) with a case preposition — a semantic time label
match: t[rel=nmod:tmod|obl:tmod|nmod:unmarked|obl:unmarked|nmod:npmod|obl:npmod]
require: none c[rel=case, head=t]
severity: warn
source: UD en obl:unmarked, nmod:unmarked; counter
```

<a id="verb-feats"></a>

# ATIS — verb features derived from the PTB tag: imperative as infinitive, agreement only in 3sg

**Gist.** Verb FEATS in ATIS depend only on the PTB tag (VB, VBP, VBZ, MD…), not on the sentence. Therefore:
- the imperative *show me…* has the tag VB and gets `VerbForm=Inf`, like an infinitive;
- VBP and VBD have no Number and Person: the tag does not carry them;
- MD has no features at all;
- there is no `Voice=Pass` anywhere.

EWT and `mova` write the imperative as `Mood=Imp|VerbForm=Fin`, and set Number and Person on every finite verb from the subject.

**Conditions and exceptions.**
- VBZ gets `Number=Sing|Person=3` — this is visible from the tag.
- `Tense=Pres|VerbForm=Part` on all *-ing* forms, 884 times. There is no `VerbForm=Ger`, although since 2.14 EWT distinguishes gerund and participle.
- *you* has `Number=Sing` (214 subjects), while in EWT *you* has no Number. EWT gives a verb with subject *you* number `Sing`, 759 of 765.

**Examples.**
- `0003.dev` *show me round trip flights…*: *show* — `VerbForm=Inf`, a root without a subject.
- `0002.dev` *i want a flight…*: *want* — `Mood=Ind|Tense=Pres|VerbForm=Fin`, without `Number=Sing|Person=1`.
- `0007.dev` *could i have listings…*: *could* — FEATS «_».

**In UD.** Counters — the general rules from `../seeds-overview.md`. Entries: violations / matches, all parts together.

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.imp-mood`: VERB root without subject, aux and mark lacks `Mood=Imp` | 1709 / 2599 | 38 / 3066 |
| `tb.en.fin-agree`: finite verb with Tense without Number/Person | 1654 / 2903 | 0 / 19 126 |
| — of them present tense (VBP) | 1632 | 0 |
| — past (VBD) | 22 | 0 |
| `tb.en.modal-fin`: modal does not have exactly `VerbForm=Fin` | 783 / 783 | 0 / 4048 |
| `tb.en.part-voice`: participle with `aux:pass` without `Voice=Pass` | 9 / 9 | 0 / 1643 |

Modals without features: *will* 335, *would* 235 (including *'d* in *i'd*), *can* 183, *should* 14, *may* 14, *must* 1.

The remaining 890 of the 2599 `imp-mood` matches are roots with a subject, aux or mark, such as *i would like…* with an infinitive under aux.

**Sources.** Cesur et al. 2024, `2024.bucc-1.11`, §3 (PTB tags → UD by rules); README `UD_English-Atis` («Features: converted from manual»); UD `docs/_en/feat/Mood.md`, `feat/VerbForm.md`; EWT README, v2.13 («Ensure verb features are complete and consistent with tags»), v2.14 (`VBG`: Ger versus Part); UD `docs/changes.md` No. 18 (2.19): a feature underspecified by the form is taken from context, so Number/Person on VBP come from the subject; `en/README.md` ("Agreement", ATIS −2 UFeats).

```rule
rule: tb.atis.vbp-bare
what: ATIS — present finite without Number (VBP form without agreement)
match: v[feats.VerbForm=Fin, feats.Tense=Pres, !feats.Number]
require: v[feats.Number]
severity: warn
source: README UD_English-Atis (FEATS from the PTB tag); counter
```

```rule
rule: tb.atis.you-number
what: ATIS — you has Number=Sing (in EWT you has no Number)
match: p[form=you, upos=PRON]
require: not p[feats.Number]
severity: warn
source: EWT 2.18 (you: Case|Person=2|PronType=Prs); counter
```
