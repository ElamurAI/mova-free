# 14. Irregular verbs

An English verb is **irregular** when its past or past participle is not made by adding *-ed* (with the spelling rules of [06](06-spelling-rules.md)). The irregular verbs are a closed, slowly shrinking class of old, mostly native verbs; but they are the most frequent verbs of the language, so in running text irregular forms are common. This chapter gives their number and frequency (§1), their classes (§2), the verbs whose forms vary between varieties or meanings (§§3–5), the full table (§6), participle adjectives (§7) and how Mova handles them (§8).

## 1. How many, and how frequent

- **Number.** In AGID 2016.01.19 (Kevin Atkinson's inflection database, the source of Mova's paradigms), 339 of 8 940 verb entries have a first-listed past form that the regular *-ed* rules do not produce (counted 2026-10-17 over `infl.txt`; the regular test accepts *-ed*, *-d* after *e*, *y → ied* and consonant doubling). About 150 of these are simple verbs; the rest are prefixed or compound verbs that inherit the irregularity: *understand, overcome, undergo, mistake, outrun, rebuild, foresee, withdraw, babysit, proofread, broadcast, typeset*. A few AGID entries are artifacts (*gullet, inlet, missing* listed as verbs).
- **Hidden irregulars.** Some verbs look regular in spelling but are irregular in sound: *hear–heard* /hɪə–hɜːd/, *say–said* /seɪ–sed/, *flee–fled*, *lay–laid, pay–paid* (spelling *-aid*), *make–made, have–had*. The *-t* forms *learnt, burnt, spelt* are spelling-irregular too.
- **Frequency.** Irregular verbs dominate the top of the frequency list. Of the 40 most frequent past-tense forms in UD EWT + ESLSpok train (`en/data/forms.tsv`, counted 2026-10-17), 23 are irregular: *was* (1 244), *had* (403), *were* (393), *did* (292), *said* (209), *got* (116), *took* (91), *went* (82), *came* (81), *told* (77), *made* (63), *found* (54), *gave* (46), *thought* (41), *left* (38), *felt* (25), *saw* (25), *knew* (23), *brought* (20), *put* (20), *began* (17), *spent* (17), *sent* (28). The ten most frequent verb lemmas in EWT train (counting VB* and MD tags) are all irregular: *be* (7 621), *have* (2 607), *do* (1 466), *can* (712), *get* (623), *go* (606), *say* (456), *know* (428), *make* (403), *take* (394).
- **Direction of change.** The class loses members to the regular pattern (*help–holp → helped, climb–clomb → climbed, work–wrought → worked*), and regular pasts compete with irregular ones (*dreamed/dreamt, learned/learnt*); only a handful moved the other way (*dive–dove, sneak–snuck* in AmE, *dig–dug* replacing *digged*, *spit–spat*). New verbs are always regular (Sweet NEG I §1287), even when they look like an irregular one: *ping–pinged* (not \**pang*), *bing–binged*.

## 2. The classes

Old English had seven classes of **strong** verbs (vowel change: *singan–sang–sungen*) and **weak** verbs (dental suffix: *-de, -te*). After a thousand years of sound change and analogy these old classes no longer describe the forms; Jespersen MEG VI 4.1₁ says the old division "into three classes, strong, reduplicating, and weak, [has] no sense if we consider the flexions as found in actual usage", and proposes new classes by what happens to the present-day forms (MEG VI 4.3–5.6). This manual follows his eleven classes, with example verbs; Sweet NEG I §§1293–1472 groups them similarly into "consonantal" (with *t/d*), "invariable", "vocalic" and "mixed" verbs.

| class (Jespersen) | what happens | examples |
|---|---|---|
| 1 (4.3) | *-t* instead of *-d*, kernel unchanged | *burn–burnt, dwell–dwelt, learn–learnt, smell–smelt, spell–spelt, spill–spilt, spoil–spoilt* (all also regular, esp. AmE); *build–built, send–sent, spend–spent* belong to class 4 |
| 2 (4.4) | no change at all: stems in *-t, -d* | *bet, burst, cast, cost, cut, fit* (AmE), *hit, hurt, let, put, quit, rid, set, shed, shut, slit, split, spread, thrust, wet*; *broadcast, forecast* |
| 3 (4.5) | *-t* added, vowel shortened | *creep–crept, deal–dealt, dream–dreamt, feel–felt, keep–kept, kneel–knelt, lean–leant, leap–leapt, leave–left, mean–meant, sleep–slept, sweep–swept, weep–wept*; *lose–lost* |
| 4 (4.6) | *-t* replaces final *-d* | *bend–bent, build–built, lend–lent, rend–rent, send–sent, spend–spent* |
| 5 (4.7) | *-d* added with vowel change | *flee–fled, hear–heard, say–said, sell–sold, shoe–shod, tell–told*; *lay–laid, pay–paid* |
| 6 (4.8) | *-d* added, final consonant lost | *have–had, make–made* |
| 7 (4.9) | *-t* added after a further change of the stem: the *-ought/-aught* verbs | *beseech–besought, bring–brought, buy–bought, catch–caught, fight–fought, seek–sought, teach–taught, think–thought* |
| 8 (5.1) | no ending; one vowel change, past = participle | *bind–bound, find–found, grind–ground, wind–wound; cling–clung, dig–dug, fling–flung, hang–hung, sling–slung, slink–slunk, spin–spun, stick–stuck, sting–stung, strike–struck, string–strung, swing–swung, wring–wrung; bleed–bled, breed–bred, feed–fed, lead–led, meet–met, read–read, speed–sped; hold–held, behold; light–lit, shine–shone, shoot–shot, sit–sat, slide–slid, spit–spat, stand–stood, understand; win–won; get–got* (BrE) |
| 9 (5.2) | no ending; three vowels (*i–a–u* type) | *begin–began–begun, drink–drank–drunk, ring–rang–rung, shrink–shrank–shrunk, sing–sang–sung, sink–sank–sunk, spring–sprang–sprung, stink–stank–stunk, swim–swam–swum*; *run–ran–run, come–came–come, become* |
| 10 (5.3–5.4) | no ending in the past; participle in *-(e)n* | *arise, awake, bear, beat, bite, blow, break, choose, draw, drive, eat, fall, fly, forbid, forget, forgive, forsake, freeze, get* (AmE *gotten*), *give, grow, hide, know, lie (lay–lain), ride, rise, see, shake, slay, speak, steal, stride, strive, swear, take, tear, throw, tread, wake, wear, weave, write* |
| 11 (5.5–5.6) | mixed: regular past, *-n* participle | *hew–hewed–hewn, mow–mowed–mown, prove–proved–proven, saw–sawed–sawn, sew–sewed–sewn, show–showed–shown, sow–sowed–sown, strew–strewed–strewn, swell–swelled–swollen* |
| suppletive | forms from different roots | *be–was/were–been, go–went–gone*; *do–did–done* (reduplicated past) |

The modals (*can–could, may–might, shall–should, will–would, must, ought*) have past forms but no participles ([13](13-verbs.md) §5).

### 2.1 Patterns in the vowel changes

The most regular sub-patterns, which children overgeneralize (*brang* after *sang*) and which still attract a few verbs:

| pattern | examples |
|---|---|
| *i – a – u* | *sing–sang–sung, ring, drink, sink, swim, begin, spring, stink, shrink* |
| *i – u – u* | *cling–clung, sting, swing, string, wring, fling, sling, slink, stick, dig, spin, win* (*won*) |
| *ee/ea – e – e* + *-t* | *keep–kept, sleep, sweep, weep, feel, kneel, creep, mean, deal, dream, leap, leave* |
| *ee/ea – e – e* | *bleed–bled, breed, feed, lead, meet, read, speed, flee* |
| *i–e – o – i–en* | *drive–drove–driven, ride–rode–ridden, rise–rose–risen, write–wrote–written, stride, strive, smite* |
| *ea/ee – o – o–en* | *speak–spoke–spoken, steal–stole–stolen, break–broke–broken, freeze–froze–frozen, weave–wove–woven, choose–chose–chosen, wake–woke–woken* |
| *ear – ore – orn* | *bear–bore–borne, swear–swore–sworn, tear–tore–torn, wear–wore–worn* |
| *ow – ew – own* | *blow–blew–blown, grow, know, throw, fly–flew–flown, draw–drew–drawn* |
| *-ought/-aught* | class 7 |
| *-ind – -ound* | *bind, find, grind, wind* |

## 3. Variation between British and American English

| verb | BrE | AmE | note |
|---|---|---|---|
| *get* | *got – got* | *got – gotten* ("obtained, become, received") / *got* ("have, must": *I've got a car, I've got to go*) | Horwill 1935: *gotten* survives in England only in *forgotten, begotten*; [08](08-national-varieties.md) §2.3 |
| *burn, dream, dwell, kneel, lean, leap, learn, smell, spell, spill, spoil* | *-t* forms common (*learnt, dreamt, spelt*) | regular *-ed* usual | *burnt* adjective in both (*burnt toast*) |
| *dive* | *dived* | *dove* or *dived* | |
| *fit* | *fitted* | *fit* (past and participle) | *the suit fitted/fit him* |
| *sneak* | *sneaked* | *snuck* (also *sneaked*) | |
| *plead* | *pleaded* | *pled* (legal) or *pleaded* | |
| *prove* | *proved* (*proven* in Scots law, as adjective) | *proven* and *proved* | |
| *quit, wet* | *quitted, wetted* possible | *quit, wet* | |
| *spit* | *spat* | *spit* or *spat* | |
| *stink, shrink, spring, sink* | *stank, shrank, sprang, sank* | also *stunk, shrunk, sprung, sunk* as past | |
| *forbid* | *forbade* /fəˈbæd/ | *forbade* /-ˈbeɪd/, *forbid* | |
| *knit* | *knitted* | *knit* (figurative and literal) | |
| *bet* | *bet, betted* | *bet* | |

## 4. One spelling, two verbs (or two meanings)

| verb | meaning 1 → forms | meaning 2 → forms |
|---|---|---|
| *lie* | recline: *lie – lay – lain – lying* | tell untruths: *lie – lied – lied – lying* |
| *lay* | put down (transitive): *lay – laid – laid* | (the past of *lie* is also *lay*: a constant source of confusion, *I laid down* for *I lay down*) |
| *hang* | suspend: *hung* | execute: *hanged* (formal; *hung* informal) |
| *shine* | give light: *shone* | polish: *shined* |
| *wind* /waɪnd/ | turn, coil: *wound* /waʊnd/ | *wind* /wɪnd/ "make breathless": *winded* |
| *find* | *found* | *found* /faʊnd/ "establish": *founded* |
| *bear* | carry, endure: *borne* | give birth: *born* in the passive (*she was born*), *borne* in the active (*she has borne three children*) |
| *bid* | offer a price: *bid – bid* | command, say (literary): *bade – bidden* |
| *ring* | sound: *rang – rung* | encircle: *ringed* |
| *tear* /teə/ | rip: *tore – torn* | *tear* /tɪə/ "shed tears": *teared* |
| *fly* | *flew – flown* | baseball "hit a fly ball": *flied* |
| *speed* | *sped* | *speeded up* (accelerated) |
| *weave* | make cloth: *wove – woven* | move in and out: *weaved* |
| *cleave* | split: *cleft/clove – cleft/cloven* (also *cleaved*) | cling: *cleaved* |
| *spit* | expel saliva: *spat* | pierce with a spit: *spitted* |
| *lead* /liːd/ | guide: *led* | *lead* /led/ "cover with lead": *leaded* |
| *leave* | go away: *left* | produce leaves: *leaved* |
| *pay* | *paid* | nautical "slacken": *payed* |
| *slit* | cut: *slit* | narrow (the eyes): *slitted* |

Jespersen MEG VI 4.3–5.6 discusses these pairs verb by verb; AGID marks them with explanations in braces (*hung, hanged {execute}*).

## 5. Non-standard and older forms

- **Levelling of past and participle**, common in non-standard speech: *I seen it, I done it, he come yesterday, she has went, I have wrote*; Jespersen MEG VI 5.3 cites *he has just drove up to the door* as "now vulgar". UD EWT keeps such a form and tags it by its function, marking it `Typo=Yes` with the standard form in `CorrectForm` when the annotators judged it an error (e.g. *come* tagged VBZ with `CorrectForm=comes`; *give* as VBN with `Typo=Yes`), or `Style=Vrnc` for deliberate vernacular (15 tokens in EWT train).
- **Over-regularization** in learners and children: *goed, bringed, catched, teached, thinked, buyed*; Mova's error seed `en/seeds/errors/overregularized-verbs.md` describes how UD annotates them (the intended lemma and tag).
- **Archaic and biblical forms**: *spake, brake, bare, sware, gat, digged, holpen, wrought* ([09](09-early-modern-and-archaic.md) §3.4).
- **Dialect forms**: *drownded, attackted* (with an extra syllable), *brung, clumb, het* (heated), *knowed, growed, throwed* (regularized), *et* /et/ (BrE for *ate*, once standard in the south of England).

## 6. The table

The table below lists the simple irregular verbs and the most common prefixed ones, with their past and past participle as given by AGID 2016.01.19, the source of Mova's paradigms (generated from `infl.txt` on 2026-10-17 and checked by hand; less-preferred variants of AGID level 1 are marked †, level-2 variants (archaic, rare or doubtful) are left out, and AGID's notes on meaning are kept in brackets). Prefixed verbs inflect like their base (*mistake* like *take*, *overcome* like *come*, *undergo* like *go*) and are mostly not repeated.

| base | past | past participle |
|---|---|---|
| abide | abode, abided† | abode, abided† |
| arise | arose | arisen |
| awake | awoke, awaked† | awoken, awaked†, awoke† |
| bear | bore | borne, born† |
| beat | beat | beaten, beat† |
| become | became | become |
| befall | befell | befallen |
| beget | begot | begotten, begot† |
| begin | began | begun |
| behold | beheld | beheld |
| bend | bent | bent |
| bereave | bereaved, bereft† | bereaved, bereft† |
| beseech | besought, beseeched† | besought, beseeched† |
| bet | bet, betted† | bet, betted† |
| bid | bid, bade (command) | bidden, bid |
| bind | bound | bound |
| bite | bit | bitten |
| bleed | bled | bled |
| blow | blew | blown |
| break | broke | broken |
| breed | bred | bred |
| bring | brought | brought |
| broadcast | broadcast, broadcasted† | broadcast, broadcasted† |
| build | built | built |
| burn | burned, burnt† | burned, burnt† |
| burst | burst, bursted† | burst, bursted† |
| buy | bought | bought |
| cast | cast | cast |
| catch | caught | caught |
| choose | chose | chosen |
| cleave | cleaved (split), clove (split), cleaved (cling), cleft (split) | cleaved, cleft, cloven |
| cling | clung | clung |
| clothe | clothed, clad† | clothed, clad† |
| come | came | come |
| cost | cost | cost |
| creep | crept | crept |
| cut | cut | cut |
| deal | dealt | dealt |
| dig | dug | dug |
| dive | dived, dove† (AmE) | dived |
| do | did | done |
| draw | drew | drawn |
| dream | dreamed, dreamt† | dreamed, dreamt† |
| drink | drank | drunk, drank† |
| drive | drove | driven |
| dwell | dwelt, dwelled† | dwelt, dwelled† |
| eat | ate | eaten |
| fall | fell | fallen |
| feed | fed | fed |
| feel | felt | felt |
| fight | fought | fought |
| find | found | found |
| fit | fitted† (BrE), fit† (AmE) | fitted, fit |
| flee | fled | fled |
| fling | flung | flung |
| fly | flew, flied (baseball) | flown |
| forbid | forbade, forbad† | forbidden |
| forecast | forecast, forecasted† | forecast, forecasted† |
| forget | forgot | forgotten |
| forgive | forgave | forgiven |
| forsake | forsook | forsaken |
| freeze | froze | frozen |
| get | got | got (BrE); gotten (AmE), got |
| gild | gilded, gilt† | gilded, gilt† |
| gird | girded, girt† | girded, girt† |
| give | gave | given |
| go | went | gone |
| grind | ground | ground |
| grow | grew | grown |
| hang | hung, hanged (execute) | hung, hanged (execute) |
| have | had | had |
| hear | heard | heard |
| hew | hewed | hewn, hewed |
| hide | hid | hidden, hid† |
| hit | hit | hit |
| hold | held | held |
| hurt | hurt | hurt |
| keep | kept | kept |
| kneel | knelt, kneeled† | knelt, kneeled† |
| knit | knitted, knit† (AmE, figurative) | knitted, knit |
| know | knew | known |
| lay | laid | laid |
| lead | led, leaded (use lead) | led, leaded (use lead) |
| lean | leaned, leant† | leaned, leant† |
| leap | leaped, leapt, lept† | leaped, leapt, lept† |
| learn | learned, learnt† | learned, learnt† |
| leave | left, leaved (produce leaves) | left, leaved (produce leaves) |
| lend | lent | lent |
| let | let | let |
| lie | lay (recline); lied (tell lies) | lain (recline); lied (tell lies) |
| light | lighted, lit† | lighted, lit† |
| lose | lost | lost |
| make | made | made |
| mean | meant | meant |
| meet | met | met |
| mow | mowed | mowed, mown† |
| overcome | overcame | overcome |
| pay | paid, payed (slacken) | paid, payed (slacken) |
| plead | pleaded, pled† | pleaded, pled† |
| prove | proved | proved, proven† |
| put | put | put |
| quit | quit, quitted† | quit, quitted† |
| read | read† /red/ | read† /red/ |
| rend | rent | rent |
| rid | rid, ridded† | rid, ridded† |
| ride | rode | ridden |
| ring | rang† (sound), ringed† (encircle) | rung†, ringed |
| rise | rose | risen |
| run | ran | run |
| say | said | said |
| see | saw | seen |
| seek | sought | sought |
| sell | sold | sold |
| send | sent | sent |
| set | set | set |
| sew | sewed | sewn, sewed† |
| shake | shook | shaken |
| shear | sheared | sheared, shorn† |
| shed | shed | shed |
| shine | shone, shined (polish) | shone, shined (polish) |
| shoe | shod, shoed† | shod, shoed†, shodden† |
| shoot | shot | shot |
| show | showed | shown, showed† |
| shrink | shrank, shrunk† | shrunk, shrunken† |
| shut | shut | shut |
| sing | sang | sung |
| sink | sank, sunk† | sunk, sunken† |
| sit | sat | sat |
| slay | slew | slain |
| sleep | slept | slept |
| slide | slid | slid |
| sling | slung | slung |
| slink | slunk, slinked† | slunk, slinked† |
| slit | slit (cut), slitted (narrow) | slit (cut), slitted (narrow) |
| smell | smelled, smelt† | smelled, smelt† |
| smite | smote | smitten, smote†, smit† |
| sneak | sneaked, snuck† | sneaked, snuck† |
| sow | sowed | sown, sowed† |
| speak | spoke | spoken |
| speed | sped, speeded† | sped, speeded† |
| spell | spelled, spelt† | spelled, spelt† |
| spend | spent | spent |
| spill | spilled, spilt† | spilled, spilt† |
| spin | spun | spun |
| spit | spat, spit† (saliva, AmE), spitted† (pierce) | spat, spit†, spitted |
| split | split | split |
| spoil | spoiled, spoilt† | spoiled, spoilt† |
| spread | spread | spread |
| spring | sprang, sprung† | sprung |
| stand | stood | stood |
| steal | stole | stolen |
| stick | stuck | stuck |
| sting | stung | stung |
| stink | stank, stunk† | stunk |
| strew | strewed | strewn, strewed |
| stride | strode | stridden |
| strike | struck | struck, stricken† |
| string | strung | strung |
| strive | strove, strived† | striven, strived† |
| swear | swore | sworn |
| sweat | sweat, sweated† | sweat, sweated† |
| sweep | swept | swept |
| swell | swelled | swollen, swelled |
| swim | swam | swum |
| swing | swung | swung |
| take | took | taken |
| teach | taught | taught |
| tear | tore† (rip), teared† (cry) | torn†, teared |
| tell | told | told |
| think | thought | thought |
| throw | threw | thrown |
| thrust | thrust | thrust |
| tread | trod | trodden, trod† |
| undergo | underwent | undergone |
| understand | understood | understood |
| undertake | undertook | undertaken |
| wake | woke, waked | woken, waked, woke† |
| wear | wore | worn |
| weave | wove (fabric), weaved (zigzag) | woven |
| weep | wept | wept |
| wet | wet, wetted† | wet, wetted† |
| win | won | won |
| wind† /waɪnd/ | wound† (curve, wind up) | wound |
| withdraw | withdrew | withdrawn |
| withhold | withheld | withheld |
| work | worked, wrought† | worked, wrought† |
| wring | wrung | wrung |
| write | wrote | written, writ† |

## 7. Old participles as adjectives

Several verbs have an old *-en* participle that survives only as an adjective, beside a newer participle used in verb forms (Jespersen MEG VI 5.7, "participles in *-n*"):

| verb form | adjective | example |
|---|---|---|
| *drunk* | *drunken* | *a drunken sailor* (but *he was drunk*) |
| *sunk* | *sunken* | *sunken cheeks, a sunken ship* |
| *shrunk* | *shrunken* | *a shrunken head* |
| *struck* | *stricken* | *grief-stricken, panic-stricken* |
| *melted* | *molten* | *molten lava* |
| *proved* | *proven* | *a proven method* (also verbal in AmE) |
| *shaved* | *shaven* | *clean-shaven* |
| *swelled* | *swollen* | *a swollen ankle* (*swollen* is also the usual participle) |
| *rotted* | *rotten* | *rotten eggs* |
| *loaded* | *laden* | *heavily laden, debt-laden* |
| *engraved* | *graven* | *graven images* |
| *beholden, misshapen, unbeknown, ill-gotten, sodden* (from *seethe*), *forlorn* (from *forlese*), *wrought* (from *work*) | | fixed adjectives |
| *learned* /lɜːnd/ | *learned* /ˈlɜːnɪd/ | *a learned man* ([04](04-connected-speech.md) §6.2) |
| *bound* | *-bound* | *housebound, snowbound* (from *bind*); *bound for* "heading for" is another word |

## 8. How Mova handles irregular verbs

- **Lexicon.** All irregular forms are listed, not computed. `en/data/forms.tsv` holds AGID's full paradigm of every verb (sources `a` and `r` in its fifth column) and the UD counts of every form seen in EWT + ESLSpok train (source `u`), e.g. *went* VBD → *go* with 82 UD tokens; *geese* NNS → *goose* from AGID only. `en/build.rs` compiles it into static hash tables, so nothing is read at run time.
- **Lemmatization** (`Morph::lemmatize`, `en/src/morph.rs`): Early Modern forms first (`archaic`: *spake → speak, brake → break*), then the most frequent UD analysis of the form with its tag, then AGID. Homograph pasts and participles are decided by the tag: *read* VBD and VBN → *read*; *put, cut, hit, set* likewise; *left* VBD → *leave* but JJ → *left*; *found* VBD → *find* but VB → *found*; *lay* VBD → *lie* but VB/VBP → *lay*; *saw* VBD → *see* but NN → *saw* (seed `en/seeds/morph/verb-homograph-past.md`).
- **Inflection** (`Morph::inflect`): the UD-preferred form, then AGID's main form for the paradigm cell; so *go* + VBD → *went*, *sing* + VBN → *sung*. The unit test `dictionary` checks *went ↔ go* both ways and the case-preserving encoding (*Went* → lemma *go* with `Case::Title`).
- **Variants: a gap.** The lexicon takes from AGID only the **main form** of each paradigm cell (`en/README.md`: "UD / main form of an AGID cell / clean for reverse lookup / stem"); AGID's less-preferred variants (the † forms in §6) are absent unless UD EWT or ESLSpok happens to contain them. Checked on 2026-10-17 in `en/data/forms.tsv`: *learnt* (VBD, VBN), *gotten* (VBN, 7 tokens), *proven* (VBN, 4) and *knelt* are present; *dreamt, spelt, spilt, spoilt, leant, leapt* as verbs, *snuck*, and *dove* as a past (only the noun *dove* is there) are **not**. For these the lemmatizer falls through to the regular reverse rules, which have nothing for a final *-t*, so *dreamt*/VBD comes out with lemma *dreamt*. Generation uses the main forms: *dreamed, learned, burned, got*. Adding AGID's level-1 variants to the reverse-lookup table would close the gap.
- **Unknown verbs** fall back to the regular rules (`suffix`), which is right for new verbs (*googled*) and wrong only for unknown prefixed irregulars that AGID lacks (a rare case: AGID lists most of them, from *babysit* to *withstand*).
- **Seeds** describing the classes and their UD annotation: `en/seeds/morph/verb-strong-ablaut.md`, `verb-mixed-weak.md`, `verb-invariable.md`, `verb-homograph-past.md`, `verb-vbd-past.md`, `verb-vbn-participle.md`, `verb-participle-adjectives.md`; errors: `en/seeds/errors/overregularized-verbs.md`.

## Sources for this chapter

- Jespersen 1942, MEG VI chapters IV–V (4.1 introductory; 4.2 regular; 4.3–5.6 irregular verbs, classes 1–11; 5.7 participles in *-n*). https://archive.org/details/modernenglishgra06jesp ; library `books/j/je/jespersen-1914-modern-english-grammar-historical` (text-5)
- Sweet 1891, NEG I §§1286–1287 (regular verbs), §§1293–1476 (irregular verbs: consonantal, invariable, vocalic, mixed, isolated forms). https://archive.org/details/newenglishgramma01sweeuoft
- Horwill 1935, *A Dictionary of Modern American Usage* (*gotten, dove*). https://archive.org/details/dli.ernet.15683
- Hejná & Walkden 2022, §5.3.2 ("Strong verbs vs. irregular verbs"). https://zenodo.org/records/6560337
- Atkinson, Kevin. AGID 2016.01.19, `infl.txt` (permissive licence). http://wordlist.aspell.net/other/ ; raw copy `$MOVA_DATA/raw/en-gram-agid`; table and counts generated 2026-10-17.
- UD English EWT 2.18 train (CC BY-SA 4.0): frequency counts, 2026-10-17. https://universaldependencies.org/treebanks/en_ewt/index.html
- Mova: `en/src/morph.rs`, `en/data/forms.tsv`, `en/build.rs`, seeds listed in §8.
