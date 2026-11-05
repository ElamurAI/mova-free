# 19. The lexicon: vocabulary, multiword expressions and register

This chapter describes the English vocabulary as a whole: its size and frequency profile (§1), its layers by origin and how they pattern with style (§2), borrowing (§3), **multiword expressions**, the units of the lexicon that are longer than a word (§§4–5), **register**, the marking of words for formality, field, region and attitude (§6), and the main meaning relations (§7). §8 says what Mova's lexical resources hold. The use of register in discourse is treated in [37-register-and-variation.md](37-register-and-variation.md); idioms and figurative meaning in [34-irony-and-figurative-language.md](34-irony-and-figurative-language.md).

## 1. Size and frequency

### 1.1 How many words

There is no fixed number. Large dictionaries list several hundred thousand headwords; the inflection database AGID, from which Mova's lexicon is generated, has 112 505 lines (one per word and part of speech), and Mova's lemma heap `en/data/lemmas.tsv` has 110 308 lemmas, of which 61 477 are flagged as known (from UD or AGID without doubt marks) (`en/README.md`; counted 2026-10-17). The number of words an adult native speaker knows is usually estimated in the tens of thousands of word families; Reynolds 2026 §8.3.2 discusses such estimates and the coverage a learner needs.

### 1.2 Frequency: few words do most of the work

Word frequencies follow a steep, Zipf-like curve: a few words are extremely frequent, most are rare. In UD English EWT train (179 836 word tokens without punctuation, symbols and unclassified tokens; 12 911 distinct lemmas; counted 2026-10-17):

| most frequent lemmas | share of all tokens |
|---|---|
| top 10 (*the, be, to, and, a, I, of, in, have, that*) | 25.9% |
| top 50 | 45.1% |
| top 100 | 53.1% |
| top 1 000 | 79.0% |
| top 2 000 | 86.5% |
| top 3 000 | 90.3% |
| top 5 000 | 94.4% |

5 798 lemmas (45% of all distinct lemmas) occur only once. The frequent words are almost all function words and native verbs; Reynolds 2026 §8.3.1 describes the same distribution and §8.3.2 relates vocabulary size to text coverage, citing Hu & Nation (2000) for the estimate that readers need to know about 98% of the words in a text for comfortable comprehension. The New General Service List (Browne, Culligan & Phillips 2013; CC BY-SA, `$MOVA_DATA/raw/en-ngsl`) selects about 2 800 headwords for maximal coverage of general English. Mova's build compiles the 5 000 most frequent lemmas as integer constants (`en/build.rs`), which by the table above cover about 94% of EWT tokens.

### 1.3 Specialized vocabulary

Beyond the general core, each field has its own vocabulary (academic, legal, medical, technical); Reynolds 2026 §8.3.3 treats specialized vocabulary as a separate layer for teaching. In Mova, field vocabulary appears as the register lexicons (§6.3) and the course vocabularies of the school (`$MOVA_DATA/school/courses/`).

## 2. Layers of the vocabulary

### 2.1 Native, French, Latin, Greek

The strata of [00-introduction.md](00-introduction.md) §5 are not only history; they pattern with **register**. Native words are the everyday, concrete, emotional layer; French words are the layer of polite society, law and cooking; Latin and Greek the learned and technical layer. Triplets show the three levels side by side:

| native | French | Latin / Greek |
|---|---|---|
| *ask* | *question* | *interrogate* |
| *kingly* | *royal* | *regal* |
| *rise* | *mount* | *ascend* |
| *fear* | *terror* | *trepidation* |
| *holy* | *sacred* | *consecrated* |
| *time* | *age* | *epoch* |
| *fast* | *firm* | *secure* |
| *belly* | *stomach* | *abdomen* |
| *help* | *aid* | *assistance* |
| *end* | *finish* | *conclusion* |
| *buy* | *purchase* | — |
| *get* | *obtain, receive* | *acquire* |

Animal names are native, the meat names French, a classic of English historical lexicology: *cow/ox – beef, calf – veal, swine/pig – pork, sheep – mutton, deer – venison, hen – poultry*. Sweet NEG I §§610, 622 (French influence in Old and Middle English) and Jespersen's *Growth and Structure* (chapters on French and Latin) describe the layering; Hejná & Walkden 2022 §5.5.1 discuss French borrowing in Middle English and §4.5.3 the "inkhorn controversy" over Latin and Greek loans in the 16th century.

### 2.2 Doublets

The same Latin word borrowed twice, once early through French and once later directly, gives **doublets** with different forms and meanings: *loyal / legal, royal / regal, sure / secure, poor / pauper, frail / fragile, chance / cadence, blame / blaspheme, count / compute, sever / separate, ray / radius, dainty / dignity*. Native and Scandinavian doublets also exist: *shirt / skirt, shatter / scatter, no / nay, from / fro, rear / raise, ditch / dike*.

## 3. Borrowing

English borrows freely and assimilates loans in sound, spelling and inflection:

| degree of assimilation | examples | signs |
|---|---|---|
| fully native | *street* (Latin), *beef* (French), *sky* (Norse), *chocolate* (Nahuatl via Spanish) | English stress, spelling and plural |
| partly | *café, ballet, garage, genre, résumé* | French stress or sounds kept (/ʒ/), accents in careful writing |
| foreign plural kept | *criteria, phenomena, cherubim* | [11](11-nouns.md) §4 |
| unassimilated (italicized in edited prose) | *Schadenfreude, zeitgeist, joie de vivre, sotto voce, sukiyaki* | foreign spelling and pronunciation; PTB tag FW when treated as foreign |

Kinds of borrowing:

- **loanwords** (the form and meaning): *algebra, yoghurt, tsunami, emoji*;
- **loan translations** (calques: the meaning translated morpheme by morpheme): *superman* (German *Übermensch*), *loanword* (*Lehnwort*), *flea market* (*marché aux puces*), *it goes without saying* (*cela va sans dire*);
- **semantic loans** (a native word takes a foreign meaning): *dream* "vision in sleep" (from Norse; Old English *drēam* meant "joy").

Sources over time: Latin (all periods), Scandinavian (800–1100), French (1066–1500 and later), Latin and Greek (1500–1700), Dutch and Low German (shipping and trade: *yacht, skipper, cookie, boss*), Italian (music, art: *piano, opera, balcony*), Spanish (*canyon, ranch, mosquito*), languages of the empire and of migration (*bungalow, shampoo* from Hindi; *kiosk* from Turkish via French; *safari* from Swahili; *boomerang* from Dharug; *ketchup* via Malay), Yiddish (*schmooze, glitch, klutz*), Japanese (*karaoke, emoji, sushi*). Hejná & Walkden 2022 §2.5.1 ("Global vocabulary") and §4.5.2 ("Colonial borrowings").

## 4. Multiword expressions

A **multiword expression** (MWE) is a combination of words that is a unit of the lexicon: its meaning, its form, or its use is not fully predictable from its parts and the general rules. Reynolds 2026 §8.2.1 contrasts the semantic view (dictionaries list *give up, at the moment* as single entries) with the syntactic view (*give* and *up* are separate words: *I gave it up*), and §§8.4.2–8.4.3 treats collocations, idioms and formulaic language. Schulte im Walde & Smolka (eds.) 2020 collects interdisciplinary work on the role of constituents in MWEs.

### 4.1 Types

| type | examples | fixedness | in UD |
|---|---|---|---|
| **phrasal verbs** (verb + particle) | *give up, look up, turn down, pick up, find out, set off, put off, take over* | particle can move after a pronoun object: *give it up* | particle ADP, relation `compound:prt` |
| **prepositional verbs** | *look at, depend on, listen to, believe in, deal with* | preposition fixed, object after it | ADP `case` on the object |
| **phrasal-prepositional verbs** | *put up with, look forward to, get away with, run out of, come up with* | | `compound:prt` + `case` |
| **light-verb constructions** | *take a walk, make a decision, have a look, give a speech, pay attention* | the noun carries the meaning | ordinary `obj` |
| **verbal idioms** | *kick the bucket, spill the beans, break the ice, pull someone's leg* | often figurative, partly frozen (\**the bucket was kicked*) | ordinary syntax |
| **fixed expressions** | *by and large, of course, in order to, as well as, such as, because of, instead of, according to, rather than* | invariable | relation `fixed` on all but the first word |
| **complex prepositions and conjunctions** | *in front of, in spite of, on behalf of, as soon as, even though, so that* | invariable or nearly | `fixed` (some) or ordinary structure |
| **binomials** | *salt and pepper, black and white, ladies and gentlemen, bread and butter, odds and ends, null and void, safe and sound* | order fixed (\**pepper and salt* is marked) | ordinary `conj` |
| **compound nouns** | *bus stop, credit card, hot dog* | | `compound` ([18](18-compounding-conversion-minor.md) §2) |
| **names** | *New York, Barack Obama, the United Nations* | | `flat` (names), `compound` |
| **similes** | *as cold as ice, as busy as a bee, like a fish out of water* | | |
| **proverbs and sayings** | *the early bird catches the worm, too many cooks spoil the broth* | whole sentences | |
| **formulae and routines** | *how do you do, thank you, excuse me, you know, I mean, see you later, no worries* | pragmatic functions | `discourse`, `fixed` |
| **collocations** | *heavy rain, strong tea, make a mistake, do homework, highly likely, deeply sorry* | free in form, restricted in choice | ordinary |

### 4.2 Collocations

Collocations are habitual combinations whose parts keep their meanings but cannot be replaced by synonyms: *strong tea* (not \**powerful tea*), *heavy rain* (not \**weighty rain*), *make a mistake* (not \**do a mistake*), *do your homework* (not \**make your homework*), *commit a crime*, *pay a visit*, *bitterly cold*, *utterly ridiculous*. They are the main difficulty of advanced learners and a typical source of unidiomatic machine output. Reynolds 2026 §8.4.2 treats them as a separate layer of word knowledge.

### 4.3 How fixed are idioms?

Idioms differ in how much they allow:

| property | flexible example | frozen example |
|---|---|---|
| passive | *the beans were spilled* (*spill the beans* "reveal a secret") | \**the bucket was kicked* |
| modification | *pull some strings* (*pull strings* "use influence") | \**kick the big bucket* |
| inflection | *kicked the bucket, kicks the bucket* (the verb inflects) | *by and large* (nothing inflects) |
| pronoun slots | *pull someone's leg, lose one's temper* (with the possessive bound to the subject) | |
| literal reading | possible: *kick the bucket* (a pail) | none for *by and large* |

Transparent idioms (where parts map onto parts of the meaning: *spill* = reveal, *beans* = secret) are more flexible than opaque ones (*kick the bucket*). For a parser the consequence is that idioms must be matched on **lemmas** and with gaps, not as fixed strings.

### 4.4 Annotated resources

- **PARSEME** 1.1 English (Ramisch et al. 2018; CC BY-SA, internal and tests only; `$MOVA_DATA/raw/en-lex-parseme-1.1-en`): verbal MWEs annotated in 7 436 sentences (train 3 471, test 3 965): 832 VMWEs: 297 full phrasal verbs (VPC.full), 45 semi-compositional (VPC.semi), 244 light-verb constructions (LVC.full) and 43 causative ones (LVC.cause), 139 verbal idioms (VID), 60 inherently adpositional verbs (IAV), 4 multi-verb constructions (counted 2026-10-17).
- **STREUSLE** 5.0 (Schneider & Smith 2015; Schneider et al. 2018; MWE and supersense layers CC BY-SA 4.0; `$MOVA_DATA/raw/en-lex-streusle-5.0`): comprehensive strong and weak MWE annotation of the EWT reviews section, with supersenses for nouns, verbs and prepositions.
- Older dictionaries of phrases and idioms in the public domain: Dixon, *Dictionary of Idiomatic English Phrases* (1891); Kleiser, *Fifteen Thousand Useful Phrases* (1917); Smith, *Words and Idioms* (1925); Brewer's *Dictionary of Phrase and Fable*; proverb collections (Apperson, Bohn, Hazlitt) (raw copies `$MOVA_DATA/raw/en-lex-dixon-idioms-1891`, `en-lex-kleiser-phrases-1917`, `en-lex-smith-words-idioms-1925`, `en-brewer-phrase-fable`, `en-apperson-proverbs`…).

## 5. Phrasal verbs in more detail

Phrasal verbs are the most frequent and most productive MWE type of English, and among the hardest for learners.

### 5.1 The particles

The particles are the prepositions and adverbs of place: *up, down, in, out, on, off, over, away, back, about, around, through, along, by, forward*. Their meanings range from literal to aspectual to idiomatic:

| particle | literal | aspectual / completive | idiomatic |
|---|---|---|---|
| *up* | *stand up, look up* (raise one's eyes) | *eat up, use up, clean up, finish up* (completely) | *give up, make up (a story), put up with* |
| *out* | *go out, take out* | *sort out, work out, find out* (to the end, to discovery) | *pass out* (faint), *freak out* |
| *off* | *take off (a coat), get off (a bus)* | *finish off, kill off* | *put off* (postpone), *show off* |
| *on* | *put on (clothes)* | *go on, carry on, keep on* (continue) | *turn on* (attract) |
| *down* | *sit down, fall down* | *calm down, slow down, write down* | *turn down* (refuse), *let down* (disappoint) |
| *over* | *fall over* | *think over, talk over* (thoroughly) | *get over* (recover from) |
| *away* | *run away, give away* | *work away, chat away* (continuously) | *pass away* (die) |

### 5.2 Syntax and stress

- **Transitive separable** phrasal verbs allow the object on either side of the particle when it is a noun (*pick up the box / pick the box up*), but a pronoun object must come before the particle (*pick it up*, \**pick up it*).
- **Prepositional verbs** do not allow this (*look at it*, \**look it at*) and allow the preposition to front with *which/whom* (*the picture at which she looked*).
- **Stress**: the particle of a phrasal verb is stressed (*give ˈup*), the preposition of a prepositional verb is not (*ˈlook at*); Kruisinga I §264 notes that the particle takes the stronger stress at the end of a sentence ([03](03-syllable-stress-intonation.md) §6.4).
- **Register**: phrasal verbs are informal or neutral, their Latinate equivalents formal: *put up with – tolerate, find out – discover, put off – postpone, give up – abandon, come up with – devise, go up – increase, look into – investigate, turn down – reject* (§6.1).

Reynolds 2026 §4.3.5 analyses phrasal verbs as a verb with a preposition complement and discusses why they challenge learners. UD English tags the particle ADP with relation `compound:prt` (758 RP tokens, 745 `compound:prt` relations in EWT train; counted 2026-10-17).

## 6. Register

### 6.1 Formal and informal vocabulary

Many meanings have a formal and an informal word, the formal one often Latinate, the informal one native or phrasal:

| informal | neutral | formal |
|---|---|---|
| *kid* | *child* | *minor, offspring* |
| *get* | *receive, obtain* | *acquire* |
| *buy* | *buy* | *purchase* |
| *start* | *begin* | *commence* |
| *ask* | *ask* | *enquire, inquire* |
| *find out* | *discover* | *ascertain* |
| *need* | *need* | *require* |
| *help* | *help* | *assist* |
| *enough* | *enough* | *sufficient* |
| *about* | *about* | *approximately, concerning* |
| *show* | *show* | *demonstrate* |
| *tell* | *tell* | *inform* |
| *go up* | *rise, increase* | *escalate* |
| *let* | *allow* | *permit* |
| *before* | *before* | *prior to* |

Mova's pupil dictionary records these levels for the student: under *about* it notes that "in the exam voice «about» is replaced by stiff words: «concerning», «regarding», «in respect of», «pertaining to»; «approximately» replaces «about 10»" (`world/data/dictionary-improved.tsv`, entry *about*).

### 6.2 Kinds of register labels

Dictionaries mark words by:

| dimension | labels | examples |
|---|---|---|
| formality | formal, informal, colloquial, slang | *commence* (formal), *guy* (informal), *dope* "great" (slang) |
| attitude | derogatory, pejorative, offensive, approving, humorous, ironic, euphemistic | *geezer* (derogatory), *pass away* (euphemistic), *pre-loved* (euphemistic) |
| taboo | vulgar, taboo, offensive | swear words and slurs |
| time | archaic, dated, old-fashioned, obsolete, neologism | *thou, wireless, groovy, doomscrolling* |
| field | legal, medical, technical, nautical, computing… | *tort, myocardial, starboard, compile* |
| region | British, American, Australian, Scottish, Indian… | *lorry, faucet, arvo* ([08](08-national-varieties.md) §2.5) |
| medium | literary, poetic, spoken | *ere, steed, gonna* |
| child language | | *tummy, doggy, choo-choo* |

**Slang** is informal vocabulary of a group, often short-lived: *cool, awesome, sick, lit, sus, cap* "lie", *salty* "bitter", *ghost* "stop answering". Generational slang changes fast; older slang dictionaries in the public domain (Grose 1785, Hotten 1859, Farmer & Henley 1890–1904, Barrère & Leland) record earlier layers (raw copies `$MOVA_DATA/raw/en-grose-vulgar-tongue`, `en-hotten-slang`, `en-farmer-henley-slang`, `en-barrere-leland-slang`). **Euphemisms** replace taboo or painful words: *pass away, let go* (dismiss), *collateral damage, restroom, between jobs* (Mova's school has a course on them: `$MOVA_DATA/school/courses/euphemisms/`).

### 6.3 Register measured on corpora

Mova measures register lexically: `world register-stats` compares word frequencies in a register corpus with a reference corpus (log-odds with an informative Dirichlet prior, after Monroe et al. 2008) over public-domain Project Gutenberg books, and stores the words used far more in the register (`global/data/register-*.tsv`, derived counts, open). The top of each list (2026-10-17):

| register | corpus | top words |
|---|---|---|
| children's books | 1 200 books, 67.0 M tokens | *you, we, had, don, was, it, said, up, ll, captain, get, mr, think, little, could, do, mrs, just, yes, going* |
| legal and official | 113 books, 7.3 M tokens | *law, states, of, state, government, constitution, laws, or, court, united, courts, goods, be, legal, case, footnote, rights, any, property, treaty* |
| tales | 1 488 books, 102.5 M tokens | *king, thou, of, the, thy, thee, son, god, is, and, ye, his, gods, ii, earth, death, him, name, knight, hath* |

The lists show register as co-occurring features: children's books are dialogue-heavy (*you, we, said, don't, I'll*), legal texts nominal and institutional, tales archaic (*thou, thee, ye, hath*: [09](09-early-modern-and-archaic.md)). `global::register_z("legal")` uses the legal list as evidence for the formal domain ([docs/pragmatics.md](../../docs/pragmatics.md)). [37-register-and-variation.md](37-register-and-variation.md) treats register in syntax and discourse.

## 7. Meaning relations in brief

| relation | definition | examples |
|---|---|---|
| polysemy | one word, related senses | *head* (of the body, of a company, of a bed); *run* (hundreds of senses) |
| homonymy | one form, unrelated words | *bank* (river) / *bank* (money); *bat*; *bear* (animal / carry) |
| homophony, homography | same sound / same spelling | [05](05-writing-system.md) §6 |
| synonymy | same or near meaning, differing in register, collocation or connotation | *big/large, begin/commence, house/home* |
| antonymy | opposites: gradable (*hot/cold*), complementary (*dead/alive*), converse (*buy/sell, parent/child*) | negative prefixes ([16](16-derivation-prefixes.md) §2) |
| hyponymy | kind-of | *rose < flower < plant* |
| meronymy | part-of | *wheel < car, finger < hand* |
| semantic change | broadening, narrowing, amelioration, pejoration, metaphor | *bird* (OE "young bird" → all birds), *meat* (food → flesh), *nice* (foolish → pleasant), *silly* (blessed → foolish); Hejná & Walkden 2022 §3.5.1 (amelioration and pejoration) and §5.5.3 (*indeed*) |

The open WordNet resources (Princeton WordNet, Open English WordNet: `$MOVA_DATA/raw/en-princeton-wordnet`, `en-oewn`) encode these relations for nouns, verbs, adjectives and adverbs. Reynolds 2026 §8.4.1 discusses polysemy and monosemy as a matter of degree.

## 8. How Mova handles the lexicon

| resource | what it is | licence | where |
|---|---|---|---|
| the lemma heap and forms | 110 308 lemmas, 315 675 form entries (AGID + UD counts) | AGID permissive; EWT counts CC BY-SA | `en/data/lemmas.tsv`, `en/data/forms.tsv` |
| word constants | the 5 000 most frequent lemmas + listed domain words, as integers in code | ours | `en/build.rs`, `en/data/consts.txt` |
| the pupil dictionary | 19 720 entries: part of speech, meanings for a pupil, sense in the lessons, examples from the lessons, notes (traps, registers, maths reading) | ours | `world/data/dictionary-improved.tsv` |
| expressions and idioms | 122 922 entries from the English Wiktionary (8 732 idioms, 1 433 proverbs, 2 712 phrases, 60 205 verb phrases, 55 phrasal verbs, and words with register labels) | CC BY-SA, internal, not in the free release | `$MOVA_PETA_DATA/global/expressions.tsv` (private data outside the repository); matched by `global::expressions_in_sentence` |
| register lexicons | children, legal, tale (§6.3) | ours (derived from PD texts) | `global/data/register-*.tsv` |
| the lens | reads expressions, their meanings and register labels in a sentence and switches domains (slang, formal, humour, hidden meaning, `variety:<country>`) | ours | `prag::lens`, [docs/pragmatics.md](../../docs/pragmatics.md) |
| the school courses | a course on euphemisms, on English, on language, and regional-street courses | ours | `$MOVA_DATA/school/courses/` |

How expressions are matched: `global::expressions_in_sentence(forms, lemmas)` finds non-overlapping matches, longest first, on lemmas (*kicked the bucket* → *kick the bucket*) and on forms (*spill the beans*); *one's* in an entry matches any possessive and *someone* any one word ([docs/pragmatics.md](../../docs/pragmatics.md)). The parser itself analyses MWEs by UD conventions only: `compound:prt` for phrasal-verb particles, `fixed` for grammatical fixed expressions, `flat` for names, `compound` for compound nouns; it does not mark idioms or light-verb constructions. The measured effect of the expression lens as a feature of the pragmatics classifiers was within noise (documented in the same file), so the lens is used for "what was meant" and for domains, not for act classification.

**Not yet**: no collocation model; no lexical register feature in `en` (register is a sentence-level judgement made in `prag` and `world`); no link between formal and informal synonyms; PARSEME and STREUSLE are used only for measurement.

## Sources for this chapter

- Reynolds 2026, ch. 8 (§8.2.1 words, §8.3 frequency and coverage, §8.4 polysemy, collocations, idioms and formulaic language), §4.3.5 (phrasal verbs). https://zenodo.org/records/20612113
- Hejná & Walkden 2022, §2.5.1 (global vocabulary), §3.5.1 (amelioration and pejoration), §4.5.2–4.5.3 (colonial borrowings; inkhorn controversy), §5.5.1 (French borrowing), §5.5.3 (*indeed*). https://zenodo.org/records/6560337
- Sweet 1891, NEG I §§605–646 (influences and strata). https://archive.org/details/newenglishgramma01sweeuoft
- Jespersen 1912, *Growth and Structure of the English Language*. https://archive.org/details/growthstructureo1912jesp
- Kruisinga 1925, *English Sounds* §264 (stress of verb + adverb groups). https://archive.org/details/handbookofpresen0000ekru
- Schulte im Walde, Sabine & Eva Smolka (eds.). 2020. *The Role of Constituents in Multiword Expressions.* Language Science Press, CC BY 4.0. https://zenodo.org/records/3598577
- Ramisch, Carlos et al. 2018. "Edition 1.1 of the PARSEME Shared Task on Automatic Identification of Verbal Multiword Expressions." LAW-MWE-CxG 2018. https://aclanthology.org/W18-4925/ ; counts of 2026-10-17.
- Schneider, Nathan & Noah A. Smith. 2015. "A Corpus and Model Integrating Multiword Expressions and Supersenses." NAACL 2015. https://aclanthology.org/N15-1177/ ; STREUSLE https://github.com/nert-nlp/streusle
- Monroe, Burt L., Michael P. Colaresi & Kevin M. Quinn. 2008. "Fightin' Words: Lexical Feature Selection and Evaluation for Identifying the Content of Political Conflict." *Political Analysis* 16(4). (method of the register lexicons)
- Browne, Charles, Brent Culligan & Joseph Phillips. 2013. The New General Service List. CC BY-SA 4.0. http://www.newgeneralservicelist.org/
- Dixon, James Main. 1891. *Dictionary of Idiomatic English Phrases.* https://archive.org/details/dictionaryofidio00dixouoft ; Smith, Logan Pearsall. 1925. *Words and Idioms.* https://archive.org/details/rbanms.26629.wordsidioms0000loga_u8o6
- Mova: `en/data/`, `en/build.rs`, `world/data/dictionary-improved.tsv`, `global/data/register-*.tsv`, [docs/pragmatics.md](../../docs/pragmatics.md), [licence/DATA.md](../../licence/DATA.md); counts from UD English EWT 2.18 train (2026-10-17).
