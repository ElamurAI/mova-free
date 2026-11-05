# 13. Biblical Hebrew in Mova: the tools

This chapter gathers in one place what the other chapters say piece by piece: which of Mova's tools read Ancient Hebrew, what they produce, how they render each construction of the grammar, how well they do it (measured, with dates), and where they stop. Everything here can be run from the repository; the data lives under `$MOVA_DATA`, never in git.

## Contents

1. The pipeline at a glance
2. The source: MACULA Hebrew
3. The trees: `macula` and its CoNLL-U
4. Tokens: what a "word" is
5. FEATS and MISC
6. The relations: head rules and the UD conventions for Hebrew
7. Measured against UD Ancient Hebrew PTNK
8. The pragmatics: `macula prag`
9. The dictionary
10. The Torah course and the parse database
11. A construction-by-construction map
12. Known defects of the data
13. Gaps and next steps
15. Sources

---

## 1. The pipeline at a glance

```
MACULA Hebrew lowfat XML (CC BY 4.0)
   │  macula <lowfat-dir> <out-dir> GEN,EXO,LEV,NUM,DEU
   ▼
torah-wlc-macula/{GEN,EXO,LEV,NUM,DEU}.conllu + verses.tsv          ← trees (hbo)
   │                    │                         │
   │ macula prag        │ dictionary hebrew       │ macula compare (PTNK, NC, a measure only)
   ▼                    ▼                         ▼
hbo-prag.tsv         dict/torah-en.tsv(.examples)  UAS/LAS on test and dev
   │
   ▼
parsedb-hbo/ (seed trees + prag for the cycle)   $MOVA_DATA/school/courses/torah/*.lesson (the Torah course)
```

Three things are worth stating at the start, because they decide how far any claim in this manual about "what Mova knows" can go:

- **The morphology is MACULA's, not ours.** Mova has no Hebrew morphological analyser of its own. Lemma, part of speech, gender, number, person, binyan and verb form (qatal, wayyiqtol …) all come from MACULA, which in turn takes the text and morphology from the Open Scriptures Hebrew Bible (OSHB).
- **The dependency relations are ours.** They are read off MACULA's constituency-like word groups by deterministic head rules (`tools/macula/src/main.rs`), not hand-annotated in UD.
- **The pragmatics is ours and is rule-based.** `macula prag` reads six fields per sentence from the tree alone, without a teacher model.

Every step is deterministic: two runs give the same files byte for byte (tools/macula/README.md, measured 2026-10-17).

## 2. The source: MACULA Hebrew

MACULA Hebrew (Clear Bible / Biblica, with the J. Alan Groves Center's Westminster Hebrew Syntax and the OSHB morphology; https://github.com/Clear-Bible/macula-hebrew/) is released under CC BY 4.0, which is why it can be the base of what Mova builds (licence/DATA.md). It gives the Westminster Leningrad Codex (WLC), sentence by sentence, as nested word groups:

- clauses (`cl`) whose children carry roles: `s` subject, `v` verb, `o` and `o2` objects, `io` indirect object, `pp` prepositional complement, `adv` adverbial, `p` predicate (in verbless clauses);
- phrases: noun (`np`), preposition (`pp`), adjective, number (`nump`), relative clause (`relp`), coordinations;
- for every word: the pointed form, lemma, class, morphological attributes, an English gloss, Strong's number (with MACULA's lettered extensions such as `2050b` for the conjunction ו), a word reference (`GEN 1:1!3`), a `type` (the verb form, or the kind of pronoun, article, particle), and for many verbs a `frame` naming the words that fill its argument roles.

The fields from the Semantic Dictionary of Biblical Hebrew (SDBH: `sdbh`, `lexdomain`, `coredomain`, `contextualdomain`) are deliberately left out of what Mova carries (tools/macula/README.md).

Mova's English side of the same text is the JPS 1917 translation (public domain, from eBible.org) at `$MOVA_DATA/corpora/judaism/text/torah-jps1917`, split into sentences by `school-sents usfm` with the same verse addresses, so Hebrew and English line up verse by verse.

## 3. The trees: `macula` and its CoNLL-U

```
cargo run --release -p macula -- <lowfat-dir> $MOVA_DATA/corpora/judaism/text/torah-wlc-macula GEN,EXO,LEV,NUM,DEU
```

Output, one file per book, one CoNLL-U block per MACULA sentence:

```
# sent_id = wlc:GEN:2
# verse = GEN.1.3
# text = וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר וַֽיְהִי־אֽוֹר׃
1  וַ      וְ      CCONJ _ _                                          2 cc    _ Gloss=and|…|Strong=2050b|SpaceAfter=No
2  יֹּ֥אמֶר אָמַר   VERB  _ Gender=Masc|HebBinyan=QAL|Number=Sing|Person=3 0 root  _ Gloss=he.said|…|Strong=0559|Frame=A0:010010030021;|Type=wayyiqtol
3  אֱלֹהִ֖ים אֱלֹהִים NOUN _ Gender=Masc|Number=Plur                  2 nsubj _ Gloss=God|…|Strong=0430
4  יְהִ֣י   הָיָה   VERB  _ Gender=Masc|HebBinyan=QAL|Number=Sing|Person=3 2 ccomp _ Gloss=let.it.be|…|Type=jussive
5  א֑וֹר    אוֹר    NOUN  _ Number=Sing                                4 nsubj _ Gloss=light|…|Strong=0216
6  וַֽ      וְ      CCONJ _ _                                          7 cc    _ Gloss=and|…
7  יְהִי־   הָיָה   VERB  _ Gender=Masc|HebBinyan=QAL|Number=Sing|Person=3 2 conj  _ Gloss=it.was|…|Type=wayyiqtol
8  אֽוֹר    אוֹר    NOUN  _ Number=Sing                                7 nsubj _ Gloss=light|…
```

The ids: `wlc:<BOOK>:<n>` counts sentences from 0 within each book; `# verse` gives the verse in the OSIS-like form `GEN.1.3`; `verses.tsv` maps every id to its verses. In the Torah as converted on 2026-10-17 no MACULA sentence spans two verses, so every id maps to exactly one verse; long verses can be split over several sentences.

**Size of the Torah (2026-10-17).**

| book | sentences | words (tokens) |
|---|---:|---:|
| GEN | 1 533 | 32 365 |
| EXO | 1 213 | 26 102 |
| LEV | 859 | 18 912 |
| NUM | 1 289 | 25 253 |
| DEU | 959 | 23 175 |
| **all** | **5 853** | **125 807** |

0 trees fail the check (one head per word, one root, no cycle); a negative control in the crate's tests (`check_shows_red`) shows that the check can fail. 598 word groups had no head rule and took their first child. The conversion takes about 5 s.

**UPOS distribution (2026-10-17):** NOUN 29 610, ADP 21 085, VERB 17 922, CCONJ 14 236, PRON 13 768, DET 9 110, PROPN 8 004, SCONJ 3 420, ADV 2 868, NUM 2 422, ADJ 2 273, X 565, INTJ 394, PART 130.

The X tokens are MACULA's paragogic and directional letters split off as words of their own: lemma ה 460 (directional he 392 plus paragogic he 73, minus overlap) and lemma נ 105 (paragogic nun). UD has no part of speech for them.

## 4. Tokens: what a "word" is

MACULA segments the text more finely than the space-separated orthographic word, and Mova keeps its segmentation (the UD guidelines for Ancient Hebrew do the same: prefixed prepositions, the conjunction ו, the article and the pronominal suffixes are tokens of their own; UD hbo docs, `_hbo/index.md`, CC BY-SA, paraphrased):

| orthographic word | tokens | example |
|---|---|---|
| ו + verb | `וַ` + `יֹּאמֶר` | GEN.1.3, `wlc:GEN:2` |
| preposition + noun | `בְּ` + `רֵאשִׁית` | GEN.1.1, `wlc:GEN:0` |
| article + noun | `הַ` + `שָּׁמַיִם` | GEN.1.1 |
| preposition + hidden article + noun | `לָ` + (empty) + `רָקִיעַ` | GEN.1.7, `wlc:GEN:6` |
| noun + suffix | `אָבִ` + `יו` | EXO.21.15 |
| verb + object suffix | stem + `ךָ` | DEU.5.12 |

Two consequences follow for anyone reading the files:

1. **Empty FORMs.** When the article is absorbed into the vowel of a prefixed preposition (לָרָקִיעַ = lə + ha + rāqîaʿ), MACULA still gives the article as a word, and its pointed form is empty. In the Torah 1 679 tokens have an empty FORM, all of them the article הַ (2026-10-17). Their LEMMA is הַ and their UPOS DET, so they still count for definiteness.
2. **Split points inside a syllable.** A suffix token's FORM can begin with the vowel point that belongs to the preceding consonant (the FORM of the suffix in DEU.5.12 begins with a shewa: `ְךָ`). The FORM column therefore does not always hold a pronounceable string; rebuild the surface word by joining the tokens marked `SpaceAfter=No`.

The `# text` line is the full pointed and accented verse text as in the WLC, with maqqef and sof pasuq, and is the safe source for display.

## 5. FEATS and MISC

**FEATS** carries only what MACULA marks on the word and the UD Hebrew documentation names (UD hbo feature index, `_hbo/feat-index.md`):

| feature | values in our files | source attribute |
|---|---|---|
| `Gender` | Masc, Fem, Com | `gender` |
| `Number` | Sing, Plur, Dual (779 dual tokens) | `number` |
| `Person` | 1, 2, 3 | `person` |
| `HebBinyan` | QAL 12 757, HIPHIL 2 069, PIEL 1 859, NIPHAL 883, HITHPAEL 179, HOPHAL 149, PUAL 92, POEL 13, POLEL 7, HITHPOLEL 7, PILPEL 5, HITHPALPEL 3, NITHPAEL 1, PALEL 1, UNKNOWN 30 | `stem`, upper-cased |

Not carried (yet): `State` (absolute/construct), `Definite`, `VerbForm`, `Aspect`, `Mood`, `Tense`, `Voice`, `PronType`, `NumType`. PTNK carries several of these (README changelog: `VerbForm` and `PronType` from v2.11, `Case` from v2.17). In our files the construct state is visible only in the tree (`compound:smixut`, section 6), and the verb form only in MISC `Type`.

**MISC** carries, in this order: `Gloss` (MACULA's English, with dots for multiword glosses: `he.created`), `Ref` (MACULA's word reference, `GEN_1:1!2`), `MId` (MACULA's word id, used to align with PTNK), `Strong` (Strong's number, four digits, with MACULA's letter extensions), `Frame` (argument frame), `Type`, and `SpaceAfter=No` inside a written word.

**MISC `Type` counts in the Torah (2026-10-17).**

| Type | count | Type | count |
|---|---:|---|---:|
| pronominal (suffix) | 12 172 | negative | 1 576 |
| definite_article | 9 114 | participle_active | 1 472 |
| wayyiqtol | 4 187 | personal (pronoun) | 1 400 |
| direct_object_marker | 4 153 | demonstrative | 1 197 |
| yiqtol | 3 776 | imperative | 782 |
| qatal | 2 694 | gentilic | 436 |
| weqatal | 2 434 | participle_passive | 395 |
| cardinal_number | 2 253 | directional_he | 392 |
| relative | 1 911 | interrogative | 342 |
| infinitive_construct | 1 739 | affirmation | 269 |
| adjective | 1 503 | infinitive_absolute | 255 |
| ordinal_number | 235 | jussive | 192 |
| cohortative | 129 | paragogic_nun | 105 |
| exhortation (נָא) | 95 | paragogic_he | 73 |
| interjection | 37 | | |

**Frame.** `Frame=A0:010010010031;A1:010010010052;010010010072;` names, by MACULA word id, the fillers of the verb's roles: A0 the agent-like argument, A1 the patient-like one, A2 a third; several ids after one role are a coordination (GEN.1.1: A1 = "the heavens" and "the earth"). 17 087 tokens carry a frame (GEN 4 847, EXO 3 540, DEU 3 376, NUM 2 950, LEV 2 374; 2026-10-17). Frames reach across clauses (a zero subject's frame points to the noun of an earlier clause), so they are the data's own participant tracking; chapter 9 uses them.

## 6. The relations: head rules and the UD conventions for Hebrew

The rules (tools/macula/README.md, `head_child()` and `relation()` in `src/main.rs`):

- a clause is headed by its verb; with no verb by its predicate; else by its first clause, else its first content word;
- a preposition phrase is headed by its noun (or clause), the preposition being `case`;
- a noun phrase is headed by its first content word: the first noun of a construct chain, the noun after the article (`det`) or after the object marker (`case`);
- in a coordination the first member heads, later members are `conj`, and each conjunction is the `cc` of the member after it;
- roles become relations: `s` → nsubj, `o`/`o2` → obj, `io` → iobj, `pp` → obl (advcl if its head is a verb), `adv` → advmod / obl / advcl, `p` → xcomp beside a verb and nsubj otherwise.

On top of these, the UD conventions for Hebrew (introduced in commit ed0701a37, 2026-10-17):

| construction | relation | example |
|---|---|---|
| construct chain, second member | `compound:smixut` (7 421) | פְּנֵי תְהוֹם "the face of the deep", GEN.1.2 |
| pronoun suffix on a noun | `nmod:poss` (6 596) | אָבִיו "his father", EXO.21.15 |
| subordinators כִּי אִם לְמַעַן פֶּן טֶרֶם יַעַן עֵקֶב לְבִלְתִּי | `mark`, UPOS SCONJ | כִּי טוֹב GEN.1.4 |
| relative אֲשֶׁר | `mark` of the relative clause, which is `acl:relcl` (1 475) | GEN.1.7 |
| demonstrative after a noun (הַזֶּה) | `det` (but 286 still `amod`, section 12) | |
| infinitive after a preposition | `advcl` (1 744 advcl in all) | |
| parts of a compound number | `flat` (450) | שִׁבְעִים וְשִׁבְעָה GEN.4.24 |
| clause right after a verb of saying (אמר דבר צוה ענה שאל קרא), no conjunction between | `ccomp` (1 051) | יְהִי אוֹר GEN.1.3 |
| ו at the head of a clause | `cc` on the clause's head; a conjunction never hangs on another | |

**Relation counts in the Torah (2026-10-17):** case 20 761, cc 14 183, conj 13 460, obl 11 666, det 9 242, nsubj 7 982, obj 7 561, compound:smixut 7 421, nmod:poss 6 596, root 5 853, nmod 5 105, advmod 3 731, mark 3 395, appos 1 966, advcl 1 744, acl:relcl 1 475, amod 1 369, ccomp 1 051, nummod 572, flat 450, acl 160, dep 59, discourse 5.

A worked tree, GEN.1.7 (`wlc:GEN:6`), "And God made the firmament, and divided the waters which were under the firmament from the waters which were above the firmament; and it was so" (JPS 1917):

```
וַ   cc ──────────────┐
יַּעַשׂ  root (wayyiqtol, QAL) ◄─────────────────────────────┐
אֱלֹהִים nsubj → יַּעַשׂ                                       │
אֶת־ case → רָקִיעַ ; הָ det → רָקִיעַ ; רָקִיעַ obj → יַּעַשׂ    │
וַ cc → יַּבְדֵּל ; יַּבְדֵּל conj → יַּעַשׂ (wayyiqtol, HIPHIL)   │
בֵּין case → מַּיִם ; הַ det ; מַּיִם obl → יַּבְדֵּל              │
   אֲשֶׁר mark → רָקִיעַ ; מִ case ; תַּחַת case ; לָ case ; (הַ) det
   רָקִיעַ acl:relcl → מַּיִם        ← the verbless relative clause is headed by its predicate's noun
וּ cc ; בֵין case ; הַ det ; מַּיִם conj → מַּיִם
   אֲשֶׁר mark ; מֵ case ; עַל case ; לָ case ; (הַ) det ; רָקִיעַ acl:relcl → מַּיִם
וַ cc ; יְהִי conj → יַּעַשׂ ; כֵן advmod → יְהִי
```

The verbless relative clause "which (were) under the firmament" has no verb, so the rules make the noun of its predicate phrase its head, and the stacked prepositions מִן + תַּחַת + לְ all become `case` of that noun. This is the generic reading the README warns about: special constructions get the general rules.

## 7. Measured against UD Ancient Hebrew PTNK

UD Ancient Hebrew PTNK (Swanson & Tyers 2022; UD 2.18 at `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/`) is the hand-verified UD treebank of the same text. Its README declares CC BY-NC 4.0 (its `LICENSE.txt` names CC BY-SA 4.0; we follow the stricter reading), so it is only a measure: nothing of it is copied, trained on, or shipped. Its morphology comes from the ETCBC BHSA (CC BY-NC) and its syntax was produced by a rule-based parser of about 300 rules over the MACULA trees and then checked by hand (Swanson & Tyers 2022, abstract). Its splits cover GEN 1–18 (dev), 19–30 (test), 31–50 (train); EXO, LEV, NUM (train); DEU 1–6 (dev), 7–12 (test), 13–34 (train); and Ruth (train).

Because PTNK's words carry MACULA ids, `macula compare` lines the two up word for word. LAS is counted on the relation without its subtype. The rules are decided on train and dev; test is read once per change of the rules.

| 2026-10-17 | test UAS | test LAS | dev UAS | dev LAS |
|---|---:|---:|---:|---:|
| first head rules | 86.24 | 73.79 | 84.59 | 70.87 |
| first rules, construct read as nmod | | 78.31 | | |
| UD conventions for Hebrew | **88.38** | **82.23** | 86.61 | 79.98 |

Where the two still part (train, right head, other relation; tools/macula/README.md): `parataxis` → our `conj` 788, `conj` → `nmod` 385, `obl` → `advmod` 278, `det` → `amod` 263, `mark` → `det` 250, `advcl` → `conj` 247. PTNK also reads הָיָה as a copula where we keep it the head. Each of these is a choice of analysis, discussed with the construction in chapters 8 and 9:

- **parataxis vs conj.** PTNK links juxtaposed clauses (especially speech and asyndetic clauses) as parataxis; our rules coordinate every clause sequence.
- **conj vs nmod.** Where a noun sequence is an apposition-like list in PTNK, MACULA's grouping may put a coordination.
- **det vs amod.** A demonstrative or article on an adjective (הַגָּדֹל) is `det` in PTNK in places where we have `amod` of the adjective, or the converse.
- **mark vs det.** The article on a participle used as a relative ("the one who …") is `mark` in places in PTNK.

## 8. The pragmatics: `macula prag`

```
cargo run --release -p macula -- prag $MOVA_DATA/corpora/judaism/text/torah-wlc-macula $MOVA_DATA/corpora/judaism/hbo-prag.tsv
```

For every sentence it writes the same six fields the English pragmatics uses (`prag/`): form, act, indirect, hedge, polarity, voice, plus `teacher=rules`. The rules (`hebrew_prag()` in `tools/macula/src/main.rs`) look only at the root and the words that hang on it directly:

- **narrator's frame:** the root is a wayyiqtol or qatal not in the 1st or 2nd person → voice narrator, form decl; act narrate for a wayyiqtol, assert for a qatal. "And God said: …" is narration, whatever the speech inside says.
- **volitive:** a root imperative, jussive or cohortative → form imp, act request.
- **command of the law:** a root yiqtol or weqatal in the 2nd person → act request, indirect 1 ("thou shalt …").
- **questions:** an interrogative in the root's clause; מִי מָה מַדּוּעַ אֵיךְ מָתַי לָמָּה אַיֵּה → form wh; the interrogative הֲ → form q; act ask.
- **hedge:** נָא in the root's clause → hedge 1.
- **polarity:** לֹא, אַל or אַיִן in the root's clause → neg.
- **voice:** character if the root is in the 1st or 2nd person, or is volitive, or carries נָא, or a word of the root's clause is 1st or 2nd person; else narrator.

**Distribution over the Torah (2026-10-17, 5 853 sentences):**

| field | values |
|---|---|
| form | decl 5 591, imp 211, wh 34, q 17 |
| act | assert 2 869, narrate 2 010, request 927, ask 47 |
| hedge | 1 in 17 |
| polarity | neg 427 |
| voice | narrator 4 149, character 1 704 |

**Agreement with the English reading of the same verses** (`macula prag-compare`, JPS 1917 read by Mova's English pragmatics; commit ed0701a37, 2026-10-17): form 89%, voice 66–74%, polarity 75–82%, act about 50%. This is a first baseline. The low act score is largely a matter of categories: the English side does not have "narrate", and the Torah's laws in the 2nd person yiqtol are "request" in Hebrew but often "assert" in the English reading.

## 9. The dictionary

```
dictionary hebrew <trees-dir> BDB_Dictionary.jsonl LexicalIndex.xml torah-en.tsv [en-pool.tsv] [--lang arc] [--jastrow Jastrow_Dictionary.jsonl]
```

`$MOVA_DATA/corpora/judaism/dict/torah-en.tsv` has one row per word of the Torah with the columns `key, lemma, count, glosses, oshb_def, bdb_headword, bdb_short, bdb_full, why`; `tanakh-en.tsv` has the same for the Hebrew words of the whole Bible, and `aramaic-en.tsv` for the Aramaic chapters (keyed by lemma, with Jastrow 1903 in three more columns). How they are made: `tools/macula/README.md`, «The dictionaries»:

- **key:** Strong's number, or `lemma:<lemma>` for MACULA's lettered extensions (`lemma:וְ`, since `2050b` is the conjunction and not Strong's 2050);
- **glosses:** MACULA's commonest English glosses with counts (`and (13503); so (99); …` for ו);
- **oshb_def:** the short definition of the OpenScriptures lexical index (CC BY 4.0);
- **bdb_short / bdb_full:** the entry of Brown–Driver–Briggs 1906 (public domain, Sefaria's digital text): its bold senses as a short meaning, and its whole text without markup. Where several entries share a Strong's number, they are ranked by part of speech, headword, the verses where the word is used that the entry cites, and the glosses; `why` says how the entry was chosen.

`torah-en.tsv.examples` has, per word, its two shortest one-verse sentences in layers: `verse, hebrew, literal` (MACULA's glosses word for word), `literary` (JPS 1917), and an empty `idea` column for a commentary to come.

**Figures (2026-10-18):** 3 486 keys; 3 471 with a BDB entry; 3 477 with an OSHB definition; 5 816 examples. The whole Tanakh: 8 377 keys; Biblical Aramaic: 698 lemmas.

## 10. The Torah course and the parse database

**The course.** `$MOVA_DATA/school/courses/torah/*.lesson` holds 54 lessons, one per weekly portion (parashah, from Sefaria's index), written by a teacher model from the portion's JPS 1917 text; three key verses of each are given as facts in layers taken from the texts — Hebrew (MACULA), word for word, JPS 1917 and the tradition's reading (commit 3dfca471a). The student (the SLM) answers from the world it reads, never from the teacher. Measured scores and the breakdown of its failures are in `docs/school.md` (Torah ★ 236 → 252 and exam 96 → 99 after references like "(Genesis 12:1)" are taken off when a lesson is read).

**The parse database.** `$MOVA_DATA/corpora/judaism/parsedb-hbo/` is a tree store of the kind `tools/cycle` keeps (`current.conllu`, `history.conllu`, `log.tsv`, `prag.tsv`, `verses.tsv`), seeded on 2026-10-17 with all 5 853 MACULA trees (5 853 `seed added` rows in `log.tsv`) and their rule-based pragmatics. It can be given to the cycle as one of `CYCLE_TRAIN_DBS`, so that the one model's training sees Hebrew trees beside the English ones. In the vote over versions of a tree the seed has weight 1, below any teacher model (`tools/cycle/src/db.rs`, `weight()`).

## 11. A construction-by-construction map

| construction (chapter) | where it shows in our files | handled? |
|---|---|---|
| begadkefat, dagesh, vowels (1, 3) | FORM only (pointed) | no rule reads them |
| cantillation accents (2) | FORM and `# text` only | not read; comparisons strip points (`consonants()`) |
| gender, number (4) | FEATS | yes, from MACULA |
| construct state (4, 8) | relation `compound:smixut`; no `State` feature | partly |
| article (4) | token DET, `Type=definite_article`; empty FORM when absorbed | yes |
| pronominal suffix (4, 5) | token PRON, `Type=pronominal`, `nmod:poss` / `obj` | yes; FORM may start with a point |
| object marker אֵת (5, 8) | ADP `case`, `Type=direct_object_marker` | yes |
| binyan (6, 7) | FEATS `HebBinyan` | yes, from MACULA |
| verb form (6) | MISC `Type` (qatal, wayyiqtol …) | yes, but not in FEATS |
| weak roots (7) | LEMMA = dictionary form of the root | lemma only; no analyser of ours |
| verbless clause (8) | head = predicate | generic rule |
| relative clause (8) | `mark` + `acl:relcl` | yes |
| infinitive after a preposition (8) | `advcl` | yes |
| infinitive absolute (6, 8) | `Type=infinitive_absolute`; generic relation | generic rule |
| casus pendens, oaths (8) | generic rules | no special reading |
| direct speech (9) | `ccomp` after a verb of saying; לֵאמֹר as an infinitive | partly |
| wayyiqtol chains (9) | `conj` chains under the first verb; prag act narrate | yes |
| questions, volitives, negation, נָא (5, 9) | `macula prag` | yes, rule-based |
| argument roles (9) | MISC `Frame` | carried, not yet used by a tool |
| lexicon (10) | `torah-en.tsv` | yes |
| Mishnaic Hebrew (11) | nothing | no trees, no tools |

## 12. Known defects of the data

Found while writing this manual (2026-10-17), none fixed yet:

- **Spaces inside FORM.** 592 tokens have a space inside FORM, against the CoNLL-U rule that a word holds no whitespace. Two causes: the paseq ׀ written after a word is kept with it (387 tokens, e.g. `ְךָ֖֣ ׀`, DEU.5.12), and multiword proper names are one MACULA word (205 tokens: בְּאֵר לַחַי רֹאִי "Beer-lahai-roi", קָדֵשׁ בַּרְנֵעַ "Kadesh-barnea", דִּי זָהָב "Di-zahab"). The lemmas most affected: the suffix הוּא 39, יהוה 18, אֵת 18, כֹּל 16, כִּי 16.
- **`dep` for multiword names.** Such names inside a name group produce most of the 59 `dep` relations.
- **Empty FORMs** for the absorbed article (1 679), valid CoNLL-U but surprising for any tool that expects a surface string.
- **No `State` feature.** A construct noun at the end of a chain can only be recognised by its dependent.

Found by the chapter authors (details in the chapters named):

- **Invalid FEATS values** (chapter 6): `HebBinyan=QAL PASSIVE` (6 tokens), `UNKNOWN: V` (24, the hishtaphel/hithpalel of חוה) and `UNKNOWN: U` (6, the hothpaal הָתְפָּקְדוּ) contain a space or colon, which CoNLL-U forbids; 21 other forms of חוה are `HITHPAEL`.
- **Paragogic nun** is a token (UPOS X, lemma נ) attached as `nmod` (chapter 6).
- **Demonstratives** after a noun come out as `amod` in 286 cases, not `det` as the convention says (chapter 4).
- **Numerals** head their noun (MACULA's grouping), so `nummod` is rare (572) (chapter 4); a predicate adjective after הָיָה is attached as `obj` (chapters 4, 8).
- **Speech** (chapters 8, 9): only the first clause after a verb of saying is `ccomp`; later clauses of the same speech attach to the narrator's verb as `conj`, so a negation inside the speech can leak into the narrator's sentence in `macula prag` (DEU.1.42). There is no `cop`, `dislocated` or `vocative`; in conditionals the protasis heads and the apodosis is its `conj`.
- **`macula prag`** (chapters 6, 9): the wh-list lacks אֵיכָה and אֵי (DEU.1.12 read as a yes/no question); the infinitive absolute as command (זָכוֹר DEU.25.17) and the third-person prescriptions of the law (LEV.1.4, EXO.21.15) read as assertions; a request below a root וַיְהִי / הִנֵּה / עַתָּה is not seen (GEN.27.8).
- **`# text` without spaces** in a few verses (GEN.33.4, and the doubly accented Decalogue in EXO.20 and DEU.5): split on tokens, not on the text line (chapters 2, 6).
- **Glosses shifted** across the parts of a split word (GEN.24.67 הָאֹהֱלָה, GEN.12.10) (chapter 4); one Strong number with two lemmas (הלך 227 / הָלַךְ 85) (chapter 6); בֵּית־אֵל duplicated in GEN.35.7 (chapter 8).
- **Ketiv** is not carried: only the qere is in the trees (chapter 1).
- **Homonyms under one Strong's number** (chapter 10): the OSHB or BDB column of `torah-en.tsv` gave the wrong homonym (חֶסֶד → "shame", בַּיִת → "between", שֵׁשׁ "fine linen" → "six"). Fixed 2026-10-18: the entries are now ranked (`tools/macula/README.md`), which changed the BDB entry of 647 Torah words and the OSHB definition of 259.
- **לָמָּה is spelled לָמָה and tagged ADP** in the trees, so the wh-list of `macula prag` never matches it; אָן and אֵיפֹה are missing too (chapter 5).
- **Dictionary** (chapter 6): the OSHB definition of קוּם (6965) is a name; הָיָה gets a sense of the neighbouring entry הָוָה; BDB short fields of verbs often reduce to stem labels.
- **JPS 1917 versification** differs from the Hebrew in GEN.32, EXO.8, DEU.5, DEU.13 and elsewhere; the English pool must be matched by verse map, not by number (chapters 4, 6).

## 13. Gaps and next steps

Marked as not done; each would need the operator's word before an expensive run.

1. A Rust morphological analyser for Biblical Hebrew (root + binyan + form from the pointed word), measured against MACULA on the Torah, so that Mova can read texts MACULA does not cover (the Mishnah, the rest of the Bible beyond what MACULA gives).
2. `VerbForm`, `State`, `Definite` and `PronType` in FEATS, from MACULA `type` and the tree, to come nearer to PTNK.
3. A reading of the accents (chapter 2): the atnach and silluq divide every verse; a test of how often a disjunctive accent falls on the last word of a subtree would turn the Masoretes' phrasing into a check on the trees.
4. Special readings for oaths, the infinitive absolute, casus pendens and the copular הָיָה.
5. `macula prag`: a "narrate" category on the English side, or a mapping, before the act agreement is read as a measure.
6. Using `Frame` for participant tracking in the world of the text.
7. Mishnaic Hebrew: an open, licence-clean text with morphology (chapter 11, "wanted").
8. The paseq and multiword names (section 12).

## 15. Sources

- Clear Bible / Biblica, the Groves Center, OpenScriptures. *MACULA Hebrew Linguistic Datasets.* CC BY 4.0. https://github.com/Clear-Bible/macula-hebrew/
- Open Scriptures Hebrew Bible (morphhb), CC BY 4.0; WLC public domain. https://github.com/openscriptures/morphhb ; local `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/`
- OpenScriptures HebrewLexicon (Lexical Index, BDB XML), CC BY 4.0. `$RESEARCH_RAW/lexres/dictionaries/oshb-hebrewlexicon/`
- Brown, F., Driver, S. R., Briggs, C. A. 1906. *A Hebrew and English Lexicon of the Old Testament.* Oxford. Public domain. Sefaria text `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; scan https://archive.org/details/hebrewenglishlex00browuoft
- Swanson, D., Tyers, F. 2022. "A Universal Dependencies Treebank of Ancient Hebrew." LREC 2022, 2353–2361. https://aclanthology.org/2022.lrec-1.252 ; data UD 2.18 PTNK (CC BY-NC, a measure only), README at `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/README.md`
- Universal Dependencies. Ancient Hebrew documentation (`_hbo`), CC BY-SA, paraphrased. `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/`
- The Holy Scriptures according to the Masoretic Text, Jewish Publication Society 1917. Public domain, via eBible.org.
- Mova repository: `tools/macula/README.md`, `tools/macula/src/main.rs`, `tools/cycle/src/db.rs`, `docs/school.md`, `licence/DATA.md`; commits 57f01449a, 2dd207e22, 585ceae61, ed0701a37, 3dfca471a.
