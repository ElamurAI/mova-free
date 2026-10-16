# Data and model licences

Mova's **own** code, data and models are under the **Elamur Cosmic License** ([../LICENSE.md](../LICENSE.md)). It allows everything, except Big Brother.

**Third-party** data and anything derived from it keep the terms their sources require (section 7 of the license). A build gate (`en::license::commercial_gate`) refuses training sources whose license is unknown or restricts the purpose of use.

- **Never shipped:** data under non-commercial (NC), no-derivatives (ND) or research-only terms. Benchmarks under such terms are only *measured against*; their files are not in this repository.
- **CC BY-SA, as little as possible:** we quote it, test and measure on it, and learn from it. We do not ship it or modified copies of it. Files whose header declares CC BY-SA stay out of the free release (`tools/freesync` holds them). A model trained on CC BY-SA treebanks stays under CC BY-SA.

| path | source | license |
|---|---|---|
| `en/data/forms.tsv`, `en/data/lemmas.tsv` | the AGID inflection database (Kevin Atkinson), plus word and tag frequencies from UD English EWT | AGID permissive license; the EWT-derived counts are CC BY-SA 4.0 |
| `$MOVA_DATA/en/models/en.bin` (not in git; `install.sh` downloads it) | tokenizer, tagger, parser and lemmatizer trained by Mova on UD English EWT and ESLSpok plus the agreed school trees | CC BY-SA 4.0, as EWT |
| `gparse` models (in the data directory, not in git) | Mova's GPU parser, same training data | CC BY-SA 4.0, as EWT |
| `en/data/train-licenses.tsv`, `en/data/ud-registry-en.tsv` | the registry of training sources and their licenses | ours, Elamur Cosmic License |
| `en/seeds/**`, `global/seeds/**` (except below), `prag/data/cues.tsv`, `math/data/**`, `latex/data/**`, `mlab/data/**` | written for Mova | ours, Elamur Cosmic License |
| `global/seeds/shortcuts/atomic-*.md` | derived from ATOMIC-2020 (Hwang et al. 2021) | CC BY 4.0 |
| `global/seeds/wikidata/core.md` | derived from Wikidata | CC0 1.0 |
| `world/data/school/**` | the school's courses and lessons, written for Mova by a large model as teacher | ours, Elamur Cosmic License |
| `world/data/**` (fables) | questions on Aesop's fables (Project Gutenberg #21) | text public domain; questions ours, Elamur Cosmic License |
| `world/data/rules-induced.tsv` | tree-repair rules induced by Mova (`world induce`) | ours, Elamur Cosmic License |
| `global/data/absurdity-roles.tsv`, `global/data/absurdity-roles-tale.tsv` | the absurdity matrix: plausibility scores rated by a large model for Mova | ours, Elamur Cosmic License |
| `global/data/register-*.tsv` | register lexicons: log-odds counts over public-domain Project Gutenberg books | ours, Elamur Cosmic License (texts public domain) |
| `tera/global/data/expressions.tsv`, `tera/global/data/idioms.tsv` (closed part, not in the free release) | expressions and idioms with meanings, from the English Wiktionary and other open sources (STREUSLE, ConceptNet, MAGPIE, PARSEME 1.3, WordNet) | CC BY-SA 4.0 — **internal, not in the free release** |
| `mlab/tests/data/ext/nist/**` | NIST StRD reference datasets | public domain (U.S. Government work) |
| `mlab/tests/data/ext/boost/**` | Boost.Math test data | Boost Software License 1.0 (`LICENSE_1_0.txt`) |
| `*/tests/data/vmm/**` | prompts and model outputs produced for Mova | ours, Elamur Cosmic License |
| `corpus/open/**` (downloaded by `train/get-corpus.sh`) | see each folder's README | per folder |
| `papers/*.md` | our summaries of papers and books; the papers keep their own licenses (named on each page) and are not copied here | the summaries are ours, Elamur Cosmic License |
