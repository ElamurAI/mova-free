# Mova's own corpus: silver UD for English

Mova's corpus holds random English sentences, annotated in a loop:
1. a large model annotates the sentence;
2. the Rust annotator `en` annotates the same tokens;
3. the large model compares the two annotations and gives the final one;
4. Rust gates check the result.

## Where it lives

The corpus is kept outside the repo, in the data directory. This file only describes it; older versions of the corpus are in the git history.

- The DuckDB annotation database `db/mova.duckdb` holds every layer and source over the same sentences: treebank gold, silver, bronze, the large-model and Rust runs, and the tales. [`db/README.md`](../db/README.md) describes the schema and gives example queries; the tool is `tools/db`.
- The CoNLL-U files `corpus/mova-silver-en.conllu` and `corpus/mova-bronze-en.conllu`: `annot silver` appends to them, `db load` loads them into the database, and `db export` writes them back byte for byte.
- **The school** (2026-10-15): `corpus/school/school-silver.conllu` holds the teacher's trees, 44 011 sentences. Next to it, `school-agreed.conllu` is the agreed layer: the teacher plus Stanza, Trankit and spaCy-trf, combined by the written rules of [`tools/treeagree`](../tools/treeagree), whose README has the description and measurements.

## Silver

`mova-silver-en.conllu` holds the sentences annotated by the `annot silver` loop above. The Rust gates are:
- closed lists of UPOS, PTB tags, FEATS and relations;
- the UD 2.18 registry for English (`en/data/ud-registry-en.tsv`);
- one root, no cycles, forms equal to the tokens.

**Format: CoNLL-U, UD 2.18, EWT conventions.** The columns are ID, FORM, LEMMA, UPOS, XPOS (PTB), FEATS, HEAD and DEPREL with subtypes; DEPS and MISC are «_». Every sentence has:
- `# source`: the source and licence of the text;
- `# annotation`: who annotated it, at what effort, and the run folder;
- `# text` and `# sent_id`.

**State on 2026-09-26: 348 silver sentences.**
- 20 sentences from Tatoeba and from papers, annotated in two passes at maximum effort.
- 328 examples from the seeds, raised from bronze in edit mode at high effort. First came the examples of 5–30 tokens: 171 sentences, 49.5 k output tokens. Then the short ones: 157 sentences, 36 k. Long, complex sentences are kept along with the short ones; nothing is filtered by length. The first pass skipped 20 of them, and the second pass took all.

`annot/README.md` explains, with its calibration, why edit mode was chosen.

## Bronze

`mova-bronze-en.conllu` holds sentences annotated by the Rust annotator `en` alone, without the large model (`annot bronze`). The gates are the same as for silver.

Bronze is a queue: a large-model pass can raise its sentences to silver, and they also test the rules of the expert system ([`docs/expert.md`](../docs/expert.md)). On 2026-09-26 it held 326 sentences from the examples in the seeds `en/seeds/`; the gate stopped two more.

## Sources of the sentences

| source | licence | raw material |
| --- | --- | --- |
| `tatoeba:<n>` | Tatoeba, CC0 1.0 | `annot/src/tatoeba-eng-cc0.tsv` in the data directory |
| `papers:<folder>:<n>` | open papers of the library, CC BY 4.0 (ACL Anthology) | `annot sentences papers …` |
| `seeds:<seed>:<hash>` | examples from the seeds `en/seeds/`: our own sentences, or short quotations from grammars (the seed's «Sources» section names them) | `annot seed-examples en/seeds …` |

The author and title of each paper are in the library's card for it.

**The corpus licence is CC BY 4.0.** The annotation is ours; the texts are CC0 or CC BY, with attribution in `# source`.

## Checks

- **A third opinion.** UDPipe 2 (the EWT model) is used only for comparison in the run reports. UDPipe models are under a non-commercial licence, so their output never goes into the corpus or into training.
- **Quality.** Calibration on EWT sentences that have a gold standard: `annot calib`, with reports in the data directory under `annot/runs/`. Only independent gold counts as a reference, never our own final annotation.

## Licence

The corpus itself is CC BY 4.0, as stated above: the texts are CC0 or CC BY, and the annotation is ours. Mova's code and models are under the Elamur Cosmic License ([LICENSE.md](../LICENSE.md)).
