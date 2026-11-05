# Seeds: treebanks/en/gum

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# GUM — what these data are and which layers exist

**Gist.** Georgetown University Multilayer: a multilayer corpus. It is extended every year by students of the LING-4427 course; texts are taken from openly licensed sources. In UD 2.18 — 14,353 sentences and 256,739 words from 16 main genres: academic, biographies, conversations, fiction, interviews, news, textbooks, travel guides, Reddit (separately, `../gumreddit`), wikiHow, court, essays, letters, podcasts, etc. Manual annotation:
- PTB tags and lemmas — manual;
- UPOS and FEATS — automatic from PTB and the graph;
- dependencies up to GUM v5 — Stanford Dependencies converted by DepEdit rules taking the entity and coreference layers into account; since v6 — directly UD;
- then everything was corrected manually.

**Conditions and exceptions.**
- Split: train 11,314 / dev 1575 / test 1464 sentences.
- License CC BY-NC-SA 4.0: for `mova` test and research only, not training commercial models (`en/README.md`).
- GUM is converging with EWT deliberately. From the changelog:
  - 2021: MWT, hyphen splitting, HYPH;
  - 2022: `advcl:relcl`, `nsubj:outer`, features as in EWT;
  - 2024: `:unmarked`, `ExtPos`;
  - 2025: `nmod:desc`, `dep` → `parataxis` for footnotes.

**Examples.** `GUM_bio_padalecki-2` *Jared Tristan Padalecki (born July 19, 1982) [1] is an American actor.*; `GUM_conversation_grounded-5` *You guys are always in trouble.*

**In UD.** All layers, as in EWT, and richer:

| layer | state |
|---|---|
| lemmas, XPOS | manual; XPOS without the EWT tags `ADD`, `NFP`, `AFX` (`xpos-web.md`) |
| FEATS | from PTB and the graph; «_» in 82,308 words (EWT — 79,476) |
| DEPS | enhanced UD, 154 empty nodes |
| MISC | `Entity` (coreference), `Discourse` (eRST), `MSeg` (morphemes), `Cxn`, `XML`, PDTB; «_» only in 97,819 words |
| MWT | 4455 |

Our annotator (EWT + ESLSpok train) on the GUM test: UPOS 94.26, UFeats 90.84, LAS 76.85, MLAS 60.08 (`en/README.md`). UFeats is 4.8 points lower than on EWT — the customs from `you-number.md` and `adv-degree.md` are at work here. Adding GUM train to training gave EWT LAS +0.8, PUD +3.1, GUM +4.2.

**Sources.** README `UD_English-GUM` (Summary, Changelog); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_gum/index.md`; Zeldes 2017, LRE 51(3); Zeldes & Schneider 2023, `2023.udw-1.7` (papers/z/ze/zeldes-2023-are-ud-treebanks-getting-more); Peng & Zeldes 2018, `W18-4918`; digest the UD survey digest «english-banks», §1.5.

<a id="adv-degree"></a>

# GUM — `Degree=Pos` on invariable adverbs, and *such* without Degree

**Gist.** Two opposite small details with one consequence: GUM and EWT UFeats diverge on frequent words.
1. GUM puts `Degree=Pos` on adverbs that have no degrees of comparison: *really, back, still, again, too, therefore, down, above*. EWT puts `Degree` only on adverbs with degrees: *well, far, soon, long, hard, early, late, little, close, high, fast, badly, low* (all ADV lemmas with `Degree=Pos` in EWT) and on RBR/RBS.
2. *such* as ADJ has no `Degree` in GUM; in EWT it has `Degree=Pos`.

**Conditions and exceptions.**
- ADV with `Degree`: GUM 6033 of 12,206, EWT 997 of 12,595.
- Of the list of 24 invariable adverbs (*really, back, still, again, too, here, there, now, then, just, so, very…*), `Degree` is on:
  - GUM — 1146 of 4668;
  - GENTLE — 88 of 287;
  - EWT — 0 of 5076.
- GUM does not put `Degree=Pos` on *so, just, very, also, even, only* — the same words as EWT. The boundary between "has degrees" and "does not" is different in GUM, not absent.
- *such* ADJ without `Degree` — 145 (all ADJ without Degree in GUM). In EWT *such* ADJ — 88, all with `Degree=Pos`.

**Examples.**
- *really* RB: GUM — `Degree=Pos` 234 times; EWT — without Degree (196 of 199).
- `GUM_academic_exposure-4` *However, it is not enough to have attained such native-like levels.* — *such*: ADJ, FEATS «_»; in EWT — `Degree=Pos`.

**In UD.**

| rule | GUM | EWT | GENTLE |
|---|---:|---:|---:|
| `tb.en.adv-degree`: invariable adverb with `Degree` | 1146 / 4668 | 0 / 5076 | 88 / 287 |
| `tb.en.adj-degree`: ADJ without `Degree` (here it is *such*) | 145 / 16 637 | 0 / 16 789 | 11 / 1228 |

**Sources.** UD `docs/_en/feat/Degree.md`; EWT 2.18 (data: ADV with `Degree=Pos` — only 13 lemmas); README `UD_English-GUM` (FEATS from PTB and the graph, «Many updates to UPOS and FEATS consistency with EWT», 2022-10-21).

<a id="dep"></a>

# GUM — `dep` where EWT has a specific label

**Gist.** `dep` in UD is an "unspecified dependency": a last resort. EWT uses it 5 times per 254,820 words, GUM 184 per 256,739. Most GUM uses are three stable customs, each of which EWT annotates differently:
1. ***you guys, you people, you idiot*** — 56. GUM: *you* — `dep` → the noun, the head is the noun. EWT since 2.17 (#436): the head is *you*, the noun is `nmod:unmarked` → *you*. Not only the label changes, but also the direction.
2. **Photo captions and credits**: *Image: David Titley.*, *(credit: Ken Thomas)*, *(copyright Frank Messina…)* — 58 `dep` on *image/credit/copyright/photo/source*. EWT has 4 such captions, and none is `dep`.
3. **Numbers and symbols that do not count**: item numbers *1.*, *2.*, footnotes *[1]*, *ISO 639-3*, *Page 3*. GUM gives them `dep`, and `nummod` only to real counting (*3 pages*) — 24 times NUM/X/SYM. EWT uses `nummod` or `flat` with `FlatType=Enumerated` (2.16). List numbering markers are attached by EWT as `discourse` since 2.14 (#518).

**Conditions and exceptions.**
- The remaining 46 `dep` are isolated cases: *Pursuant to…*, *drum drumming* etc.
- GUM 2025-11 moved footnotes and citations from `dep` to `parataxis`. Some footnotes *[1]* remained `dep`: `GUM_bio_padalecki-2`.
- The GUM family: GENTLE has 96 `dep` per 17,799 words, of them NUM/X/SYM — 16; GUMReddit — 12.

**Examples.**
- `GUM_conversation_grounded-5` *You guys are always in trouble.* — *You*: `dep` → *guys*. In EWT *guys*: `nmod:unmarked` → *You*.
- `GUM_interview_cyclone-3` *Image: David Titley.* — *David*: `dep` → *Image*.
- `GUM_letter_flood-36` *2. Legislation to increase…* — *2.*: `dep` → *Legislation*. In EWT — `discourse`.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `dep` in total (`grep`) | 184 | 5 |
| `tb.gum.you-dep`: *you* + noun, *you* — `dep` | 56 / 56 | 0 / 0 |
| `tb.gum.credit-dep`: `dep` on *image, credit…* | 58 / 58 | 0 / 0 |
| `tb.gum.num-dep`: `dep` on NUM/X/SYM | 24 / 24 | 2 / 2 |

Merging without a converter will teach the parser `dep` on *you guys* — a label absent from the EWT test.

**Sources.** UD `docs/_u-dep/dep.md`; EWT README v2.14 (#518 numbering markers — `discourse`), v2.16 (numbered entities, `nmod:desc`), v2.17 (#436 *you guys* — `nmod:unmarked`); README `UD_English-GUM`, changelog 2025-11-01 («Changed dep to parataxis for footnotes and citations»); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 (*Page 3*: `dep` in GUM, `nummod` in EWT).

```rule
rule: tb.gum.you-dep
what: you guys — you as dep of the noun (EWT: guys as nmod:unmarked of you)
match: h[upos=NOUN|ADJ|PROPN]; y[form=you, rel=dep, head=h]
require: not y[rel=dep]
severity: warn
source: EWT README v2.17 (#436); counter
```

```rule
rule: tb.gum.credit-dep
what: caption Image:/credit: X — X as dep
match: h[form=image|credit|credits|copyright|photo|source]; d[rel=dep, head=h]
require: not d[rel=dep]
severity: warn
source: UD dep; counter
```

```rule
rule: tb.gum.num-dep
what: dep on a number or symbol (item number, footnote) — EWT: nummod, flat or discourse
match: h[]; d[rel=dep, head=h, upos=NUM|X|SYM]
require: not d[rel=dep]
severity: warn
source: 2023.udw-1.7; EWT README v2.14, v2.16; counter
```

<a id="errors"></a>

# GUM — errors and residues after the rules

**Gist.** GUM was annotated manually with several passes of alignment with EWT, so the general rules find almost no systematic errors. What remains beyond the customs from other seeds occurs just as often in EWT. This is not a dialect difference but a limit of the rules themselves.

**Examples and numbers** (violations / matches; EWT in parentheses):
- **Infinitive root without a subject and without `Mood=Imp`** (`tb.en.imp-mood`): 64 / 2472 (EWT 38 / 3066). All 64 have `VerbForm=Inf`: these are items of instruction lists (`GUM_academic_librarians-18` *Identify possible subjects, based on experience…*, *Get feedback from librarians…*). EWT has 38 such roots of 3066, i.e. the "infinitive or imperative" boundary in list items is shaky in both treebanks.
- ***to* before a gerund — SCONJ `mark`** (`tb.en.to-part`): 48 / 3516 (EWT 58 / 4021): `GUM_letter_arendt-35` *I look forward to going over manuscript…*. A custom shared with EWT, not an error.
- ***whose* without `Case=Gen`** (`tb.en.poss-gen`): 26 / 3438 (EWT 16 / 3689), for example `GUM_fiction_lunre-13`. A residue shared by both treebanks.
- **Non-agentive `by`** (`tb.en.obl-agent`): 17 / 497 (EWT 16 / 411): *observed … by …* meaning "by (a time)", *dissolved by force*. Rule noise.
- ***excuse me* with `iobj`** — 6 (`iobj.md`). A candidate for a real error.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 3 / 1689 (EWT 0).

**In UD.** For merging, GUM is dangerous not because of errors but because of customs: `dep` (`dep.md`), `obl` in fragments (`obl-fragment.md`), the number of *you* (`you-number.md`), `Degree` (`adv-degree.md`), XPOS (`xpos-web.md`).

**Sources.** Rules in `../seeds-overview.md`; `docs/english.md` ("Self-check on gold data": EWT errors caught by seed rules).

<a id="iobj"></a>

# GUM — `iobj`: the data do not show wider use than in EWT

**Gist.** The brief assumed that GUM uses `iobj` more widely. The UD 2.18 data do not confirm this:
- `iobj` occurs less often in GUM: 482 per 256,739 words (0.19%), in EWT — 795 per 254,820 (0.31%);
- the share of "sole" `iobj` without `obj`, `ccomp` and `xcomp` is almost the same: GUM 101 of 482 (21%), EWT 150 of 795 (19%);
- a personal pronoun with *tell/ask/show* is `iobj` in 125 of 132 in GUM and in 177 of 180 in EWT.

Since UD 2.12 (the "Sole iobj" amendment) both treebanks write `iobj` also where there is no direct object: *tell them* just like *tell them a story*.

**Conditions and exceptions.** The only lexical difference visible: *excuse me* — in GUM *me* is `iobj` 6 times out of 6. EWT has no *excuse me*. By UD, *me* here is a direct object (`obj`), so this is a candidate GUM error, not a custom.

**Examples.** GUM, 6 sentences with *Excuse me* — *me*: `iobj` → *Excuse*.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `tb.en.sole-iobj`: `iobj` without `obj` | 255 / 482 | 399 / 795 |
| `tb.gum.excuse-me`: *excuse me* — *me* not `obj` | 6 / 6 | 0 / 0 |

The rule `tb.en.sole-iobj` does not count `ccomp` and `xcomp`: *tell me that…* is also "sole" there. Hence 255 versus 101 above.

**Sources.** UD `docs/changes.md` No. 7 (Sole iobj, 2.12), No. 16 (Multiple Objects, 2.17); UD `docs/_en/dep/iobj.md`; EWT README v2.12 (#55).

```rule
rule: tb.gum.excuse-me
what: excuse me — me as obj (GUM uses iobj)
match: v[lemma=excuse]; o[form=me|us, head=v]
require: o[rel=obj]
severity: warn
source: UD en obj, iobj; counter
```

<a id="obl-fragment"></a>

# GUM — `obl` in fragments without a verb

**Gist.** In verbless fragments (*What about you?*, *Good morning to all.*, *Order out of chaos.*) GUM attaches the prepositional phrase to the noun root as `obl`, as if it were a predicate. EWT uses `nmod` in such fragments, because the head is a noun and there is no copula. UD 2.18 clarified the `nmod`/`obl` boundary, and the validator now catches `obl-should-be-nmod` (`docs/changes.md` No. 17).

**Conditions and exceptions.**
- We count `obl` on a NOUN/PROPN/PRON/NUM root without `cop`: GUM 56 of 207, EWT 12 of 196.
- On all noun heads without a copula, not only roots: GUM 105 of 335, EWT 17 of 291.
- The clearest marker is *What about X?*: in GUM X is always `obl` (13 of 13), in EWT always `nmod` (7 of 7).
- The GUM family: GENTLE 11 of 19, GUMReddit 2 of 6. CHILDES also has many such cases: 49 of 106.

**Examples.**
- `GUM_podcast_bangladesh-13` *What about you?* — *you*: `obl` → *What*.
- `GUM_court_negligence-1` *Good morning to all.* — *all*: `obl` → *morning*.
- `GUM_essay_system-37` *Order out of chaos.* — *chaos*: `obl` → *Order*.

**In UD.**

| rule | GUM | EWT | GENTLE | CHILDES |
|---|---:|---:|---:|---:|
| `tb.en.obl-fragment`: `obl` on a noun root without `cop` | 56 / 207 | 12 / 196 | 11 / 19 | 49 / 106 |
| `tb.gum.what-about`: *What about X* — X not `nmod` | 13 / 13 | 0 / 7 | — | — |

For LAS without subtypes, `obl` versus `nmod` are different labels, hence an error. Zeldes & Schneider 2023 call the `nmod`/`obl` confusion the most frequent error of cross-corpus EWT↔GUM parsing, although most of it is PP attachment, not custom.

**Sources.** UD `docs/changes.md` No. 17 (`nmod` and `obl`, 2.18); UD `docs/_u-dep/obl.md`, `docs/_u-dep/nmod.md`; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 and appendix (confusion matrices).

```rule
rule: tb.gum.what-about
what: What about X? — in GUM X as obl, in EWT as nmod
match: h[form=what, head=0]; o[head=h, after=h]; c[form=about, rel=case, head=o]
require: o[rel=nmod]
severity: warn
source: EWT 2.18; UD 2.18 nmod/obl; counter
```

<a id="xpos-web"></a>

# GUM — XPOS without the EWT web tags: `ADD`, `NFP`, `AFX`

**Gist.** EWT tags XPOS following the LDC Webtext addendum (Mott et al. 2012), which added these tags to PTB:
- `ADD` — addresses and e-mail;
- `NFP` — emoticons and decorative punctuation;
- `AFX` — a detached prefix;
- `GW` — part of a split word.

GUM tags following classic PTB with the BioMedical addendum: `HYPH` is present, but there is not a single `ADD`, `NFP`, `AFX`. `GW` is present (24). The same tokens get ordinary PTB tags in GUM.

**Conditions and exceptions.**
- URLs and e-mail: EWT — `ADD` (475 in total, 364 of them URL/e-mail by form), GUM — `NNP` (33).
- Emoticons *:) :( ;-)*: EWT — `NFP` (96), GUM — `SYM` (1).
- Ellipsis *...*: EWT — `,` (235) or `.` (95), GUM — `:` (94).
- Prefixes before a hyphen (*post-, non-, pre-*): EWT — `AFX` (75: *over* 12, *mid* 10, *post* 4, *non* 4), GUM — `NN`/`NNP`/`VB`/`IN` (*post* 11 NN, 4 NNP, 3 VB).
- UPOS mostly agrees: URL — PROPN, emoticon — SYM. The difference is only in XPOS.
- GENTLE and GUMReddit (the same pipeline) also lack `ADD`, `NFP`, `AFX`. PUD lacks `ADD`, `NFP`, `LS`.

**Examples.** EWT *spahnn@hnks.com*: `ADD`, PROPN. In GUM a URL like *http://…*: `NNP`, PROPN.

**In UD.** For `mova` XPOS are EWT tags, so GUM → `mova` should restore `ADD`, `NFP`, `AFX`. `ADD` can be set by form: `://`, `www.`, `@` and a dot. For `NFP` there is a list of emoticons. `AFX` — before `HYPH`, for a closed list of prefixes. For the tagger this matters little: GUM has dozens of such tokens.

**Sources.** Digest the UD survey digest «english-banks», §1.4 (Webtext addendum: GW, NFP, ADD, AFX) and §1.1 (BioMedical addendum: HYPH, AFX); UD `docs/_en/pos/X.md` (ADD, AFX, GW → X), `pos/SYM.md` (NFP → SYM), `pos/PUNCT.md`; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 4 (GUM v2.8: HYPH, merging `-LSB-`/`-RSB-` with `-LRB-`/`-RRB-`).

<a id="you-number"></a>

# GUM — *you* has Number=Sing or Plur, and the verb agrees with it

**Gist.** GUM distinguishes singular and plural *you*: `Number=Sing` or `Number=Plur` by meaning. A verb with subject *you* takes the same number: *you guys are* — `Number=Plur`. In EWT *you* has no Number, and a verb with *you* always has `Number=Sing|Person=2`. `mova` currently follows EWT.

**Conditions and exceptions.**
- *you* in GUM:
  - `Number=Sing` — 2203 (Nom 1857, Acc 346);
  - `Number=Plur` — 256 (Nom 181, Acc 75).

  In total with Number — 2461 of 2461.
- A verb with Tense with subject *you*:
  - *you*:Sing — verb Sing 819;
  - *you*:Plur — verb Plur 63.

  In EWT: Sing 759, Plur only 6.
- Likewise in the GUM family: GENTLE (*you*: Sing 134, Plur 17), GUMReddit (Sing 203, Plur 6). In ATIS *you* also has Number, but only Sing.
- LittlePrince has Number on *you* 11 times of 85, LinES and ParTUT never.

**Examples.** `GUM_conversation_grounded-5` *You guys are always in trouble.* — *You*: `Number=Plur`, *are*: `Number=Plur|Person=2`. In EWT *are* would have `Number=Sing|Person=2`, and *You* no Number.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `tb.atis.you-number`: *you* with Number | 2461 / 2461 | 0 / 2768 |
| `tb.gum.you-plur`: verb with subject *you* has `Number=Plur` | 63 / 882 | 6 / 765 |

**Conclusion for `mova`.** The GUM custom is more informative: it distinguishes singular and plural *you*. But it cannot be derived from EWT: EWT does not say how many addressees are being spoken to. By the dialect selection rule (`docs/dialect.md`: "more consistent and more informative and convertible to the standard without loss") this is a candidate for borrowing in `dialects/mova/seeds.md`. In `mova` → EWT it converts without loss: remove Number from *you*, give the verb Sing. Mova decides. For now `mova` = EWT, and the converter removes Number (`convert.md`).

**Sources.** UD `docs/_en/feat/Number.md`, `docs/_en/pos/PRON.md`; README `UD_English-GUM`, changelog 2022-10-21 («Many updates to UPOS and FEATS consistency with EWT» — *you* kept Number); Stanovsky & Tamari 2019, `D19-5549` «Y'all should read this! Identifying Plurality in Second-Person Personal Pronouns in English Texts» (papers/s/st/stanovsky-2019-y-all-should-read-this) — distinguishing singular and plural *you* as a separate task.

```rule
rule: tb.gum.you-plur
what: verb with subject you in the plural (GUM agrees with the number of you; EWT — always Sing)
match: v[feats.VerbForm=Fin, feats.Tense]; s[form=you, rel=nsubj|nsubj:pass, head=v]
require: not v[feats.Number=Plur]
severity: warn
source: EWT 2.18; README UD_English-GUM; counter
```
