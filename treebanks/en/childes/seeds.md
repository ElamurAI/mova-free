# Seeds: treebanks/en/childes

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# CHILDES — what these data are and which layers exist

**Gist.** Spoken language in conversations of children with adults from CHILDES transcripts: 11 children, 48,183 sentences, 302,740 words. The treebank was merged from three previously annotated sets, each with its own guideline:
- S+24 — Adam, UD 1.0, converted to 2.0 by a script;
- LP21 — Eve;
- LP23 — the other children.

The authors harmonized them and made ≈ 8000 edits: most often UPOS versus relation label, auxiliary verbs and particles (→ `compound:prt`). README metadata:
- dependencies — manual;
- lemmas and UPOS — by software with partial manual correction;
- XPOS — by software without checking;
- FEATS — absent.

**Conditions and exceptions.**
- The split is by child. Train (34,732) and dev (3860) — Adam, Lily, Naima, Sarah, Roman, Laura, Abe. Test (9591) — Eve, Violet, Emma, Thomas: children the model has not seen.
- **11,446 sentences have `# gold_annotation = False`** — all from Adam: train 10,274, dev 1172. These are the "10k missing Adam sentences" added in 2.17. The test has only `True`. For `mova` training they are closer to silver than to gold.
- The authors added punctuation themselves: they capitalized the first letter and derived the final sign from the sentence type (`# type`). So PUNCT does not come from the transcript.
- Sentence metadata: `child_name`, `child_age`, `speaker_role`, `corpus_name`, `type` (declarative 31,996, question 14,241, imperative_emphatic 797…), `childes_toks`, `original_sent_id`.
- License CC BY-SA 4.0. Separately from the release there are ≈ 1.2M silver sentences annotated by stanza.

**Examples.** README: `sent_id = 24433`, *A world of Easter.* (Adam, mother, `type = trail off`).

**In UD.**

| layer | state |
|---|---|
| FEATS | absent: «_» in 302,504 of 302,740; the rest — only `ExtPos` (209) and `Typo` (27) |
| XPOS | PTB, automatic; punctuation `?` (8918) and `!` (734) instead of `.` (`punct.md`) |
| lemmas | customs differ by subcorpus (`pron-lemma.md`) |
| DEPS | present |
| MISC | `SpaceAfter` and `reparandum`/`parataxis` subtypes from the sources, moved into MISC |
| MWT | 12,905 |

Our annotator on the CHILDES test: UPOS 93.38, XPOS 93.48, LAS 80.01 (`en/README.md`). Adding 35k CHILDES sentences to training gave EWT LAS −0.2.

**Sources.** README `UD_English-CHILDES` (in particular Changelog v2.17); `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_childes/index.md`; Yang et al. 2025, `2025.udw-1.6` (papers/y/ya/yang-2025-ud-english-childes-collected-resource), sec. 3 («Harmonization»: punctuation, reparandum, UD 1 → 2); Szubert et al. 2022, `2022.scil-1.24`.

<a id="errors"></a>

# CHILDES — known annotation errors

**Gist.** XPOS in CHILDES was assigned by software without checking. Lemmas and UPOS were corrected only partly. So most errors are in the tags. Dependencies were annotated manually; errors there are isolated. The converter does not fix them.

**Examples and numbers** (all parts together):
- **XPOS `POS` on the verb *'s*** — 79: UPOS AUX, relation `cop` 52, `aux` 20, `root` 7. That is, *'s* = *is* has the possessive tag. The rule `tb.en.poss-s` gives 101 of 798: another 16 *'s* PART with `nmod:poss` and 13 *s* PART in `reparandum` or `obj`.
- **XPOS not in PTB**, besides `?` and `!` — 25 tokens: `ADV` 3, `adv` 1, `DET` 1, `INTJ` 1, `PP` 2, `TP` 1, `VV` 2, `remember` 1, `remembered` 1 and `_` 12 (on X fragments *a, k, stairs…*).
- **PUNCT with XPOS `NNP`/`NN`/`HYPH`** — 5.
- **Lowercase *i* as a lemma** — 53.
- **A garbage comment line** `# chi l d` — once in the file.
- **A noun before a head noun not `compound`** (`tb.en.nn-compound`) — 395 of 3650: `amod` 130, `nmod` 90, `nsubj` 75, `reparandum` 37 and others. Some are child speech with repetitions and broken-off phrases, some are errors.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`) — 49 of 106. Fragments, as in GUM (`../gum/seeds.md#obl-fragment`).
- **A sole `iobj` without `obj`** — 414 of 963. Not an error: allowed since UD 2.12.

**In UD.** For merging, CHILDES XPOS is unreliable. `en` already skips sentences with tags outside PTB (`Sentence::tagged()`), but *'s*/`POS`-on-AUX pass as valid tags. They should be passed through tagger suspicions rather than trusted.

**Sources.** README `UD_English-CHILDES` («XPOS: automatic»; «Lemmas, UPOS: automatic with corrections»); Yang et al. 2025, `2025.udw-1.6`, sec. 3 (≈ 8000 edits, most often UPOS versus relation); `en/README.md`, "Examples are not truth".

<a id="npmod"></a>

# CHILDES — `:tmod` renamed, but `:npmod` remained

**Gist.** In 2.17 CHILDES replaced all `:tmod` with `:unmarked`, as UD 2.15 asks (docs#1028). `:npmod` was not touched: `obl:npmod` — 226, `nmod:npmod` — 15. In 2.15 both subtypes were merged, so CHILDES now has two labels for an NP without a preposition:
- `obl:unmarked` 494 / `nmod:unmarked` 110 — the former `:tmod`, temporal *today, morning, yesterday*;
- `obl:npmod` / `nmod:npmod` — measures and quantity *a little bit, a lot, way too…*.

In EWT since 2.15 both are `:unmarked`, and the temporal ones also have `TemporalNPAdjunct=Yes` in MISC. CHILDES does not set this trace.

**Conditions and exceptions.**
- `:npmod`: *way* 53, *bit* 46, *little* 42, *lot* 13, *years* 9, *time* 8, *minute* 8, *while* 7, *yourself* 6.
- `:unmarked`: *today* 163, *morning* 69, *yesterday* 59, *time* 49, *night* 48, *tomorrow* 40, *day* 36.
- *time* and *years* occur in both groups: the "time / measure" boundary in CHILDES is semantic, not formal.
- 35 of 845 such modifiers have a `case` preposition — for an NP without a preposition this is an error (`tb.atis.tmod-case`).

**Examples.** *a little bit* — *bit*: `obl:npmod`; *today* on a verb — `obl:unmarked`.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 241 / 241 | 0 / 0 |
| `tb.atis.tmod-case`: `:tmod`/`:npmod`/`:unmarked` with a preposition | 35 / 845 | 0 / 2604 |

**Sources.** README `UD_English-CHILDES`, v2.17 («Replace all `:tmod` with `:unmarked`»); UD `docs/_en/dep/obl-unmarked.md`; docs#1028; EWT README v2.15.

<a id="old-ud"></a>

# CHILDES — customs before EWT 2.11: *when/how* as SCONJ `mark`, relative *that* as `mark`, *that* as a DET pronoun

**Gist.** Part of CHILDES comes from the UD 1 guidelines (S+24, converted to 2.0 by a script) and from early UD 2. So several customs there are older than EWT 2.11:
1. **Subordinate *when/how/where/why*** — SCONJ `mark`. EWT since 2.11 (#88) uses ADV `advmod`: 676 of 3308 in CHILDES, 0 of 1049 in EWT.
2. **Relative *that*** — `mark` without a role in the clause: 61 of 271 relative *that/which* before an `acl:relcl` verb. In EWT — 1 of 909.
3. **Demonstrative *that/this* as a noun** — DET, not PRON: 304 of 5970. In EWT — 0 of 1531 (PRON since v1.2).

**Conditions and exceptions.**
- Among subordinate WH in `mark`: *when* SCONJ — 420, *how* SCONJ — 132. The other *where, why* are mostly ADV `advmod`, as in EWT: *where* 593, *why* 533, *how* 574.
- DET pronoun *that/this*: `nsubj` 111, `obj` 41, `advmod` 14, `obl` 7, `root` 4.
- CHILDES has no features, so `PronType=Rel` on relatives (`tb.en.rel-prontype` 242 of 242) is not a custom but a missing layer.
- The customs seem to follow the sources, like the lemmas in `pron-lemma.md`, but the `corpus_name` metadata do not show this directly.

**Examples.**
- `34984` *Hold it up so Adam can see how it looks.* — *how*: SCONJ `mark`; in EWT — ADV `advmod`.
- `34841` *And then after that we will play a long game right?* — *that*: DET as a noun with *after*; in EWT — PRON.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 676 / 3308 | 0 / 1049 |
| `tb.atis.relcl-mark`: relative *that/which* — `mark` | 61 / 271 | 1 / 909 |
| `tb.en.dem-pron`: *this/that…* as a noun not PRON | 304 / 5970 | 0 / 1531 |
| `tb.en.to-case`: *to* in `case` not ADP | 24 / 1104 | 0 / 2029 |
| `tb.en.pass-subj`: subject with `aux:pass` — `nsubj` | 22 / 263 | 0 / 1405 |

**Sources.** EWT README v1.2 (this/that → PRON), v2.11 (#88 WH adverbs — `advmod`, relatives); Yang et al. 2025, `2025.udw-1.6`, sec. 3 («As S+24 was annotated using the UD guidelines version 1.0, we convert the annotation using UD version 2.0 with a script…»); README `UD_English-CHILDES`, v2.17.

<a id="pron-lemma"></a>

# CHILDES — pronoun lemmas depend on the subcorpus

**Gist.** Since 2.11 (docs#517) EWT has a rule for pronoun lemmas:
- oblique case → nominative: *me* → *I*, *them* → *they*;
- possessive → the possessive itself: *your* → *your*, *mine* → *my*.

CHILDES was merged from three sources, and each carries its own custom:
- Adam (S+24 + LP23) and Eve (LP21): *your* → *your*, but *me* → *me*;
- the other children (LP23): *your* → *you* — the old EWT rule, and *me* → *I*.

Merging without a converter gives the model two opposite "form → lemma" pairs.

**Conditions and exceptions.**
- *me*: → *I* 995, → *me* 256 (Adam 194, Eve 62).
- *your* PRON: → *your* 784 (Adam 682, Eve 102), → *you* 1093 (all others).
- *them* → *them* 223 versus → *they* 460; *his* → *he* 286 versus → *his* 214.
- *mine, yours, hers, ours, theirs* — the lemma is the form itself: *mine* 135, *yours* 81. In EWT *mine* → *my*, *yours* → *your*.
- *i* → lowercase *i* — 53 times, the rest → *I* (7955).

**Examples.** Adam: *give me* — *me*: lemma *me*. Laura: *give me* — *me*: lemma *I*.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.pron-lemma`: personal or possessive lemma not per EWT 2.11 | 1940 / 6594 | 9 / 5039 |

**Sources.** EWT README v2.11 («revised guidelines for English pronouns (lemmas, features)», docs#517); UD `docs/_en/pos/PRON.md`; Yang et al. 2025, `2025.udw-1.6`, sec. 3 (three sources, each with its own guideline).

<a id="punct"></a>

# CHILDES — added punctuation with XPOS `?` and `!`

**Gist.** CHILDES transcripts have no punctuation. The harmonizers added the final sign from the sentence type (`# type`) and capitalized the first letter (Yang et al. 2025). The XPOS of these signs are the symbols `?` and `!` themselves, not `.` as PTB requires: in PTB `.` is the tag for sentence-final punctuation of any kind. EWT tags `?` and `!` as `.`.

**Conditions and exceptions.**
- Punctuation XPOS in CHILDES: `.` 38,560, `?` 8918, `!` 734. Another 5 PUNCT have XPOS `NNP`, `NN`, `HYPH` — errors.
- The UPOS of all these signs is PUNCT, the relation is `punct`, so this does not affect parsing.
- The punctuation is synthetic. Its amount and position say nothing about speech, and in the model the final sign correlates with `# type`.

**Examples.** `34740` (Sarah) *What else?* — *?*: PUNCT, XPOS `?`. In EWT the question mark has XPOS `.`.

**In UD.** The tag `?` or `!` in XPOS is not in the `en` PTB tag list (`en/src/gram.rs`, `Tag`). The CoNLL-U reader turns it into "no tag", so such sentences already do not go into tagger training (`Sentence::tagged()`). The `?`/`!` → `.` converter will bring them back into training.

**Sources.** Yang et al. 2025, `2025.udw-1.6`, sec. 3 "Harmonization" («we capitalize the first word of each utterance and infer sentence-final punctuation…»); Santorini 1990 (PTB), `.`; README `UD_English-CHILDES` («XPOS: automatic»).
