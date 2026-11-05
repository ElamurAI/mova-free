# macula

MACULA Hebrew lowfat syntax trees of the Hebrew Bible into dependency trees, in CoNLL-U, as Ancient Hebrew (`hbo`) and Biblical Aramaic (`arc`); the Targums as text layers and a verse-aligned index in three languages.

**Source.** MACULA Hebrew Linguistic Datasets, available at https://github.com/Clear-Bible/macula-hebrew/. © Biblica, Inc., CC BY 4.0. The syntax is the Westminster Hebrew Syntax of the J. Alan Groves Center (CC BY 4.0). The text and morphology come from the Open Scriptures Hebrew Bible (CC BY 4.0). The fields from the Semantic Dictionary of Biblical Hebrew (`sdbh`, `lexdomain`, `coredomain`, `contextualdomain`) are left out.

## How it works

MACULA gives each sentence as word groups: clauses, with the roles of their parts (`s`, `v`, `o`, `o2`, `io`, `pp`, `adv`, `p`), and noun, preposition, number and adjective phrases, relative clauses and coordinations. The tool reads a dependency tree off them by head rules that follow the UD conventions for Hebrew and the readings of UD Ancient Hebrew PTNK:

- **clause:** the verb (`v`) heads; with no verb, the predicate (`p`); with neither, the first clause, then the first content word. הָיָה with a predicate is a copula (`cop`, AUX) and the predicate heads: always when MACULA marks the predicate `o`; with a prepositional predicate unless the verb is the narrative וַיְהִי (the phrase with ל and a noun first, then one with ל, then the first).
- **clauses side by side:** in `ClCl2` and `CLandCL2` the last clause heads; the ones before it are `discourse` (הִנֵּה), `dislocated` (a hanging noun), `advcl` (a condition) or `parataxis`. In a group of clauses only, each ו belongs to the clause after it; a clause after a subordinator (כִּי, אִם …) is `advcl`, after no conjunction `parataxis`. After a verb of saying, the clauses that follow the quote's first clause continue the quote (`conj`/`parataxis` of it) up to the next narrative wayyiqtol.
- **לֵאמֹר:** the ל heads and אמר is its `fixed`; the ל is the `mark` of the quoted clause, which is the `ccomp` of the verb of saying (`advcl` after another verb); with no quote in the sentence, לֵאמֹר is the verb's `ccomp`.
- **preposition phrase:** the noun (or clause) heads; the preposition is its `case`. Before a headless relative clause («כַּאֲשֶׁר», «מֵאֲשֶׁר», «אֵת אֲשֶׁר») the preposition or את heads and the clause is its `acl:relcl`. תַּחַת, נֶגֶד, עִמָּד and נֹכַח are nouns in construct (NOUN), and head their phrase.
- **noun and other phrases:** the first content word heads, as in the first noun of a construct chain, or the noun after the article (`det`) or the object marker את (`case`). A number, adverb or adjective before a noun («seven days») leaves the noun the head. Groups of function words only (מֵ + עִם, וְ + עַתָּה) are spliced into the group above, so each word hangs on the phrase it introduces.
- **coordination:** the first member heads, and each later member of the same kind is `conj`, also in lists with no conjunction (`NpNpNp`, `2PpaPp`, `PpPp` …). Each conjunction is a `cc` of the member after it; גַּם and אַף are `advmod`; כִּי אִם is one subordinator (אִם `fixed`).
- **roles become relations:** `s` → nsubj, `o` → obj (a clause: ccomp), `o2` → xcomp, `io` → iobj, `pp` → obl (an infinitive: advcl; xcomp after יכל, צוה, נתן, יסף, מאן … ), `adv` → advmod / obl (a noun phrase) / obl:npmod (the infinitive absolute) / advcl / discourse (נָא), `p` → xcomp beside a verb.
- **inside phrases:** the construct chain (`NPofNP`, `QuanNP`, `AdjpofNp`) is `compound:smixut`, a pronoun suffix nmod:poss, an apposition appos. A relative clause is acl:relcl (on כָּל when the chain begins with it), a participle after a noun too (its article is then the relative `mark`), an adjective amod, a demonstrative (זֶה, זֹאת, אֵלֶּה, הַהוּא) det, a number nummod; the parts of a number are `flat`, or `conj` after ו.

A group with no rule for its head takes its first child.

Words are MACULA's: prefixes (ו, ה, ב, ל …) and suffix pronouns are words of their own, and a written word of more than one is a multiword token (as in PTNK); the article swallowed by a preposition (בַּ = בְּ + הַ) is written ה. A name MACULA keeps as one word with a space (בְּאֵר שָׁבַע) is a word per part, `flat:name`. The paseq ׀, sof pasuq ׃ and the paragraph marks ס, פ are PUNCT tokens. FORM and LEMMA are in NFC. FEATS holds gender, number, person, PronType and the verb stem (`HebBinyan`, letters only). MISC holds the English gloss, the word reference, Strong's number and MACULA's argument frame (`Frame=A0:…;A1:…`, who does what), where it has one, and `Lang=arc` on the Aramaic words. The files are made to pass the UD validator at level 2 (`--lang hbo`; the Aramaic with `--lang arc`); level 3 is not met (mostly `advmod` on a word that is no ADV).

Every tree is checked before it is written: one head per word, one root, no cycle (a negative control in the tests shows red). Two runs give the same files byte for byte.

## Run

```
macula <lowfat-dir> <out-dir> [BOOK,…]
```

It writes `<out-dir>/<BOOK>.conllu`, with `# sent_id = wlc:<BOOK>:<n>` (one document per book) and `# verse = GEN.1.1`. It also writes `<out-dir>/verses.tsv`, which maps ids to verses. `school-sents usfm` gives English translations the same verse map, so the two line up verse by verse.

The Aramaic sentences go apart, to `<out-dir>/arc/<BOOK>.conllu` (`# sent_id = wlc-arc:<BOOK>:<n>`) and `arc/verses.tsv`. A sentence goes by the language of most of its words (MACULA's `lang="A"`).

Words are written in the order of the text. MACULA gives them in the order of its groups, which sometimes takes a word from further on (Ruth 1:1: «וַיֵּלֶךְ אִישׁ מִבֵּית לֶחֶם», אִישׁ in the subject group after the place). The heads are read in MACULA's order, then the words are sorted by their id and the heads renumbered. Comparisons with PTNK match words by id, so the order does not affect them.

`tools/hbo` trains Mova's own parser for raw Hebrew on these trees.

## Biblical Aramaic

MACULA marks every word with its language (`lang="H"` or `"A"`). The Aramaic words are Daniel 2:4b–7:28, Ezra 4:8–6:18 and 7:12–26, Jeremiah 10:11 and two words of Genesis 31:47. A sentence most of whose words are Aramaic goes to `<out-dir>/arc/` (UD code `arc`), as above. Only two sentences mix the languages: Daniel 2:4 (the Hebrew frame «and the Chaldeans spoke to the king in Aramaic», then their words) goes to Aramaic, Genesis 31:47 (the name יְגַר שָׂהֲדוּתָא) stays Hebrew. Every Aramaic word has `Lang=arc` in MISC (in either folder), and a Hebrew word in an Aramaic sentence `Lang=hbo`.

The Aramaic trees use the same head rules, FEATS and MISC as the Hebrew ones; the stem is in `HebBinyan` too (PEAL, PAEL, HAPHEL, HITHPEEL …). Three lists have Aramaic members, used only for Aramaic words: the copula הֲוָה, the demonstratives דְּנָה, דָּא, אִלֵּין, דֵּךְ, אִלֵּךְ, דִּכֵּן (`det`), and the subordinator הֵן «if». The rest (verbs of saying אמר, ענה) share their consonants with Hebrew.

Limits: there is no Aramaic treebank to measure against, so the Aramaic trees are unmeasured. MACULA's English glosses in the Aramaic chapters are shifted by one word around the emphatic article (the noun «the», the article «king»), and are carried as they are. The emphatic ending א is a word of its own (DET, `det`), as MACULA splits it.

## The whole Tanakh

`macula <WLC lowfat> <out>` converts all 39 books; the corpus keeps `<out>` as `text/tanakh-wlc-macula` and `<out>/arc` as `text/tanakh-arc-macula` (Aramaic in DAN, EZR and JER).

## The Targums and the three-language index

```
macula targums <sefaria Tanakh/Targum dir> <text-dir>
macula tanakh3 <text-dir> <en-pool.tsv> <out-dir>
```

`targums` copies the Targums from Sefaria's export as text layers, verse by verse (`BOOK.C.V` → text, markup off, NFC): Targum Onkelos (the Torah, the toratemetfreeware edition), Targum Jonathan (the Prophets, Mikraot Gedolot) and the later Targums of the Writings (Psalms, Proverbs, Job, the Song, Ruth, Lamentations, Ecclesiastes, Esther from Mikraot Gedolot; Chronicles from Wikisource). Only an edition whose licence is «Public Domain» is copied; any other is refused (a test shows red on CC BY-SA and NC). Each layer has a `SOURCES.tsv`.

`tanakh3` writes one TSV per book, a row per verse in the Hebrew numbering: the verse, the Hebrew sentence ids, the Aramaic (Biblical Aramaic sentence ids, and the Targum verse as `onkelos:`, `jonathan:` or `targum:BOOK.C.V`) and the English sentence ids (the pool's `.verses`). A sentence that spans verses is listed at each.

The Writings' Aramaic is missing where there is no Targum (Daniel's Hebrew chapters, Ezra, Nehemiah); the verses of Daniel and Ezra in Aramaic have the Aramaic and no Hebrew. A few verses have no English sentence of their own: JPS joins them to the verse before inside one sentence (Psalm titles, Numbers 26:1).

The English follows the Hebrew numbering: eBible's JPS 1917 gives JPS's own (Hebrew) numbers in brackets where they differ from the English ones, and `school-sents usfm … --hebrew-verses` maps each sentence to the Hebrew verse it begins in (the marks are taken off before the cut, so the ids are the same with and without the flag). In the Ten Commandments (Exodus 20, Deuteronomy 5) JPS counts by the upper accents and the Westminster Leningrad Codex by the lower ones, as the English does, so there the English numbering stays.

There is no Aramaic parser: the Targums are text only, without trees.

## Comparing with UD PTNK

UD Ancient Hebrew PTNK (UD 2.18) is CC BY-NC, so it serves only as a measure. Its words carry MACULA ids, so they line up word for word (`macula compare`). LAS is counted on the relation without its subtype, and lemmas compare by their consonants, since the order of points differs between sources. The rules are decided on train and dev; the test split is read once per change of the rules.

The head rules were made on the Torah. Where the trees still differ from PTNK, it is mostly a choice of analysis: gapping is not read (PTNK `orphan`), a second accusative of material («overlay it with gold») is `obl` where PTNK has `xcomp`, and many head differences are where PTNK's sentences and MACULA's differ (a word is the root in one and a `conj` or `parataxis` in the other).

## The dictionaries

The English dictionaries of the words of these trees (the Hebrew Bible, Biblical Aramaic, Targum Onkelos; Brown–Driver–Briggs 1906, the OpenScriptures lexical index, Jastrow 1903) are built by the Elamur Dictionary's builder, a closed tool. `macula` keeps the readers it shares with it, in `src/lib.rs`: `unescape`, `html_text` (the text of an entry's HTML), `english_verses`, `hebrew_verses`, `verse_key` and `BOOKS`. The Elamur Dictionary's site writes the dictionaries' pages from their tables.

## Pragmatics

`macula prag` reads each Hebrew sentence into the columns of the parse database's `prag.tsv`: form, act, indirect, hedge, polarity, voice. It does this by rules on MACULA's words: the verb type in MISC `Type=` (wayyiqtol, imperative, jussive, cohortative, yiqtol, weqatal, …), person in FEATS, interrogatives, negatives and נָא. The code is in `src/prag.rs`. The rules read words only (multiword tokens and PUNCT are left out).

```
macula prag <trees-dir> <out.tsv> [--with rule,…] [--without rule,…]
macula prag-compare <out.tsv> <trees-dir>/verses.tsv <en prag.tsv> <en pool.tsv.verses>
macula prag-quotes <en pool.tsv>
```

The rules are also a library call, `macula::prag::read_sentence(rows, open_before, &Rules)`, which `roundtrip mixed` uses on the trees of the Hebrew parser.

`prag` adds four columns after `teacher`:

- `speech`: `-` narration, `quote` a frame and its quote, `frame` a frame whose quote is in the next sentence, `cont` inside a quote begun earlier, `bare` speech with no frame;
- `marked`: the sentence shows speech on its own, through a first or second person, a volitive, a question word or נָא;
- `negation`: there is a negative word anywhere in the sentence;
- `frame`: the frame's own reading, `form/act/negation`.

`prag-quotes` prints the JPS quote state of each English sentence, so the speech measure can be checked by eye.

### How it works

1. **The sentence is cut into frame and quote.**
   - לֵאמֹר opens a quote whatever verb comes before it, and everything after it is the quote. If only its own words follow («לֵאמֹר לוֹ»), the quote is in the next sentence. On the current trees לֵאמֹר is not taken for a quote of its own verb.
   - Otherwise the `ccomp` our trees hang on a verb of saying (אמר, דבר, צוה, ענה, שאל, קרא) is the quote, together with the later clauses on that verb.
   - A narrator's wayyiqtol in the third person ends the quote: «וַיְהִי אוֹר», and there was light.
   - קָרָא with שֵׁם as its object names something. It does not speak.
   - In a multi-verb frame (וַיַּעַן … וַיֹּאמֶר) the later verb takes the quote.
2. **The quote is read as a character's speech, and the frame as narration.** The row follows the English convention: a sentence that holds a quote takes the quote's form and act and voice «character». The frame's reading is kept in `frame`.
3. **Speech is carried from sentence to sentence within a book.**
   - A frame that ends a verse («And the LORD spoke unto Moses, saying:») and a quote that reaches the end of its verse both leave a quote open.
   - The next sentence goes on in it (`cont`) until a narrator takes the story up again. That is a wayyiqtol in the third person with no first or second person in its clause, or a qatal with no mark of speech in the sentence.
4. **The speech is read.**
   - An imperative, jussive or cohortative, or an infinitive absolute heading the clause, gives form `imp` and act `request`.
   - A yiqtol or weqatal in the second person («thou shalt (not) …») gives act `request` with `indirect` 1.
   - A question word (מִי, מָה, אֵיךְ, …) gives form `wh`, and interrogative הֲ gives `q`. A question takes act `ask` unless its first clause is a volitive.
   - A yiqtol or weqatal in the first person gives `promise`.
   - The form is the first clause's. The act is that of any clause.
   - נָא gives hedge 1.
   - Narration is `decl`, with act `narrate` for a wayyiqtol and `assert` otherwise.
5. **Polarity is left `none`.** The English scheme's polarity is the speaker's evaluation ([context](../../docs/context.md)), not negation. Negation has its own column.

### Rules

Each rule can be left out (`--without`) or put in (`--with`); the defaults are `prag::RULES` minus `prag::OFF`.

| rule | what it does | default |
|---|---|---|
| `ccomp` | the ccomp of a verb of saying is the quote | on |
| `carry` | quotes go on into the next sentences | on |
| `lemor` | לֵאמֹר opens a quote after any verb | on |
| `close-qatal` | a qatal with no mark of speech closes a quote | on |
| `promise` | first-person yiqtol/weqatal in speech is a promise (commissive) | on |
| `first-form` | the form is the first clause's | on |
| `infabs` | an infinitive absolute heading a clause commands | on |
| `ask-first` | a question asks even with a second-person verb | on |
| `amar` | after a finite אמר, what follows its arguments is the quote, or the next sentence is | off |
| `inner-frame` | a frame inside open speech, or in the 1st/2nd person, is the character's | off |
| `propose` | a plural cohortative, or a jussive with נָא, suggests | off |
| `resolve` | a singular cohortative is the speaker's resolve (promise) | off |
| `rhetorical` | הֲלֹא asserts | off |
| `close-offline` | any unmarked clause that is not yiqtol/weqatal/volitive closes a quote | off |
| `law3` | a third-person yiqtol/weqatal in carried law prescribes | off |
| `neg-polarity` | a negation makes polarity «neg» | off |
| `evaluate` | words that judge (טוב, ברך … / רע, ארר, חטא …) give polarity, negation flips it | off |

### What `prag-compare` reports

`prag-compare` lines up the verses that are one Hebrew sentence with the English reading of the same verses, which a teacher gave JPS 1917 sentence by sentence in the parse database. It gives:

- **Raw agreement with the teacher**, field by field. The English values of a verse are taken as before: «request» if any of its sentences is, then «ask», else the first sentence's; voice «character» if any is; polarity «neg» if any is. This is agreement with a noisy teacher, not accuracy: the teacher is not consistent even on the plainest forms, such as «(And) thou shalt not …».
- **The constant answer.** For every field this is the share of the commonest English value, the floor a rule has to beat.
- **What both schemes can say.** Acts are grouped by Searle's classes: directive (request, suggest, warn), question (ask), commissive (promise, offer), expressive, and statement (assert, narrate and the rest). The teacher splits narration between `narrate` and `assert` almost at random, which no rule can follow. Polarity is also given with «pos» read as «none».
- **Voice of the sentence read alone.** Hebrew «character» only for speech the sentence shows itself: a frame with its quote, bare speech, or a carried sentence with a mark of speech. The teacher labels sentences one by one, and its «character» follows what a sentence shows: it misses most carried speech.
- **Speech against the quotes of JPS 1917**, an independent measure that needs no teacher. JPS marks direct speech with single quotes, and the tracker follows them from sentence to sentence (possessives are not quotes, and quotes are not nested). A verse is speech if any of it is inside a quote, and this is compared with the Hebrew voice. It holds for Genesis, Exodus and Numbers. JPS leaves the law of Leviticus and Moses' speech in Deuteronomy unquoted, so there a low precision means JPS gave no quotes, not that the Hebrew is wrong.
- **Negation against the English negation words** (not, no, nor, neither, never, nothing, none, nought, naught, cannot, nay). This compares a negative word anywhere in the Hebrew sentence with one anywhere in the English verse.
- **Each field by what the Hebrew verse holds of speech** (`-`, `quote`, `cont`, …). This shows where the disagreements live.

### Sources

These were read, from the Scientist's library, and this is what each gave.

- C. L. Miller-Naudé & J. A. Naudé (2015), *The Translation of Quotative Frames in the Hebrew Bible*, Folia Orientalia 52.
  - In narrative the frame comes before the quote.
  - וַיֹּאמֶר is the neutral frame.
  - In multi-verb frames the general verb comes last.
  - לֵאמֹר is a grammaticalized infinitive that always introduces the quote, even after verbs that are not of saying.
  - These gave rules `lemor`, `ccomp` and `amar`, and the multi-verb frame.
- J. A. Cook (2008), *The Vav-Prefixed Verb Forms in Elementary Hebrew Grammar*, Journal of Hebrew Scriptures 8.
  - Wayyiqtol is the past narrative form, and weqatal is modal.
  - לֹא + yiqtol and אַל + jussive are the negated modal forms.
  - These gave the narrator's wayyiqtol that closes a quote, and weqatal as a command.
- A. Andrason (2011), *Biblical Hebrew Wayyiqtol: A Dynamic Definition*, Journal of Hebrew Scriptures 11. Wayyiqtol is the foreground of narration (the discourse view it surveys), and it is never performative.
- E. T. Racher (2017), *An interpersonal sketch of the biblical Hebrew clause*, Functional Linguistics 4:9.
  - Volitives: אַל + jussive is specific, and לֹא + yiqtol is general.
  - The imperative is never negated.
  - נָא after an imperative is polite, and after a cohortative or jussive it proposes (after Christiansen 2009).
  - הִנֵּה draws attention.
  - Unmarked questions are doubtful (after Meister 1996), so the rules mark no question without a question word.
  - These gave the command rules, `propose`, the hedge, and the prohibition test.
- B. Isaksson (2024), *The Verb in Classical Hebrew: The Linguistic Reality behind the Consecutive Tenses*, Open Book Publishers.
  - Weqatal after an imperative goes on with the command.
  - וְהִנֵּה after a verb of seeing is a perception, not speech, so הִנֵּה is no mark of speech here.
  - A quote can open with כִּי, which is behind `amar`.
- Jones (2024), *Prayer and (Im)Politeness: Reading Psalm 7*, Old Testament Essays 37/3. נָא and the long imperative mark deference, which is the hedge.
- A. Andrason (2012), *Making It Sound: The Performative Qatal and Its Explanation*, Journal of Hebrew Scriptures 12. A first-person qatal in speech can perform the act («I hereby give»). There is no rule yet, because the English scheme has no class for declarations.
- Benigni, *The biblical Hebrew particle ky from a discourse analysis perspective*, Zeitschrift für Althebraistik. It reads כִּי as a shift of textual level. It does not support a «recitative» כִּי, so no rule was made from it.
- J. R. Searle (1976), *A classification of illocutionary acts*, Language in Society 5. Its classes are used for the act measure.

### Limits

- **The teacher is the only gold for act and form, and it is noisy.** Its narrate/assert split and its polarity cannot be followed by any rule.
- **JPS quotes are a gold for speech only where JPS uses them.** That is Genesis, Exodus and Numbers.
- **Nothing here is strictly held out.** The rules are decided on the Genesis speech measure and on act class; Exodus and Numbers serve as a check.
- **Speech carrying fails in some places.** It cannot see the end of a speech that ends without a narrator's wayyiqtol or qatal, nor a narrator's clause that opens with a subject and a participle, such as «וְרִבְקָה שֹׁמַעַת».
- **The frame and quote follow our trees.** A quote the trees do not hang as `ccomp`, and that has no לֵאמֹר, is missed; `amar`, which would catch these, is off.

## Torah lessons

```
macula lessons requests <trees-dir> <en-pool.tsv> <parashot.tsv> <work-dir>
macula lessons post <work-dir> <trees-dir> <en-pool.tsv> <school-dir>
```

The Torah course of the Judaism school: one lesson per weekly portion (parashah, 54), written by a teacher model from the portion's whole English text (JPS 1917, aliyot marked). `requests` writes one request per portion to `<work-dir>/batch-requests.jsonl`; the model is `MOVA_TEACHER_MODEL`, else `MACULA_MODEL`, else the shared `mova-teacher` default, at medium effort. The teacher also names three key verses and the tradition's reading of each. `post` reads `<work-dir>/batch-results.jsonl` and writes each lesson to `<school-dir>/torah/NNN-<parashah>.lesson`, with each key verse as a fact in layers taken from the texts, not from the teacher: the Hebrew verse (MACULA), word for word (its glosses), the JPS 1917 verse and the idea. Lessons that fail the checks are reported and not written. Marking the lessons for the pupils' dictionary is done by a closed tool.

## Limits

- **The relations are ours.** They are read off MACULA's groups by the rules above, not hand-annotated in UD; [Comparing with UD PTNK](#comparing-with-ud-ptnk) says how they are measured.
- **The head rules are general.** Many group rules in the Torah share them, so special constructions (oaths, infinitive absolutes, casus pendens) get the generic reading. The rules were made on the Torah and are not measured beyond it.
