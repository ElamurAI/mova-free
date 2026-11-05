# 10. Vocabulary: archaic words and false friends

Most of the EModE lexicon is still in use, which is why the period reads easily. The danger is in two kinds of
words:

- **archaic words** that have left the language (`peradventure`, `wherewith`, `kine`, `murrain`) — a parser
  meets them as unknown words and guesses their part of speech from their shape;
- **false friends** that are still in use with another meaning (`even` = evening, `corn` = grain, `suffer` =
  allow, `quick` = alive, `meat` = food, `let` = hinder). These are worse: the parser is sure of itself, the tree is
  fine, and the **world** built from it is wrong.

This chapter is a working lexicon for the biblical register and the tales: form, part of speech, EModE meaning,
an example with its id, and what Mova does now. Counts are from JPS 1917 (2026-10-17) unless marked.

## 10.1 Pro-adverbs of place, direction and reference

EModE (and Old English) build adverbs from `here`, `there`, `where` + a preposition and use `-ce`/`-ther`
forms for direction. They are closed-class and regular.

| Form | Meaning | JPS | Example | db | SLM now |
|---|---|---:|---|---|---|
| hither | to here | 6 | «they shall come back hither» (`jps:GEN:392`, GEN 15.16) | ADV 6 | ADV 9 / ADJ 9 (older rounds) |
| thither | to there | 24 | «let me escape thither» (`jps:GEN:499`, GEN 19.20) | ADV 24 | |
| whither | to where | 38 | «whence camest thou? and whither goest thou?» (`jps:GEN:405`, GEN 16.8) | ADV 35, SCONJ 1 | **ADJ 38** |
| hence | from here; therefore | | «seeing I go hence childless» (`jps:GEN:378`, GEN 15.2) | | |
| thence | from there | | «and from thence it was parted» (`jps:GEN:43`, GEN 2.10) | | ADV 54, NOUN 2 |
| whence | from where | 10 | «the ground from whence he was taken» (`jps:GEN:84`, GEN 3.23) | ADV 10 | |
| thereof | of it | 361 | «wherein is the seed thereof» (`jps:GEN:12`, GEN 1.11) | ADV 359 | **ADJ 361** |
| therein | in it | 64 | «multiply therein» (`jps:GEN:224`, GEN 9.7) | ADV 64 | |
| thereon | on it | 26 | «poured oil thereon» (`jps:GEN:1102`, GEN 35.14) | | ADJ (26 errors) |
| thereby | by it | 6 | | | |
| therewith | with it | 10 | | | |
| wherein | in which | 36 | «wherein is the seed» (`jps:GEN:12`) | | |
| whereof | of which | 10 | «the tree, whereof I commanded thee» (`jps:GEN:72`, GEN 3.11) | | |
| whereon | on which | 13 | «The land whereon thou liest» (`jps:GEN:839`, GEN 28.13) | | |
| wherewith | with which | 47 | «every living creature that creepeth, wherewith the waters swarmed» (`jps:GEN:22`, GEN 1.21) | ADV 48 | **NOUN 32**, ADV 17 |
| whereby | by which; how | 9 | «whereby shall I know that I shall inherit it?» (`jps:GEN:384`, GEN 15.8) | ADV 6, SCONJ 1 | |
| wherefore | why; therefore | 55 | «wherefore it is said» (`jps:GEN:255`, GEN 10.9) | ADV 52, SCONJ 1 | NOUN 49 / ADV 47 (older rounds) |
| hitherto | until now | 1 | «behold, hitherto thou hast not hearkened» (`jps:EXO:193`, EXO 7.16) | | |
| henceforth | from now on | 4 | «it shall not henceforth yield unto thee her strength» (`jps:GEN:101`, GEN 4.12) | | |
| thenceforth | from then on | 1 | | | |
| yonder | over there | 3 | «I and the lad will go yonder» (`jps:GEN:580`, GEN 22.5) | ADV 3 | |
| afar off | far away | 9 | «saw the place afar off» (`jps:GEN:579`, GEN 22.4) | | |
| round about | all around | 72 | «in all the border thereof round about» (`jps:GEN:617`, GEN 23.17) | | |
| abroad | outside, widely | 23 | «spread abroad» (`jps:GEN:264`, GEN 10.18) — not «in a foreign country» | | |
| nigh | near | 29 | «Draw not nigh hither» (`jps:EXO:57`, EXO 3.5) | ADV 25, ADJ 4, ADP 1 | |
| without | outside | | «told his two brethren without» (`jps:GEN:239`, GEN 9.22) — **false friend**: not «lacking» | | |

**UD.** All are ADV with `advmod`, except when they head a relative clause (`wherein`, `whereof`, `whereon`,
`wherewith`, `whereby` introduce relative clauses: in UD they are ADV, `advmod` or `obl` inside the relative
clause, and the clause is `acl:relcl` of the antecedent). `PronType=Rel` or `PronType=Int` may be added.

**Mova now.** `global/seeds/shortcuts/selection.md` defines a selection `archaic_adverbs` used by the reranker
(`world/src/rerank.rs`, repair `ArchaicAdverb`: «noun → ADV advmod») and by rule induction (`world/src/induce.rs`,
feature `F_ARCHAIC`): *whence whither thence hither thither hence yonder wherefore thereupon whereupon forthwith*.
The list misses the most frequent ones in the Torah: **thereof** (361), **therein** (64), **wherewith** (47),
**wherein** (36), **thereon** (26), **nigh** (29), **peradventure** (18), **whereon** (13), **howbeit** (12),
**whereof** (10), **therewith** (10), **whereby** (9), **thereby** (6), **henceforth** (4), **hitherto**,
**thenceforth**, **betimes**, **yesternight**, **afar**. Of 6 013 aligned Torah sentences, **473** have a UPOS
error on one of these adverbs (SLM now). Chapter 13, T2.

## 10.2 Discourse words and particles

| Form | Meaning | JPS | Example | UD |
|---|---|---:|---|---|
| behold | look!, here is | 237 | «Behold, I have given you every herb» (`jps:GEN:30`, GEN 1.29) | INTJ `discourse` (db 222); VERB when it has an object: «and beheld all the plain» |
| lo | look! | 24 | «and lo in her mouth an olive-leaf» (`jps:GEN:206`, GEN 8.11) | INTJ `discourse` (db 24) |
| yea | yes; indeed, even | 9 | «Yea, hath God said…?» (`jps:GEN:61`, GEN 3.1) | INTJ or ADV |
| nay | no; rather | 9 | «Nay, but Sarah thy wife shall bear thee a son» (`jps:GEN:432`, GEN 17.19) | INTJ |
| verily | truly | 5 | «Verily, the cry of Sodom and Gomorrah is great» (`jps:GEN:462`, GEN 18.20) | ADV `advmod` |
| peradventure | perhaps | 18 | «Peradventure there are fifty righteous within the city» (`jps:GEN:466`, GEN 18.24) | ADV (db 17, NOUN 1) |
| howbeit | however, nevertheless | 12 | «howbeit his younger brother shall be greater than he» (`jps:GEN:1595`, GEN 48.19) | ADV (db 11, CCONJ 1); lemma must be lower-case (db has `Howbeit` 8) |
| I pray thee | please | 52 | «Say, I pray thee, thou art my sister» (`jps:GEN:326`, GEN 12.13) | `parataxis` or `discourse` |
| amen | so be it | | Deuteronomy 27 | INTJ |
| O | vocative particle | | «O Lord GOD» (`jps:GEN:378`) | INTJ `discourse` on the vocative |
| oh | | | «oh, let me escape thither» (`jps:GEN:499`) | INTJ |
| even | namely; indeed (before an apposition) | ADV 224 | «at the hand of man, even at the hand of every man's brother» (`jps:GEN:222`, GEN 9.5) | ADV `advmod` |
| so | thus, then | | «And it was so» | ADV |
| now | (discourse) | | «Now the serpent was more subtle» (`jps:GEN:60`, GEN 3.1) | ADV `advmod` |
| betimes | early | 1 | «they rose up betimes in the morning» (`jps:GEN:767`, GEN 26.31) | ADV |
| still | continually, always | 9 | «going on still toward the South» (`jps:GEN:322`, GEN 12.9) — **false friend** | ADV |
| straitly | strictly, closely | 2 | «The man asked straitly concerning ourselves» (`jps:GEN:1406`, GEN 43.7) | ADV |

## 10.3 False friends

The meaning in the Torah differs from the modern one. The tree is right; the **frame** is wrong if the modern
meaning is used.

| Word | EModE / biblical meaning | Modern trap | Example | Note for the world layer |
|---|---|---|---|---|
| even (noun) | evening | the adverb *even* | «came to Sodom at even» (`jps:GEN:479`, GEN 19.1); `at even` 11 | db: NOUN 38 (right), SLM: ADV (38 errors). `at even` → time = evening |
| corn | grain (wheat, barley) | maize | «plenty of corn and wine» (`jps:GEN:803`, GEN 27.28); 32 | no maize in the ancient Near East |
| meat | food in general | flesh | «prepare the meat» (`jps:GEN:1416`, GEN 43.16; here it is flesh); KJV uses *meat offering* for a grain offering, JPS writes *meal-offering* (100) | in KJV texts *meat* = food |
| suffer | allow | feel pain | «didst not suffer me to kiss my sons» (`jps:GEN:965`, GEN 31.28); 6 | `suffer` + object + `to`-infinitive = permit |
| quick | alive, living | fast | «quick raw flesh» (`jps:LEV:320`, LEV 13.10): living flesh; «the quick and the dead» | |
| sore | grievous, severe; very | painful skin | «the famine was sore in the land» (`jps:GEN:323`, GEN 12.10); db ADJ 12, ADV 4 | intensity, not a wound |
| exceeding | very, exceedingly | going beyond | «thy reward shall be exceeding great» (`jps:GEN:377`, GEN 15.1) | ADV, lemma *exceeding* (db: 5) or *exceedingly*; not the verb *exceed* (db: 3 with lemma *exceed*) |
| cunning | skilful | sly | «Esau was a cunning hunter» (`jps:GEN:724`, GEN 25.27) | positive sense |
| subtle | crafty | delicate | «the serpent was more subtle than any beast» (`jps:GEN:60`, GEN 3.1) | |
| replenish | fill (a first time) | refill | «Be fruitful, and multiply, and replenish the earth» (`jps:GEN:29`, GEN 1.28) | |
| seed | offspring, descendants | plant seed | «thy seed shall be a stranger in a land that is not theirs» (`jps:GEN:389`, GEN 15.13); 93 | person-collective, animate; plant seed also occurs (`jps:GEN:12`) |
| stranger | resident alien, sojourner | unknown person | `jps:GEN:389`; 53 | a social status |
| know (a person) | have sexual relations with | be acquainted | «And Cain knew his wife; and she conceived» (`jps:GEN:106`, GEN 4.17) | the frame is a union, followed by conception |
| travail | labour pains; toil | travel | «I will greatly multiply thy pain and thy travail» (`jps:GEN:77`, GEN 3.16) | |
| ear | ear of grain | body part | «the barley was in the ear» (`jps:EXO:266`, EXO 9.31) | |
| goodly | fine, handsome, valuable | fairly large | «all goodly things of his master's» (`jps:GEN:630`, GEN 24.10) | |
| froward | perverse, contrary | forward | «a very froward generation» (`jps:DEU:905`, DEU 32.20) | |
| wont | accustomed | won't | «if the ox was wont to gore» (`jps:EXO:608`, EXO 21.29) | ADJ + `to`-infinitive; never the contraction *won't* (tokenizer!) |
| want | lack, need | desire | «according to the want of their little ones» (`jps:GEN:1552`, GEN 47.12) | noun = need |
| issue | offspring; discharge | problem | «thy issue, that thou begettest after them» (`jps:GEN:1581`, GEN 48.6) | |
| base | (here) pedestal; elsewhere *low, vile* | military base | «even its base, and its shaft» (`jps:EXO:731`, EXO 25.31) | |
| jealous | zealous, demanding exclusive devotion | envious | «I the LORD thy God am a jealous God» (`jps:EXO:558`, EXO 20.5) | |
| abroad | outdoors, far and wide | in another country | `jps:GEN:264` | |
| without | outside | lacking | `jps:GEN:239` | ADV, not ADP |
| countenance | face, expression | approval | «his countenance fell» (`jps:GEN:93`, GEN 4.5) | |
| wroth | angry | (wrath is the noun) | «Cain was very wroth» (`jps:GEN:93`) | ADJ |
| expired | died | ran out | «And Abraham expired, and died» (`jps:GEN:704`, GEN 25.8) | death event |
| spoil | plunder, booty | ruin | «at even he divideth the spoil» (`jps:GEN:1626`, GEN 49.27) | noun |
| lad | boy, young servant | | «because of the lad» (`jps:GEN:553`, GEN 21.12) | |
| damsel | young woman | | `jps:GEN:634` | |
| matter | affair, thing said | physical substance | «concerning this matter» (`jps:GEN:629`, GEN 24.9) | |
| soul | person, living being; life; self | spirit only | «wherein there is a living soul» (`jps:GEN:31`, GEN 1.30); «that soul shall be cut off» | often = *person* (count noun) |
| flesh | body, meat, mankind | | «closed up the place with flesh» (`jps:GEN:55`, GEN 2.21); «all flesh» = all living beings | |
| fowl | birds (collective) | poultry | «let fowl fly above the earth» (`jps:GEN:21`, GEN 1.20) | plural in meaning |
| cattle | livestock, domestic animals | cows | `jps:GEN:25` (GEN 1.24) | |
| creeping thing | small animals (reptiles, insects) | | `jps:GEN:25` | |
| the deep | the primeval ocean | depth | «upon the face of the deep» (`jps:GEN:1`, GEN 1.2) | |
| firmament | the sky as a solid vault | | `jps:GEN:6` (GEN 1.6) | |
| kind | species | nice | «after its kind» (`jps:GEN:12`) | |
| let (verb) | allow; (KJV, Shakespeare) **hinder** | allow | «Let there be light» (allow/jussive); «I'll make a ghost of him that lets me» (*Hamlet* I.4) = hinders | JPS uses only the modern sense |
| prevent | go before, anticipate | stop | KJV Ps 119:147 «I prevented the dawning of the morning» | KJV only |
| conversation | way of life, conduct | talk | KJV Phil 1:27 | KJV only |
| nephew | grandson, descendant | brother's son | KJV Judg 12:14 | KJV only |
| charity | love | giving to the poor | KJV 1 Cor 13 | KJV only |
| silly | simple, innocent | foolish | | Shakespeare, tales |
| naughty | worthless, wicked | mischievous | KJV Prov 6:12 «a naughty person» | |
| presently | immediately | soon / now | | Shakespeare, tales |
| anon | at once, soon | | «have I not told you that anon fear fell upon them?» (`book:pg16:650`, *Peter Pan*) | |
| fond | foolish | loving | «a fond old man» (*King Lear* IV.7) | |
| nice | precise, fastidious | pleasant | | |
| artillery | weapons, bow and arrows | big guns | KJV 1 Sam 20:40 | |
| carriage | baggage, what is carried | vehicle | KJV Acts 21:15 | |

Sources for the KJV-only false friends: W. A. Wright, *The Bible Word-Book* (2nd ed. 1884, public domain), which
lists such words with their 1611 senses (listed under «wanted» in `sources.md`: not yet in our library); Hejná
& Walkden 2022 ch. 7 on semantic change; the examples themselves are verified in the texts cited.

## 10.4 Archaic nouns, adjectives and verbs (no modern homograph)

| Word | Meaning | JPS | Example | UPOS, lemma |
|---|---|---:|---|---|
| kine | cows | 17 | «forty kine and ten bulls» (`jps:GEN:1015`, GEN 32.15) | NOUN, lemma *cow* (db: *kine*) |
| brethren | brothers, kinsmen | 123 | «told his two brethren without» (`jps:GEN:239`, GEN 9.22) | NOUN, lemma *brother* (db: *brethren* 120) |
| raiment | clothing | 14 | «and raiment, and gave them to Rebekah» (`jps:GEN:678`, GEN 24.53) | NOUN |
| victuals | food supplies | 3 (+ *victual* 2) | `jps:GEN:363` (GEN 14.11) | NOUN |
| provender | fodder | 4 | «straw and provender enough» (`jps:GEN:646`, GEN 24.25) | NOUN |
| pottage | thick soup | 3 | «And Jacob sod pottage» (`jps:GEN:726`, GEN 25.29) | NOUN |
| murrain | cattle plague | 1 | «a very grievous murrain» (`jps:EXO:237`, EXO 9.3) | NOUN |
| emerods | haemorrhoids, tumours | 1 | `jps:DEU:764` (DEU 28.27) | NOUN |
| firstling | first-born of an animal | 13 | «every firstling that is a male» (`jps:EXO:374`, EXO 13.12) | NOUN (db: 11, ADJ 2) |
| morrow | the next day | 22 (+ `to-morrow` 9) | «on the morrow» (`jps:GEN:514`, GEN 19.34) | NOUN |
| yesternight | last night | 3 | «I lay yesternight with my father» (`jps:GEN:514`) | ADV |
| bondwoman, bondman, handmaid | slave woman, slave, maidservant | | `jps:GEN:551`, `jps:GEN:1448`, `jps:GEN:398` | NOUN |
| kinsman | relative (the redeemer) | 3 | `jps:LEV:758` | NOUN |
| harlot | prostitute | 12 | `jps:GEN:1087` (GEN 34.31) | NOUN |
| tabernacle | the portable sanctuary | 91 | `jps:EXO:709` (EXO 25.9) | NOUN |
| loins | hips, as the seat of procreation | 12 | «kings shall come out of thy loins» (`jps:GEN:1099`, GEN 35.11) | NOUN |
| bowels | inward parts, womb, seat of feeling | 3 | «out of thine own bowels» (`jps:GEN:380`, GEN 15.4) | NOUN |
| lovingkindness | steadfast love (*ḥesed*) | 2 | «plenteous in lovingkindness» (`jps:NUM:540`, NUM 14.18) | NOUN |
| countenance | face | 6 | `jps:GEN:93` | NOUN |
| aught / ought | anything | 16 | «nor aught that is thine» (`jps:GEN:375`) | PRON (db: NOUN 12, PRON 3) |
| nought | nothing; for nothing | 8 | «serve me for nought» (`jps:GEN:866`) | PRON (db: NOUN 5, PRON 2) |
| goodly | fine | 10 | `jps:GEN:630` | ADJ |
| froward | perverse | 1 | `jps:DEU:905` | ADJ |
| wroth | angry | | `jps:GEN:93` | ADJ |
| fain | gladly; obliged | 0 (tales: «would fain eat», `book:pg128:1765`; «the Robber was fain to fly», `book:pg11339:976`) | | ADV (*would fain*) or ADJ (*was fain to*) |
| sojourn | dwell as an alien | 14 | «went down into Egypt to sojourn there» (`jps:GEN:323`) | VERB |
| hearken | listen, obey | 57 | «hearken unto my speech» (`jps:GEN:112`, GEN 4.23) | VERB; `unto` + object = `obl` |
| smite | strike, kill | 32 | «lest any finding him should smite him» (`jps:GEN:104`, GEN 4.15) | VERB |
| seethe / sod / sodden | boil | 1 / 4 | `jps:GEN:726`, `jps:EXO:319` | VERB, lemma *seethe* |
| mete | measure | 1 | «when they did mete it with an omer» (`jps:EXO:465`, EXO 16.18) | VERB |
| lade | load | 1 | «lade your beasts» (`jps:GEN:1491`, GEN 45.17) | VERB |
| requite | repay | 2 | «will fully requite us all the evil» (`jps:GEN:1650`, GEN 50.15) | VERB |
| chastise | punish | 3 | `jps:LEV:806` | VERB |
| covet | desire wrongfully | 4 | «Thou shalt not covet thy neighbour's house» (`jps:EXO:570`, EXO 20.17) | VERB |
| hallow | make holy | 9 | `jps:EXO:836` (EXO 28.38) | VERB |
| entreat | plead | 7 | «entreat for me to Ephron» (`jps:GEN:608`, GEN 23.8) | VERB |
| heed | attention | 17 | «Take heed to thyself» (`jps:GEN:960`, GEN 31.24) | NOUN in *take heed* |
| wot, wist | know, knew | 0 (KJV 5, 4; tales 2, 2) | KJV Gen 21:26 «I wot not who hath done this thing» — JPS: «I know not» (`jps:GEN:567`) | VERB, lemma *wit* |
| quoth | said | 0 (tales 71) | «quoth the old woman» (`book:pg14241:304`) | VERB, always finite past; `world/src/rerank.rs` already treats it as finite |
| methinks | it seems to me | 0 (tales 8) | «methinks he might be master with you» (`book:pg14241:551`) | VERB (impersonal *think* = seem) + `me`; or ADV as a unit |
| prithee | please | 0 | | INTJ |
| ere | before | 2 (tales 38) | | ADP / SCONJ |
| whilst | while | (tales 217) | | SCONJ |
| betwixt | between | (tales) | «betwixt and between» (`book:pg236:2310`) | ADP |

## 10.5 Measures, ritual and law words

These are not archaic in the sense of the period — they are technical — but they are the words a modern parser
does not know as nouns: *shekel* 33, *cubit* 20, *ephah* 18, *hin* 16, *omer* 3, *tithe* 13, *wave-offering* 15,
*heave-offering*, *sin-offering* 99, *burnt-offering* 148, *meal-offering* 100, *peace-offerings* 54,
*first-fruits* 14, *atonement* 79, *unclean* 142, *abomination* 18, *statute* 30, *ordinance* 19, *testimony* 35.
The SLM gets `hin` (a liquid measure, 16) right as a NOUN, which shows that a word unknown to modern English is not the problem when its shape is ordinary; the Judaism dictionary of the world layer
(made by the dictionary builder, a closed tool) is the place to give these words their records.

## 10.6 What to put in the lexicon

1. **Closed class, fixed analysis** (no context needed): the 30-odd pro-adverbs of 10.1, the discourse words of
   10.2, the pronouns of chapter 3 and the auxiliaries of chapter 5. A table in front of the tagger.
2. **Open class, lemma only**: the old plurals and strong verbs (`kine` → *cow*, `brethren` → *brother*, `sod` →
   *seethe*). A table in the lemmatizer, applied with the right tag.
3. **False friends**: nothing in the parser. They need a **register-dependent sense** in the world dictionary:
   when the text is in the biblical register (the `Style=Arch` density of the document, or its source), `corn`
   means grain, `even` after `at` means evening, `suffer` + object + infinitive means allow, `know` + a person as
   object + conception in the next clause means a union. `world/src/domains.rs` already has a «formal» domain
   that includes senses labelled archaic or literary; the false friends above are a seed list for it.

## 10.7 The dictionary layer (`dictionary emode`)

The lists above are the seed of a dictionary layer for the pupil: the dictionary builder (closed) finds every
archaic word, form and sense of the archaic corpora (the JPS Torah, the 0.3 tales, the English Sefer Yetzirah)
and writes `$MOVA_DATA/school/dictionaries/emode.tsv` (pages in `emode/` by `dictionary schools`). Each
record: the form, its kind (archaic form / word / sense), the modern equivalent, part of speech, the meaning a
pupil needs, how it differs from today, up to three corpus sentences with their source, and the tests that took
it in, with their numbers. The base dictionary is not touched.

    dictionary emode requests <work> --titles <tales books.tsv> torah.tsv pool.tsv sy.tsv
    annot batch-run <work>                       # the teacher, in batches
    dictionary emode post <work>                 # → emode-senses.tsv
    dictionary schools <out>                     # → emode.tsv and the pages

The tests (2026-10-18; 140 555 sentences, 3 055 469 tokens, 29 548 forms; EWT 173 060 tokens):

| Test | Rule | Records |
|---|---|---:|
| manual | the lists of chapters 3, 5, 6, 9, 10, 11 (seen in the corpora) | 201 |
| ending | `-eth`/`-est`/`-st`, never in EWT, the lexicon knows it as nothing but a verb, the stem a verb of EWT; `-est` also once with *thou* | 191 |
| rate: word | ≥ 3 uses, never in EWT (or ≥ 10× per token), Wiktionary's first sense archaic | 63 |
| rate: sense | ≥ 3 uses in the Torah, ≥ 10× EWT there, an archaic sense in Wiktionary | 55 |

The rate test only proposes: Wiktionary's first sense is often just the oldest (`horrid` = bristling), and an
archaic sense somewhere says nothing of our sentences; a first version over all the corpora took 52 words, 47 of
them tales' topics (*princess*, *palace*, *tree*). The teacher therefore also answers «old here: yes/no», and a
rate word stays only on *yes*: 97 of 118 stayed. The ending forms get no teacher: their record is built from the
stem's record. The teacher wrote the senses of 319 words (8 requests, one retried with more tokens: 78 856 input
and 87 819 output tokens). Result: **489 records** (251 archaic forms, 141 archaic words, 97
archaic senses). Two runs give the same files, byte for byte.

Limits: The teacher is lenient on the rate words (it kept *basket*, *voice*, *tent* for narrow old uses); a
manual false friend stays even when the corpora use only today's sense (*let*, *prevent*: the record says so,
though for *prevent* The teacher wrongly claims the old sense); the OCR'd Westcott 1893 brings some library
boilerplate into the examples. Wiktionary's labels (CC BY-SA) are used only as a test; nothing of their text is
carried.
