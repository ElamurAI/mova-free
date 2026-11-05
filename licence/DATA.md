# Data and model licences

**Code and data have separate licences.** The **Elamur Cosmic License** ([../LICENSE.md](../LICENSE.md)) covers
**only Mova's code**. Mova's data — the in-code dictionary, the school's lessons and dictionaries, the corpora, the
trained models and tables — is distributed under **CC BY-SA 4.0**, or under its sources' terms where they are
stricter, with every source cited. Each data module of the table below sits in its own folder with a `LICENSE.md`
that names its licence and cites its sources, starting with a short machine-readable block (`module`, `licence` as an
SPDX id, `requires`, `allows`, `sources`). Smaller data files elsewhere in the repository have no folder `LICENSE.md`;
the path table further below gives their licence and sources.

| data module | folder | licence |
|---|---|---|
| the in-code English dictionary (`lemmas.tsv`, `forms.tsv`) and its extension | [en/data/](../en/data/LICENSE.md) | CC BY-SA 4.0 (UD English EWT, AGID, OEWN, SPECIALIST, Census, Names Corpus cited) |
| the school: courses and lessons, the school and commands dictionaries | not in git: the release archive `school-0.5.tar.gz` (`mova fetch 0.5 school`, into `$MOVA_DATA/school`), with `LICENSE.txt` and a `LICENSE.md` in each folder | CC BY-SA 4.0 |
| the pupils' dictionary and the other world data | [world/data/](../world/data/LICENSE.md) | CC BY-SA 4.0 |
| the absurdity matrix, register lexicons, ATOMIC and Wikidata queries | [global/data/](../global/data/LICENSE.md) | CC BY-SA 4.0 (ATOMIC-2020 CC BY 4.0, Wikidata CC0 cited) |
| VerbNet and WordNet tables, pragmatic cues | [prag/data/](../prag/data/LICENSE.md) | CC BY-SA 4.0 (VerbNet and WordNet notices kept) |
| the world model's core and check sets | [onto/data/](../onto/data/LICENSE.md) | CC BY-SA 4.0 |
| the Yom HaDin jurisdictions, cases and verdicts | [yom-hadin/](../yom-hadin/LICENSE.md) | CC BY-SA 4.0 (the JPS 1917 Torah public domain; Guggenheimer's Jerusalem Talmud CC BY, via Sefaria) |
| trained models (`en.bin`, `gparse`) and the coreference table, the corpus archive | not in git; each release archive has its `LICENSE.txt` and `README.md` | CC BY-SA 4.0 |
| add-on modules (the Wiktionary dictionary, for instance) | `$MOVA_DATA/modules/<name>/`, not in git | each module's own licence (`module.json`) |

Documentation (`docs/**`, crate READMEs, and `docs/mysticism/`, Mova's own working philosophy) comes with the code under the Elamur Cosmic License, and so do the other prose folders written for Mova in our own words: the grammar manuals of `languages/**` (each cites its sources in `sources.md`), the dialect notes of `dialects/**`, the treebank notes `treebanks/*/*/*.md`, and the READMEs of `train/` and `corpus/` (the corpus files themselves keep the licences of their folders, `corpus/open/*/README.md`).

The shipped `dictionary.tsv` (the archive `dictionaries-0.5.tar.gz`) is `world/data/dictionary-improved.tsv` without the examples that quote a restricted book; its licence is the same.

The other data files of the repository (the test and check suites of `math/data`, `mlab/data`, `yom-hadin/data`,
`tools/db/data`, the hand transcriptions of `tools/ocrfix/data`, the tables in the table below) are data too: CC BY-SA
4.0, with their sources in the table. Seeds (`*/seeds/**`) and the tables a `build.rs` turns into code
(`latex/data/commands.txt`, `math/data/lexicon.tsv`) are rule sources of the code and stay with it.

**Direction (not implemented yet):** from 0.6 Mova is to have two licence modes, «Cosmic» (code and the data we own
outright) and «Cosmic + other licences» (also loading the separately licensed modules), and to show the mode's
licences, with what they require and allow, on start; the `LICENSE.md` blocks above are written for that loader.

**Third-party** data and anything derived from it keep the terms their sources require (section 7 of the license). A build gate (`en::license::commercial_gate`) refuses training sources whose license is unknown or restricts the purpose of use.

- **Open** (public domain, CC0, CC BY, CC BY-SA, permissive code): may be built into data modules, models and releases, with attribution. Sources that are not legally open (shadow libraries, borrowed books) are never used.
- **Never shipped:** data under non-commercial (NC), no-derivatives (ND) or research-only terms. Benchmarks under such terms are only *measured against*; their files are not in this repository.
- **Add-on modules:** data under other licences (a Wiktionary dictionary, for instance) is distributed apart from Mova as add-on modules, installed into the data directory and loaded at run time ([docs/modules.md](../docs/modules.md)). The core and its built-in dictionaries never contain module data.
- **CC BY-SA:** a data module that carries CC BY-SA material is itself CC BY-SA 4.0 and cites its sources in its `LICENSE.md`; it never makes the code CC BY-SA. A model trained on CC BY-SA treebanks stays under CC BY-SA.
- **The in-code dictionary** (`en/data`) carries forms, lemmas and counts from UD English EWT (CC BY-SA 4.0) and is compiled into every binary, so it is distributed under CC BY-SA 4.0 with its citations in [en/data/LICENSE.md](../en/data/LICENSE.md); `en licence` prints them, and the text is compiled into the `en` library (`en::dict::LICENCE`) outside the dictionary's blob, so the fingerprint `en.bin` is bound to does not move (a unit test pins it). The EWT e-mail and web genres put personal data into the dictionary as tokens: e-mail addresses, links, `@handles` and telephone numbers. `en lexicon-redact` replaced those 415 lemmas by `<redacted-N>` placeholders (lemma numbers kept) and dropped their 321 form rows (2026-11-04); `en lexicon` skips them when it rebuilds, and a unit test fails if one comes back.

| path | source | license |
|---|---|---|
| `en/data/forms.tsv`, `en/data/lemmas.tsv` | the AGID inflection database (Kevin Atkinson): `forms.tsv` rows with source `a`, `ar`, `r`, `b`; plus the forms, lemmas, tags and counts seen in UD English EWT and ESL Spoken: rows with source `u`, the count column, and the lemma lines that come only from UD | CC BY-SA 4.0 as a whole, AGID's notice kept; citations in [en/data/LICENSE.md](../en/data/LICENSE.md) |
| `en/data/lexicon-ext.tsv`, `en/data/acronyms.tsv` | the lexicon extension compiled into `en` (made by `en lexgrow`, `en/src/lexgrow.rs`): rows `o` from Open English WordNet 2025 (McCrae et al., the JSON edition, not `-plus`); rows `s:<EUI>` from the SPECIALIST Lexicon, 2026 release (U.S. National Library of Medicine, Lexical Systems Group), selected from `LRAGR` and `LEXICON` (single-word nouns, verbs, adjectives and adverbs mapped to PTB tags; acronyms and their expansions) — the modification is that selection and mapping; rows `c` from the 2010 U.S. Census surnames; rows `k` from the Names Corpus 1.3 (Mark Kantrowitz, additions by Bill Ross), whose README the file's header carries as its terms ask. No Wiktionary or Wikipedia data | OEWN rows CC BY 4.0, crediting Princeton WordNet and the OEWN team; SPECIALIST rows under the SPECIALIST NLP Tools Terms and Conditions (a U.S. Government product, free to use and redistribute with attribution of the release; no endorsement by HHS, NIH or NLM is implied); Census rows public domain (U.S. Government work); Names Corpus rows free for any purpose with credit, redistributable with its README; the files CC BY-SA 4.0 with the dictionary ([en/data/LICENSE.md](../en/data/LICENSE.md)) |
| `$MOVA_DATA/en/models/en.bin` (not in git; `install.sh` downloads it) | tokenizer, tagger, parser and lemmatizer trained by Mova on UD English EWT and ESLSpok plus the agreed school trees | CC BY-SA 4.0, as EWT |
| `gparse` models (in the data directory, not in git) | Mova's GPU parser, same training data | CC BY-SA 4.0, as EWT |
| `en/data/train-licenses.tsv`, `en/data/ud-registry-en.tsv` | the registry of training sources and their licenses | ours, CC BY-SA 4.0 |
| `en/seeds/**`, `global/seeds/**` (except below), `latex/data/commands.txt`, `math/data/lexicon.tsv` | rule sources compiled into code, written for Mova | ours, Elamur Cosmic License, with the code |
| `prag/data/cues.tsv`, `math/data/suite-v1.tsv`, `math/data/holdout-v1.tsv`, `mlab/data/**`, `tools/db/data/**` | check suites, cases and tables written for Mova | ours, CC BY-SA 4.0 |
| `yom-hadin/data/**`, `yom-hadin/jurisdictions/**` | Yom HaDin's cases, gold verdicts and jurisdictions: rules read from the JPS 1917 Torah and from the Jerusalem Talmud in Guggenheimer's translation (Berlin, De Gruyter, 1999–2015, via the Sefaria export, which states CC BY; `jurisdictions/yerushalmi/rules.tsv` quotes it in its `text` column), paraphrases after King's Hammurabi (1910), Ukrainian official texts and U.S. federal law ([LICENSE.md](../yom-hadin/LICENSE.md)) | ours, CC BY-SA 4.0; the Talmud translation CC BY with attribution; the other sources public domain or outside copyright |
| `global/seeds/shortcuts/atomic-*.md` | derived from ATOMIC-2020 (Hwang et al. 2021) | CC BY 4.0 |
| `global/seeds/wikidata/core.md` | derived from Wikidata | CC0 1.0 |
| `$MOVA_DATA/school/courses/**` (not in git; the archive `school-0.5.tar.gz`) | the school's courses and lessons, written for Mova by a large model as teacher (the archive's `courses/LICENSE.md`) | ours, CC BY-SA 4.0 |
| `$MOVA_DATA/school/dictionaries/primary-school.v2.tsv` (not in git; the archive `school-0.5.tar.gz`), `mystic-school.tsv`, `judaism.tsv`, `emode.tsv` and their `*-senses.tsv` | the school dictionaries: part of speech, meanings for a pupil and senses written for Mova by a large model as teacher (Claude Haiku and Sonnet), the mystic senses by hand from `docs/mysticism`; examples quoted from the school's lessons and from public-domain texts (the Torah and Writings in the JPS 1917 translation, Sefer Yetzirah in the Kalisch 1877 and Westcott 1887 translations and Sefaria's community translation, Mordell 1914, fairy-tale books of Project Gutenberg). The Early Modern English records use Wiktionary only as a fact (whether a sense is marked archaic), not its text. The tools that build these files are internal and not in this repository | ours, CC BY-SA 4.0; the quoted texts public domain, Sefaria's community translation CC0 |
| `$MOVA_DATA/school/dictionaries/commands.tsv` (not in git; the archives `school-0.5.tar.gz` and `dictionaries-0.5.tar.gz`) | the commands dictionary: Nautilus's native draft from the command course and its corpus (counts, examples), meanings from the pupils' dictionary; loaded as the layer `builtin:commands` when installed (`world dict`) | ours, CC BY-SA 4.0 |
| `world/data/dictionary-improved.tsv`, `world/data/dictionary-sources/**` | the pupils' dictionary records, written for Mova by a large model as teacher; examples from the lessons and from public-domain tales (Project Gutenberg) | ours, CC BY-SA 4.0; the quoted texts public domain |
| `$MOVA_DATA/nautilus/layers/wikipedia*.tsv` and their `*.examples.jsonl` (not in git, never shipped; `tools/nautilus` refuses to write a CC BY-SA layer inside a Git work tree) | Wikipedia dictionary layers: drafts made natively by 🐚 Nautilus from English Wikipedia articles, with examples quoted with their article and sentence id; meanings from the pupils' dictionary. Mova loads them only for internal runs and measurements (`scope: internal`); the runtime store refuses them for a shipped use | CC BY-SA 4.0, as Wikipedia — **internal, never in the repository, a release or a binary** |
| `$MOVA_DATA/modules/<name>/<version>/**` and `MOVA_MODULES_PATH` folders (not in git, never shipped; installed by `mova modules`, [docs/modules.md](../docs/modules.md)) | add-on modules, such as dictionary layers, each with a `module.json` naming its licence, licence class and attribution; loaded only when selected (`MOVA_MODULES`, or `module:` layers of a `MOVA_DICT` manifest); a non-commercial / research-only module only for internal runs | each module's own licence. What Mova derives from a CC BY-SA or non-commercial module carries that licence in its header and is refused inside a Git work tree, so it never enters this repository, a release or a binary |
| `$MOVA_DATA/modules/wiktionary-en/<version>/**` (not in git, never shipped; made by `tools/wikt-extract` from the raw dump of `$MOVA_DATA/raw/wiktionary/enwiktionary/<run>/`) | the add-on module `wiktionary-en`: the English entries of the English Wiktionary as a dictionary layer (senses, labels, examples, synonyms, antonyms, forms, form-of links, each row with its page, revision and dump offset); distributed separately from Mova under its own licence, never part of the core, its dictionaries or a binary; loaded only when selected (`MOVA_MODULES`, or a `module:`/internal layer of a `MOVA_DICT` manifest) | CC BY-SA 4.0 and GFDL 1.3 or later, Wiktionary contributors; its layer's header declares it, so `shippable` refuses it |
| `world/data/questions.tsv` | questions on Aesop's fables (Project Gutenberg #21) | text public domain; questions ours, CC BY-SA 4.0 |
| `world/data/cmds-*.txt`, `world/data/v3-merchant.tsv` | world command scripts written for Mova over public-domain tales (Aesop's fables, a Japanese tale, Lang's *Arabian Nights*, Project Gutenberg #128), and that book's sentences | texts public domain; scripts ours, CC BY-SA 4.0 |
| `world/data/kinds.txt`, `world/data/know-qa*.tsv`, `world/data/skeleton-ops.tsv`, `world/data/stepgame-contrast*.tsv`, `world/data/brain-base.md`, `world/data/self.md` | body plans, independent check questions, operation words, StepGame contrast phrasings and the base self-description, written for Mova | ours, CC BY-SA 4.0 |
| `world/data/rules-induced.tsv` | tree-repair rules induced by Mova (`world induce`) | ours, CC BY-SA 4.0 |
| `global/data/absurdity-roles.tsv`, `global/data/absurdity-roles-tale.tsv` | the absurdity matrix: plausibility scores rated by a large model for Mova | ours, CC BY-SA 4.0 |
| `$MOVA_DATA/coref/gender.tsv` (not in git; a release ships it as a separate file) | the coreference noun lexicon: counts of noun–pronoun dependency paths over Mova's own parses (`en.bin`) of the public-domain (EU/UA) Project Gutenberg tales of `corpora/tales/free` and of the school's lessons (`coref lexicon`, `coref/README.md`) | sources public domain and ours, but the trees come from `en.bin` (CC BY-SA 4.0): never in the repository; **shipped in a release as a separate file under CC BY-SA 4.0**, apart from the code; `coref lexicon` rebuilds it |
| `$MOVA_DATA/coref/gender-wikipedia.tsv` (not in git, never shipped) | the same counts over the Wikipedia school's trees, for an internal comparison only; its header declares CC BY-SA and the loader refuses it unless `COREF_GENDER_INTERNAL=1` | CC BY-SA 4.0 — **internal** |
| `global/data/register-*.tsv` | register lexicons: log-odds counts over public-domain Project Gutenberg books | ours, CC BY-SA 4.0 (texts public domain) |
| `$MOVA_DATA/hard-lane/**` (not in git, never shipped; made by `tools/hard-cases`) | the hard-case lane: sentences Mova read badly, with their documents' context, teacher annotations and witness parses; each case keeps its source's licence, and the licence decides its learning track: CC BY, CC0 and public domain may train the shipped model; CC BY-SA trains only a separate internal model, never shipped; NC, ND, research-only and unknown sources are only measured | each case's source licence; CC BY-SA — **internal, not in the free release** |
| Optional expression and idiom tables supplied outside the repository (`MOVA_PETA_DATA`, compiled in by `global/build.rs` when present; see the README's «Paths») | expressions and idioms with meanings from third-party lexical sources (the English Wiktionary, STREUSLE, ConceptNet, MAGPIE, PARSEME 1.3, WordNet) | CC BY-SA 4.0 and per-source terms — **not in the public source or the free release** |
| `mlab/tests/data/ext/nist/**` | NIST StRD reference datasets | public domain (U.S. Government work) |
| `mlab/tests/data/ext/boost/**` | Boost.Math test data | Boost Software License 1.0 (`LICENSE_1_0.txt`) |
| `*/tests/data/vmm/**` | prompts and model outputs produced for Mova | ours, CC BY-SA 4.0 |
| `tools/nautilus/tests/data/course/**` | a test lesson of the commands course, written for Mova by a large model as teacher | ours, CC BY-SA 4.0 |
| `docs/assets/mova-octopus.webp`, `docs/assets/mova-octopus.png` | Mova's octopus mascot artwork, generated for Mova with OpenAI image generation ([docs/assets/README.md](../docs/assets/README.md)) | ours, CC BY-SA 4.0 |
| `corpus/open/**` (downloaded by `train/get-corpus.sh`) | see each folder's README | per folder |
| `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/**` (not in git; made by `tools/macula`) | the Torah's dependency trees, converted from MACULA Hebrew Linguistic Datasets, available at https://github.com/Clear-Bible/macula-hebrew/ (Biblica, Clear Bible, the Groves Center, OpenScriptures); the Semantic Dictionary of Biblical Hebrew fields are not carried | CC BY 4.0 |
| `$MOVA_DATA/corpora/judaism/text/torah-jps1917/**` (not in git) | the Torah in the JPS 1917 translation, from eBible.org | public domain |
| `tools/ocrfix/data/gt/**`, `tools/ocrfix/data/confusions.tsv` | hand transcriptions of pages of public-domain printed editions (Cowley 1923, Schechter-Taylor 1899, Driver 1913, Ungnad 1911, Kautzsch 1884) and the confusable letters of the square script, made for Mova | the printed texts public domain; the transcriptions ours, CC BY-SA 4.0 |
| `$MOVA_DATA/corpora/ocr/**` (not in git; made by `tools/ocrfix`) | OCR text of public-domain editions (Cowley 1923, Schechter-Taylor 1899) | public domain texts; the OCR ours |
| `papers/README.md`, `papers/*.tsv` | our list of the papers and books Mova uses, with links to their cards; the works keep their own licenses and are not copied here | ours, CC BY-SA 4.0 |
| `kg/examples/gold-v1.tsv`, `kg/examples/heldout-v2.tsv` | Ukrainian sentences from Tatoeba (by sentence id) with our hand verdicts on the graphs built from them | sentences CC BY 2.0 FR, Tatoeba contributors (each sentence's author is on its Tatoeba page); verdicts ours, CC BY-SA 4.0. Not derived from VESUM |
| `kg/examples/prosti-rechennia.jsonl` | a hand-written example graph of five simple sentences | ours, CC BY-SA 4.0. Not derived from VESUM |
| `formal/*.table` | tables extracted from successful Lean runs (`mova-modelcheck extract-table`) | ours, CC BY-SA 4.0 |
| `treebanks/cards.tsv` | facts about the UD treebanks (language, licence, genres, sizes, authors, papers) collected from their UD pages and READMEs; no treebank text | ours, CC BY-SA 4.0; the facts belong to the treebanks' pages |
| `docs/eval/stepgame-excluded.tsv` | StepGame test question ids excluded from the clean test, with our reasons; no StepGame text | ours, CC BY-SA 4.0 |
| `docs/usage.tsv` | our log of where papers and books are applied | ours, CC BY-SA 4.0 |
| `tools/langseg/audit/*.tsv` | segment ids with our hand labels; no corpus text | ours, CC BY-SA 4.0 |
| `tools/sources/rules/*.tsv` | title patterns for collecting papers by topic | ours, CC BY-SA 4.0 |
| `models/mova.groupant/transfer-*.tsv` | our timing samples of the GPU transfer measurements | ours, CC BY-SA 4.0 |
| `onto/seeds/**` | seeds of the world model: hand-written defaults with OEWN 2025 glosses as sources (CC BY 4.0), Wikidata candidates (CC0 1.0), and second opinions and risk estimates written by a large model (Claude Opus) as teacher | ours, CC BY-SA 4.0 (rule sources, with the code); OEWN CC BY 4.0 and Wikidata CC0 cited |
| `$MOVA_DATA/school/dictionaries/primary-school.tsv` (not in git) | the earlier edition of the school dictionary, kept in the data store for the site; **not in a release**, which carries `primary-school.v2.tsv` | ours, CC BY-SA 4.0 |

## Made with third-party models

These parts were made with the help of Anthropic's Claude models: drafts patched by the model as teacher, texts and ratings written with it. Anthropic's terms bind Anthropic's customers, here the Elamur team: they restrict using model outputs to develop competing models (license, section 8). We distribute these parts under CC BY-SA 4.0 (data, not code) and add no restriction of our own; we state the provenance so that each recipient can judge whether such terms concern their use. This is a statement of provenance, not legal advice.

- `corpus/open/**` treebanks marked silver or agreed (the teacher's patches), and `en.bin`/`gparse` models trained on them;
- the school's courses and lessons (`school-0.5.tar.gz`);
- the school dictionaries (`school-0.5.tar.gz`), `world/data/dictionary-improved.tsv`, `world/data/dictionary-sources/**`, the dictionaries' meanings and senses (except the hand-written mystic senses);
- `global/data/absurdity-roles*.tsv`, the absurdity matrix;
- `*/tests/data/vmm/**`, prompts and model outputs.

## Measured against or used, not shipped

Their files are not in this repository; each keeps its own terms. The cards are on [Elamur Research](https://research.elamur.ai/free/).

| work | what for | license |
|---|---|---|
| bAbI (Weston et al. 2015, [card](https://research.elamur.ai/free/papers/weston-2015-towards-ai-complete-question-answering/)) | measured against | code BSD; data CC BY 3.0 (as Hugging Face states it) |
| SVAMP (Patel et al., NAACL 2021, [card](https://research.elamur.ai/free/papers/patel-2021-are-nlp-models-really-able/)) | measured against (the train part in `cv`, the test once per release) | MIT |
| GSM8K (Cobbe et al. 2021, [card](https://research.elamur.ai/free/papers/cobbe-2021-training-verifiers-solve-math-word/)) | measured against | MIT |
| ProofNet (Azerbayev et al. 2023, [card](https://research.elamur.ai/free/papers/azerbayev-2023-proofnet-autoformalizing-formally-proving-undergraduate/)), ProofNet# (Poiroux et al. 2025, [card](https://research.elamur.ai/free/papers/poiroux-2025-reliable-evaluation-benchmarks-statement-autoformalization/)) | measured against | MIT |
| TalkBank CHILDES database (MacWhinney 2000, [card](https://research.elamur.ai/free/papers/macwhinney-2000-childes-project-tools-analyzing-talk/)) | tests and research only, never training; the UD English-CHILDES treebank is a separate UD release under CC BY-SA 4.0 | TalkBank: CC BY-NC-SA 3.0, and no training of large language models |
| VerbNet (Kipper Schuler 2005, [card](https://research.elamur.ai/free/papers/kipper-2005-verbnet-broad-coverage-comprehensive-verb/)) | the frames reader's verb classes | VerbNet 3.0 license (permissive, as WordNet's), © University of Colorado |
| Open English WordNet (McCrae et al. 2019, [card](https://research.elamur.ai/free/papers/mccrae-2019-english-wordnet/); 2020, [card](https://research.elamur.ai/free/papers/mccrae-2020-english-wordnet-2020-improving-extending/)) | the world model's concepts: the 2025 edition's JSON, read at run time from `$MOVA_DATA/raw/en-oewn/json` and fetched there by `install.sh` from the OEWN GitHub release (SHA-256 checked); not in this repository | CC BY 4.0, crediting Princeton WordNet and the OEWN team |
| Tatoeba ([card](https://research.elamur.ai/free/papers/tatoeba-2006-collection-sentences-translations/)) | sentences | CC BY 2.0 FR, crediting each sentence's author; some sentences CC0 |
| Stanza (Qi et al. 2020, [card](https://research.elamur.ai/free/papers/qi-2020-stanza-python-natural-language-processing/)), Trankit (Nguyen et al. 2021, [card](https://research.elamur.ai/free/papers/nguyen-2021-trankit-light-weight-transformer-based/)) | witness parsers | Apache-2.0 (code); their models keep their treebanks' licenses |
| spaCy (Honnibal et al. 2020, [card](https://research.elamur.ai/free/papers/honnibal-2020-spacy-industrial-strength-natural-language/), doi 10.5281/zenodo.1212303) | witness parser | MIT |
| UDPipe 2 (Straka 2018, [card](https://research.elamur.ai/free/papers/straka-2018-udpipe-2-0-prototype-conll/)) | parser compared against | code MPL-2.0; models CC BY-NC-SA, so measured against only |
