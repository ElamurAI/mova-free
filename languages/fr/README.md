# Modern Standard French (fr) — a reference manual for Mova

This is an open reference grammar of **modern standard French** (ISO 639-1 `fr`, ISO 639-3 `fra`) as it is written and, where the two differ, as it is spoken. It has chapters on the sounds and the spelling, the nominal and verbal systems, the clause, the punctuation, the regional varieties and the lexicon. It is written for two readers at once: a person who reads English and wants a compact account of how French works, and Mova, whose tools read French text as Universal Dependencies trees. The examples are French with English glosses. Every chapter ends with a section, **How Mova handles it**, that says what Mova's tools do with the point today and where they stop.

The manual is written in our own words. Facts come from public-domain grammars and phonetic manuals (Passy 1906, Nyrop 1902, Martinon 1913, Clédat 1894, Brunot 1899 and others), from the official 1990 orthographic rectifications, and from open-access research. Works under share-alike or non-commercial terms are cited and paraphrased, never copied. The full list, with licences, and the key works we lack, are in [sources.md](sources.md).

**What Mova can do with French today is small.** There is no French lexicon and no French morphology in Mova. French is read by the English tree parser (`en`) after a treebank's tags have been mapped to the tag set it knows (`en xpos-fill`). On UD French GSD (UD 2.18, train → dev, 2026-10-22) that gives LAS 87.00 with the parser's own tags and 89.78 with the treebank's gold tags. The tagger's accuracy on unknown words is 84.8%. The chapters say, construction by construction, what that does and does not cover.

## Chapters

| # | file | what it covers |
|---|---|---|
| 1 | [01-sounds-and-spelling.md](01-sounds-and-spelling.md) | the phonemes, syllables and the mute *e*, liaison and enchaînement, elision and contraction, the letters and their sounds, accents, the 1990 rectifications |
| 2 | [02-noun.md](02-noun.md) | gender and how to guess it, feminine formation, plural formation, irregular plurals, compounds, proper names, number words as nouns, gender-inclusive writing |
| 3 | [03-determiners.md](03-determiners.md) | definite, indefinite and partitive articles, *de* after negation and before adjectives, demonstratives, possessives, quantifiers, cardinal and ordinal numerals |
| 4 | [04-pronouns.md](04-pronouns.md) | subject, object and strong pronouns, the clitic cluster and its order, *y* and *en*, reflexives, *on*, demonstratives, relatives, indefinites |
| 5 | [05-adjectives-adverbs.md](05-adjectives-adverbs.md) | agreement, position and the change of meaning, comparison and superlative, adverbs in *-ment*, adverb position |
| 6 | [06-verb.md](06-verb.md) | the three groups, the stem alternations, the simple tenses, mood, the passé simple and the literary tenses, periphrases, the imperative, non-finite forms |
| 7 | [07-auxiliaries-participle.md](07-auxiliaries-participle.md) | *avoir* and *être*, compound tenses, the agreement of the past participle, pronominal verbs, the passive, causatives |
| 8 | [08-negation.md](08-negation.md) | *ne … pas* and its relatives, negative quantifiers, *ne … que*, expletive *ne*, scope, *de* after negation |
| 9 | [09-questions.md](09-questions.md) | yes/no questions by intonation, *est-ce que* and inversion, wh-questions in three registers, answers, indirect questions |
| 10 | [10-clause-structure.md](10-clause-structure.md) | word order, subjects and objects, impersonal constructions, dislocation, clefts and pseudo-clefts, information structure, the place of adverbs and complements |
| 11 | [11-subordination.md](11-subordination.md) | complement clauses, the subjunctive and when it is required, relative clauses, conditional clauses and the sequence of tenses, adverbial clauses, infinitival clauses |
| 12 | [12-punctuation-typography.md](12-punctuation-typography.md) | the French spaces, the guillemets and dialogue, capitals, numbers and dates, abbreviations, hyphens and apostrophes |
| 13 | [13-varieties-registers.md](13-varieties-registers.md) | Québec, Belgium, Switzerland, Africa, spoken and written French, *ne* dropping, *on* for *nous*, the registers |
| 14 | [14-lexicon.md](14-lexicon.md) | word formation, Latinate doublets, loanwords, false friends with English, multiword expressions, dictionaries |
| — | [sources.md](sources.md) | every source cited, with licence and where it is held; the wanted list |

French continues spoken Latin; the [Latin manual](../la/README.md) gives the source of its negation, its subjunctive, its *avoir* tenses and its prepositions *de* and *à* ([la chapter 13](../la/13-late-latin.md)).

## Conventions

**Examples** are given in three lines:

> **fr-ud-dev_00007** · UD French GSD
> Elle lutte pour échapper aux tueurs à ses trousses.
> *she fights for escape-INF to-the killers at her heels* — "She is fighting to escape the killers on her heels."

- A bold id such as `fr-ud-dev_00007` is the `sent_id` of a sentence of an open treebank (UD French GSD unless named otherwise: ParisStories, Rhapsodie, Sequoia, ParTUT, PUD, FQB). It can be looked up in the UD 2.18 release. Treebank examples are short quotations; the treebanks are CC BY-SA, LGPL-LR or CC BY-NC-SA (see [sources.md](sources.md)), and the manual does not reproduce any of them in bulk. Their text is kept as published, including its errors and its spacing.
- An example without an id is constructed by us.
- A starred example (\*) is ungrammatical in the standard; a question mark (?) marks a doubtful one; `%` marks a form that is normal in speech and avoided in careful writing.
- The word-for-word gloss is in italics, with hyphens joining the English words that translate one French word and the usual abbreviations (SG, PL, F, M, 1, 2, 3, INF, PTCP, SUBJ, IMP, REFL, CL for a clitic). The translation is in double quotes.

**Spelling.** French is given in its standard orthography with accents. Where the 1990 rectified spelling differs, the traditional one is used in the examples (it is what the treebanks have) and the rectified one is given in [chapter 1](01-sounds-and-spelling.md).

**Phonetic notation.** Pronunciation is in IPA between slashes, for example /ʁə/ for the mute *e* word *re*. The notation is broad: it shows phonemes, not detail of realisation. The mute *e* is written (ə) when it is optional.

**Grammatical terms.** The terms follow the traditional French grammars where they are standard (the *passé simple*, the *conditionnel*), with an English equivalent in brackets at first use. The UD labels (`nsubj`, `aux:pass`, `expl:comp`) are in code type. Where the UD French GSD tree differs from the traditional analysis, the section **How Mova handles it** says so.

**Citations.** In the text a work is cited as `(author-year)` with the slug of its card in the research library where a card exists, for example `(passy-1906)`. A citation names the work, not a page. A point cited to a work was checked in the text we hold; the large body of standard description that every reference grammar shares (the paradigms, the basic agreement rules) is not attributed to a single work. Works we know only by their card are marked *card only* in [sources.md](sources.md), and no claim rests on them except where the sentence says "reported in".

**Numbers.** Every count is measured on our data and given with its date. Treebank counts are on UD 2.18 files as released (2026-10-22 unless said otherwise).

## The data behind the examples

| what | where | licence |
|---|---|---|
| UD French GSD (400 k words) | `$RESEARCH_RAW/fr-ud/ud-treebanks-v2.18/UD_French-GSD/` | CC BY-SA 4.0 |
| UD French ParisStories (spoken, with liaison features) | `$RESEARCH_RAW/fr-ud/ud-treebanks-v2.18/UD_French-ParisStories/` | CC BY-SA 4.0 |
| UD French Rhapsodie (spoken, with prosody) | `UD_French-Rhapsodie/` | CC BY-SA 4.0 |
| UD French Sequoia (Europarl, Est Républicain, Wikipedia, medical) | `UD_French-Sequoia/` | LGPL-LR |
| UD French ParTUT, FQB, PUD | `UD_French-ParTUT/`, `UD_French-FQB/`, `UD_French-PUD/` | CC BY-NC-SA 4.0, LGPL-LR, CC BY-SA 3.0 |
| the 1990 rectifications (transcription of the *Journal officiel* text) | `$RESEARCH_RAW/fr-gram-rectifications-orthographe-1990/` | official text |

Non-commercial and research-only data is used only for tests and measurements, never in what is built.

*The Elamur team*
