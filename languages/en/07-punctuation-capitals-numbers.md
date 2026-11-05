# 07. Punctuation, capital letters, abbreviations and numbers

This chapter covers the conventions of the written page that are not letters: punctuation marks, the apostrophe, capitalization, abbreviations, and the writing of numbers, dates and symbols. These are conventions of editing, not of grammar, and the authorities differ; where British and American practice differ, both are given. The chapter ends with how Mova's tokenizer and lemmatizer treat each point (§8).

The main references are the US Government Publishing Office *Style Manual* (2016; chapters "Capitalization Rules", "Compounding Rules", "Punctuation", "Abbreviations and Letter Symbols", "Numerals"), Hart's *Rules for Compositors* (Oxford, 1905), the *Manual of Style* of the University of Chicago Press (1906), the European Commission's *English Style Guide*, and older handbooks of punctuation collected from Project Gutenberg (Hamilton, *Punctuation*; Allardyce, *"Stops", or How to Punctuate*; Klein, *Why We Punctuate*; Cocker, *Hand-book of Punctuation*). Full entries are at the end and in [sources.md](sources.md).

## 1. The marks and their tags

| mark | name (BrE / AmE) | main uses | PTB tag | UD |
|---|---|---|---|---|
| . | full stop / period | end of a statement; abbreviations (§4) | `.` | PUNCT |
| ? | question mark | end of a direct question | `.` | PUNCT |
| ! | exclamation mark / point | end of an exclamation | `.` | PUNCT |
| , | comma | separates items, clauses, parentheticals (§2.1) | `,` | PUNCT |
| ; | semicolon | joins independent clauses; separates complex list items | `:` | PUNCT |
| : | colon | introduces an explanation, list, quotation; times (*10:30*), ratios | `:` | PUNCT |
| ' ' " " | quotation marks / quotes, inverted commas | direct speech, quotation, titles of short works, scare quotes | ` `` ` and `''` | PUNCT |
| ( ) | brackets / parentheses | asides, references | `-LRB-`, `-RRB-` | PUNCT |
| [ ] | square brackets / brackets | insertions in quotations, editorial notes | `-LRB-`, `-RRB-` | PUNCT |
| - | hyphen | compounds, word division (§2.4; [06](06-spelling-rules.md) §10) | `HYPH` | PUNCT |
| – | en rule / en dash | ranges (*1990–2000*); BrE spaced dash | `:` or `,` | PUNCT |
| — | em rule / em dash | AmE unspaced dash for parenthesis or a break | `:` | PUNCT |
| … | ellipsis | omission, trailing off | `:` | PUNCT |
| / | slash, oblique / virgule | alternatives (*and/or*), fractions, *per* (*km/h*) | `,` or `SYM` | PUNCT or SYM |
| ' | apostrophe | genitive, contraction (§3) | part of a token (*'s, n't*) | — |
| & | ampersand | *and* in names (*AT&T*) | `CC` | CCONJ |
| emoticons, decorative marks | *:) :( *** ====* | in web text | `NFP` | PUNCT or SYM |

In UD English EWT + ESLSpok train (as counted in Mova's lexicon `en/data/forms.tsv`, 2026-10-17) the most frequent tags are `.` (10 317 tokens), `,` (8 035), `-RRB-` (1 007), `-LRB-` (973), `:` (867), ` `` ` (812), `''` (786), `HYPH` (660) and `NFP` (332; most frequent forms *-, *, :), --, |, :(*). Santorini 1990 defines the PTB punctuation tags; EWT added `HYPH`, `NFP` ("non-final punctuation") and `ADD` (web and e-mail addresses), which Mova's tag set also has (`en/build.rs`, `TAGS`).

## 2. Uses of the main marks

### 2.1 The comma

| use | example | note |
|---|---|---|
| between items in a list | *red, white and blue* / *red, white, and blue* | the comma before *and* is the **serial** or **Oxford comma**: usual in AmE and Oxford style, optional in general BrE; required when its absence would mislead |
| between independent clauses joined by *and, but, or, so, yet* | *It was late, but we stayed.* | often omitted when the clauses are short |
| after an introductory phrase or clause | *In 1990, he left. When she arrived, we ate.* | |
| around non-restrictive (non-defining) relative clauses and appositions | *My brother, who lives in Paris, is here. Paris, the capital, …* | a restrictive clause takes no commas: *the man who called* (the syntax chapters chapters 20–38) |
| around parenthetical words | *however, of course, for example* | |
| before a direct quotation | *She said, "Come in."* | AmE; BrE also *She said: 'Come in.'* |
| in dates, addresses | *October 17, 2026* (AmE); *Paris, France* | BrE *17 October 2026* takes no comma |
| in numbers | *1,000,000* | §6.2 |
| vocative | *Thank you, John. Hi, everyone.* | |

A **comma splice**, two independent clauses joined by a comma alone (*It was late, we stayed*), is an error in edited prose but common in web text.

### 2.2 Quotation marks

| | AmE | BrE |
|---|---|---|
| primary | "double" | 'single' (traditional book style) or "double" (newspapers) |
| nested | "She said 'no.'" | 'She said "no".' |
| period and comma | always inside: *"Yes," she said. He called it "nonsense."* | inside only if part of the quotation (**logical punctuation**): *He called it 'nonsense'.* |
| question and exclamation marks | inside if part of the quotation, outside otherwise | same |

Typographic quotes (" " ' ') and typewriter ones (" ') are both common; the right single quote is the same character as the typographic apostrophe (U+2019), which makes *'s* after a quotation ambiguous for a tokenizer.

### 2.3 Colon, semicolon, dashes

- The **colon** introduces what follows as an explanation or list: *She had one aim: to win.* AmE capitalizes a full sentence after a colon; BrE usually does not.
- The **semicolon** joins closely related independent clauses (*It rained; we stayed in.*) and separates list items that contain commas.
- **Dashes**: AmE uses an unspaced em dash (*It was—I think—Tuesday*); BrE a spaced en dash (*It was – I think – Tuesday*). An unspaced en dash marks ranges and relations: *pages 10–20, the London–Paris train, a Labour–Liberal coalition*. Web text uses *-* or *--* for all of these.

### 2.4 The hyphen

The hyphen joins compound words and compound modifiers ([06-spelling-rules.md](06-spelling-rules.md) §10) and marks a word broken at the end of a line. Soft hyphens from line breaks in scanned books (*know-|ledge*) must be removed before tokenizing; end-of-line hyphens that belong to compounds must be kept: a source of noise in OCR text.

### 2.5 Sentence-final marks

A sentence ends with *.*, *?* or *!*, combined in informal text (*?!*, *!!!*). An abbreviation's period at the end of a sentence is not doubled: *He works for Acme Inc.* A sentence ending in a quotation follows §2.2.

## 3. The apostrophe

| use | examples | notes |
|---|---|---|
| genitive singular | *the cat's tail, James's car / James' car* | [11-nouns.md](11-nouns.md) §6 |
| genitive plural in *-s* | *the cats' tails, the Joneses' house* | apostrophe after the plural *s* |
| genitive of irregular plurals | *the children's room, men's shoes* | |
| group genitive | *the King of England's daughter* | attaches to the phrase |
| contraction | *don't, it's, I'm, they've, we'll, she'd, o'clock, ma'am* | [13-verbs.md](13-verbs.md) §9 |
| omitted letters or figures | *rock 'n' roll, '90s, 'cause, goin'* | |
| plural of letters, sometimes of numbers and abbreviations | *mind your p's and q's; the 1990's* (now usually *1990s*); *CD's* (now usually *CDs*) | the "greengrocer's apostrophe" in ordinary plurals (*apple's 50p*) is an error |
| not in possessive pronouns | *its, hers, ours, yours, theirs, whose* | *it's* = *it is / it has*; confusing *its/it's* is one of the commonest errors |

Place names and organizations often drop the apostrophe: *St Andrews, Kings Cross* (station; but *King's College*), *Harrods, Starbucks, Barclays*.

## 4. Capital letters

### 4.1 Where capitals are required

| rule | examples |
|---|---|
| the first word of a sentence | *The dog barked.* |
| the pronoun *I* | *I think I can.* (but *me, my* lowercase) |
| proper nouns: people, places, organizations, brands | *Mary, London, the Thames, Google, the United Nations* |
| nationalities, languages, peoples and their adjectives | *English, French, Japanese, an American, Ukrainian food* |
| days, months, holidays | *Monday, October, Christmas, Ramadan* (seasons lowercase: *spring*) |
| titles before a name | *President Lincoln, Dr Smith, Queen Elizabeth* (but *the president said*) |
| religious names | *God, Allah, the Bible, the Torah, the Koran* |
| historical periods and events | *the Middle Ages, the Renaissance, World War II* |
| the first word of a direct quotation that is a sentence | *She said, "We're late."* |

### 4.2 Titles of works

**Title case** (AmE, most book and article titles): capitalize the first and last words and all words except articles, short prepositions and coordinators: *The Lord of the Rings, Gone with the Wind*. **Sentence case** (scientific references, many British publishers): only the first word and proper nouns: *The lord of the rings*. Headlines follow house style.

### 4.3 Capitals in British vs American usage

British usage tends to lowercase more (*the government, the prime minister* in running text; *the north of England*), American usage to capitalize titles and institutions. Both capitalize *North* as part of a name (*North America, the North* (US Civil War)).

### 4.4 All capitals and small capitals

Acronyms and initialisms are written in capitals (*NATO, BBC, USA*); some long-established acronyms become lowercase words (*laser, radar, scuba*) or title case (*Nato, Unesco* in British newspaper style). All-caps text in web writing signals shouting. Lowercase-only text (*i dont know*) is frequent in chat and search queries, and the lemmatizer must not rely on capitals to recognize names (§8).

## 5. Abbreviations

| kind | examples | punctuation |
|---|---|---|
| shortened words | *Mr, Mrs, Dr, St, Prof., etc., e.g., i.e., vs., approx., Jan., Mon.* | AmE: period after all (*Mr.*); BrE: no period when the abbreviation ends with the word's last letter (*Mr, Dr, St*) but a period otherwise (*Prof., etc.*) |
| initialisms (said letter by letter) | *BBC, USA, UK, CEO, FAQ, DIY* | usually no periods; AmE *U.S.* in some styles |
| acronyms (said as a word) | *NATO, NASA, UNESCO, AIDS, laser* | no periods ([18](18-compounding-conversion-minor.md) §6) |
| Latin abbreviations | *e.g.* (*exempli gratia*, "for example"), *i.e.* (*id est*, "that is"), *etc.* (*et cetera*), *et al.*, *cf.*, *viz.*, *a.m./p.m.*, *A.D./B.C.* | periods usual; AmE commas after *e.g.,* and *i.e.,* |
| units and symbols | *km, kg, m, s, °C, %, $, £, €* | never a period; no plural *s* (*5 km*, not \**5 kms*) |
| contractions of names | *Wm, Chas, Geo.* (old) | |

The GPO *Style Manual* chapter "Abbreviations and Letter Symbols" lists standard word abbreviations, Latin abbreviations and the symbols for units.

## 6. Numbers

### 6.1 Words or figures

Common style rules (they differ by house):

- Spell out numbers from one to nine (or to one hundred in literary style) and use figures for larger ones: *three apples, 12 apples, 250 people* (GPO style uses figures from 10).
- Spell out a number at the start of a sentence: *Twelve people came*, not \**12 people came*.
- Always figures for dates, times, measurements, money, percentages, page numbers: *5 km, $3.50, 7%, page 4, 3 p.m.*
- Round large numbers mix words and figures: *2 million, 1.5 billion*. *Billion* now means 10⁹ everywhere; the old British "long scale" (10¹²) is obsolete.

### 6.2 Decimal point and thousands separator

English uses a **decimal point** (*3.14*) and a **comma** (or thin space in scientific style) to group thousands (*1,000,000*). Four-digit numbers often go without the comma (*2500*), and years always do (*2026*). This is the opposite of most of continental Europe and a common source of error in translated or learner text.

### 6.3 Ordinals, fractions, ranges

- Ordinals in figures add the last letters of the word: *1st, 2nd, 3rd, 4th, 11th, 12th, 13th, 21st, 22nd, 101st*; old or legal *2d, 3d*. Spelled: *first, second, third, fourth* ([12](12-pronouns-determiners-numerals.md) §10).
- Fractions: *one half, two-thirds, three-quarters* (BrE) / *three-fourths* (AmE); in figures *½* or *1/2*.
- Ranges: *1990–2000* (en dash), *from 1990 to 2000*, not \**from 1990–2000*.

### 6.4 Dates and times

| | AmE | BrE and most of the world | ISO 8601 |
|---|---|---|---|
| written | *October 17, 2026* | *17 October 2026* | *2026-10-17* |
| numeric | *10/17/2026* | *17/10/2026* | *2026-10-17* |
| spoken | *October seventeenth* | *the seventeenth of October* | |
| time | *5:30 p.m.* | *5.30 pm* / *17:30* | *17:30* |

Numeric dates like *03/04/2026* are ambiguous between 3 April and 4 March; this manual (and Mova's docs) use ISO dates.

### 6.5 Money and measures

The currency symbol precedes the number with no space: *$5, £20, €3.50*; the cent and penny symbols follow (*50¢*, *50p*). Spoken *five dollars*, with the noun in the plural and after the number. Imperial and US customary units (*miles, feet, pounds, ounces, pints, gallons, °F*) are used in the US and partly in the UK (*miles, pints*); metric units elsewhere. The British and US gallon and pint differ in size.

## 7. Other conventions of the page

- **Italics** for titles of books and films, ship names, foreign words not yet naturalized, words cited as words, and emphasis; in plain text replaced by underscores or asterisks (*\*really\**), which EWT tags `NFP`.
- **Paragraphs**, **lists**, **headings**: outside sentence grammar; a list item may be a fragment, and EWT tags list numbers `LS` (117 tokens in train).
- **E-mail and web addresses**: tagged `ADD` in EWT (249 tokens; e.g. an e-mail address from the Enron e-mail section of the corpus).

## 8. How Mova handles punctuation, capitals and numbers

- **Tokenization** (`en/src/tok.rs`): a table-driven automaton over character classes (letter, digit, apostrophe, hyphen, period, comma, space…). On top of it, dictionaries learned from UD train decide (a) which words with a final period are **abbreviations** kept whole (*Mr., U.S.*) and which periods end a sentence; (b) which **hyphenated** words UD keeps as one token (*e-mail*) and which it splits; (c) which **multi-character punctuation** tokens exist (*..., :), :D*; repeated *!!!* and *??* are merged); (d) which **numbers with letter suffixes** stay whole (*1990s, 2nd*) and which split (*221bn → 221 · bn*); (e) which words may start with an apostrophe (*'s, 'em, '90s*) and where a leading apostrophe is a quotation mark (*'yuk*). The clitics *n't 's 're 've 'll 'd 'm* are split off by rule ([13](13-verbs.md) §9). Each token carries a "space after" bit, so the text can be rebuilt exactly.
- **Typographic apostrophe** ⟨'⟩ is normalized to ⟨'⟩ for clitic splitting and lemmatization.
- **Case and lemmas** (`en/src/morph.rs`, `Morph::lemmatize`): lemmas are lowercased, except that the original case is kept for proper nouns (`NNP`), addresses (`ADD`), symbols (`SYM`), list markers (`LS`), foreign words (`FW`) and numbers with digits, as EWT annotates (*et al* keeps lemma *al*, *01-Feb-02* is not lowercased, sentence-initial *One* → *one*). An all-lowercase form takes its case from the dictionary (*dallas → Dallas*), which suits ATIS-style lowercase queries.
- **Case in the encoding** (`Morph::encode`): a form is stored as lemma + tag + case (`AsIs`, `Title`, `Upper`), or raw when inflection cannot reproduce it.
- **Generation**: the word-order generator capitalizes the first word of a sentence (`en/README.md` notes the effect on chrF).
- **Not yet handled**: quotation-mark styles are not normalized; dates and numbers are tokens without a value in the parser (the `math` crate reads numbers in word problems); the serial comma and other style choices are not checked.

## Sources for this chapter

- U.S. Government Publishing Office. 2016. *Style Manual: An official guide to the form and style of Federal Government publications.* Public domain (US government work). https://www.govinfo.gov/app/details/GPO-STYLEMANUAL-2016
- Hart, Horace. 1905. *Rules for Compositors and Readers at the University Press, Oxford.* https://archive.org/details/rulesforcomposi00bradgoog
- University of Chicago Press. 1906. *Manual of Style.* Public domain. https://archive.org/details/manualofstylebei00univrich
- European Commission, Directorate-General for Translation. *English Style Guide.* (raw copy `en-gram-ec-english-style-guide`)
- Hamilton, Frederick W. *Punctuation* (PG #20787); Allardyce, Paul. *"Stops", or How to Punctuate* (PG #20938); Klein, William Livingston. *Why We Punctuate* (PG #47126); Cocker, William Johnson. *Hand-book of Punctuation* (PG #72479). Public domain. https://www.gutenberg.org/ebooks/20938 etc.
- Santorini 1990, *Part-of-Speech Tagging Guidelines for the Penn Treebank Project* (punctuation tags). https://repository.upenn.edu/cis_reports/570/
- UD English EWT 2.18 (Silveira et al. 2014, "A Gold Standard Dependency Corpus for English", LREC 2014), CC BY-SA 4.0. Card: [treebanks/INDEX.md#en-ewt](../../treebanks/INDEX.md#en-ewt); https://universaldependencies.org/treebanks/en_ewt/index.html
- Mova: `en/src/tok.rs`, `en/src/morph.rs`, `en/build.rs`; counts from `en/data/forms.tsv` (2026-10-17).
