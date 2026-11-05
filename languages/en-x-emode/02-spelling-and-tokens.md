# 2. Spelling, print conventions and tokens

EModE had no fixed spelling until well into the 17th century. A writer could spell the same word three ways on
one page, and printers padded or trimmed words to justify a line. The texts Mova reads today are almost all
**respelled**: the 1769 KJV, JPS 1917 and the Gutenberg tales use modern spelling with a thin layer of old
forms. This chapter lists both: the real EModE variation that a future corpus (Shakespeare quartos, EEBO) would
bring, and the residue that is already in Mova's data, with what the tokenizer and tagger do with it.

## 2.1 Letters

| Feature | EModE practice | Example (original print) | Modern | Status in Mova's texts |
|---|---|---|---|---|
| `u` / `v` | positional: `v` at the start of a word, `u` inside, whatever the sound | `vnto`, `vp`, `haue`, `loue`, `euer` | unto, up, have, love, ever | absent (respelled) |
| `i` / `j` | `j` is a variant of `i`; capitals `I` for both | `Iesus`, `iudge`, `ioy` | Jesus, judge, joy | absent |
| long `ſ` | `ſ` everywhere except at the end of a word | `ſeede`, `bleſſed` | seed, blessed | absent; OCR may turn it into `f` (`fhall` for `shall`) |
| `vv` for `w` | in some founts | `vvhich` | which | absent |
| thorn `y` | `þ` written as `y` in abbreviations | `ye` = *the*, `yt` = *that*, `ym` = *them* | the, that, them | absent in our texts, but it is the source of the mock-antique «Ye Olde» — **not** the pronoun `ye` |
| final `-e` | silent `-e` added or dropped freely | `shoppe`, `olde`, `sonne`, `runne` | shop, old, son, run | rare |
| `-ie` / `-y` | free | `citie`, `merrie`, `onely` | city, merry, only | rare |
| doubled consonants | free | `shippe`, `goddes`, `farre` | ship, gods, far | rare |
| `-our`, `-re` | standard in British use | `honour`, `neighbour`, `savour` | honor, neighbor | **present** in JPS 1917: neighbour 51, savour 46, favour 33, honour 15, colour 12, labour 9 (2026-10-17) |
| `-ize` / `-ise` | both | | | both in the tales |
| `sh` for `ch`/`s` | old spellings | `shew` (show), `sith` (since), `sithence` | show, since | KJV `shew` 27, `shewed` 23; JPS 0 and 0 (modernised to `show`) |
| `-'d`, `-t` past | elided `-ed`, devoicing | `lov'd`, `blest`, `curst`, `stript`, `dropt` | loved, blessed, cursed | tales and verse; JPS writes `-ed` |
| `-'st`, `-'dst` | elided 2nd person | `lov'st`, `may'st`, `could'st` | — | tales: `may'st` refused at the gate (chapter 12) |

Sources: Hejná & Walkden 2022, ch. 4 (orthography and the printing press); Yang & Eisenstein 2016, Fig. 1 («and
drewe vnto hym all ryottours & wylde dysposed persones» → «and drew unto him all ryottours & wild disposed
persons», a VARD normalization). Yang & Eisenstein report that VARD at a 50 % threshold rewrites 12 % of the
PPCEME tokens (236 298 of 1 961 157) and that tagging the PPCEME with a Penn Treebank-trained tagger reaches only
74.2 % without normalization and 76.7 % with it (their Table 4); combining normalization and domain adaptation
gives 79.1 %.

## 2.2 Spellings and word splits that are still in JPS 1917

| Form in JPS 1917 | Count | Modern | What it is | Mova's tokens (db) |
|---|---:|---|---|---|
| `to-morrow` | 9 | tomorrow | hyphen between preposition and noun | `to` ADP + `-` PUNCT + `morrow` NOUN; `morrow` 22 tokens in all |
| `to-day` | 14 | today | same | `to` ADP/case + `-` + `day` NOUN; relation of `day` split 6 `obl`, 4 `obl:unmarked` |
| `to-night` | 1 | tonight | same | same |
| `for ever` | 67 | forever | two words | **7 different analyses** of `ever` in the db: ADV/advmod 16, ADV/obl 15, NOUN/obl 5, ADV/nmod 4, ADV/fixed 4, ADV/obl:unmarked 3, … |
| `any thing` | 33 | anything | two words | DET + NOUN (obj 12, conj 10, nmod 6): consistent |
| `every one` | 45 | everyone | two words | DET + NOUN or NUM |
| `every where` | 2 | everywhere | two words | — |
| `no wise` | 1 | in no way | | — |
| compounds with hyphen | `burnt-offering` 148, `meal-offering` 100, `first-born` 62, `peace-offerings` 54, `he-goat` 41, `maid-servant` 17, `man-servant` 8, `she-goat` 5 | burnt offering, firstborn | hyphenated compound | split at the hyphen; `he` in `he-asses` is ADJ in the db (`jps:GEN:329`) |
| `LORD` | 1 811 | the Tetragrammaton | printed in small capitals in JPS, all capitals in the digital text | PROPN; lemma split: `LORD` 1 176, `Lord` 574, `lord` 36 (NOUN) |
| `GOD` | 8 | the Tetragrammaton after *Adonai* | «O Lord GOD» (`jps:GEN:378`, GEN 15.2) | PROPN |
| capitalised divine pronouns | `He` 287, `My` 177, `His` 141, `Me` 140, `Thou` 70, `Thy` 67, `Him` 47, `Mine` 28, `Thee` 25, `Thine` 10 (all after a lower-case word or punctuation, so not sentence-initial) | — | JPS capitalises pronouns that refer to God | the SLM tags 114 of 1 155 capitalised pronoun tokens as PROPN (SLM now) |
| `IN THE beginning` | 1 per book opening | — | the first words of a book in capitals | tagged normally |
| `mightiest` | 1 | mightest | a transcription error in the digital source for *mightest* (DEU 4.35, `jps:DEU:147`; the same text has `mightest` at DEU 6.2) | read as the superlative of *mighty*, made AUX by the parser and refused at the gate (`AUX with lemma 'mighty'`) in rounds 5, 7 and 9 |

The `LORD` lemma split matters for the world of the text: one entity, three lemmas. The usual UD practice keeps
the case of a proper noun's lemma as written (`LORD`). Chapter 13 proposes one lemma for all 1 811 tokens.

## 2.3 Quotation, punctuation and verse marks

- **Quotation.** JPS 1917 introduces speech with a colon and single quotes: «And God said: 'Let there be
  light.'» (`jps:GEN:2`, GEN 1.3). 855 sentences contain `: '`. Nested speech uses no second level of quotes:
  «God hath said: Ye shall not eat of it» (`jps:GEN:63`, GEN 3.3). The KJV has no quotation marks at all: «And God
  said, Let there be light» (KJV Gen 1:3). A parser must find the reported clause from the colon and the verb of
  saying, not from quote marks; in the db the quoted clause attaches as `ccomp` of `said` (`jps:GEN:69`).
- **Colon and semicolon** carry clause boundaries that modern English marks with full stops; sentences are long
  (183 660 tokens in 6 065 sentences = 30.3 tokens per sentence in the db).
- **Italics and brackets.** The KJV prints words added by the translators in italics; the eBible text marks them
  with brackets, «darkness [was] upon the face of the deep» (KJV Gen 1:2). They must be removed before parsing
  (done for the counts in chapter 1). JPS 1917 has no such marks.
- **Paragraph marks.** The KJV `¶` (pilcrow) opens a paragraph (KJV Gen 1:6). Remove it.
- **Verse numbers inside sentences.** A sentence that spans a different verse division in Hebrew and English may
  carry both numbers: «(32-33) Therefore the children of Israel eat not the sinew…» (db text of `jps:GEN:1032`).

## 2.4 Elision and contraction

EModE speech forms that appear in plays and in the tales (none in JPS 1917):

| Form | Meaning | Example | Tokenization advice |
|---|---|---|---|
| `'tis` | it is | «'Tis true» | split `'t` + `is` (lemma *it*, *be*) |
| `'twas`, `'twere`, `'twill`, `'twould` | it was / were / will / would | | split; `'twould` was refused at the gate in round 121 as one AUX token |
| `o'er` | over | | one token, lemma *over* |
| `e'er`, `ne'er` | ever, never | | one token |
| `e'en` | even | | one token, lemma *even* |
| `i'` | in | «i' the name of» | one token, lemma *in* |
| `o'` | of, on | «o' clock», «o' the head» | one token |
| `th'` | the | «th' expense» | one token, lemma *the* |
| `an't`, `and't` | if it | «an't please your majesty» | split `an` (SCONJ, *if*) + `'t` |
| `prithee` | I pray thee | | one token, INTJ or VERB; lemma *pray* if analysed |
| `thou'rt`, `thou'lt` | thou art, thou wilt | | split as `'re`/`'ll` are split in modern English |
| `ha'`, `wi'`, `gi'` | have, with, give | dialect in the tales | one token |

Abbott notes that metre drives many of these contractions in the plays (Abbott 1884, §459 on spelling, §460 on
dropped prefixes, §461 «Other Contractions», §§462–466 on contraction that the spelling does not show). They are a tokenizer matter more than a grammar matter:
a split that follows the modern contraction rules (`'s`, `'ll`, `'re` are separate tokens in UD English) keeps the
EModE forms parseable.

## 2.5 Spelling variation in the tales

The 0.3 tales are 19th-century translations and retellings. They use modern spelling, but the dialogue keeps
EModE and dialect forms on purpose:

- «"Then if ye did it your own self," cried the elf-mother shrilly, "what's the use o' making all this fash
  about it?"» (`book:pg14241:294`, *More English Fairy Tales*) — `ye` as a plain plural or polite you, `o'` for
  *of*, Scots *fash* (bother).
- «"Hasn't thou naught for supper, dame?" said he to his wife.» (`book:pg14241:1137`) — the northern dialect
  agreement `thou has`, contracted with `n't`, a mix that no standard EModE grammar lists.
- «"Well," quoth the old woman, "yon's for you."» (`book:pg14241:304`) — `quoth` (past of *quethe*, say) and
  `yon` (that, yonder).

## 2.6 What Mova's tokenizer and tagger do with spelling

| Case | Behaviour (2026-10-17) | Problem |
|---|---|---|
| modern spelling with `-our` | tagged normally | none |
| hyphenated `to-morrow`, `to-day` | split in three; `morrow` NOUN, `day` NOUN | the temporal adverb is lost; `to` read as a preposition. Acceptable if annotated consistently; it is not (2.2) |
| `for ever` | two tokens | seven different trees in the db |
| capitalised divine pronouns | `Me` 46×, `Thou` 15×, `Thy` 27× tagged PROPN by the SLM (SLM now, aligned with the db) | the capital letter is read as a name |
| `LORD` | SLM: NOUN 774×, db: PROPN | the teacher fixes it, but lemmas differ |
| OCR `f` for `ſ` | not in the corpus | would make `fhall`, `fay` unknown words |
| `ye` = *the* | not in the corpus | a future EEBO text would need it; the pronoun `ye` and the article `ye` must not be merged |

**Normalization or not?** Yang & Eisenstein (2016) show that spelling normalization alone helps a modern tagger by
about 2.5 points on PPCEME, and that the combination with domain adaptation is best. For the texts Mova reads
now, normalization is not the issue: their spelling is already modern. What breaks the parser is **morphology and
closed-class words** (chapters 3–5), which normalization tools like VARD leave alone because `thou`, `hath` and
`shalt` are correct words, not misspellings. If Mova ever reads unnormalized EModE (EEBO-TCP, the First Folio),
a respelling layer before the tokenizer — a deterministic table plus the positional `u`/`v` and `i`/`j` rules —
is the cheap first step; the lexicon of this manual is the second.
