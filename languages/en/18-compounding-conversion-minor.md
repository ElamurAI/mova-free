# 18. Compounding, conversion and the minor processes

Besides affixation ([16](16-derivation-prefixes.md), [17](17-derivation-suffixes.md)), English makes new words by **compounding** (*bedroom, blue-eyed*), **conversion** (*a walk, to email*), and a set of minor processes: **back-formation** (*edit* from *editor*), **clipping** (*phone, exam*), **blending** (*brunch, smog*), **acronyms and initialisms** (*NATO, BBC*), **reduplication** (*zigzag, chit-chat*), and words from names. Neoclassical compounds (*telephone, biology*) stand between compounding and affixation (§8). This chapter treats each, with how UD and Mova annotate the results.

The main sources are Jespersen MEG VI chapters VI–X (the "naked word", i.e. conversion; compounds; reduplication) and XXIX (shortenings), Sweet NEG I §§63–68, 900–915, 1536–1556 (composition), Kruisinga II.3 §§1904–1918, Schäfer 2018 on the meaning of noun compounds, and Mova's seeds `en/seeds/morph/wf-compounds.md`, `wf-conversion.md`, `wf-shortening.md`, `wf-reduplication.md`, which give page references.

## 1. Compounds: what counts as one

A compound is a combination of two (or more) stems that works as one word with its own meaning: *a blackbird* is a species, *a black bird* any bird that is black. Three criteria are used, none sufficient alone:

| criterion | compound | phrase |
|---|---|---|
| stress | usually fore-stress: *a ˈblackbird, a ˈgreenhouse* | end-stress: *a black ˈbird* ([03](03-syllable-stress-intonation.md) §6) |
| meaning | specialized, not fully predictable: *greenhouse* is not a green house | compositional |
| inseparability | no modification of the first part: \**a very blackbird*; no coordination inside: \**black- and bluebirds* (marginal) | *a very black bird* |
| inflection | inside elements uninflected: *a toothbrush, a five-pound note* | *a brush for teeth* |
| spelling | solid, hyphenated or open: unreliable | open |

Spelling is the least reliable: Jespersen MEG VI 8.1 calls English compound spelling "little short of chaotic", and the same compound is written *ice-cream, ice cream* and (rarely) *icecream* ([06](06-spelling-rules.md) §10). Sweet NEG I §§1551–1556 takes stress as "the main criterion". In UD, an open compound is two tokens joined by the relation `compound`; a solid or hyphenated one is a single token (seed `wf-compounds.md`). In UD EWT train the relation `compound` occurs 7 182 times, mostly with a NOUN (4 472) or PROPN (2 267) dependent (counted 2026-10-17).

## 2. Types of compounds

### 2.1 Compound nouns

| pattern | examples | notes |
|---|---|---|
| noun + noun | *bookshelf, sunrise, toothbrush, bus stop, credit card, birthday, football* | the most numerous and productive type; recursive: *credit card fraud investigation team* |
| adjective + noun | *blackbird, greenhouse, hotdog, software, high school, big toe* | |
| verb + noun | *pickpocket, breakwater, playground, swimming pool, washing machine* | the verb stem or *-ing* form |
| noun + verb | *sunshine, haircut, rainfall, bloodshed, nosebleed* | the noun is often the subject or object of the verbal idea |
| particle + noun | *outcry, bystander, underworld, afterthought, income* | |
| verb + particle (from phrasal verbs) | *a breakdown, a takeover, a check-in, a sit-in, a lookout, a drawback, a grown-up* | fore-stressed; §2.5 |
| noun + *'s* + noun (genitive compounds) | *bird's-eye view, lady's maid, bull's-eye, St John's wort* | classifying genitive ([11](11-nouns.md) §6.3) |
| phrase compounds | *forget-me-not, mother-in-law, jack-in-the-box, has-been, know-it-all, merry-go-round* | plural on the head ([11](11-nouns.md) §4.1) |

### 2.2 Head and kinds of relation

- **Endocentric** (determinative) compounds name a kind of the head: *a toothbrush* is a brush, *a bookshelf* a shelf. The head is on the right; the left element modifies it.
- **Exocentric** (*bahuvrihi*) compounds name something characterized by the compound: *a redhead, a paleface, a pickpocket, a scarecrow, a skinhead, a lazybones*. They have no head inside; the plural is regular even for irregular heads (*sabretooths, Maple Leafs*; Jespersen MEG VI 8.7).
- **Copulative** (*dvandva*-like) compounds join coordinate elements: *singer-songwriter, fridge-freezer, Austria-Hungary, bittersweet*.
- **Appositional** compounds name something that is both: *maidservant, boy king, woman doctor* (plural *women doctors*).

The relation between the parts of an endocentric noun–noun compound is not marked: *a mountain cabin* (located on), *a wood cabin* (made of), *a hunting cabin* (used for), *a dog bed* (for), *a dog bite* (caused by), *a dog owner* (object of the action). Levi's (1978) "recoverably deletable predicates" (CAUSE, HAVE, MAKE, USE, BE, IN, FOR, FROM, ABOUT) are one classification of these relations; Schäfer 2018 (ch. 4) reviews Levi's and later approaches, and studies how **semantic transparency** (how far the meaning comes from the parts: *bookshelf* vs *hogwash*) can be measured and modelled (ch. 2–3, 5–7). Transparency also correlates, weakly, with stress (§3.1.2 of his book).

### 2.3 Synthetic (verbal) compounds

When the head is derived from a verb, the first element is usually its object: *truck driver* (drives trucks), *bookseller, heart-breaking, decision-making, mind-reader, peace-keeping, man-eating*. The pattern is productive; a subject interpretation is rare (*\*a child-crying* "a crying by a child").

### 2.4 Compound adjectives

| pattern | examples |
|---|---|
| noun + adjective | *colour-blind, sugar-free, world-famous, waterproof, sky-blue, ice-cold* |
| adjective + adjective | *bittersweet, light-green, dark-blue, deaf-mute* |
| noun + participle | *heart-breaking, man-made, home-made, sun-dried, God-given, hand-written* |
| adjective/adverb + participle | *good-looking, hard-working, far-reaching, well-known, newly-born* |
| adjective + noun + *-ed* | *blue-eyed, long-legged, open-minded, kind-hearted* ([17](17-derivation-suffixes.md) §3.2) |
| number + noun | *five-year(-old), ten-mile, three-day* (noun singular) |
| phrase compounds | *up-to-date, day-to-day, state-of-the-art, over-the-counter, would-be* |

Before a noun they are hyphenated (*a well-known writer*), after a verb often not (*the writer is well known*). PTB tags a hyphenated prenominal compound modifier JJ (*income-tax/JJ return*); EWT keeps most hyphenated words as one token (173 tokens with an internal hyphen between letters in EWT train), but splits others with `HYPH` tokens, following its tokenization dictionary.

### 2.5 Compound verbs

Most compound verbs are made by **back-formation** from compound nouns or adjectives (§4): *babysit* (from *babysitter*), *sightsee* (*sightseeing*), *housekeep, brainwash, spoon-feed, test-drive, window-shop, stage-manage, sleepwalk*; or by conversion of compound nouns (*to blacklist, to pigeonhole, to short-circuit, to fast-track, to bad-mouth*). Particle + verb compounds are old and mostly inseparable: *overcome, undergo, uphold, withdraw, outrun, overtake* (Jespersen MEG VI 9.7). **Phrasal verbs** (*give up, look after*) are not compounds but multiword expressions ([19](19-lexicon-mwe-register.md) §3.1); their nominalizations are compounds (*a give-away, a lookout*). In UD the particle of a phrasal verb is `compound:prt` (745 in EWT train).

### 2.6 Inflection of compounds

- The head inflects: *toothbrushes, blackbirds, babysits, babysat* (irregular head verb keeps its irregularity: *understood, overcame*).
- The first element is normally singular even with plural meaning (*a toothbrush, a five-pound note, a ten-year-old*), except pluralia tantum and some fixed plurals (*clothes-brush, sales tax, arms race, sports car*) (Jespersen MEG VI 8.8–8.9; [11](11-nouns.md) §9).
- Exocentric compounds take regular inflection: *lowlifes, still lifes, Maple Leafs, sabre-tooths*.
- Phrase compounds pluralize their head: *mothers-in-law, passers-by*; colloquially the end: *mother-in-laws*.
- Genitive attaches at the end: *my mother-in-law's car* ([11](11-nouns.md) §6.2).

## 3. Conversion

### 3.1 What it is

Conversion (zero derivation, functional shift) moves a word to another class without any affix: *a bottle → to bottle, to walk → a walk, dry → to dry, up → to up (the price)*. After conversion the word takes **all** the marks of its new class: *bottles, bottled, bottling* as a verb; *a walk, two walks* as a noun. Jespersen MEG VI chapters VI–VII call it "the naked word" (derivation with zero suffix, 6.1); Sweet NEG I §§105–106 stresses that a converted word takes all the formal marks of the new class. Reynolds 2026 §8.2.2 cites *run* (noun from verb) as a morphological fact with "nothing changed at all". Hejná & Walkden 2022 §4.5.1 note (after Millward & Hayes) that conversion became a common way of expanding the vocabulary in Early Modern English, 1500–1700, and that it stays productive today (their web example: *I was bumblebeed if I was going to buy one*).

**Scale.** Of the 16 064 words that AGID lists as verbs (including uncertain entries), 5 801 (36%) are also listed as nouns (counted 2026-10-17 over `infl.txt`); in Mova's lexicon 5 856 lemmas have both a NN and a VB base entry, and 996 both JJ and VB.

### 3.2 Directions

| direction | examples | typical meanings of the new word |
|---|---|---|
| noun → verb | *to bottle, to butter, to email, to google, to phone, to text, to hammer, to skin, to father, to table, to elbow, to ship, to blacklist* | put in/on (*bottle, pocket*), remove (*skin, dust, peel*), use as instrument (*hammer, elbow, email*), act as (*father, nurse, captain*), go by (*bike, ship*) |
| verb → noun | *a walk, a look, a run, a cut, a drink, a guess, a hit, a find, a must, a read, an ask* (informal) | single act, result, thing (*have a look, a good read, a big ask*) |
| adjective → verb | *to dry, to empty, to clean, to calm, to free, to open, to slow, to wet, to better, to warm* | make or become X |
| adjective → noun | *a regular, a native, a criminal, the rich* (partial: [15](15-adjectives-adverbs.md) §7) | person or thing with the property |
| adverb/preposition → verb | *to up, to down, to out, to off* (slang "kill"), *to near* | |
| preposition → noun | *the ins and outs, the ups and downs* | |
| function word → noun | *ifs and buts, no ifs, ands or buts* | |
| phrase → adjective, noun | *a must-see film, an over-the-top reaction, a has-been, a wannabe* | |
| proper name → noun or verb | *a Plato, a Judas; to boycott, to hoover, to google* | §7 |

### 3.3 Partial conversion with sound changes

Some noun–verb pairs differ in stress, final voicing or vowel, which marks them as historically related but not converted in the modern sense ([03](03-syllable-stress-intonation.md) §4.2; [01](01-consonants.md) §3.2; [04](04-connected-speech.md) §8):

| difference | pairs |
|---|---|
| stress | *ˈrecord / reˈcord, ˈobject / obˈject, ˈpresent / preˈsent, ˈprotest / proˈtest* |
| voicing | *use* /s/–/z/, *house* /s/–/z/, *belief–believe, proof–prove, bath–bathe, advice–advise* |
| vowel | *food–feed, blood–bleed, full–fill, sing–song, sit–seat, lie–lay* |

Jespersen MEG VI chapters XI–XII treat these "change of vowel" and "change of consonant without any addition of formative".

### 3.4 Constraints

- Verbs are formed from the singular of the noun (*to dice* is an exception; *to arm* from *arms*).
- Conversion is **blocked** by an existing word: *to cook* (not \**to cooker*), *a thief* (not \**a steal* in the agent sense), *arrival* blocks \**an arrive*.
- The converted word follows the regular inflection, even if its source is irregular: *to grandstand – grandstanded*, *to ring* "encircle" – *ringed* ([14](14-irregular-verbs.md) §4), *to fly out* (baseball) – *flied*.

### 3.5 Conversion and tagging

Conversion is the main reason English words are ambiguous in class ([10-word-classes.md](10-word-classes.md) §5). A word with an article or determiner before it cannot be a finite verb (seed `wf-conversion.md` has this rule); a word after *to* or a modal is a verb; the inflectional ending, when present, decides (*walks* is NNS or VBZ; *walked* VBD/VBN only).

## 4. Back-formation

Back-formation removes what looks like an affix to make a new, simpler word, usually a verb from a noun:

| source | back-formation | supposed affix |
|---|---|---|
| *editor* | *edit* | *-or* |
| *burglar* | *burgle* (BrE), *burglarize* (AmE: suffixation instead) | *-ar* |
| *television* | *televise* | *-ion* |
| *donation* | *donate* | *-ion* |
| *emotion* | *emote* | *-ion* |
| *enthusiasm* | *enthuse* | *-asm* |
| *diagnosis* | *diagnose* | |
| *babysitter, housekeeper, sightseeing, brainwashing* | *babysit, housekeep, sightsee, brainwash* | *-er, -ing* |
| *peddler* | *peddle* | *-er* |
| *pease* (old singular) | *pea* | plural *-s* ([11](11-nouns.md) §10.2) |
| *cherise* | *cherry* | plural *-s* |
| *greedy* | *greed* | *-y* |
| *laser* | *lase* | *-er* (technical) |

Jespersen MEG VI 29.3 ("back-formations") and 4.4₃ (*edit*); Kruisinga II.3 §§1904–1911 (*burgle, caretake, subedit*). New back-formations are frequent in journalism and jargon (*to liaise, to surveil, to explicate*), often criticized at first.

## 5. Clipping

### 5.1 Kinds

| what is kept | examples |
|---|---|
| the beginning (most common) | *exam(ination), lab(oratory), ad(vertisement), photo(graph), gym(nasium), math(ematic)s, info(rmation), demo(nstration), vet(erinarian), pub(lic house), fan(atic), doc(tor), memo(randum), sub(marine, stitute)* |
| the end | *(tele)phone, (omni)bus, (aero)plane, (rac)coon, (alli)gator, (in)fluenza* is both ends cut → *flu* |
| the middle | *(in)flu(enza), (de)tec(tive) → tec* |
| two parts (clipped compounds) | *sitcom* (situation comedy), *hi-fi* (high fidelity), *sci-fi*, *Interpol*, *Wi-Fi* |

Clippings become independent lexemes with their own plural and register: *exams, phones*; many were slangy and became neutral (*bus, pub, mob* from *mobile vulgus*, *fan, cab* from *cabriolet*). Some keep a stylistic difference: *doc, prof, uni* (BrE, AusE), *telly* (BrE). Jespersen MEG VI 29.2–29.8 calls the clipped words "stump-words" (seed `wf-shortening.md`).

### 5.2 Aphesis

Loss of an unstressed initial syllable gave doublets with different meanings: *mend* (< *amend*), *fence* (< *defence*), *sport* (< *disport*), *spite* (< *despite*), *squire* (< *esquire*), *cute* (< *acute*), *lone* (< *alone*), *'cause, 'bout, 'round* (casual) (Jespersen MEG VI 29.8).

### 5.3 Clipping with a suffix

Australian and British informal English clip and add *-ie/-y* or *-o*: *barbie, brekkie, footy, sunnies, telly, nightie, hankie; arvo, servo, journo, ambo, rego, smoko, combo, lesbo* ([08](08-national-varieties.md) §6; [17](17-derivation-suffixes.md) §7).

## 6. Acronyms and initialisms

| kind | pronounced | examples |
|---|---|---|
| initialism | letter by letter | *BBC, FBI, USA, UK, CEO, DIY, FAQ, PhD, ATM, DVD, OK* (from *oll korrect*, 1839) |
| acronym | as a word | *NATO, NASA, UNESCO, AIDS, PIN, radar* (radio detection and ranging), *laser, scuba, sonar* |
| mixed | partly | *JPEG* (jay-peg), *CD-ROM* |
| recursive / backronyms | | *GNU*; *SOS* (not an abbreviation) |

Acronyms that become ordinary words lose their capitals (*laser, radar, scuba, zip code*) and inflect (*lasers*). The article goes by the first sound of the spoken form: *an FBI agent, a NATO summit, an MRI* ([12](12-pronouns-determiners-numerals.md) §8.2). Text messaging and web slang use many initialisms as words or phrases: *lol, omg, btw, idk, tbh, imo, brb* (EWT tags some as UH, some by their function). UD gives written abbreviations `Abbr=Yes`, inconsistently for initialisms (seed `wf-shortening.md`). Jespersen MEG VI 29.9 is an appendix on "alphabetic shortenings".

## 7. Blends

A blend joins the beginning of one word to the end of another: *brunch* (breakfast + lunch), *smog* (smoke + fog), *motel* (motor + hotel), *podcast* (iPod + broadcast), *Brexit* (Britain + exit), *emoticon, infotainment, staycation, spork, chortle* (chuckle + snort, Lewis Carroll's "portmanteau word"), *Spanglish, Oxbridge*. Blends are frequent in brand names and journalism. Carroll's Humpty Dumpty explains *slithy* as "lithe and slimy… like a portmanteau: there are two meanings packed up into one word" (*Through the Looking-Glass*, 1871, ch. 6; public domain).

## 8. Neoclassical compounds

Learned vocabulary combines bound stems from Greek and Latin (**combining forms**), usually joined with *-o-* (Greek) or *-i-* (Latin):

| initial combining forms | final combining forms |
|---|---|
| *auto-* (self), *bio-* (life), *geo-* (earth), *tele-* (far), *photo-* (light), *micro-* (small), *macro-* (large), *psycho-* (mind), *neuro-* (nerve), *hydro-* (water), *electro-*, *thermo-* (heat), *chrono-* (time), *socio-*, *anthropo-* (human), *phil(o)-* (love), *poly-*, *mono-* | *-logy* (study), *-graphy/-graph* (writing), *-phone* (sound), *-scope* (viewing), *-meter* (measure), *-cracy/-crat* (rule), *-phobia* (fear), *-phile* (lover), *-cide* (killing), *-vore* (eating), *-morph, -nomy, -pathy, -sphere, -gram* |

Examples: *biology, geography, telephone, photograph, microscope, thermometer, democracy, technophobia, anglophile, pesticide, herbivore, chronometer*. Unlike affixes, combining forms can combine with each other without a free base (*tele-phone, bio-logy*) and can occasionally stand alone (*phone, photo, graph, phobia*). New hybrid formations mix them with English bases (*television* from Greek *tele-* + Latin *vision*; *workaholic* from the clipped *-(a)holic* of *alcoholic*). Stress follows [03](03-syllable-stress-intonation.md) §5.3: *ˈphotograph, phoˈtography, photoˈgraphic*.

## 9. Words from names and from sounds

- **Eponyms** (words from proper names): *sandwich* (Earl of Sandwich), *boycott* (Captain Boycott), *cardigan, wellington, diesel, volt, watt, ampere, braille, saxophone, guillotine, lynch, mesmerize, pasteurize, bowdlerize*; trademarks: *hoover, aspirin, escalator, google, xerox, kleenex, zipper*. As common nouns and verbs they lose the capital and inflect (*to google, googled*).
- **Onomatopoeia**: *buzz, hiss, bang, crash, cuckoo, sizzle, splash, meow, woof*; with sound symbolism in clusters (*gl-* for light: *glow, gleam, glitter, glint*; *sn-* for the nose: *sniff, snore, sneeze, snout*; *-ash* for violent contact: *bash, crash, smash, splash*).
- **Reduplication**: §10.
- **Coinage from nothing** is rare: *googol*, *quark* (from Joyce), *blurb*, *grok* (from Heinlein).

## 10. Reduplication

A small set of words repeat a stem:

| type | examples |
|---|---|
| exact repetition | *bye-bye, so-so, goody-goody, pooh-pooh, night-night, tut-tut, choo-choo* |
| vowel change (ablaut; *i* before *a/o*) | *zigzag, chit-chat, ping-pong, tick-tock, flip-flop, dilly-dally, shilly-shally, tip-top, sing-song, criss-cross, see-saw, wishy-washy, riff-raff* |
| rhyme (initial consonant changed) | *hanky-panky, helter-skelter, higgledy-piggledy, hocus-pocus, hip-hop, hodgepodge, willy-nilly, nitty-gritty, teeny-weeny, super-duper* |
| *shm-* reduplication (from Yiddish) | *fancy-shmancy, money-shmoney* (dismissive) |
| contrastive focus reduplication | *I'm not LIKE-like him; Is it a salad or a SALAD-salad?* (the "real, prototypical" X) |

Meanings: sound (*tick-tock*), repetition or alternation (*zigzag, flip-flop*), contempt or playfulness (*hocus-pocus, fancy-shmancy*), child language (*bye-bye, choo-choo*). The class depends on use: *zigzag* noun and verb (*zigzagged*), *so-so* adjective and adverb. Repetition for emphasis (*very very good*) is syntax, not word formation (Jespersen MEG VI ch. X, 10.2–10.4; Kruisinga II.3 §§1913–1918; seed `wf-reduplication.md`).

## 11. How Mova handles these processes

- **Compounds**: solid and hyphenated compounds are single tokens with their own lemma (*toothbrushes → toothbrush*, *sons-in-law → son-in-law*: UD convention); the tokenizer decides which hyphenated words stay whole from a dictionary learned from UD train (`hyphen_words` in `en/src/tok.rs`). Open compounds are two tokens, and the parser attaches the first as `compound` (EWT has 4 472 NOUN and 2 267 PROPN `compound` dependents in train). Compound meaning (the relation between the parts) is not analysed in `en`; Mova's frames and world layers read *bus stop, credit card* as a head with a modifier.
- **Conversion**: handled entirely by tagging: the same string gets NN or VB by context, and the lemma follows the class (*walks* NNS → *walk* as a noun; VBZ → *walk* as a verb). The lexicon knows both uses for thousands of words (§3.1).
- **Clippings, blends, acronyms, eponyms**: ordinary lemmas; acronyms keep their case in the lemma when tagged NNP (`Morph::lemmatize` keeps case for proper nouns: [07](07-punctuation-capitals-numbers.md) §8).
- **Back-formations and neoclassical compounds**: ordinary lemmas; no link to their sources.
- **Not yet**: no compound splitter for unknown solid compounds (*crimefighter*), which the tagger handles through the suffix of the last element; no representation of compound semantics; no list of phrasal-verb nominalizations. The LADEC database of 8 956 English compounds with transparency ratings (Gagné et al. 2019, `$MOVA_DATA/raw/en-lex-ladec-2019`) is CC BY-NC, so usable for tests only.

## Sources for this chapter

- Jespersen 1942, MEG VI chapters VI–VII ("the naked word": conversion), VIII–IX (compounds), X (reduplicative compounds), XI–XII (vowel and consonant change), XXIX (shortenings: 29.2 clipped compounds, 29.3 back-formations, 29.4–29.8 stump-words, 29.9 alphabetic shortenings). https://archive.org/details/modernenglishgra06jesp
- Sweet 1891, NEG I §§63–68, 105–106, 164, 900–915, 1536–1556. https://archive.org/details/newenglishgramma01sweeuoft
- Kruisinga 1932, *Handbook* II.3 §§1904–1918 (shortening, reduplication). https://archive.org/details/handbookofpresen32krui
- Schäfer, Martin. 2018. *The Semantic Transparency of English Compound Nouns.* Language Science Press, CC BY 4.0 (ch. 2–3 transparency; §3.1.2 transparency and stress; ch. 4 semantic analyses: Levi 1978, Fanselow 1981, Pustejovsky, Jackendoff, Asher). https://zenodo.org/records/1134595
- Schulte im Walde, Sabine & Eva Smolka (eds.). 2020. *The Role of Constituents in Multiword Expressions.* Language Science Press, CC BY 4.0 (compounds and MWEs across languages). https://zenodo.org/records/3598577
- Hejná & Walkden 2022, §4.5.1 (conversion in Early Modern English). https://zenodo.org/records/6560337
- Reynolds 2026, §8.2.2. https://zenodo.org/records/20612113
- Carroll, Lewis. 1871. *Through the Looking-Glass*, ch. 6. Public domain (Project Gutenberg #12).
- Gagné, Christina L., Thomas L. Spalding & Daniel Schmidtke. 2019. "LADEC: The Large Database of English Compounds." *Behavior Research Methods* 51. Data CC BY-NC 4.0 (tests only).
- Atkinson, AGID 2016.01.19 (counts of 2026-10-17). http://wordlist.aspell.net/other/
- Mova: `en/src/tok.rs`, `en/src/morph.rs`; seeds `en/seeds/morph/wf-compounds.md`, `wf-conversion.md`, `wf-shortening.md`, `wf-reduplication.md`, `noun-plural-compounds.md`; counts from UD English EWT 2.18 train (2026-10-17).
