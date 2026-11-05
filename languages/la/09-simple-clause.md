# 9. The simple clause: agreement and word order

A Latin clause is held together by agreement and by the cases, not by the order of its words. The verb agrees with its subject in person and number, an adjective or participle agrees with its noun in gender, number and case, and each noun phrase shows its function by its ending. The order of words is therefore free in the sense that it does not decide who did what to whom. It is not free in the sense of random: it shows what is given and what is new, what is stressed, and the style of the text and of the period. This chapter gives agreement, the subject and the verb, the copula, the impersonal constructions, and the order of words from classical prose to the Vulgate and the medieval charters.

## Contents

1. Agreement
2. The subject, and its absence
3. The copula and its omission
4. Impersonal verbs and the impersonal passive
5. Possession: *habēre*, the dative, the genitive
6. Word order in the clause
7. Word order in the phrase
8. Linking the sentences
9. How Mova handles it

---

## 1. Agreement

**Verb and subject.** The finite verb agrees with its subject in person and number: *puer currit, puerī currunt*. With subjects joined by *et*, the verb is plural, and takes the first person over the second, the second over the third: *ego et tū valēmus*. It may agree with the nearest subject alone, especially when it comes first: *mōvit et Caesar et exercitus*. A collective noun may take a plural verb by sense (*constructio ad sensum*): *pars mīlitum fūgērunt* (Bennett 1918 §§234–235, 250).

**Adjective and noun.** An adjective, participle or pronoun agrees with its noun in gender, number and case, wherever it stands in the clause ([chapter 3](03-adjectives.md)). With nouns of different genders, a predicate adjective for persons is masculine, for things neuter plural: *pater et māter mortuī sunt; honōrēs et victōriae fortuita sunt*.

**Apposition.** A noun in apposition agrees in case: *Rōma, caput orbis*; *urbs Rōma*.

**The relative pronoun** agrees with its antecedent in gender and number and takes its case from its own clause ([chapter 4](04-pronouns.md)).

## 2. The subject, and its absence

The verb ending shows the person and number of the subject, so a pronoun subject is left out unless it is stressed or contrasted: *amō* "I love"; ***ego*** *amō, tū rīdēs* "*I* love, *you* laugh". Latin is a null-subject language, in this unlike French ([fr chapter 10](../fr/10-clause-structure.md)) and like Italian and Spanish. A third person subject is understood from the context: *Caesar ... castra posuit. Postrīdiē ... profectus est.*

## 3. The copula and its omission

*Esse* joins a subject to a predicate noun or adjective, both in the nominative: *Rōma urbs magna est*. The copula is often left out, above all:
- in proverbs and maxims: *Omnia praeclāra rāra.* "All splendid things are rare.";
- in headings, exclamations and lively narrative;
- in the present, with predicate adjectives and participles: *dictum factum* "no sooner said than done".

The copula with the dative of possession (§5), the predicative dative (*mihi cūrae est* "it is a concern to me") and the genitive of quality (*vir magnī ingenī*) are in [chapter 7](07-cases.md).

## 4. Impersonal verbs and the impersonal passive

**Impersonal verbs** have no subject but the clause or infinitive that follows (Bennett 1918 §138):
- of weather: *pluit, ningit, tonat*;
- of feeling, with the accusative of the person and the genitive of the cause: *mē paenitet erroris* "I regret my mistake", *pudet, piget, taedet, miseret*;
- of necessity and propriety: *oportet, decet, licet, libet*;
- *interest, rēfert* "it matters" with *meā, tuā* or the genitive.

**The impersonal passive** of an intransitive verb names an action without a subject: *pugnātur* "there is fighting", *ītur* "people go", *diū et ācriter pugnātum est* "the fighting was long and fierce" (literally "it was fought"). It is common in Caesar and the historians.

## 5. Possession: *habēre*, the dative, the genitive

Possession is expressed three ways (Bennett 1918 §§190, 198, 359):
- *habēre*: *Marcus librum habet*;
- the dative with *esse*, which stresses the thing possessed: *Marcō liber est* "Marcus has a book" (literally "to Marcus there is a book");
- the genitive with *esse*, which stresses the owner: *liber Marcī est* "the book is Marcus'".

The *habēre* construction spreads in late Latin and gives the Romance *avoir/avere/haber*, and with a past participle the perfect (*habeō scrīptum* → French *j'ai écrit*) ([chapter 13](13-late-latin.md)).

## 6. Word order in the clause

**The classical prose norm** places the verb at the end of the clause, the subject first, the object before the verb (SOV). What comes first is the topic, what comes just before the verb carries the focus, and the verb at the end closes the clause. The norm is a strong tendency, not a rule. A verb placed first marks a vivid event or a new paragraph; a word placed at the head of the clause, or separated from its noun, is stressed (Bennett 1918 §§348–349).

The order changes with the genre and the period. We counted the order of subject, verb and object in main clauses whose verb has a noun subject and a direct object (UD 2.18 training sets, counted 2026-10-22):

| text | treebank | SOV | SVO | OSV | OVS | VSO | VOS | clauses |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Caesar, *Gallic War* | PROIEL (NC, counted only) | **53%** | 10% | 9% | 14% | 8% | 6% | 217 |
| Seneca's tragedies, Tacitus' *Germania* | CIRCSE | 22% | 20% | 20% | 16% | 11% | 11% | 167 |
| the Vulgate New Testament | PROIEL (NC, counted only) | 24% | **44%** | 5% | 6% | 8% | 13% | 1 080 |
| Lombard charters, 8th–9th century | LLCT | 26% | 23% | 7% | 14% | 16% | 14% | 273 |

Caesar's prose is verb-final in more than half the clauses. Verse (Seneca) uses every order nearly equally, because metre and art decide. The Vulgate is SVO in nearly half, following its Greek and Hebrew sources and the spoken Latin of its time: the road to the Romance order. The charters, full of formulae with the verb first (*constat me … vendere*), spread over all orders.

## 7. Word order in the phrase

- **The adjective** usually follows its noun in classical prose when it describes and precedes when it classifies or is stressed: *vir bonus* but *bonus vir* "a *good* man"; numerals and quantity words precede (*multī hominēs*) (Bennett 1918 §350).
- **The genitive** may precede or follow: *Caesaris exercitus, exercitus Caesaris*.
- **Hyperbaton**: an adjective separated from its noun by other words, for emphasis or art: *magnā cum laude*, *Aquītānia ... ad Pȳrēnaeōs montēs ... pertinet*. Poetry takes it much further, interlacing two noun phrases (*aurea purpuream subnectit fibula vestem*, Verg. *Aen.* 4.139: "a golden brooch fastens the purple robe", the two adjectives first and the two nouns last). The frame *adjective ... verb ... noun* closes a line or a sentence.
- **Second-position words** (*autem, enim, vērō, quidem, igitur*) follow the first word or phrase of the clause ([chapter 8](08-prepositions-particles.md)).

## 8. Linking the sentences

A Latin text links each sentence to the one before:
- with a particle (*enim, autem, igitur, itaque, nam, tamen*: [chapter 8](08-prepositions-particles.md));
- with the relative connection, a relative pronoun at the head of the sentence standing for a demonstrative: ***Quae*** *cum ita sint* "Since these things are so", ***quō*** *factō* "when this was done";
- with a participle or an ablative absolute that sums up the last event: ***Hīs rēbus gestīs*** *Caesar ...* ([chapter 6](06-nonfinite.md)).

Asyndeton, sentences without any link, marks lists, haste and emotion: *vēnī, vīdī, vīcī*.

## 9. How Mova handles it

**In the UD trees:**
- agreement is in the `FEATS` of each word (`Case`, `Gender`, `Number`, `Person`);
- the relations do not depend on the order;
- an unexpressed subject has no node (UD has no empty pronoun);
- an impersonal passive has no `nsubj`, and its verb carries `Voice=Pass`.

**In Mova.** The parser Mova uses for Latin is its English one after `en xpos-fill`, and it learns the Latin attachments from the trees (LLCT dev LAS 87.35, 2026-10-22). Two things it cannot do well without a Latin morphology:
- **Find the subject and the object in a free order.** An English-style parser leans on order. In Latin the case decides, and the case is not in the tags `en xpos-fill` gives. That is the main reason Latin is harder for Mova on classical and poetic texts than on the LLCT charters, whose formulae repeat (a measurement on CIRCSE is still to make).
- **Supply the missing subject** from the verb's person. The frames reader needs it to know who acts in *amō* and in *profectus est*: the verb's `Person` and `Number` (in the trees) and the discourse context (the English reader already carries a subject across clauses).

A Latin frames reader also needs:
- the impersonal verbs and the impersonal passive read as events with no agent;
- the dative of possession read as possession (§5);
- the relative connection (§8) read as a link to the previous sentence, not as a relative clause.

The word order counts of §6 are measured by a small script on the treebanks; they are a stylometric signal Mova can compute for any parsed Latin text (period, genre, translation from Greek).
