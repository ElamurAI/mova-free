# 13. Varieties and registers

French is spoken as a first or usual language in France, Belgium, Switzerland, Luxembourg, Monaco, Canada (Québec, Acadia, Ontario, Manitoba), and as a language of school, administration and the city across West and Central Africa, the Maghreb, the Indian Ocean (Madagascar, Mauritius, Réunion), the Caribbean (Haiti, Martinique, Guadeloupe) and the Pacific. The written standard is nearly the same everywhere. The differences between countries lie in the sounds, the vocabulary and a few points of grammar. The differences between speech and writing are as large, and Mova meets them in every spoken corpus. This chapter gives the registers first, then the national varieties.

## 13.1 Spoken and written French

The written standard and ordinary speech differ systematically. The spoken forms below are the norm of conversation in all regions and social groups, not errors; the written forms are the norm of writing, formal speech and reading aloud.

| point | written | spoken | chapter |
|---|---|---|---|
| negation | *Je ne sais pas.* | % *Je sais pas.* (/ʃepa/) | [8](08-negation.md) |
| first person plural | *Nous partons.* | % *On part.* | [4](04-pronouns.md) |
| demonstrative pronoun | *cela, ceci* | *ça* | [4](04-pronouns.md) |
| questions | inversion: *Viens-tu ?* | intonation: % *Tu viens ?*, question word in place: % *Tu vas où ?* | [9](09-questions.md) |
| topic | subject in place | dislocation: % *Mon frère, il est parti.* | [10](10-clause-structure.md) |
| impersonal *il* | *Il faut, il y a* | % *Faut, y a* | [10](10-clause-structure.md) |
| past tense | *passé simple* in narrative | *passé composé* always | [6](06-verb.md) |
| relative *dont* | *le livre dont je parle* | % *le livre que je parle* (informal) | [11](11-subordination.md) |
| reduced forms | *tu es, il y a, je suis* | % *t'es, y a, chuis* | [1](01-sounds-and-spelling.md) |
| discourse markers | — | *bon, ben, bah, hein, quoi, du coup, genre* | [10](10-clause-structure.md) |

On our data (UD 2.18, training sets, counted 2026-10-22):

| | spoken ParisStories (22 540 words) | written GSD (364 333 words) |
|---|---:|---:|
| *on* | 400 | 479 |
| *nous* as subject | 3 | 223 |
| *ça* | 133 | 37 |
| *cela* | 2 | 122 |

In the spoken corpus *on* stands for "we" about 130 times as often as *nous*, and *ça* replaces *cela* almost completely; in the written corpus *nous* and *cela* hold. Together with the counts of negation ([chapter 8](08-negation.md): *ne* kept in about one negated clause in seven in speech) and dislocation ([chapter 10](10-clause-structure.md): forty times as frequent in speech), these are the measurable marks of the spoken register.

### Registers

French grammars and dictionaries mark three registers below the neutral one, and one above it:
- ***soutenu*** (formal, literary): the *passé simple* and the imperfect subjunctive, inversion, *nous*, expletive *ne*, *lequel*;
- ***courant*** (neutral, the standard of the press);
- ***familier*** (informal): *ne* dropped, *on*, *ça*, *bouquin* for *livre*, *bagnole* for *voiture*;
- ***populaire*** and ***argotique*** (slang), with ***verlan***, words with their syllables reversed: *meuf* (*femme*), *relou* (*lourd* "annoying"), *chelou* (*louche* "shady"), *ouf* (*fou*), *vénère* (*énervé*).

The labels change with time: *mec* "guy" was slang and is now *familier*. Young urban speech (the *français des cités*) adds borrowings from Arabic, Romani, the African languages and English (*wesh, kiffer, daron*).

## 13.2 Belgium

- **Numbers:** *septante* (70), *nonante* (90) for *soixante-dix, quatre-vingt-dix*; *quatre-vingts* (80) as in France.
- **Vocabulary:** *déjeuner, dîner, souper* for breakfast, lunch, dinner (France: *petit-déjeuner, déjeuner, dîner*); *une fois* as a softening particle (% *Viens une fois*); *s'il vous plaît* also for "here you are"; *savoir* for *pouvoir* (% *Je ne sais pas venir* "I can't come"); *GSM* for a mobile phone; *kot* for a student room.
- **Sounds:** the distinction between *brin* /ɛ̃/ and *brun* /œ̃/ is kept; *w* in *wagon* is /w/.

## 13.3 Switzerland

- **Numbers:** *septante, huitante* (in Vaud, Valais, Fribourg), *nonante*.
- **Vocabulary:** *déjeuner, dîner, souper* as in Belgium; *natel* for a mobile phone; *votation* for a referendum; *panosse* for a floor cloth; *s'encoubler* "to trip".
- **Typography:** usually no space before *; ! ?* ([chapter 12](12-punctuation-typography.md)).

## 13.4 Canada: Québec and Acadia

Quebec French is the largest variety outside Europe, with its own standard norm of the written language, very close to that of France, and a distinct spoken language.
- **Sounds:**
  - /t/ and /d/ become [ts], [dz] before /i, y/ (*petit* [ptsi], *dur* [dzyʁ]);
  - the high vowels are lax in closed syllables (*vite* [vɪt]);
  - the old /ɛː/ is diphthongized in casual speech (*père* [paɛ̯ʁ]);
  - /ɑ/ is distinct from /a/.
- **The particle *-tu*** marks a yes/no question with any subject: % *Tu viens-tu ?* % *C'est-tu prêt ?* ([chapter 9](09-questions.md)).
- **Vocabulary:**
  - official French forms for English words: *courriel* (e-mail), *clavardage* (chat), *magasiner* (to shop), *fin de semaine* (weekend), *stationnement* (parking), where France uses the anglicism (*mail, chat, faire du shopping, week-end, parking*);
  - in speech, English loans: *char* "car", *chum* "boyfriend, friend", *blonde* "girlfriend", *pis* "and then";
  - religious swear words (*sacres*): *tabarnak, câlice, ostie*;
  - *bienvenue* "you're welcome", *dîner* "lunch", *souper* "dinner", *déjeuner* "breakfast".
- **Grammar:** *après* + infinitive for the progressive (% *Je suis après manger* "I'm eating"); *icitte* (*ici*), *moé, toé* (*moi, toi*) in rural and casual speech; *ils* for *elles* in casual speech.

**Acadian French** (New Brunswick, Nova Scotia, Louisiana's Cajun French) keeps old forms: *je* with plural verbs (% *j'avons, je parlons*), the simple past in speech, and *-ont* for the third person plural (% *ils parlont*).

## 13.5 Africa and elsewhere

In West and Central Africa French is the language of school, administration and the cities, spoken beside the national languages, and in some cities (Abidjan, Kinshasa, Libreville) the usual language of daily life.
- **Vocabulary:**
  - new words and senses: *deuxième bureau* "mistress", *gâter* "to break", *grilleur* "seller of grilled food", *cadeauter* "to give a gift", *enceinter* "to make pregnant", *essencerie* "petrol station" (Senegal);
  - borrowings from Wolof, Lingala, Bambara and Arabic;
  - *Nouchi* in Côte d'Ivoire: a youth argot mixing French and the local languages.
- **Grammar:** *pour* for a possessive (% *C'est pour moi* "It's mine"); *-là* after the noun as a definite or familiar marker (% *la femme-là* "that woman, the woman we know"); calques from the national languages.
- **Sounds:** an alveolar trilled /r/; few nasal vowels, or none, in some regions.

The Maghreb's French is marked by borrowings from Arabic and Berber and by code-switching. In the Caribbean and the Indian Ocean French stands beside French-based **creoles**: Haitian, Martinican, Réunionese, Mauritian and Seychellois. These are languages of their own, not dialects of French, with their own grammar (Haitian *mwen pa konnen* "I don't know", *li te manje* "he ate").

## How Mova handles it

**The treebanks Mova has** cover:
- **written French of France:** GSD (blogs, news, reviews, Wikipedia), Sequoia (Europarl, regional newspaper, Wikipedia, medicine), ALTS (law);
- **spoken French of France:** ParisStories (stories told in Paris), Rhapsodie (prosody, several genres);
- **a regional variety:** PoitevinDIVITAL (the Poitevin-Saintongeais langue d'oïl, grammar examples).

There is no Belgian, Swiss, Canadian or African treebank among them, and no creole. The licences are in [sources.md](sources.md).

**In Mova:**
- The parser is trained on GSD, written, through `en xpos-fill`. It has seen the spoken relations of [chapter 10](10-clause-structure.md) only when the spoken treebanks are added. Its scores on spoken French are not measured yet.
- Nothing in Mova marks a French register. The English side has the half-plain, half-slang voice of the school courses and the register lexicons, `global/data/register-*.tsv`.
- The marks of 13.1 (*ne* dropped, *on*, *ça*, dislocation, discourse markers) are counted features in the trees and could give a French register score the same way.
- The national vocabulary of 13.2–13.5 needs a lexicon with variety labels. The French Wiktionary has them (*Belgique*, *Québec*, *Afrique*), and its raw dump is on disk (CC BY-SA, for learning and tests only).
