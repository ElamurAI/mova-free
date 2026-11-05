# 11. Nouns

The English noun inflects for **number** (singular, plural) and has a **genitive** (*'s*) that is really a phrasal clitic. It has no grammatical gender and no other cases. This chapter gives the plural rules and every class of exception (§§1–4), countability (§5), the genitive (§6), gender (§7), proper nouns (§8), compound and multiword nouns (§9), and the noun-forming morphology that feeds the class (§10); §11 says how Mova tags and lemmatizes nouns.

Most of the facts below are also recorded in Mova's noun seeds (`en/seeds/morph/noun-*.md`, `en/seeds/nominal/*.md`), which give exact page references to Sweet, Whitney, Jespersen, Kruisinga, Mätzner, Poutsma and Curme; the main ones are repeated here.

## 1. Number: the regular plural

The plural adds *-s*, spelled *-es* after sibilant letters and after some *-o*, with *y → ie* after a consonant:

| singular | plural | spelling rule | pronunciation |
|---|---|---|---|
| *cat, book, cliff* | *cats, books, cliffs* | + *s* | /s/ |
| *dog, day, song* | *dogs, days, songs* | + *s* | /z/ |
| *bus, box, church, wish, quiz* | *buses, boxes, churches, wishes, quizzes* | + *es* (*quiz* doubles) | /ɪz/ |
| *horse, judge, rose* | *horses, judges, roses* | + *s* (the *e* is already there) | /ɪz/ |
| *city, baby* | *cities, babies* | *y → ies* | /z/ |
| *potato, hero* | *potatoes, heroes* | + *es* (closed set) | /z/ |
| *photo, piano* | *photos, pianos* | + *s* | /z/ |

The spelling rules are in [06-spelling-rules.md](06-spelling-rules.md) §4, the pronunciation rules in [04-connected-speech.md](04-connected-speech.md) §6.1. Sweet NEG I §1000 and §1021 and Whitney §§123–124 state both. The regular plural is fully productive: every new noun takes it (*emails, selfies, apps, blogs*).

## 2. Irregular plurals I: stem changes

### 2.1 Vowel change (mutation)

Seven native nouns form the plural by changing the vowel, a relic of Old English i-mutation ([04](04-connected-speech.md) §8):

| singular | plural |
|---|---|
| *man* | *men* |
| *woman* /ˈwʊmən/ | *women* /ˈwɪmɪn/ |
| *foot* | *feet* |
| *tooth* | *teeth* |
| *goose* | *geese* |
| *mouse* | *mice* |
| *louse* | *lice* |

Compounds follow: *firemen, Englishmen, dormice, titmice, chessmen*; but *mongoose → mongooses* (not a compound of *goose*), *German → Germans, Norman → Normans, talisman → talismans, human → humans* (not compounds of *man*) (Jespersen MEG VI 11.1; Sweet NEG I §§1002–1004). *Mouse* "computer device" has *mice* and, rarely, *mouses*. The list is closed: new words never mutate.

Example: *Owls, foxes, cats, weasels, snakes and hawks all hunt **mice**, and a family of barn owls can eat over a thousand **mice** and voles in a year.* (`school:biology:120-how-real-mice-survive.lesson:12`)

### 2.2 The *-en* plural

| singular | plural | notes |
|---|---|---|
| *ox* | *oxen* | the only plain *-en* plural (Jespersen MEG VI 20.2₁) |
| *child* | *children* | double plural: Old English *cildru* + *-en* |
| *brother* | *brethren* | only "members of a religious community"; ordinary plural *brothers* |
| *cow* | *kine* | archaic ([09](09-early-modern-and-archaic.md) §4) |

### 2.3 Voicing: *-f* → *-ves*

About fifteen old nouns in *-f/-fe* (with a long vowel or *l* before the *f*) voice the fricative in the plural: *wife → wives, life → lives, knife → knives, thief → thieves, leaf → leaves, loaf → loaves, half → halves, calf → calves, elf → elves, self → selves, shelf → shelves, wolf → wolves, sheaf → sheaves*. Other nouns in *-f* take plain *-s*: *roofs, chiefs, beliefs, proofs, cliffs, safes, chefs, giraffes*. Some vary: *scarfs/scarves, hoofs/hooves, dwarfs/dwarves, wharfs/wharves*; *staff* has *staffs* (personnel) and *staves* (rods; from which a new singular *stave*). The genitive does not voice: *the wife's*. In pronunciation only, some nouns in /θ/ and *house* voice too: *paths, mouths, oaths* /ðz/, *houses* /zɪz/ (Sweet NEG I §§999–1001; Whitney §124; Jespersen MEG VI 16.2–16.4; seed `en/seeds/morph/noun-plural-f-ves.md`).

Example: *Walk, never run, in the kitchen: **knives**, hot pans and slippery floors make running dangerous.* (`school:everyday:294-kitchen-freezer-plugs-and-balconies.lesson:16`)

Several *-ves* forms are also verb forms: *lives* (from *live*), *leaves* (from *leave*), *halves, shelves, calves* (verbs *halve, shelve, calve*). The part of speech decides the lemma ([§11](#11-how-mova-handles-nouns)).

## 3. Irregular plurals II: zero plurals

Some nouns have the same form in the singular and plural; the number shows only in agreement (*this sheep / these sheep, a deer / many deer*):

| group | nouns |
|---|---|
| animals, mostly game and fish | *sheep, deer, swine, moose, elk* (also *elks*), *bison, grouse, salmon, trout, cod, carp, pike, fish* (*fishes* for kinds), *aircraft, craft, spacecraft* |
| Latin *-s* nouns | *series, species, means, crossroads, headquarters, barracks, works* ("factory") |
| nationality words in *-ese, -ss* | *Chinese, Japanese, Portuguese, Swiss* |
| measures after a number | *two dozen eggs, three hundred people, five thousand, a two-year-old*, (in some varieties) *ten pound, six foot tall* |
| hunting usage | *to shoot lion, elephant* (zero plural for game) |

Sweet NEG I §§1004–1006 and Whitney §127 list them; UD tags the number by agreement (*these sheep*: NNS, `Number=Plur`; seed `en/seeds/morph/noun-plural-zero.md`).

Example: *Jesus fed five thousand people with a boy's five loaves and two **fish**, and twelve baskets were left over.* (`school:religion-ethics:016-the-miracles-of-jesus.lesson:9`): *fish* plural, *thousand* zero plural after a numeral, *loaves* with *-ves*.

## 4. Irregular plurals III: foreign plurals

Loans from Latin and Greek, and some from French, Italian and Hebrew, may keep their original plural. The trend is toward the regular *-s*, fastest in everyday words; in scientific usage the foreign plural holds:

| ending | singular | foreign plural | regular plural | notes |
|---|---|---|---|---|
| *-us → -i* | *stimulus, fungus, cactus, nucleus, radius, alumnus, syllabus* | *stimuli, fungi, cacti, nuclei, radii, alumni, syllabi* | *funguses, cactuses, syllabuses* | *virus → viruses* only; *bus, bonus, campus, circus* → *-es* |
| *-us → -era, -ora* | *genus, corpus, opus* | *genera, corpora, opera* | *corpuses* | |
| *-a → -ae* | *formula, antenna, larva, vertebra, alga, alumna* | *formulae, antennae, larvae, vertebrae, algae* | *formulas, antennas* (radio) | |
| *-um → -a* | *datum, medium, curriculum, memorandum, bacterium, stratum, criterion* (Gk *-on*) | *data, media, curricula, memoranda, bacteria, strata, criteria* | *mediums* (spiritualist), *curriculums, memorandums, stadiums, museums, forums* | *data* and *media* now often singular mass nouns |
| *-on → -a* | *phenomenon, criterion, automaton* | *phenomena, criteria, automata* | *automatons* | |
| *-is → -es* | *analysis, crisis, thesis, basis, diagnosis, hypothesis, oasis, parenthesis* | *analyses, crises, theses, bases, diagnoses* /-iːz/ | — | |
| *-ex, -ix → -ices* | *index, appendix, matrix, vertex, apex* | *indices, appendices, matrices, vertices* | *indexes* (books), *appendixes* (anatomy) | |
| *-eau → -eaux* (French) | *bureau, plateau, tableau, chateau* | *bureaux, plateaux* | *bureaus, plateaus* (AmE) | |
| *-o → -i* (Italian) | *libretto, tempo, virtuoso, graffito, paparazzo* | *libretti, tempi, virtuosi, graffiti, paparazzi* | *librettos, tempos* | *graffiti* usually mass singular |
| Hebrew *-im, -oth* | *cherub, seraph, kibbutz* | *cherubim, seraphim, kibbutzim* | *cherubs* (art) | |
| *-ma → -mata* (Greek) | *stigma, schema, dogma* | *stigmata, schemata* | *stigmas, schemas, dogmas* | |

Meaning sometimes splits the two plurals: *indexes* (in books) vs *indices* (in mathematics); *geniuses* (people) vs *genii* (spirits); *antennas* (radio) vs *antennae* (insects). Sweet NEG I §§1007–1015 and Whitney §126 record the 19th-century state; Jespersen MEG II 2.6 the tendency to *-s*.

Examples from the school lessons: ***bacteria**, viruses and **fungi*** (`school:medicine:030-germs-the-immune-system-and-hand-washing.lesson:5`); *Lots of common childhood spots and itches are caused by tiny living things: viruses, **bacteria**, a **fungus**, …* (`school:medicine:011-spots-and-itches-chickenpox-head-lice-and-friends.lesson:0`).

### 4.1 Plurals of compounds

The plural goes on the head, wherever it is ([18](18-compounding-conversion-minor.md) §2):

| type | examples |
|---|---|
| head last (most compounds) | *bedrooms, toothbrushes, girlfriends, grown-ups, forget-me-nots, go-betweens, handfuls* (also *handsful*) |
| head first, with a phrase after it | *sons-in-law, passers-by, hangers-on, commanders-in-chief, men-of-war, attorneys general* (also *attorney generals*), *courts martial* (also *court-martials*) |
| both parts plural | *men drivers, women doctors* (with *man, woman* as first element) |
| informal regularization | *mother-in-laws* (colloquial) |

Sweet NEG I §§1018–1019, Whitney §130, Jespersen MEG II 2.3 and VI 17.8 (group plurals); seed `en/seeds/morph/noun-plural-compounds.md`.

## 5. Countability

### 5.1 Count and mass nouns

English nouns divide into **count** nouns (with a plural, *a/an*, numerals, *many, few*: *a book, two books, many books*) and **mass** (non-count, uncountable) nouns (no plural, no *a/an*, with *much, little*: *water, much water, a little water*). Reynolds 2026 §1.2.2 treats countability as a property of noun *uses* more than of nouns.

| test | count | mass |
|---|---|---|
| plural | *books* | \**informations* |
| *a/an* | *a book* | \**an advice* |
| numeral | *three books* | \**three furnitures* |
| *many/few* vs *much/little* | *many books* | *much water* |
| bare singular | \**Book is useful* | *Water is useful* |
| unit word | — | *a piece of advice, a glass of water, an item of news* |

Strict mass nouns that are count nouns in many other languages (and so frequent learner errors: seed `en/seeds/errors/much-many-countability.md`): *information, advice, furniture, equipment, luggage, baggage, homework, news, feedback, research, evidence, knowledge, progress, traffic, weather, software, machinery, jewellery, accommodation* (BrE; AmE *accommodations*), *bread, money, music, work* ("labour"). West African and South Asian Englishes often count some of them (*furnitures, equipments*: [08](08-national-varieties.md) §10).

### 5.2 Dual nouns and conversion between the classes

Many nouns are count in one sense and mass in another:

| mass | count |
|---|---|
| *some cake* | *a cake, two cakes* |
| *glass* (material) | *a glass, glasses* |
| *paper* | *a paper* (newspaper, essay) |
| *chicken* (meat) | *a chicken* (animal) |
| *experience* | *an experience* (an event) |
| *coffee* | *two coffees* (cups) |
| *wine* | *French wines* (kinds) |
| *light* | *a light* (lamp) |
| *hair* (all of it) | *a hair* (one) |

The "universal grinder" (any count noun can be used as mass: *there was dog all over the road*) and the "universal packager" (any mass noun can be counted in context: *two beers*) make countability a matter of use. Seeds: `en/seeds/nominal/mass-nouns.md` (after Poutsma GLME III ch. XXV §§22–24 and Curme 1931 ch. XXVI).

### 5.3 Plural-only nouns (pluralia tantum)

| group | examples |
|---|---|
| paired objects | *scissors, trousers, jeans, pants, shorts, pyjamas, tights, glasses* (spectacles), *binoculars, pliers, tongs, tweezers, scales* (for weighing) |
| collections, results, surroundings | *clothes, goods, riches, belongings, earnings, savings, proceeds, outskirts, surroundings, whereabouts, premises, remains, thanks, congratulations, regards, arms* (weapons), *stairs, contents, minutes* (of a meeting), *customs* (at a border) |
| with zero-plural *-s* nouns | *headquarters, barracks, means, series* (§3) |

They take a plural verb (*my jeans are*) and are counted with *a pair of* (paired objects) or not at all. In UD the lemma is the plural form and the feature is `Number=Ptan` (*clothes*, *scissors*). EWT train uses `Number=Ptan` 206 times, most often for *regards* (39), *troops* (15), *politics, means* (9 each), *supplies* (7), *grounds, clothes* (6), *thanks, headquarters, goods, contents, the Philippines* (5) and decades like *1970s* (counted 2026-10-17). Example: *A see-saw, **scissors**, a bottle opener and a wheelbarrow all use levers.* (`school:world:052-simple-machines.lesson:13`). Seed `en/seeds/morph/noun-pluralia-tantum.md`.

### 5.4 Singular nouns in *-s*

*news*; sciences and activities in *-ics* (*mathematics, physics, linguistics, economics, politics, athletics, gymnastics*); diseases (*measles, mumps, diabetes, rabies*); games (*billiards, darts, dominoes, cards* in some uses); some place names (*the Netherlands* as a country, *the United States*). They agree in the singular: *the news is good; mathematics is hard*. *Politics, statistics, economics* are plural when they mean "opinions, figures, financial aspects": *his politics are conservative*. Example: *Wait for trusted **news** before believing …* (`school:everyday:284-facts-opinions-and-saying-no-to-dares.lesson:15`). Seed `en/seeds/morph/noun-singular-in-s.md`.

### 5.5 Collective nouns

| type | examples | agreement |
|---|---|---|
| singular form, group meaning | *family, team, government, committee, staff, army, class, crowd, audience, jury, band, company* | singular or plural verb; plural common in BrE (*the team are playing well*), singular in AmE (*the team is*) |
| plural meaning without *-s* | *people, police, cattle, clergy, vermin, livestock, poultry* | always plural: *the police are here; these cattle* |

*People* is also an ordinary count noun "nation" with a regular plural: *the peoples of Africa*. Hejná & Walkden 2022 §2.3.2 describe the British–American difference in verbal concord with collectives; seed `en/seeds/nominal/collective-nouns.md`.

## 6. The genitive

### 6.1 Form

| possessor | ending | written | example |
|---|---|---|---|
| singular | *'s* | /s, z, ɪz/ | *the boy's, the cat's, the boss's, James's / James'* |
| plural in *-s* | *'* (apostrophe only) | no sound added | *the boys', the cats', the Joneses'* |
| plural without *-s* | *'s* | | *the children's, men's, women's, people's, mice's* |
| names in a sibilant, classical and biblical | *'* (often) | | *Jesus', Moses', Socrates', Achilles'* |
| pronouns | no apostrophe | | *its, hers, ours, yours, theirs, whose*; but *one's, someone's, everybody's, somebody else's* |

Sweet NEG I §1022, Whitney §§133–135, 142. In speech the singular, the plural and the plural genitive of regular nouns sound the same (*boys, boy's, boys'*); only writing distinguishes them. Example: ***Snake charmers'** cobras do not dance to music* (`school:biology:322-snakes-to-keep-away-from-cobras-mambas-vipers-and-rattlesnakes.lesson:5`); *with your **parents'** permission* (`school:everyday:467-friends-through-games-and-clubs.lesson:38`).

### 6.2 The genitive is a clitic

*'s* attaches to the end of a whole noun phrase, not to the head noun:

- *the King of England's crown* (the king's, not England's)
- *my son-in-law's car*
- *somebody else's coat*
- *the man I saw yesterday's son* (colloquial)
- *John and Mary's house* (joint) vs *John's and Mary's houses* (separate)

This is why Penn Treebank and UD tokenize *'s* as a separate word (PTB `POS`, UD PART with relation `case` on the possessor: *The head of school 's speech*: `case(head, 's)`). Sweet NEG I §§1016–1017 and §443, Whitney §§137–138, Jespersen MEG VI 17.1–17.5; seed `en/seeds/morph/noun-genitive-group.md`. Hejná & Walkden 2022 §5.4.3 trace how the Old English case ending became this phrasal clitic, competing with *of*.

### 6.3 Uses of the genitive

| use | example | alternative |
|---|---|---|
| possession, relation | *John's car, my sister's husband* | *the car of John* (unnatural) |
| subjective | *the doctor's arrival* (the doctor arrived) | *the arrival of the doctor* |
| objective | *the prisoner's release* (someone released the prisoner) | *the release of the prisoner* |
| classifying (descriptive) | *a children's book, a women's college, a doctor's degree, a fool's errand* | — (stress on the first word, like a compound) |
| measure, time | *a day's work, two weeks' holiday, a moment's thought, a stone's throw* | *two weeks of holiday* |
| independent (head omitted) | *at the baker's, at my aunt's, St Paul's* (cathedral) | |
| double (*of* + genitive) | *a friend of my father's, that remark of Tom's* | *a friend of mine* (with pronouns) |

The choice between *'s* and *of* depends on the possessor: *'s* is usual with persons, animals, collective human nouns, places, time expressions, and short NPs; *of* with inanimates and long or heavy NPs (*the end of the story*, *the roof of the house*; but *the car's engine* is common). Seeds: `en/seeds/nominal/classifying-genitive.md`, `double-genitive.md`, `absolute-genitive.md` (after Poutsma GLME III ch. XXIV and Curme 1931 §10).

## 7. Gender

English nouns have no grammatical gender; adjectives and articles do not agree with them. Gender shows only in the choice of pronoun (*he, she, it*) and follows meaning:

- **Persons**: male *he*, female *she*; unknown or non-binary *they* (singular *they*, accepted in edited prose since the 2010s; old in speech: [12](12-pronouns-determiners-numerals.md) §2.3).
- **Animals**: *it*, or *he/she* when the sex is known or the animal is a pet.
- **Things**: *it*; traditionally *she* for ships, countries (as political entities) and machines treated with affection.
- **Gendered pairs** marked lexically or by suffix: *man/woman, king/queen, horse/mare*; *actor/actress, waiter/waitress, host/hostess, prince/princess, hero/heroine, widow/widower, bride/bridegroom*. The suffixes *-ess, -ette, -trix* are now avoided for professions (*actor, author, flight attendant, chair*) ([17](17-derivation-suffixes.md) §2.2).

## 8. Proper nouns

Proper nouns (names) refer to a unique individual: *Mary, London, the Thames, Google, Christmas*. They are capitalized ([07](07-punctuation-capitals-numbers.md) §4); most have no plural except in special uses (*the two Marys in my class, the Smiths* "the Smith family", *the Kennedys*); place names may have an article as part of the name (*the Netherlands, the Hague, the United States, the Alps, the Thames, the Sahara*). Names can become common nouns (*a sandwich, a hoover, a diesel, a Judas*; [18](18-compounding-conversion-minor.md) §7) and vice versa.

UD tags proper nouns PROPN (PTB NNP, NNPS), and multiword names are joined with `flat` or `compound` (`en/seeds/nominal/flat-names.md`). In EWT train NNP is 6.1% of all tokens.

## 9. Noun + noun sequences

Nouns modify nouns freely: *stone wall, kitchen table, bus stop, credit card fraud investigation unit*. The modifying noun is normally singular even when the meaning is plural (*a toothbrush, a shoe shop, a ten-mile walk, a five-year plan*; seed `en/seeds/nominal/adnominal-noun-singular.md`), except for pluralia tantum and some established plurals (*a clothes shop, a sales department, the arms race, a savings account, a sports car, a jobs fair*, more in BrE). UD marks the modifier with the relation `compound`. Whether such a sequence is a compound word or a phrase is discussed in [18-compounding-conversion-minor.md](18-compounding-conversion-minor.md) §2.1.

## 10. Notes on noun forms

### 10.1 Plural forms with special meanings

Some plurals have meanings the singular lacks: *arms* (weapons), *customs* (border control), *goods*, *manners* (behaviour), *quarters* (lodgings), *spirits* (mood; alcohol), *works* (factory; mechanism), *looks* (appearance), *glasses*, *colours* (flag), *letters* (literature), *pains* (effort).

### 10.2 Misdivision and back-formed singulars

The boundary between article and noun has moved in a few words: *a napron → an apron, a nadder → an adder, a numpire → an umpire*; the other way *an ewt → a newt, an eke-name → a nickname*. Old singulars in *-s* were reanalysed as plurals and lost their *-s*: *pease → pea, cherise → cherry, sherris → sherry*; and *riches* (French *richesse*), *alms* were reanalysed as plurals. Sweet NEG I §998 lists *alms, eaves, riches* as old singulars. See also back-formation in [18](18-compounding-conversion-minor.md) §4.

### 10.3 Nouns from other classes

Nouns are made from verbs (*arrival, movement, decision, building, a run*), adjectives (*happiness, truth, the rich*), and other nouns (*friendship, kingdom, booklet*) by suffixes and by conversion; [17-derivation-suffixes.md](17-derivation-suffixes.md) §2 and [18](18-compounding-conversion-minor.md) §3. Adjectives used as noun heads without *-s* (*the rich, the poor, the English*) remain adjectives in UD (`en/seeds/morph/adj-as-noun.md`).

## 11. How Mova handles nouns

- **Tags and features.** The tagger assigns NN, NNS, NNP, NNPS; UD `Number` follows the tag (Sing, Plur). Zero plurals get their number from agreement (*these sheep* → NNS), which the tagger learns from context; the seed `noun-plural-zero.md` and the rules in `en/seeds/nominal/dem-number.md`, `numeral-noun-number.md` check it.
- **Lemmas.** `Morph::lemmatize` (`en/src/morph.rs`) looks up the form in the lexicon: the most frequent UD analysis first (*children → child*, *criteria → criterion*; EWT lemmatizes *data* as a singular mass noun *data* NN and *media* as *media*, and since UD comes first Mova does too, although AGID would give *datum*, *medium*), then AGID paradigms (*knives → knife, geese → goose, cacti → cactus, bacteria → bacterium*), then reverse suffix rules for unknown words (*-ies → -y, -es, -s*). The unit test `dictionary` checks *mice → mouse* both ways (`lemmatize("mice", NNS)` and `inflect("mouse", NNS)`).
- **Verb–noun homographs.** *lives, leaves, halves* are told apart by the tag: NNS → *life, leaf, half*; VBZ → *live, leave, halve*; the seed `noun-plural-f-ves.md` has rules for both (`en.morph.ves-plural-lemma`, `en.morph.lives-noun-lemma`, `en.morph.lives-leaves-verb-lemma`), with *leaves* as a noun allowed both *leaf* and *leave* (warning only).
- **The genitive.** The tokenizer splits *'s* and a final *'* after *s* (*parents'*) as separate tokens tagged POS; *'s* is ambiguous with *is, has, us* and is resolved by the tagger ([13](13-verbs.md) §9).
- **Pluralia tantum.** Lemmatized as the plural, as UD requires (*scissors → scissors*, *clothes → clothes*), because the lexicon takes the UD analysis first; the `Number=Ptan` feature comes from Mova's FEATS tables, learned from EWT.
- **Not yet**: countability is not a lexical feature in Mova; the agreement and article checks that need it (*\*an advice, \*many furniture*) exist only as error seeds (`en/seeds/errors/much-many-countability.md`, `missing-article.md`), not in the generator.

## Sources for this chapter

- Sweet 1891, NEG I §§998–1022 (nouns: plural, genitive), §443. https://archive.org/details/newenglishgramma01sweeuoft
- Whitney 1877, *Essentials of English Grammar*, §§123–144. https://archive.org/details/essentialsofengl01whit
- Jespersen 1942, MEG VI: 11.1 (mutated plurals), 16.1–16.4 (*s*-ending), 17.1–17.8 (group genitive, group plurals), 20.2 (*-en* plural). https://archive.org/details/modernenglishgra06jesp ; MEG II (1914) ch. 2–5 on number. Library `books/j/je/jespersen-1914-modern-english-grammar-historical` (text-2).
- Poutsma, H. 1914–1929. *A Grammar of Late Modern English*, vol. III ch. XXIV–XXVI. https://archive.org/details/grammaroflatemod01poutuoft (library `books/p/po/poutsma-1904-grammar-late-modern-english`)
- Curme, George O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*, §§8, 10, ch. XXVI. Library `books/c/cu/curme-1931-grammar-english-language-vol`.
- Hejná & Walkden 2022, §2.3.2 (collective nouns), §5.4.3 (possessives *of* and *'s*). https://zenodo.org/records/6560337
- Reynolds 2026, §1.2.2 (countability). https://zenodo.org/records/20612113
- Universal Dependencies, English features: https://universaldependencies.org/en/feat/Number.html ; `case` and `nmod:poss`: https://universaldependencies.org/en/dep/
- Mova: `en/src/morph.rs`, seeds `en/seeds/morph/noun-*.md`, `en/seeds/nominal/*.md`, `en/seeds/errors/much-many-countability.md`; school sentences from `$MOVA_DATA/parsedb/current.conllu`.
