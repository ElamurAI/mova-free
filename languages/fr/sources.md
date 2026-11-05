# Sources

The manual is written in our own words. The works below are where its facts were checked. Public-domain works and open-access papers are the base; works under share-alike or non-commercial terms are cited and paraphrased, never copied; copyrighted reference grammars are known to us only by their cards and no claim rests on them. Each work has a card on [Elamur Research](https://research.elamur.ai/free/) under its slug. The chapters that cite a work are listed after it, so each source links back to where it is used.

## Grammars, phonetics and spelling (public domain)

| work | slug | licence | cited in |
|---|---|---|---|
| Paul Passy, *Les sons du français* (6th ed., 1906) and *Abrégé de prononciation française* | `passy-1906` | public domain | [1](01-sounds-and-spelling.md) |
| Kristoffer Nyrop, *Manuel phonétique du français parlé* (2nd ed., trans. Philipot, 1902) | `nyrop-1902` | public domain | [1](01-sounds-and-spelling.md) |
| Philippe Martinon, *Comment on prononce le français* (1913) | `martinon-1913` | public domain | [1](01-sounds-and-spelling.md) |
| Léon Clédat, *Grammaire raisonnée de la langue française* (1894) | `cledat-1894` | public domain | general reference |
| Ferdinand Brunot, *Précis de grammaire historique de la langue française* (1899) | `brunot-1899` | public domain | general reference, [14](14-lexicon.md) |
| Louis-Nicolas Bescherelle, *L'art de conjuguer* (1848 ed.) | `bescherelle-1848` | public domain | [6](06-verb.md) |
| Conseil supérieur de la langue française, *Les rectifications de l'orthographe* (Journal officiel, 6 December 1990) | `cslf-1990` | official text, freely reusable | [1](01-sounds-and-spelling.md), [2](02-noun.md), [12](12-punctuation-typography.md) |

## Research (open access)

| work | slug | licence | cited in |
|---|---|---|---|
| Saumya Sahai & Dravyansh Sharma, Predicting and explaining French grammatical gender (2021) | `sahai-2021` | CC BY 4.0 | [2](02-noun.md) |
| Enzo Doyen & Amalia Todirascu, GeNRe: a French gender-neutral rewriting system using collective nouns (2025) | `doyen-2025` | CC BY 4.0 | [2](02-noun.md) |
| Marie Flesch & Heather Burnett, Characterizing non-binary French: a first step towards debiasing gender inference (2025) | `flesch-2025` | CC BY 4.0 | [2](02-noun.md) |
| Marc Olivier, Christina Sevdali & Raffaella Folli, Clitic climbing and restructuring in the history of French (Glossa, 2023) | `olivier-2023` | CC BY 4.0 | [4](04-pronouns.md) |
| Bruno Guillaume, Marie-Catherine de Marneffe & Guy Perrier, Conversion et améliorations de corpus du français annotés en Universal Dependencies (2019) | `guillaume-2019` | CC BY 4.0 | [README](README.md), How Mova handles it |
| de Marneffe et al., Universal Dependencies (2021) | `demarneffe-2021` | CC BY 4.0 | How Mova handles it, throughout |

## Treebanks (the examples and the counts)

| treebank (UD 2.18) | licence | use here |
|---|---|---|
| UD French GSD | CC BY-SA 4.0 | short quoted examples (`fr-ud-…` ids), counts |
| UD French ParisStories (spoken) | CC BY-SA 4.0 | short quoted examples, counts (chapters [8](08-negation.md), [9](09-questions.md), [10](10-clause-structure.md), [13](13-varieties-registers.md)) |
| UD French Rhapsodie (spoken) | CC BY-SA 4.0 | measurements |
| UD French Sequoia, FQB | LGPL-LR | measurements |
| UD French ALTS, PUD, PoitevinDIVITAL | CC BY-SA | measurements |
| UD French ParTUT | CC BY-NC-SA 4.0 | tests only, never quoted |

## Lexicons (for the lexicon Mova lacks: [chapter 14](14-lexicon.md))

| resource | slug of its paper | licence | use |
|---|---|---|---|
| Lefff | `sagot-2010` | the paper CC BY-NC 4.0; the lexicon LGPL-LR (to be confirmed from its distribution) | candidate |
| GLÀFF | `sajous-2013`, `hathout-2014` | the papers CC BY-NC(-SA); the lexicon derived from Wiktionnaire, CC BY-SA | learning and tests only |
| Morphalou | — | the lexicon: LGPL-LR (ATILF, to be confirmed) | candidate |
| French Wiktionary (raw dump, 2026-10-01) | — | CC BY-SA | learning and tests only |

## Wanted, copyrighted: known by card only

| work | slug | why we want it |
|---|---|---|
| Grevisse & Goosse, *Le Bon Usage* (16th ed., 2016) | `grevisse-2016` | the reference descriptive grammar; usage and its variation |
| Riegel, Pellat & Rioul, *Grammaire méthodique du français* (2021) | `riegel-2021` | the modern linguistic reference grammar |
| *Lexique des règles typographiques en usage à l'Imprimerie nationale* | — | the typography of [chapter 12](12-punctuation-typography.md) |
| A descriptive grammar of spoken French with corpus counts (e.g. Blanche-Benveniste's work) | — | [chapters 8–10, 13](13-varieties-registers.md) |

No fact in the manual rests on these; where the manual states a usage rule they would confirm, it was checked against the public-domain grammars and our own counts.
