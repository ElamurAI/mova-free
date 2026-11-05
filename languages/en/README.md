# English: a reference manual for Mova

This folder is an open reference grammar of **modern standard English**, written for Mova and for anyone who reads Mova's code. It describes the language as it is spoken and written today, with notes on the main national varieties and on the older forms that still turn up in the texts Mova reads (the King James Bible, the JPS 1917 Torah, Shakespeare, Victorian tales).

Each chapter states the facts, gives the rules and their exceptions, shows tables and many examples, cites its sources, and says **how Mova handles the point** today: which file implements it, which seed describes it, and where Mova does not handle it yet.

The manual is in two halves written by two authors. The **form** half (sounds, spelling, inflection, word formation, the lexicon) is in the files `0x-*` and `1x-*`. The **syntax and pragmatics** half is in the files `2x-*` and `3x-*` (chapters 20–38).

## How to read it

- Sounds are written in the IPA between slashes for phonemes (`/kæt/`) and square brackets for actual pronunciations (`[kʰæʔ]`). Spellings are in italics or angle brackets (`⟨ough⟩`). The vowel transcription is described in [02-vowels.md](02-vowels.md).
- Word classes and forms are named twice: in traditional terms and in the tags Mova uses, the **Penn Treebank (PTB) tags** (`NNS`, `VBD`, `JJR`…) and **Universal Dependencies (UD)** part-of-speech tags and features (`NOUN`, `Number=Plur`). [10-word-classes.md](10-word-classes.md) has the full table.
- Examples from Mova's own data carry their ids so they can be looked up:
  - `school:<course>:<lesson>:<n>`: a sentence from the school lessons (`$MOVA_DATA/school/courses/`), as stored in the parse database (`$MOVA_DATA/parsedb/current.conllu`);
  - `dict:…`, `book:…`: sentences from the pupil dictionary and the book lessons in the same database;
  - `ewt:<sent_id>`: UD English EWT 2.18, train set unless stated; GUM 2.18 is CC BY-NC-SA, so these chapters only count it and quote none of its sentences.
  - Unmarked examples are constructed for this manual.
- Measurements carry their date. Counts made for this manual were made on **2026-10-17** over the files named next to them.
- Sources are cited briefly in the text, e.g. *(Sweet NEG I §1001)*, and listed in full, with library paths and open URLs, in [sources.md](sources.md). Each chapter also ends with its own short list.

## Contents

### Sounds and writing

| file | what it covers |
|---|---|
| [00-introduction.md](00-introduction.md) | What English is: its family, history in brief, typological profile, standard and varieties, the make-up of its vocabulary; the notation of this manual |
| [01-consonants.md](01-consonants.md) | The 24 consonant phonemes, their allophones (aspiration, flapping, glottalling, dark *l*, rhotic and non-rhotic *r*), phoneme frequencies |
| [02-vowels.md](02-vowels.md) | Vowel systems of Southern British (SSBE/RP) and General American; the lexical sets; reduced vowels; the main splits and mergers |
| [03-syllable-stress-intonation.md](03-syllable-stress-intonation.md) | Phonotactics and the syllable; word stress and its rules; stress in compounds and phrases; stress shifts with suffixes; rhythm and intonation in outline |
| [04-connected-speech.md](04-connected-speech.md) | Weak forms, assimilation, elision, linking and intrusive *r*, contractions in speech; the morphophonology of *-s*, *-ed*, *-ing* and other alternations |
| [05-writing-system.md](05-writing-system.md) | The alphabet and letter names; graphemes; how sounds are spelled and how spellings are read; silent letters; why English spelling looks the way it does |
| [06-spelling-rules.md](06-spelling-rules.md) | The rules of inflectional spelling (doubling, silent *e*, *y → i*, *-es*); spelling of derived words; compounds and hyphens; national spelling standards |
| [07-punctuation-capitals-numbers.md](07-punctuation-capitals-numbers.md) | Punctuation marks, the apostrophe, capital letters, abbreviations, numbers, dates and symbols, and how Mova's tokenizer treats them |
| [08-national-varieties.md](08-national-varieties.md) | British, American, Canadian, Irish, Scottish, Australian, New Zealand, South African, Indian, West African and Singaporean English: sounds, spelling, forms, words |
| [09-early-modern-and-archaic.md](09-early-modern-and-archaic.md) | *Thou* and *ye*, *-est* and *-eth*, old strong verbs, *art*, *wilt*, *spake*; the forms of the King James Bible and Shakespeare; how Mova lemmatizes them |

### Forms and words

| file | what it covers |
|---|---|
| [10-word-classes.md](10-word-classes.md) | The word classes, open and closed; tests for each; PTB and UD tags; words that belong to several classes |
| [11-nouns.md](11-nouns.md) | Number (regular and irregular plurals, foreign plurals, zero plurals, pluralia tantum), countability, the genitive *'s*, gender, proper nouns |
| [12-pronouns-determiners-numerals.md](12-pronouns-determiners-numerals.md) | Personal, possessive, reflexive, reciprocal, demonstrative, interrogative, relative and indefinite pronouns; articles and other determiners; numerals |
| [13-verbs.md](13-verbs.md) | The five forms of the lexical verb; regular inflection; *be*, *have*, *do*; the modals; negative and contracted forms; the forms of tense, aspect, mood and voice |
| [14-irregular-verbs.md](14-irregular-verbs.md) | The irregular verbs by class, with a full table, British and American variants, and counts from AGID and EWT |
| [15-adjectives-adverbs.md](15-adjectives-adverbs.md) | Comparison (*-er/-est* and *more/most*), irregular comparison, adverbs in *-ly*, flat adverbs, participial adjectives |
| [16-derivation-prefixes.md](16-derivation-prefixes.md) | Prefixes: negative, reversive, locative, temporal, numeral, evaluative, neoclassical; their meanings, bases and spelling |
| [17-derivation-suffixes.md](17-derivation-suffixes.md) | Suffixes that make nouns, adjectives, verbs and adverbs; productivity; stress and sound changes they cause; native and Latinate strata |
| [18-compounding-conversion-minor.md](18-compounding-conversion-minor.md) | Compounds and their stress and spelling; conversion; back-formation; clipping; blends; acronyms and initialisms; reduplication; neoclassical compounds |
| [19-lexicon-mwe-register.md](19-lexicon-mwe-register.md) | The make-up of the vocabulary; borrowing; multiword expressions (phrasal verbs, idioms, collocations, binomials); register, slang and taboo; frequency |

### Syntax and pragmatics

| file | what it covers |
|---|---|
| [20-syntax-overview.md](20-syntax-overview.md) | Syntax in the Universal Dependencies frame |
| [21-clause-structure.md](21-clause-structure.md) | The simple clause |
| [22-phrases.md](22-phrases.md) | Noun, adjective, adverb and prepositional phrases |
| [23-complementation.md](23-complementation.md) | Complementation |
| [24-coordination-subordination.md](24-coordination-subordination.md) | Coordination and subordination |
| [25-relative-clauses.md](25-relative-clauses.md) | Relative clauses |
| [26-questions.md](26-questions.md) | Questions |
| [27-negation.md](27-negation.md) | Negation |
| [28-voice-and-passive.md](28-voice-and-passive.md) | Voice and the passive |
| [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md) | Ellipsis, word order and information structure |
| [30-pragmatics-overview.md](30-pragmatics-overview.md) | Pragmatics and discourse: overview |
| [31-speech-acts.md](31-speech-acts.md) | Speech acts |
| [32-politeness-and-hedging.md](32-politeness-and-hedging.md) | Politeness and hedging |
| [33-implicature-and-presupposition.md](33-implicature-and-presupposition.md) | Implicature and presupposition |
| [34-irony-and-figurative-language.md](34-irony-and-figurative-language.md) | Irony and figurative language |
| [35-reference-and-coreference.md](35-reference-and-coreference.md) | Reference and coreference |
| [36-discourse-relations.md](36-discourse-relations.md) | Discourse relations and coherence |
| [37-register-and-variation.md](37-register-and-variation.md) | Register and variation |
| [38-dialogue-and-discourse-markers.md](38-dialogue-and-discourse-markers.md) | Dialogue, discourse markers and reported speech |

The form chapters (00–19) link to these where a point belongs to syntax or pragmatics.

### Sources

- [sources.md](sources.md): every source cited in the manual, with its licence, the path in the research library and an open URL, in two parts (chapters 00–19, chapters 20–38); the works still **wanted**.

## Mova's English machinery at a glance

| point | where it lives | chapter |
|---|---|---|
| tokenization (clitics *n't*, *'s*; *cannot*; abbreviations; hyphens) | `en/src/tok.rs` | [07](07-punctuation-capitals-numbers.md), [13](13-verbs.md) |
| tagging (PTB tags) | `en/src/tag.rs`, `en/src/gram.rs` | [10](10-word-classes.md) |
| lemmatization and inflection | `en/src/morph.rs` (`Morph::lemmatize`, `Morph::inflect`, `suffix`, `unsuffix`) | [06](06-spelling-rules.md), [11](11-nouns.md), [13](13-verbs.md), [14](14-irregular-verbs.md), [15](15-adjectives-adverbs.md) |
| Early Modern verb forms | `en/src/morph.rs` (`archaic`) | [09](09-early-modern-and-archaic.md) |
| the built-in lexicon | `en/data/lemmas.tsv`, `en/data/forms.tsv` (AGID + UD counts), compiled by `en/build.rs` | [14](14-irregular-verbs.md) |
| grammar seeds (facts with checkable rules) | `en/seeds/morph/`, `en/seeds/nominal/`, `en/seeds/verbal/`, `en/seeds/lexicon/`, `en/seeds/errors/` | every chapter |
| expressions, idioms, register words | `global::expressions_in_sentence`, `prag::lens` ([docs/pragmatics.md](../../docs/pragmatics.md)); register lexicons `global/data/register-*.tsv` | [19](19-lexicon-mwe-register.md) |
| the pupil dictionary | `world/data/dictionary-improved.tsv` | [19](19-lexicon-mwe-register.md) |

## Licence

The text of this manual is ours, under the Elamur Cosmic License ([../../LICENSE.md](../../LICENSE.md)). Short quotations and examples from other works keep their own licences, named in [sources.md](sources.md): example sentences from UD EWT (CC BY-SA 4.0) are quoted sparingly, never carried in bulk; GUM (CC BY-NC-SA 4.0) is used for measurement only (see [licence/DATA.md](../../licence/DATA.md)).

*The Elamur team*
