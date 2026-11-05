# 06. Spelling rules: inflection, derivation, compounds and national standards

When an ending is added to an English word, the spelling of the stem may change: a final consonant doubles (*stop → stopped*), a silent *e* drops (*hope → hoping*), *y* becomes *i* (*carry → carried*), *es* replaces *s* (*box → boxes*). These rules are regular and well described; they apply to inflection (*-s, -ed, -ing, -er, -est*) and to most vowel-initial derivational suffixes (*-able, -ance, -ish, -y*). This chapter states them with their exceptions (§§1–8), then the differences between the national spelling standards (§9) and the spelling of compounds (§10), and says what Mova's lemmatizer and inflector do with each rule (§11).

The rules below follow Jespersen MEG VI 4.2₃, 16.1, 21.9₃ and 22.8, Kruisinga I §§567–583 and Mätzner I pp. 224, 273, 337, as already collected in Mova's spelling seeds (`en/seeds/morph/spell-consonant-doubling.md`, `spell-silent-e.md`, `spell-y-to-i.md`, `noun-plural-s-spelling.md`), which give page references for each point.

## 1. The suffixes concerned

| suffix | begins with | spelling rules that apply |
|---|---|---|
| *-ed* (past, participle, adjective) | vowel | doubling, *e*-drop (*e* + *ed* → *ed*), *y → i* |
| *-ing* | vowel | doubling, *e*-drop, *ie → y* |
| *-er, -est* (comparison); *-er* (agent) | vowel | doubling, *e*-drop, *y → i* |
| *-s* (plural, 3rd sg) | consonant | *-es* after sibilants and some *-o*; *y → ie* |
| *-able, -ance, -ence, -ish, -y, -ous, -al, -ist, -ism, -ize* | vowel | doubling (with stress conditions), *e*-drop, *y → i* |
| *-ness, -less, -ful, -ly, -ment, -ship, -hood, -dom* | consonant | *y → i* (*happiness, merciless, beautiful, happily*); no *e*-drop (*hopeful, careless*) |

## 2. Doubling of a final consonant

### 2.1 The rule

A single final consonant letter is **doubled** before a vowel-initial suffix when it follows a single vowel letter that is **stressed**:

| base | form | why |
|---|---|---|
| *stop* | *stopped, stopping, stopper* | one vowel letter + one consonant, stressed |
| *big* | *bigger, biggest* | |
| *run* | *running, runner* | |
| *preˈfer* | *preferred, preferring* | final syllable stressed |
| *beˈgin* | *beginning, beginner* | |
| *ocˈcur* | *occurred, occurrence* | |
| *conˈtrol* | *controlled, controller* | |

The double letter keeps the vowel checked: *hopping* (/ɒ/) vs *hoping* (/əʊ/), *starred* vs *stared*, *planned* vs *planed* ([05-writing-system.md](05-writing-system.md) §4.3).

### 2.2 No doubling

- **Two vowel letters** before the consonant: *rain → rained, cool → cooler, beat → beating, need → needed*.
- **Two consonants** at the end: *help → helped, fast → faster, want → wanted*.
- **Unstressed final syllable**: *ˈvisit → visited, ˈoffer → offered, ˈopen → opened, deˈvelop → developed, ˈgallop → galloping, ˈbenefit → benefited* (*benefitted* also occurs); *ˈtarget → targeted, ˈbudget → budgeted, ˈfocus → focused* (BrE also *focussed*).
- **Stress moves off the syllable**: *preˈfer → preferred*, but *ˈpreference, ˈpreferable*; *conˈfer → conferring*, but *ˈconference*.
- **Final ⟨w x y⟩** never double: *snow → snowing, fix → fixed, play → player*.
- **Final ⟨h⟩** (in digraphs): *wish → wished*.

### 2.3 Exceptions and variation

| case | BrE | AmE | notes |
|---|---|---|---|
| final ⟨l⟩ after one vowel, unstressed | doubles: *travelled, travelling, traveller, cancelled, labelled, modelling, jewellery, crueller* | does not: *traveled, traveling, traveler, canceled, labeled, modeling, jewelry, crueler* | the main systematic difference; *controlled, compelled* (stressed) double in both. Kruisinga I §§572–574 records the British *-ll-*; Mova's lexicon knows *travelled* only from EWT (2 tokens as VBD) and *traveled* from both AGID and EWT (7 tokens). |
| *worship, kidnap, handicap, humbug* | *worshipped, kidnapped* | both forms, *kidnaped* rarer | Jespersen MEG VI 4.2₃ |
| *program, diagram* | *programmed, diagrammed* | same (also *programed*) | |
| *bias, focus, bus, gas* | *biassed/biased, focussed/focused, buses/busses, gases* | *biased, focused, buses, gases* | Kruisinga I §§581–583 |
| *combat* | *combated/combatted* | *combated* | |
| final ⟨c⟩ | adds ⟨k⟩ before *e, i, y*: *panic → panicked, panicking, panicky; picnic → picnicked; mimic → mimicked; traffic → trafficking; frolic → frolicked* | same | keeps /k/ ("hard c") before a front vowel letter; Kruisinga I §§581–583, Mätzner I p. 337 |
| *wool* | *woollen, woolly* | *woolen, wooly* | |
| *fulfil, enrol, instil, distil* | single *l* in the base, double before a vowel: *fulfilled* | base with *-ll*: *fulfill, enroll* | Webster spellings |

### 2.4 Doubling and the comparative

The same rule applies to *-er, -est*: *big → bigger, hot → hottest, sad → sadder, thin → thinnest*; not *cool → cooler, quick → quicker*. British *cruel → crueller* doubles by the *l*-rule.

## 3. Dropping the silent *e*

### 3.1 The rule

A final silent ⟨e⟩ drops before a vowel-initial suffix and stays before a consonant-initial one:

| base | vowel suffix | consonant suffix |
|---|---|---|
| *hope* | *hoping, hoped* | *hopeful, hopeless* |
| *make* | *making, maker* | |
| *love* | *loving, lovable* | *lovely* |
| *large* | *larger, largest* | *largely* |
| *use* | *using, usable* | *useful, useless* |
| *care* | *caring* | *careful* |
| *sure* | *surest* | *surely* |

With *-ed* and *-er/-est* the *e* merges with the suffix's *e*: *loved, freed, guaranteed, nicer, finest*.

### 3.2 Exceptions

- **⟨ce⟩ and ⟨ge⟩ keep the *e* before ⟨a, o⟩** to keep the "soft" sound: *noticeable, peaceable, changeable, manageable, courageous, outrageous, advantageous* (but *noticing, changing*).
- **⟨ee, ye, oe⟩ keep the *e* before *-ing***: *seeing, agreeing, dyeing* (colouring) vs *dying* (from *die*), *eyeing, hoeing, shoeing, canoeing, tiptoeing*.
- ***singeing*** and ***swingeing*** keep it to differ from *singing, swinging*; ***ageing*** (BrE) beside ***aging*** (AmE).
- **⟨ie⟩ before *-ing*** becomes ⟨y⟩: *lie → lying, die → dying, tie → tying, vie → vying*.
- **Consonant suffix, *e* dropped anyway**: *truly, duly, wholly, awful, ninth, argument, judgment* (AmE, legal BrE; *judgement* general BrE), *acknowledgment* (AmE; BrE *acknowledgement*), *abridgment*.
- ***-able* with or without *e***: *likeable/likable, liveable/livable, sizeable/sizable, saleable/salable, rateable/ratable*: BrE prefers the *e* (Jespersen MEG VI 22.6).
- **Mile + age**: *mileage* (also *milage*); ***acreage***; ***hoeing***.
- **Words in *-le*** drop *le* before *-ly*: *simple → simply, gentle → gently, able → ably, subtle → subtly* (Jespersen MEG VI 22.8).

## 4. Plural and third person: *-s* or *-es*

| stem ends in | ending | examples |
|---|---|---|
| ⟨s ss x z zz ch sh⟩ | *-es* | *bus → buses, kiss → kisses, box → boxes, quiz → quizzes, buzz → buzzes, church → churches, wish → wishes*; verbs *misses, fixes, watches, pushes* |
| silent ⟨e⟩ | *-s* | *horse → horses, judge → judges, rose → roses* (the *e* is already there) |
| consonant + ⟨y⟩ | *-ies* | *city → cities, baby → babies, cry → cries, carry → carries* |
| vowel + ⟨y⟩ | *-s* | *day → days, boy → boys, key → keys, valley → valleys, monkey → monkeys, play → plays* |
| consonant + ⟨o⟩ | *-es* in a set of old words; *-s* in recent, clipped and foreign words | *-oes*: *potatoes, tomatoes, heroes, echoes, vetoes, torpedoes, embargoes*; verbs *goes, does, echoes*. *-os*: *photos, pianos, radios, kilos, solos, avocados, logos, memos, zeros (zeroes), studios, videos*. Both: *mosquito(e)s, volcano(e)s, cargo(e)s, tornado(e)s, mango(e)s* |
| ⟨f, fe⟩ | *-s*, or *-ves* in a closed set | *wives, knives, leaves*: see [11-nouns.md](11-nouns.md) §2.3 |
| ⟨ch⟩ = /k/ | *-s* | *stomachs, monarchs, epochs* |
| proper names | *-s*, no *y → i* | *the Kennedys, two Marys, the Joneses* (sibilant: *-es*) |
| letters, digits, words cited | *'s* or *s* | *p's and q's, the 1990s* (also *1990's*), *dos and don'ts* |

Sweet NEG I §1021, Whitney §§123–124, 128 and Jespersen MEG VI 16.1₁–16.1₆ give this table; the pronunciation of these endings is in [04-connected-speech.md](04-connected-speech.md) §6.

## 5. *y* to *i*

A final ⟨y⟩ after a consonant becomes ⟨i⟩ before any suffix except one beginning with ⟨i⟩:

| base | changes | keeps *y* |
|---|---|---|
| *carry* | *carried, carrier, carries* | *carrying* |
| *happy* | *happier, happiest, happily, happiness* | |
| *beauty* | *beautiful, beautify* | |
| *study* | *studied, studious* | *studying* |
| *busy* | *business* (lexicalized), *busily* | *busyness* (the state of being busy) |
| *mercy* | *merciless, merciful* | |
| *baby* | *babies* | *babyish, babyhood* |
| *copy* | *copied, copier* | *copyist* |

After a vowel the ⟨y⟩ stays: *played, player, enjoyment, destroyed, greyish, coyly*. Old exceptions: *laid, paid, said* (and *staid*, now an adjective), *daily, gaily* (beside *gayly*). Adjectives in *-y* from monosyllables vary: *dry → drier/dryer, drily/dryly; shy → shier/shyer, shyly; sly → slyly; spry → spryer* (Jespersen MEG VI 22.8₂–22.8₆).

## 6. Other stem changes

| rule | examples |
|---|---|
| *-ic* + *-ly* → *-ically* | *basic → basically, tragic → tragically, automatic → automatically*; exception *publicly* |
| *-ble, -ple, -tle* + *-ly* | *possibly, simply, gently* (§3.2) |
| *-ll* + *-ly*, *-ful*, *-ness* | *full → fully, dull → dully, skill → skilful* (BrE) / *skillful* (AmE), *will → wilful* (BrE) / *willful* (AmE), *all → also, always, already, although, altogether* |
| *-our* + *-ous*, *-ize*, *-ation* | *humour → humorous, vigour → vigorous, glamour → glamorous, colour → coloration* (BrE drops the *u* here too) |
| *-ain* → *-an-* | *maintain → maintenance, sustain → sustenance, explain → explanation, exclaim → exclamation, proclaim → proclamation* |
| *-ounce* → *-unc-* | *pronounce → pronunciation, denounce → denunciation, renounce → renunciation* |
| *-eive* → *-ept-* | *receive → reception, deceive → deception, conceive → conception, perceive → perception* |
| *-ceed/-cede* → *-cess* | *proceed → process, succeed → success, concede → concession* |
| *-y* + *-ous* → *-eous* (a few words) | *beauty → beauteous, plenty → plenteous, bounty → bounteous* (literary) |

## 7. The *ie/ei* rule

"*I* before *e* except after *c*, when the sound is /iː/": *believe, field, piece, chief, niece* vs *receive, ceiling, deceit, conceive*. Exceptions are many: *seize, weird, either, neither, leisure* (with /iː/ in some accents), *protein, caffeine*; *species, science, ancient, efficient* (*c* + *ie* in other sounds); and with other sounds *ei* is normal: *eight, weigh, vein, their, height, foreign*. The rule is a mnemonic, not a rule of the system.

## 8. Letters English avoids

- **No final ⟨v⟩**: always ⟨ve⟩, which spoils the silent-*e* rule (*have, give, love*: [05](05-writing-system.md) §4.1). Exceptions are recent clippings and loans: *rev, spiv, Slav, shiv*.
- **No final ⟨j⟩**: /dʒ/ at the end is ⟨dge⟩ after a short vowel (*edge, bridge, judge*), ⟨ge⟩ otherwise (*page, huge, large*). Exceptions: *raj, hajj*.
- **No final ⟨u⟩** in native words: *you, thou* (with ⟨o⟩), *true, blue, due* (with ⟨e⟩). Exceptions: *menu, emu, gnu, guru, tofu, flu* (clipped).
- **⟨q⟩ is always followed by ⟨u⟩** in native and Latinate words; exceptions are transliterations (*Qatar, Iraq, qi, qanat*).
- **⟨ck⟩ after a short vowel, ⟨k⟩ after a long one or a consonant**: *back, neck, sick* vs *bake, leek, bank, milk*. **⟨tch⟩** vs **⟨ch⟩** the same way: *catch, fetch* vs *each, coach, bench* (exceptions *much, such, rich, which, attach*).
- **Short words in content classes have at least three letters**: *inn* vs *in*, *bee* vs *be*, *eye* vs *I*, *ewe* vs *you*, *add* vs *ad*, *egg*, *odd*, *err*, *ebb*. Function words can be shorter (*in, be, I, of, to, an, at*): a spelling distinction between content and function words.

## 9. National spelling standards

There is no single standard spelling; there are two main systems, British and American, plus national mixtures. The differences are small (a few hundred word families) but systematic. Webster's *American Dictionary* (1828) established most American forms; Sweet NEG I §1710 already notes that "in America the shorter -*or* is consistently extended" to words like *honor*, while British usage prefers *-our* in popular words. Horwill's *Dictionary of Modern American Usage* (1935) collects British–American differences of vocabulary and usage ([08](08-national-varieties.md) §§2–3, [19](19-lexicon-mwe-register.md) §5).

### 9.1 The pattern classes

| pattern | British | American | notes |
|---|---|---|---|
| *-our / -or* | *colour, honour, labour, neighbour, favour, behaviour, humour, harbour* | *color, honor, labor, neighbor, favor, behavior, humor, harbor* | BrE keeps *-or* in agent nouns (*actor, author, doctor*) and *-or-* before Latinate suffixes (*humorous, laborious*); *glamour* and *Saviour* also in AmE |
| *-re / -er* | *centre, theatre, metre, fibre, litre, sombre, spectre* | *center, theater, meter, fiber, liter, somber, specter* | after *c, g* the *-re* stays in both: *acre, massacre, ogre, mediocre*; *meter* "measuring device" in both |
| *-ise / -ize* | *realise, organise, recognise* (or *-ize*, "Oxford spelling") | *realize, organize, recognize* | words with *-ise* in both: *advise, advertise, arise, compromise, despise, devise, exercise, improvise, revise, supervise, surprise, televise*; BrE *-yse* / AmE *-yze*: *analyse/analyze, paralyse/paralyze* |
| *-ll- / -l-* | *travelled, cancelling, jeweller, woollen* | *traveled, canceling, jeweler, woolen* | §2.3; also BrE *skilful, wilful, enrol, fulfil, instalment* vs AmE *skillful, willful, enroll, fulfill, installment* |
| *-ogue / -og* | *catalogue, dialogue, analogue* | *catalog, dialog, analog* (both used) | *-ogue* stays in *rogue, vogue, league* |
| *ae, oe / e* | *anaemia, encyclopaedia, oestrogen, foetus, manoeuvre, paediatric* | *anemia, encyclopedia, estrogen, fetus, maneuver, pediatric* | *archaeology* in both; *aesthetic* usual in AmE too |
| *-ence / -ense* | *defence, offence, licence* (noun), *pretence* | *defense, offense, license, pretense* | BrE verb *license*, noun *licence*; also *practise* (verb) / *practice* (noun) in BrE, *practice* both in AmE |
| *-mme / -m* | *programme* (TV, plan), *kilogramme* (old) | *program* | BrE *program* for computer programs |
| individual words | *grey, tyre, kerb, cheque, gaol* (old), *plough, draught, mould, smoulder, pyjamas, aluminium, sceptic, storey, sulphur, cosy, moustache, doughnut, whisky* | *gray, tire, curb, check, jail, plow, draft, mold, smolder, pajamas, aluminum, skeptic, story, sulfur, cozy, mustache, donut* (informal), *whiskey* | *aluminium/aluminum* differ in sound too; *whisky* (Scotch) and *whiskey* (Irish, American) differ by origin, not by country of the writer |

### 9.2 How big each class is (VarCon)

The VarCon database (Kevin Atkinson and Benjamin Titze, version 2020.12.07; permissive licence; `$MOVA_DATA/raw/en-varcon`) records preferred American (A), British *-ise* (B), British *-ize* "Oxford" (Z), Canadian (C) and Australian (D) spellings. Among its 13 559 lines that pair an American and a British preferred spelling of the same word form (all inflected forms counted separately; counted 2026-10-17):

| class | lines | share |
|---|---|---|
| *-ize / -ise* (incl. derived forms) | 9 106 | 67% |
| *e / ae, oe* (mostly medical and scientific) | 1 820 | 13% |
| *-or / -our* | 952 | 7% |
| *-l- / -ll-* | 519 | 4% |
| *-er / -re* | 264 | 2% |
| *-yze / -yse* | 137 | 1% |
| *-ense / -ence* | 30 | 0.2% |
| *-og / -ogue* | 4 | — |
| other (*-ment/-ement, aging/ageing, airplane/aeroplane*…) | 727 | 5% |

Two-thirds of all differences are *-ise/-ize*, which the Oxford (Z) standard does not even have: an Oxford-style British text differs from an American one mostly in *-our*, *-re*, *-ll-* and the *ae/oe* words.

### 9.3 Other national standards

- **Canada** uses British *-our* and *-re* (*colour, centre*) with American *-ize*, *-yze* and often American single *l* (*traveled*) and *tire, curb, aluminum*: VarCon's C column.
- **Australia and New Zealand** follow British spelling with *-ise*, but Australia has *program* and the Australian Labor Party spells *Labor* without *u*.
- **Ireland, South Africa, India, Pakistan, Nigeria, Singapore, Hong Kong** follow British spelling, with growing American influence in software and media.
- **International organizations**: the UN and EU use British spelling with *-ize* in the UN, *-ise* in the EU English Style Guide (raw copy `$MOVA_DATA/raw/en-gram-ec-english-style-guide`).

## 10. Compounds: open, hyphenated, closed

A compound may be written as two words (**open**: *ice cream, high school*), with a hyphen (**hyphenated**: *mother-in-law, well-known*), or as one word (**closed**: *notebook, bedroom*). The choice is lexical and drifts toward closing as a compound becomes familiar (*e-mail → email, web site → website, on-line → online, to-day → today* (*to-day* was usual until the early 20th c.)). General guidance:

| type | usual form | examples |
|---|---|---|
| noun + noun, established | closed or open | *bedroom, toothbrush, football*; *bus stop, credit card, coffee table* |
| compound modifier before a noun | hyphenated | *a well-known writer* (but *the writer is well known*), *a long-term plan, a 10-year-old girl, an English-speaking country* |
| adverb in *-ly* + participle | open | *a highly paid job* (never \**highly-paid* in careful style) |
| phrasal-verb noun | closed or hyphenated | *a breakdown, a takeover, a check-in, a sit-in* (the verb is open: *break down*) |
| numbers 21–99, fractions | hyphenated | *twenty-one, two-thirds* |
| prefix + capital or number | hyphenated | *un-American, pre-1990, mid-July* |
| prefix with doubled vowel or ambiguity | hyphenated | *re-enter, co-operate* (BrE; AmE *cooperate*), *re-cover* "cover again" vs *recover* |

The US Government Publishing Office *Style Manual* (2016; public domain; `$MOVA_DATA/raw/en-gram-gpo-style-manual-2016`, https://www.govinfo.gov/app/details/GPO-STYLEMANUAL-2016) gives an extensive list of compound spellings and hyphenation rules; Hart's *Rules for Compositors* (1905; https://archive.org/details/rulesforcomposi00bradgoog) gives the older Oxford practice. Compounds as a word-formation process are in [18-compounding-conversion-minor.md](18-compounding-conversion-minor.md) §2.

## 11. How Mova handles spelling rules

Mova's morphology is in `en/src/morph.rs`. It has three layers, tried in order:

1. **UD counts**: the most frequent analysis of the form in UD EWT + ESLSpok train (`en/data/forms.tsv`, source `u`).
2. **AGID paradigms**: the full paradigm of each lemma from Kevin Atkinson's Automatically Generated Inflection Database (version 2016.01.19, permissive licence; sources `a`, `r`), which already contains every spelling change and exception (*stopped, hoping, carried, panicked, potatoes, travelled* and *traveled*…).
3. **Suffix rules** (`suffix` for inflection, `unsuffix` for lemmatization), used only when neither UD nor AGID knows the word.

The rule layer implements part of this chapter:

| rule | `suffix` (lemma → form) | `unsuffix` (form → lemma candidates) |
|---|---|---|
| *-es* after *s x z ch sh* (§4) | yes | yes (*-es*, then *-s*) |
| consonant + *y* → *-ies, -ied, -ier, -iest* (§5) | yes | yes |
| silent *e* + *-ed* → *-ed*; *e* dropped before *-ing* (except *-ee*) (§3) | yes | yes (tries stem and stem + *e*) |
| *-ie* → *-ying* (§3.2) | yes | yes |
| *-e* + *-er/-est* (§3) | yes | yes |
| consonant doubling (§2) | **no**: an unknown verb *blog* would give *bloged* | yes (undoubles: *stopped* → *stop*) |
| *-o* → *-oes* (§4) | no (*-os* always) | — (stripping *-es* covers it) |
| *-f* → *-ves* (§4) | no | no |
| *-c* → *-ck-* (§2.3) | no | no |

Since the lexicon contains about 315 000 form entries (242 166 distinct forms; `en/README.md`), the rule layer is reached only for unknown words, for which doubling is the most frequent gap. Candidates from `unsuffix` are ranked: a known lemma from which inflection reproduces the form exactly, then any known lemma, then the first candidate.

**Spelling variants are separate lemmas.** *colour* and *color*, *centre* and *center*, *judgement* and *judgment*, *programme* and *program*, *realise* and *realize* are distinct entries in `en/data/lemmas.tsv`, each with its own UD count (checked 2026-10-17: e.g. *color* NN 13 tokens, *colour* only from AGID; *programme* NN 5, *program* NN 41). Inflected British forms resolve to the shared lemma when they are in the lexicon: *traveled* comes from AGID's main form and from EWT, *travelled, travelling, cancelled, labelled* only from EWT (1–7 tokens each), because the lexicon keeps only AGID's main form of each cell and AGID's main forms are American ([14-irregular-verbs.md](14-irregular-verbs.md) §8). A British form that neither source has (e.g. an unseen *modelled*) goes to the reverse rules, which undouble the *ll* and find the right lemma (*model*) anyway. Mova does not yet map British and American spellings to a common lemma; VarCon (§9.2) would supply the table.

## Sources for this chapter

- Jespersen 1942, MEG VI: 4.2₃, 16.1₁–16.1₆, 21.9₃, 22.6, 22.8. https://archive.org/details/modernenglishgra06jesp
- Kruisinga 1925, *English Sounds* (with the spelling chapter), §§567–583. https://archive.org/details/handbookofpresen0000ekru
- Mätzner, Eduard. 1874. *An English Grammar*, transl. C. J. Grece, vol. I, pp. 224, 273, 337. Public domain. https://archive.org/details/englishgrammarme01maetuoft
- Sweet 1891, NEG I §1021 (plural spelling), §1710 (*-our/-or*). https://archive.org/details/newenglishgramma01sweeuoft
- Whitney, William Dwight. 1877. *Essentials of English Grammar*, §§123–128. Public domain. https://archive.org/details/essentialsofengl01whit
- Horwill, H. W. 1935. *A Dictionary of Modern American Usage.* Oxford: Clarendon. https://archive.org/details/dli.ernet.15683 (raw copy `en-gram-horwill-american-usage-1935`)
- U.S. Government Publishing Office. 2016. *Style Manual.* Public domain. https://www.govinfo.gov/app/details/GPO-STYLEMANUAL-2016
- Hart, Horace. 1905. *Rules for Compositors and Readers at the University Press, Oxford.* https://archive.org/details/rulesforcomposi00bradgoog
- Atkinson, Kevin. 2016. AGID (Automatically Generated Inflection Database) 2016.01.19. http://wordlist.aspell.net/other/
- Atkinson, Kevin & Benjamin Titze. 2020. VarCon 2020.12.07. http://wordlist.aspell.net/varcon-readme/ ; counts of 2026-10-17
- Mova: `en/src/morph.rs` (`suffix`, `unsuffix`, `Morph::lemmatize`, `Morph::inflect`), `en/data/forms.tsv`, `en/data/lemmas.tsv`, seeds `en/seeds/morph/spell-*.md`, `noun-plural-s-spelling.md`.
