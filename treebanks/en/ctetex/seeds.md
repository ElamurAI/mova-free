# Seeds: treebanks/en/ctetex

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# CTeTex — what these data are and which layers exist

**Gist.** Technical text: 196 software requirements, mostly from the PURE corpus. That is 276 sentences and 9273 words. The annotation is manual, directly in UD; the main annotator spent ≈ 180 hours. The sentences are long: 33.6 tokens on average versus 17.7 in eight other English UD 2.8 treebanks. Typical phenomena:
- abbreviations and acronyms — 579;
- specialized vocabulary — 503;
- lists — 41, 2736 tokens in total.

The treebank is test-only: fewer than 20k tokens, so it is not split.

**Conditions and exceptions.**
- Lemmas were added in 2.16. There is no XPOS.
- `# sent_id` — a number, `# newdoc id` — the PURE file name and section, 196 documents.
- MISC has its own attribute `LineAfter=Yes` (175): in the source text there was a line break after the word. It is needed to reconstruct vertical lists.
- License CC BY-SA 4.0.

**Examples.** `199` *Lag Frames – The BE shall receive LTA or Speed Dump Lag Frames from the Correlator.*; `270` *User's will be able to pick up a tile from the wall if it is their turn.*

**In UD.**

| layer | state |
|---|---|
| XPOS | absent: «_» in 9273 of 9273 |
| FEATS | only `Number` (NOUN, AUX, some VERB) and `Tense` (VERB, AUX); «_» in 6070 words (`feats-minimal.md`) |
| lemmas | manual since 2.16; *an* → *an* (41) |
| DEPS | absent |
| MWT | absent: *User's* — two words without a `1-2` line |
| MISC | `SpaceAfter` 1560, `LineAfter` 175, `Foreign` 21 (in MISC, not in FEATS, on formula symbols), `CorrectForm` 1 |

Our annotator on CTeTex: UPOS 87.03, UFeats 58.07, LAS 61.49, MLAS 13.26 (`en/README.md`). UFeats and MLAS are low primarily because the gold data lack features that we set: only part of them agrees with the reduced FEATS. Stanza 1.2.3 on CTeTex, according to the authors: UPOS 0.870, LAS 0.688.

**Sources.** README `UD_English-CTeTex` (Specificities: `LineAfter`); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_ctetex/index.md`; Hassert et al. 2021, `2021.udw-1.5` (papers/h/ha/hassert-2021-ud-software-requirements-application-challenges), sec. 3–4 and table 1.

<a id="errors"></a>

# CTeTex — known annotation errors

**Gist.** CTeTex was annotated manually by one main annotator (≈ 180 hours). There are few systematic errors. The problem for merging is incomplete FEATS and non-standard customs, not faults. Below is what contradicts the UD guideline itself. The converter does not fix this.

**Examples and numbers.**
- **`dep` on *s* in *requirement(s)*** — 7 of 8 `dep`. This is a plural ending in parentheses. EWT annotates such *s* as X/AFX with `amod` (9 times, 1 more — `compound`), not `dep`.
- **`reparandum` in written text** — 2 (*on*, *for*): requirements have no speaker self-corrections, so these are most likely typos. `Typo` or `goeswith` would be more correct.
- **`Foreign=Yes` in MISC on formula symbols** — 21 (*n*, *2*, */*, *160*). `Foreign` is a FEATS feature for foreign words, not for mathematical notation.
- **Possessive *'* as PUNCT `punct`** — 4: *users'* etc. It should be PART `case`, like *'s*.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 3 of 107.
- **Numbers in variables and TBD.** The variables *t*, *n* have UPOS NUM, *TBD* — VERB or NUM. This is a deliberate choice by the authors (sec. 4.1–4.2), not an error, but EWT has no analogues.

**In UD.** CTeTex is test only. Evaluating `en` on it requires the reverse converter (`convert.md`): otherwise UFeats and MLAS measure the difference in customs, not quality.

**Sources.** Hassert et al. 2021, `2021.udw-1.5`, sec. 4.1–4.2 (variables — NUM, TBD — by the head of the expression); UD `docs/_u-dep/goeswith.md`, `docs/_u-dep/reparandum.md`, `docs/_u-feat/Foreign.md`.

<a id="feats-minimal"></a>

# CTeTex — FEATS reduced to `Number` and `Tense`

**Gist.** Of all English UD features, CTeTex sets only:
- `Number` — on nouns and 3sg verbs;
- `Tense` — on verbs;
- isolated `Typo` and `ExtPos`.

There is no `VerbForm`, `Mood`, `Person`, `PronType`, `Definite`, `Degree`, `Case`, `NumType`, `NumForm`, `Polarity`, `Voice`. EWT and `mova` have the full set. So for CTeTex the FEATS layer is not "absent" but incomplete. Such gold data can neither be trained on as complete nor used to measure UFeats directly.

**Conditions and exceptions.**
- The most frequent FEATS:
  - NOUN `Number=Sing` 2222, `Number=Plur` 424;
  - VERB `Tense=Past` 286 — participles and past alike;
  - AUX `Number=Sing|Tense=Pres` 183;
  - VERB `Number=Sing` 47.
- VERB without any feature — 385, AUX — 280. These are infinitives and modals: *shall* has «_».
- PROPN have no `Number` (293 of 293), all NOUN have it.

**Examples.** `199` *…The BE shall receive…* — *shall*: AUX, FEATS «_»; *receive*: VERB, «_». In EWT *shall* — `VerbForm=Fin`, *receive* — `VerbForm=Inf`.

**In UD.** General rules from `../seeds-overview.md` (CTeTex / EWT):

| rule | CTeTex | EWT |
|---|---:|---:|
| `tb.en.art-feats` | 811 / 811 | 0 / 16 365 |
| `tb.en.adj-degree` | 647 / 647 | 0 / 16 789 |
| `tb.en.modal-fin` | 246 / 246 | 0 / 4048 |
| `tb.en.num-feats` | 317 / 317 | 15 / 5051 |
| `tb.en.propn-number` | 293 / 293 | 475 / 16 562 |
| `tb.en.not-polarity` | 52 / 52 | 0 / 2077 |
| `tb.en.fixed-extpos` | 51 / 52 | 0 / 586 |
| `tb.en.expl-there` | 8 / 8 | 0 / 458 |

**Sources.** README `UD_English-CTeTex` («Features: manual native»); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_ctetex/index.md` (feature list: ExtPos, Number, Tense, Typo); Hassert et al. 2021, `2021.udw-1.5`.

<a id="lists"></a>

# CTeTex — vertical lists: `parataxis` to the first item, `list` between items, the number is `nummod`, the break is `LineAfter`

**Gist.** Requirements often look like "introduction: + a list of items on new lines". UD does not say how to attach the list to the introduction, so CTeTex has its own custom (Hassert et al. 2021, sec. 4.3):
- if the introduction is a full sentence, the first item attaches to its predicate as `parataxis`;
- if the introduction is incomplete (*…has the following:*), the relation is the one it would be without the colon, most often `obj`;
- items among themselves — `list`;
- multi-level lists — `list` inside an item.

Line breaks are preserved by `LineAfter=Yes` in MISC. EWT also uses `list` (1061, mostly addresses and signatures) and `parataxis` for such structures, but has no `LineAfter`.

**Conditions and exceptions.**
- `list` — 116: on NOUN 90, VERB 12, SYM 6, ADJ 5.
- `parataxis` — 127: NOUN 64, VERB 25, ADV 17, ADJ 11.
- `LineAfter=Yes` — 175. `# text` does not preserve lines: the list is written on one line.
- Item numbers (*1* + *.*) — NUM `nummod` on the item's predicate: 25 times on a verb, adjective or adverb (`tb.ctetex.enum-nummod`). CTeTex has not a single `discourse`. EWT since 2.14 (#518) attaches numbering markers as `discourse` (LS — 113); `nummod` on a non-noun there — 2, in GUM — 7. UD 2.14 says directly that `nummod` for numbering markers contradicts its definition (`docs/changes.md` No. 14).

**Examples.** `112` *The TCS shall, as a minimum, have the functionality to provide the following control capabilities: 1. Send and receive tactical communication messages. [SSS222]* — *Send*: `parataxis`; *1*: NUM `nummod` → *Send*; after *:* and *]* — `LineAfter=Yes`.

**In UD.** For `mova` the customs with `list` and `parataxis` do not contradict EWT: the labels are the same, only the frequency differs. The converter does not touch them, but moves numbering markers to `discourse`. `LineAfter` is kept in MISC as is. It is useful for a text generator, because it shows where the document had a line break.

**Sources.** Hassert et al. 2021, `2021.udw-1.5`, sec. 4.3 («Lists and Enumerations», figs. 6–8); README `UD_English-CTeTex` (Specificities: `LineAfter`); UD `docs/_u-dep/list.md`, `docs/_u-dep/parataxis.md`.

```rule
rule: tb.ctetex.enum-nummod
what: list item number — nummod on the predicate (EWT 2.14: discourse)
match: h[upos=VERB|ADJ|ADV|AUX]; n[upos=NUM|X, rel=nummod, head=h, before=h]
require: n[rel=discourse]
severity: warn
source: UD docs/changes.md No. 14 (List Item Markers, 2.14); EWT README v2.14 (#518); counter
```

<a id="nmod-compound"></a>

# CTeTex — a noun before a noun: half `nmod`, half `compound`

**Gist.** An English noun before a head noun (*water use*, *Lag Frames*) is `compound` in UD. EWT annotates 4750 of 4784 such pairs this way. CTeTex uses `compound` only in 324 of 638 pairs; in 307 it is `nmod` without a preposition. Among the `nmod` pairs there are many capitalized terms (*Subscription Version*, *Redundancy Management*, *Provider ID*); in EWT such terms are `compound`.

**Conditions and exceptions.**
- Adjacent nouns where the left one depends on the right one:
  - `compound` 324;
  - `nmod` 307;
  - `amod` 6;
  - `nsubj` 1.
- The most frequent `nmod` pairs: *Subscription Version* 8, *Redundancy Management* 7, *Provider ID* 5, *water use* 4, *User Location* 4, *System Initialization* 4, *RAM scrub* 4.
- The CTeTex guideline (Hassert et al. 2021) does not explain this choice. It seems to treat terms as modifiers rather than parts of a compound, as in the Italian TUT (`../partut/seeds.md#nmod-compound`).

**Examples.** `199` *Lag Frames – The BE shall receive LTA or Speed Dump Lag Frames…* — *Lag*: `nmod` → *Frames*, both times. In EWT — `compound`.

**In UD.**

| rule | CTeTex | ParTUT | EWT |
|---|---:|---:|---:|
| `tb.en.nn-compound`: a noun immediately before a head noun not `compound` | 314 / 726 | 862 / 974 | 19 / 5376 |

In the first column the rule counts matches — all pairs of adjacent NOUN — and violations — those that depend on the right neighbour other than as `compound`.

**Sources.** UD `docs/_en/dep/compound.md`, `docs/_u-dep/nmod.md` (`nmod` — usually with `case`); Hassert et al. 2021, `2021.udw-1.5`, sec. 4.4 (specialized vocabulary).

<a id="old-conventions"></a>

# CTeTex — `compound` without `:prt`, possessives as DET `nmod`, *when* as `mark`, lemma *an*

**Gist.** CTeTex was annotated in 2021 following the UD guidelines of the time, without the EWT English subtypes. Hence four customs that look different in `mova`:
1. **The phrasal-verb particle** (*pick up*, *checked out*) — `compound`, not `compound:prt`: 14 of 14. CTeTex has not a single `compound:prt`.
2. **Possessives *its, their, our, my, his*** — DET with `nmod`, not PRON with `nmod:poss`. CTeTex has not a single `nmod:poss`, and 16 possessive DET.
3. **Subordinate *when*** — SCONJ `mark` (34), not ADV `advmod` as in EWT since 2.11. Likewise *where* SCONJ 2.
4. **The lemma of the indefinite article *an*** — *an* (41 of 41), not *a*.

**Conditions and exceptions.**
- Possessive *'s* — PART `case`, as in EWT (4 times), and *'* — PUNCT `punct` (4). The latter is probably a tokenization error in *users'*.
- In EWT 33 of 3501 *my, its, their…* are also not `nmod:poss`, but these are not modifiers: independent pronouns (*his* as `xcomp`) and typos (*their* instead of *there* as `expl` — 6).
- `obl:agent` is present (22), `nsubj:outer` (1), `csubj:pass` (1) — so the annotators partly knew the 2.10–2.13 subtypes.

**Examples.**
- `270` *User's will be able to pick up a tile from the wall…* — *up*: ADP, `compound` → *pick*. In EWT — `compound:prt`.
- `92` *An FAQ section…* — *An*: lemma *an*.
- `112` *…the following control capabilities: 1. Send and …when…* — *when*: SCONJ, `mark`.

**In UD.**

| rule | CTeTex | EWT |
|---|---:|---:|
| `tb.en.compound-prt`: particle on a verb — not `compound:prt` | 14 / 14 | 40 / 939 |
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 36 / 47 | 0 / 1049 |
| `tb.en.an-lemma`: *an* with a lemma other than *a* | 41 / 41 | 0 / 619 |
| `tb.ctetex.poss-nmod`: *my, its, their…* — not `nmod:poss` | 16 / 16 | 33 / 3501 |

**Sources.** UD `docs/_en/dep/compound-prt.md`, `dep/nmod-poss.md`, `docs/_en/pos/PRON.md` (possessives — PRON), `docs/_en/pos/DET.md`; EWT README v2.11 (#88); Hassert et al. 2021, `2021.udw-1.5`.

```rule
rule: tb.ctetex.poss-nmod
what: possessive modifier my/its/their… — nmod:poss (CTeTex: DET nmod)
match: p[form=my|your|his|its|our|their, rel!=conj]
require: p[rel=nmod:poss]
severity: warn
source: UD en nmod:poss; counter
```
