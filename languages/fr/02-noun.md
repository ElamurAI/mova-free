# 2. The noun: gender and number

A French noun has a grammatical gender, masculine or feminine, that it keeps in all its uses, and a number, singular or plural, that it takes according to the sense. Gender and number are marked on the words around the noun (the determiner, the adjective, the pronoun that refers back to it, the past participle) far more than on the noun itself, and in speech the plural of the noun is not heard at all. This chapter describes the two categories, the ways of forming the feminine and the plural, the classes of exception, and the way gender is treated in modern usage. The agreement of the words around the noun is taken up in chapters 3, 4, 5 and 7.

## Contents

1. Gender
2. Guessing the gender
3. Words with two genders
4. Forming the feminine
5. Forming the plural
6. Compounds, proper names, invariable and plural-only nouns
7. Gender and the names of people: feminisation and inclusive writing
8. How Mova handles it

---

## 1. Gender

French has two genders, masculine and feminine. The neuter of Latin was lost; its traces are in the pronouns (*ce*, *cela*, *ça*, *il* in impersonal use, *le* when it stands for a clause: [ch. 4](04-pronouns.md)). Every noun is of one gender or the other, whether it names a thing or a person: *le livre* "the book", *la table* "the table", *le soleil* "the sun", *la lune* "the moon".

Gender is shown in four places:

- the **determiner**: *le*, *un*, *ce*, *mon*, *quel* (masculine) against *la*, *une*, *cette*, *ma*, *quelle* (feminine); before a vowel, the masculine/feminine contrast is lost in the definite article (*l'ami*, *l'amie*) and in the possessives, where the masculine form is used before a vowel-initial feminine noun (*mon amie*, not *ma amie*);
- the **adjective**: *un grand homme*, *une grande femme*;
- the **pronoun** that refers to the noun: *il* / *elle*, *le* / *la*, *lui*, *celui* / *celle*;
- the **past participle** in the contexts where it agrees ([ch. 7](07-auxiliaries-participle.md)).

For the noun itself gender is marked only in part of the vocabulary: by the ending of a derivational suffix (§2), and by the feminine suffix on nouns that denote people and animals (§4).

## 2. Guessing the gender

The gender of a noun denoting a thing is lexical, but there are strong tendencies; Sahai and Sharma (sahai-2021) build interpretable classifiers of the gender of a French noun from its endings, its sound and its meaning; in their experiments the phonology alone predicts the gender better than the meaning alone (81% against 77%), and the meaning adds most where the spelling is silent about it. The tendencies below are the ones a reference grammar lists.

**By meaning.** Nouns denoting male persons and animals are masculine, female ones feminine, when the language has a pair (*un garçon* / *une fille*, *un cheval* / *une jument*). The days of the week, the months, the seasons, the names of the languages, of the colours and of the trees are masculine (*le lundi*, *le français*, *le rouge*, *le chêne*). Names of countries and rivers that end in *-e* are feminine (*la France*, *la Seine*) with exceptions (*le Mexique*, *le Cambodge*, *le Rhône*).

**By ending.**

| ending | gender | examples | exceptions |
|---|---|---|---|
| *-age* | masc. | *le voyage*, *le fromage* | *la page*, *la plage*, *la cage*, *la nage*, *une image*, *la rage* |
| *-ment* | masc. | *le gouvernement* | *la jument* |
| *-isme*, *-eau*, *-eur* (agent, tool) | masc. | *le tourisme*, *le bureau*, *le moteur* | *l'eau*, *la peau*; *la fleur*, *la peur*, *la chaleur* |
| *-tion*, *-sion*, *-té*, *-ée* | fem. | *la nation*, *la liberté*, *la journée* | *le côté*, *le comité*, *le musée*, *le lycée* |
| *-ure*, *-ance*, *-ence*, *-esse*, *-ette* | fem. | *la voiture*, *la chance*, *la jeunesse*, *la fourchette* | *le silence*, *le murmure*, *le squelette* |
| *-ie*, *-ine*, *-ade*, *-ille* | fem. | *la vie*, *la cuisine*, *la salade*, *la famille* | *le parapluie*, *le magazine* |

A few semantic classes override the ending (*le tourisme* is masculine because of *-isme*, and names of the sciences in *-ique* are feminine: *la physique*, *la musique*). Gender can be guessed correctly for the greater part of the vocabulary from the last syllable; the exceptions are frequent enough, and the nouns are common enough, that a learner must still learn the gender of each noun together with the word.

## 3. Words with two genders

Some forms exist in both genders, with different meanings. The pair is two words, not one word with two genders (*le livre* "book" / *la livre* "pound"):

| masculine | feminine |
|---|---|
| *le tour* "turn" | *la tour* "tower" |
| *le poste* "post, job, set" | *la poste* "post office" |
| *le mode* "manner, mood" | *la mode* "fashion" |
| *le vase* "vase" | *la vase* "mud" |
| *le manche* "handle" | *la manche* "sleeve" |
| *le voile* "veil" | *la voile* "sail" |
| *le critique* "critic" | *la critique* "criticism" |
| *le mémoire* "dissertation" | *la mémoire* "memory" |
| *le moule* "mould" | *la moule* "mussel" |
| *le page* "page boy" | *la page* "page" |

A few nouns are masculine in the singular and feminine in the plural in literary use: *amour*, *délice*, *orgue* (*un grand amour*, *de folles amours*). The word *gens* "people" is a plural without a singular and takes the feminine for the adjective that stands before it and the masculine for the one that follows, a rule of the classical grammar that is now often relaxed: *de bonnes gens* but *des gens heureux*. In the treebank, *Les gens sont accueillants* (fr-ud-train_01230) has the masculine plural predicate.

## 4. Forming the feminine

For nouns that denote people and animals the feminine is derived from the masculine; the same rules apply to adjectives ([ch. 5](05-adjectives-adverbs.md)). The ways are:

| masculine | feminine | rule | examples |
|---|---|---|---|
| *-C* | *-Ce* | add *-e*; the final consonant, silent in the masculine, is heard in the feminine | *petit* → *petite*, *ami* → *amie* (no change of sound), *Français* → *Française* |
| *-e* | *-e* | epicene: the same form, gender shown by the determiner | *un/une élève*, *un/une artiste*, *un/une collègue* |
| *-er* | *-ère* | | *boulanger* → *boulangère*, *premier* → *première* |
| *-eur* | *-euse* | from a verb stem | *danseur* → *danseuse*, *vendeur* → *vendeuse* |
| *-teur* | *-trice* | from a Latin agent noun | *acteur* → *actrice*, *directeur* → *directrice* |
| *-eur* | *-eure* | for trade names, outside France in particular | *professeur* → *professeure*, *auteur* → *autrice* or *auteure* |
| *-f* | *-ve* | | *veuf* → *veuve*, *neuf* → *neuve* |
| *-x* | *-se* | | *heureux* → *heureuse*, *époux* → *épouse* |
| *-on*, *-en*, *-el*, *-et*, *-eil*, *-ul* | doubled consonant + *e* | with exceptions (*complet* → *complète*, *secret* → *secrète*) | *lion* → *lionne*, *chien* → *chienne*, *Gabriel* → *Gabrielle*, *chat* → *chatte*, *nul* → *nulle* |
| *-c* | *-que* | | *public* → *publique*, *Turc* → *Turque* |
| *-in*, *-ain*, *-un* | *-ine*, *-aine*, *-une* | single consonant, the vowel loses its nasal | *voisin* → *voisine*, *Romain* → *Romaine*, *brun* → *brune* |
| noble ranks | *-esse* | | *comte* → *comtesse*, *poète* → *poétesse*, *maître* → *maîtresse* |
| none | a different word | suppletion | *homme* / *femme*, *garçon* / *fille*, *père* / *mère*, *frère* / *sœur*, *roi* / *reine*, *oncle* / *tante*, *coq* / *poule* |

In speech the feminine is audible when the added *-e* uncovers a final consonant: *petit* /pəti/, *petite* /pətit/; *grand* /ɡʁɑ̃/, *grande* /ɡʁɑ̃d/. Where the masculine already ends in a pronounced consonant or the feminine adds nothing audible, gender is inaudible on the noun or the adjective: *ami* and *amie* are both /ami/, *joli* and *jolie* both /ʒɔli/, *vrai* and *vraie* both /vʁɛ/. For a large part of the lexicon and for the grammar of agreement, gender is therefore visible in the written text and invisible in speech.

## 5. Forming the plural

The regular plural adds *-s*, which is not pronounced: *le livre* / *les livres*, *une table* / *des tables*. In speech the plural of a noun is carried by the determiner (*le* / *les*, *un* / *des*, *ce* / *ces*, *mon* / *mes*) and, before a vowel, by liaison (*les‿amis*). An adjective that follows is not audibly marked either (*des livres rouges* = *un livre rouge* /ʁuʒ/), unless the plural has a stem change.

The exceptions fall in a short list.

- **Nouns ending in *-s*, *-x*, *-z*** do not change: *un fils* / *des fils*, *une voix* / *des voix*, *un nez* / *des nez*.
- **Nouns in *-au*, *-eau*, *-eu*** take *-x*: *un bateau* / *des bateaux*, *un jeu* / *des jeux*; so do *-ou* in seven words (*bijou*, *caillou*, *chou*, *genou*, *hibou*, *joujou*, *pou*) while the others take *-s* (*un trou* / *des trous*). Exceptions in the other direction: *bleu* / *bleus*, *pneu* / *pneus*, *landau* / *landaus*, *sarrau* / *sarraus*.
- **Nouns in *-al*** make *-aux*: *un cheval* / *des chevaux*, *un journal* / *des journaux* (the treebank has *aux journaux*, fr-ud-dev_00755), with a few in *-s* (*bal*, *carnaval*, *festival*, *récital*, *régal*). The stem vowel changes with the ending of the written plural, and the plural is audible (/ʒuʁnal/ / /ʒuʁno/).
- **Nouns in *-ail*** take *-s* (*détail* / *détails*) apart from a group of seven in *-aux* (*travail* / *travaux*, *vitrail* / *vitraux*, *corail* / *coraux*, *émail* / *émaux*, *bail* / *baux*, *soupirail* / *soupiraux*, *vantail* / *vantaux*). The treebank has *les nouveaux travaux* (fr-ud-test_00286).
- **Irregular plurals**: *un œil* / *des yeux*, *le ciel* / *les cieux* (with *les ciels* in paintings and weather), *un aïeul* / *des aïeux* (ancestors), *monsieur* / *messieurs*, *madame* / *mesdames*, *mademoiselle* / *mesdemoiselles*, *bonhomme* / *bonshommes*, *gentilhomme* / *gentilshommes*.
- **Audible plurals by loss of a consonant**: *un œuf* /œf/ / *des œufs* /ø/, *un bœuf* /bœf/ / *des bœufs* /bø/, *un os* /ɔs/ / *des os* /o/. These are the three nouns where the plural is carried by the noun itself in speech.

The 1990 rectification regularised the plural of loanwords and of compounds; see [chapter 1](01-sounds-and-spelling.md), §8.

## 6. Compounds, proper names, invariable and plural-only nouns

**Compounds.** A compound written with a hyphen takes plural marks according to its parts, and the tradition is complex: noun + adjective compounds take the mark on both (*un grand-père* / *des grands-pères*, *un coffre-fort* / *des coffres-forts*), verb + noun compounds are traditionally invariable or marked on the noun by the sense (*un gratte-ciel* / *des gratte-ciel*, as in fr-ud-train_04898 in the singular, but *un porte-monnaie* / *des porte-monnaie*), and compounds with an adverb or a preposition take the mark on the noun (*un après-midi* / *des après-midis* after the rectification). The rectified rule (verb + noun and preposition + noun: the mark on the second element, in the plural only) removes most of the cases ([ch. 1](01-sounds-and-spelling.md)).

**Proper names.** Family names are invariable: *les Dupont*, *les Martin*. Names of nations that are plural take a plural article (*les États-Unis*, *les Pays-Bas*, *les Philippines*; in the treebank, fr-ud-dev_00569 *aux États-Unis*). A name for the people of a country is a noun with a capital (*un Français*, *les Italiens*); the same word as an adjective or as a language has no capital (*un livre français*, *le français*).

**Nouns with no singular**: *gens*, *funérailles*, *fiançailles*, *obsèques*, *mœurs*, *frais* "costs", *vivres*, *environs*, *archives*, *ciseaux* (with *des ciseaux* "scissors" and *une paire de ciseaux*), *lunettes*. **Nouns with no plural**: mass and abstract nouns (*le courage*, *la faim*, *le sang*) which may take a plural with a different sense (*des courages*, *des sangs*).

**Collective nouns** (*foule*, *groupe*, *majorité*, *équipe*) take a singular verb, with the plural possible when the noun is followed by a plural complement (*une foule de gens se pressaient* beside *se pressait*).

## 7. Gender and the names of people: feminisation and inclusive writing

French grammar traditionally uses the masculine as the unmarked form: it is used for a group of mixed sex (*les étudiants*), for a person whose sex is unknown or unimportant (*le candidat doit …*), and for the dictionary form of a noun. Two modern developments bear on this.

**Feminised trade names.** For a woman, the title may be feminised (*une directrice*, *une avocate*, *une ministre*, *une professeure*, *une autrice*) or the masculine title may be kept with a feminine determiner (*une professeur*, *une ministre*). Practice differs from country to country: Québec adopted feminised forms early and generally, in Belgium the French Community publishes an official guide of feminine forms ("Mettre au féminin", moreau-2014, card only), Switzerland has federal drafting instructions on non-sexist language (suisse-2000, card only), and in France the official body, the Académie française, long refused them and changed its position in 2019 (reported; we have not read the declaration). In the treebank, *elle est nommée ministre de la Culture* (fr-ud-dev_01095) shows a feminine referent and the masculine-tagged noun *ministre* (the annotation is the treebank's choice: in UD French GSD the lemma *ministre* occurs 86 times as masculine and 7 as feminine).

**Inclusive writing** (*écriture inclusive*) uses doublets (*les étudiants et les étudiantes*), the median dot (*les étudiant·e·s*), collective or epicene nouns (*le corps étudiant*, *les personnes étudiantes*), and, for non-binary reference, new pronouns (*iel*, *ielle*) and neologism endings. Two recent studies treat it as an NLP problem: Doyen and Todirascu (doyen-2025) build the first system that neutralises the gender of French text by means of collective nouns, and Flesch and Burnett (flesch-2025) describe how non-binary authors use grammatical gender, in three patterns (non-binary markers, one gender throughout, or both). Whether these forms are accepted is a matter of register and of institution (the typographic side is in [chapter 12](12-punctuation-typography.md)), and a parser must be ready to meet them, for example in the forms *étudiant·e·s* and *iel*.

## 8. How Mova handles it

**What is done.** Mova has no French morphology: no analyser for the feminine and the plural, no table of irregular plurals, no gender lexicon. Gender and number are available only as annotations on the treebanks that the parser is trained on. The English parser's tag set (the Penn Treebank tags) has no gender; the tool `en xpos-fill` maps each word's UPOS and FEATS to an English tag (a noun with `Number=Plur` becomes `NNS`, a noun otherwise `NN`), so the information on gender is dropped on the way and the agreement of a noun with its determiner and adjective is not visible to the tagger as a feature. The parser can still learn some of it from the words themselves (*la* before a noun is likely feminine) but gender is never checked.

**Measured.** UD French GSD (UD 2.18, all three files, 2026-10-22) has 75 330 `NOUN` tokens; 75 175 of them carry `Gender` and 75 029 carry `Number`. The gender of the epicene nouns is set by the annotator: of 148 tokens of *enfant*, 128 carry `Number` only and the rest are split between masculine and feminine. There are 13 326 distinct noun forms and 9 466 noun lemmas in GSD. Train → dev, 10.07% of the dev `NOUN` tokens (684 of 6 792) have a form that is not in the training noun forms (our count, 2026-10-22); over all words but punctuation the figure is 8.48% (2 706 of 31 919). These unseen forms are the ones for which the tagger relies on the shape of the word: its accuracy on them is 84.8%.

**UD conventions** (UD French GSD):

- Nouns have `Gender` and `Number`. Plural forms of epicene nouns may have `Number` only.
- The contracted forms *au*, *aux*, *du*, *des* are multiword tokens split into a preposition and a determiner ([ch. 1](01-sounds-and-spelling.md), §9).
- A compound written with a hyphen is one token (`gratte-ciel`, `États-Unis` as a name); name sequences are `flat:name`.
- A title before a name is a `NOUN` with the name as `flat:name`: *Madame Tellier* (fr-ud-test_00263) has `Madame` as the head and `Tellier` as `flat:name`.

**What is not handled.** The feminine and plural formation, the irregular plurals, gender prediction for unseen nouns, the inclusive forms (*étudiant·e·s* would be split or tagged as unknown), and agreement checking in parsing. A first step would be to derive a form → lemma + features lexicon from the lemma and FEATS columns of the open French treebanks; that lexicon would cover only forms that occur in them, so it would not solve the unseen-form problem.

**Sources.** Sahai and Sharma 2021 (sahai-2021), Doyen and Todirascu 2025 (doyen-2025), Flesch and Burnett 2025 (flesch-2025), the Belgian and Swiss guides (moreau-2014, suisse-2000; card only); the general descriptions are those of every reference grammar and are listed in [sources.md](sources.md).
