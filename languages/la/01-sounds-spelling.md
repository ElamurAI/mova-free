# 1. Sounds and spelling

This chapter describes the sounds of Latin, how they were written, how the writing changed from the early republic to the Middle Ages, and the three conventions in which Latin is read aloud today. Vowel length matters throughout the book, so it is marked in forms we write ourselves (*amō*, *amāre*, *rēs*); quoted texts are given exactly as the edition prints them, and none of the editions we hold prints macrons.

## Contents

1. The alphabet
2. Vowels and diphthongs
3. Consonants
4. Syllables, quantity and stress
5. Verse: how quantity is seen
6. Regular sound changes
7. Spelling in the stages of Latin
8. Three ways to read Latin aloud
9. How Mova handles it

---

## 1. The alphabet

The Roman alphabet of the republic had 21 letters (A B C D E F G H I K L M N O P Q R S T V X); Y and Z were added from Greek, for Greek words, around the middle of the first century BC, so that the classical alphabet has 23 letters (Bennett 1918 §1). There is no W. Three facts matter for reading editions:

- **I and V served as vowels and as consonants.** Roman writers had only the capitals I and V; *IVLIVS*, *VIVVS*. Modern editions differ in how they show the semivowels: some print *j* and *v* (*jam*, *vivus*), some print *i* and *u* only (*iam*, *uiuus*), some print *i* and *v*. The Latin Library files print *v* but not *j* in some books (*Italiam, Laviniaque*, Verg. *Aen.* 1.2) and *u* in others (*uestigia*, *Aen.* 3.659); the treebank UD_Latin-CIRCSE, whose text comes from the LASLA corpus, prints *i* and *u* throughout (*Iouem*, *uidua*, in the speech of Juno, *Her. F.* 1). A search or an analysis that does not first normalise *u/v* and *i/j* will see two words where there is one.
- **K and Q are survivals.** K is written only in a few words (*Kalendae*, *Karthago*); Q is written only before *u*, and *qu* stands for a single labiovelar [kʷ] (*quis*, *aqua*). *Gu* before a vowel likewise stood for [gʷ] after *n* (*lingua*, *sanguis*).
- **Capitals only, and no word division** in the earliest inscriptions; word division by a point (interpunct) was usual in inscriptions, and the lower-case letter forms and the spaces between words come from late antique and Carolingian script. Everything we read is a product of that second system.

The ligatures *æ* and *œ* belong to later printing and manuscript practice; the Clementine Vulgate as we hold it prints them (*cælum*, *tenebræ*, Gen 1:1–2), and a separate section below treats the spelling *e* for *ae*.

## 2. Vowels and diphthongs

### 2.1 Five vowels, long and short

Latin has five vowel qualities, **a e i o u** (and Greek *y* in loans), each short and long. The length is phonemic and carries the grammar: it separates forms of the same word and different words.

| short | long | |
|---|---|---|
| *venit* "he comes" | *vēnit* "he came" | present / perfect, 3 sg of *venīre* |
| *liber* "book" | *līber* "free" | noun / adjective |
| *mălum* "evil" | *mālum* "apple" | |
| *ŏs* "bone" | *ōs* "mouth" | the genitives differ too: *ossis*, *ōris* |
| *lĕvis* "light" | *lēvis* "smooth" | |
| *populus* "people" | *pōpulus* "poplar" | |
| *hŏc* (nom. neuter) | *hōc* (abl.) | |

In the nominative singular of the first declension *-a* is short, and in the ablative *-ā* is long: *rosă* against *rosā*. The distinction is therefore the difference between the subject and an adjunct of means or place, and it is invisible in the usual spelling. This is why the printed texts are often ambiguous in a way the spoken language was not, and why a parser cannot read the length off the page: it has to be recovered from the context or from a lexicon.

Length is also the base of metre (section 5) and, by position, of the accent (section 4).

### 2.2 Diphthongs

The classical diphthongs are **ae**, **au**, **oe**; **ei**, **eu** and **ui** are rare (*heu*, *cui*, *huic*, *ei*) (Bennett 1918 §2). Archaic Latin had a series of others: *ai* for *ae* (*aedīlis* once *aidīlis*), *oi* for *oe*, *ei* for *ī* (*deivos*, *"god"*). The old spellings survive in inscriptions; the Twelve Tables, quoted by later writers in modernised form (*Si in ius vocat, ito*, Latin Library *12tables*), keep the archaic grammar (the future imperative *ito*) but not the spelling.

Already in the republic, in the speech of Rome's countryside, *ae* was monophthongised to a long open *ē*, and *oe* to *ē* or *ū* (*oenus*, *ūnus*). By the late empire the monophthong was general, and medieval writers spelled it *e*, or *ę*, or kept *ae* by a learned tradition. The Tuscan charters of the 8th and 9th centuries show both: *ecclesie* and *ecclesiae* are written in the same charter (UD_Latin-LLCT, `la_llct` train-s649 *ecclesie*; train-s57 *ecclesiae*).

### 2.3 The collapse of the length system

Between the 2nd and the 5th centuries the length contrast was lost in the spoken language; it was replaced by differences of quality, and the vowels of the Romance languages descend from this later system (the "vowel merger" of Vulgar Latin: short *i* and long *ē* to a close *e*; short *u* and long *ō* to a close *o*). This is the reason why in Late and Medieval texts an *e* written for *i* or an *o* for *u* is not a slip: *pertenentes* for *pertinentes*, *abere* for *habēre* (train-s649), *onure* for *honōre* (train-s151); *potemus* in the same charter is a regularised *potēre* for *posse*, not a spelling slip. Chapters 14 and 15 use these spellings as evidence.

## 3. Consonants

The inventory of the classical language, in the reconstructed pronunciation (section 8):

| | labial | dental | velar | glottal |
|---|---|---|---|---|
| voiceless stops | p | t | c [k], qu [kʷ] | |
| voiced stops | b | d | g, gu [gʷ] | |
| fricatives | f | s | | h |
| nasals | m | n | n before a velar [ŋ] | |
| liquids | | l, r | | |
| semivowels | v [w] | | i [j] | |

Further points:

- **c** and **g** are always hard in the classical language: *Cicero* [ˈkikero], *gero*, *genus* [ˈgenus]. The soft values of Italian and of ecclesiastical Latin are medieval (section 8).
- **x** is [ks], **z** [dz] or [zz] and occurs only in Greek loanwords (*zōna*, *gaza*).
- **ph, th, ch** render Greek φ, θ, χ (aspirated stops, which Latin pronounced as *p, t, k* plus a puff) and occur only in loans: *philosophia*, *theātrum*, *chorus*. In popular speech they were *p t c*; later they became fricatives (*ph* > [f]).
- **h** was a weak glottal fricative in the classical period, and was dropped in the common speech; the hesitation is visible in spellings that add and omit it (*honor/onor*, *abere* above). It is silent in the Romance languages.
- **Gemination** is phonemic: *anus* "old woman" against *annus* "year", *caro* "flesh" against *carrus* "cart". A double letter is two consonants, and makes the preceding syllable heavy.
- **Final -m** was weak: it nasalised the vowel before it and was elided in verse before a vowel (*monstrum horrendum* scanned as *monstr' horrendum*). In the medieval charters it is often dropped or added by mistake, which is the reason for the confusion of accusative and ablative described in chapter 15.
- **-s** after a short vowel was also weak in early Latin; Ennius and Lucretius sometimes treat it as not making position (*omnibu' praestans*),; the grammars treat this as a feature of early verse.

## 4. Syllables, quantity and stress

### 4.1 Syllables

There are as many syllables as vowels and diphthongs (Bennett 1918 §4). A single consonant between vowels begins the next syllable (*pa-ter*, *vi-vit*); two or more consonants are shared, the first closing the previous syllable (*ar-ma*, *an-nus*), except that a stop plus *l* or *r* usually begins the next one (*pa-tris*, *vo-lu-cris*, but in verse sometimes *pat-ris*). *Qu* and *gu* count as one consonant.

### 4.2 Quantity

A syllable is **long** if it contains a long vowel or a diphthong (**long by nature**) or if its vowel is followed by two consonants or by *x* or *z* (**long by position**). Otherwise it is **short** (Bennett 1918 §5). Two things follow:

- A vowel that is short by nature can stand in a long syllable: *ār-ma* has a short *a* in a long syllable. Position does not lengthen the vowel, only the syllable, and modern editions rarely mark the difference.
- A stop plus a liquid (*tr, cr, pl, br*) usually does not make position in classical prose and in verse, but may in comedy and in the Augustan poets as a metrical licence.

Long syllables are heavy; the stress rule counts them, and the metre is built on them.

### 4.3 Stress

The classical accent was one of pitch and, increasingly, of stress. The rule (Bennett 1918 §6) is:

- A word of two syllables is accented on the first: *pá-ter*.
- A longer word is accented on the **penult** (the next-to-last syllable) if it is long, and otherwise on the **antepenult**: *Ca-ti-lī-na* (penult long: *Catilína*), *pa-ti-en-ti-a* (penult long by position: *patiéntia*).
- The final syllable is never accented, except in the rare words where a final syllable has been lost (*illīc* from *illīce*).
- Enclitics (*-que, -ne, -ve, -cum* in *mēcum*) pull the accent onto the last syllable of the word they follow: *Troiáque*, *tantóne* (Bennett 1918 §6).

Two examples from the same sentence (Cic. *Cat.* 1.1, *Quo usque tandem abutere, Catilina, patientia nostra?*): *Catilína* has a long penult (*-līna*), so it is stressed there; *abútere* has a short penult (*-te-*) and the accent falls on the antepenult *-bū-*, which is long by nature; *patiéntia* has a penult *-en-* long by position (*e* followed by *nt*), and is stressed there.

The rule is the basis of the later development: in Romance the stressed syllable survives and the unstressed ones weaken or vanish (*populum* > Italian *popolo*; *dómina* > *donna*; *ómnia* > *ogni* with the late stress shifts). Medieval Latin verse is built on the stress, not on the length (chapter 15).

## 5. Verse: how quantity is seen

Because the length of a syllable is not written, it is recovered from verse. Classical poetry is built on patterns of heavy and light syllables, and the most common line is the dactylic hexameter, six feet, each a dactyl (– ᴗ ᴗ) or a spondee (– –), the fifth almost always a dactyl and the sixth a spondee or a trochee.

> **Verg. *Aen.* 1.1** · Latin Library
> Arma virumque cano, Troiae qui primus ab oris
> *ārma vĭ | rūmquĕ că | nō Trō | iaē quī | prīmus ăb | ōrīs* — "I sing of arms and the man, who first from the shores of Troy …"

Reading the line gives: *ārma*, with short *a* in a long syllable by position (*rm*); *virumque*, whose final *-que* is short; *Troiae*, whose *Tro-* is long by position (the *i* is the semivowel [j] and doubles); the final syllable *-ris* is anceps. Elision removes a final vowel (or vowel plus *m*) before an initial vowel and is a good test: *monstrum horrendum informe ingens* (*Aen.* 3.658) is scanned with the *-um* in *monstrum* and *horrendum* elided. The scansion tells us the length of individual syllables of a word in a way no dictionary can, and the OLD, L&S and Whitaker's lists mark length for this reason.

## 6. Regular sound changes

These are the changes that explain most irregularities in the paradigms of chapters 2 and 5. They are given with Bennett's treatment (1918 §§7–8) and Roby's *Sounds* (Roby 1887, Part I).

| change | rule | examples |
|---|---|---|
| **Rhotacism** | an intervocalic *s* became *r* (about the 4th century BC) | *honōs* > *honor*; gen. *honōris* from *honōsis*; *gerō* beside *gestus*; *iūs* gen. *iūris*; *eram* beside *est* |
| **Vowel weakening** in non-initial syllables | *a* > *i* before one consonant, > *e* before two | *faciō* > *cōnficiō*; *capiō* > *accipiō*, *-ceptus*; *cadō* > *occidō*; *tangō* > *contingō*; *factum* > *perfectum*, *artifex* |
| **Assimilation** of prefixes | the final consonant of a prefix becomes like the next consonant | *ad* + *tendō* > *attendō*; *in* + *mobilis* > *immobilis*; *con* + *rumpō* > *corrumpō*; *sub* + *cēdō* > *succēdō* |
| **Voicing assimilation** | a voiced stop before *t, s* became voiceless | *rēgō* > *rēxī*, *rēctum* (*g* + *s* = *x*; *g* + *t* = *ct*); *scrībō* > *scrīpsī*, *scrīptum* |
| **Loss of a dental before -s** | a final *d, t* disappeared before *s* | *mīles* for *mīlet-s*; *pēs* for *ped-s* (gen. *pedis*) |
| **Lengthening before ns, nf** | a vowel before *ns* or *nf* was long, and the *n* later lost in speech | *cōnsul*, *īnferior*, *ēnsis* |
| **Final shortening** | long final syllables shortened in iambic words | *benĕ*, *malĕ* beside *bonē*; *egŏ*, *modŏ* |

## 7. Spelling in the stages of Latin

| period | features |
|---|---|
| **Archaic** (to c. 100 BC) | *-os, -om* for *-us, -um* (*filios*, *donom*); *quom* "when"; *oinos* "one"; *ei* for *ī*; double consonant not written; final *-d* in ablatives (*in altōd marī*); *ai, oi* diphthongs. Ennius is said to have introduced the doubling of consonants. |
| **Classical** (c. 100 BC–AD 150) | fixed by Cicero's generation and Augustan grammarians; *-us, -um*; *quom* gradually *cum*; after Sulla *x* for *cs*, and the apex mark over long vowels in inscriptions; *-ae, -ī* and so on. |
| **Imperial and Late** (to the 6th century) | *b* and *v* interchange (*vivere/bibere*, *bixit* for *vixit*); loss of *h* and of final *m*; *ae* spelled *e* (and conversely *ae* where *e* is correct: hypercorrection); *-i* for *-ē*; *k* and *ch* for *c* in some Christian writers. The Vulgate and Augustine's *Confessions* are edited with standard classical forms: scribal variants are not printed. |
| **Medieval** (6th–15th c.) | *e* for *ae* and *oe* (*ecclesie, celum, penitet*), *ti* for *ci* before a vowel (*tercius*/*tertius*, *gracia*/*gratia*), *y* for *i* (*ymago*), *michi, nichil* for *mihi, nihil* (the *h* was voiced as *k* in speech, hence spelling *ch*), *-ci-/-ti-* confusion, *mm* for *m* in Greek words, doubled and single consonants unstable, *set* for *sed*, *ydola*; the **Carolingian reform** (c. 800) re-established classical spelling in the learned tradition but not in the notarial one. |

The Tuscan charters we hold are not just "a corrupted classical". They show a stable local orthography of the 8th–9th centuries: *abere* (habere), *onure* (honōre), *ausiliante* (auxiliante), *eclesia* beside *ecclesia*, *visu fuit* with the infinitive construction, *Mihaeli* (for Michaeli) (train-s151). Their word forms are discussed in chapter 15.

## 8. Three ways to read Latin aloud

There are three living conventions, and the choice affects only the speaker, not the text.

1. **Restored classical pronunciation**, a reconstruction from the ancient grammarians, the inscriptions, comparative evidence and verse, taught widely since the late nineteenth century: *c, g* hard; *v* [w]; *ae* [ai]; *au* [au]; *h* [h]; *i* before a vowel [j]; *qu* [kʷ]; *ti* [ti]; long and short vowels distinguished; *Caesar* [ˈkaisar]; *Cicero* [ˈkikero]; *Gallia est omnis divisa in partes tres*, [ˈgalːia est ˈomnis dɪˈwiːsa in ˈpartes ˈtreːs].
2. **Ecclesiastical (Italianate) pronunciation**, the standard of the Roman liturgy and of church music: *c* and *g* soft before *e, i, ae, oe, y* ([tʃ], [dʒ]); *sc* before those vowels [ʃ]; *ti* before a vowel [tsi]; *gn* [ɲ]; *ae* and *oe* [e]; *v* [v]; *h* silent, but *mihi* and *nihil* [ˈmiki], [ˈnikil] by tradition; *z* [dz]; long vowels not distinguished, stress as in section 4.3; *Caesar* [ˈtʃezar]; *Cicero* [ˈtʃitʃero].
3. **National or "traditional" pronunciations** (English, French, German), produced by the local sound changes of each language; they survive in legal and scientific Latin.

The ecclesiastical convention is the one under which the Vulgate, the liturgy and medieval Latin are set to music, but it was not the Vulgate's own: Jerome's Latin was read with the late Roman sound system, in which *c* before *e, i* was already softening in some regions. For the analysis of the text the convention is irrelevant, but not so the evidence it leaves: the spellings *ti* for *ci*, *michi*, and *e* for *ae* in a Medieval text record how the scribe heard the word.

## 9. How Mova handles it

**Today (2026-10-22).** Mova has no Latin phonology. The only Latin component is the parser path that reads UD Latin: `en xpos-fill` followed by `en parse-eval`, trained on UD_Latin-LLCT (train) and scored on its dev set, gives LAS **87.35** with Mova's own tags and **87.80** with gold tags (`languages/PACKAGE.md`). The input to this path is the surface text as the treebank prints it, which has these properties:

- LLCT prints *u* and *i*; the Vulgate files print *v* (*creavit*) and ligatures; Perseus and the Latin Library print *v*. Normalisation of *u/v*, *i/j*, *æ/ae/e*, and of the medieval spellings of section 7 is **not done**; a model trained on LLCT sees the Tuscan spellings as ordinary forms.
- **Vowel length is lost** in every corpus we hold: LLCT, CIRCSE and the editions print no macrons. Mova therefore cannot, from the written word alone, tell *rosa* (nom.) from *rosā* (abl.) or *venit* from *vēnit*, and relies on the context, as a reader does.
- CIRCSE's text, from LASLA, is lower-cased and without punctuation in the `# text` line; the tokens carry the original punctuation as separate lines only where the corpus marks it.

**Where it stops.** No orthographic normalisation, no length restoration, no scansion, no pronunciation. A metrical pass over the poetry in CIRCSE (Seneca's tragedies) would give the first data on vowel length with a check (the line must scan), and a rule-based spelling normaliser for medieval Latin (section 7 as a table of rewrites, tested against the lemmas in LLCT) are the two natural first steps; both would need the operator's word.

**Sources for this chapter.** Bennett 1918 §§1–9, Roby 1887 (Part I), Allen & Greenough 1903 (Perseus, CC BY-SA, paraphrased), Gildersleeve & Lodge 1895, Hale & Buck 1903, Lane 1903; examples: Cic. *Cat.* 1.1 and Verg. *Aen.* 1.1 (Latin Library, public domain), Vulgate Gen 1:1–2 (Clementine), UD_Latin-LLCT (CC BY-SA 4.0). The reconstructed pronunciation follows the standard account of Allen's *Vox Latina*, which is not in our library (see `sources.md`, gaps).
