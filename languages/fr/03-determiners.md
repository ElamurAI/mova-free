# 3. Determiners and numerals

A French common noun in an argument position almost never stands alone: it is preceded by a determiner. This is stricter than in English, where plural and mass nouns can be bare (*I like dogs*, *I drink water*); in French the corresponding sentences need an article (*J'aime les chiens*, *Je bois de l'eau*). The determiner also carries the gender and number of the noun in the places where the noun does not ([ch. 2](02-noun.md)), and the possessive, the demonstrative and the quantifier agree with the noun that follows, not with the possessor. This chapter covers the articles, the demonstratives, the possessives, the quantifiers and the numerals.

## Contents

1. The articles: definite, indefinite, partitive
2. When there is no article
3. *De* in place of the article
4. Demonstratives
5. Possessives
6. Interrogative, exclamative and relative determiners
7. Quantifiers and indefinite determiners
8. Numerals
9. How Mova handles it

---

## 1. The articles: definite, indefinite, partitive

| | masculine sg. | feminine sg. | plural |
|---|---|---|---|
| definite | *le* (*l'*) | *la* (*l'*) | *les* |
| indefinite | *un* | *une* | *des* |
| partitive | *du* (*de l'*) | *de la* (*de l'*) | *des* |

**Definite.** The definite article marks a noun that is identified in the context (*Passe-moi le livre*), a unique referent (*la lune*), and, more widely than in English, a generic or class reading (*Les chats sont des animaux*, *J'aime le café*, *La vie est belle*). In English a bare plural or a mass noun does this; in French it is the definite article. With the prepositions *à* and *de* the masculine and plural forms fuse (*au*, *aux*, *du*, *des*; [ch. 1](01-sounds-and-spelling.md), §5). The article is also used with a body part when the owner is clear (*Il lève la main*, "He raises his hand") and with units of price or measure (*trois euros le kilo*, "three euros a kilo").

**Indefinite.** *Un* and *une* introduce a new, singular, countable referent (*Il y a une maison au bout du chemin*); *des* is the plural of *un/une* (*J'ai vu des chiens*). The plural *des* is not an optional word: there is no zero plural, and the sentence *J'ai vu chiens* is ungrammatical.

**Partitive.** *Du*, *de la*, *de l'* introduce an undetermined quantity of a mass noun (*Je veux du pain*, *Elle boit de l'eau*, *Il faut du courage*). The partitive is formed with *de* plus the definite article, and the two uses of the form *du* have to be told apart: *Je viens du marché* (preposition *de* plus article, "from the market") and *Je mange du pain* (partitive article, "some bread"). The plural of the partitive and the plural indefinite article have the same form, *des*. A noun can move between the two readings with a change of meaning: *du poulet* "chicken (meat)" and *un poulet* "a chicken".

The treebank, which contains the three uses, shows them with the same tag: *du temps* "time", fr-ud-train_00391 (*Il faudra encore du temps*); *du vin*, fr-ud-train_04454.

## 2. When there is no article

A noun is used without an article in a closed set of contexts.

- **After the preposition *en*** and in many prepositional phrases of manner, place or material (*en voiture*, *en France*, *sans argent*, *avec plaisir*, *par avion*, *de pierre*, *à cheval*).
- **A predicate noun of profession, nationality, religion, or status** with no modifier: *Elle est avocate*, *Il est français*, *Il est devenu médecin*. With a modifier or with *c'est*, the article returns: *C'est une avocate*, *Elle est une avocate renommée*.
- **Apposition and titles**: *Paris, capitale de la France*; *Madame Tellier* is a name in a sequence.
- **Lists, headlines, proverbs, signs, labels**: *Entrée libre*; *Pierre qui roule n'amasse pas mousse*.
- **Verb–noun idioms** (*avoir faim*, *avoir peur*, *prendre froid*, *faire peur*, *perdre pied*) and **a bare noun with a negative** in the form *sans* (*sans doute*, *sans fin*).
- **Vocative and addresses**: *Bonjour, monsieur*; *Merci, docteur*.

## 3. *De* in place of the article

*De* (*d'* before a vowel) replaces the indefinite or partitive article in three environments. They are the main places where the form of the article, not just the noun, must be learned.

1. **After a negation of quantity.** In *Il n'a pas de temps* "He has no time" the article *du/de la/des/un/une* is replaced by *de*, if the negation is total: *J'ai des amis* → *Je n'ai pas d'amis*; *Il boit du vin* → *Il ne boit pas de vin*. The treebank sentence is *La fille qui répond n'a pas de temps à nous consacrer* (fr-ud-dev_00353). The negation side of the rule — which negations trigger it, and why *ne … que* does not — is in [chapter 8, §8.5](08-negation.md). The exception is *être* and the contrast cases: *Ce n'est pas un livre* ("It is not a book"; the negation of the predicate keeps the article) and *Je n'ai pas des amis, j'ai des alliés* (contrastive).
2. **After an expression of quantity.** The quantity word is followed by *de* and a bare noun: *beaucoup de gens*, *peu de temps* (fr-ud-dev_00069, *peu de temps après*), *assez de*, *trop de*, *plus de*, *moins de*, *tant de*, *autant de*, *combien de*, *une bouteille de vin*, *une dizaine de thoniers* (fr-ud-train_03512), *un kilo de pommes*. The determiner is not repeated. After *bien des*, *la plupart des*, *la majorité des*, the definite *des* (de + les) is used because the noun is definite.
3. **Before a plural adjective that precedes the noun.** In writing, *des* becomes *de* in front of an adjective + noun: *de grands arbres*, *de bons amis*, *de beaux jours*. This is the written standard; in speech, and in much informal writing, *des grands arbres*, *des bons amis* is the norm. The treebank has *de bons petits plats* (fr-ud-train_04010). When the adjective follows the noun, or when the adjective and noun form a compound, *des* stays: *des arbres magnifiques*, *des petits pois*, *des jeunes gens*, *des grands-parents*.

**The preposition *de* with a quantity noun.** In *une série de grands concerts* (fr-ud-train_06911) *de* is the preposition that introduces the complement of the noun *série*, and the following *grands* is an ordinary adjective: the same *de* serves both as a preposition and as the article form, which is why the UD trees give it `case` in one structure and `det` in another (§9).

## 4. Demonstratives

| | masculine | feminine |
|---|---|---|
| singular | *ce* (*cet* before a vowel) | *cette* |
| plural | *ces* | *ces* |

*Ce livre*, *cet arbre*, *cet homme*, *cette maison*, *ces enfants*. The demonstrative points to a referent present in the speech situation or the text (*ce système*, fr-ud-dev_00053; *cette audace*, fr-ud-dev_00070) and often has the sense "the one just mentioned". When the distinction between near and far is wanted, the particles *-ci* and *-là* are attached to the noun: *ce livre-ci* "this book", *ce livre-là* "that book". In spoken French *-là* is far more frequent than *-ci* and it is used for both. The demonstrative pronoun (*celui*, *celle*, *ceux*, *celles*, *ce*, *ceci*, *cela*, *ça*) is in [chapter 4](04-pronouns.md).

## 5. Possessives

The possessive determiner agrees in gender and number with the **possessed** noun and in person and number with the **possessor**. It is a single word for the possessor, which means that *son livre* can be "his book" or "her book".

| possessor | before masc. sg. | before fem. sg. | before plural |
|---|---|---|---|
| 1 sg. | *mon* | *ma* | *mes* |
| 2 sg. | *ton* | *ta* | *tes* |
| 3 sg. | *son* | *sa* | *ses* |
| 1 pl. | *notre* | *notre* | *nos* |
| 2 pl. | *votre* | *votre* | *vos* |
| 3 pl. | *leur* | *leur* | *leurs* |

Before a feminine noun that begins with a vowel the masculine form is used: *mon amie*, *ton école*, *son énergie*. The plural possessor's forms *leur(s)* are used when each owner has one thing or when the things are the owners' shared property (*Les enfants aiment leur mère*, *leurs parents*).

**Possessive replaced by the article.** When the owner is clear from the subject, a part of the body or a close possession takes the definite article, not the possessive: *Elle a levé la main*, *Il s'est cassé la jambe*, *J'ai mal à la tête*. The owner can be shown by an object clitic (*Je lui ai pris la main*, "I took his hand").

**The pronoun forms** (*le mien*, *la tienne*, *les siens*) and the predicative use (*Ce livre est à moi*, *Ce livre est le mien*) are in [chapter 4](04-pronouns.md). The possessive of a thing, in the sense of "its", is often rendered by *en* and the definite article (*Le livre est ancien; j'en connais l'auteur*, "I know its author") in the written standard, which is a structure that English does not have.

## 6. Interrogative, exclamative and relative determiners

*Quel*, *quelle*, *quels*, *quelles* are an interrogative (*Quel livre veux-tu ?*), exclamative (*Quelle belle journée !*), and, with *lequel*, a relative determiner (*auquel cas*). In the treebank *Quel est le coût d'un tel projet ?* (fr-ud-test_00178) shows the interrogative used with *être* as the predicate; the question types are in [chapter 9](09-questions.md).

## 7. Quantifiers and indefinite determiners

| form | meaning | notes |
|---|---|---|
| *quelque(s)* | some, a few | *quelques-uns*, *quelqu'un* |
| *plusieurs* | several | plural only |
| *certain(s)*, *certaine(s)* | some, a certain | *certains centres* (fr-ud-dev_00221) |
| *chaque* | each | singular only (*chaque soir*, fr-ud-test_00158) |
| *tout*, *toute*, *tous*, *toutes* | all, the whole | *tout le monde*, *toute la journée*, *tous les jours* |
| *aucun(e)*, *nul(le)* | no, not any | with *ne*: *aucun impact* (fr-ud-dev_00110) |
| *autre(s)*, *même(s)* | other, same | *les mêmes gens* |
| *différents*, *divers*, *maint* | various | before the noun |
| *n'importe quel*, *quelconque*, *tel* | any, such | *un tel projet* |

*Tout* + definite article + singular noun means "the whole" (*tout le camping* "all of the campsite", fr-ud-dev_00311), and *tous les* + plural noun means "all the" or "every" (*tous les sens*, fr-ud-dev_00883). The form *tous* is pronounced /tus/ as a pronoun and /tu/ as a determiner.

## 8. Numerals

**Cardinals.** *zéro*, *un*, *deux*, *trois*, *quatre*, *cinq*, *six*, *sept*, *huit*, *neuf*, *dix*, *onze*, *douze*, *treize*, *quatorze*, *quinze*, *seize*, *dix-sept*, *dix-huit*, *dix-neuf*, *vingt*, *trente*, *quarante*, *cinquante*, *soixante*. The numerals from 70 to 99 follow a partly vigesimal system in France: *soixante-dix* (60 + 10), *soixante et onze*, *quatre-vingts* (4 × 20), *quatre-vingt-un*, *quatre-vingt-dix* (4 × 20 + 10). In Belgium, the French-speaking part of Switzerland and Aosta the decimal forms are used for 70 and 90: *septante*, *nonante*; and in parts of Switzerland *huitante* or *octante* for 80 ([ch. 13](13-varieties-registers.md)). The Académie française writes the numbers up to 99 with hyphens, except *et* (*vingt et un*, *soixante et onze*, but *quatre-vingt-un*); the 1990 rectification hyphenates all of them ([ch. 1](01-sounds-and-spelling.md), §8).

**Agreement.** *Un* has a feminine (*une*); the others are invariable, except that *vingt* and *cent* take *-s* when they are multiplied and not followed by another number: *quatre-vingts ans* but *quatre-vingt-trois ans*, *deux cents francs* but *deux cent trois francs*. *Mille* is invariable (*deux mille*); *million* and *milliard* are nouns and take *-s* and the preposition *de*: *deux millions d'habitants*. The numeral stands after the determiner and before an adjective (*les deux premiers jours*, *ces trois grandes villes*).

**Ordinals.** The ordinal is formed from the cardinal with *-ième* (*deuxième*, *troisième*, *vingt et unième*), with the exceptions *premier/première* and the alternative *second/seconde* (said of the second in a series of two). The ordinal precedes the noun: *le premier fournisseur* (fr-ud-dev_00374), *la deuxième fois*. Dates use the cardinal except for the first: *le premier mai*, *le deux mai*; in the treebank dates have the day as a number (*le 3 octobre 1997*). The abbreviations are *1er*, *2e* (or *2ème* in informal writing), as in *Le 24e RIC* (fr-ud-dev_00054).

**Collective numerals**: *une dizaine*, *une douzaine*, *une quinzaine*, *une vingtaine*, *une centaine*, *un millier* are feminine nouns (*millier* is masculine) meaning "about ten", with *de* and a bare plural noun: *une dizaine de thoniers* (fr-ud-train_03512). A fraction is a cardinal and an ordinal-like noun (*un tiers*, *deux tiers*, *un quart*, *trois quarts*, *la moitié*).

## 9. How Mova handles it

**What is done.** The English parser reads French determiners as the word class `DET`, and the learned model does well on them because they are frequent and closed: in UD French GSD (UD 2.18, three files, 2026-10-22) there are 61 098 `DET` tokens, of which the lemma *le* accounts for 43 301, *un* for 10 065 (all the indefinite plurals *des* and *de* are lemmatised under *un*), *son* for 4 348 and *ce* for 2 214. There is no table of French determiners in Mova and no check that the determiner agrees with the noun.

**UD conventions** (UD French GSD):

- Articles are `DET` with `Definite=Def` or `Definite=Ind` and `PronType=Art`; the relation is `det` to the noun. The partitive *du* in *du temps* (fr-ud-train_00391) is `DET` with `Definite=Ind` (lemma *du*, 100 tokens) and not a contraction. Only the contracted preposition *du* is a multiword token ([ch. 1](01-sounds-and-spelling.md)).
- *De* after a negation or a quantity is a `DET` when the noun that follows is its head and no determiner precedes: *pas de temps* (fr-ud-dev_00353) has `de` as `det` of *temps*, `Definite=Ind|Number=Sing`. In GSD the plural indefinite *de/d'* as a determiner occurs 300 + 107 times, against 1 700 for *des*.
- *De* with a quantity noun is `ADP` with `case` and the quantity noun is the head: *une série de grands concerts* (fr-ud-train_06911) has *série* as head and *concerts* as `nmod`; *une dizaine de thoniers* (fr-ud-train_03512) has *dizaine* as head with `nmod`. *Peu de temps* (fr-ud-dev_00069) is analysed with *peu* as the head (an adverb treated as a pronoun, `ExtPos=PRON`) and *temps* as its `obl:arg`.
- Possessives are `DET` with `Poss=Yes`, `PronType=Prs`, the number of the noun, and `Number[psor]`, `Person[psor]` for the possessor (*ses enfants*, fr-ud-train_04603). Demonstratives are `DET` with `PronType=Dem`.
- Numerals are `NUM` with `nummod` to the noun; a numeral with a space (*25 785*) is one token ([ch. 1](01-sounds-and-spelling.md)). A numeral may carry `Number=Plur` (*douze espèces*, fr-ud-train_00231).
- Quantifiers like *plusieurs*, *quelques*, *certains*, *chaque*, *aucun* are `DET` (252, 150, 132, 133, 108 tokens), the relation `det`; *tout* in *tout le camping* is a `DET` with `det`.

**What is not handled.** The choice between the article forms (*des*/*de*, *du*/*de*, article/no article) is not modelled: the parser sees a surface word and assigns it a class; there is no check that a sentence such as *Il n'a pas des amis* breaks the rule of §3. The possessive's two agreements (with the noun, with the possessor) are not exposed: the possessor features are in the treebank but the English tag set has no place for them, so `xpos-fill` drops them. In the tag set that `en xpos-fill` produces, a numeral is `CD`, a possessive determiner is `PRP$`, an article or demonstrative is `DT`, and an interrogative or relative determiner is `WDT`; gender, the possessor's person and the difference between the indefinite and the partitive article are not in the tags. Measured on GSD with `en xpos-fill` and `en parse-eval` (train → dev, 2026-10-22): LAS 87.00 with the parser's own tags and 89.78 with gold tags; we have not broken this down by relation, so we cannot say what the `det` relation scores alone.

**Sources.** The descriptions are those of the reference grammars (see [sources.md](sources.md)); the counts are ours on UD French GSD.
