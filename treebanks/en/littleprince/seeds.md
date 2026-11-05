# Seeds: treebanks/en/littleprince

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# LittlePrince — what these data are and which layers exist

**Gist.** 500 sentences (6852 words) from the English translation of "The Little Prince". The basis is silver Stanza parses from the English Little Prince SNACS corpus (v1.0). Students of the course 11-422 Grammar Formalisms (CMU, Lori Levin) corrected them in ArboratorGrew: two annotators per sentence, ≈ 25% checked by the course instructor. Test only. License CC BY-SA 4.0. In UD since 2.17.

**Conditions and exceptions.**
- `# sent_id` — *lpp_1943.NNN*, by the sentence number in the text.
- `# mwe` in 499 sentences — inherited from the SNACS corpus: the same text as `# text`.
- The README metadata say «XPOS: not available», but the data have XPOS on all words: PTB tags, 39 values. Stanza probably left it, and it is unknown whether it was corrected together with UPOS.
- `# text` — a tokenized string with spaces (*mistakes , also .*), there is no `SpaceAfter`. No MWT. So the original text cannot be reconstructed, as in ESLSpok.
- `NumForm` is in MISC (31: `NumForm=Word` 16, `NumForm=Digit` 15), not in FEATS.

**Examples.** `lpp_1943.265` *Bit by bit I came to understand the secrets of your sad little life ...*; `lpp_1943.284` *" One day , " you said to me , " I saw the sunset forty - four times ! "*

**In UD.**

| layer | state |
|---|---|
| XPOS | PTB from Stanza; no `ADD`, `AFX`, `FW`, `GW`, `LS`, `NFP`, `WP$`, `$`, `-LRB-`/`-RRB-` |
| FEATS | present; older EWT customs (`stanza-legacy.md`) |
| DEPS | absent |
| MISC | «_» in 6820 of 6852 |
| MWT | absent |

Our annotator on LittlePrince: UPOS 96.29, XPOS 95.81, UFeats 85.90, LAS 79.80, MLAS 59.02 (`en/README.md`). UPOS and LAS are almost as on EWT, UFeats is 10 points lower — the customs from `stanza-legacy.md`.

**Sources.** README `UD_English-LittlePrince`; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_littleprince/index.md`; Qi et al. 2020 (Stanza), `2020.acl-demos.14` (papers/q/qi/qi-2020-stanza-python-natural-language-processing); digest the UD survey digest «english-banks», §1.11 (it says "no XPOS" — per the metadata, not per the data).

<a id="errors"></a>

# LittlePrince — known annotation errors

**Gist.** LittlePrince trees and UPOS were corrected by two students per sentence, while features and lemmas were probably corrected selectively (`stanza-legacy.md`). Tree errors are isolated. Feature inconsistency is massive, but it is inherited, not errors on individual words. The converter does not fix errors.

**Examples and numbers.**
- **Mixed norms in one treebank** — the main flaw:
  - *day* as `obl:unmarked` (5) and as `obl:tmod` (2);
  - *his* → *he* (39) and *his* → *his* (5);
  - Number on past *say* (6), but not on most other verbs.
- **XPOS not aligned with the corrected UPOS:** NN on ADJ (1) and on VERB (1). NN on PRON — 26, but that is an EWT custom (*something* NN/PRON), not an error.
- **Infinitive root without a subject** — 1: `lpp_1943.627` *" To admire mean that you regard me…"* — *mean* here is a finite verb with an infinitive subject, but is annotated as `VerbForm=Inf` without `csubj`.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 1 of 28.

**In UD.** 500 sentences — test only. It is useful for evaluating `en` after the reverse converter, but the mixed norms will produce noise even after it (`convert.md`, "Round trip").

**Sources.** README `UD_English-LittlePrince`; rules in `../seeds-overview.md`.

<a id="stanza-legacy"></a>

# LittlePrince — the Stanza legacy: EWT customs before 2.11–2.17 alongside new ones

**Gist.** The annotation grew out of the output of Stanza trained on older EWT versions. The students corrected mainly trees and UPOS. So FEATS, lemmas and some labels keep the old norm, while what was corrected is already new. The 2017-era treebank mixes two norms:
- **`:tmod`/`:npmod` alongside `:unmarked`:** `obl:unmarked` 19, `obl:tmod` 6, `obl:npmod` 3. Even *day* occurs both as `obl:unmarked` (5) and as `obl:tmod` (2).
- **Agreement by form only:** 317 of 625 finite verbs with Tense lack Number/Person. Past with Number — 151, mostly *was/were* (76), but also *say* 6, *do* 5, *have* 4: the students filled in some.
- **Possessive lemmas by the old rule:** *his* → *he* (39 versus 5 *his* → *his*), *your* → *you* (14 versus 2), *their* → *they* (3), *our* → *we* (2), *mine* → *mine*.
- **`Case=Gen` on possessives** — only 12 of 122 in `nmod:poss` (`tb.en.poss-gen` 110 of 122).
- **Expletive *there* without `PronType=Dem`** — 15 of 15. EWT since 2.17 — `Dem`.
- ***not* without `Polarity=Neg`** — 65 of 69. EWT since 2.15 — everywhere.
- **`NumForm` in MISC**, not in FEATS: `tb.en.num-feats` 142 of 142.
- **Subordinate *when/how*** as SCONJ `mark`: 18 of 43. EWT since 2.11 — ADV `advmod`.
- ***you* with Number** — 11 of 85: traces of the GUM custom that Stanza was also trained on (`../gum/seeds.md#you-number`).

**Conditions and exceptions.** `advcl:relcl` (6), `nsubj:outer` (2), `obl:agent` (6), `ExtPos` (22) are already in the treebank: newer customs appear where the students corrected the tree.

**Examples.**
- `lpp_1943.284` *" One day , " you said to me…* — *day*: `obl:tmod`.
- `lpp_1943.228` *…there were on the planet…* — *there*: `expl` without `PronType=Dem`.
- `lpp_1943.265` *…the secrets of your sad little life…* — *your*: lemma *you*, without `Case=Gen`.
- `lpp_1943.267` *…on the morning of the fourth day , when you said to me…* — *when*: SCONJ `mark`.

**In UD.**

| rule | LittlePrince | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod` | 9 / 9 | 0 / 0 |
| `tb.en.fin-agree` | 317 / 625 | 0 / 19 126 |
| `tb.en.poss-gen` | 110 / 122 | 16 / 3689 |
| `tb.en.pron-lemma` | 15 / 201 | 9 / 5039 |
| `tb.en.expl-there` | 15 / 15 | 0 / 458 |
| `tb.en.not-polarity` | 65 / 69 | 0 / 2077 |
| `tb.en.num-feats` | 142 / 142 | 15 / 5051 |
| `tb.en.wh-advmod` | 18 / 43 | 0 / 1049 |

**Sources.** README `UD_English-LittlePrince` («silver parses … output of the Stanza parser … manually corrected»); Qi et al. 2020, `2020.acl-demos.14`; EWT README v2.11, v2.15, v2.17; UD `docs/changes.md` No. 18.
