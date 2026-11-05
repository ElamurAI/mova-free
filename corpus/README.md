# Mova's own corpus: silver UD for English

Mova's corpus holds random English sentences, annotated in a loop:
1. a large model annotates the sentence;
2. the Rust annotator `en` annotates the same tokens;
3. the large model compares the two annotations and gives the final one;
4. Rust gates check the result.

## Where it lives

The corpus is kept outside the repo, in the data directory. This file only describes it.

- **The open release** `corpus/open/`: the downloaded corpus 0.4 (books, treebanks, dictionary, math scripts, contrast sets), unpacked by `train/get-corpus.sh 0.4` (the 0.5 corpus: `mova fetch 0.5 corpus`); only its READMEs are in git ([open/README.md](open/README.md)).

- The DuckDB annotation database `$MOVA_DATA/db/mova.duckdb` holds every layer and source over the same sentences: treebank gold, silver, bronze, the large-model and Rust runs, and the tales. [`tools/db/README.md`](../tools/db/README.md) describes the schema and gives example queries; the tool is `tools/db`.
- The CoNLL-U files `corpus/mova-silver-en.conllu` and `corpus/mova-bronze-en.conllu`: `annot silver` appends to them, `db load` loads them into the database, and `db export` writes them back byte for byte.
- **The school**: `corpus/school/school-silver.conllu` holds the teacher's trees, 44 011 sentences (built 2026-10-15). Next to it, `school-agreed.conllu` is the agreed layer: the teacher plus Stanza, Trankit and spaCy-trf, combined by the written rules of [`tools/treeagree`](../tools/treeagree), whose README describes them.

## Silver

`mova-silver-en.conllu` holds the sentences annotated by the `annot silver` loop above. The Rust gates are:
- closed lists of UPOS, PTB tags, FEATS and relations;
- the UD 2.18 registry for English (`en/data/ud-registry-en.tsv`);
- one root, no cycles, forms equal to the tokens.

**Format: CoNLL-U, UD 2.18, EWT conventions.** The columns are ID, FORM, LEMMA, UPOS, XPOS (PTB), FEATS, HEAD and DEPREL with subtypes; DEPS and MISC are «_». Every sentence has:
- `# source`: the source and licence of the text;
- `# annotation`: who annotated it, how, and the run folder;
- `# text` and `# sent_id`.

**Size: 348 silver sentences** (as in corpus 0.4).
- 20 sentences from Tatoeba and from papers, annotated in two passes.
- 328 examples from the seeds, raised from bronze in edit mode: 171 sentences of 5–30 tokens and 157 shorter ones. Long, complex sentences are kept along with the short ones; nothing is filtered by length.

`annot/README.md` describes edit mode and its calibration.

## Bronze

`mova-bronze-en.conllu` holds sentences annotated by the Rust annotator `en` alone, without the large model (`annot bronze`). The gates are the same as for silver.

Bronze is a queue: a large-model pass can raise its sentences to silver, and they also test the rules of the expert system ([`docs/english.md`](../docs/english.md)). It holds 326 sentences from the examples in the seeds `en/seeds/` (as in corpus 0.4).

## Sources of the sentences

| source | licence | raw material |
| --- | --- | --- |
| `tatoeba:<n>` | Tatoeba, CC0 1.0 | `annot/src/tatoeba-eng-cc0.tsv` in the data directory |
| `papers:<folder>:<n>` | open papers of the library, CC BY 4.0 (ACL Anthology) | `annot sentences papers …` |
| `seeds:<seed>:<hash>` | examples from the seeds `en/seeds/`: our own sentences, or short quotations from grammars (the seed's «Sources» section names them) | `annot seed-examples en/seeds …` |

The author and title of each paper are in the library's card for it.

**Licence.** The annotation is ours; the texts are CC0 or CC BY, with attribution in `# source`. As shipped in the open corpus 0.4, the corpus is under CC BY-SA 4.0 as a whole.

## Checks

- **A third opinion.** UDPipe 2 (the EWT model) is used only for comparison in the run reports. UDPipe models are under a non-commercial licence, so their output never goes into the corpus or into training.
- **Quality.** Calibration on EWT sentences that have a gold standard: `annot calib`, with reports in the data directory under `annot/runs/`. Only independent gold counts as a reference, never our own final annotation.

## Licence

The open corpus 0.4 is under CC BY-SA 4.0 as a whole ([open/README.md](open/README.md)); its silver texts are CC0 or CC BY, and the annotation is ours. The trained models (`en.bin`, the GPU parser) are under CC BY-SA 4.0; Mova's code is under the Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Details per file: [licence/DATA.md](../licence/DATA.md).
