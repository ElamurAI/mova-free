# Latin (la) — a reference manual for Mova

This is an open reference grammar of **Latin** (ISO 639-1 `la`, ISO 639-3 `lat`). It covers Latin from the classical language of Caesar and Cicero through the Latin of the Bible and the Church Fathers to the medieval Latin of charters and schools. It has chapters on the sounds and the spelling, the inflection of nouns, adjectives, pronouns and verbs, the cases, the clause and the period, verse, and the late and medieval stages. It is written for two readers at once:
- a person who reads English and wants a compact account of how Latin works;
- Mova, whose tools read Latin text as Universal Dependencies trees.

The examples are Latin with English translations. Every chapter ends with a section, **How Mova handles it**, that says what Mova's tools do with the point today and where they stop.

The manual is written in our own words. Facts come from public-domain grammars, above all Bennett's *New Latin Grammar* (1918), with Roby, Gildersleeve and Lodge, Lane and Hale and Buck. Counts come from the open UD Latin treebanks (LLCT, CIRCSE; CC BY-SA). Where a point needs the Vulgate and the classics in quantity, counts are also made on the PROIEL treebank: it is CC BY-NC-SA, so it is counted, never quoted. Works under share-alike or non-commercial terms are cited and paraphrased, never copied. The full list, with licences, is in [sources.md](sources.md).

**What Mova can do with Latin today is small.** There is no Latin lexicon and no Latin morphology in Mova. Latin is read by the English tree parser (`en`) after a treebank's tags have been mapped to the tag set it knows (`en xpos-fill`). On UD Latin LLCT (UD 2.18, train → dev, 2026-10-22) that gives LAS 87.35 with the parser's own tags and 87.80 with the treebank's gold tags. The open parsers trained on the same treebank reach 95.92 (Stanza) and 95.03 (UDPipe 2) ([docs/parsers.md](../../docs/parsers.md)). The tag set has no case, and the case is what Latin syntax rests on. Almost every chapter's last section comes back to that.

## Chapters

| # | file | what it covers |
|---|---|---|
| 1 | [01-sounds-spelling.md](01-sounds-spelling.md) | the alphabet, vowels and diphthongs, consonants, syllables, quantity and stress, sound changes, spelling through the stages, three ways to read Latin aloud |
| 2 | [02-nouns.md](02-nouns.md) | gender, number, the six cases, the five declensions, irregular, defective and Greek nouns |
| 3 | [03-adjectives.md](03-adjectives.md) | the two classes of adjectives, the pronominal adjectives, comparison, adverbs, use and position |
| 4 | [04-pronouns.md](04-pronouns.md) | personal and reflexive pronouns, possessives, demonstratives, *ipse* and *īdem*, relatives and interrogatives, indefinites, correlatives |
| 5 | [05-verb.md](05-verb.md) | the three stems, the endings, the four conjugations, the perfect system, the passive, principal parts, *sum* and the irregular verbs, deponents |
| 6 | [06-nonfinite.md](06-nonfinite.md) | the infinitive, the participles, the ablative absolute, the gerund and gerundive, the supine, the periphrastic conjugations |
| 7 | [07-cases.md](07-cases.md) | the uses of each case, the locative, place and time |
| 8 | [08-prepositions-particles.md](08-prepositions-particles.md) | prepositions by case, *in* with the accusative or the ablative, conjunctions and enclitics, negation, questions, discourse particles, prepositions in late and medieval Latin |
| 9 | [09-simple-clause.md](09-simple-clause.md) | agreement, the null subject, the copula, impersonals, possession, word order measured from Caesar to the charters, hyperbaton, sentence linking |
| 10 | [10-subordination.md](10-subordination.md) | the subjunctive, the sequence of tenses, the accusative with the infinitive, indirect questions and commands, purpose and result, *cum*, conditionals, indirect discourse, the late shift to *quod/quia* |
| 11 | [11-numerals-lexicon.md](11-numerals-lexicon.md) | numerals, Roman numerals, the calendar, years, word formation, the layers of the vocabulary |
| 12 | [12-verse-style.md](12-verse-style.md) | quantity, elision, the hexameter and the elegiac couplet, lyric and dramatic metres, the poetic language, accentual verse |
| 13 | [13-late-latin.md](13-late-latin.md) | late and Christian Latin, the Vulgate: sound changes, the cases and the articles, the *habēre* periphrases, parataxis, Hebraisms and Grecisms, the Christian vocabulary |
| 14 | [14-medieval-latin.md](14-medieval-latin.md) | the periods, the Lombard charters of LLCT with a dating formula read exactly, spelling, the cases in disorder, formulae, scholastic and humanist Latin |
| — | [sources.md](sources.md) | every source cited, with licence; the treebanks; the lexicons Mova would need |

## Measured in this manual

All on UD 2.18 training sets, counted 2026-10-22:

| what | where | numbers |
|---|---|---|
| the order of subject, verb and object | [9](09-simple-clause.md) §6 | Caesar SOV 53%; the Vulgate SVO 44%; Seneca's verse spread over all six orders; the charters spread |
| the accusative with the infinitive against *quod/quia* clauses | [10](10-subordination.md) §11 | Caesar 295 : 11, Cicero 473 : 19, the Vulgate 241 : 340 |
| the share of the subjunctive among finite verbs | [10](10-subordination.md) §11 | Caesar 32.4%, Cicero 29.7%, the Vulgate 13.8%, CIRCSE 12.5%, LLCT 12.8% |
| *et* per 1 000 words | [13](13-late-latin.md) §5 | the Vulgate 75, Caesar 21 |
| the case after *ad* and *cum* | [8](08-prepositions-particles.md) §10, [14](14-medieval-latin.md) §4 | classical: never the wrong case; the charters: *ad* + ablative form 474 of 2 080 |

## Conventions

- **Long vowels** are marked with a macron (*rōsa*, *amāre*) in examples built by us and in paradigms. Quotations from the Vulgate and the treebanks are given as their editions print them: no macrons, *æ* and *œ* in the Clementine Vulgate, the charters' own spelling.
- **Citations** of grammars give the section: *(Bennett 1918 §143)*. A Bible verse is cited by book, chapter and verse (*Gen. 1:4*), a classical author by the usual abbreviation (*Verg. Aen. 1.1*).
- **Treebank names**: LLCT (*Late Latin Charter Treebank*, Lucca, 8th–9th century), CIRCSE (Seneca's tragedies and Tacitus' *Germania*), PROIEL (the Vulgate New Testament, Caesar's *Gallic War*, Cicero's *Letters to Atticus* and *De officiis*, Palladius on farming; NC, counted only).

## Related

- The same package for every language: [PACKAGE.md](../PACKAGE.md).
- French, the Latin of today: [languages/fr](../fr/README.md). The two manuals link to each other where a French rule continues a Latin one (negation, the subjunctive, the *habēre* tenses, *de* and *à*).
- Ancient Greek, the sister classical language, with the same measurements side by side (word order, the infinitive against *quod*/ὅτι, *et* and καί): [languages/grc](../grc/README.md).
- Biblical Hebrew and the Greek of the Septuagint, which the Vulgate translates: [languages/hbo](../hbo/README.md).
- The parallel Bibles, where the Vulgate will be read beside the Hebrew and the English: [Holy Wars](https://mova.elamur.ai/free/en/holy-wars/).
