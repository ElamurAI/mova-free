# 7. Auxiliaries, compound tenses, the past participle, the passive

Every compound tense of French is formed with an auxiliary, *avoir* or *être*, and a past participle. The choice of the auxiliary is not free, and the past participle agrees, or does not, according to rules that are among the best known difficulties of French orthography. The same auxiliary *être* forms the passive and the compound tenses of the pronominal verbs. This chapter gives the choice of the auxiliary, the compound tenses, the rules of the agreement of the participle with their reasons, the passive and the causative construction.

## Contents

1. *Avoir* and *être*
2. The compound tenses
3. The agreement of the past participle
4. Pronominal verbs
5. Participles with an infinitive, impersonal verbs and measure
6. The passive
7. Causatives and verbs of perception
8. How Mova handles it

---

## 1. *Avoir* and *être*

*Avoir* is the auxiliary of the transitive verbs and of most intransitive ones: *J'ai mangé*, *Il a dormi*, *Elles ont ri*, and of *avoir* and *être* themselves (*J'ai eu*, *J'ai été*). *Être* is the auxiliary of two groups.

- **A closed list of intransitive verbs of motion or change of state**: *aller*, *venir*, *arriver*, *partir*, *entrer*, *sortir*, *monter*, *descendre*, *rester*, *tomber*, *naître*, *mourir*, *devenir*, *revenir*, *retourner*, *rentrer*, *passer*, with their compounds (*parvenir*, *intervenir*, *décéder*). The list is remembered by the mnemonic of the English textbooks ("Dr and Mrs Vandertramp") and the verbs are *Il est arrivé*, *Elle est née en 1968*, *Chypre est entré en zone euro* (fr-ud-dev_00869), *Il est né le 1er février 1968* (fr-ud-dev_01108), *le modèle est resté sur place* (fr-ud-dev_00831), *est sorti en single* (fr-ud-dev_00631).
- **All pronominal verbs**: *Elle s'est levée*, *Ils se sont parlé*, *Les compétitions se sont déroulées* (fr-ud-train_05273).

Verbs of the first group that are also transitive take *avoir* when they have a direct object: *Elle est sortie* but *Elle a sorti la voiture*; *Il est monté* but *Il a monté l'escalier*; *Il est descendu* but *Il a descendu la valise*; *Il est passé* but *Il a passé l'examen*. A few verbs hesitate between the two (*demeurer*).

## 2. The compound tenses

| tense | formation | example |
|---|---|---|
| passé composé | auxiliary in the present + participle | *il a parlé*, *elle est partie* |
| plus-que-parfait | auxiliary in the imperfect + participle | *il avait parlé* |
| futur antérieur | auxiliary in the future + participle | *il aura parlé* |
| conditionnel passé | auxiliary in the conditional + participle | *il aurait parlé* (*Il aurait aimé*, fr-ud-dev_01092) |
| passé antérieur | auxiliary in the passé simple + participle | *il eut parlé* |
| subjonctif passé | auxiliary in the present subjunctive + participle | *qu'il ait parlé* |
| subjonctif plus-que-parfait | auxiliary in the imperfect subjunctive + participle | *qu'il eût parlé* |
| infinitive past | infinitive of the auxiliary + participle | *avoir parlé*, *être parti* |
| participle past | present participle + participle | *ayant parlé*, *étant parti* |

The **passé composé** is the ordinary past of speech: it expresses a completed event (*Il est arrivé hier*) or a state resulting from it (*J'ai fini*). There are two further forms, the *surcomposé* in speech (*Quand il a eu fini*) and the recent past with *venir de* ([ch. 6](06-verb.md), §8). The participle is **adjacent** to the auxiliary: the negation surrounds the auxiliary (*Il n'a pas parlé*), short adverbs stand between them (*Il a déjà fini*, *Il a bien parlé*), clitics are before the auxiliary (*Il me l'a dit*, fr-ud-test_00087), and in a question by inversion the subject clitic follows the auxiliary (*A-t-il fini ?*).

## 3. The agreement of the past participle

The past participle agrees in gender and number in three kinds of context.

1. **With *être*, the participle agrees with the subject**: *Elle est partie*, *Ils sont arrivés*, *Les compétitions se sont déroulées*. (This is not the case for pronominal verbs where the *se* is an indirect object: §4.)
2. **With *avoir*, the participle agrees with a direct object that precedes it, and only then.** The preceding object may be:
   - a clitic: *Je l'ai vue* (*l'* = a woman), *Je les ai laissés*, *Certains les ont même vus* (fr-ud-train_00882, with the participle in the masculine plural, agreeing with *les*);
   - the relative *que*: *la maison que j'ai vue*, *les lettres qu'il a écrites*;
   - an interrogative or exclamative object before the verb: *Quelle maison as-tu vue ?*, *Combien de livres as-tu lus ?*;
   - another preposed direct object (*Ces livres, je les ai lus*).
   
   If the direct object follows the verb or there is none, the participle stays masculine singular: *J'ai vu la maison*, *Elle a parlé*, *Ils ont mangé des pommes*.
3. **The participle used as an adjective** (in an absolute clause, after a noun, as a predicate adjective with *être* in the sense of a state, in front of a noun) agrees like an adjective: *une table cassée*, *les pays concernés*, *Passés les premiers jours*.

**What is invariable.** With *avoir*, the participle is invariable if the object is *en* (*Des pommes, j'en ai mangé*), though a case with *en* and a quantity word that precedes the participle is seen in the treebank with agreement: *On en avait vue une dizaine* (fr-ud-dev_00067). A participle with the impersonal subject *il* does not agree (*les chaleurs qu'il a fait*), nor does the participle of a verb with a clause or an infinitive as the object (*La maison qu'il a dit qu'il achèterait*); the one of a verb of measure (*les dix kilomètres qu'il a couru*, *les mille euros que ce livre m'a coûté*) does not agree when the verb is used in the sense of measure, but it agrees in the figurative one (*les dangers qu'il a courus*).

**Why it matters.** In speech the agreement is nearly always inaudible: *vu*, *vue*, *vus*, *vues* are one word /vy/, and *parlé*, *parlée*, *parlés*, *parlées* one word /paʁle/. The sound differs only for the participles that end in a consonant in the feminine: *mis* /mi/, *mise* /miz/; *pris* /pʁi/, *prise* /pʁiz/; *fait* /fɛ/, *faite* /fɛt/; *ouvert* /uvɛʁ/, *ouverte* /uvɛʁt/; *écrit* /ekʁi/, *écrite* /ekʁit/. This is why the agreement is part of the "spelling" taught in school and why mistakes occur in writing: in the treebank, *ses paroles ont conduits ses compatriotes à la victoire* (fr-ud-train_03882) has the participle in the plural, agreeing with the subject, which is a writing error.

## 4. Pronominal verbs

The reflexive pronoun *se* is compatible with three situations, and the agreement follows it.

- **The pronoun is the direct object**: the participle agrees with it (and so with the subject, since they refer to the same entity). *Elle s'est lavée*, *Ils se sont vus*, *Elle s'est blessée*. In *C'est à croire qu'ils se sont consultés* (fr-ud-train_06603), the *se* is an `obj` in the treebank and the participle is in the plural.
- **The pronoun is an indirect object**: the participle is invariable. *Ils se sont parlé*, *Elles se sont téléphoné*, *Six directeurs se sont succédé* (fr-ud-train_02372). If the verb has a direct object, *se* is an indirect object: *Elle s'est lavé les mains*, *Elle s'est cassé la jambe*, the participle does not agree with the object that follows it; but if the object precedes, it agrees with that object (*Les mains qu'elle s'est lavées*).
- **The verb is essentially pronominal** (it exists only in the reflexive form, the pronoun having no function of its own): the participle agrees with the subject. *Elle s'est évanouie*, *Ils se sont souvenus*, *Elle s'est enfuie*, *Les compétitions se sont déroulées au vélodrome* (fr-ud-train_05273). Exceptions: *se rire de*, *se plaire* (*Elles se sont plu*), *se déplaire*, *se complaire*, *se succéder*, *s'en vouloir*, *s'arroger* (a direct object after) are invariable.

The **1990 rectification** records that the participle of *laisser* followed by an infinitive is invariable, including in the pronominal use: *Elle s'est laissé mourir* (cslf-1990 section 6, [ch. 1](01-sounds-and-spelling.md) §8); it leaves the other participle agreements as they were (the report says it did not touch the agreement of the pronominal participle in general, because the question reaches beyond spelling into syntax and pronunciation).

## 5. Participles with an infinitive, impersonal verbs and measure

- ***Faire* followed by an infinitive**: *fait* is always invariable (*Elle s'est fait maigrir*, *Je les ai fait partir*).
- **Verbs of perception and *laisser* followed by an infinitive**: the participle agrees with the preceding object only if that object does the action of the infinitive (*Les enfants que j'ai vus jouer*, "the children that I saw play") and not if it undergoes it (*Les chansons que j'ai entendu chanter*, "the songs I heard sung"). *Laissé* is invariable in every case since the rectification, though the traditional rule applies the same test to it.
- **Impersonal verbs and *il y a***: the participle is invariable (*les neiges qu'il y a eu*).
- **Participles that precede the noun**: *Étant donné*, *vu*, *excepté*, *compris* (*y compris*), *passé* are invariable before the noun and agree after it (*vu les circonstances* but *les circonstances vues*; *Passé minuit* vs *Passés les premiers jours*).

## 6. The passive

The passive is formed with *être* and the past participle, which agrees with the subject: *Il est acheté par le collectionneur* (fr-ud-dev_00640), *Elle est nommée ministre de la Culture* (fr-ud-dev_01095), *Les écuries ne furent jamais réalisées* (fr-ud-train_03540), *Le Soudan va être divisé en deux* (fr-ud-train_04657). The agent is introduced by *par*, or by *de* with verbs of feeling and state (*aimé de tous*, *entouré de ses amis*). The tenses of the passive are those of *être* plus the participle; the compound tenses have *été*: *Il a été acheté*, *Il avait été conduit*, *où ils ont été conduits* (fr-ud-train_03972).

**Restrictions.** Only a verb with a **direct** object has a passive of this kind: the direct object becomes the subject. The indirect object does not: the English *He was given a book*, *She was told that…* has no direct counterpart (**Il a été donné un livre* is not French; the active *On lui a donné un livre* or the impersonal *Il lui a été donné un livre*, in formal style, are used). Verbs with *en* and *y* do not have it. For many verbs the passive is a written-language form, and speech uses *on* (*On l'a acheté*), the reflexive passive (*Ça se vend bien*), or the active.

**The three readings of *être* + participle**: an action passive (*La porte est ouverte par le gardien chaque matin*), a state (*La porte est ouverte*, "is open"), and a compound tense of an *être* verb (*Il est parti*); the distinction is a matter of the verb and of the context. The treebank separates the passive (`Voice=Pass`, relation `aux:pass`) from the compound tense (`aux:tense`).

## 7. Causatives and verbs of perception

*Faire* and *laisser* with an infinitive form a **causative** structure, with the subject of the infinitive as an object of the main verb: *Je fais lire les enfants*, *Je les fais lire*. The causee is in the accusative when the infinitive has no object (*Je les fais rire*) and in the dative (introduced by *à* or shown by *lui*, *leur*) when it has one (*Je fais lire un livre aux enfants*, *Je leur fais lire un livre*). The clitics are before *faire* (*Ses enfants le lui font remarquer*, fr-ud-train_04603). The verbs of perception (*voir*, *entendre*, *sentir*, *regarder*) and *laisser* work similarly, with a choice of the form: *Je l'ai vu partir*. *Faire* with a pronominal infinitive loses *se*: *Je les fais asseoir*.

## 8. How Mova handles it

**What is done.** The compound tenses and the passive are tree shapes that the parser learns from the treebank. In UD French GSD (UD 2.18, three files, 2026-10-22):

- the relation `aux:tense` is on 3 837 tokens: 3 339 forms of *avoir* and 498 of *être* (the *être* auxiliary of intransitive verbs and of pronominal verbs); so the tag `AUX` is mostly *avoir*;
- the passive is `aux:pass` on 3 401 tokens (3 380 *être* and 21 *faire*), with `nsubj:pass` for the subject on 3 663 tokens and `obl:agent` for the *par*-phrase on 1 554: *Il est acheté par le collectionneur* (fr-ud-dev_00640) has *il* as `nsubj:pass`, *est* as `aux:pass`, *acheté* as the head with `Voice=Pass`, and *collectionneur* as `obl:agent`;
- the copula *être* with an adjective, noun or adverb predicate is `cop` (5 560 tokens), and the head is the predicate: the difference between a passive and a copular state is made by the annotator, by `Voice=Pass`: *Il est acheté* against *Il est grand*;
- **the causative** *faire* is `aux:caus` (250 tokens), the causee is `nsubj:caus` (132) or `iobj:agent` (24): *Ses enfants le lui font remarquer* (fr-ud-train_04603) has *remarquer* as root, *font* as `aux:caus`, *enfants* as `nsubj:caus`, *le* as `obj` and *lui* as `iobj:agent`;
- the participle is the **head** of the compound tense, the auxiliary its dependent, and clitics and negation attach to the participle (the verb of the semantic predicate): *Il me l'a dit* has *dit* as root, with *me*, *l'* and *a* depending on it (fr-ud-test_00087).

English UD has `aux` and `aux:pass`; French GSD adds the subtypes `aux:tense` and `aux:caus`. The parser is retrained on the French treebank, so it learns them as it learns any other label.

**Agreement.** Mova does not check or use the agreement of the past participle. The feature that is needed (the gender and the number of the participle and of the preceding direct object) is in the treebank (`Gender`, `Number` on participles) but is lost in the English tags (the participle is `VBN`), so the tagger cannot tell *vue* from *vu* by its tag and the parser does not know that a preceding *les* demands an agreement. The treebank keeps agreement errors as they were written (*ont conduits*, fr-ud-train_03882) and marks some misspelt tokens with `Typo=Yes`, so it can serve as test data for a checker but not as a clean paradigm.

**What is not handled.** The choice of the auxiliary for a given verb, the agreement of the participle in all its rules (a rule-based checker for this needs the tree: the position and the function of the object), the tests for the participle of perception verbs and for the measure verbs, the rectified *laissé*, the passive restrictions. The agreement rules are among those that a deterministic reader with a tree could apply, because they depend on the object's function and position, which the tree gives; this is a candidate for the first French rule module.

**Sources.** cslf-1990 for the participle of *laisser* and the position of the report on the pronominal participle; the counts and trees are from UD French GSD.
