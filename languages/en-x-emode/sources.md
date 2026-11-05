# Sources

What this manual rests on, where each item is, its licence, and what it was used for. Local paths are on the
project's data disk (outside the repo, `licence/DATA.md`); open URLs are given where they exist. Quotations in
the chapters are short and attributed. Read or measured on 2026-10-17.

## Grammars and histories (read)

| Work | Local copy | Licence | Used for |
|---|---|---|---|
| E. A. Abbott, *A Shakespearian Grammar: an attempt to illustrate some of the differences between Elizabethan and modern English*, new ed., London: Macmillan, 1884 (1st ed. 1869/1870) | `$RESEARCH_RAW/en-gram-abbott-shakespearian-1884/` (scan and OCR text; https://archive.org/details/shakespeariangra00abborich) | public domain | §§ 101–105 (*an*, *and if*), 118–128 (*but*), 212–213 (*thee* for *thou*), 217–218, 228 (*his*, *its*), 231–237 (*thou*/*you*, *ye*, *mine*/*thine*), 241, 244, 265–274 (relatives), 295 (*be*-perfect), 298–331 (*be*, *do*, modals), 332–345 (verb inflections), 361–371 (subjunctive), 420–427 (transposition), 459–466 (spelling, contraction) |
| Míša Hejná & George Walkden, *A history of English*, Berlin: Language Science Press, 2022 (Textbooks in Language Sciences) | `$RESEARCH_RAW/en-gram-langsci-hejna-walkden-history-2022/346-hejnawalkden-2022.pdf` | CC BY 4.0 | ch. 4 «Early Modern English (1500–1700)»: §4.3.1 *thou*/*you* (Wales 1983, Brown & Gilman 1960, Quakers), §4.3.2 `-th` vs `-s`, irregular forms, §4.4.1 V-to-I movement and *do*-support (Ecay 2015, Ellegård 1953, Warner 2005, Kroch 1989) |
| Otto Jespersen, *Negation in English and Other Languages*, København: Høst, 1917 (new ed. Language Science Press 2025) | `$RESEARCH_RAW/en-gram-jespersen-negation/` | public domain (1917); new ed. CC BY | p. 14: *know not* kept into the 19th century; *not* before the verb in Shakespeare |
| Wilhelm Franz, *Shakespeare-Grammatik*, Halle: Niemeyer, 1898–1900 | `$RESEARCH_RAW/en-gram-franz-shakespeare-grammatik-1900/` | public domain | consulted for the table of contents; German, fuller than Abbott on morphology — a source for the next revision |

## NLP papers (read)

| Work | Local copy | Licence | Used for |
|---|---|---|---|
| Yi Yang & Jacob Eisenstein, «Part-of-Speech Tagging for Historical English», NAACL-HLT 2016, 1318–1328 | `$RESEARCH_RAW/aclbulk-2016-yang-part-speech-tagging-historical-english/N16-1157.pdf`; https://aclanthology.org/N16-1157/ | CC BY 4.0 | tagging accuracy by period in PPCEME (Table 3), PTB → PPCEME 74.2 % → 79.1 % with normalisation and adaptation (Table 4), VARD normalises 12 % of tokens, 23 % OOV |
| Seth Kulick, Neville Ryant & Beatrice Santorini, «Parsing Early Modern English for Linguistic Search», SCiL 2022 | `$RESEARCH_RAW/lingbulk-2022-kulick-parsing-early-modern-english-linguistic/2022.scil-1.12.pdf`; https://aclanthology.org/2022.scil-1.12/ | CC BY 4.0 | the six *do*-support sentence types, PPCEME 1.7 M words 1500–1712, EEBO 1.5 G words |
| Seth Kulick, Neville Ryant & Beatrice Santorini, «Penn-Helsinki Parsed Corpus of Early Modern English: First Parsing Results and Analysis», Findings of NAACL 2022 | `$RESEARCH_RAW/findings-2022-kulick-penn-helsinki-parsed-corpus-early/2022.findings-naacl.44.pdf`; https://aclanthology.org/2022.findings-naacl.44/ | CC BY 4.0 | background: parser F1 and per-tag accuracy on PPCEME |
| (not read in detail) Moon & Baldridge 2007, POS tagging for Middle English through alignment; Flachs et al. 2019, historical text normalisation | `$RESEARCH_RAW/aclbulk-2007-moon-part-speech-tagging-middle-english/` (CC BY-NC-SA: tests only), `$RESEARCH_RAW/acl-2019-flachs-historical-text-normalization-delayed-rewards/` (CC BY 4.0) | | candidates for the normalisation layer |

## Texts (examples and counts)

| Text | Where | Licence | Used for |
|---|---|---|---|
| The Torah, JPS 1917 (*The Holy Scriptures According to the Masoretic Text*, Jewish Publication Society of America), eBible.org USFM | `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`; sentences `$MOVA_DATA/runs/cycle-torah-2026-10-17/torah.tsv`, verse references `torah.tsv.verses` | public domain | all `jps:` examples and counts |
| King James Version, 1769 text, eBible.org | `$RESEARCH_RAW/corpus-religion/bible-kjv/` (see the research library guide `docs/papers/corpus-religion.md`) | public domain outside the UK (Crown letters patent in the UK) | KJV/JPS comparison (chapter 1.4) and KJV examples |
| 0.3 tales pool (Project Gutenberg books: PG 11027, 2591, 503, 540, 3454, 1597, 27200, 14241, 7439, 7885, 22373, 29672, 236, 976, 29021, 2306, 16, 21, 128, 11339 …) | `$MOVA_DATA/runs/cycle-tales03-2026-10-16/pool.tsv`; raw books `$MOVA_DATA/raw/tales-pg-<n>/` | public domain | `book:` examples, tales counts |
| Universal Dependencies English EWT 2.18, train | `$UD_ROOT/UD_English-EWT/en_ewt-ud-train.conllu` | CC BY-SA 4.0 — **tests and measurements only**, nothing copied | counts of archaic forms in the training data, conventions for `you guys`, `mine`, `Mood=Sub` |

## Mova's own data and code (measured)

| Item | Path | Used for |
|---|---|---|
| Torah parse database | `$MOVA_DATA/corpora/judaism/parsedb/current.conllu` (2026-10-17 21:29) | the db audit (chapters 3–12) |
| Torah cycle logs | `$MOVA_DATA/runs/cycle-torah-2026-10-17/root/round{1,3,5,7,9,121}/prepare.log`, `mmm.conllu`, `mmm-gparse.conllu`, `final.conllu`, `report.txt`, `cycle.tsv` | refusals, stage-by-stage lemma survival |
| Small model | `$MOVA_DATA/runs/mmm-parse-2026-10-15/en-agreed.bin`, run with `en annotate` built from `main` at `f3e616747` | «SLM now» |
| Lemmatizer | `en/src/morph.rs` (`archaic`, commit `3e639a08e`), `en/data/forms.tsv` (AGID entries) | chapter 4.5 |
| Gate | `en/src/udreg.rs`, `en/data/ud-registry-en.tsv`; `annot/src/main.rs` (`drafts_from`, commit `d3101c072`) | chapter 5, 12 |
| Graph parser lexicon fill | `gparse/src/data.rs` (`Lex::fill`), `gparse/src/main.rs` (`annotate`) | chapter 12.5 |
| Pragmatic features | `prag/src/feats.rs` | chapter 7, 12.7, 13 |
| World | `world/src/rerank.rs`, `world/src/induce.rs`, `world/src/worldbook.rs`, `global/seeds/shortcuts/selection.md` | chapter 10, 12.7 |

## Web pages consulted

| Page | Read | Used for |
|---|---|---|
| Wikipedia, «Tyndale Bible», https://en.wikipedia.org/wiki/Tyndale_Bible | 2026-10-17 | the «around 90 %» Tyndale share, attributed there to Coggan 1968 (CC BY-SA: cited, not copied) |

## Wanted (not in the library; would improve this manual)

| Work | Why | Status |
|---|---|---|
| W. A. Wright, *The Bible Word-Book: A Glossary of Archaic Words and Phrases in the Authorised Version* (2nd ed., 1884) | the standard list of KJV false friends and obsolete words with their 1611 senses; would replace the hand-made list in chapter 10.3 | public domain; on archive.org — ask the Scientist |
| C. T. Onions, *A Shakespeare Glossary* (1911) | lexicon of Shakespeare's senses | public domain |
| Henry Sweet, *A New English Grammar, Logical and Historical*, Part I (1892) and Part II (1898) | historical accidence and syntax; Mova has `en-gram-sweet-short-historical-1892` (*A Short Historical English Grammar*) and `en-gram-sweet-neg`, not the *New English Grammar* | public domain |
| Arvid Ellegård, *The Auxiliary Do: The Establishment and Regulation of Its Use in English* (1953) | the classic quantitative study behind chapter 7 | in copyright |
| Charles Barber, *Early Modern English* (2nd ed. 1997); Manfred Görlach, *Introduction to Early Modern English* (1991); Terttu Nevalainen, *An Introduction to Early Modern English* (2006); Roger Lass (ed.), *The Cambridge History of the English Language*, vol. III, 1476–1776 (1999) | standard handbooks; would let the manual cite page numbers for morphology and spelling | in copyright — summaries only |
| Jonathan Hope, *Shakespeare's Grammar* (2003); N. F. Blake, *A Grammar of Shakespeare's Language* (2002) | modern Shakespeare grammars | in copyright |
| David Crystal, *Begat: The King James Bible and the English Language* (2010) | the KJV's idioms in modern English (chapter 11) | in copyright |
| Jon Nielson & Royal Skousen, «How Much of the King James Bible Is William Tyndale's?», *Reformation* 3 (1998) | the Tyndale share (chapter 1.3) | article |
| Terttu Nevalainen & Helena Raumolin-Brunberg, *Historical Sociolinguistics* (2nd ed. 2016) | the `-s`/`-th` change (cited via Hejná & Walkden) | in copyright |
| Penn-Helsinki Parsed Corpus of Early Modern English (PPCEME) | a gold treebank of EModE (phrase structure) for a real test set | LDC licence — tests only if obtained |
| EEBO-TCP Phase I texts | unnormalised EModE at scale (1475–1700) | public domain (Phase I) — would need the spelling layer of chapter 2 |
| The First Folio (1623) transcription, Gutenberg or Folger Shakespeare (CC BY-NC) | Shakespeare's text for chapter 14.5 | Gutenberg texts public domain; Folger NC — tests only |
| Revised Version (1885) Pentateuch | the missing link between KJV and JPS 1917 (chapter 1.3) | public domain |
