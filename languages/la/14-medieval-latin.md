# 14. Medieval Latin

After the end of the Western Empire, Latin stops being anyone's mother tongue: the spoken language has become the early Romance dialects, and Latin is learnt at school. It remains for a thousand years the written language of the Church, the law, the chancery, the universities and science across Western Europe. Medieval Latin is not one language but a range: from the rough Latin of eighth-century notaries, written close to their speech, through the Carolingian reform that brought the written norm back towards the classical one, to the technical Latin of scholastic philosophy and the humanists' return to Cicero. This chapter describes that range, with the charters of the LLCT treebank as the measured example.

## Contents

1. The periods
2. The charters of Lombard Tuscany
3. Spelling
4. Morphology: the cases in disorder
5. Syntax and formulae
6. Vocabulary
7. Scholastic Latin
8. Humanist Latin and the end of medieval Latin
9. How Mova handles it

---

## 1. The periods

| period | centuries | what it is |
|---|---|---|
| Merovingian and Lombard Latin | 6th–8th | documents written near speech; spelling and cases unstable |
| the Carolingian reform | late 8th–9th | Alcuin and the palace school restore a classical norm; a fixed pronunciation for reading aloud cuts written Latin off from the Romance speech (the Council of Tours, 813, asks for sermons *in rusticam Romanam linguam*) |
| high medieval Latin | 10th–13th | the Latin of the schools and universities, of law (the revival of Roman law), of history and hymn; scholastic Latin |
| late medieval and humanist Latin | 14th–16th | the humanists (Petrarch, Valla, Erasmus) return to classical models and reject the medieval usage |

## 2. The charters of Lombard Tuscany

The LLCT treebank (*Late Latin Charter Treebank*, UD 2.18, CC BY-SA) is made of private charters written in Lucca and its region in the eighth and ninth centuries: sales, exchanges, gifts and leases of land, each with its date, its parties, its witnesses and the notary who wrote it. It has 194,158 syntactic words in its training set (counted 2026-10-22). A charter opens with a dating formula:

> *In Dei nomine regnante domno nostro Carulo rege Francorum et Langobardorum, anno regni eius quo coepit Langobardiam primo, septimo decimo kalendas augustas, per indictionem duodecimam.*
> "In the name of God, in the reign of our lord Charles, king of the Franks and the Lombards, in the first year of his reign since he took Lombardy, on the seventeenth day before the Kalends of August, in the twelfth indiction."

The date can be read exactly with the rules of [chapter 11](11-numerals-lexicon.md):
- Charlemagne took Lombardy in 774, so the first year of his reign is 774–775;
- the seventeenth day before the Kalends of August, counted inclusively, is 16 July;
- the twelfth indiction is the year 774, since (774 + 3) mod 15 = 12.

The three dating systems agree: 16 July 774.

The charter closes with the signatures:

> *signum + manus Silvipertis filio quondam Silboli, testis.* "The sign of the hand of Silvipert, son of the late Silbol, witness."
> *+ Ego Airualdu presbiter rogatus ad Causualdu presbiter me teste subscripsi.* "I, Airuald the priest, asked by Causuald the priest, have signed as witness."

The most frequent lemmas show what the texts are about: *rogo* "ask" (2,021: the parties *ask* the notary to write), *testis* "witness" (1,925), *casa* "house, holding" (1,287), *notarius* (1,054), *terra* (986), *solidus* "shilling" (306).

## 3. Spelling

Spelling follows speech in the early documents:
- *e* and *i*, *o* and *u* interchange (*nomen / nomin*, *Carulo* for *Carolo*);
- *ae* is *e* (*ecclesie* for *ecclesiae*);
- *b* and *v* interchange (*bendo* for *vendo*);
- final *-m* is dropped or wrongly added (*Causualdu* for *Causualdum*);
- *ti* and *ci* interchange before a vowel (*tercius, cambiationis*);
- *h* is dropped or added (*abere, hordo*).

Germanic names are latinized with endings that are often only graphic (*Silvipertis*, *Airualdu*).

## 4. Morphology: the cases in disorder

The case after a preposition and in apposition follows no fixed rule in the less learned charters. In the example above:
- *filio* (a dative/ablative form) stands in apposition to the genitive *Silvipertis*;
- *ad Causualdu presbiter* has the preposition *ad* with a name ending in *-u* (for *-um*) and a title in the nominative.

On our counts (UD 2.18 training sets, 2026-10-22; [chapter 8](08-prepositions-particles.md), §10):

| | classical (CIRCSE) | charters (LLCT) |
|---|---:|---:|
| *ad* + ablative-marked noun | 0 of 51 | 474 of 2 080 |
| *cum* + accusative | 0 | 166 of 657 |
| subjunctive share of finite verbs | 12.5% | 12.8% |

The annotators of LLCT give each word the case of its form, not the case the grammar expects: the counts measure the writing, not a normalization of it.

## 5. Syntax and formulae

The charters are written in **formulae**: fixed sequences that the notary copied from models (the formularies) and filled with the names, the places and the date. Within the formulae the syntax is stable; around them, the notary's own Latin shows the spoken language:
- *de* + noun for the genitive (*de parte mea*), *ad* + noun for the dative;
- *quod* and *qualiter* clauses for reported content;
- *habere* with a participle (*habeo scriptum*).

The order of subject, verb and object spreads over all six orders ([chapter 9](09-simple-clause.md), §6), because the formulae fix their own orders: *constat me … vendere*, *scribere rogavi*.

## 6. Vocabulary

- **Words of the land and the law:** *casa* "holding", *massa* "estate", *curtis* "manor", *livellum* "lease", *cartula* "charter", *launegild* (a Lombard counter-gift).
- **Germanic loans:** *werra* "war", *feudum* "fief", *bannum*, *marca* "border", *wadia* "pledge".
- **New derivations** in *-agium, -aticum* (*pedagium* "toll"), *-arius* (*notarius, camerarius*), *-tio*, and verbs in *-izare*.
- **Christian vocabulary** throughout: *presbiter, diaconus, ecclesia, monasterium, abbas*.

## 7. Scholastic Latin

The Latin of the universities (from the 12th century: Abelard, Aquinas, Ockham) is precise, technical and plain. It builds abstract nouns freely (*quidditas* "whatness", *haecceitas* "thisness", *entitas*), uses *quod* clauses for reported content, and orders its argument in fixed forms (*videtur quod … sed contra … respondeo dicendum quod …*). Its word order is close to the Romance one. For a parser it is easier than Cicero, and its arguments are close to the frames Mova reads: objection, counter-argument, answer.

## 8. Humanist Latin and the end of medieval Latin

From the 14th century the humanists judged medieval Latin barbarous and wrote on classical models again, in vocabulary, syntax and spelling (*ae* restored, *h* in its place). Neo-Latin, the Latin of science and scholarship until the 18th century (Newton's *Principia*, Linnaeus) is their heir. Church Latin keeps its own pronunciation and usage to this day.

## 9. How Mova handles it

**The texts and the trees.**
- The LLCT treebank is open (CC BY-SA) and is Mova's main Latin measure: train → dev LAS 87.35 with its own tags, 87.80 with the gold tags (2026-10-22, `en xpos-fill`).
- The open parsers trained on it reach 95.92 (Stanza) and 95.03 (UDPipe 2) ([docs/parsers.md](../../docs/parsers.md)).

**What a medieval Latin reader needs:**
- **Spelling variants** reduced to one lemma (*bendo / vendo*, *Carulo / Carolo*) before the lexicon is looked up: a table of the interchanges of §3, applied as alternatives;
- **the case from the form, the role from the formula:** in a charter the formula says who sells, who buys, who witnesses, whatever the endings say;
- **the dates of §2** read into calendar dates. The three dating systems agree or they do not, and the disagreement is itself a fact (a copy, a forgery, a scribe's slip).

Each charter is a small world:
- the parties, their kin (*filio quondam* …), the land with its bounds, the price, the witnesses, the date;
- the same kinds of fact the frames reader builds for English, and checkable against each other across a corpus of hundreds of charters from one region.
