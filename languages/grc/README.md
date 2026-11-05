# Ancient Greek (grc) — a reference manual for Mova

This is an open reference grammar of **Ancient Greek** (ISO 639-3 `grc`). It takes the Attic Greek of the fifth and fourth centuries BC as its norm, and has chapters on the dialects, on Homer and on the passage to the Koine of the Septuagint and the New Testament. It is written for two readers at once:
- a person who reads English and wants a compact account of how Greek works;
- Mova, whose tools read Greek text as Universal Dependencies trees.

Every chapter ends with a section, **How Mova handles it**, that says what Mova's tools do with the point today and where they stop.

The manual is written in our own words. Facts come from:
- Smyth's *Greek Grammar* (1920; the Perseus text is CC BY-SA, so it is cited by section and paraphrased, never copied);
- public-domain grammars: Goodwin, Kühner–Blass–Gerth, Monro for Homer, Thackeray, Blass and Burton for the Koine.

Counts come from the open UD treebank PTNK (the Septuagint, CC BY-SA). The classical treebanks Perseus and PROIEL are CC BY-NC-SA: they are counted, never quoted. The sources and their licences are in [sources.md](sources.md).

**What Mova can do with Greek today is small.** There is no Greek lexicon and no Greek morphology in Mova. Greek is read by the English tree parser (`en`) after a treebank's tags have been mapped to the tag set it knows (`en xpos-fill`). On UD Ancient Greek PTNK (UD 2.18, train → dev, 2026-10-22) that gives LAS 74.41 with the parser's own tags and 78.29 with the treebank's gold tags. The open parser trained on the same treebank (Stanza) reaches 80.65 ([docs/parsers.md](../../docs/parsers.md)). The tag set has no case, mood or aspect, and much of Greek syntax rests on them: almost every chapter's last section comes back to that.

## Chapters

| # | file | what it covers |
|---|---|---|
| 1 | [01-script-sounds.md](01-script-sounds.md) | the alphabet, vowels and diphthongs, breathings, consonants, sound changes: gradation, contraction, elision, crasis |
| 2 | [02-accent.md](02-accent.md) | the pitch accent, its rules and its marks, enclitics and proclitics |
| 3 | [03-noun.md](03-noun.md) | gender, number, case; the three declensions; how the treebanks mark them |
| 4 | [04-adjectives.md](04-adjectives.md) | adjectives, adverbs, comparison, numerals |
| 5 | [05-article.md](05-article.md) | the article: forms, uses, omission, attributive and predicate position, ὁ μέν … ὁ δέ |
| 6 | [06-pronouns.md](06-pronouns.md) | personal, αὐτός, reflexive, possessive, demonstrative, τίς/τις, relative, correlatives; the Septuagint's pronouns |
| 7 | [07-verb.md](07-verb.md) | the verb system: aspect and time, tense stems, augment, reduplication, endings, voice, principal parts |
| 8 | [08-contract-verbs.md](08-contract-verbs.md) | the contract verbs in -άω, -έω, -όω |
| 9 | [09-mi-verbs.md](09-mi-verbs.md) | the -μι verbs, εἰμί, εἶμι, φημί, οἶδα |
| 10 | [10-nonfinite.md](10-nonfinite.md) | the infinitive, the articular infinitive, the participle, the genitive absolute, the verbal adjectives |
| 11 | [11-cases.md](11-cases.md) | five cases for eight functions; place and time |
| 12 | [12-prepositions-particles.md](12-prepositions-particles.md) | prepositions by case, οὐ and μή, the particles |
| 13 | [13-simple-clause.md](13-simple-clause.md) | agreement, the neuter plural, omitted subject and copula, word order measured |
| 14 | [14-subordination.md](14-subordination.md) | the moods, ἄν, purpose, result, conditions, indirect discourse; classical to Koine measured |
| 15 | [15-verse.md](15-verse.md) | quantity, the hexameter, the trimeter, what metre does to the language |
| 16 | [16-dialects.md](16-dialects.md) | Ionic, Doric, Aeolic, and the language of Homer |
| 17 | [17-koine.md](17-koine.md) | from Attic to the Koine; the Septuagint as translation Greek; the measurements in one table |
| 18 | [18-lexicon.md](18-lexicon.md) | word formation, compounds, loans; the open dictionaries and their licences |
| — | [sources.md](sources.md) | every source cited, with licence |

## Measured in this manual

All on UD 2.18 training sets, counted 2026-10-22 (PROIEL and Perseus counted only):

| what | where | numbers |
|---|---|---|
| the order of subject, verb and object | [13](13-simple-clause.md) §5 | classical verse OVS 30%; Herodotus SVO 37%; New Testament SVO 47%; Septuagint VSO 69% |
| the optative, share of finite verbs | [14](14-subordination.md) §9 | Herodotus 3.9%, New Testament 0.3%, Septuagint 2.0% |
| accusative + infinitive : ὅτι clauses | [14](14-subordination.md) §9 | Herodotus 583 : 69, New Testament 398 : 875, Septuagint 23 : 93 |
| καί per 1 000 words | [12](12-prepositions-particles.md) §9 | Septuagint 76 (the Vulgate's *et*: 75) |
| the aorist among indicatives | [7](07-verb.md) §2 | Septuagint 1 264 of 1 942 |
| the possessive αὐτοῦ | [6](06-pronouns.md) §10 | Septuagint: 414 of 915 tokens of αὐτός |
| τοῦ + infinitive | [10](10-nonfinite.md) §3 | Septuagint: 41 of 58 articular infinitives |
| how the three treebanks tag the article | [5](05-article.md) §10 | PTNK, Perseus, PROIEL compared |

## Conventions

- **Greek** is in its standard polytonic spelling with accents and breathings. Long α, ι, υ are marked only in paradigms where it matters (ᾱ).
- **Citations**: grammars by section, *(smyth-1920 §1150)*; classical texts by author, work and section with their catalogue id (*Xen. An. 1.1.2*, `tlg0032.tlg006`); the Septuagint by book, chapter and verse and the PTNK `sent_id`.
- **Treebanks**: PTNK (the Septuagint: Genesis and Ruth in the training set; CC BY-SA); Perseus (Homer, Hesiod, Sophocles, Aeschylus, Herodotus and others; NC); PROIEL (Herodotus and the New Testament; NC).

## Related

- The same package for every language: [PACKAGE.md](../PACKAGE.md).
- Latin, the sister classical language, with the same measurements side by side: [languages/la](../la/README.md).
- Biblical Hebrew, the source of the Septuagint: [languages/hbo](../hbo/README.md).
- The parallel Bibles, where the Septuagint is read beside the Hebrew and the English: [Holy Wars](https://mova.elamur.ai/free/en/holy-wars/).
