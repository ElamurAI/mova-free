# 1. The period, the register and the texts Mova reads

Early Modern English (EModE) is the English written and spoken from about 1500 to about 1700: from the first
printed English Bibles to the Restoration. It is the English of Tyndale, Shakespeare, the King James Bible and
Milton. Its grammar is already close to ours, which is exactly what makes it hard for a parser trained on modern
text: almost every sentence looks normal, and then one word in five behaves differently.

This chapter fixes the scope of the manual, names the texts that Mova actually reads, and gives the corpus
figures that the later chapters lean on.

## 1.1 Dates and stages

| Stage | Dates | What changes | Source |
|---|---|---|---|
| Late Middle English | to ~1500 | Chaucer, Malory, Caxton's first prints (1476 on) | Hejná & Walkden 2022, ch. 4 |
| Early EModE | 1500–1570 | `-eth` is still the usual 3rd person singular in the south; `do` is spreading in questions and negatives | Hejná & Walkden 2022, §4.3.2, §4.4.1 |
| High EModE | 1570–1640 | Shakespeare (plays 1590–1613), King James Bible (1611); `-s` replaces `-eth` in speech; `thou` already marked | Abbott 1870/1884, §231; Hejná & Walkden 2022, §4.3.1 |
| Late EModE | 1640–1700 | `its` established, `thou` retreats to religion, poetry, dialect and Quaker usage; modern `do`-support nearly fixed | Abbott §228; Hejná & Walkden 2022, §4.3.1; Kulick et al. 2022 |
| Late Modern English | 1700 on | the biblical register goes on as a **fixed literary style** (Bible revisions, hymns, tales) | this manual, §1.3 |

Yang & Eisenstein (2016, Table 3) split the Penn-Helsinki Parsed Corpus of Early Modern English (PPCEME) into
1500–1569, 1570–1639 and 1640–1710 and show that a tagger trained on the latest slice loses accuracy steadily as
it moves back in time: 93.6 % on 1570–1639 and 87.6 % on 1500–1569 (SVM baseline). The older the text, the
harder it is. The texts Mova reads sit at the easy, late end of that scale, but they keep the morphology of 1611.

## 1.2 The two registers

1. **Living EModE** (1500–1700): plays, letters, sermons, pamphlets. Spelling varies, `you` and `thou` alternate
   for social and emotional reasons (Abbott §§231–235), `-s` and `-eth` coexist, `do` is optional.
2. **The biblical register** (1611 to the 20th century): a frozen selection of EModE forms, kept on purpose for
   solemnity. It keeps `thou`/`ye`, `-est`/`-eth`, `shalt`, `unto`, `behold`, `it came to pass`, subjunctive `be`,
   but its spelling is modern, and later revisions quietly modernise the strong verbs (§1.4).

Mova's English texts are almost all in the second register. That is good news: the register is regular,
consistent and closed. A short list of forms, written once into the lexicon, covers most of it.

## 1.3 The line of English Bibles behind JPS 1917

| Year | Version | Relation to the next |
|---|---|---|
| 1526–1530 | William Tyndale, New Testament (1526) and Pentateuch (1530) | estimates of how much KJV wording goes back to Tyndale run from about four fifths of the New Testament (Nielson & Skousen 1998, not yet in our library) to «around 90 %» of the parts Tyndale translated (Coggan 1968, as cited by Wikipedia, «Tyndale Bible», https://en.wikipedia.org/wiki/Tyndale_Bible, read 2026-10-17) |
| 1535–1568 | Coverdale, Great Bible, Geneva Bible, Bishops' Bible | the KJV translators were told to follow the Bishops' Bible and alter it as little as the original allowed |
| 1611 | King James («Authorized») Version | standard text of 1769 (Blayney, Oxford) is the one in our corpus |
| 1881–1885 | English Revised Version | keeps the KJV style, corrects the Hebrew |
| 1917 | *The Holy Scriptures According to the Masoretic Text*, Jewish Publication Society of America (JPS 1917) | revises the RV/KJV English from the Masoretic Hebrew and Jewish tradition; public domain |

The KJV text in our library is the 1769 text from eBible.org (`$RESEARCH_RAW/corpus-religion/bible-kjv/`,
public domain outside the UK, see the research library guide `docs/papers/corpus-religion.md`). The Torah that Mova parses is
JPS 1917 from eBible.org (`$MOVA_DATA/corpora/judaism/text/torah-jps1917/`, public domain, `licence/DATA.md`).

## 1.4 JPS 1917 is not the KJV: a measured comparison

JPS 1917 is a 20th-century text written in a 17th-century style. It keeps the person forms but drops the most
opaque strong-verb pasts and some function-word habits. The counts below are whole-word matches over the five
books of the Torah, KJV (1769 text, brackets of supplied words removed) against JPS 1917 (the 6 105 sentences of
`torah.tsv`), measured 2026-10-17. Both texts have about 156 600 words.

| Form | KJV | JPS 1917 | Comment |
|---|---:|---:|---|
| thou | 1 524 | 1 547 | kept |
| thee | 959 | 982 | kept |
| thy | 1 254 | 1 334 | kept; JPS uses `thy` before vowels more (see `thine`) |
| thine | 201 | 118 | KJV `thine heart`, `thine house` (22, 13); JPS `thy heart`, `thy house` (27, 26), but both keep `thine eyes` (21, 24) |
| ye | 844 | 882 | kept |
| shalt | 893 | 908 | kept |
| hath / has | 424 / 0 | 417 / 0 | `has` never occurs in either Torah |
| hast | 140 | 136 | kept |
| doth | 27 | 29 | kept |
| art | 67 | 58 | kept |
| unto | 2 356 | 2 372 | kept |
| behold / lo | 239 / 23 | 237 / 24 | kept |
| spake / spoke | 213 / 0 | 0 / 227 | **modernised** («the LORD spake» 120 → «the LORD spoke» 123) |
| begat / begot | 71 / 0 | 0 / 71 | **modernised** |
| bare / bore | 70 / 1 | 1 / 71 | **modernised** |
| sware / swore | 41 / 0 | 0 / 42 | **modernised** |
| brake / broke | 4 / 0 | 0 / 4 | **modernised** |
| gat / got | 5 / 3 | 0 / 6 | **modernised** |
| clave | 3 | 0 | **modernised** |
| shew / shewed / show / showed | 27 / 23 / 0 / 0 | 0 / 0 / 21 / 5 | **spelling modernised** |
| an (article) | 499 | 266 | KJV `an hundred` 56, `an holy` 25 → JPS `a hundred` 57, `a holy` 36 |
| its | **1** | 163 | KJV «after his kind» 23 → JPS «after its kind» 26 |
| who / which | 80 / 981 | 161 / 759 | JPS uses `who` for persons where KJV has `which` |
| it came to pass | 105 | 106 | kept |
| saith | 21 | 21 | kept |
| smote / slew / wrought | 30 / 18 / 17 | 36 / 15 / 19 | kept |
| digged | 15 | 12 | kept |

The only `its` in the KJV Torah is Leviticus 25:5, «That which groweth of its own accord» (a later printers'
change: Abbott §228 notes that *its* «was not used originally in the Authorized Version»). JPS 1917 writes `its`
163 times. Everywhere the KJV uses `his` for a thing («the fruit tree yielding fruit after his kind», KJV Gen 1:11),
JPS writes `its` («fruit-tree bearing fruit after its kind», `jps:GEN:12`, GEN 1:11).

So for Mova, JPS 1917 is **EModE morphology with modern lexis**: the hard part is the person and number system,
not the strong verbs. The 0.3 tales bring back some of the rest (`spake`, `quoth`, `methinks`; chapter 6).

## 1.5 The texts in this manual and how examples are cited

| Corpus | Where | Size | Licence | Cited as |
|---|---|---|---|---|
| Torah, JPS 1917 | `$MOVA_DATA/runs/cycle-torah-2026-10-17/torah.tsv` (+ `.verses` for references) | 6 105 sentences, 156 603 words | public domain | `jps:GEN:69` (GEN 3.9) |
| Torah parse database | `$MOVA_DATA/corpora/judaism/parsedb/current.conllu` | 6 065 JPS sentences, 183 660 tokens (plus 1 917 other sentences) | ours; text public domain | same ids |
| 0.3 tales | `$MOVA_DATA/runs/cycle-tales03-2026-10-16/pool.tsv` | 132 697 sentences | Project Gutenberg, public domain | `book:pg11027:1293` |
| KJV 1769 | `$RESEARCH_RAW/corpus-religion/bible-kjv/` | 66 books | public domain outside the UK | KJV Gen 1:11 |
| Shakespeare | quoted from Abbott 1884 and Hejná & Walkden 2022 | — | public domain / CC BY 4.0 | play, act.scene.line, with the grammar's section |

The Gutenberg books of the tales pool that carry the most EModE forms (counted by sentences with `thou`):

| PG # | Book | Sentences with `thou` |
|---|---|---:|
| 29672 | *Cossack Fairy Tales and Folk Tales* (Bain) | 361 |
| 3454 | *The Lilac Fairy Book* (Lang) | 156 |
| 27200 | *Fairy Tales of Hans Christian Andersen* | 115 |
| 14241 | *More English Fairy Tales* (Jacobs) | 95 |
| 236 | *The Jungle Book* (Kipling) | 80 |
| 7885 | *Celtic Fairy Tales* (Jacobs) | 73 |
| 22373 | *Russian Fairy Tales* (Ralston) | 67 |
| 540 | *The Red Fairy Book* (Lang) | 64 |
| 1597 | *Andersen's Fairy Tales* | 48 |
| 503 | *The Blue Fairy Book* (Lang) | 38 |
| 2591 | *Grimms' Fairy Tales* | 36 |
| 11027 | *Grimm's Fairy Stories* | 27 |

Titles from the `Title:` lines of `$MOVA_DATA/raw/tales-pg-<n>/pg<n>.txt`. In the whole pool, `thou` occurs in
1 279 lines, `thee` 770, `art` 437 (most of them the noun), `whilst` 217, `wilt` 148, `shalt` 126, `quoth` 69,
`spake` 48, `ere` 38, `methinks` 8.

The tales use EModE as a **marked voice**: a king, a frog, a fairy, a jinn, a peasant in a translated folk tale
speak in `thou`; the narrator usually does not. *Uncle Remus* (PG 2306) and the dialect stories are a different
variety (19th-century American and Scots dialect: `wuz`, `kin`, `twa`, `maun`) and are out of the scope of this
manual, though they fail at the same gate (chapter 12).

## 1.6 How this manual treats «correct»

- The reference grammar for EModE itself is Abbott (1870; new ed. 1884), checked against Hejná & Walkden (2022)
  for the history and against Kulick, Ryant & Santorini (2022) and Yang & Eisenstein (2016) for what parsers do.
- The reference for the annotation is Universal Dependencies 2.18 for English as Mova uses it
  (`en/src/udreg.rs`, `en/data/ud-registry-en.tsv`): lemma of an inflected form is the modern dictionary lemma
  (`shalt` → `shall`, `hath` → `have`, `thee` → `thou`), the archaic form is marked with `Style=Arch`, which the
  registry allows for `AUX`, `PRON`, `VERB`, `ADP` and `ADV`.
- Counts are stated with their date. Every count that says «db» is over the 6 065 JPS sentences of
  `parsedb/current.conllu` as of 2026-10-17 21:29; every count that says «SLM now» is the small model
  `en-agreed.bin` (2026-10-15) run by `en annotate` built from `main` at `f3e616747`, run twice with byte-identical
  output (2026-10-17).
