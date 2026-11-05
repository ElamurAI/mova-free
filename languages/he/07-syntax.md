# 7. Syntax

The clause and the sentence in Modern Hebrew: word order and its variations, subjects that may be dropped, clauses without a verb and the several copulas, existence and possession with יש and אין, the object marker את, relative and complement clauses, questions, negation, impersonal and passive sentences, conditionals, comparison, coordination, ellipsis, agreement, and the order inside the noun phrase. How UD annotates each construction is in [11 UD analysis](11-ud-analysis.md); here the focus is the language.

[← README](README.md) · [← 06 Prepositions and particles](06-prepositions-particles.md) · [08 Lexicon and register →](08-lexicon-register.md)

Glossing follows the Leipzig rules with these labels: M, F, SG, PL, 1/2/3 (person), DEF (article), CS (construct), ACC (object marker את), POSS, PST, FUT, PRS (participle used as present), INF, IMP, COP, NEG, EXIST, REL (relative ש/אשר), COMP (complementizer ש), Q (question marker), DAT (ל marking a possessor or experiencer). Transliteration as defined in [02](02-script-and-spelling.md).

---

## Contents

- [7.1 Basic word order](#71-basic-word-order)
- [7.2 Subjects and pro-drop](#72-subjects-and-pro-drop)
- [7.3 Verbless clauses and the copulas](#73-verbless-clauses-and-the-copulas)
- [7.4 Existence and possession: יש and אין](#74-existence-and-possession-יש-and-אין)
- [7.5 Objects and את](#75-objects-and-את)
- [7.6 Datives, experiencers and the "ethical dative"](#76-datives-experiencers-and-the-ethical-dative)
- [7.7 Relative clauses](#77-relative-clauses)
- [7.8 Complement clauses and reported speech](#78-complement-clauses-and-reported-speech)
- [7.9 Questions](#79-questions)
- [7.10 Negation](#710-negation)
- [7.11 Impersonal and generic sentences](#711-impersonal-and-generic-sentences)
- [7.12 Passive and middle](#712-passive-and-middle)
- [7.13 Modality](#713-modality)
- [7.14 Adverbial clauses and conditionals](#714-adverbial-clauses-and-conditionals)
- [7.15 Comparison](#715-comparison)
- [7.16 Coordination](#716-coordination)
- [7.17 Ellipsis](#717-ellipsis)
- [7.18 Agreement](#718-agreement)
- [7.19 The noun phrase](#719-the-noun-phrase)
- [7.20 Information structure](#720-information-structure)
- [References](#references)

---

## 7.1 Basic word order

The unmarked order of a transitive clause is **subject – verb – object** (SVO), with obliques and adverbials after the object (Glinert 1989; Coffin and Bolozky 2005; UD Hebrew guidelines, *Word order*).

```
דני קנה את הספר בחנות.
dáni kana et ha-séfer ba-khanut
Dani buy.PST.3MS ACC DEF-book in.DEF-shop
'Dani bought the book in the shop.'
```

Hebrew is head-initial throughout: prepositions, not postpositions; nouns before their adjectives and genitives; verbs before their objects; complementizers at the start of their clauses.

Order is freer than in English because the object of a definite noun is marked by את and obliques by prepositions. The commonest departures:

**Verb–subject after a fronted element ("triggered inversion").** When an adverbial, an object or a complement opens the clause, the verb may come before the subject. This is a feature of written and formal Hebrew in particular and an inheritance of the older language (Shlonsky 1997; Glinert 1989):

```
אתמול הגיעו האורחים.
etmol higí'u ha-orkhim
yesterday arrive.PST.3PL DEF-guest.PL
'Yesterday the guests arrived.'
```

```
מחר יגיע אורח.
makhar yagí'a óre'akh
tomorrow arrive.FUT.3MS guest
'Tomorrow a guest will arrive.'
```

The second example is the one the UD Hebrew guidelines use to show that VS after a temporal adverb is *more* usual than SV, though both are grammatical.

**Verb–subject with unaccusatives and presentational verbs.** Verbs of appearance, arrival and happening put an indefinite subject after the verb, even without a fronted element (Borer 1984; Shlonsky 1987 for the classic analysis of "free inversion"):

```
נפל לי העט.
nafal li ha-et
fall.PST.3MS to.me DEF-pen
'My pen fell' (lit. 'fell to-me the pen').
```

```
קרתה תאונה בכביש.
karta te'una ba-kvish
happen.PST.3FS accident in.DEF-road
'There was an accident on the road.'
```

**Counted (2026-10-17).** Among nominal subjects of verbs in the CC BY-SA IAHLT treebanks, the subject precedes the verb in 2 835 cases and follows it in 1 844 in Wikipedia text (61% SV), against 2 593 and 806 in Knesset speech (76% SV). Written exposition inverts far more than speech. (Count: every `nsubj` whose head is VERB, by linear order; all splits.)

**Existential sentences** are verb-initial: יש לו כסף "he has money", lit. "there-is to-him money" ([§ 7.4](#74-existence-and-possession-יש-and-אין)).

## 7.2 Subjects and pro-drop

Hebrew is a **partial pro-drop** language (Borer 1989; Shlonsky 2009; Ritter 1995). The rule depends on person and tense, i.e. on whether the verb form itself shows person:

| Verb form | Shows person? | Subject pronoun |
|---|---|---|
| Past, 1st and 2nd person (*katávti* "I wrote", *katávta* "you M wrote") | yes, by suffix | may be dropped, usually is in writing |
| Future, 1st and 2nd person (*ekhtov* "I'll write", *tikhtov* "you M will write") | yes, by prefix (+ suffix) | may be dropped |
| Past and future, 3rd person (*katav* "he wrote", *yikhtov* "he will write") | yes, but… | normally **required** in a main clause with no antecedent |
| Present (participle: *kotev* "write(s)") | no — only gender and number | **required** |

```
הלכתי הביתה.
halákhti ha-báyta
go.PST.1SG DEF-home.towards
'I went home.'
```

```
*הלך הביתה.            → הוא הלך הביתה.
halakh ha-báyta          hu halakh ha-báyta
go.PST.3MS home          he go.PST.3MS home
'He went home.'
```

A 3rd-person subject may be dropped when it is **controlled by an antecedent in a higher clause** ("embedded pro-drop") and in coordinate clauses with a shared subject:

```
דני אמר שיבוא מחר.
dáni amar she-yavó makhar
Dani say.PST.3MS COMP-come.FUT.3MS tomorrow
'Dani said he'd come tomorrow.'
```

A dropped 3rd-person masculine plural with no antecedent gives the **impersonal** reading ([§ 7.11](#711-impersonal-and-generic-sentences)): אומרים ש... "they say that…".

In speech the 1st and 2nd person pronouns are retained more often than the grammars' "may be dropped" suggests, especially in the future and for contrast (אני אלך "*I* will go"), and the colloquial future 1SG increasingly uses a 3MS-like form with a pronoun (*ani yelekh* for *elekh*; see [05](05-verbs.md)) — which makes the pronoun necessary.

**Counted (2026-10-17).** In IAHLTknesset (speech), of past and future verbs in 1st or 2nd person, 228 have an overt `nsubj` and 609 do not (73% dropped); in IAHLTwiki such verbs are rare (90), and 79% drop. Third-person verbs lack a dependent subject far less often, and most of those cases are relative clauses or coordination where the subject is elsewhere.

## 7.3 Verbless clauses and the copulas

In the present tense Hebrew needs no verb "to be". A noun, adjective, prepositional phrase or adverb can be the predicate:

```
הבית גדול.
ha-báyit gadol
DEF-house big.MS
'The house is big.'
```

```
דני בבית.
dáni ba-báyit
Dani in.DEF-house
'Dani is at home.'
```

```
אבא שלי רופא.
ába sheli rofé
father of.me doctor
'My father is a doctor.'
```

The verb היה *haya* "be" appears in the past, the future, the infinitive and the imperative, and agrees with the subject:

```
הבית היה גדול.
ha-báyit haya gadol
DEF-house be.PST.3MS big.MS
'The house was big.'
```

```
הדירה תהיה מוכנה בקיץ.
ha-dira tihyé mukhaná ba-káyits
DEF-flat be.FUT.3FS ready.FS in.DEF-summer
'The flat will be ready in the summer.'
```

### The pronominal copula

Between two noun phrases — especially when both are definite, or when the subject is long — a 3rd-person pronoun agreeing with the subject stands where English has *is*: הוא, היא, הם, הן (Doron 1983; Berman and Grosu 1976; Glinert 1989).

```
המורה שלנו היא אישה נהדרת.
ha-moré shelánu hi isha nehédret
DEF-teacher.F of.us COP.3FS woman wonderful.FS
'Our teacher is a wonderful woman.'
```

```
ירושלים היא בירת ישראל.
yerushaláyim hi birat yisra'el
Jerusalem COP.3FS capital.CS Israel
'Jerusalem is the capital of Israel.'
```

Without the copula, two adjacent definite noun phrases would read as one phrase (apposition or a modifier): השיטה הדיגיטלית המקובלת "the accepted digital method" vs השיטה הדיגיטלית **היא** המקובלת "the digital method is the accepted one" (IAHLTwiki `iahltwiki_mammography-58`; [11 § 11.7.2](11-ud-analysis.md#1172-copular-clauses)). The pronoun is obligatory in identity sentences (subject and predicate both definite or proper names) and with a pronoun subject in the 3rd person (*hu hu* "he is he" is avoided by restructuring), optional with an indefinite or adjectival predicate, and impossible with a locative predicate in neutral style (*\*dáni hu ba-báyit*).

### זה as a copula

In speech the neuter demonstrative זה *ze* "this, it" serves as a copula, typically with a generic or sentential subject and an evaluative predicate. It does **not** agree:

```
שוקולד זה טעים.
shokolad ze ta'im
chocolate COP tasty.MS
'Chocolate is tasty.'
```

```
ללמוד עברית זה לא קשה.
lilmod ivrit ze lo kashe
learn.INF Hebrew COP NEG hard.MS
'Learning Hebrew isn't hard.'
```

```
החברים שלי זה הכל בשבילי.
ha-khaverim sheli ze ha-kol bishvili
DEF-friend.PL of.me COP DEF-all for.me
'My friends are everything to me.'
```

The UD guidelines suggest that this use of זה may be modelled on Slavic *to / èto* (Polish, Russian); this is a plausible contact explanation but not proven, and similar uses occur in other languages (e.g. French *c'est*). The predicate adjective with ze is masculine singular even when the subject is feminine or plural (*ha-uga ze ta'im* "cake — that's tasty"), which shows that the adjective agrees with ze, not with the topic.

### Formal copulas: הינו and אינו

Formal written Hebrew has an inflected copula built on the old particle הִנֵּה: הינו *hino* (M), הינה *hina* (F), הינם *hinam* (PL) "is/are", and its negative counterpart אינו *eyno* "is not" ([§ 7.10](#710-negation)). Both inflect with pronominal suffixes for person, gender and number:

| | Positive (formal) | Negative (formal) |
|---|---|---|
| 1SG | הנני *hinéni* | אינני / איני *eynéni / eyni* |
| 2MS | הנך *hinkha* | אינך *eynkha* |
| 2FS | הנך *hinekh* | אינך *eynekh* |
| 3MS | הינו / הנו *hino / hinó* | אינו *eyno* |
| 3FS | הינה / הנה *hina* | אינה *eyna* |
| 1PL | הננו *hinénu* | איננו *eynénu* |
| 2MPL | הנכם *hinkhem* | אינכם *eynkhem* |
| 3MPL | הינם *hinam* | אינם *eynam* |
| 3FPL | הינן *hinan* | אינן *eynan* |

```
המחיר הינו סופי.
ha-mekhir hino sofi
DEF-price COP.3MS final.MS
'The price is final.'   (official notice style)
```

הִנֵּה alone is the presentative "here is": הנה הספר *hine ha-séfer* "here's the book".

## 7.4 Existence and possession: יש and אין

### Existence

יש *yesh* "there is/are" and its negative אין *eyn* "there isn't/aren't" are invariable in colloquial speech. They take the existing thing as their subject and normally precede it:

```
יש חלב במקרר.
yesh khalav ba-mekarer
EXIST milk in.DEF-fridge
'There's milk in the fridge.'
```

```
אין זמן.
eyn zman
NEG.EXIST time
'There's no time.'
```

In the past and future, the existential is a form of היה agreeing with the existing thing (in normative Hebrew):

```
היו הרבה אנשים בהפגנה.
hayu harbe anashim ba-hafgana
be.PST.3PL many people in.DEF-demonstration
'There were many people at the demonstration.'
```

```
לא תהיה בעיה.
lo tihyé be'aya
NEG be.FUT.3FS problem(F)
'There won't be a problem.'
```

Colloquially the past existential is often frozen as masculine singular היה regardless of the noun: *haya harbe anashim*, *haya be'aya* (Glinert 1989; Berman 1978). The normative grammar condemns this; it is common in speech.

Formal Hebrew can inflect יש and אין with suffixes: ישנו *yeshno* "he/it is there", ישנה, ישנם, ישנן "they are present", and אינו etc. (above). The IAHLTwiki sentence ישנן ארבע רמות "There are four levels" (`iahltwiki_glasgow-coma-scale-13`) shows the inflected form agreeing with a feminine plural.

### Possession: there is to me

Hebrew has no verb "have". Possession is an existential with the possessor in a ל phrase:

```
יש לי אח.
yesh li akh
EXIST to.me brother
'I have a brother.'
```

```
אין לנו כסף.
eyn lánu késef
NEG.EXIST to.us money
'We have no money.'
```

```
היה לה כלב.
haya la kélev
be.PST.3MS to.her dog(M)
'She had a dog.'
```

```
תהיה לכם הזדמנות.
tihyé lakhem hizdamnut
be.FUT.3FS to.you.PL opportunity(F)
'You'll have an opportunity.'
```

### The possessed as an object: יש לי את

When the possessed is definite, normative grammar treats it as the subject (no את). In speech, it is very commonly marked with את, like an object:

```
יש לי את הספר הזה.
yesh li et ha-séfer ha-ze
EXIST to.me ACC DEF-book DEF-this
'I have this book.'   (colloquial; normative: יש לי הספר הזה)
```

```
אין לי את הכוח.
eyn li et ha-kóakh
NEG.EXIST to.me ACC DEF-strength
'I don't have the strength.'
```

This is one of the best-known divergences of spoken from normative Hebrew; it shows the possessed being reanalysed as an object, with יש לי behaving like a transitive verb "have" (Berman 1978; Ziv 1976; Halevy 2007 — not held locally). The past forms show the same shift: *haya li et ha-séfer* with frozen masculine singular *haya*.

### Existence vs location

Hebrew separates "there is an X (somewhere)" from "the X is (somewhere)":

```
יש ספר על השולחן.          הספר (נמצא) על השולחן.
yesh séfer al ha-shulkhan     ha-séfer (nimtsa) al ha-shulkhan
EXIST book on DEF-table       DEF-book (be.located.PRS.MS) on DEF-table
'There's a book on the table.' 'The book is on the table.'
```

With a definite subject the locative is a verbless clause, optionally with נמצא "is found/located".

## 7.5 Objects and את

### Definite objects take את

The direct object is marked by את *et* when it is **definite** — with the article, a proper name, a construct with a definite last member, a possessive suffix, a demonstrative, or a definite pronoun (Glinert 1989; Danon 2001):

```
ראיתי את דני.          ראיתי ילד.
ra'íti et dáni          ra'íti yéled
see.PST.1SG ACC Dani    see.PST.1SG boy
'I saw Dani.'           'I saw a boy.'
```

```
קראתי את ספרו.
karáti et sifr-o
read.PST.1SG ACC book.CS-3MS.POSS
'I read his book.'
```

```
אכלנו את כל העוגה.
akhálnu et kol ha-uga
eat.PST.1PL ACC all.CS DEF-cake
'We ate the whole cake.'
```

את is not a case ending but a preposition-like particle; it never marks a subject in standard Hebrew and never an indefinite object (\*ראיתי את ילד). With a pronoun it fuses into the inflected forms אותי *oti* "me", אותך *otkha / otakh* "you", אותו *oto* "him", אותה *ota* "her", אותנו *otánu* "us", אתכם / אותכם *etkhem / otkhem* "you PL", אותם *otam* "them M", אותן *otan* "them F" (full table in [06](06-prepositions-particles.md)).

### Differential object marking: what counts as definite?

Hebrew את is a classic case of **differential object marking** (DOM) sensitive to definiteness (Danon 2001, 2006). The standard view is that *et* appears exactly with definite objects. The edges are interesting:

- **Partitives.** Hacohen, Kagan and Plaut (2021) observe that contemporary speakers also accept *et* before **partitive** objects, which are not formally definite: *et kama me-ha-sfarim* "some of the books", *et shnáyim me-ha-yeladim* "two of the children". In a judgment task with 41 native adults, *et*-marked partitives scored 3.6 on a 5-point scale on average; acceptance was higher with proportional quantifiers ("most of, some of") than with cardinals ("two of"), and higher when the object was fronted. They propose a syntactic analysis that keeps definiteness as the licensing condition (local: `$RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/`, *Glossa* 6(1): 148, CC BY 4.0).
- **Specific indefinites.** In casual speech *et* is sometimes heard with an indefinite noun that is specific, e.g. modified by a relative clause; it is judged substandard.
- **Generic bare nouns** never take *et*.
- **Quantified objects**: כל + definite noun takes *et* (*et kol ha-yeladim* "all the children"); כל + indefinite singular ("every child") varies.
- **Construct nouns**: a construct with an indefinite last member (*beyt séfer* "a school") has no *et*; with a definite last member (*beyt ha-séfer* "the school") it has *et*.

### Pronoun object suffixes on the verb

Literary and Biblical Hebrew attach the object pronoun directly to the verb: ראיתיו *re'itív* "I saw him", הרגוהו *haragúhu* "they killed him". Modern speech uses only the analytic form with אותו; the synthetic form survives in formal writing and fixed phrases (Dicta's dataset of Hebrew suffixed verbal forms collects such forms; local `$RESEARCH_RAW/lexres/morphology/dicta/hebrew_suffix_verbal_forms/`).

### Object position

The object follows the verb; a pronominal object (אותו) normally comes before a full PP (*natati oto le-dani* "I gave it to Dani"); a long object goes last. Objects can be fronted for topic or focus ([§ 7.20](#720-information-structure)).

## 7.6 Datives, experiencers and the "ethical dative"

The preposition ל *le-* marks the recipient (*natan le-dani* "gave to Dani"), the possessor in existential possession, and the **experiencer** of a state predicate, giving a family of very frequent subjectless or "dative subject" constructions:

```
קר לי.
kar li
cold.MS to.me
'I'm cold.'
```

```
נמאס לי.
nim'as li
become.fed.up.PST.3MS to.me
'I'm fed up.'
```

```
חסר לנו כסף.
khaser lánu késef
lack.PRS.MS to.us money
'We're short of money.'
```

```
כואב לי הראש.
ko'ev li ha-rosh
hurt.PRS.MS to.me DEF-head
'My head hurts.'
```

The last example is a **possessor dative**: the body part is the subject, and its owner is in the dative (Borer and Grodzinsky 1986; Landau 1999). Compare *ha-rosh sheli ko'ev* "my head hurts" (possessor with של), which is less idiomatic.

The **ethical dative** adds a ל-pronoun coreferent with the subject to colour the event, usually with verbs of motion or posture: הוא הלך לו *hu halakh lo* "he went off (on his own)", שבי לך *shvi lakh* "sit yourself down", תשתוק לי! *tishtok li* "shut up (for my sake)". It is optional, non-argumental and highly colloquial (Borer and Grodzinsky 1986; Berman 1982 — not held).

## 7.7 Relative clauses

### The markers ש and אשר

Relative clauses follow their noun and open with the prefix ש *she-* "that, which, who"; formal writing also uses the free word אשר *asher*. There is no case-marked relative pronoun like English *whom*:

```
הספר שקניתי
ha-séfer she-kaníti
DEF-book REL-buy.PST.1SG
'the book (that) I bought'
```

```
האיש אשר דיבר איתי
ha-ish asher diber iti
DEF-man REL speak.PST.3MS with.me
'the man who spoke with me'   (formal)
```

### Resumptive pronouns

Inside the relative clause, the position of the head noun is either a gap or a **resumptive pronoun** (Borer 1984; Shlonsky 1992; Doron 1982):

| Position relativized | Gap | Resumptive pronoun |
|---|---|---|
| Subject | obligatory gap: *ha-ish she-diber* | impossible: \**ha-ish she-hu diber* (except across another clause) |
| Direct object | gap common: *ha-séfer she-kaníti* | possible: *ha-séfer she-kaníti oto* |
| Object of a preposition | impossible: \**ha-ish she-dibárti im* | obligatory: *ha-ish she-dibárti **ito*** |
| Possessor | impossible | obligatory: *ha-yéled she-**imo** khola* |

```
הבית שגרנו בו
ha-báyit she-gárnu bo
DEF-house REL-live.PST.1PL in.it
'the house we lived in'
```

```
הילדה שאבא שלה רופא
ha-yalda she-aba shela rofé
DEF-girl REL-father of.her doctor
'the girl whose father is a doctor'
```

Hebrew does not strand prepositions: the preposition stays in the clause with a pronoun (בו, איתו, עליו), or, in formal style, moves to the front with the relative pronoun: *ha-báyit **bo** gárnu* "the house in which we lived" — a calque-like construction of written Hebrew.

### The article as a relative marker

With a participle or an adjective, formal Hebrew can use the article ה as a relative marker: *ha-ish **ha-medaber*** "the man (who is) speaking", *ha-mitbaker **ha**-moda'a* — the guideline example המגדל נחש בביתו "he who raises a snake at home" (UD Hebrew guidelines). With a past verb this is a hyper-formal or archaic style (*ha-hitkabel* "which was received") and is considered substandard by some (Glinert 1989).

### Free relatives

Free (headless) relatives use מי ש "whoever", מה ש "whatever, what", איפה ש "wherever", מתי ש "whenever", and in formal style מי ש / כל מי ש:

```
מי שרוצה יכול לבוא.
mi she-rotse yakhol lavo
who REL-want.PRS.MS can.MS come.INF
'Whoever wants to can come.'
```

```
זה מה שאמרתי.
ze ma she-amárti
this what REL-say.PST.1SG
'That's what I said.'
```

## 7.8 Complement clauses and reported speech

### ש as complementizer

Finite complement clauses are introduced by ש- (formal: כי *ki*):

```
אני חושב שהוא צודק.
ani khoshev she-hu tsodek
I think.PRS.MS COMP-he right.PRS.MS
'I think he's right.'
```

```
הודיעו כי הטיסה בוטלה.
hodí'u ki ha-tisa butla
announce.PST.3PL COMP DEF-flight cancel.PASS.PST.3FS
'They announced that the flight had been cancelled.'   (formal)
```

The complementizer cannot be omitted, unlike English *that*.

### No sequence of tenses

Reported speech and thought keep the tense of the original utterance, relative to the time of speaking; there is no backshift as in English (Glinert 1989; Coffin and Bolozky 2005):

```
הוא אמר שהוא עייף.
hu amar she-hu ayef
he say.PST.3MS COMP-he tired.MS
'He said he was tired.'  (lit. '…that he is tired')
```

```
היא אמרה שהיא תבוא.
hi amra she-hi tavo
she say.PST.3FS COMP-she come.FUT.3FS
'She said she would come.'  (lit. '…that she will come')
```

For a translator into English this is a frequent source of tense errors; for a frame reader it means the embedded tense is relative to the embedding event.

### Infinitival complements

Verbs of wanting, trying, starting and modals take a ל-infinitive (UD `xcomp`):

```
אני רוצה ללכת.
ani rotse lalékhet
I want.PRS.MS go.INF
'I want to go.'
```

"Want someone to do" uses a finite ש-clause with the future: *ani rotse she-tavo* "I want you to come" (lit. "that you will come").

### Embedded questions

Embedded yes/no questions use אם "if/whether" (colloquial) or האם (formal): *ani lo yodéa im hu ba* "I don't know whether he's coming". Embedded wh-questions keep the wh-word with no complementizer: *ani lo yodéa mi ba* "I don't know who's coming".

## 7.9 Questions

### Yes/no questions

In speech, a yes/no question has the order of a statement and a rising final intonation ([03](03-phonology.md)):

```
אתה בא?
ata ba?
you.MS come.PRS.MS
'Are you coming?'
```

Writing and formal speech add the particle האם *ha'im* at the start: האם אתה בא? Colloquial speech can add תגיד *tagid* "tell (me)" or נכון? *nakhon* "right?" as tags: *ata ba, nakhon?*.

### Wh-questions

The question word comes first; there is no auxiliary, and the subject-verb order is free (VS is common after the wh-word):

| Word | Meaning | Example |
|---|---|---|
| מי *mi* | who | מי בא? *mi ba?* "Who's coming?" |
| מה *ma* | what | מה אמרת? *ma amárt?* "What did you (F) say?" |
| איפה *éfo* | where | איפה הבית? "Where is the house?" |
| לאן *le'an* | where to | לאן אתה הולך? "Where are you going?" |
| מאיפה *me-éfo* | where from | מאיפה אתה? "Where are you from?" |
| מתי *matay* | when | מתי הוא מגיע? |
| למה *láma* | why | למה לא באת? "Why didn't you come?" |
| מדוע *madúa* | why (formal) | |
| איך *ekh* | how | איך קוראים לך? "What's your name?" (lit. "how do they call to you?") |
| כמה *káma* | how much / many | כמה זה עולה? "How much is it?" |
| איזה / איזו / אילו *éze / éyzo / élu* | which (M / F / PL) | איזה ספר? "Which book?" |
| של מי *shel mi* | whose | של מי הספר? "Whose is the book?" |

### No preposition stranding

The preposition moves with the question word; it cannot be left behind as in English *Who did you talk to?*:

```
עם מי דיברת?            *מי דיברת עם?
im mi dibárta?
with who speak.PST.2MS
'Who did you talk to?'
```

```
על מה אתם מדברים?
al ma atem medabrim?
on what you.MPL talk.PRS.MPL
'What are you talking about?'
```

### Echo and rhetorical questions

*Ma?!* "What?!", *ma pitom?* "what on earth? / no way" (lit. "what suddenly"), *ma ikhpat lekha?* "what do you care?" — see [09](09-pragmatics-politeness.md).

## 7.10 Negation

| Negator | Use | Example |
|---|---|---|
| לא *lo* | verbs in all tenses, adjectives, adverbs, nouns in contrast | *lo halákhti* "I didn't go"; *lo gadol* "not big" |
| אין *eyn* | negative existential; possession | *eyn khalav* "there's no milk" |
| אינו / אינני… *eyno* | formal negation of a participle or a verbless predicate, inflected for person | *eyno yodéa* "he does not know" (formal) |
| אל *al* + future | prohibition (negative imperative) | *al tid'ag* "don't worry" |
| בלי *bli* | "without" | *bli khalav* "without milk" |
| בלתי *bilti* | "un-, in-" before adjectives, formal | *bilti efshari* "impossible" |
| אי *i* | "non-, in-" before nouns | *i-havana* "misunderstanding" |
| שום *shum* | "no, any" (determiner) | *shum davar* "nothing" |
| אף *af* | "not even, no" | *af ekhad* "nobody" |

### Present tense: לא vs אין/אינו

With the participle, the colloquial negator is לא: *hu lo yodéa* "he doesn't know". Formal written Hebrew prefers inflected אין: *eyno yodéa*, *eyneni yodéa*, *eynam yod'im*. The two coexist as a register contrast; the Mishnaic origin of the אין-construction is described in [10](10-historical-layers.md).

### Prohibitions

The imperative cannot be negated. Prohibitions use אל + **future** form:

```
אל תיגע!
al tiga!
NEG.PROH touch.FUT.2MS
'Don't touch!'
```

Impersonal prohibitions on signs use לא + infinitive: *lo le-ashen* "no smoking", *lo la-ga'at* "do not touch".

### Negative concord

Negative indefinites require **לא** (or אין) in the clause — Hebrew has negative concord, unlike standard English (Glinert 1989):

```
אף אחד לא בא.
af ekhad lo ba
even one NEG come.PST.3MS
'Nobody came.'
```

```
לא ראיתי שום דבר.
lo ra'íti shum davar
NEG see.PST.1SG any thing
'I didn't see anything / I saw nothing.'
```

```
הוא אף פעם לא היה שם.
hu af pa'am lo haya sham
he even time NEG be.PST.3MS there
'He was never there.'
```

```
אין לי כלום.
eyn li klum
NEG.EXIST to.me nothing
'I have nothing.'
```

כלום *klum* "nothing" and אף אחד "nobody" can stand alone as answers: *ma ra'íta? — klum.* "What did you see? — Nothing."

### Negating a constituent

לא before a non-verbal constituent negates just that constituent: *lo dáni amar et ze, ela rina* "It wasn't Dani who said it, but Rina". אלא *éla* "but rather" follows a negation (cf. Spanish *sino*, German *sondern*); אבל *aval* is the ordinary "but".

## 7.11 Impersonal and generic sentences

### The 3rd person plural without subject

A masculine plural verb with no subject means "people, they (unspecified)":

```
אומרים שיהיה חם מחר.
omrim she-yihyé kham makhar
say.PRS.MPL COMP-be.FUT.3MS hot tomorrow
'They say it'll be hot tomorrow.'
```

```
איך קוראים לך?
ekh kor'im lekha?
how call.PRS.MPL to.you.MS
'What's your name?'  (lit. 'how do they call you?')
```

This construction often replaces the passive in speech ([§ 7.12](#712-passive-and-middle)).

### The generic 2nd person

As in English, "you" can be generic: *kshe-ata lomed safa, ata tsarikh...* "when you learn a language, you need to…".

### Modal impersonals with the infinitive

Several invariable predicates take an infinitive with no subject (UD tags many of them AUX or ADJ depending on the treebank):

| Word | Meaning | Example |
|---|---|---|
| צריך *tsarikh* | one must | *tsarikh lalékhet* "we have to go / one must go" |
| אפשר *efshar* | it's possible, may | *efshar lehikanes?* "may I come in?" |
| אי אפשר *i efshar* | it's impossible | *i efshar le-hasbir* "it can't be explained" |
| אסור *asur* | it's forbidden | *asur le-ashen* "smoking is forbidden" |
| מותר *mutar* | it's allowed | *mutar lehikanes* "entry is allowed" |
| כדאי *keday* | it's worth it, advisable | *keday lalékhet* "it's worth going" |
| חבל *khaval* | it's a pity | *khaval le-vazbez* "it's a pity to waste" |
| קשה / קל *kashe / kal* | it's hard / easy | *kashe lehavin* "it's hard to understand" |

The experiencer goes in a ל phrase: *asur lekha le-ashen* "you're not allowed to smoke".

### Weather and time

Weather predicates have no subject: חם *kham* "it's hot", קר *kar* "it's cold", מחשיך *makhshikh* "it's getting dark", or use a noun subject: יורד גשם *yored géshem* "it's raining" (lit. "rain is coming down").

## 7.12 Passive and middle

Hebrew has **morphological passives** — the binyanim pu'al (passive of pi'el), huf'al (passive of hif'il) and, for pa'al verbs, nif'al — and a passive participle of pa'al (*katuv* "written") ([05](05-verbs.md)):

```
המכתב נכתב אתמול.
ha-mikhtav nikhtav etmol
DEF-letter write.PASS.PST.3MS yesterday
'The letter was written yesterday.'
```

```
הבניין שופץ על ידי העירייה.
ha-binyan shupats al yedey ha-iriya
DEF-building renovate.PASS.PST.3MS by DEF-municipality
'The building was renovated by the municipality.'
```

The agent is introduced by על ידי *al yedey* "by" (lit. "on the hands of"), a formal device (UD: `fixed` with `ExtPos=ADP`). Passives with an overt agent are characteristic of written and journalistic style; speech prefers the active, the impersonal 3PL, or a middle in nif'al or hitpa'el (Berman 1978; Glinert 1989). The register gap shows in the treebanks: huf'al and pu'al verbs are roughly seven times rarer in Knesset speech than in Wikipedia ([11 § 11.5](11-ud-analysis.md#115-features)).

**Middle and anticausative**: many nif'al and hitpa'el verbs have a non-agentive reading — נשבר *nishbar* "broke (intr.)", התפוצץ *hitpotsets* "exploded", נפתח *niftakh* "opened (intr.)" — and are analysed as unaccusative (Borer 1991; Reinhart and Siloni 2005; Kastner 2017; see [05](05-verbs.md)).

## 7.13 Modality

| Meaning | Expression | Example |
|---|---|---|
| ability | יכול *yakhol* (inflects: *yekhola*, *yekholim*), past *yakhol haya*, future *yukhal* | *ani lo yakhol lavo* "I can't come" |
| obligation | צריך (inflects: *tsrikha, tsrikhim*); חייב *khayav* "must" | *at tsrikha lishon* "you (F) need to sleep" |
| expectation | אמור *amur* "supposed to" | *hu amur lehagi'a* "he's supposed to arrive" |
| possibility (negative) | עלול *alul* "liable to" (bad outcomes) | *ze alul lekhaot* "it may hurt" |
| possibility (positive) | עשוי *asuy* "likely to" | *ha-matsav asuy le-hishtaper* "the situation may improve" |
| willingness | מוכן *mukhan* "ready, willing" | *ani mukhan la'azor* "I'm willing to help" |
| permission | רשאי *rashay* "entitled" (formal) | |
| desire | רוצה *rotse* | |

UD Hebrew tags these AUX (with `VerbType=Mod` in IAHLT); their counts as `aux` in IAHLT are led by יכול (254), אינו (141), צריך (131), היה (112), עלול (38), חייב and אמור (31 each) (2026-10-17). The guidelines note that Hebrew modals do not form a uniform syntactic class.

**Habitual and conditional past**: היה + participle expresses a past habit ("used to") and the counterfactual conditional ([§ 7.14](#714-adverbial-clauses-and-conditionals)):

```
בילדותי הייתי הולך לים כל יום.
be-yaldut-i hayíti holekh la-yam kol yom
in-childhood-1SG.POSS be.PST.1SG go.PRS.MS to.DEF-sea every day
'As a child I used to go to the sea every day.'
```

## 7.14 Adverbial clauses and conditionals

### Subordinators

Most adverbial subordinators are a preposition or adverb + ש:

| Subordinator | Meaning |
|---|---|
| כש־ / כאשר *kshe- / ka'asher* | when |
| אחרי ש *akharey she-* | after |
| לפני ש *lifney she-* | before |
| מאז ש *me'az she-* | since |
| עד ש *ad she-* | until |
| בזמן ש *bi-zman she-* | while |
| כי *ki* / מפני ש *mipney she-* / בגלל ש *biglal she-* / היות ש *heyot she-* | because, since |
| כדי ש *kdey she-* (+ FUT) / כדי ל (+ INF) | so that / in order to |
| למרות ש *lamrot she-* / אף על פי ש *af al pi she-* | although |
| אם *im* | if |
| אילו / לו *ilu / lu* | if (counterfactual) |
| אלמלא / לולא *ilmale / lule* | if not, were it not for |
| כאילו *ke'ilu* | as if |
| ברגע ש *be-réga she-* | as soon as |

כש־ is a prefix (*kshe-higáti* "when I arrived"); UD tags it SCONJ (HTB: `Case=Tem`).

### Real conditionals

The protasis with אם takes the future (for future reference) or the past; the apodosis has the future or an imperative. The word אז *az* "then" may open the main clause:

```
אם יירד גשם, (אז) לא נצא.
im yered géshem, (az) lo netse
if fall.FUT.3MS rain (then) NEG go.out.FUT.1PL
'If it rains, we won't go out.'
```

```
אם גמרת, תגיד לי.
im gamárta, tagid li
if finish.PST.2MS tell.FUT.2MS to.me
'If you've finished, tell me.'
```

### Counterfactuals

Counterfactual conditionals use אילו / לו (formal) or אם (colloquial) with **היה + participle** in both clauses, or the past in the protasis:

```
אם הייתי יודע, הייתי בא.
im hayíti yodéa, hayíti ba
if be.PST.1SG know.PRS.MS be.PST.1SG come.PRS.MS
'If I had known, I would have come.'
```

```
לו ידעתי, הייתי בא.
lu yadáti, hayíti ba
if.CF know.PST.1SG be.PST.1SG come.PRS.MS
'Had I known, I'd have come.'   (formal)
```

The same היה + participle form expresses polite wishes and suggestions: *hayíti rotse kos máyim* "I'd like a glass of water"; *hayíti mamlits...* "I'd recommend…" ([09](09-pragmatics-politeness.md)). The construction is a Mishnaic and later development ([10](10-historical-layers.md)). The IAHLT treebanks mark conditionals in MISC as `Cxn=Conditional-NeutralEpistemic` (174) or `-NegativeEpistemic` ([11 § 11.7.7](11-ud-analysis.md#1177-construction-annotation-cxn-in-iahlt)).

## 7.15 Comparison

| Degree | Construction | Example |
|---|---|---|
| comparative | יותר *yoter* "more" + ADJ + מ *mi-* "than" | *dáni yoter gavóha mi-rina* "Dani is taller than Rina" |
| inferiority | פחות *pakhot* "less" + ADJ + מ | *pakhot yakar mi-ze* "less expensive than this" |
| equality | כמו *kmo* "like", באותה מידה כמו | *gavóha kmo aba shelo* "as tall as his father" |
| superlative (colloquial) | הכי *haki* + ADJ | *ha-séfer haki tov* "the best book" |
| superlative (formal) | ADJ + ביותר *beyoter* | *ha-séfer ha-tov beyoter* |
| superlative (literary) | construct of the adjective | *gdol ha-meshorerim* "the greatest of the poets" |
| intensive | מאוד *me'od* "very" (after the adjective) | *tov me'od* "very good" |

"Than" before a clause is ממה ש *mi-ma she-*: *ze yoter kal mi-ma she-khashávti* "it's easier than I thought". Before a pronoun מ inflects: *mimeni* "than me", *mimkha* "than you" ([06](06-prepositions-particles.md)).

## 7.16 Coordination

ו *ve-* "and" is a prefix on the first word of the last conjunct ([02](02-script-and-spelling.md) for its pronunciations *ve-*, *u-*, *va-*). In lists, only the last item has it, as in English: *lekhem, khalav ve-beytsim* "bread, milk and eggs". Other coordinators: או *o* "or", אבל *aval* "but", אלא *éla* "but rather", אך *akh* "but" (formal), גם... וגם *gam... ve-gam* "both… and", או... או "either… or", לא... ולא / לא... וגם לא "neither… nor", בין... ובין "whether… or" (formal). In UD the first conjunct heads; the conjunction is `cc` of the following conjunct.

## 7.17 Ellipsis

### VP ellipsis and verb-stranding

Hebrew can elide the object and the rest of the verb phrase while keeping the verb — "verb-stranding VP ellipsis" in the analysis of Goldberg (2005), contested by Landau (2018):

```
— שמת את המפתחות על השולחן?   — שמתי.
— samta et ha-maftekhot al ha-shulkhan?   — sámti.
put.PST.2MS ACC DEF-key.PL on DEF-table   put.PST.1SG
'Did you put the keys on the table? — I did.'  (lit. 'I put.')
```

Landau (2018) argues that what is missing in such answers is not a whole elided verb phrase but the arguments themselves — **argument ellipsis**, as in East Asian languages. Among his arguments: all the data offered for VP ellipsis are also consistent with argument ellipsis, some data are only consistent with it, and the verb before the missing object need not even be the same verb as in the antecedent clause (local: `$RESEARCH_RAW/lingbulk-2018-landau-missing-objects-hebrew-argument-ellipsis/`). For a frame reader the practical lesson is the same: a Hebrew verb with no object in an answer or a parallel clause often has an object recoverable from context.

### Null objects

Beyond answers, Hebrew drops definite objects recoverable from discourse more freely than English: *lakákhti* "I took (it)", *ra'íta? — ra'íti* "did you see (it)? — I saw (it)".

### Gapping and stripping

*dáni kara séfer ve-rina iton* "Dani read a book and Rina a newspaper" (gapping, as in English). *Gam ani* "me too", *gam lo* "not … either" (stripping).

## 7.18 Agreement

The verb agrees with its subject in person (past, future), gender and number; the participle in gender and number only; adjectives with their noun in gender, number and definiteness ([04](04-nouns-adjectives-pronouns.md)):

```
הבנות הגדולות הלכו הביתה.
ha-banot ha-gdolot halkhu ha-báyta
DEF-girl.PL DEF-big.FPL go.PST.3PL DEF-home
'The big girls went home.'
```

Special cases:

- **Mixed gender**: masculine plural wins: *dáni ve-rina halkhu* (3PL), *hem nekhmadim* "they (M+F) are nice".
- **Verb before a coordinated subject** may agree with the nearer (first) conjunct: *higía dáni ve-ishto* "arrived.3MS Dani and his wife" (formal style) (Doron 2000; Aoun, Benmamoun and Sportiche 1994 for the parallel in Arabic).
- **Construct subjects** agree with the head (first member): *beyt ha-sfarim nisgar* "the library (lit. house-of the-books) closed.3MS".
- **Collective and quantified subjects**: *rov ha-anashim khoshvim* "most people think.MPL" (semantic agreement) vs. *rov ha-tsibur khoshev* "most of the public thinks.MS".
- **Colloquial levelling**: in the future, 2nd and 3rd person feminine plural forms (*tikhtóvna*) have given way to the common plural (*tikhtevu*) in speech ([05](05-verbs.md)).
- **Numbers**: a numeral agrees in gender with its noun (with the polarity of form described in [04](04-nouns-adjectives-pronouns.md)); colloquially the feminine forms of the numerals spread to masculine nouns.
- **Existential היה** with a plural or feminine subject may stay 3MS in speech ([§ 7.4](#74-existence-and-possession-יש-and-אין)).

## 7.19 The noun phrase

The order inside the noun phrase is (Danon 2008; Shlonsky 2004; Ritter 1991):

```
(quantifier) (numeral) NOUN (construct complement) (adjective…) (demonstrative) (PP) (relative clause)
```

```
שלושת הספרים החדשים האלה של דני שקניתי
shlóshet ha-sfarim ha-khadashim ha-éle shel dáni she-kaníti
three.CS DEF-book.PL DEF-new.MPL DEF-these of Dani REL-buy.PST.1SG
'these three new books of Dani's that I bought'
```

Points to note:

- **Adjectives follow** the noun, and repeat the article when the noun is definite: *ha-séfer ha-khadash* "the new book" (lit. "the book the new").
- **Demonstratives follow** the noun and take the article: *ha-báyit ha-ze* "this house"; a demonstrative before a noun is the predicate: *ze báyit* "this is a house". Formal Hebrew also has *báyit ze* "this house" without the article.
- **Cardinal numerals** precede the noun; "one" follows it: *séfer ekhad* "one book" vs *shlosha sfarim* "three books" ([04](04-nouns-adjectives-pronouns.md)).
- **The construct** cannot be broken by an adjective: an adjective modifying the first noun comes after the whole chain, so *beyt ha-mora ha-gadol* is ambiguous in writing between "the teacher's big house" and "the house of the big teacher", resolved by agreement where gender differs (here *gadol* is masculine, agreeing with *báyit*, not the feminine *mora*).
- **Possession** by suffix (*beyt-o*), by של (*ha-báyit shelo*) or by both (*beyt-o shel dáni* "Dani's house", the "double genitive") ([04](04-nouns-adjectives-pronouns.md)).

## 7.20 Information structure

Hebrew marks topic and focus by order and by a few particles:

**Left dislocation** with a resumptive pronoun puts a topic first (Ziv 1994; Ariel 1990):

```
את דני, אני לא מכיר אותו.
et dáni, ani lo makir oto
ACC Dani I NEG know.PRS.MS him
'Dani — I don't know him.'
```

```
הבית הזה, הגג שלו דולף.
ha-báyit ha-ze, ha-gag shelo dolef
DEF-house DEF-this DEF-roof of.it leak.PRS.MS
'This house, its roof leaks.'
```

**Topicalization** without a pronoun: *et ha-séfer ha-ze kvar karáti* "This book I've already read." The fronted object keeps its את.

**Focus particles**: רק *rak* "only", גם *gam* "also", אפילו *afílu* "even", דווקא *davka* "precisely, contrary to expectation" — a particle so characteristic of Hebrew that it is often borrowed into Jewish English ([06](06-prepositions-particles.md), [09](09-pragmatics-politeness.md)).

**Clefts** are rare; Hebrew prefers stress and order or a pseudo-cleft with מה ש / מי ש: *mi she-amar et ze haya dáni* "the one who said it was Dani"; *ze dáni she-amar et ze* "it's Dani who said it" (colloquial, possibly calqued).

**Sentence-final position** is the default focus: in VS clauses the new subject comes last (*higí'a ha-dóar* "the mail has arrived").

---

## References

Works marked *not held* are cited from their standard descriptions; they are not in the local library and are listed under «wanted» in [sources.md](sources.md).

- Aoun, Joseph, Elabbas Benmamoun and Dominique Sportiche. 1994. Agreement, word order, and conjunction in some varieties of Arabic. *Linguistic Inquiry* 25: 195–220. *Not held.*
- Ariel, Mira. 1990. *Accessing Noun-Phrase Antecedents*. London: Routledge. *Not held.*
- Berman, Ruth A. 1978. *Modern Hebrew Structure*. Tel Aviv: University Publishing Projects. *Not held.*
- Berman, Ruth A. and Alexander Grosu. 1976. Aspects of the copula in Modern Hebrew. In P. Cole (ed.), *Studies in Modern Hebrew Syntax and Semantics*. Amsterdam: North-Holland. *Not held.*
- Borer, Hagit. 1984. Restrictive relatives in Modern Hebrew. *Natural Language and Linguistic Theory* 2: 219–260. *Not held.*
- Borer, Hagit. 1989. Anaphoric AGR. In O. Jaeggli and K. Safir (eds.), *The Null Subject Parameter*. Dordrecht: Kluwer. *Not held.*
- Borer, Hagit and Yosef Grodzinsky. 1986. Syntactic cliticization and lexical cliticization: the case of Hebrew dative clitics. In H. Borer (ed.), *The Syntax of Pronominal Clitics* (Syntax and Semantics 19). *Not held.*
- Coffin, Edna Amir and Shmuel Bolozky. 2005. *A Reference Grammar of Modern Hebrew*. Cambridge: CUP. *Not held* (listed in `$RESEARCH_RAW/hebgram/grammars.tsv`).
- Danon, Gabi. 2001. Syntactic definiteness in the grammar of Modern Hebrew. *Linguistics* 39(6): 1071–1116. *Not held.*
- Danon, Gabi. 2006. Caseless nominals and the projection of DP. *Natural Language and Linguistic Theory* 24: 977–1008. *Not held.*
- Danon, Gabi. 2008. Definiteness spreading in the Hebrew construct state. *Lingua* 118: 872–906. *Not held.*
- Doron, Edit. 1982. On the syntax and semantics of resumptive pronouns. *Texas Linguistic Forum* 19. *Not held.*
- Doron, Edit. 1983. *Verbless Predicates in Hebrew*. PhD thesis, University of Texas at Austin. *Not held.*
- Doron, Edit. 2000. VSO and left-conjunct agreement: Biblical Hebrew vs. Modern Hebrew. In A. Carnie and E. Guilfoyle (eds.), *The Syntax of Verb Initial Languages*. OUP. *Not held.*
- Glinert, Lewis. 1989. *The Grammar of Modern Hebrew*. Cambridge: CUP. *Not held.*
- Goldberg, Lotus. 2005. *Verb-Stranding VP Ellipsis: A Cross-Linguistic Study*. PhD thesis, McGill. *Not held.*
- Hacohen, Aviya, Olga Kagan and Dana Plaut. 2021. Differential object marking in Modern Hebrew: definiteness and partitivity. *Glossa* 6(1): 148, 1–34. https://doi.org/10.16995/glossa.5729 — local: `$RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/`.
- Kastner, Itamar. 2017. Reflexive verbs in Hebrew: deep unaccusativity meets lexical semantics. *Glossa* 2(1): 75. Local: `$RESEARCH_RAW/lingbulk-2017-kastner-reflexive-verbs-hebrew-deep-unaccusativity/`.
- Landau, Idan. 1999. Possessor raising and the structure of VP. *Lingua* 107: 1–37. *Not held.*
- Landau, Idan. 2018. Missing objects in Hebrew: argument ellipsis, not VP ellipsis. *Glossa* 3(1): 76. Local: `$RESEARCH_RAW/lingbulk-2018-landau-missing-objects-hebrew-argument-ellipsis/`.
- Reinhart, Tanya and Tal Siloni. 2005. The lexicon-syntax parameter: reflexivization and other arity operations. *Linguistic Inquiry* 36: 389–436. *Not held.*
- Ritter, Elizabeth. 1991. Two functional categories in noun phrases: evidence from Modern Hebrew. In S. Rothstein (ed.), *Perspectives on Phrase Structure* (Syntax and Semantics 25). *Not held.*
- Shlonsky, Ur. 1987. *Null and Displaced Subjects*. PhD thesis, MIT. *Not held.*
- Shlonsky, Ur. 1992. Resumptive pronouns as a last resort. *Linguistic Inquiry* 23: 443–468. *Not held.*
- Shlonsky, Ur. 1997. *Clause Structure and Word Order in Hebrew and Arabic*. OUP. *Not held.*
- Shlonsky, Ur. 2004. The form of Semitic noun phrases. *Lingua* 114: 1465–1526. *Not held.*
- Shlonsky, Ur. 2009. Hebrew as a partial null-subject language. *Studia Linguistica* 63(1): 133–157. *Not held.*
- UD Hebrew documentation (word order, non-verbal clauses, relation and feature pages). https://universaldependencies.org/he/ — local: `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_he/`.
- UD 2.18 IAHLTwiki and IAHLTknesset (CC BY-SA 4.0) — counts made 2026-10-17; local: `$RESEARCH_RAW/lexres/treebanks/ud-2.18/`.
- Wintner, Shuly. 2000. Definiteness in the Hebrew noun phrase. *Journal of Linguistics* 36: 319–363. *Not held.* Related computational grammar: Wintner, Shuly. 1998. Linguistically motivated computational grammar for Hebrew — local: `$RESEARCH_RAW/aclbulk-1998-wintner-linguistically-motivated-computational-grammar-hebrew/`.
- Ziv, Yael. 1976. On the reanalysis of grammatical terms in Hebrew possessive constructions. In P. Cole (ed.), *Studies in Modern Hebrew Syntax and Semantics*. *Not held.*
- Ziv, Yael. 1994. Left and right dislocations: discourse functions and anaphora. *Journal of Pragmatics* 22: 629–645. *Not held.*
