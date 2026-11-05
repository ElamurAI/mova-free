# The cases and their uses

Greek has five cases: nominative, vocative, accusative, genitive and dative. Indo-European had eight. The Greek genitive took over the ablative (separation, "from"), and the Greek dative took over the instrumental ("with, by") and the locative ("in, at"). Each Greek case therefore carries several old functions, and the prepositions sort them out ([chapter 12](12-prepositions-particles.md)). This chapter gives the uses of each case, grouped by those old functions. It compares them with Latin, which kept the ablative ([la chapter 7](../la/07-cases.md)), and counts how the cases fall into UD relations in the Septuagint.

## Contents

1. The five cases and the eight functions
2. Nominative and vocative
3. Accusative
4. Genitive
5. Dative
6. Place, time and the old locative
7. The cases in the treebank
8. How Mova handles it
9. Sources

---

## 1. The five cases and the eight functions

| Greek case | old functions it carries | Latin counterpart |
|---|---|---|
| nominative | subject, predicate | nominative |
| vocative | address | vocative |
| accusative | direct object, extent, direction, respect | accusative |
| genitive | genitive proper (of belonging, part, quality) **and** ablative (separation, source, comparison) | genitive and ablative of separation |
| dative | dative proper (recipient, interest) **and** instrumental (means, manner, accompaniment) **and** locative (place, time) | dative and ablative of means, place and time |

(smyth-1920 §§1279–1282)

## 2. Nominative and vocative

The **nominative** is the case of the subject and of the predicate with verbs of being, becoming, seeming, being named (smyth-1920 §§938–940): Κῦρος βασιλεὺς ἐγένετο "Cyrus became king". The article tells the subject from the predicate when both are nominatives ([chapter 5](05-article.md), §5).

The **vocative** is the case of address, often with ὦ (smyth-1920 §§1283–1288): ὦ ἄνδρες Ἀθηναῖοι "men of Athens". It has its own form only in the singular of some nouns (Σώκρατες, ἄνθρωπε); elsewhere it is the nominative form.

## 3. Accusative

The accusative is the case of the **direct object** and of what the action reaches or extends over (smyth-1920 §§1551–1633):
- **direct object**: ὁρῶ τὸν ἄνδρα "I see the man";
- **internal object** (cognate accusative), the content of the verb: μάχην μάχεσθαι "fight a battle", νικᾶν τὴν μάχην "win the battle";
- **double accusative**: of the person and the thing with verbs of asking, teaching, depriving: διδάσκω τοὺς παῖδας τὴν μουσικήν "I teach the boys music"; of the object and the predicate with verbs of making, naming, choosing: στρατηγὸν αὐτὸν ἀπέδειξε "he appointed him general";
- **respect**: καλὸς τὸ εἶδος "beautiful in appearance", πόδας ὠκὺς Ἀχιλλεύς "swift-footed Achilles";
- **extent** of space and time: ἔμεινε τρεῖς ἡμέρας "he stayed three days", ἀπέχει δέκα σταδίους "it is ten stades away";
- **adverbial accusatives**: τὴν ταχίστην "by the quickest way", τέλος "finally", χάριν "for the sake of";
- **the subject of an infinitive** ([chapter 10](10-nonfinite.md), §2).

## 4. Genitive

**The genitive proper** limits a noun, as an adjective would (smyth-1920 §§1289–1338):
- **possession** and belonging: ἡ τοῦ πατρὸς οἰκία "the father's house"; with the article alone, τὰ τῆς πόλεως "the affairs of the city";
- **the subject and the object** of a verbal noun: ὁ τῶν πολεμίων φόβος "the fear of the enemy" (they fear, or we fear them);
- **the part** of a whole (partitive): οἱ πλεῖστοι τῶν στρατιωτῶν "most of the soldiers"; also with verbs of sharing, touching, taking hold, eating: ἅπτεσθαι τῆς χειρός "to take hold of the hand", ἐσθίειν τοῦ ἄρτου "to eat (some) of the bread";
- **material, measure, quality**: στέφανος ῥόδων "a crown of roses", ὀκτὼ σταδίων τεῖχος "a wall of eight stades";
- **price and value**: πέντε δραχμῶν ἀγοράζειν "to buy for five drachmas";
- **with verbs of remembering, forgetting, caring, ruling, desiring**: μέμνημαι τοῦ λόγου "I remember the word", ἄρχει τῆς πόλεως "he rules the city" (smyth-1920 §§1339–1449).

**The ablatival genitive**: separation, source and comparison:
- **separation**: παύω τινὰ τοῦ λέγειν "I stop someone from speaking"; ἀπέχειν τῆς πόλεως "to be away from the city";
- **source and origin**: Δαρείου καὶ Παρυσάτιδος γίγνονται παῖδες δύο "of Darius and Parysatis are born two sons" (Xen. *An.* 1.1.1);
- **comparison**: μείζων τοῦ πατρός "bigger than his father" (or μείζων ἢ ὁ πατήρ, with ἤ and the same case).

**The genitive of time within which**: νυκτός "by night", τριῶν ἡμερῶν "within three days" (§6).

## 5. Dative

**The dative proper**: the person or thing interested in the action (smyth-1920 §§1450–1550):
- **indirect object**: δίδωμι τὸ βιβλίον τῷ παιδί "I give the book to the boy";
- **interest, advantage and disadvantage**: ἄλλῳ ὁ τοιοῦτος πλουτεῖ "such a man is rich for another";
- **possession** with εἶναι: ἔστι μοι βιβλίον "I have a book" ([la chapter 9](../la/09-simple-clause.md), §5);
- **the ethical dative**: μή μοι θορυβήσητε "don't, I beg you, make a noise";
- **the agent of the perfect passive and the verbal adjective**: ταῦτα πέπρακταί μοι "this has been done by me"; λυτέον ἐστί σοι ([chapter 10](10-nonfinite.md), §9);
- **with adjectives and verbs of likeness, fitness, help, obedience**: ὅμοιος τῷ πατρί "like his father", πείθεσθαι τοῖς νόμοις "to obey the laws".

**The instrumental dative**:
- **means and instrument**: βάλλειν λίθοις "to pelt with stones";
- **cause**: φόβῳ "out of fear";
- **manner**: σιγῇ "in silence", βίᾳ "by force";
- **accompaniment**, chiefly of troops: ἀφίκοντο τοῖς ἵπποις "they came with their horses";
- **the degree of difference** with comparatives: πολλῷ μείζων "much bigger" (bigger by much).

**The locative dative**: place (in poetry and in names: Μαραθῶνι "at Marathon") and **time at which**: τῇ ὑστεραίᾳ "on the next day" (§6).

## 6. Place, time and the old locative

| | place | time |
|---|---|---|
| accusative | to which (poetry; prose uses εἰς, πρός) | how long: τρεῖς ἡμέρας "for three days" |
| genitive | (ablatival) from which (poetry); within which: τῆς ὁδοῦ "on the road" | within which: νυκτός "by night", τοῦ λοιποῦ "in future" |
| dative | at which: Μαραθῶνι, οἴκοι (locative form) | at which: τῇ αὐτῇ ἡμέρᾳ "on the same day" |

A few old locatives survive: οἴκοι "at home", Ἀθήνησι "at Athens", Πυθοῖ "at Delphi"; and adverbs of place in -θεν "from", -δε "to": οἴκοθεν "from home", οἴκαδε "homewards", Ἀθήναζε "to Athens" (smyth-1920 §§342–344, 1531–1548).

## 7. The cases in the treebank

How the cases of nouns and names fall into UD relations in the PTNK training set (Genesis and Ruth, UD 2.18, counted 2026-10-22):

| case | main relations |
|---|---|
| nominative | `nsubj` 720, `conj` 113, `appos` 90, `obl` 48, `nsubj:pass` 36 |
| accusative | `obj` 508, `obl` 213, `conj` 92, `appos` 37 |
| genitive | `nmod:poss` 585, `obl` 192, `appos` 50, `conj` 29 |
| dative | `obl` 204, `iobj` 155, `nmod` 33, `conj` 31 |

The accusative is an object two times in three and otherwise `obl` (with a preposition or of extent). The genitive is a possessor three times in four. Here the Septuagint follows the Hebrew construct chain and uses the genitive more than Attic does (υἱοὶ Ἰσραήλ "the sons of Israel"). The dative is `obl` (with ἐν and other prepositions, or of means) more than an indirect object.

## 8. How Mova handles it

**In the UD trees.** The case is in the noun's `FEATS`. The relation (`obj`, `iobj`, `obl`, `nmod`) is the function. A case does not decide its relation: an accusative may be an object or extent, a genitive a possessor, a partitive object or a separation. In PTNK the ambiguous forms keep the set of cases (`Case=Acc,Nom`: [chapter 3](03-noun.md)).

**In Mova.** As for Latin, the tag set `en xpos-fill` maps to has no case, so the parser sees only the word and its English-style tag, and must learn the relation from the order and the words around. For Greek, whose order is freer than the Septuagint's, this is the main loss: the PTNK baseline is LAS 74.41 with Mova's own tags and 78.29 with gold tags (dev, 2026-10-22), well below its Latin LLCT score. What a Greek frames reader needs from this chapter:
1. **The case in the parser's input**, as a feature beside the tag. This is the step that would help Latin and Greek alike, and it is the next one to measure.
2. **The functions of §1 read from case and relation together**: an `obl` genitive with a verb of separating is a source, a dative `obl` with no preposition is a means or a time, an `obl` accusative of a time noun is extent.
3. **The genitive of comparison** read as the second term of a comparison, like *than*.

## 9. Sources

- Smyth, H. W. 1920 (smyth-1920): §§1279–1635 (the cases), §§342–344 (adverbs of place), paraphrased from the Perseus text (CC BY-SA).
- Goodwin, W. W. 1892 (goodwin-1892): the cases, public domain.
- Xenophon, *Anabasis* 1.1.1 (Perseus canonical-greekLit, CC BY-SA).
- UD Ancient Greek PTNK (CC BY-SA 4.0): counts.
