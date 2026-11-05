# Early Modern English and the biblical register (`en-x-emode`)

A reference manual of the English of about 1500–1700 and of the biblical register that continues it — the
King James Bible (1611), the JPS 1917 Tanakh, Shakespeare, and the old voice of 19th-century fairy tales — written
for Mova: what each feature is, how it differs from modern English, how Mova parses it today, where it fails
(measured), and what to change.

The tag `en-x-emode` is a private-use BCP 47 subtag: English, variety «Early Modern».

## Chapters

| File | Contents |
|---|---|
| [01-period-and-texts.md](01-period-and-texts.md) | dates and stages; the two registers; the line of Bibles from Tyndale to JPS 1917; a measured KJV / JPS comparison (JPS keeps *thou*, *-eth*, *shalt*, but writes *spoke*, *begot*, *its*); the corpora and how examples are cited |
| [02-spelling-and-tokens.md](02-spelling-and-tokens.md) | u/v, i/j, long s, *ye* = *the*; what is left in JPS (*to-morrow*, *for ever*, *LORD*, divine capitals); quotation and verse marks; elision (*'tis*, *may'st*); normalisation and when it helps |
| [03-pronouns.md](03-pronouns.md) | *thou/thee/thy/thine/thyself*, *ye/you*; T/V use in Shakespeare vs number use in the Bible; *mine/thine* before vowels; *his* for *its*; relatives and indefinites; lemma and FEATS decisions; what the small model does |
| [04-verb-endings.md](04-verb-endings.md) | *-est/-st*, *-eth/-th*, plural *-s/-th/-en*; the 2nd person past; spelling rules; `morph::archaic` measured on 1 484 db tokens: three bugs and their fixes |
| [05-auxiliaries.md](05-auxiliaries.md) | *shalt, wilt, hath, hast, doth, dost, didst, art, wast, wert, canst, mayest*; meanings (*shall* = law, *will* = want, *may* = purpose); *be*-perfect (*is come*); UD analysis; the gate |
| [06-strong-verbs.md](06-strong-verbs.md) | *spake, brake, bare, sware, wrought, smote, sod, digged, builded, holpen, quoth*…; old plurals *kine*, *brethren*; what the lemmatizer returns for each |
| [07-do-and-negation.md](07-do-and-negation.md) | *I know not*, *Fear not*, *Knowest thou*, *did eat*; the history of *do*-support; where Mova attaches postverbal *not* (72 errors) |
| [08-word-order.md](08-word-order.md) | inversion after adverbs and fronted objects; dislocation; appositions (*the LORD thy God*); partitive *of*; *from off*; *unto*; ethical datives |
| [09-subjunctive.md](09-subjunctive.md) | *if he be*, *lest ye die*, *until he come*, *Be it*, *cursed be*; *an* = *if*; how UD and the db mark the mood |
| [10-vocabulary.md](10-vocabulary.md) | pro-adverbs (*thereof*, *wherewith*, *whither*…), discourse words (*behold*, *peradventure*…), ~50 false friends (*even* = evening, *corn* = grain, *suffer* = allow…), archaic nouns and verbs, with examples and Mova's tags |
| [11-biblical-style.md](11-biblical-style.md) | and-chains; *it came to pass* (five analyses in the db); speech framing; Hebraisms as idioms; numbers; the register in the tales |
| [12-mova-pipeline.md](12-mova-pipeline.md) | the Torah cycle stage by stage; the 17.10 fixes; refusals per round; the small model's errors by class; how wrong lemmas and features survive; the db audit (D1–D15); effects on pragmatics and the world |
| [13-improvements.md](13-improvements.md) | 30 concrete changes (tables T1–T5, lemmatizer L1–L6, features F1–F3, syntax S1–S7, pragmatics P1–P4, world W1–W4, tokenizer K1–K2, training R1–R2), each with its count and negative control |
| [14-sample-texts.md](14-sample-texts.md) | Genesis 1:1–5 in KJV and JPS; Genesis 3 and 2:17 word by word (db vs small model); a Grimm tale; Shakespeare lines from the grammars |
| [sources.md](sources.md) | grammars, papers, texts, Mova's data and code, web pages, and the «wanted» list |

## Quick reference

| | Present | Past |
|---|---|---|
| thou + lexical verb | thou lov**est**, thou se**est**, thou go**est** | thou lov**edst**, thou cam**est**, thou sa**idst** |
| he + lexical verb | he lov**eth**, he go**eth**, he **saith** | he loved, he came |
| be | I am, **thou art**, he is; subj. **be** | I was, **thou wast/wert**, he was; subj. **were** |
| have | **thou hast**, **he hath** | **thou hadst** |
| do | **thou dost**, **he doth** (aux); *doest*, *doeth* (lexical) | **thou didst** |
| modals | **thou shalt, wilt, canst, mayest** | **thou shouldest, wouldest, couldest, mightest** |

| Pronoun | Nom | Acc | Gen |
|---|---|---|---|
| 2 sg | thou | thee | thy / thine (before a vowel, and predicative) |
| 2 pl | ye | you | your / yours |

| Construction | Old | Modern |
|---|---|---|
| negation | I know not | I do not know |
| question | Knowest thou? | Do you know? |
| negative imperative | Fear not | Do not fear |
| affirmative *do* | he did eat (= ate) | he ate |
| subjunctive | if he be, lest ye die | if he is, so that you do not die |
| perfect of motion | they are come | they have come |
| condition | an thou knowest | if you know |

## The state on 2026-10-17, in five numbers

- **1 087** distinct Torah sentences were refused at the annotation gate during the day because an archaic
  auxiliary kept its form as its lemma (`shalt` 969 refusals, `hath` 401, `hast` 167, `art` 141); after the two
  fixes of the evening (`3e639a08e`, `d3101c072`) the refusal rate fell from 28 % to 0.4 % of drafts.
- The small model `en-agreed.bin` agrees with the Torah db on **94.25 %** of UPOS; archaic tokens are 6 % of the
  text and 41 % of the disagreements; `-eth` and 2nd person `-est` verbs are wrong 90 % of the time.
- **2 242** of 6 013 Torah sentences have a tagging error that a closed table and an ending rule would fix
  (chapter 13, T1–T3).
- The db carries **400** verb tokens whose lemma is still the archaic form (`giveth`, `goeth`, `hath`) in 328
  sentences, **98** sentences with a third-person `thou`, and **202** with `thee` lemmatized as *thee* — all
  repairable by deterministic rules (chapter 13, L5, L6, F1).
- **72** postverbal `not` are attached to the wrong word, which flips the polarity of the root in the
  pragmatic layer (chapter 7.4, rule S1).

## How the numbers were made

Paths: `$MOVA_DATA` is Mova's data directory (corpora, runs, models; outside the repo), `$RESEARCH_RAW` the raw folder
of the shared research library (papers and public-domain grammars, one folder per work with a `SOURCES.md`),
`$UD_ROOT` a checkout of Universal Dependencies 2.18.

Counts over text are whole-word matches over `torah.tsv` (6 105 sentences, 156 603 words), the KJV Torah
(eBible 1769 text, brackets removed) and the tales pool. Counts over trees are over the 6 065 JPS sentences of
`$MOVA_DATA/corpora/judaism/parsedb/current.conllu` (2026-10-17 21:29). «SLM now» is
`en annotate $MOVA_DATA/runs/mmm-parse-2026-10-15/en-agreed.bin` built from `main` at `f3e616747`, run twice
with byte-identical output, aligned with the db by sentence id and token index where the forms agree (6 013
sentences, 180 871 tokens). The lemmatizer probe in chapters 4 and 6 is a throw-away Rust binary that links the
`en` crate and calls `en::morph::archaic` and `Morph::lemmatize`; it is not part of the repo. The analysis
scripts were one-off counts; any number here can be reproduced from the files named in the chapter.

## Scope and limits

- The manual covers the biblical register thoroughly (that is what Mova reads) and living EModE (Shakespeare,
  letters, EEBO) from the grammars only; none of the plays is in Mova's corpora yet.
- Pronunciation (the Great Vowel Shift) is out of scope: Mova reads text.
- Dialect forms in the tales (*wuz*, *kin*, *twa*, *maun*) belong to other varieties.
- Nothing in this folder changes code. Chapter 13 is a list of proposals for separate, measured commits.
