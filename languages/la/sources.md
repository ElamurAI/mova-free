# Sources

The manual is written in our own words. The works below are where its facts were checked. Public-domain grammars and open-access papers are the base. Works under share-alike or non-commercial terms are cited and paraphrased, never copied. Copyrighted reference grammars are known to us only by their cards, and no claim rests on them. Each work has a card in the Elamur Research library under its slug. The chapters that cite a work are listed after it, so each source links back to where it is used.

## Grammars (public domain)

| work | slug | licence | cited in |
|---|---|---|---|
| Charles E. Bennett, *New Latin Grammar* (1918) | `bennett-1918-new-latin-grammar` | public domain (Project Gutenberg #15665) | [1](01-sounds-spelling.md)–[12](12-verse-style.md): the base of the classical chapters, by section |
| Henry John Roby, *A Grammar of the Latin Language from Plautus to Suetonius* (1875/1887) | `roby-1875-grammar-latin-language-from` | public domain | [1](01-sounds-spelling.md), [2](02-nouns.md) |
| Basil L. Gildersleeve & Gonzalez Lodge, *Gildersleeve's Latin Grammar* (1895) | `gildersleeve-1895-gildersleeves-latin-grammar` | public domain | general reference ([9](09-simple-clause.md), [10](10-subordination.md)) |
| George M. Lane, *A Latin Grammar for Schools and Colleges* (1898/1903) | `lane-1903-latin-grammar-schools-colleges` | public domain (Project Gutenberg #44653) | general reference |
| William Gardner Hale & Carl Darling Buck, *A Latin Grammar* (1903) | `hale-1903-latin-grammar` | public domain (EU from 2026) | general reference ([10](10-subordination.md)) |
| Raphael Kühner & Carl Stegmann, *Ausführliche Grammatik der lateinischen Sprache* (1912–1914) | `kuhner-1912-ausfuhrliche-grammatik-der-lateinischen` | public domain | general reference ([10](10-subordination.md)) |

## Grammars (share-alike or copyrighted: cited, not used as text)

| work | slug | licence | use |
|---|---|---|---|
| J. H. Allen & J. B. Greenough, *New Latin Grammar* (1903), Perseus edition | `allen-1903-allen-greenoughs-new-latin` | text CC BY-SA 4.0 (Perseus) | checked against, never copied |
| Harm Pinkster, *The Oxford Latin Syntax* (2015–2021) | `pinkster-2015-oxford-latin-syntax-vol` | © Oxford University Press | card only |
| Manu Leumann, J. B. Hofmann & Anton Szantyr, *Lateinische Grammatik* (1963–1977) | `leumann-1963-lateinische-grammatik` | © C. H. Beck | card only |
| Hermann Menge, *Lehrbuch der lateinischen Syntax und Semantik* (rev. 2000) | `menge-2000-lehrbuch-der-lateinischen-syntax` | © WBG | card only |
| Michael Weiss, *Outline of the Historical and Comparative Grammar of Latin* (2020) | `weiss-2020-outline-historical-comparative-grammar` | © Beech Stave Press | card only |
| Frederic M. Wheelock, *Wheelock's Latin* (7th ed., 2011) | `wheelock-2011-wheelocks-latin` | © HarperCollins | card only |

## Texts

| text | where | licence | cited in |
|---|---|---|---|
| The Clementine Vulgate (1592), eBible `latVUC`, with the *Glossa ordinaria* in its notes | the Scientist's scripture corpora | public domain | [8](08-prepositions-particles.md), [10](10-subordination.md), [13](13-late-latin.md) |
| Virgil, *Aeneid* 1.1 and 4.139 | quoted as the standard examples of the hexameter and of interlaced order | public domain | [9](09-simple-clause.md), [12](12-verse-style.md) |

## Treebanks (UD 2.18)

| treebank | what it holds | licence | use | cited in |
|---|---|---|---|---|
| UD Latin LLCT | charters from Lucca, 8th–9th century | CC BY-SA 4.0 | Mova's Latin measure (train → dev); counts; short quotations | [8](08-prepositions-particles.md), [9](09-simple-clause.md), [10](10-subordination.md), [14](14-medieval-latin.md) |
| UD Latin CIRCSE | Seneca's tragedies, Tacitus' *Germania* | CC BY-SA 4.0 | counts | [8](08-prepositions-particles.md), [9](09-simple-clause.md), [10](10-subordination.md), [12](12-verse-style.md) |
| UD Latin PROIEL | the Vulgate New Testament, Caesar, Cicero, Palladius | CC BY-NC-SA 4.0 | counts only, never quoted | [9](09-simple-clause.md), [10](10-subordination.md), [13](13-late-latin.md) |

## Research (open access)

| work | slug | licence | use |
|---|---|---|---|
| Federica Gamba & Daniel Zeman, Universalising Latin Universal Dependencies: a harmonisation of Latin treebanks in UD (2023) | `gamba-2023-universalising-latin-universal-dependencies-harmonisation` | CC BY 4.0 | the annotation of the treebanks compared ([8](08-prepositions-particles.md) §11) |
| Federica Gamba & Daniel Zeman, Latin morphology through the centuries: ensuring consistency for better language processing (2023) | `gamba-2023-latin-morphology-through-centuries-ensuring` | CC BY 4.0 | the morphology of the stages ([13](13-late-latin.md), [14](14-medieval-latin.md)) |
| Marisa Hudspeth, Brendan O'Connor & Laure Thompson, Latin treebanks in review: an evaluation of morphological tagging across time (2024) | `hudspeth-2024-latin-treebanks-review-evaluation-morphological` | CC BY 4.0 | why a tagger trained on one period does not carry to another ([14](14-medieval-latin.md) §9) |
| Marco Passarotti & Eleonora Litta, The LiLa Knowledge Base (2026) | `la-nlp-2026-passarotti-lila-knowledge-base` | CC BY 4.0 | the lemma bank a Latin lexicon would link to ([13](13-late-latin.md) §8) |
| Francesco Mambrini, Eleonora Litta, Marco Passarotti & Paolo Ruffolo, Linking the Lewis & Short dictionary to the LiLa knowledge base (2021) | `mambrini-2021-linking-lewis-short-dictionary-lila` | CC BY 4.0 | the open dictionary candidate |
| Rachele Sprugnoli, Federica Iurescia & Marco Passarotti, Overview of the EvaLatin 2024 evaluation campaign (2024) | `sprugnoli-2024-overview-evalatin-2024-evaluation-campaign` | CC BY-NC 4.0 | the shared-task scores Mova's Latin is compared with; cited, not copied |
| Federica Iurescia, Marco Passarotti & Rachele Sprugnoli, Overview of the dependency parsing task at EvaLatin 2026 (2026) | `iurescia-2026-overview-dependency-parsing-task-evalatin` | CC BY-NC 4.0 | the same, for parsing |

## What Mova would need, and where it may come from

| need | candidates | licence to check |
|---|---|---|
| a Latin morphology (case, mood, lemma) | Lemlat; LatinCy's lemmatizer and tagger; Whitaker's *Words* | each its own; open-source code, data licences differ |
| a Latin dictionary | Lewis & Short (1879, public domain; the Perseus text is CC BY-SA); Wiktionary (CC BY-SA, tests only) | text public domain, editions vary |
| a spelling normalizer for medieval Latin | built by us from the interchanges of [chapter 14](14-medieval-latin.md) §3 | ours |
| versification of the Vulgate against the Hebrew | the STEPBible tables in the library | CC BY 4.0 |
