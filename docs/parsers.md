# Open parsers: witnesses and tests

This page covers the open parsers Mova uses as witnesses and test references: for English, and for French, Latin, Ancient Greek, Biblical Hebrew and Modern Hebrew. It says which licences allow what, how the parsers are used, and which parser serves which role. The papers behind the parsers are in the library.

## What it is

Open parsers and their models are supplied separately; Mova does not ship them. They serve three purposes:

- **Witnesses** for the teacher's trees: where independent parsers agree on a head and the teacher disagrees, the word is a suspect to check first.
- **Test references** for Mova's own `en` parser and for the parsers of the language packages.
- **Candidates** for transparent rule-based components (Link Grammar, ACE + ERG, Grammatical Framework).

A witness runner is configured through `CYCLE_NEURAL` ([tools/cycle](../tools/cycle/README.md)). Its inputs and outputs must keep token alignment, and each model's licence is checked apart from its code's licence.

## Licences: build vs test

By the data-licence rule ([licence/DATA.md](../licence/DATA.md)), share-alike, non-commercial and research-only models are for **tests and measurements only**, never built into Mova or shipped. A permissive code licence does not settle it: a model trained on restricted treebanks needs a data-licence check before any build use.

| parser | tree | model licence | use |
|---|---|---|---|
| UDPipe 1 (UD 2.5), UDPipe 2 (UD 2.15) | UD | CC BY-NC-SA 4.0 | tests only |
| UDify | UD (multilingual BERT) | LINDAT, NC | tests only |
| Stanza | UD; PTB constituents | Apache-2.0 code; models trained on UD/PTB (some UD treebanks NC: grc/hbo PTNK, he HTB) | tests; build only after a data-licence check |
| Trankit (XLM-R) | UD | Apache-2.0; trained on UD | tests |
| spaCy `en_core_web_*` | ClearNLP-style dependencies | MIT (trained on OntoNotes) | tests; build only after a data-licence check |
| spaCy `fr_core_news_*`, `fr_dep_news_trf` | UD-style, Sequoia labels | LGPL-3.0 | tests |
| LatinCy `la_core_web_*` | UD (harmonised) | MIT | tests |
| OdyCy `grc_odycy_joint_*` | UD (Perseus/PROIEL) | MIT | tests |
| GreCy `grc_dep_*_trf` | UD | not stated | tests only |
| DictaBERT joint (he) | UD (HTB or IAHLT style) | CC BY 4.0; training data not checked | tests |
| YAP (he) | SPMRL labels | Apache-2.0 code; MILA models and lexicon | tests only |
| Stanford CoreNLP | Stanford/UD dependencies, PTB constituents | GPL-3.0 | tests only |
| Berkeley Neural Parser (benepar) | PTB constituents | MIT (trained on PTB) | tests |
| SuPar | dependencies, constituents, semantic dependencies | MIT (trained on PTB/UD/SDP) | tests |
| MaltParser + `engmalt` | Stanford-basic dependencies | BSD code; maltparser.org model terms | tests |
| AllenNLP, EasyCCG, depccg | dependencies, constituents, CCG | Apache-2.0 / MIT (trained on PTB, CCGbank) | tests |
| BLLIP (Charniak–Johnson) | PTB constituents | unclear (trained on PTB/LDC) | tests only |
| Link Grammar | links (hand-written rules) | LGPL-2.1 | candidate for build: transparent rules |
| ACE + ERG 2018 | HPSG + MRS semantics | MIT-like | candidate for build: transparent rules |
| Grammatical Framework + RGL + gf-wordnet | abstract syntax trees | LGPL/BSD | candidate for build: real ASTs |
| syntaxdot (Rust) | dependencies | MIT/Apache-2.0 | code to read; no English model published |

Code only, no pretrained English model: Enju (HPSG), TurboParser, Hexatagger, DiaParser. CLTK has no parser of its own (it wraps Stanza or a large model) and is not set up.

## How they are used

- **School witnesses.** `tools/cycle` runs the neural parsers over school sentences (runner path in `CYCLE_NEURAL`), pretokenized so trees align token by token; [`tools/parsecmp`](../tools/parsecmp/README.md) compares them and `tools/treeagree` merges their agreement with the teacher's trees ([training.md](training.md)). Stanza and Trankit (UD, GPU) are the witnesses; a word where both agree on another head than the teacher is a suspect, and spaCy `trf` agreeing as well puts it at the top of the list. Agreement with a participating witness is not independent accuracy evidence; linguistic quality is measured on an admissible held-out gold set.
- **Convention, not error.** spaCy (ClearNLP) and SuPar (Stanford dependencies) make prepositions and copulas heads of their phrases. Low agreement with UD trees from them is a head-rule difference; use them as witnesses only after conversion. Constituency parsers (Stanza, benepar, SuPar CRF) serve bracket comparisons.
- **Language packages.** For each language, dev sets are parsed with gold syntactic words (multiword tokens already split, tokenizer off) and scored on all words, punctuation included: LAS on the full label and LAS-b on the label before the colon, each on the parser's own tags. A model trained on the same treebank is not a fair witness; a model from another treebank is fair but measures convention differences as much as quality.

Which parser serves which role:

| language | witness for in-treebank style | fair witness (other treebank) | notes |
|---|---|---|---|
| English | Stanza, Trankit | spaCy `trf` (after head-rule differences) | constituency: Stanza, benepar, SuPar |
| French (GSD) | UDPipe 2, Stanza `gsd`, Trankit | Stanza `sequoia` | spaCy labels in Sequoia style |
| Latin (LLCT) | Stanza `llct`, UDPipe 2, LatinCy | none good | LLCT is medieval with its own conventions; for other text compare across treebanks |
| Ancient Greek (PTNK) | Stanza `ptnk`, UDPipe 2 `grc_all` | Stanza/Trankit `proiel`, OdyCy | Perseus/PROIEL models show the register gap of the Greek Pentateuch |
| Biblical Hebrew (PTNK) | Stanza `hbo`, UDPipe 2 `hbo_ptnk` | none (the only treebank) | vowel points and cantillation fed as they are |
| Modern Hebrew | UDPipe 2 `he_all`; Stanza `htb` (HTB style), Stanza `combined` and DictaBERT `iahlt_ud` (IAHLT style) | across the two conventions | YAP only after a label mapping |

Not measured here: raw-text parsing with multiword-token recovery.

## Teacher and dispute interfaces

The annotation teacher is Codex CLI through [`mova-teacher`](../teacher/README.md) ([training](training.md)). `treeagree` also has a **dispute mode** (`dispute-ask`, `dispute-apply`): only the words whose head the parsers dispute are sent, as short multiple-choice questions. The teacher uses the full mode (a little world context, the sentence, and the SLM's tree and pragmatics); dispute mode stays available. Neither interface establishes annotation quality or authorizes a teacher run.
