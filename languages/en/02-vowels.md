# 02. Vowels

English has one of the larger vowel systems among the world's languages, and it is where its accents differ most. This chapter describes the two reference accents, SSBE (the modern southern British standard, successor of RP) and GA (General American), names each vowel by a **lexical set** (a keyword that stands for all the words with that vowel), and lists the splits and mergers that make the accents differ. The other varieties are in [08-national-varieties.md](08-national-varieties.md).

## 1. How the vowels are classified

- **Height** (close/high, mid, open/low), **backness** (front, central, back) and **lip rounding**. Reynolds 2026 §7.4.2 gives this three-way description and a vowel chart for Canadian English; Sweet NEG I §§669–691 gives the classical analysis (rounding, tongue-retraction, tongue-height, "the vowels in detail").
- **Checked and free vowels.** The *checked* (lax, "short") vowels /ɪ e æ ɒ ʌ ʊ/ cannot end a stressed word: there is no word \*/bɪ/ or \*/bæ/. The *free* (tense, "long") vowels and diphthongs can: *bee, bay, buy, bow, boy, bough, bar, bore, burr, boo*. This distinction matters more than length: Reynolds 2026 §7.6 warns that "long" and "short" in school usage mix length with quality, so that *mate* is "long *a*" although its vowel is a diphthong.
- **Monophthongs and diphthongs.** A diphthong glides from one quality to another inside one syllable: *ride* /aɪ/, *loud* /aʊ/, *boy* /ɔɪ/.
- **Stressed and weak vowels.** In unstressed syllables the vowel system shrinks to a few **weak vowels**, chiefly /ə/ (schwa), /ɪ/ (or /ə/), /i/, /u/ (§5).

## 2. The lexical sets

The names are the standard keywords of English dialectology, introduced by Wells (1982, *Accents of English*; not in the library, see [sources.md](sources.md) "wanted"); Hejná & Walkden 2022 use them throughout (e.g. "the FOOT-STRUT split", §3.2.3; the FACE, GOAT and MOUTH sets, §4.2.1.2). Values are broad phonemic transcriptions; SSBE follows the current British dictionary convention, with the older RP symbol in brackets where it differs.

| set | keywords | SSBE | GA | notes |
|---|---|---|---|---|
| KIT | *kit, bid, hymn, minute (n.)* | ɪ | ɪ | |
| DRESS | *dress, step, head, bury, many* | e (ɛ) | ɛ | |
| TRAP | *trap, bad, cab, hand* | æ (a) | æ | SSBE now often [a] |
| LOT | *lot, odd, stop, wash* | ɒ | ɑ | GA merges with PALM (§4.3) |
| STRUT | *strut, cup, love, blood, son* | ʌ | ʌ | absent in the north of England (§4.1) |
| FOOT | *foot, good, put, wolf, could* | ʊ | ʊ | |
| BATH | *bath, staff, dance, ask, laugh, can't* | ɑː | æ | the TRAP–BATH split (§4.2) |
| CLOTH | *cloth, off, cough, long, cross* | ɒ | ɔ or ɑ | |
| NURSE | *nurse, bird, word, heard, term* | ɜː | ɝ (ɜr) | |
| FLEECE | *fleece, sea, machine, people* | iː | i | |
| FACE | *face, day, steak, vein, they* | eɪ | eɪ | monophthong [eː] in Scotland, North of England |
| PALM | *palm, father, bra, spa* | ɑː | ɑ | |
| THOUGHT | *thought, law, north, caught, broad* | ɔː | ɔ (or ɑ) | cot–caught merger (§4.4) |
| GOAT | *goat, home, soul, toe, know* | əʊ | oʊ | |
| GOOSE | *goose, two, blue, music, through* | uː | u | fronted to [ʉː] in younger speech (§6) |
| PRICE | *price, ride, try, high, eye* | aɪ | aɪ | Canadian raising (§6) |
| CHOICE | *choice, boy, noise* | ɔɪ | ɔɪ | |
| MOUTH | *mouth, now, loud, cow* | aʊ | aʊ | |
| NEAR | *near, here, beard, fierce* | ɪə | ɪr | SSBE often [ɪː] |
| SQUARE | *square, fair, care, there, bear* | eə (ɛː) | ɛr | SSBE now usually [ɛː] |
| START | *start, far, heart, sharp* | ɑː | ɑr | |
| NORTH | *north, war, horse, short* | ɔː | ɔr | |
| FORCE | *force, four, wore, hoarse, door* | ɔː | ɔr (or) | NORTH–FORCE merger (§4.6) |
| CURE | *cure, poor, tour, jury* | ʊə (ɔː) | ʊr | SSBE increasingly /ɔː/: *poor = pour* (§4.7) |
| happY | *happy, city, coffee, money* (final) | i | i | older RP /ɪ/ (§5.2) |
| lettER | *letter, doctor, colour* (final) | ə | ɚ | |
| commA | *comma, sofa, China* (final) | ə | ə | |

SSBE thus has about 20 stressed vowel phonemes (counting /ɪə eə ʊə/), GA about 15 plus the r-coloured combinations. Reynolds 2026 §7.4.2 counts "about nine main vowel phonemes and seven diphthongs" for his Canadian accent, which has merged LOT, PALM and THOUGHT.

### 2.1 Example words by set from Mova's school lessons

Vowels cannot be read off the spelling. These school sentences (`$MOVA_DATA/parsedb/current.conllu`) show how one letter covers several sets: *A dog that growls, shows its teeth or goes stiff is warning you to stay away* (`school:everyday:012-sounds-that-warn.lesson:13`) has ⟨o⟩ in LOT (*dog*; CLOTH in GA) and GOAT (*shows, goes*), ⟨ar⟩ in NORTH (*warning*, the THOUGHT vowel in SSBE), ⟨a⟩ in TRAP (*that*), commA (*away*) and, with ⟨y⟩, FACE (*stay, away*), and ⟨ee⟩ in FLEECE (*teeth*).

## 3. The vowels described

### 3.1 Checked vowels

- **KIT /ɪ/**: front, close-mid, lax. Spelled ⟨i⟩ (*sit*), ⟨y⟩ (*myth*), ⟨e⟩ (*pretty, English*), ⟨u⟩ (*busy*), ⟨o⟩ (*women*), ⟨ui⟩ (*build*).
- **DRESS /e/**: front, mid. ⟨e⟩, ⟨ea⟩ (*head, bread*), ⟨a⟩ (*any, many*), ⟨ai⟩ (*said*), ⟨ie⟩ (*friend*), ⟨u⟩ (*bury*).
- **TRAP /æ/**: front, open. In SSBE it has lowered to near [a] since the 1970s. In GA and Canadian English it is raised and diphthongized before nasals: *man* [mɛən], **TRAP-tensing**; in New York and Philadelphia the tensing makes a separate phoneme (*can* "be able" lax vs *can* "tin" tense).
- **LOT /ɒ/** (SSBE): back, open, rounded. GA has unrounded /ɑ/ = PALM.
- **STRUT /ʌ/**: in SSBE a central open-mid [ɐ], not back. In the north of England there is no STRUT vowel: *cup, love, but* have /ʊ/ (§4.1).
- **FOOT /ʊ/**: back-ish, close-mid, weakly rounded. Few words: *put, book, good, could, should, would, woman, wolf, bush, pull, full, sugar*.

### 3.2 Free vowels and diphthongs

- **FLEECE /iː/** and **GOOSE /uː/** are often slightly diphthongal ([ɪi], [ʊu]).
- **FACE /eɪ/**, **PRICE /aɪ/**, **CHOICE /ɔɪ/**, **GOAT /əʊ/** (GA /oʊ/), **MOUTH /aʊ/** are closing diphthongs.
- **NEAR /ɪə/**, **SQUARE /eə/**, **CURE /ʊə/** are centring diphthongs of non-rhotic accents, born from the loss of /r/ ([01-consonants.md](01-consonants.md) §2.6). In present SSBE they are monophthongizing: SQUARE [ɛː], NEAR [ɪː], CURE → /ɔː/.
- **NURSE /ɜː/** is the merger of Middle English /ɪr, ɛr, ʊr/ (*fir, fern, fur*), complete in the standard accents; Scottish and Irish English keep two or three vowels (*fern* ≠ *fir* ≠ *fur*).
- **START /ɑː/**, **NORTH/FORCE /ɔː/** in SSBE are long monophthongs; in GA they are vowel + /r/.

### 3.3 Length and the voicing of the next consonant

All vowels are longer before a lenis consonant than before a fortis one (*bead* > *beat*, *rise* > *rice*) and longest at the end of a word (*bee*). This "pre-fortis clipping" is phonetic, not phonemic, but it is the main cue to final /t/ vs /d/ ([01-consonants.md](01-consonants.md) §2.1; Reynolds 2026 §7.6).

## 4. Splits and mergers: why accents differ

A **split** makes two phonemes out of one; a **merger** makes one out of two. Hejná & Walkden 2022 §3.2.3 defines a split as the rise of a new phoneme B beside the old phoneme A, both now contrastive.

| change | what happened | where | examples |
|---|---|---|---|
| **FOOT–STRUT split** (§4.1) | Middle English /ʊ/ unrounded to /ʌ/ in most words, but stayed /ʊ/ after labials in some (*put, pull, bush*) | the south of England and all overseas varieties; not in the north of England | *put* /ʊ/ ≠ *putt* /ʌ/; northern *cup* [kʊp] |
| **TRAP–BATH split** (§4.2) | /æ/ lengthened to /ɑː/ before /f θ s/ and some /n/ + consonant clusters | SSBE, southern England, Australia (partly), NZ, South Africa; not GA, not northern England | *bath, path, grass, dance* SSBE /ɑː/, GA and northern /æ/ |
| **LOT–PALM merger** (father–bother) (§4.3) | /ɒ/ unrounded and merged with /ɑ/ | GA, most of North America | *bother* rhymes with *father* |
| **LOT–THOUGHT merger** (cot–caught) (§4.4) | /ɑ/ and /ɔ/ merged | Canada, western and Midland US, Scotland | *cot = caught, Don = dawn* |
| **NURSE merger** | /ɪr ɛr ʊr/ → /ɜr/ | all standard accents | *fir, fern, fur* rhyme |
| **NORTH–FORCE merger** (§4.6) | /ɔr/ and /or/ merged | nearly everywhere; not in Scotland, Ireland, parts of the US | *horse = hoarse, for = four* |
| **CURE–FORCE merger** (§4.7) | /ʊə/ → /ɔː/ | younger SSBE, Australia, NZ | *poor = pour, sure = shore* |
| **Mary–marry–merry merger** | /ɛər, ær, ɛr/ before a vowel merged | most of the US and Canada | *Mary = marry = merry* |
| **pin–pen merger** | /ɪ/ and /ɛ/ merged before nasals | US South, African American English | *pin = pen* |
| **meet–meat merger** | Middle English /eː/ and /ɛː/ merged in /iː/ | all standard accents (a result of the Great Vowel Shift, §7) | *meet = meat, see = sea* |
| **wine–whine merger** | /hw/ → /w/ | most accents ([01](01-consonants.md) §2.7) | *which = witch* |

### 4.1 FOOT–STRUT

The split happened in the 17th century in the south of England; the north was left out. A northern English speaker thus has one vowel in *put* and *putt*, *could* and *cud*, and the absence of /ʌ/ is the main marker of a northern accent (Hejná & Walkden 2022 §3.2.3, who also note intermediate STRUT qualities among northern speakers).

### 4.2 TRAP–BATH

The words with /ɑː/ in SSBE are a closed, partly arbitrary list: *bath, path, laugh, staff, after, grass, pass, class, ask, last, fast, castle, dance, chance, plant, can't, example, demand, half, calf, rather, father* (the last three with ⟨al⟩ or ⟨a⟩ + /ð/), but not *gas, mass, maths, ant, cant, hassle, Africa*. Variation remains in *plastic, graph, chancellor, transport*. Northern English and GA have /æ/ in all of them; Australian English has /æ/ in *dance, chance, plant* and /aː/ in *bath, grass, laugh*.

### 4.3–4.4 LOT, PALM, THOUGHT in North America

GA has lost the rounded LOT vowel: *lot* /lɑt/ has the vowel of *palm* /pɑm/. Many North Americans have also merged THOUGHT with it, so *cot* and *caught* are homophones; this is general in Canada (Reynolds 2026 §7.4) and the western US. The CLOTH words (*off, long, dog, cross*) go with THOUGHT in GA (/ɔ/) where THOUGHT is distinct.

### 4.6–4.7 NORTH, FORCE, CURE

Most speakers no longer distinguish *horse* and *hoarse*, *morning* and *mourning*. In SSBE the CURE words are moving to the NORTH/FORCE vowel: *poor, sure, your, moor* now often /ɔː/; the change is slower in words with /j/ (*cure, pure, Europe*).

## 5. Weak vowels and reduction

### 5.1 Schwa

/ə/ is the most frequent vowel in running speech and in CMUdict (ARPAbet AH0 with the stressless AH1/AH2 = /ʌ/; together 8.3% of all phonemes, the largest single class; §8). It occurs only in unstressed syllables, spelled by any vowel letter: *about, taken, pencil, lemon, circus, the*. Reynolds 2026 §7.4.2 places it as "mid central … the first syllable of *about*".

### 5.2 The weak vowels

| weak vowel | where | examples |
|---|---|---|
| /ə/ | most unstressed syllables | *a*bout, sof*a*, Lond*o*n |
| /ɪ/ (SSBE, older), /ə/ (GA, younger SSBE) | before a consonant in some suffixes and prefixes | *-es* in *buses*, *-ed* in *wanted*, *be-*, *re-*, *-ic*: the **weak vowel merger** of GA makes *Lenin = Lennon*, *rabbit* /ˈræbət/ |
| /i/ (happY) | final, and before a vowel | *happy, coffee, react* /riˈækt/ |
| /u/ | before a vowel, and in weak *to, you* | *influence* /ˈɪnfluəns/, *to eat* /tu ˈiːt/ |
| syllabic consonants | /ən, əl/ after an obstruent | *button* [ˈbʌtn̩], *bottle* [ˈbɒtl̩] |

**happY-tensing**: the final vowel of *happy, city, money* was /ɪ/ in older RP; in present SSBE, GA and most other accents it is a short tense [i]. This manual writes /i/.

### 5.3 Reduction and full vowels in related words

Because vowels reduce when they lose stress, related words that differ in stress show different vowels for the same letters ([03-syllable-stress-intonation.md](03-syllable-stress-intonation.md) §5; [17-derivation-suffixes.md](17-derivation-suffixes.md) §9):

| unstressed → stressed | |
|---|---|
| *ph*o*tograph* /ˈfəʊtəɡrɑːf/ – *ph*o*t*o*graphy* /fəˈtɒɡrəfi/ | ⟨o⟩ = /əʊ/ ~ /ə/ ~ /ɒ/ |
| *Canada* /ˈkænədə/ – *Canadian* /kəˈneɪdiən/ | ⟨a⟩ = /æ/ ~ /ə/ ~ /eɪ/ |
| *atom* /ˈætəm/ – *atomic* /əˈtɒmɪk/ | |
| *origin* /ˈɒrɪdʒɪn/ – *original* /əˈrɪdʒɪnəl/ | |

The spelling keeps the full vowel, so a word's family stays visible on the page; this is one of the arguments for English spelling given in [05-writing-system.md](05-writing-system.md) §7.

### 5.4 Strong and weak forms

Function words have a strong form with a full vowel (used under stress or at the end of a phrase) and a weak form with /ə/ or no vowel: *can* /kæn/ vs /kən/, *to* /tuː/ vs /tə/, *and* /ænd/ vs /ən, n/. The list is in [04-connected-speech.md](04-connected-speech.md) §2.

## 6. Some ongoing changes

- **GOOSE- and GOAT-fronting**: /uː/ and /əʊ/ have moved forward in younger speech in England, the US and Australia ([ʉː], [əʉ]).
- **Canadian raising**: in Canada and parts of the northern US, /aɪ/ and /aʊ/ have raised first elements before voiceless consonants: *write* [ʌɪt] ≠ *ride* [aɪd], *out* [ʌʊt]. Reynolds 2026 §7.4.2.3 lists /aɪ/–/ʌɪ/ and /aʊ/–/ʌʊ/ separately for his Toronto speech for this reason.
- **The Northern Cities Shift** (Inland North of the US: Chicago, Detroit, Buffalo): TRAP raised, LOT fronted into TRAP's place, THOUGHT lowered, STRUT backed, DRESS backed: a chain shift (Hejná & Walkden 2022 §2.2.2).
- **The short front vowel shift** of Australia, New Zealand and parts of England: KIT, DRESS, TRAP move (NZ *fish and chips* sounding "fush and chups" to outsiders, [08](08-national-varieties.md) §7).
- **SSBE monophthongization** of SQUARE and NEAR (§3.2) and the lowering of TRAP (§3.1).

## 7. History: the Great Vowel Shift

Between about 1400 and 1700 the Middle English long vowels moved up one step, and the highest became diphthongs (Hejná & Walkden 2022 §4.2.1). The spelling had been fixed before or during the shift, which is why ⟨i⟩ is /aɪ/ in *time* but /ɪ/ in *tim*ber, why ⟨ee⟩ and ⟨ea⟩ both say /iː/, and why "long a" names the diphthong /eɪ/.

| Middle English | Present SSBE | example (ME → now) | spelling kept |
|---|---|---|---|
| /iː/ | /aɪ/ | *bite* [biːt] → [baɪt] | ⟨i–e⟩ |
| /eː/ | /iː/ | *beet* [beːt] → [biːt] | ⟨ee⟩ |
| /ɛː/ | /iː/ (via /eː/) | *beat* [bɛːt] → [biːt] | ⟨ea⟩ (except *great, break, steak*) |
| /aː/ | /eɪ/ | *name* [naːmə] → [neɪm] | ⟨a–e⟩ |
| /uː/ | /aʊ/ | *house* [huːs] → [haʊs] | ⟨ou⟩, ⟨ow⟩ |
| /oː/ | /uː/ | *goose* [ɡoːs] → [ɡuːs] | ⟨oo⟩ |
| /ɔː/ | /əʊ/ | *boat* [bɔːt] → [bəʊt] | ⟨oa⟩, ⟨o–e⟩ |

Hejná & Walkden note two further outcomes: some British dialects never diphthongized FACE and GOAT (monophthongs [eː], [oː]), and in Scots and some northern English dialects pre-shift /uː/ did not diphthongize, giving *hoose, coo, moose* for *house, cow, mouth* (§4.2.1.2). Vowels shortened before the shift escaped it, which gives pairs like *sane/sanity, wise/wisdom, five/fifth, keep/kept, child/children* (the same section; and [04](04-connected-speech.md) §7 on the resulting alternations).

## 8. How often each vowel occurs (CMUdict)

Shares of all phonemes in the first pronunciation of each CMUdict headword (counted 2026-10-17); ARPAbet does not distinguish /ʌ/ and /ə/ (both AH), and ER covers both stressed /ɝ/ and unstressed /ɚ/:

| ARPAbet | IPA (GA) | share | | ARPAbet | IPA | share |
|---|---|---|---|---|---|---|
| AH | ʌ, ə | 8.32% | | OW | oʊ | 2.29% |
| IH | ɪ | 5.71% | | EY | eɪ | 1.58% |
| IY | i | 4.02% | | AO | ɔ | 1.35% |
| ER | ɝ, ɚ | 3.42% | | AY | aɪ | 1.31% |
| EH | ɛ | 3.19% | | UW | u | 1.15% |
| AA | ɑ | 2.98% | | AW | aʊ | 0.40% |
| AE | æ | 2.56% | | UH | ʊ | 0.28% |
| | | | | OY | ɔɪ | 0.15% |

## 9. How Mova handles vowels

Mova reads and writes text only; it has no vowel model. Vowel alternations matter to it where they are **morphological**: *sing–sang–sung*, *foot–feet*, *keep–kept*, *wide–width*. All of them are listed, not computed: the irregular forms are entries in the built-in lexicon (`en/data/forms.tsv`, from AGID and UD), and the lemmatizer (`en/src/morph.rs`, `Morph::lemmatize`) finds them there before it tries suffix rules ([13-verbs.md](13-verbs.md) §10). Derivational vowel alternations (*sane–sanity*, *wise–wisdom*) are not analysed at all: Mova lemmatizes each as its own word.

## Sources for this chapter

- Hejná & Walkden 2022, *A History of English*: §2.2.2 (Northern Cities Shift), §3.2 (Late Modern sounds: /h/, /r/, FOOT–STRUT), §4.2.1 (Great Vowel Shift). https://zenodo.org/records/6560337
- Reynolds 2026, *Language Landscapes*, §§7.4.2, 7.6. https://zenodo.org/records/20612113
- Sweet 1891, *A New English Grammar I*, §§669–691 (vowels). https://archive.org/details/newenglishgramma01sweeuoft
- Kruisinga 1925, *English Sounds*, chapter III (free, checked and unstressed vowels). https://archive.org/details/handbookofpresen0000ekru
- Wells, John C. 1982. *Accents of English* (3 vols). Cambridge University Press. Not in the library (copyrighted); cited for the lexical sets; listed under "wanted" in [sources.md](sources.md).
- CMUdict, BSD-2-Clause, counts of 2026-10-17. https://github.com/cmusphinx/cmudict
