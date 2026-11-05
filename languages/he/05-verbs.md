# 05. The verb: system, binyanim, tense and use

This chapter describes the Modern Hebrew verb as a system: how a verb is built from a consonantal root and a pattern (*binyan*), what the seven binyanim mean and how they pair up in valency alternations, how new verbs are made from loanwords, the three tenses and the participle, the imperative and the forms speakers use in its place, the infinitive and the verbal noun, the copula *haya*, and the modal words. Full paradigm tables for a strong root in every binyan and the key cells of every weak-root class are in the companion file [05b-verb-paradigms.md](05b-verb-paradigms.md). The manual's index is [README.md](README.md); nouns, adjectives and pronouns (including the participle as a noun class and the possessive suffixes) are in [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md); inflected prepositions and את in [06-prepositions-particles.md](06-prepositions-particles.md); clause-level syntax (copula, negation, existentials, word order) in [07-syntax.md](07-syntax.md). Transliteration follows chapter 02: broad Israeli pronunciation, *kh* = /χ/, *ts* = צ, *sh* = ש, *'* = a glottal stop where it is actually pronounced, stress marked with an acute accent only when it is not on the last syllable.

Glosses use Leipzig abbreviations: M, F, SG, PL, 1/2/3 (person), PST (past), PRS (present participle used as a verb), FUT, IMP, INF, VN (verbal noun), PTCP, PASS, DEF, ACC (the object marker את), CS (construct state), NEG, COP.

---

## 1. Root and pattern

### 1.1 The idea

A Hebrew verb is the interleaving of two pieces:

- a **root** (*shóresh*): usually three consonants, sometimes four, that carry a lexical field. The root כ-ת-ב (k-t-v) is about writing; ל-ב-ש (l-v-sh) about clothing; ג-ד-ל (g-d-l) about size and growth.
- a **pattern**: a template of vowels, sometimes with a prefix and with gemination (historical doubling) of a root consonant, that carries a broad meaning (active, passive, causative, reflexive) and fixes the conjugation class. The verbal patterns are called **binyanim** (sing. *binyan*, "building").

Every Hebrew verb belongs to exactly one binyan, and Hebrew has only seven of them for verbs, against roughly a hundred patterns (*mishkalim*) for nouns (Ornan 2014). This is why a new verb, even one coined from an English noun, must be poured into one of the seven moulds, while a new noun can simply be borrowed as it is (§ 4).

The citation form of a verb in Hebrew dictionaries and grammars is the **past tense, 3rd person masculine singular** (*katav* "he wrote"), because it is the shortest form and shows the root plainly. In English-language teaching materials the infinitive (*likhtov* "to write") is also common. UD Hebrew treebanks lemmatise to the 3MS past form (the lemma of תגידו "say! (PL)" is הגיד), see chapter 11.

### 1.2 One root through all seven binyanim

The root כ-ת-ב happens to have a living verb in every binyan, which makes it a good first illustration:

| Binyan | Past 3MS | Transliteration | Meaning | Relation |
|---|---|---|---|---|
| pa'al (qal) | כָּתַב | *katav* | wrote | basic, active |
| nif'al | נִכְתַּב | *nikhtav* | was written | passive / middle of pa'al |
| pi'el | כִּתֵּב | *kitev* | sent a copy, cc'd (e-mail) | new denominal verb from the noun *kitúv* "cc" |
| pu'al | כֻּתַּב | *kutav* | was cc'd | passive of pi'el |
| hif'il | הִכְתִּיב | *hikhtiv* | dictated; imposed | causative ("made write") |
| huf'al | הֻכְתַּב | *hukhtav* | was dictated | passive of hif'il |
| hitpa'el | הִתְכַּתֵּב | *hitkatev* | corresponded (with each other) | reciprocal |

The same root also yields nouns in noun patterns: מִכְתָּב *mikhtav* "letter", כְּתֹבֶת *któvet* "address", כְּתִיבָה *ktiva* "writing" (the verbal noun of pa'al), כַּתָּב *katav* "reporter", כְּתָב *ktav* "script, handwriting". The root is an abstraction: it has no pronunciation of its own, and speakers know it from the family of words that share it.

A second family, ג-ד-ל "big, grow":

| Binyan | Past 3MS | Translit. | Meaning |
|---|---|---|---|
| pa'al | גָּדַל | *gadal* | grew (intransitive) |
| nif'al | — | — | (not in use) |
| pi'el | גִּדֵּל | *gidel* | raised (a child), grew (plants) |
| pu'al | גֻּדַּל | *gudal* | was raised |
| hif'il | הִגְדִּיל | *higdil* | enlarged |
| huf'al | הֻגְדַּל | *hugdal* | was enlarged |
| hitpa'el | הִתְגַּדֵּל | *hitgadel* | magnified oneself, boasted (literary) |

Most roots fill only two to four cells. The pattern of empty and filled cells is lexical: speakers cannot predict that ג-ד-ל has no nif'al, but they can coin one, and the "empty squares" of this root-by-pattern array are exactly where neologisms are made (Ornan 2014; Yannai 1983).

### 1.3 Binyanim in a corpus

The seven binyanim are far from equal in use. Counting the `HebBinyan` feature on VERB and AUX words in the two IAHLT UD treebanks (UD 2.18, all splits; counted for this manual on 2026-10-17):

| Binyan | IAHLTwiki (Wikipedia) | % | IAHLTknesset (parliament, spoken) | % |
|---|---:|---:|---:|---:|
| PAAL | 3 924 | 33.8 | 3 571 | 41.7 |
| HIFIL | 1 853 | 16.0 | 1 332 | 15.5 |
| PIEL | 1 641 | 14.1 | 1 125 | 13.1 |
| NIFAL | 1 538 | 13.3 | 469 | 5.5 |
| HITPAEL | 717 | 6.2 | 433 | 5.1 |
| HUFAL | 600 | 5.2 | 76 | 0.9 |
| PUAL | 563 | 4.9 | 128 | 1.5 |
| NITPAEL | 17 | 0.1 | 4 | 0.05 |
| no binyan (יש, אין, צריך, יכול, אפשר, חייב …) | 753 | 6.5 | 942 | 11.0 |

The written encyclopaedic register uses the passive binyanim (nif'al, pu'al, huf'al) more than twice as often as transcribed parliamentary speech: 23.4 % of verbs against 7.9 %. This matches the general description that pu'al and huf'al finite forms are rare in speech, and that their participles survive mainly as adjectives (Glinert 2005; Wikipedia, "Modern Hebrew grammar"). The words with no binyan feature are the existentials and the modal adjectives that UD tags as VERB or AUX (§ 9).

---

## 2. The seven binyanim

The traditional names come from the paradigm root פ-ע-ל (p-'-l) "do": *pa'al*, *nif'al*, *pi'el*, *pu'al*, *hif'il*, *huf'al* (also *hof'al*), *hitpa'el*. Pa'al is also called *qal* "light". Hebrew grammars arrange them in three active–passive pairs plus hitpa'el:

| | Active | Passive |
|---|---|---|
| simple | pa'al | nif'al |
| intensive (with gemination) | pi'el | pu'al |
| causative (with prefix ה־) | hif'il | huf'al |
| reflexive / reciprocal | hitpa'el | (hitpa'el itself; formal nitpa'el) |

This table is a useful map but not a rule of meaning. The meanings below are **tendencies**: each binyan hosts many verbs whose sense cannot be derived from the root and the pattern. Modern work treats a binyan as a morphological spell-out of syntactic heads (voice, causative, middle) interacting with the lexical semantics of the root, rather than as a fixed meaning (Kastner 2017; Doron 2003).

### 2.1 Pa'al (qal), פָּעַל

**Form.** Past *CaCaC* (*katav*), present *CoCeC* (*kotev*), future *yiCCoC* (*yikhtov*) or *yiCCaC* (*yilmad*), infinitive *liCCoC* (*likhtov*, *lilmod*).

**Meaning.** The basic, most frequent binyan: simple active verbs, transitive (כתב *katav* "wrote", אכל *akhal* "ate", שבר *shavar* "broke") and intransitive (הלך *halakh* "went", ישן *yashan* "slept", גדל *gadal* "grew"). It hosts most of the oldest, most basic verbs of the language and most irregular ones. It is **closed to new coinage**: almost no new verbs enter pa'al, because the pattern does not leave room for four-consonant stems or clusters (Bolozky 1999).

Pa'al also has a **passive participle** *CaCuC* (כָּתוּב *katuv* "written", סָגוּר *sagur* "closed", אָהוּב *ahuv* "beloved"), used as an adjective or a resultative (§ 7.4).

**Stative pa'al.** A handful of old stative verbs have an *e* in the past (*CaCeC*): יָשֵׁן *yashen* (adj. "asleep"), זָקֵן *zaken* "old"; in Modern Hebrew these are mainly adjectives, and the verb past is levelled to *CaCaC* (*yashan* "slept").

### 2.2 Nif'al, נִפְעַל

**Form.** Past *niCCaC* (*nikhtav*), present *niCCaC* (identical in 3MS: *nikhtav* "is written"), future *yiCaCeC* (*yikatev*), infinitive *lehiCaCeC* (*lehikatev*). The future and infinitive show the *n* of the prefix assimilated into the first root consonant (historically geminated).

**Meanings.**

1. **Passive of pa'al**, mainly in written language:

```
המכתב נכתב בשנת 1948.
ha-mikhtav nikhtav bi-shnat 1948
DEF-letter write.PASS.PST.3MS in-year.CS 1948
'The letter was written in 1948.'
```

2. **Anticausative / inchoative** (change of state with no agent):

```
הכוס נשברה.
ha-kos nishbera
DEF-glass break.NIF.PST.3FS
'The glass broke.'
```

against pa'al transitive *shavar et ha-kos* "broke the glass".

3. **Middle and lexical intransitives** with no active partner: נכנס *nikhnas* "entered", נרדם *nirdam* "fell asleep", נשאר *nish'ar* "stayed", נזכר *nizkar* "remembered (suddenly)", נלחם *nilkham* "fought", נהנה *nehene* "enjoyed".

Nif'al is the binyan most associated with the passive in writing, but Kastner (2017) and Doron (2003) show that its core is not "passive" as such: it marks the absence of an external argument in the syntax, so passive and anticausative readings fall out of the same structure depending on the root.

### 2.3 Pi'el, פִּעֵל

**Form.** Past *CiCeC* (*diber*, *sider*), present *meCaCeC* (*medaber*), future *yeCaCeC* (*yedaber*), infinitive *leCaCeC* (*ledaber*). Historically the middle consonant was doubled (*dibber*); in Modern Hebrew gemination is lost, but it still blocks the spirantization of ב, כ, פ in the middle slot: *siper* "told" (ספר, middle פ is [p]) beside pa'al *safar* "counted" (middle [f]) (see chapter 03).

**Meanings.**

1. **Intensive or iterative** of a pa'al verb: שבר *shavar* "broke" → שיבר *shiber* "smashed to pieces"; קפץ *kafats* "jumped" → קיפץ *kipets* "hopped about".
2. **Causative / factitive**, often from adjectives: גדל *gadal* "grew" → גידל *gidel* "raised"; *naki* "clean" → ניקה *nika* "cleaned"; *khadash* "new" → חידש *khidesh* "renewed".
3. **Denominal** verbs from nouns: *séder* "order" → סידר *sider* "arranged"; *tik* "file" → תייק *tiyek* "filed"; *mispar* "number" → מספר *misper* "numbered".
4. **Lexical** verbs with no partner: דיבר *diber* "spoke", ביקש *bikesh* "asked for", שילם *shilem* "paid", טייל *tiyel* "travelled".

Pi'el is the **main productive binyan** for new transitive verbs and for nearly all verbs made from foreign nouns (§ 4), because its template can hold four or more consonants and consonant clusters: טלפן *tilfen* "phoned", גוגל *gigel* "googled" (Bolozky 1999; Bat-El 1994).

### 2.4 Pu'al, פֻּעַל

**Form.** Past *CuCaC* (*sudar*), present *meCuCaC* (*mesudar*), future *yeCuCaC* (*yesudar*). Pu'al has **no infinitive and no imperative** in normative grammar.

**Meaning.** The passive of pi'el: סידר *sider* "arranged" → סודר *sudar* "was arranged"; בישל *bishel* "cooked" → בושל *bushal* "was cooked". Finite pu'al forms are formal and rare in speech (1.5 % of verbs in the Knesset treebank; § 1.3). The **present participle** is the living part of pu'al, used overwhelmingly as an adjective: מסודר *mesudar* "tidy, organised", מבושל *mevushal* "cooked", מפורסם *mefursam* "famous", מסובך *mesubakh* "complicated", מבוגר *mevugar* "adult", מיוחד *meyukhad* "special".

### 2.5 Hif'il, הִפְעִיל

**Form.** Past *hiCCiC* (*hirgish*), present *maCCiC* (*margish*), future *yaCCiC* (*yargish*), infinitive *lehaCCiC* (*lehargish*), verbal noun *haCCaCa* (*hargasha*).

**Meanings.**

1. **Causative** of pa'al: אכל *akhal* "ate" → האכיל *he'ekhil* "fed"; לבש *lavash* "put on (clothes)" → הלביש *hilbish* "dressed (someone)"; רקד *rakad* "danced" → הרקיד *hirkid* "made dance"; כתב *katav* → הכתיב *hikhtiv* "dictated"; קם *kam* "rose" → הקים *hekim* "set up, founded".
2. **Causative of nif'al**: נרדם *nirdam* "fell asleep" → הרדים *hirdim* "put to sleep, anaesthetised"; נכנס *nikhnas* "entered" → הכניס *hikhnis* "put in, let in".
3. **Inchoative of states and colours** (become X): הלבין *hilbin* "turned white", האדים *he'edim* "blushed", השמין *hishmin* "got fat", הרזה *hirza* "lost weight", החמיר *hekhmir* "got worse".
4. **Lexical**: הרגיש *hirgish* "felt", הבין *hevin* "understood", התחיל *hitkhil* "began", הזמין *hizmin* "invited; ordered", הגיע *higia* "arrived", האמין *he'emin* "believed".

Hif'il is the second most frequent binyan (§ 1.3) and moderately productive for new verbs, especially causatives and inchoatives (e.g. slang הפליק *hiflik* "slapped"; Bolozky 1999 ranks hif'il third in productivity after pi'el and hitpa'el).

### 2.6 Huf'al (hof'al), הֻפְעַל

**Form.** Past *huCCaC* (*hurgash*), present *muCCaC* (*murgash*), future *yuCCaC* (*yurgash*). No infinitive, no imperative.

**Meaning.** The passive of hif'il: הזמין *hizmin* "invited" → הוזמן *huzman* "was invited"; הכניס *hikhnis* → הוכנס *hukhnas* "was put in". Like pu'al, it is formal in finite use (0.9 % of verbs in Knesset, 5.2 % in Wikipedia, § 1.3), while its participles live on as adjectives: מוכן *mukhan* "ready", מוכשר *mukhshar* "talented", מוצלח *mutslakh* "successful", מוזר *muzar* "strange", מובן *muvan* "understood, obvious".

### 2.7 Hitpa'el, הִתְפַּעֵל

**Form.** Past *hitCaCeC* (*hitlabesh*), present *mitCaCeC* (*mitlabesh*), future *yitCaCeC* (*yitlabesh*), infinitive *lehitCaCeC* (*lehitlabesh*), verbal noun *hitCaCCut* (*hitlabshut*). Like pi'el it once geminated the middle consonant, so begadkefat letters there are stops.

**Metathesis and assimilation of the prefix *t*** (normative and fully regular in speech):

| First root consonant | Rule | Example (root → past 3MS) |
|---|---|---|
| ס *s*, ש *sh*, שׂ *s* | *t* and the sibilant swap: *hit-s…* → *his-t…* | ס-כ-ל → הסתכל *histakel* "looked"; ש-מ-ש → השתמש *hishtamesh* "used" |
| צ *ts* | swap, and *t* becomes emphatic ט (pronounced *t*) | צ-ל-מ → הצטלם *hitstalem* "had a photo taken" |
| ז *z* | swap, and *t* voices to ד *d* | ז-ק-נ → הזדקן *hizdaken* "grew old" |
| ד *d*, ט *t* | *t* assimilates: written התד־/הד־, pronounced *hid-*/*hit-* | ד-ר-ד-ר → הידרדר / התדרדר *hidarder* "deteriorated" (both spellings occur) |

(Glinert 1989 gives metathesis with sibilants; Wikipedia, "Modern Hebrew verb conjugation".)

**Meanings.**

1. **Reflexive** (with "grooming" and body-care verbs): התלבש *hitlabesh* "got dressed", התרחץ *hitrakhets* "washed (oneself)", התגלח *hitgaleakh* "shaved", התאפר *hit'aper* "put on make-up". Kastner (2017) argues that reflexive verbs exist **only** in hitpa'el, because only this template combines the structure for an agent with the absence of a separate object.
2. **Reciprocal**: התכתב *hitkatev* "corresponded", התנשק *hitnashek* "kissed (each other)", התחבק *hitkhabek* "hugged", התווכח *hitvake'akh* "argued", התראה *hitra'a* "saw each other". The reciprocal can take a comitative: *hitkatávti im Dana* "I corresponded with Dana".
3. **Inchoative / anticausative** (change of state), the largest group in use: התרגש *hitragesh* "got excited", התעורר *hit'orer* "woke up", התקרר *hitkarer* "cooled down; caught a cold", התקלקל *hitkalkel* "broke down", התפוצץ *hitpotsets* "exploded".
4. **Pretence / behaving as**: התחפש *hitkhapes* "dressed up, disguised oneself", התחכם *hitkhakem* "acted the wise guy", התיילד *hityaled* "behaved childishly", התקטנן *hitkatnen* "was petty" (from *katan* "small", with reduplicated last consonant).
5. **Passive** (formal, with some roots): התגלה *hitgala* "was discovered", התקבל *hitkabel* "was received, accepted".
6. **Lexical**: התפלל *hitpalel* "prayed", התנהג *hitnaheg* "behaved" (unrelated in sense to pa'al *nahag* "drove"), התקשר *hitkasher* "called (on the phone)".

**Nitpa'el.** Rabbinic Hebrew had a passive-reflexive *nitpa'el* (נִתְקַבֵּל *nitkabel*, נִתְגַּלָּה *nitgala*), with the nif'al prefix *ni-* on the hitpa'el stem. In Modern Hebrew it survives only as a high-register past tense (newspaper headlines, legal and rabbinic style) and is otherwise replaced by hitpa'el; Wigderson (2015) attributes part of its disappearance to Yiddish influence on early modern speakers. The IAHLT treebanks keep a separate value `HebBinyan=NITPAEL` for these forms: 21 tokens in all (נתגלו, נתגלתה, נתקבלה, נתברר, נשתנה …), against 1 150 hitpa'el.

---

## 3. Valency alternations

The binyanim are the main device of Hebrew for changing the number of arguments of a verb. The tables below show the commonest correspondences; each is lexical (not every root has every member), but the direction is regular.

### 3.1 Transitive ↔ intransitive (causative / anticausative)

| Intransitive (no agent) | Binyan | Transitive (agent) | Binyan |
|---|---|---|---|
| נשבר *nishbar* "broke" | nif'al | שבר *shavar* "broke (sth)" | pa'al |
| נפתח *niftakh* "opened" | nif'al | פתח *patakh* "opened (sth)" | pa'al |
| נסגר *nisgar* "closed" | nif'al | סגר *sagar* "closed (sth)" | pa'al |
| גדל *gadal* "grew" | pa'al | גידל *gidel* "raised" / הגדיל *higdil* "enlarged" | pi'el / hif'il |
| נרדם *nirdam* "fell asleep" | nif'al | הרדים *hirdim* "put to sleep" | hif'il |
| התעורר *hit'orer* "woke up" | hitpa'el | העיר *he'ir* "woke (someone)" | hif'il |
| התקלקל *hitkalkel* "broke down" | hitpa'el | קלקל *kilkel* "spoiled, broke" | pi'el |
| התרגש *hitragesh* "got excited" | hitpa'el | ריגש *rigesh* "moved, excited (sb)" | pi'el |
| נמס *names* "melted" | nif'al | המיס *hemis* "melted (sth)" | hif'il |

Two templates serve as the "intransitive" side (nif'al and hitpa'el) and three as the "transitive" side (pa'al, pi'el, hif'il). Roughly, roots whose transitive verb is pa'al take nif'al as the anticausative, and roots whose transitive verb is pi'el take hitpa'el (Doron 2003; Kastner 2017).

### 3.2 Active ↔ passive

| Active | Passive | Register of the passive |
|---|---|---|
| pa'al כתב *katav* | nif'al נכתב *nikhtav* | written; spoken in some fixed uses |
| pi'el בישל *bishel* | pu'al בושל *bushal* | written; participle *mevushal* everywhere |
| hif'il הזמין *hizmin* | huf'al הוזמן *huzman* | written; participle *muzman* everywhere |

The by-phrase is על ידי *al yedey* "by (lit. on hands of)":

```
ההצעה נדחתה על ידי הוועדה.
ha-hatsa'a nidkhata al yedey ha-va'ada
DEF-proposal reject.NIF.PST.3FS on hands.CS DEF-committee
'The proposal was rejected by the committee.'
```

In speech, the passive is usually avoided in favour of an **impersonal 3PL active** with no subject (see chapter 07):

```
דחו את ההצעה.
dakhu et ha-hatsa'a
reject.PST.3PL ACC DEF-proposal
'The proposal was rejected.' (lit. 'They rejected the proposal.')
```

### 3.3 Causative

Hif'il is the productive causative of pa'al intransitives and of some transitives. A causative may take two objects, the causee often marked with את or with ל־:

```
היא הלבישה את הילד מעיל.
hi hilbísha et ha-yéled me'il
she dress.CAUS.PST.3FS ACC DEF-boy coat
'She put a coat on the boy.'
```

```
המורה הכתיבה לנו את התרגיל.
ha-mora hikhtíva lánu et ha-targil
DEF-teacher.F dictate.PST.3FS to.us ACC DEF-exercise
'The teacher dictated the exercise to us.'
```

Where there is no lexical causative, Hebrew uses analytic *garam le-* "caused to" + infinitive, or *hekhriakh* "forced" + infinitive.

### 3.4 Reflexive and reciprocal

Hitpa'el gives intrinsic reflexives (§ 2.7). For other verbs, Hebrew uses the reflexive pronoun *atsmo* "himself" (lit. "his bone"), which inflects: *atsmi, atsmekha, atsmekh, atsmo, atsma, atsménu, atsmekhem, atsmekhen, atsmam, atsman*:

```
הוא שונא את עצמו.
hu sone et atsmo
he hate.PRS.MS ACC self.3MS
'He hates himself.'
```

Reciprocal without hitpa'el: *ze et ze* (M), *zo et zo* (F), *ehad et ha-sheni* "one another": *hem azru ze la-ze* "they helped each other".

### 3.5 Doublets and lexical drift

Many binyan pairs have drifted apart in meaning, so morphology is a hint, not a guarantee:

- נהג *nahag* "drove" vs התנהג *hitnaheg* "behaved";
- ספר *safar* "counted" vs סיפר *siper* "told; cut hair" vs הסתפר *histaper* "had a haircut";
- שמר *shamar* "guarded, kept" vs השתמר *hishtamer* "was preserved";
- חשב *khashav* "thought" vs חישב *khishev* "calculated" vs התחשב *hitkhashev* "took into account, was considerate";
- רצה *ratsa* "wanted" vs ריצה *ritsa* "appeased; served (a sentence)" vs התרצה *hitratsa* "relented".

---

## 4. Making new verbs

### 4.1 Pi'el and hitpa'el as the open classes

New verbs in Modern Hebrew are coined almost entirely in **pi'el** (transitive) and **hitpa'el** (intransitive, reflexive, inchoative), with **hif'il** a distant third; pa'al and nif'al are practically closed (Bolozky 1999; Bat-El 1994). The reasons usually given are structural: pi'el and hitpa'el keep the stem's consonants in a fixed skeleton *C1iC2eC3* / *hitC1aC2eC3* that can stretch to four or five consonants (*C1iC2C3eC4*), and they have no vowel changes inside the stem across the paradigm, so a borrowed consonant cluster is preserved (Bat-El 1994).

### 4.2 Denominal verbs from Hebrew nouns

Speakers and the Academy build verbs from nouns, which may themselves come from a root. The verb then gets a new root extracted from the noun, sometimes including the noun's prefix or suffix (root augmentation; Yannai 1983):

| Noun | Verb | Note |
|---|---|---|
| תפקיד *tafkid* "function, role" | תפקד *tifked* "functioned" | root ת-פ-ק-ד includes the noun's *t-* prefix (Yannai 1983) |
| מספר *mispar* "number" | מספר *misper* "numbered" | *m-* of the noun kept |
| מסגרת *misgéret* "frame" | מסגר *misger* "framed" | |
| חשבון *kheshbon* "account" | התחשבן *hitkhashben* "settled accounts, nitpicked" | *-n* of the noun kept |
| קטן *katan* "small" | התקטנן *hitkatnen* "was petty" | reduplication of last consonant |
| שקר *shéker* "lie" | שיקר *shiker* "lied" | pi'el from the noun's root |

### 4.3 Verbs from foreign words

A borrowed noun has no root, so speakers **extract** one by taking its consonants in order (Ornan 2014 calls this "squeezing"; Bat-El 1994 shows that consonant clusters of the source word are kept as clusters). The consonants are then put into pi'el or hitpa'el:

| Source | Extracted root | Verb (past 3MS) | Infinitive | Meaning |
|---|---|---|---|---|
| *telefon* "telephone" | ט-ל-פ-נ | טלפן *tilfen* | לטלפן *letalfen* | phoned |
| *faks* "fax" | פ-ק-ס-ס | פקסס *fikses* | לפקסס *lefakses* | faxed |
| SMS | ס-מ-ס | סימס *simes* | לסמס *lesames* | texted |
| Google | ג-ג-ל (ג-ו-ג-ל) | גיגל *gigel* | לגגל *legagel* | googled |
| *flirt* | פ-ל-ר-ט-ט | פלרטט *flirtet* | לפלרטט *leflartet* | flirted |
| *fabrikatsya* | פ-ב-ר-ק | פברק *fibrek* | לפברק *lefabrek* | fabricated |
| *format* | פ-ר-מ-ט | פרמט *pirmet* (*firmet*) | לפרמט *lefarmet* | formatted |
| *sinkhron* | ס-נ-כ-ר-נ | סנכרן *sinkhren* | לסנכרן *lesankhren* | synchronised |
| *kompyuter* | none; analytic *la'avod al ha-makhshev* | | | |

Note two regularities:

- A two-consonant base is stretched to three by **copying the last consonant**: *faks* → פקסס *f-k-s-s* (*fikses*); compare *katan* → *hitkatnen*.
- The spirantised value of the borrowed consonant stays fixed: *f* in *fikses*, *flirtet* stays *f* in all forms, even where a native פ would be [p] (*lefakses*, never *\*lepakses*) (Bat-El 1994; on the *p/f* split see chapter 03).

Hitpa'el gives the intransitive, reflexive or "becoming" verb: התאקלם *hit'aklem* "acclimatised", התפלסף *hitpalsef* "philosophised", התחרדן *hitkharden* "became ultra-Orthodox" (slang, from *kharedi*), התברגן *hitbargen* "became bourgeois" (from *burgani*).

### 4.4 Light-verb constructions

Where no verb exists or a nominal style is preferred, Hebrew pairs a general verb with a noun: עשה מאמץ *asa ma'amats* "made an effort", הרים טלפון *herim télefon* "picked up the phone, phoned", ערך ביקור *arakh bikur* "paid a visit", קיבל החלטה *kibel hakhlata* "took a decision", נתן מכה *natan maka* "hit" (Ornan 2014). Liebeskind and HaCohen-Kerner (2016) built a lexical resource of such Hebrew verb–noun multiword expressions, classified as morphological, syntactic and semantic types; many are fully idiomatic (*shavar et ha-rosh* "broke the head" = "racked his brains").

---

## 5. Tense, aspect and mood

### 5.1 Three tenses

Modern Hebrew has a three-way tense system:

| Tense | Form | Agreement | Example |
|---|---|---|---|
| past (*avar*) | suffixal (old perfect, *qatal*) | person, gender, number | כתבתי *katávti* "I wrote" |
| present (*hove*) | participle (*beynoni*) | gender, number only | כותבת *kotévet* "(she/I/you F) write(s)" |
| future (*atid*) | prefixal (old imperfect, *yiqtol*) | person, gender, number | אכתוב *ekhtov* "I will write" |

**Aspect is not marked.** Biblical Hebrew's aspectual and consecutive forms (*wayyiqtol*, *weqatal*) are gone; the old perfect is now simply past and the old imperfect simply future (see chapter 10 on the history). Progressive, perfect, and habitual readings are left to context and adverbs:

```
אכלתי כבר.
akhálti kvar
eat.PST.1SG already
'I've already eaten.'
```

```
כשנכנסת, דיברתי בטלפון.
kshe-nikhnásta, dibárti ba-télefon
when-enter.PST.2MS speak.PST.1SG in.DEF-telephone
'When you came in, I was talking on the phone.'
```

### 5.2 The present is the participle

The present tense is the active participle (*beynoni* "intermediate"), which inflects like an adjective for gender and number only, with **four forms** and no person:

| | M | F |
|---|---|---|
| SG | כותב *kotev* | כותבת *kotévet* |
| PL | כותבים *kotvim* | כותבות *kotvot* |

Because the form does not show person, the subject pronoun is **obligatory** in the present (*ani kotev* "I write", *ata kotev* "you write", *hu kotev* "he writes"), whereas in the past and future 1st and 2nd person pronouns are optional (*katávti* "I wrote" alone is a full sentence; 3rd person usually keeps its pronoun or noun). See chapter 07 on pro-drop.

```
אני גר בחיפה, והיא גרה בתל אביב.
ani gar be-kheyfa, ve-hi gara be-tel aviv
I live.PRS.MS in-Haifa and-she live.PRS.FS in-Tel Aviv
'I live in Haifa and she lives in Tel Aviv.'
```

(In hollow roots the present FS and the past 3FS differ only in stress: present גרה *gara* "lives", final stress; past גרה *gára* "lived", penultimate. See 05b § 3.4.)

The present expresses general truths, habits, ongoing events and near future:

```
מחר אני טס לאילת.
makhar ani tas le-eylat
tomorrow I fly.PRS.MS to-Eilat
'Tomorrow I'm flying to Eilat.'
```

### 5.3 The participle as verb, noun and adjective

The *beynoni* sits between verb, noun and adjective, which is the source of its name. The same form can be:

- a **verb** in the present tense: *hu shomer al ha-bayit* "he guards the house";
- a **noun** (agent or instrument): שומר *shomer* "guard", מורה *more* "teacher", מנהל *menahel* "manager", מזכיר *mazkir* "secretary", מתנדב *mitnadev* "volunteer", מחשב *makhshev* "computer" (hif'il-like pattern of ח-ש-ב);
- an **adjective**: מעניין *me'anyen* "interesting", מפתיע *maftia* "surprising", נפלא *nifla* "wonderful", מוכן *mukhan* "ready", מסודר *mesudar* "tidy".

As a noun it takes the construct state and definiteness of a noun (*shomer ha-saf* "the doorkeeper", lit. "guard.CS the-threshold"; *ha-menahel* "the manager"). This three-way ambiguity is one of the hardest problems in tagging Hebrew. Adler et al. (2008) report that in their corpus *beynoni* forms make up under 3 % of tokens, of which about 90 % are verbal; existing lexicons disagreed widely on their part of speech, and inter-annotator agreement rose to above 99 % only after the tagset introduced a dedicated participle tag. UD Hebrew tags them by function (VERB with `VerbForm=Part|Tense=Pres` when verbal, NOUN or ADJ otherwise); see chapter 11.

### 5.4 Habitual past and counterfactual: *haya* + participle

The past of the copula *haya* followed by the participle (agreeing with the subject) gives a **habitual past** and, in conditionals, the **counterfactual**:

```
כשהייתי ילד, הייתי הולך לים כל יום.
kshe-hayíti yéled, hayíti holekh la-yam kol yom
when-be.PST.1SG boy be.PST.1SG go.PRS.MS to.DEF-sea every day
'When I was a boy, I used to go to the sea every day.'
```

```
אם הייתי יודע, הייתי בא.
im hayíti yodéa, hayíti ba
if be.PST.1SG know.PRS.MS be.PST.1SG come.PRS.MS
'If I had known, I would have come.'
```

The same construction expresses polite wishes: *hayíti rotse kos mayim* "I would like a glass of water". In IAHLTknesset such *haya* forms are tagged AUX; the analytic conditional is described in Glinert (2005) and in Wikipedia, "Modern Hebrew grammar".

The future of *haya* + participle gives a future progressive or a supposition: *bi-shmone ani ihye yoshev ba-kita* "at eight I'll be sitting in class".

### 5.5 The future and modality

The prefixal future is also the form of:

- **polite or softened commands** (§ 6.2);
- **negative commands** with *al* (§ 6.3);
- **purpose and subordinate clauses** after *she-*, *kedey she-* "so that", *lifney she-* "before":

```
אמרתי לו שיבוא מחר.
amárti lo she-yavo makhar
say.PST.1SG to.him that-come.FUT.3MS tomorrow
'I told him to come tomorrow.'
```

- **wishes and suggestions** in the 1PL: *nelekh?* "shall we go?", *nir'e* "we'll see".

A jussive survives only in fixed expressions: *yekhi ha-melekh* "long live the king", *yehi* "let there be", *yitgadal* (prayer).

---

## 6. The imperative and its spoken substitutes

### 6.1 Morphology

The imperative is built on the future stem with the prefix removed, for 2nd person only, in four forms (MS, FS, MPL, FPL), and exists in pa'al, nif'al, pi'el, hif'il and hitpa'el (not in pu'al or huf'al):

| Binyan | Future 2MS | Imperative MS | FS | MPL |
|---|---|---|---|---|
| pa'al | תכתוב *tikhtov* | כתוב *ktov* | כתבי *kitvi* | כתבו *kitvu* |
| nif'al | תיכנס *tikanes* | היכנס *hikanes* | היכנסי *hikansi* | היכנסו *hikansu* |
| pi'el | תדבר *tedaber* | דבר *daber* | דברי *dabri* | דברו *dabru* |
| hif'il | תרגיש *targish* | הרגש *hargesh* | הרגישי *hargíshi* | הרגישו *hargíshu* |
| hitpa'el | תתלבש *titlabesh* | התלבש *hitlabesh* | התלבשי *hitlabshi* | התלבשו *hitlabshu* |

Nif'al restores the prefix *hi-*; hif'il restores *ha-*. The feminine plural (*któvna*, *dabérna*) is formal (05b).

### 6.2 What speakers actually say

The morphological imperative is felt as **direct, brusque or formal**, and everyday speech uses other forms (Glinert 2005; Wikipedia, "Modern Hebrew verb conjugation"):

1. **The future form** is the ordinary, neutral way to tell or ask someone to do something:

```
תסגור את הדלת, בבקשה.
tisgor et ha-délet, bevakasha
close.FUT.2MS ACC DEF-door please
'Close the door, please.'
```

Here *tisgor* is formally future ("you will close"); in context it is a request. The morphological imperative *sgor et ha-délet* is sharper. The IAHLTknesset treebank annotates such future forms by their function, with `Mood=Imp` and no `Tense` (e.g. *tagídu* "say (PL)!", *ta'amínu* "believe (PL)!", in sentence `19_ptv_245469-7`: «תאמינו לי, זה שטויות.» "Believe me, it's nonsense."), while HTB-style annotation keeps the future reading. See chapter 11.

2. **Short, high-frequency imperatives** remain fully alive, often as discourse words: בוא *bo*, בואי *bói*, בואו *bóu* "come! / let's"; לך *lekh* "go"; תן *ten* "give" (also *tavi*, see below); שב *shev* "sit"; קום *kum* "get up"; תראה *tir'e* "look" (future form); *ten li* "let me (lit. give me)". In the Knesset treebank the 15 commonest command forms are led by בואו (14), then שב, תנו, תן, תגידו, תאמינו, בוא, עזבו, לך (counted 2026-10-17). *Bo/bóu* + future = "let's": *bóu nelekh* "let's go".

3. **The infinitive** as a general or impersonal command, on signs, in recipes, to children, to groups, and as a brusque order (Wikipedia, "Modern Hebrew verb conjugation"):

```
לא לעשן!
lo le-ashen
NEG INF-smoke
'No smoking!'
```

```
לשבת, ילדים!
lashévet, yeladim
INF.sit children
'Sit down, children!'
```

4. **Suppletion with *havi* and *ten*.** The imperative of הביא *hevi* "brought" is normatively *havé* (הבא); speakers say the future *tavi* "bring!" or *ten* "give". Similarly, *amar* "said" supplies past and present, while *higid* "told" supplies future, imperative and infinitive (*agid*, *tagid*, *lehagid*); speakers do not use *yomar*, *emor* in ordinary speech (*tagid li* "tell me", *lehagid* "to say") (Wikipedia, "Modern Hebrew verb conjugation").

### 6.3 Negative commands

The imperative cannot be negated. A prohibition uses ***al*** + future (individual, immediate) or ***lo*** + infinitive / ***asur*** + infinitive (general):

```
אל תדאג.
al tid'ag
NEG.IMP worry.FUT.2MS
'Don't worry.'
```

*Lo* + future is a strong prohibition or a statement: *lo tignov* "thou shalt not steal" (biblical style), *lo tetse ha-érev* "you are not going out tonight".

### 6.4 Softeners

*Bevakasha* "please", *na* "please" (formal, after the verb: *shev na*), *yalla* (Arabic, "come on"), and the conditional *haya efshar…?* "would it be possible…?" soften commands. See chapter 09 for the pragmatics.

---

## 7. Non-finite forms

### 7.1 The infinitive with ל־

The Modern Hebrew infinitive (*shem ha-po'al*) is the old infinitive construct with the preposition ל־ fused to it: לכתוב *likhtov*, לדבר *ledaber*, להרגיש *lehargish*, להתלבש *lehitlabesh*, להיכנס *lehikanes*. In Modern Hebrew the ל־ is no longer separable; the form is a single word (Schwarzwald 2013). Pu'al and huf'al have none.

Uses:

- complement of verbs and modals: *ani rotse lishon* "I want to sleep", *hu tsarikh la'avod* "he has to work";
- purpose: *báti lir'ot otkha* "I came to see you";
- subject of evaluative predicates: *asur le-ashen* "smoking is forbidden", *kashe lilmod ivrit* "it's hard to learn Hebrew";
- command (§ 6.2).

The vowel of the pa'al infinitive is *o* even where the future has *a*: *lilmod* / *yilmad* "learn", *lishmo'a* / *yishma* "hear", *lishkav* (normative *lishkav*) / *yishkav* "lie down"; *likro* / *yikra* "read".

### 7.2 The infinitive construct with other prepositions (formal)

In written registers the bare infinitive stem combines with ב־, כ־, מ־ and takes possessive suffixes as its subject, as in Biblical and Rabbinic Hebrew:

```
בבואו לעיר
be-vo'o la-ir
in-coming.3MS to.DEF-city
'upon his arrival in the city'
```

```
עם צאתו לגמלאות
im tseto li-gimla'ot
with going.out.3MS to-pension
'upon his retirement'
```

Speech uses a finite clause instead: *kshe-hu ba la-ir* "when he came to the city".

### 7.3 The infinitive absolute

The infinitive absolute (*makor nifrad*) of pa'al, *CaCoC* (*halokh*, *shamor*), survives in Modern Hebrew only in idioms and high style:

- הלוך ושוב *halokh va-shov* "there and back, round trip";
- emphatic doubling in literary prose: *ra'o ra'iti* "I have surely seen";
- instructions in older formal style: *shamor* "keep!", *zakhor* "remember!" (in religious texts).

Schwarzwald (2013) describes the Modern Hebrew infinitive system and its residual absolute forms.

### 7.4 Passive participles

Two kinds of passive participle serve as adjectives and resultatives:

| Source | Pattern | Examples |
|---|---|---|
| pa'al | *CaCuC* | כתוב *katuv* "written", סגור *sagur* "closed", פתוח *patú'akh* "open", אהוב *ahuv* "loved", שבור *shavur* "broken", עסוק *asuk* "busy" |
| pu'al | *meCuCaC* | מבושל *mevushal* "cooked", מסודר *mesudar* "tidy", מפורסם *mefursam* "famous" |
| huf'al | *muCCaC* | מוכן *mukhan* "ready", מוגזם *mugzam* "exaggerated", מוזמן *muzman* "invited" |

With the copula they make a resultative state, which speakers use where English uses a passive:

```
החנות סגורה בשבת.
ha-khanut sgura be-shabat
DEF-shop closed.F on-Saturday
'The shop is closed on Saturdays.'
```

```
הספר כתוב בעברית קלה.
ha-sefer katuv be-ivrit kala
DEF-book written.MS in-Hebrew easy.F
'The book is written in easy Hebrew.'
```

Pa'al *CaCuC* is also a semi-productive adjective pattern with active or stative meaning: *atsuv* "sad", *batú'akh* "sure", *amus* "busy, loaded", *zakuk* "in need of", *ra'ul* "poisonous".

### 7.5 Verbal nouns (*shem pe'ula*)

Each active binyan has a regular verbal noun pattern, used for actions and processes; most are also lexicalised as ordinary nouns:

| Binyan | Pattern | Examples |
|---|---|---|
| pa'al | *CCiCa* | כתיבה *ktiva* "writing", שמירה *shmira* "guarding", אכילה *akhila* "eating", ריצה *ritsa* "running" (hollow *CiCa*), קנייה *kniya* "purchase" (final weak) |
| nif'al | *hiCaCCut* | היכנסות *hikansut* "entering", הישארות *hisha'arut* "staying", הירדמות *hiradmut* "falling asleep" |
| pi'el | *CiCuC* | דיבור *dibur* "speech", סידור *sidur* "arrangement; prayer book", בישול *bishul* "cooking", טיפול *tipul* "treatment, care" |
| pu'al | — | (none) |
| hif'il | *haCCaCa* | הרגשה *hargasha* "feeling", הזמנה *hazmana* "invitation; order", הבנה *havana* "understanding" (hollow *haCaCa*) |
| huf'al | — | (none) |
| hitpa'el | *hitCaCCut* | התלבשות *hitlabshut* "dressing", התפתחות *hitpatkhut* "development", התנהגות *hitnahagut* "behaviour" |

Pa'al verbs often have lexically specific action nouns in other noun patterns instead of or beside *CCiCa*: *avoda* "work" (עבד), *halikha* "walking" (הלך), *shena* "sleep" (ישן), so the *CCiCa* form is reliable for new or technical uses but not exhaustive.

Verbal nouns take the construct state and possessive suffixes like any noun, with the object as a construct dependent or with של: *ktivat ha-sefer* "the writing of the book"; *hitpatkhut ha-yéled* "the child's development".

---

## 8. Agreement, pronouns and object suffixes

### 8.1 Person, number and gender

Hebrew verbs agree with the subject in person (past, future), gender and number. Gender is marked in the 2nd person (*katávta* M / *katavt* F) and 3rd person singular, and normatively in all plurals; in speech the plural gender distinction survives in the past 2PL (*-tem/-ten*) only weakly and in the present plural (*-im/-ot*) fully, but has collapsed in the future plural (§ 10.2).

### 8.2 Pronominal object suffixes (literary)

Older and formal written Hebrew can attach object pronouns directly to the verb: שמרני *shmarani* "he guarded me", ראיתיו *re'itiv* "I saw him", להורגו *le-horgo* "to kill him". Spoken Modern Hebrew uses the inflected object marker instead: *ra'íti oto* "I saw him", *laharog oto* "to kill him". The suffixed forms create genuine ambiguity in unpointed text: Dicta's *Suffixed Verbal Forms* dataset (2 853 sentences, CC BY 4.0) is built around such cases, e.g. ל-י-מ-ד-ו "they taught" vs "he taught him" (Dicta 2024). See chapter 06 for *oto, ota, otam*.

### 8.3 Subject–verb order and agreement

With verb–subject order (common after adverbs and in existentials), agreement is regular, but in speech a preceding verb may stay 3MS before a feminine or plural subject, especially with *haya*, *nish'ar*, *higia*:

```
היה שם הרבה אנשים.
haya sham harbe anashim
be.PST.3MS there many people
'There were a lot of people there.'
```

Normative grammar requires *hayu sham harbe anashim*. See chapter 07 for existentials and word order.

---

## 9. The copula *haya* and the modal words

### 9.1 The copula היה

*Haya* "was, existed, happened" has past and future but **no present**: the present copula is zero or a pronoun (*hu, hi, hem, hen*) or *ze* (see chapter 07).

| | Past | | Future | |
|---|---|---|---|---|
| 1SG | הייתי | *hayíti* | אהיה | *ehye* (spoken *ihye*) |
| 2MS | היית | *hayíta* | תהיה | *tihye* |
| 2FS | היית | *hayit* | תהיי | *tihyi* |
| 3MS | היה | *haya* | יהיה | *yihye* |
| 3FS | הייתה | *hayta* | תהיה | *tihye* |
| 1PL | היינו | *hayínu* | נהיה | *nihye* |
| 2MPL | הייתם | *hayitem* (spoken *hayítem*) | תהיו | *tihyu* |
| 2FPL | הייתן | *hayiten* | תהיו (formal תהיינה *tihyéna*) | *tihyu* |
| 3PL | היו | *hayu* | יהיו (FPL formal תהיינה) | *yihyu* |

Imperative הֱיֵה *heye*, הֲיִי *hayi*, הֱיוּ *heyu* (in speech mostly *tihye*: *tihye bari* "be well"); infinitive להיות *lihyot*. Note the unpointed spelling היית for both 2MS and 2FS.

Uses: copula in past/future (*hu haya more* "he was a teacher"); existential past/future (*haya sham khatul* "there was a cat there", the past of *yesh*); possession (*hayta li mekhonit* "I had a car", lit. "was to-me a car"); auxiliary of habitual and counterfactual (§ 5.4).

The verb *nihya* (נהיה) "became" is nif'al of the same root: *hu nihya rofe* "he became a doctor"; colloquially also "happened": *ma nihya?* "what's up?".

### 9.2 Modal words

Most Hebrew modals are **adjectives or participles** that agree with the subject in gender and number but not person, or **impersonal predicates** that do not agree at all. They take an infinitive. Their past and future are formed with *haya*.

| Modal | Type | Agreement | Meaning | Example |
|---|---|---|---|---|
| יכול *yakhol* | verb-like adjective; FS *yekhola*, PL *yekholim*, *yekholot* | G, N | can, be able | *ani lo yekhola lavo* "I (F) can't come" |
| צריך *tsarikh* | participle-adjective; *tsrikha*, *tsrikhim*, *tsrikhot* | G, N | must, need to; need | *ata tsarikh lalékhet* "you have to go" |
| חייב *khayav* | adjective; *khayévet*, *khayavim*, *khayavot* | G, N | must, be obliged | *hi khayévet le-hagia* "she must arrive" |
| אמור *amur* | passive participle; *amura*, *amurim*, *amurot* | G, N | supposed to | *ha-rakévet amura lehagia ba-shalosh* "the train is supposed to arrive at three" |
| עלול *alul* | adjective | G, N | liable to, might (bad) | *ze alul lehistabekh* "it might get complicated" |
| עשוי *asui* | passive participle | G, N | likely to | |
| מוכרח *mukhrakh* | huf'al participle | G, N | must, forced | |
| מסוגל *mesugal* | pu'al participle | G, N | capable of | |
| מוכן *mukhan* | huf'al participle | G, N | willing, ready | |
| כדאי *keday* | impersonal | none | worthwhile, should | *keday lekha lishon* "you'd better sleep" |
| אסור *asur* | impersonal | none | forbidden | *asur le-ashen kan* "no smoking here" |
| מותר *mutar* | impersonal | none | allowed | *mutar lehikanes?* "may one come in?" |
| אפשר *efshar* | impersonal | none | possible, one may | *efshar le-vakesh khéshbon?* "may I have the bill?" |
| צריך *tsarikh* (impersonal) | impersonal 3MS | none | one must | *tsarikh lavo be-zman* "one must come on time" |

*Yakhol* has a full verbal paradigm in past and future, with an irregular stem: past יכולתי *yakhólti*, יכול *yakhol*, יכלה *yakhla*; present יכול *yakhol*; future אוכל *ukhal*, תוכל *tukhal*, יוכל *yukhal*, יוכלו *yukhlu* (spoken 1SG *yukhal* also heard). UD lemmatises it as יכל (IAHLT) and tags it AUX.

Past and future of modals: *hayíti tsarikh lalékhet* "I had to go", *yihye asur* "it will be forbidden", *haya keday* "it would have been worth it". The impersonal ones take a dative experiencer: *keday lekha*, *asur lahem*, *mutar li*.

In the Knesset treebank the commonest AUX lemmas without a binyan feature are צריך (115), יכל (98), אינו (31, the negative copula), חייב (29), אמור (17), מוכן (8), מוכרח (5), עלול (4), מסוגל (3) (counted 2026-10-17).

---

## 10. Normative and spoken: the living verb

Modern Hebrew grammar was codified on Biblical and Mishnaic models, while the spoken language, acquired natively since the early 20th century, has regularised the system in many places. The **Academy of the Hebrew Language** keeps the normative paradigm; speakers, including educated ones, use the levelled forms in everyday speech, and some of them now appear in informal writing. The main tendencies, described in Ravid (1995), Bolozky (1999), Glinert (2005) and Schwarzwald (2001), are below; forms marked "spoken" are common in casual speech and are not errors in the descriptive sense.

### 10.1 1SG future with *y-* / *i-*

The 1SG future prefix *e-* / *a-* (אכתוב *ekhtov*, אדבר *adaber*) is widely replaced in speech by *yi-* / *ye-*, identical to the 3MS (*ani yikhtov*, *ani yedaber*, *ani yavi*) or by *i-* (*ani ikhtov*, *ihye*). The pronoun *ani* then carries the person. The levelled form is common among young speakers and in informal speech; the normative *e-* form is taught at school and expected in writing (Ravid 1995; Wikipedia, "Modern Hebrew verb conjugation", for the variation). Example:

```
אני יבוא מחר. (spoken)  ~  אני אבוא מחר. (normative)
ani yavo makhar  ~  ani avo makhar
I come.FUT.3MS tomorrow ~ I come.FUT.1SG tomorrow
'I'll come tomorrow.'
```

### 10.2 Loss of the feminine plural in the future and imperative

The 2nd and 3rd person feminine plural future (*tikhtóvna*) and the feminine plural imperative (*któvna*) are replaced by the masculine (common-gender) plural *tikhtevu*, *yikhtevu*, *kitvu* in all but very formal or literary writing (Wikipedia, "Modern Hebrew verb conjugation"; Glinert 2005). The future plural therefore has one form per person:

| | Normative | Spoken (and most writing) |
|---|---|---|
| 2MPL | תכתבו *tikhtevu* | תכתבו *tikhtevu* |
| 2FPL | תכתובנה *tikhtóvna* | תכתבו *tikhtevu* |
| 3MPL | יכתבו *yikhtevu* | יכתבו *yikhtevu* |
| 3FPL | תכתובנה *tikhtóvna* | יכתבו *yikhtevu* |

The 2PL past feminine *-ten* is also often replaced by *-tem* in speech when addressing women, though less consistently.

### 10.3 Stress in the 2PL past

Normatively, the 2PL past has final stress and a reduced first vowel in pa'al: כתבתם *ktavtem*, הייתם *heyitem*. Speech keeps the stem of the other persons and stresses it: *katávtem*, *hayítem*, *dibártem*. The spoken form is universal in casual speech.

### 10.4 Lamed-aleph merges with lamed-he

Roots ending in א (*kara* "read", *matsa* "found", *mila* "filled") lost their glottal stop long ago, and in speech they tend to conjugate like roots ending in ה (*kana* "bought"): normative *yikra* "will read" vs spoken *yikre* (like *yikne*); normative 1SG past of nif'al *nikréti* (נקראתי) vs spoken *nikríti*; pi'el *miléti* (מילאתי) vs spoken *milíti*. Spelling errors such as מיליתי reflect the merger (Ravid 1995). Details in 05b § 3.6.

### 10.5 *-íti* spreads in final-weak roots

In final-weak (lamed-he) verbs the normative 1st and 2nd person past has *-íti* in pa'al and pi'el (קניתי *kaníti*, שיניתי *shiníti*) but *-éti* in nif'al, pu'al, hif'il, huf'al and hitpa'el (נקניתי *niknéti*, הראיתי *her'éti*, התנסיתי *hitnaséti*). Speakers tend to generalise one vowel across binyanim, mostly *-íti* (*hitnasíti*, *her'íti*) (Ravid 1995). The distribution varies by verb and speaker; treat it as a tendency.

### 10.6 Binyan confusions

Some verbs move between nif'al and hitpa'el or take an innovative binyan in speech:

- נתקלתי *nitkálti* "I bumped into" → spoken התקלתי *hitkálti* (the nif'al read as hitpa'el by analogy);
- *hitkashárti* "I called (phoned)" is standard; *hitkasher* has displaced older *tilfen* in many contexts.

### 10.7 Other spoken regularisations

- **Pe-aleph** verbs: normative 1SG future *okhal* "I'll eat" is often *yokhal* by § 10.1; normative infinitive *le'ekhol* is pronounced *le'ekhol* or *lekhol*.
- **Hollow hif'il** future and imperative: the future *tavi* "you'll bring" serves as the imperative (§ 6.2); the infinitive *lehavi* is stable.
- **Pa'al future vowel**: verbs with future *a* (*yilmad*, *yishkav*) sometimes take *o* in speech by analogy, and vice versa; such variation is lexically limited.
- **Spirantisation levelling**: the alternation of *b/v*, *k/kh*, *p/f* inside a paradigm (pa'al past ספר *safar* "counted" but infinitive לספור *lispor*) is sometimes levelled in speech toward one value (*lisfor* is heard beside *lispor*). In verbs made from foreign words the borrowed value is fixed throughout (*fikses*, *lefakses*). Chapter 03 describes the alternation.

(For several of these, quantitative studies exist mainly in Hebrew; see "wanted" in sources.md.)

---

## 11. Verb classes by root shape (overview)

Roots whose consonants are "weak" (gutturals, glides *y* and *v*, *n* that assimilates, א or ה at the end, identical 2nd and 3rd consonants) follow sub-paradigms. Their full key cells are tabulated in [05b-verb-paradigms.md](05b-verb-paradigms.md) § 3. Traditional Hebrew names cite the position of the weak consonant with the letters of פ-ע-ל: *pe* = 1st, *ayin* = 2nd, *lamed* = 3rd.

| Class | Hebrew name | Example | Typical deviation |
|---|---|---|---|
| strong (sound) | שלמים *shlemim* | כתב *katav* | none |
| 1st *n* | פ"נ | נפל *nafal*, נסע *nasa*, נתן *natan* | *n* drops or assimilates: *yipol*, *yisa*, *latet* |
| 1st *y* (*v*) | פ"י / פ"ו | ישב *yashav*, ירד *yarad*, ידע *yada* | future *yeshev*, inf *lashévet*; hif'il *hoshiv* |
| 1st guttural | פ' גרונית | עבד *avad*, חשב *khashav* | future *ya'avod*, *yakhshov*; vowel *a/e* by the guttural |
| 1st *'* (aleph) | פ"א | אכל *akhal*, אמר *amar* | future *yokhal*, *yomar* |
| 2nd guttural | ע' גרונית | שאל *sha'al*, צחק *tsakhak* | pi'el *te'er*, *tse'ek*; no gemination |
| hollow (2nd *v/y*) | ע"ו / ע"י | קם *kam*, שר *shar*, בא *ba* | two-consonant stem: *kámti*, *yakum*, *lakum* |
| final weak (3rd *h*, old *y*) | ל"ה / ל"י | קנה *kana*, שינה *shina* | *-íti/-éti*, future *-e*, inf *-ot* |
| 3rd *'* (aleph) | ל"א | קרא *kara*, מצא *matsa* | *karáti*; future *yikra*; merges with ל"ה in speech |
| 3rd guttural (ח, ע) | ל' גרונית | שמע *shama*, פתח *patakh* | furtive *a*: *shoméa*, *liftóakh* |
| geminate | ע"ע / כפולים | סבב *savav*, הגן *hegen* | short stems in hif'il: *hesev*, *megen* |
| quadriliteral | מרובעים | טלפן *tilfen*, גלגל *gilgel* | pi'el / pu'al / hitpa'el only |

A verb can be weak in two positions at once (ידע *yada* is 1st *y* and 3rd guttural; נשא *nasa* is 1st *n* and 3rd aleph; הביא *hevi* is hollow and 3rd aleph).

---

## 12. Computational notes

- **Ambiguity of unpointed forms.** Without vowel points, one written verb form often stands for several analyses: כתב = *katav* "he wrote" / *ktav* "script" / *katav* "reporter"; ספר = *safar* "counted" / *siper* "told" / *sefer* "book" / *sapar* "barber"; דיברו can be *dibru* "they spoke" or imperative *dabru* in defective writing. Full spelling (*ktiv male*, chapter 02) removes part of this ambiguity by writing *i* in pi'el (דיבר) and *o/u* (כותב, סודר).
- **Morphological analysers.** Hspell's morphological analyser (Har'El and Kenigsberg, hspell 1.4, AGPL) generates the full verb paradigms from root and binyan tables and is the basis of the HebMorph analyser; the UniMorph Hebrew data (from Wiktionary, CC BY-SA 3.0) has 1 176 lemmas with pointed and unpointed paradigms and was used in SIGMORPHON 2021. Dicta's wordlist gives lemma, root and features for unpointed forms (CC BY-NC, so only for tests and measurement in Mova; licence/DATA.md).
- **UD features.** UD Hebrew encodes the binyan as `HebBinyan=PAAL|NIFAL|PIEL|PUAL|HIFIL|HUFAL|HITPAEL|NITPAEL`, tense as `Tense=Past|Pres|Fut`, the participle as `VerbForm=Part`, the infinitive as `VerbForm=Inf`, the imperative as `Mood=Imp`, voice as `Voice=Act|Pass|Mid`. Chapter 11 describes these in detail.

---

## References

Local paths are on the Mova research server; URLs are open.

- Adler, Meni; Netzer, Yael; Goldberg, Yoav; Gabay, David; Elhadad, Michael. 2008. *Tagging a Hebrew Corpus: The Case of Participles.* LREC 2008. Local: `$RESEARCH_RAW/lingbulk-2008-adler-tagging-hebrew-corpus-case-participles/`. URL: http://www.lrec-conf.org/proceedings/lrec2008/pdf/315_paper.pdf
- Bat-El, Outi. 1994. Stem modification and cluster transfer in Modern Hebrew. *Natural Language and Linguistic Theory* 12: 571–596. (Not held; cited from secondary literature — see sources.md «wanted».)
- Bolozky, Shmuel. 1999. *Measuring Productivity in Word Formation: The Case of Israeli Hebrew.* Leiden: Brill. (Not held.)
- Bolozky, Shmuel. 1996. *501 Hebrew Verbs.* Barron's. (Not held.)
- Dicta. 2024. *Suffixed Verbal Forms Detection Dataset for Modern Hebrew* (dicta-il/hebrew_suffix_verbal_forms), CC BY 4.0. Local: `$RESEARCH_RAW/lexres/morphology/dicta/hebrew_suffix_verbal_forms/`. URL: https://huggingface.co/datasets/dicta-il/hebrew_suffix_verbal_forms
- Doron, Edit. 2003. Agency and voice: the semantics of the Semitic templates. *Natural Language Semantics* 11: 1–67. (Not held; summarised via Kastner 2017.)
- Glinert, Lewis. 1989. *The Grammar of Modern Hebrew.* Cambridge University Press. (Not held; cited via Wikipedia.)
- Glinert, Lewis. 2005. *Modern Hebrew: An Essential Grammar*, 3rd ed. Routledge. (Not held; listed in `$RESEARCH_RAW/hebgram/grammars.tsv` as list-only.)
- Har'El, Nadav; Kenigsberg, Dan. Hspell 1.4: Hebrew spell checker and morphological analyser, AGPL-3.0. Local: `$RESEARCH_RAW/lexres/morphology/sa/hspell/`, `$RESEARCH_RAW/hebgram/sa/hebmorph/`. URL: http://hspell.ivrix.org.il/
- Kastner, Itamar. 2017. Reflexive verbs in Hebrew: Deep unaccusativity meets lexical semantics. *Glossa* 2(1): 75, 1–33. DOI: https://doi.org/10.5334/gjgl.299. Local: `$RESEARCH_RAW/lingbulk-2017-kastner-reflexive-verbs-hebrew-deep-unaccusativity/`.
- Liebeskind, Chaya; HaCohen-Kerner, Yaakov. 2016. A Lexical Resource of Hebrew Verb-Noun Multi-Word Expressions. LREC 2016. Local: `$RESEARCH_RAW/lingbulk-2016-liebeskind-lexical-resource-hebrew-verb-noun/`.
- Ornan, Uzzi. 2014. Innovation of Verbs in Hebrew. *Proceedings of the Joint Workshop on Social Dynamics and Personal Attributes in Social Media*, ACL, 94–97. Local: `$RESEARCH_RAW/aclbulk-2014-ornan-innovation-verbs-hebrew/`.
- Ravid, Dorit. 1995. *Language Change in Child and Adult Hebrew: A Psycholinguistic Perspective.* Oxford University Press. (Not held.)
- Schwarzwald, Ora (Rodrigue). 2001. *Modern Hebrew.* Languages of the World/Materials 127. Munich: Lincom. (Not held.)
- Schwarzwald, Ora. 2013. Infinitive: Modern Hebrew. In *Encyclopedia of Hebrew Language and Linguistics*, Brill. (Not held; cited via Wikipedia.)
- Universal Dependencies 2.18: UD_Hebrew-IAHLTwiki and UD_Hebrew-IAHLTknesset (Zeldes et al.), CC BY-SA 4.0. Local: `$RESEARCH_RAW/lexres/treebanks/ud-2.18/`. Counts in §§ 1.3, 2.7, 6.2, 9.2 made for this manual on 2026-10-17. Sentence `19_ptv_245469-7` quoted (IAHLTknesset).
- UniMorph Hebrew (heb, heb_voc), annotator Omer Goldman, from Wiktionary, CC BY-SA 3.0. Local: `$RESEARCH_RAW/hebgram/sa/unimorph-heb/`. URL: https://github.com/unimorph/heb. Used to check paradigm cells.
- Wigderson, Shira. 2015. The sudden disappearance of nitpael and the rise of hitpael in Modern Hebrew. *Journal of Jewish Languages* 3. (Not held; cited via Wikipedia.)
- Wikibooks (he). *דקדוק העברית החדשה* (Grammar of Modern Hebrew), page «הטיות פעלים». CC BY-SA. Local: `$RESEARCH_RAW/hebgram/sa/wikibooks-he-dikduk-haivrit-hahadasha/`. Consulted for the traditional binyan table; not copied.
- Wikipedia. "Modern Hebrew verb conjugation." https://en.wikipedia.org/wiki/Modern_Hebrew_verb_conjugation (consulted 2026-10-17).
- Wikipedia. "Modern Hebrew grammar." https://en.wikipedia.org/wiki/Modern_Hebrew_grammar (consulted 2026-10-17).
- Yannai, Yig'al. 1983. Innovation and neologism in Modern Hebrew. In B. Snell (ed.), *Term Banks for Tomorrow's World: Translating and the Computer 4*, London: Aslib. Local: `$RESEARCH_RAW/lingbulk-1982-yannai-innovation-neologism-modern-hebrew/`.
