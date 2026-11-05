# 05. The writing system

English is written in the Latin alphabet of 26 letters, without diacritics in native words. Its spelling is alphabetic but **deep**: one sound has many spellings and one spelling many sounds, because the spelling kept the pronunciation of the 15th century while the sounds moved on, and because it often prefers to show a word's family and origin over its present sound. This chapter describes the letters, the main sound–spelling correspondences in both directions, silent letters, and why the system is the way it is. The rules for spelling inflected and derived forms are in [06-spelling-rules.md](06-spelling-rules.md); punctuation and capitals in [07](07-punctuation-capitals-numbers.md).

## 1. The alphabet

| letter | name (BrE) | name (AmE where different) | letter | name (BrE) | name (AmE) |
|---|---|---|---|---|---|
| A a | /eɪ/ | | N n | /en/ | |
| B b | /biː/ | | O o | /əʊ/ | /oʊ/ |
| C c | /siː/ | | P p | /piː/ | |
| D d | /diː/ | | Q q | /kjuː/ | |
| E e | /iː/ | | R r | /ɑː/ | /ɑr/ |
| F f | /ef/ | | S s | /es/ | |
| G g | /dʒiː/ | | T t | /tiː/ | |
| H h | /eɪtʃ/ (also /heɪtʃ/, usual in Ireland) | | U u | /juː/ | |
| I i | /aɪ/ | | V v | /viː/ | |
| J j | /dʒeɪ/ | | W w | /ˈdʌbəljuː/ | |
| K k | /keɪ/ | | X x | /eks/ | |
| L l | /el/ | | Y y | /waɪ/ | |
| M m | /em/ | | Z z | /zed/ | /ziː/ (*zee*) |

Letter names matter for reading initialisms (*an MP, a UN report*: the article goes by the name's first sound; [12](12-pronouns-determiners-numerals.md) §8.2) and for spelling aloud.

**Vowel letters** are ⟨a e i o u⟩, and ⟨y⟩ when it spells a vowel (*myth, happy, fly*); ⟨w⟩ spells part of a vowel in digraphs (*now, saw, new*). All others are consonant letters.

**Letters no longer used.** Old English had ⟨þ⟩ (thorn) and ⟨ð⟩ (eth) for /θ ð/, ⟨ƿ⟩ (wynn) for /w/, ⟨æ⟩ (ash), and later ⟨ȝ⟩ (yogh) for /j x/ (Reynolds 2026 §9.4.2.1). Thorn survives as the ⟨y⟩ of mock-old *Ye Olde Shoppe*, which is *the* /ðə/ written with a ⟨y⟩-shaped thorn, not the pronoun *ye* (Hejná & Walkden 2022 §5.2.2, "Ye Olde Shop"). Long s ⟨ſ⟩ was used non-finally until about 1800 and appears in scans of old books (*ſhall*: the 1611 Bible text quoted by Hejná & Walkden 2022 ch. 4 has *ſmall, neſt*); OCR may read it as ⟨f⟩.

**Letter frequency.** In the 34 013 sentences of Mova's school lessons (764 445 words; `$MOVA_DATA/parsedb/current.conllu`, `school:*` sentences, counted 2026-10-17), the letters rank: e 11.7%, a 9.1%, t 8.3%, s 7.6%, o 7.5%, n 7.3%, i 6.5%, r 6.0%, h 4.9%, l 4.5%, d 4.0%, u 2.8%, c 2.7%, m 2.5%, g 2.3%, y 2.0%, p 2.0%, f 2.0%, w 2.0%, b 1.7%, k 1.2%, v 0.8%, j 0.15%, x 0.15%, z 0.11%, q 0.08%.

## 2. Graphemes

A **grapheme** is a letter or letter group that spells one phoneme. Beside single letters English uses:

| kind | graphemes |
|---|---|
| consonant digraphs | ⟨sh⟩ /ʃ/, ⟨ch⟩ /tʃ, k, ʃ/, ⟨th⟩ /θ, ð/, ⟨ph⟩ /f/, ⟨gh⟩ /f, ɡ, —/, ⟨ng⟩ /ŋ/, ⟨ck⟩ /k/, ⟨wh⟩ /w, h/, ⟨wr⟩ /r/, ⟨kn⟩ /n/, ⟨gn⟩ /n/, ⟨qu⟩ /kw/, ⟨dg⟩ /dʒ/, doubled consonants ⟨pp tt ss ll⟩… |
| consonant trigraphs | ⟨tch⟩ /tʃ/, ⟨dge⟩ /dʒ/ |
| vowel digraphs | ⟨ee⟩ /iː/, ⟨ea⟩ /iː, e, eɪ/, ⟨oo⟩ /uː, ʊ, ʌ/, ⟨ai, ay⟩ /eɪ/, ⟨oa⟩ /əʊ/, ⟨ou⟩ /aʊ, uː, ʌ, əʊ/, ⟨ow⟩ /aʊ, əʊ/, ⟨oi, oy⟩ /ɔɪ/, ⟨au, aw⟩ /ɔː/, ⟨ew⟩ /juː, uː/, ⟨ie⟩ /iː, aɪ/, ⟨ei⟩ /eɪ, iː/, ⟨ue⟩ /uː, juː/, ⟨ui⟩ /uː, ɪ/ |
| vowel trigraphs | ⟨igh⟩ /aɪ/, ⟨eau⟩ /əʊ, juː/ (*beau, beauty*), ⟨ough⟩ (see §4.4) |
| split digraphs ("magic e") | ⟨a–e⟩ /eɪ/ (*name*), ⟨e–e⟩ /iː/ (*theme*), ⟨i–e⟩ /aɪ/ (*time*), ⟨o–e⟩ /əʊ/ (*home*), ⟨u–e⟩ /juː/ (*cute*) |
| r-controlled vowels | ⟨ar⟩ /ɑː/, ⟨or⟩ /ɔː/, ⟨er, ir, ur, ear, or (after w)⟩ /ɜː/, ⟨air, are⟩ /eə/, ⟨eer, ear, ere⟩ /ɪə/ |

Reynolds 2026 §9.3 notes that vowel spellings become far more regular when read together with the following consonants (the **rime**): ⟨each⟩ is /iːtʃ/ in *each, teach, reach, beach, peach, bleach*, ⟨ate⟩ is /eɪt/ in stressed syllables, though ⟨ea⟩ alone has at least four values (*teach, real, head, great*).

## 3. From sound to spelling

The most common spellings of each phoneme, most frequent first. Consonants are in [01-consonants.md](01-consonants.md) §1.1; vowels here.

| vowel | spellings | examples |
|---|---|---|
| KIT /ɪ/ | i, y, e, u, o, ui, ie | *sit, myth, pretty, busy, women, build, sieve* |
| DRESS /e/ | e, ea, a, ai, ie, u, eo | *bed, head, any, said, friend, bury, leopard* |
| TRAP /æ/ | a, ai | *cat, plaid* |
| LOT /ɒ/ | o, a (after w, qu), ou, ow | *hot, want, squad, cough, knowledge* |
| STRUT /ʌ/ | u, o, ou, oo, oe | *cup, son, love, young, blood, does* |
| FOOT /ʊ/ | oo, u, ou, o | *book, put, could, woman* |
| FLEECE /iː/ | ee, ea, e–e, ie, ei, i, ey, eo, ay | *see, sea, theme, field, receive, machine, key, people, quay* |
| FACE /eɪ/ | a–e, ai, ay, ei, ey, ea, eigh, aigh, ae | *name, rain, day, vein, they, great, eight, straight, sundae* |
| PRICE /aɪ/ | i–e, y, igh, ie, y–e, i, ei, uy, eye, ai | *time, fly, high, pie, type, find, height, buy, eye, aisle* |
| GOAT /əʊ/ | o–e, oa, ow, o, oe, ou, ough, ew, eau | *home, boat, show, go, toe, soul, though, sew, plateau* |
| GOOSE /uː/ | oo, u–e, ew, ue, o, ou, ui, oe, ough | *moon, rule, flew, blue, do, soup, fruit, shoe, through* |
| /juː/ | u–e, u, ew, ue, eu, iew, eau, ou | *cute, music, few, cue, feud, view, beauty, you* |
| MOUTH /aʊ/ | ou, ow, ough | *out, cow, bough* |
| CHOICE /ɔɪ/ | oi, oy | *coin, boy* |
| NURSE /ɜː/ | ur, ir, er, ear, or (after w), our, yr | *turn, bird, her, earth, word, journey, myrtle* |
| START/PALM /ɑː/ | ar, a, al, ear, au, er | *car, father, calm, heart, aunt, sergeant* |
| NORTH/THOUGHT /ɔː/ | or, aw, au, al, ough, augh, our, oar, oor, ar (after w) | *for, saw, cause, talk, thought, caught, four, board, door, war* |
| SQUARE /eə/ | are, air, ear, ere, eir, ayer | *care, hair, bear, there, their, prayer* |
| NEAR /ɪə/ | ear, eer, ere, ier, eir | *hear, deer, here, pier, weird* |
| schwa /ə/ | any vowel letter, and combinations | *about, taken, pencil, lemon, circus, famous, nation* |

## 4. From spelling to sound

### 4.1 The vowel letters: "short", "long" and "r" values

Each vowel letter has a **checked** value (the "short" vowel, in a closed syllable), a **free** value (the "long" vowel, the letter's name, in an open syllable or before consonant + silent *e*), and an **r-controlled** value:

| letter | checked ("short") | free ("long") | before r | reduced |
|---|---|---|---|---|
| a | /æ/ *mat* | /eɪ/ *mate, ma-ple* | /ɑː/ *car*, /eə/ *care* | /ə/ *about* |
| e | /e/ *pet* | /iː/ *Pete, me* | /ɜː/ *her*, /ɪə/ *here* | /ə, ɪ/ *taken, begin* |
| i, y | /ɪ/ *kit, gym* | /aɪ/ *kite, my* | /ɜː/ *bird*, /aɪə/ *fire* | /ɪ, ə/ *pencil* |
| o | /ɒ/ *hop* | /əʊ/ *hope, go* | /ɔː/ *for*, /ɔː/ *more* | /ə/ *lemon* |
| u | /ʌ/ *cut*, /ʊ/ *put* | /juː/ *cute*, /uː/ *rule* | /ɜː/ *turn*, /jʊə/ *cure* | /ə/ *circus* |

The silent ⟨e⟩ that marks the long value was pronounced as /ə/ in Middle English (*name* /ˈnaːmə/); Reynolds 2026 §9.4.2.2 calls it a fossil of the Great Vowel Shift, and [02-vowels.md](02-vowels.md) §7 gives the shift.

**Where the rule fails.** Common words with a short vowel before consonant + *e*: *have, give, live* (verb), *love, come, some, done, gone, one, none, were, there, where, are, move, prove, lose, whose*. Most of them end in ⟨ve⟩, because English avoids final ⟨v⟩ ([06](06-spelling-rules.md) §8).

### 4.2 The consonant letters with several values

| letter | values | rule of thumb | examples |
|---|---|---|---|
| c | /k/; /s/ | /s/ before ⟨e i y⟩ ("soft c") | *cat, cot, cut* / *cent, city, cycle* |
| g | /ɡ/; /dʒ/ | /dʒ/ often before ⟨e i y⟩ ("soft g"), but native words keep /ɡ/ | *go, gap* / *gem, giant, gym*; but *get, give, girl, gift, begin, tiger* |
| s | /s/; /z/; /ʃ/; /ʒ/ | /z/ often between vowels and in endings | *sit* / *rose, dogs, was* / *sure, sugar* / *vision, measure* |
| x | /ks/; /ɡz/; /z/ | /ɡz/ before a stressed vowel | *box, taxi* / *exam, exist* / *xylophone* |
| th | /θ/; /ð/; /t/ | /ð/ in function words and between vowels in native words | *thin, method* / *this, mother, bathe* / *Thomas, Thames, thyme* |
| ch | /tʃ/ (native, French loans before 1600); /k/ (Greek); /ʃ/ (later French) | origin | *chin, church* / *chorus, chemistry, school* / *machine, chef, chute* |
| gh | /f/; /ɡ/ (initial); silent | | *laugh, tough* / *ghost* / *night, though, daughter* |
| s, ss, t, c + i before a vowel | /ʃ/, /ʒ/ | palatalization ([04](04-connected-speech.md) §7) | *mission, nation, special, vision* |

### 4.3 Doubled consonants

A doubled consonant letter spells one sound; its job is to show that the preceding vowel is checked: *hopping* (from *hop*) vs *hoping* (from *hope*); *later* vs *latter*; *dinner* vs *diner*. This is the basis of the doubling rule in inflection ([06](06-spelling-rules.md) §2). Final ⟨ff ss ll zz⟩ after a short vowel in monosyllables are a separate convention (*staff, kiss, bell, buzz*), with exceptions in function words (*if, of, is, as, has, was, us, this, bus, gas, yes*). Doubling across a morpheme boundary is pronounced as one consonant, but some prefixed words keep a longer sound (*unnamed, misspell*); Ben Hedia 2019 (ch. 6–8) measured this gemination for *un-, in-, dis-* and *-ly* and found a gradient: strongest with *un-*, weaker with *in-* and *dis-*, none with *-ly* ([16-derivation-prefixes.md](16-derivation-prefixes.md) §1.2).

### 4.4 The notorious ⟨ough⟩

| value | words |
|---|---|
| /ʌf/ | *tough, rough, enough* |
| /ɒf/ (GA /ɔf/) | *cough, trough* |
| /əʊ/ | *though, dough* |
| /uː/ | *through* |
| /aʊ/ | *bough, plough* (AmE *plow*) |
| /ɔː/ | *thought, bought, ought, fought* |
| /ə/ | *thorough, borough* (GA /oʊ/) |
| /ɒk/ | *lough* (Irish; also /ɒx/) |
| /ʌp/ | *hiccough* (= *hiccup*) |

The ⟨gh⟩ once spelled /x/ (as in German *Nacht*); its loss lengthened the vowel before it or turned into /f/ in some words. Reynolds 2026 §9.4.1 tells the history of *knight*: when first spelled, it was pronounced /knixt/, so the spelling was then a good match.

## 5. Silent letters

| letter | where silent | examples |
|---|---|---|
| b | after m; before t | *lamb, climb, bomb, thumb, tomb, debt, doubt, subtle* |
| c | after s before e/i | *science, scene, muscle* |
| d | in some clusters | *Wednesday, handsome, handkerchief* |
| e | final, marking the vowel; in some suffixes | *name, love*; *-ed* in *walked* (no vowel heard) |
| g | before n, at the start or end | *gnome, gnaw, sign, foreign, design* |
| gh | after a vowel | *night, though, daughter, weigh* |
| h | initial in some French loans; after w, r, ex | *hour, honest, heir, honour; what, rhyme, exhaust* |
| k | before n at the start | *knife, know, knee, knight* |
| l | before k, m, f, d (after a, o, ou) | *walk, calm, half, could, would, salmon, folk* |
| n | after m at the end | *autumn, column, hymn, damn* |
| p | Greek *ps-, pn-, pt-*; *-mpt-* | *psychology, pneumonia, receipt, raspberry, cupboard* |
| s | in some French and folk-etymology words | *island, aisle, isle, debris, corps* |
| t | after s or f before -en, -le; in French loans | *listen, castle, often* (variable), *ballet, buffet* |
| u | after g, q | *guess, guitar, guard, tongue, plaque* |
| w | before r at the start; in some clusters | *write, wrong, two, sword, answer, who* |

The ⟨b⟩ of *debt, doubt* and the ⟨s⟩ of *island* were added by Renaissance scholars to echo Latin (*debitum, dubitare*) or a supposed Latin source (*insula*), though *island* comes from Old English *īgland* (Reynolds 2026 §9.4.2.3).

## 6. Homographs and homophones

**Homographs** (same spelling, different words) are a parsing problem, since Mova must pick one reading:

| spelling | readings |
|---|---|
| *lead* | /liːd/ verb "guide"; /led/ noun "metal" |
| *read* | /riːd/ present; /red/ past and participle |
| *live* | /lɪv/ verb; /laɪv/ adjective |
| *wind* | /wɪnd/ noun "air"; /waɪnd/ verb "turn" |
| *tear* | /tɪə/ "drop from the eye"; /teə/ "rip" |
| *close* | /kləʊs/ adjective "near"; /kləʊz/ verb "shut" |
| *row* | /rəʊ/ "line; paddle"; /raʊ/ "quarrel" |
| *bow* | /baʊ/ "bend; front of a ship"; /bəʊ/ "weapon; knot" |
| *wound* | /wuːnd/ "injury"; /waʊnd/ past of *wind* |
| *minute* | /ˈmɪnɪt/ "60 seconds"; /maɪˈnjuːt/ "tiny" |
| *object, record, present…* | noun–verb stress pairs ([03](03-syllable-stress-intonation.md) §4.2) |

**Homophones** (same sound, different spellings) are a writing problem: *their/there/they're, its/it's, your/you're, to/too/two, whose/who's, then/than* (in weak forms), *affect/effect, principal/principle, stationary/stationery, complement/compliment*. Confusions between them are frequent real-word errors in web text; Mova's error seeds (`en/seeds/errors/real-word-confusions.md`) describe how UD annotates them (the intended word's lemma, with `Typo=Yes` where the treebank marks it).

## 7. Why English spelling is the way it is

Reynolds 2026 §9.4.2 calls the English writing system "a palimpsest, with layers of history visible in its quirks". The layers, oldest first:

1. **Old English** adopted the Latin alphabet and added ⟨þ ð ƿ æ⟩ (§1).
2. **Norman scribes** (12th–14th c.) brought French conventions: ⟨qu⟩ for /kw/ (*cwen → queen*), ⟨sh⟩ for /ʃ/ (*scip → ship*), ⟨c⟩ for /s/ before front vowels (*cell*), ⟨ou⟩ for /uː/ (*hus → house*), ⟨gh⟩ for /x/ (*niht → night*), and, on the usual explanation, ⟨o⟩ for /u/ next to ⟨m n u w⟩, whose strokes (minims) were hard to read in a row (*son, love, come, wonder*). Reynolds 2026 §9.4.2.2 gives the ⟨qu⟩ and ⟨sh⟩ examples; Hejná & Walkden 2022 §5.2.2 describes Middle English ⟨ou⟩ = /uː/ (*mouth*, beside older *muð*) and thorn.
3. **Printing** (Caxton, from 1476) fixed spellings, often from the London and Kentish usage of the time; Dutch-speaking typesetters introduced ⟨gh⟩ in *ghost* (Hejná & Walkden 2022 §4.2.1.2; Reynolds 2026 §9.4.2.3).
4. **The Great Vowel Shift** (c. 1400–1700) changed the long vowels after their spelling had settled ([02](02-vowels.md) §7). Hejná & Walkden 2022 §4.2.1.2 cite Millward & Hayes's view that the 16th century was the last time a thorough spelling reform was possible.
5. **Renaissance etymological respellings** (§5): *debt, doubt, receipt, island, scissors, advice* (for older *avis*), *fault* (for *faute*).
6. **Dictionaries**: Cawdrey's *Table Alphabeticall* (1604) listed 2 543 hard words (Reynolds 2026 §9.4.2.4; the text is in `$MOVA_DATA/raw/en-cawdrey-table-alphabeticall`); Johnson's *Dictionary* (1755) set much of British spelling (`$MOVA_DATA/raw/en-johnson-1755`); Webster's *American Dictionary* (1828) set the American differences (*color, center, defense*) (`$MOVA_DATA/raw/en-webster-1828`). National spelling standards are in [06-spelling-rules.md](06-spelling-rules.md) §9.

The result is a spelling that is poor at showing sounds but good at keeping **morphemes** constant: *sign–signature*, *nation–national*, *electric–electricity*, *photograph–photography*, *-ed* in *walked, played, wanted* (three sounds, one spelling), *-s* in *cats, dogs, horses*. A reader who knows the language reads by morpheme; a learner who reads by letter is misled. For a text model this is an advantage: Mova's lemmatizer works on spellings, which are stable across accents and across sound alternations.

## 8. How Mova handles the writing system

- **Characters.** The tokenizer (`en/src/tok.rs`) is a table-driven automaton over character classes (letter, digit, apostrophe, hyphen, period, comma…); letters with diacritics count as letters. The typographic apostrophe ⟨'⟩ (U+2019) is treated as the typewriter apostrophe ⟨'⟩ by both the tokenizer and the lemmatizer (`Morph::lemmatize`: "’d" → *would*, "n’t" → *not*).
- **Case.** Lemmas are lowercase except for proper nouns and some symbols; a capitalized form at the start of a sentence lemmatizes to lowercase (*Went* → *go*), and the encoder records the case separately (`Case::Title`, `Case::Upper`, or the raw form when inflection does not reproduce it: `Morph::encode`). See [07](07-punctuation-capitals-numbers.md) §5.
- **Spelling to sound.** Not modelled ([01](01-consonants.md) §6).
- **Old letters.** The long s and thorn are not normalized; book texts in the school are taken from Project Gutenberg editions, which already use modern letters.

## Sources for this chapter

- Ben Hedia 2019, *Gemination and Degemination in English Affixation*, ch. 3, 6–8. https://zenodo.org/records/3232849
- Hejná & Walkden 2022, ch. 4 (§4.2.1.2 spelling and printing), §5.2.2 (Middle English graphical features). https://zenodo.org/records/6560337
- Reynolds 2026, §§9.3–9.4 (phonics; why English spelling looks this way). https://zenodo.org/records/20612113
- Cawdrey, Robert. 1604. *A Table Alphabeticall*; Johnson, Samuel. 1755. *A Dictionary of the English Language*; Webster, Noah. 1828. *An American Dictionary of the English Language*. All public domain; raw copies in `$MOVA_DATA/raw/` (`en-cawdrey-table-alphabeticall`, `en-johnson-1755`, `en-webster-1828`).
- Mova: `en/src/tok.rs`, `en/src/morph.rs`, `en/seeds/errors/real-word-confusions.md`; school sentences in `$MOVA_DATA/parsedb/current.conllu` (letter counts 2026-10-17).
