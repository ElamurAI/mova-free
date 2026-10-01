# Data and model licenses

The **code** in this repository is licensed under either of Apache-2.0 or MIT,
at your option (see `LICENSE-APACHE`, `LICENSE-MIT`).

**Data and trained models** keep the license their sources require. A build
gate (`en::license::commercial_gate`) refuses training sources whose license is
unknown or restricts the purpose of use.

- **Never included:** data under non-commercial (NC) or no-derivatives (ND)
  terms. Benchmarks with such licenses are only *measured against* by the code;
  their files are not part of this repository.
- **CC BY-SA sources** allow any purpose, including commercial use. Files and
  models derived from them are distributed under CC BY-SA as well, with
  attribution.

| Path | Source | License |
|---|---|---|
| `ai/en/data/forms.tsv`, `ai/en/data/lemmas.tsv` | AGID inflection database (Kevin Atkinson) + word/tag frequencies from Universal Dependencies English EWT | AGID permissive license; EWT-derived counts CC BY-SA 4.0 |
| `ai/en/models/ud-ewt-eslspok.bin` | tagger, parser and lemmatizer trained by Mova on UD English EWT and ESLSpok | CC BY-SA 4.0 (as the training data) |
| `ai/en/data/train-licenses.tsv`, `ai/en/data/ud-registry-en.tsv` | registry of training sources and their licenses | ours, Apache-2.0 OR MIT |
| `ai/en/seeds/**`, `ai/global/seeds/**` (except below), `ai/prag/data/cues.tsv`, `ai/math/data/**`, `ai/latex/data/**`, `ai/mlab/data/**` | written for Mova | ours, Apache-2.0 OR MIT |
| `ai/global/seeds/shortcuts/atomic-*.md` | derived from ATOMIC-2020 (Hwang et al. 2021) | CC BY 4.0 |
| `ai/global/seeds/wikidata/core.md` | derived from Wikidata | CC0 1.0 |
| `ai/world/data/**` | questions on Aesop's fables (Project Gutenberg #21) | text public domain; questions ours |
| `ai/mlab/tests/data/ext/nist/**` | NIST StRD reference datasets | public domain (U.S. Government work) |
| `ai/mlab/tests/data/ext/boost/**` | Boost.Math test data | Boost Software License 1.0 (`LICENSE_1_0.txt`) |
| `ai/*/tests/data/vmm/**`, `ai/coder/prompts/**` | prompts and model outputs produced for Mova | ours, Apache-2.0 OR MIT |
