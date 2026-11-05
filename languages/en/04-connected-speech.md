# 04. Connected speech and morphophonology

Words in running speech do not sound as they do in a dictionary. Function words lose their vowels, consonants assimilate to their neighbours or drop out, and non-rhotic accents insert /r/ between vowels. This chapter describes those processes (§§1–5) and then the **morphophonology** of English: the regular alternations in the shape of endings and stems that inflection and derivation cause (§§6–8).

## 1. Why it matters for a text model

Most connected-speech processes are invisible in standard spelling, but some surface in writing:

- in **contractions**, which are standard (*don't, it's, I'll*: [13-verbs.md](13-verbs.md) §9);
- in **eye dialect** and informal spelling: *gonna, wanna, gotta, kinda, lemme, gimme, dunno, y'all, 'em, o'* (*cup o' tea*), *-in'* for *-ing*;
- in **spelling errors** that follow pronunciation: *should of* for *should've*, *alot*, *could care less*;
- in **historical spellings** that fixed an assimilation: *impossible, illegal, irregular* ([16-derivation-prefixes.md](16-derivation-prefixes.md) §2.2).

## 2. Weak forms

Function words have a **strong form**, used when stressed, cited or at the end of a phrase, and a **weak form**, used otherwise, which is far more frequent. Kruisinga I §184 introduces weak forms and §265 lists "the chief words showing gradation in English … chiefly pronouns, auxiliaries, prepositions, and conjunctions"; the list below follows his, with modern SSBE and GA values.

| word | strong | weak | example of the weak form |
|---|---|---|---|
| *a* | /eɪ/ | /ə/ | *a cup* /ə ˈkʌp/ |
| *an* | /æn/ | /ən/ | *an egg* |
| *the* | /ðiː/ | /ðə/ (before C), /ði/ (before V) | *the end* /ði ˈend/ |
| *and* | /ænd/ | /ənd, ən, n/ | *bread and butter* [ˈbredn̩ ˈbʌtə] |
| *but* | /bʌt/ | /bət/ | |
| *or* | /ɔː(r)/ | /ə(r)/ | *two or three* |
| *than* | /ðæn/ | /ðən/ | *bigger than me* |
| *that* (conj./rel.) | /ðæt/ | /ðət/ | *I said that it was* (the demonstrative *that* has no weak form) |
| *as* | /æz/ | /əz/ | *as good as* |
| *at* | /æt/ | /ət/ | *at home* |
| *for* | /fɔː(r)/ | /fə(r)/ | *for you* |
| *from* | /frɒm/ | /frəm/ | |
| *of* | /ɒv/ | /əv, ə/ | *a cup of tea* [ə ˈkʌp ə ˈtiː] |
| *to* | /tuː/ | /tə/ (before C), /tu/ (before V) | *to go*, *to eat* |
| *some* (quantifier) | /sʌm/ | /səm, sm̩/ | *some milk* (the strong form means "a certain") |
| *am* | /æm/ | /əm, m/ | *I'm* |
| *are* | /ɑː(r)/ | /ə(r)/ | *we're* |
| *is* | /ɪz/ | /z, s/ | *it's, he's* |
| *was* | /wɒz/ | /wəz/ | |
| *were* | /wɜː(r)/ | /wə(r)/ | |
| *be, been* | /biː, biːn/ | /bi, bɪn/ | |
| *have, has, had* | /hæv, hæz, hæd/ | /həv, əv, v; həz, əz, z, s; həd, əd, d/ | *should have* /ʃʊd əv/ → *should've* |
| *do, does* | /duː, dʌz/ | /də, du; dəz/ | *do you* /dʒu/ |
| *can, could* | /kæn, kʊd/ | /kən, kn̩; kəd/ | *I can go* [aɪ kŋ ˈɡəʊ] |
| *shall, should* | /ʃæl, ʃʊd/ | /ʃəl, ʃl̩; ʃəd/ | |
| *will, would* | /wɪl, wʊd/ | /l, əl; wəd, əd, d/ | *I'll, I'd* |
| *must* | /mʌst/ | /məst, məs/ | |
| *not* | /nɒt/ | /nt, n/ | *isn't, can't* |
| *he, him, his, her* | /hiː, hɪm, hɪz, hɜː(r)/ | /i, ɪm, ɪz, ə(r)/ (no /h/ unless at the start) | *tell him* /ˈtel ɪm/ |
| *them* | /ðem/ | /ðəm, əm/ | *tell 'em* |
| *us* | /ʌs/ | /əs, s/ | *let's* |
| *you, your* | /juː, jɔː(r)/ | /ju, jə; jə(r)/ | *see you* /ˈsiː jə/ |
| *there* (existential) | /ðeə(r)/ | /ðə(r)/ | *there's a cat* |
| *saint* | /seɪnt/ | /snt, sn/ | *Saint Paul* |

Rules of use:

- The strong form is used under contrast (*I said AND, not OR*), when quoted, and **at the end of a phrase**: *Where are you FROM* /frɒm/, *I can* /kæn/, *That's what I'm looking AT*. Kruisinga I (text-1 p. 120) cites Jones's observation that the weak forms of *to, do, for* are never final. This is why contractions cannot end a clause: \**Yes, I'm*; \**I don't know where it's* ([13-verbs.md](13-verbs.md) §9.3).
- Weak *have* after a modal is often heard as *of* and written so by unsure writers: *should of, would of, could of*: a common real-word error. UD annotates such an *of* as AUX with lemma *have* and the feature `Style=Vrnc`, and Mova's seed `en/seeds/errors/vernacular-style.md` has a rule for it (rule `what: of in the role of have`).
- *Them* has a separate weak form *'em* /əm/ that is not a reduction of *them* but a survival of Middle English *hem* (Hejná & Walkden 2022 §3.2.1 and §5.3.3).

## 3. Elision

Sounds drop in fast or casual speech:

- **/t d/ between consonants**: *next week* [neks wiːk], *old man* [əʊl mæn], *handbag* [ˈhæmbæɡ], *mostly* [ˈməʊsli], *kindness* [ˈkaɪnnəs]. The past suffix is affected: *I walked home* [aɪ wɔːk həʊm], *he missed the bus* [hi mɪs ðə bʌs].
- **Schwa** before /r l n/ in unstressed syllables: *camera* [ˈkæmrə], *family* [ˈfæmli], *history* [ˈhɪstri], *police* [pliːs], *suppose* [spəʊz], *chocolate* [ˈtʃɒklət], and in the first syllable of *potato, tonight* (GA).
- **/h/** in weak pronouns and auxiliaries (§2).
- **/v/** in *of* before a consonant: *lots of* [ˈlɒts ə].
- **Clusters with /θ/**: *months* [mʌns], *clothes* [kləʊz] (= *close*), *sixths* [sɪks].
- **Whole syllables** in frequent phrases: *I don't know* [aɪ dəˈnəʊ] → *dunno*, *going to* → *gonna*, *want to* → *wanna*, *got to* → *gotta*, *kind of* → *kinda*, *let me* → *lemme*, *give me* → *gimme*, *probably* [ˈprɒbli].

## 4. Linking and intrusion

- **Linking /r/**: in non-rhotic accents, a word-final ⟨r⟩ is pronounced before a vowel: *far away* [fɑːr əˈweɪ], *four eggs*, *better off*.
- **Intrusive /r/**: by analogy, an /r/ is inserted after /ə ɪə ɑː ɔː ɜː/ before a vowel even where the spelling has no ⟨r⟩: *law and order* [lɔːr ənd ˈɔːdə], *the idea of it* [ði aɪˈdɪər əv ɪt], *Africa and Asia*, *drawing* [ˈdrɔːrɪŋ]. It is general in SSBE, Australian and New Zealand English but long stigmatized. Hejná & Walkden 2022 §3.2.2 ("Linking and intrusive /r/") ties both to the origin of non-rhoticity: once *car* and *spa* rhyme, the /r/ before a vowel spreads to both.
- **Glides**: between vowels a [j] or [w] glide appears after front or back vowels: *I am* [aɪ jæm], *go on* [ɡəʊ wɒn], *the end* [ði jend].
- **Liaison of final consonants** is free: *an apple* [ə ˈnæpl̩] (from which *an ewt, a nadder* became *a newt, an adder* by misdivision: [11-nouns.md](11-nouns.md) §10.2).

## 5. Assimilation

A sound takes features of a neighbour, mostly the following one (**regressive**):

| type | examples |
|---|---|
| place: /n/ → [m] before labials, [ŋ] before velars | *ten boys* [tem bɔɪz], *in Paris* [ɪm ˈpærɪs], *one car* [wʌŋ kɑː], *handkerchief* [ˈhæŋkətʃɪf] |
| place: /t/ → [p, k]; /d/ → [b, ɡ] | *that boy* [ðæp bɔɪ], *that car* [ðæk kɑː], *good girl* [ɡʊɡ ɡɜːl] |
| place: /s z/ → [ʃ ʒ] before /ʃ j/ | *this shop* [ðɪʃ ʃɒp], *does she* [dʌʒ ʃi], *this year* [ðɪʃ jɪə] |
| coalescence: /t d s z/ + /j/ → /tʃ dʒ ʃ ʒ/ | *don't you* [ˈdəʊntʃu], *would you* [ˈwʊdʒu], *bless you* [ˈbleʃu], *as yet* [əˈʒet]; fixed inside words: *nature, soldier, sure, measure* |
| voicing (progressive, in inflection) | *-s* and *-ed* take the voicing of the stem (§6) |
| devoicing at a word boundary | *have to* [ˈhæftə], *used to* [ˈjuːstə], *has to* [ˈhæstə], *of course* [əf ˈkɔːs] |

The last row explains spellings like *hafta, useta* and why *used to* (habit) is homophonous with *used to* (accustomed), but not with *used* /juːzd/ "employed".

## 6. Morphophonology of the inflections

### 6.1 The *-s* endings: plural, genitive, third person singular

The three *-s* suffixes, plural *-s*, genitive *'s* and verbal *-s*, share one set of allomorphs, chosen by the last sound of the stem:

| last sound of the stem | form | plural | genitive | verb |
|---|---|---|---|---|
| sibilant /s z ʃ ʒ tʃ dʒ/ | /ɪz/ (GA often /əz/) | *buses, roses, wishes, garages, watches, judges* | *the boss's, George's* | *misses, buzzes, washes, catches, judges* |
| other voiceless /p t k f θ/ | /s/ | *cups, cats, books, cliffs, months* | *Pat's, the cook's* | *hops, wants, walks, laughs* |
| voiced, and vowels | /z/ | *dogs, rooms, cars, days, songs* | *Bob's, the girl's* | *runs, sees, plays, sings* |

Reynolds 2026 §7.6.1 states the same three-way rule and explains the voicing choice as progressive assimilation. The rule is phonological, not orthographic: *box, quiz, maze, house* (all sibilant-final) take /ɪz/ though only some take ⟨es⟩ ([06-spelling-rules.md](06-spelling-rules.md) §4).

Special cases:

- **Plural genitive**: a plural that already ends in *-s* adds nothing in speech: *the boys'* /bɔɪz/ = *the boy's* = *the boys*. Irregular plurals add the ending: *the children's* ([11-nouns.md](11-nouns.md) §6).
- **Names in a sibilant**: *James's* /ˈdʒeɪmzɪz/ or *James'* /dʒeɪmz/; classical and biblical names prefer the short form: *Jesus', Moses', Achilles'*. Jespersen MEG VI 16.7–16.8 records the tendency to use *-s* instead of *-ses* (*for conscience' sake, for fashion sake*).
- **Fortis–lenis alternation in the stem**: *wife–wives* /f–vz/, *house–houses* /s–zɪz/, *path–paths* /θ–ðz/ (§8; [11-nouns.md](11-nouns.md) §2.3). Jespersen MEG VI 16.2–16.4 discusses which words voice.
- **Contracted *is* and *has*** follow the same rule: *Pat's here* /s/, *Bob's here* /z/; after a sibilant they cannot contract in standard writing: *the bus is late*, not \**the bus's late* (in speech /ɪz/).

### 6.2 The *-ed* ending: past and past participle

| last sound of the stem | form | examples |
|---|---|---|
| /t d/ | /ɪd/ (GA /əd/) | *wanted, needed, started, added* |
| other voiceless | /t/ | *walked, kissed, laughed, wished, watched* |
| voiced, and vowels | /d/ | *played, loved, called, robbed, judged* |

(Reynolds 2026 §7.6.1.) Adjectives in *-ed* keep an old syllabic /ɪd/ where the verb has /d/ or /t/:

| adjective /ɪd/ | verb form |
|---|---|
| *aged* (*an aged man*) | *aged* /eɪdʒd/ |
| *blessed* (*the blessed saint*) | *blessed* /blest/ |
| *crooked, naked, wicked, ragged, rugged, wretched* | (no verb, or a different one) |
| *learned* (*a learned man*) | *learned* /lɜːnd/ |
| *beloved* | *loved* |
| *dogged* (determined) | *dogged* /dɒɡd/ (followed) |
| *legged* in *three-legged*, *two-legged* (often /ɪd/) | |
| *-edly* adverbs: *supposedly, allegedly, markedly, deservedly* | |

Older poetry spells the syllabic pronunciation *-èd* or contracts the other as *-'d*: *belovèd*, *lov'd*. Jespersen MEG VI 24.1 (adjectives in *-ed*) treats these as remnants of the older syllabic ending.

### 6.3 *-ing*

The *-ing* suffix is /ɪŋ/ (casual /ɪn/, written *-in'*). After a stem in /ŋ/ the /ɡ/ is not pronounced: *singing* /ˈsɪŋɪŋ/ ([01-consonants.md](01-consonants.md) §2.4). After a stem in /ə/ or /ɪə/ a linking /r/ appears in non-rhotic accents: *hearing* /ˈhɪərɪŋ/, and intrusively in *drawing* (§4).

### 6.4 Comparative and superlative

*-er* /ə(r)/ and *-est* /ɪst/ (GA /əst/). Stems in /ŋ/ restore the /ɡ/: *longer* /ˈlɒŋɡə/, *strongest, younger* ([01](01-consonants.md) §2.4). Stems in syllabic /l/ lose the syllabicity: *simple* /ˈsɪmpl̩/ → *simpler* /ˈsɪmplə/.

## 7. Morphophonology of derivation

Latinate derivation is accompanied by fixed alternations inherited from Latin and French and from the history of English vowels. They are not active rules (new words do not always follow them), but they are regular across many word families. [17-derivation-suffixes.md](17-derivation-suffixes.md) gives them per suffix.

| alternation | name | examples |
|---|---|---|
| vowel shortening before certain suffixes and in the third syllable from the end | trisyllabic laxing, shortening | *divine–divinity* /aɪ–ɪ/, *serene–serenity* /iː–e/, *sane–sanity* /eɪ–æ/, *profound–profundity* /aʊ–ʌ/, *nation–national* /eɪ–æ/, *please–pleasant* /iː–e/, *wise–wisdom*, *five–fifty* |
| vowel reduction when stress moves | reduction | *ˈphotograph–phoˈtography*, *ˈatom–aˈtomic*, *ˈmetal–meˈtallic* ([02](02-vowels.md) §5.3, [03](03-syllable-stress-intonation.md) §5.3) |
| /k/ → /s/ before *-ity, -ism, -ize, -ian* | velar softening | *electric–electricity, critic–criticize, logic–logician, classic–classicism* |
| /ɡ/ → /dʒ/ | velar softening | *analogue–analogy, pedagogue–pedagogy, regal–regicide* |
| /t/ → /ʃ/ before *-ion, -ial, -ious* | palatalization | *act–action, part–partial, infect–infectious, relate–relation* |
| /d/ → /ʒ/ before *-ion* | palatalization | *decide–decision, explode–explosion, invade–invasion* |
| /s/ → /ʃ/ | palatalization | *confess–confession, race–racial, office–official* |
| /t/ → /s/ before *-cy* | spirantization | *democrat–democracy, pirate–piracy, president–presidency* |
| /d/ → /s/ | | *evade–evasive, defend–defence/defensive, persuade–persuasive* |
| /b/ → /p/ | | *describe–description, absorb–absorption, prescribe–prescription* |
| silent letter pronounced in a derivative | | *bomb–bombard, sign–signature, damn–damnation, hymn–hymnal, muscle–muscular, paradigm–paradigmatic, malign–malignant, column–columnist* (some speakers) |

Hejná & Walkden 2022 §4.2.1 explain the *sane/sanity* type: the vowel of *sanity* was shortened before the Great Vowel Shift and so escaped it, while the spelling kept the old shared letter.

## 8. Native alternations in stems

Old English sound changes left alternations in a small, closed set of native words. They are listed in the lexicon, never computed:

| alternation | source | examples |
|---|---|---|
| vowel change in the plural | i-mutation | *foot–feet, goose–geese, tooth–teeth, man–men, woman–women, mouse–mice, louse–lice* ([11](11-nouns.md) §3) |
| vowel change in verb forms | ablaut (strong verbs) | *sing–sang–sung, ride–rode–ridden, choose–chose–chosen* ([14](14-irregular-verbs.md)) |
| vowel shortening in the past | shortening before two consonants | *keep–kept, sleep–slept, feel–felt, mean–meant, leave–left* |
| voicing of fricatives | Old English voicing between vowels | *wife–wives, house–houses, bath–bathe* ([01](01-consonants.md) §3.2) |
| vowel change between noun and verb or adjective | mutation and ablaut | *blood–bleed, food–feed, full–fill, gold–gild, tale–tell, sing–song, bind–bond, strong–strength, long–length, broad–breadth, deep–depth, foul–filth, hale–health* (Jespersen MEG VI ch. XI) |

## 9. How Mova handles connected speech and morphophonology

- **Spoken reductions in writing.** Mova's tokenizer (`en/src/tok.rs`) splits the standard clitics *n't, 's, 're, 've, 'll, 'd, 'm* (the constant `CLITICS`) and *cannot* → *can not*, as UD EWT does. Lemmas come from the lexicon, which knows the reduced forms as EWT annotates them (counts in UD EWT + ESLSpok train, `en/data/forms.tsv`, 2026-10-17): *'s* as POS 550 tokens, as *be* 329, as *have* 15, as *us* in *let's* 13; *'d* as *would* 42 and *had* 11; *ca* (in *can't*) → *can* 78; *wo* (in *won't*) → *will* 48; *ai* (in *ain't*) → *be* 4. Spelling variants without the apostrophe are frequent in web text and also known: *nt* → *not* (75), *s* → *be* (73), *u* → *you* (7), *ur* → *your* (4). Eye-dialect forms like *gonna, wanna* are not split by the tokenizer itself ([01-consonants.md](01-consonants.md) §6).
- **Ambiguous *'s* and *'d*.** The choice between *is/has/us/genitive* for *'s* and *had/would* for *'d* is made by the tagger and parser from context (*he's gone* = has, *he's going* = is); the seed `en/seeds/morph/verb-clitics.md` states the rules.
- **Allomorphs of *-s*, *-ed*.** Not modelled, since spelling hides them.
- **Derivational alternations.** Not modelled: each derived word is its own lemma, and the lexicon does not link *divine* and *divinity*. The open CatVar database (`$MOVA_DATA/raw/en-catvar`, Open Software License 1.1) has categorial-variation families (*decide, decision, decisive, decisively*) and could supply those links ([17-derivation-suffixes.md](17-derivation-suffixes.md) §12).

## Sources for this chapter

- Hejná & Walkden 2022, §3.2.1 (/h/-dropping, *'em*), §3.2.2 (linking and intrusive /r/), §4.2.1 (shortening before the Great Vowel Shift). https://zenodo.org/records/6560337
- Jespersen 1942, MEG VI: ch. XI (vowel change), 16.1–16.8 (the *s*-ending), 24.1 (*-ed* adjectives). https://archive.org/details/modernenglishgra06jesp
- Kruisinga 1925, *English Sounds*, §184 (weak forms), §§264–265 (stress of verb + adverb; gradation list); text-1 pp. 105, 120, 138. https://archive.org/details/handbookofpresen0000ekru
- Reynolds 2026, §7.6.1 (pronunciations of *-(e)s* and *-ed*), §7.9 (connected speech). https://zenodo.org/records/20612113
- CatVar: Habash, Nizar & Bonnie Dorr. 2003. "A Categorial Variation Database for English." NAACL-HLT 2003. https://aclanthology.org/N03-1013/ ; data https://github.com/nizarhabash1/catvar
- Mova: `en/src/tok.rs`, `en/data/forms.tsv`, `en/seeds/morph/verb-clitics.md`, `en/seeds/errors/modal-do-to-base.md`.
