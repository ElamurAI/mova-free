# 12. How Mova reads this English today: pipeline, fixes, failures

This chapter follows a JPS 1917 sentence through Mova's annotation cycle and says, stage by stage, what happens
to its EModE forms. Every number has a date; all are reproducible from the files named.

## 12.1 The pipeline (Torah cycle, 2026-10-17)

The run is `$MOVA_DATA/runs/cycle-torah-2026-10-17/` (scripts `run*.sh`, root `root/`, five Torah flows: rounds
1 `torah-gen`, 3 `torah-exo`, 5 `torah-rest`, 7 `torah-left`, 9 `torah-final`; round 121 is the 0.3 tales flow).
For each round (`tools/cycle/src/pipeline.rs`, `main.rs`):

| Stage | Program | Output | What it does with EModE |
|---|---|---|---|
| 1. SLM draft | `annot drafts` with the Rust annotator `en-agreed.bin` (`CYCLE_EN_MODEL`, 2026-10-15) | `roundK/mmm.conllu` | tokens, UPOS/XPOS, lemma (`en::morph`), FEATS, tree. Trained on UD English: it has never seen `thee`, `shalt`, `-eth` (chapter 3.9) |
| 2. graph parser | `gparse annotate` with `GP_LEX` = `corpus/school/school-agreed.conllu` | `mmm-gparse.conllu` | re-tags and re-parses; where it changes a tag, it refills LEMMA and FEATS from the lexicon (`gparse/src/data.rs`, `Lex::fill`) |
| 3. open parsers + agreement | Stanza, Trankit, spaCy trf, `treeagree` | `pre/agreed.conllu` | votes on the tree |
| 4. draft gates | `annot batch-prepare` → `drafts_from` (`annot/src/main.rs`) → `ann::parse_block` with the UD registry (`en/src/udreg.rs`) | `prepare.log` | a draft that breaks the registry is **refused** and its sentence leaves the round |
| 5. teacher | the middle and top tiers share the calls 80 % / 20 % (`CYCLE_SHARES=0,80,20`) and patch the draft | `opus-a.conllu`, `final.conllu` | repairs most UPOS and relations; lemmas and features it does not touch stay |
| 6. database | `cycle` → `parsedb/current.conllu` | 6 065 JPS sentences | what the world layer reads |

## 12.2 Timeline of the EModE fixes (2026-10-17)

| Time | Commit | Change | Effect |
|---|---|---|---|
| before 20:11 | — | `Morph::lemmatize` knows no archaic forms | `shalt` → lemma `shalt`, tagged NOUN by the SLM; made AUX by stage 2/3 with lemma `shalt`; refused at stage 4 |
| 20:11 | `3e639a08e` | `en::morph::archaic`: 19 verb-only forms whatever the tag; 25 irregular forms and the `-eth`/`-est` stripper with a verb tag | `shalt`, `hath`, `hast`, `didst`, `doth`, `dost`, `canst`, `wast` get the right lemma even as nouns; `art` does not (it is also a noun) |
| 21:11 | `d3101c072` | `annot drafts_from`: an AUX whose lemma equals its form gets `archaic(form, VBP)` as its lemma before the gate | `art` → `be` once the parser has made it AUX |

## 12.3 The refusals, round by round

From `root/roundK/prepare.log`, lines `draft … refused by the gates` (2026-10-17):

| Round | Flow | Drafts | Refused | Share | Main lemmas refused |
|---|---|---:|---:|---:|---|
| 1 | torah-gen | 1 664 | 191 | 11.5 % | hath 65, shalt 49, hast 33, art 24, didst 6 |
| 3 | torah-exo | 1 583 | 497 | 31.4 % | shalt 281, hath 109, hast 49, art 28, didst 11 |
| 5 | torah-rest | 3 791 | 1 050 | 27.7 % | shalt 639, hath 227, hast 85, art 42, didst 28, doth 12 |
| 7 | torah-left | 1 299 | 50 | 3.8 % | art 47, Art 2, mighty 1 (morph fix in, annot fix not yet) |
| 9 | torah-final | 244 | 1 | 0.4 % | mighty 1 (`mightiest`, a transcription error, chapter 2) |
| 121 | tales03 | (4 191 sent) | 209 | — | art 28, shalt 19, dost 18, wuz 16, hast 15, sha 10, kin 8, twa 6, didst 6 … |

Torah total: **1 789 refusals of 1 087 distinct sentences**. By lemma: shalt 969 (+ Shalt 3), hath 401 (+1), hast
167 (+6), art 141 (+6), didst 45, doth 20, dost 9, canst 7, hadst 5, wast 4, mighty 3, sawest 1, bear 1. The
refused sentences were picked up by the later flows («torah-left»), so the cost was rounds and money, not lost
text: the db holds 6 065 of the 6 105 sentences (40 missing, 2026-10-17 21:29).

The tales round 121 ran before both fixes; its refusals include dialect forms of other varieties (`wuz`, `kin`,
`twa`, `sha'`, `mus'`, `maun`, `wer`, `bin`, `ai`, `sorter`, from *Uncle Remus* (PG 2306), Scots and northern
English tales) and EModE forms not on any list yet: `mayst`, `may'st`, `couldst`, `wouldst`, `Wouldst`, `shouldst`,
`'twould`, `'tis`, `forsooth`, `mayhap`, and the tokenizer fragments `St`, `st`, `e`, `er`, `k`, `lt` (of `may'st`,
`could'st`, `ne'er`…).

## 12.4 What the small model does (SLM now)

`en annotate $MOVA_DATA/runs/mmm-parse-2026-10-15/en-agreed.bin` over the 6 105 lines of `torah.tsv`, built from
`main` at `f3e616747` (both fixes in), 4.95 s, two runs byte-identical (2026-10-17). Aligned with the db by sentence
id and token index where the forms match: 6 013 sentences, 180 871 tokens.

| | Tokens | UPOS agreement with the db |
|---|---:|---:|
| all | 180 871 | **94.25 %** |
| modern words | 169 654 | 96.4 % |
| closed archaic words (pronouns, auxiliaries, pro-adverbs, discourse words; 77 forms) | 10 444 | 65.8 % |
| `-eth` verbs | 607 | 10.0 % |
| 2nd person `-est`/`-st` lexical verbs | 166 | 9.0 % |

Archaic tokens are 6.2 % of the text and **41 %** of the UPOS disagreements (3 573 + 546 + 151 = 4 270 of 10 393). If they
were all right, agreement would be about 96.6 %.

Sentences (of 6 013): **1 821** have no UPOS disagreement at all; **2 245** have a disagreement on an archaic
token; **704** have disagreements *only* on archaic tokens — they would be clean with a lexicon. By class:

| Class | Sentences with a UPOS error on it |
|---|---:|
| archaic auxiliaries (`shalt`, `hath`, `hast`, `art`, `didst`, `doth`, `mayest`, …) | 1 129 |
| archaic pronouns (`thee`, `thy`, `thine`, `ye`, `thou`) | 821 |
| pro-adverbs (`thereof`, `wherewith`, `whither`, …) | 473 |
| `-eth` verbs | 454 |
| other closed words (`unto`, `behold`, `aught`, `whosoever`, …) | 127 |
| 2nd person `-est` verbs | 126 |
| `lest` | 59 |

The full list of the largest single disagreements (SLM → db), from the same alignment: `shalt` NOUN→AUX 840,
`LORD` NOUN→PROPN 774, `thee` NOUN→PRON 568, `thereof` ADJ→ADV 359, `thy` ADJ→PRON 231, `hath` NOUN→AUX 196,
`hast` NOUN→AUX 109, `thine` NOUN→PRON 92, `thee` VERB→PRON 87, `lest` ADJ→SCONJ 62, `hath` ADV→AUX 50, `me`
PROPN→PRON 46, `ye` NOUN→PRON 43, `art` NOUN→AUX 43, `mayest` ADJ→AUX 42, `didst` NOUN→AUX 40, `even` ADV→NOUN 38,
`whither` ADJ→ADV 35, `giveth` ADJ→VERB 32, `wherewith` NOUN→ADV 31, `thyself` NOUN→PRON 27, `goest` ADJ→VERB 27,
`thy` PROPN→PRON 26, `thereon` ADJ→ADV 26 (these from the round drafts, `round{1,3,5,7,9}/mmm.conllu`, which agree
with SLM now to within a few tokens).

## 12.5 How wrong lemmas and features survive

Three mechanisms, each traced in the data:

**(a) The tagger's tag decides the lemmatizer's path.** `archaic()` acts only with a verb tag (except the
19 verb-only forms). The SLM tags `giveth` NOUN or ADJ, so `Morph::lemmatize` returns `giveth`. Round 5:
`mmm.conllu` has `-eth` forms NOUN|NN 319, ADJ|JJ 83, ADP|IN 37, all with lemma = form; only the 50 tagged VERB are
lemmatized.

**(b) The graph parser fixes the tag but keeps the lemma.** `Lex::fill(form, lemma, upos, xpos)` looks up
(form, UPOS, XPOS) in `GP_LEX` (the school corpus, which has no `giveth`), and falls back to «the most frequent
FEATS of the new tag, with the old lemma». Round 5 `mmm-gparse.conllu`: `-eth` VERB|VBZ with lemma = form 405,
lemmatized 39. The same fallback gives a re-tagged `thou` the FEATS of the most frequent `PRP`, which is *it*:
`Case=Nom|Gender=Neut|Number=Sing|Person=3` — the source of the **115** third-person `thou` and **98** sentences
in the db (chapter 3.8).

**(c) The teacher patches what it is shown.** The teacher's patch format (`annot/src/batch.rs`, flow `patch`)
corrects the draft; a lemma that looks plausible (`giveth`) is not corrected. The gate checks the lemma only for
AUX. Result in the db: **400** VERB/AUX tokens whose lemma equals an archaic form (`giveth` 35, `goeth` 20,
`toucheth` 19, `cometh` 18, `lieth` 13, `hath` 10, `saith` 10 …; chapter 4.5), in **307 + 21** sentences.

## 12.6 Errors that pass every gate (db audit, 2026-10-17)

| # | Problem | Tokens | Sentences | Chapter |
|---|---|---:|---:|---|
| D1 | `-eth` VERB with lemma = form | 375 | 307 | 4.5 |
| D2 | `hath`/`hast`/`saith` VERB with lemma = form | 21 | 21 | 4.5 |
| D3 | `thee` with lemma `thee` (713 others have `thou`) | 243 | 202 | 3.8 |
| D4 | 2nd person pronoun with `Person=3` (`thou` 115 also `Gender=Neut`, `thy` 20, `thine` 6) | 141 | 98 | 3.8 |
| D5 | `Style=Arch` missing on archaic forms | — | 1 681 | 3.8, 4.7 |
| D6 | `LORD` lemma split (`LORD` 1 176, `Lord` 574, `lord` 36) | 611 not `LORD` | — | 2.2 |
| D7 | strong-verb lemma = form (`wrought` 19, `smote` 10, `abode` 5, `waxen` …) | 67 | 62 | 6.2 |
| D8 | `kine`, `brethren` lemmas | 137 | — | 6.4 |
| D9 | capitalised lemma of a non-proper word (`Behold`, `Whosoever`, `Thy`, `Howbeit` …) | 91 | 87 | 3.7 |
| D10 | postverbal `not` attached to a word other than its verb | 72 | ~70 | 7.4 |
| D11 | subjunctive tagged `VerbForm=Inf` | 53+ | ~50 | 9.5 |
| D12 | «it came to pass»: five analyses | 106 | 106 | 11.2 |
| D13 | `for ever`: seven analyses of `ever` | 67 | 67 | 2.2 |
| D14 | `ye` + noun as `det` | 4 | 4 | 3.3 |
| D15 | `aught`/`nought` as NOUN | 17 | — | 3.7 |

These do not stop a sentence from entering the db; they change what the world layer reads from it.

## 12.7 Effects downstream

- **Pragmatics** (`prag/src/feats.rs`): the subject's person comes from FEATS → D4 makes 98 sentences
  third-person; the modal comes from the form (`norm_word`) → `shalt`, `wilt`, `mayest`, `canst`, `shouldest`
  are features apart from `shall`, `will`, `may`, `can`, `should`; polarity looks for a negator on the root → D10
  turns «I know not the day of my death» into a positive root; the negator list lacks `nought`, `naught`, `nay`,
  `lest`. The second-person detector (`you` list) includes `thee thou thy thine ye` but not `thyself`.
- **World facts** (`world/src/worldbook.rs`, `stands_alone`): a sentence with `Thee`, `Thou`, `Thy`, `Thine`, `thee`,
  `thou`, `thy` is never chosen to word a principle («no prayer or verse») — 1 580 of the 6 105 Torah sentences.
  The filter misses `ye`, `thine`, `hath`, `-eth`; a register score (density of `Style=Arch`) would be a cleaner
  test, if the filter is wanted at all for the Torah course.
- **Reranker** (`world/src/rerank.rs`): the `ArchaicAdverb` repair knows 11 adverbs; the most frequent Torah ones
  are not among them (chapter 10.1).
- **Lessons**: the Torah course (`$MOVA_DATA/school/courses/…`, Torah ★ 252 on 2026-10-17 per `f3e616747`) reads the db.
  `docs/school.md` lists where the small model fails on the course; the EModE layer is one of the causes
  that does not need a model to fix.

## 12.8 What works

- Lemmas of the auxiliaries in the db: `shalt` → `shall` 890 of 890; `hath` → `have` 351 AUX; `art` → `be` 43;
  `didst` → `do` 42; `mayest` → `may` 50; `wilt` → `will` 35 (db, 2026-10-17).
- `unto` ADP `case` 2 339 of 2 359; `behold` INTJ `discourse` 222; `lo` INTJ 24 (db).
- `ye` → PRON `nsubj` 839; `thou` PRON 1 452 `nsubj` (db).
- Hebrew letter names in `-eth` (*Beth*, *Heth*, *Daleth*, *Teth*) stay PROPN: the verb-tag guard works.
- `art` the noun stays a noun (db `art|NOUN` 2, tales round 121 four nouns): the negative control of
  `archaic_verbs` holds in the data.
- The pipeline is deterministic: `en annotate` gave byte-identical output in two runs.
