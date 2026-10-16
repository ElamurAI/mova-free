# English parsers — local collection

On 2026-10-15 we gathered every available open parser for English, to compare them and to use the best as witnesses. This page covers code and models; the papers behind them are in the library.

Everything lives in `parsers/` in the data directory, outside the repo:

- `manifest.tsv`, `manifest2.tsv` — what was fetched: name, kind, licence, local path, URL.
- `repos.tsv`, `repos2.tsv` — source repositories, shallow clones in `src/<name>`.
- `fetch.log` — one line per item: status, size, path, file type (or commit and date for clones).

The fetch script is a plain loop over the manifests (`curl -C -`, `git clone --depth 1`); rerunning it resumes partial files and skips existing clones.

## Licences — build vs test

Per the 10.10 licence rule (build vs test): share-alike, non-commercial and research-only models are for **tests and measurements only**, never built into Mova or shipped.

| Parser | Kind of tree | Licence of the model | Use |
|---|---|---|---|
| UDPipe 1 (UD 2.5: EWT, GUM, LinES, ParTUT) | UD dependencies | CC BY-NC-SA 4.0 | tests only |
| UDPipe 2 (UD 2.15, all languages) | UD dependencies | CC BY-NC-SA 4.0 | tests only |
| UDify | UD dependencies (multilingual BERT) | see `udify/license.txt` (LINDAT, NC) | tests only |
| Stanza (EWT, GUM, combined; PTB constituency) | UD dependencies, PTB constituents | Apache-2.0 (models trained on UD/PTB data) | tests; build only after a data-licence check |
| Stanford CoreNLP 4.5.10 | Stanford/UD dependencies, PTB constituents | GPL-3.0 | tests only |
| spaCy `en_core_web_{sm,md,lg,trf}` 3.8.0 | ClearNLP-style dependencies | MIT (trained on OntoNotes) | tests; build only after a data-licence check |
| Berkeley Neural Parser `benepar_en3`, `_large`, `_wsj` | PTB constituents | MIT (trained on PTB) | tests |
| SuPar (biaffine, CRF2o, CRF constituency, DM semantic dependencies, UD XLM-R) | dependencies, constituents, SDP | MIT (trained on PTB/UD/SDP) | tests |
| Trankit (XLM-R base and large) | UD dependencies | Apache-2.0 | tests |
| MaltParser 1.9.2 + `engmalt` linear/poly | Stanford-basic dependencies | BSD (code), models: maltparser.org terms | tests |
| AllenNLP biaffine (PTB) and ELMo constituency | dependencies, constituents | Apache-2.0 | tests |
| BLLIP (Charniak–Johnson) WSJ, Gigaword, OntoNotes, SANCL, GENIA | PTB constituents | unclear (Dropbox, trained on PTB/LDC) | tests only |
| Link Grammar 5.12.5 | links (rule-based, hand-written dictionary) | LGPL-2.1 | candidate for build: transparent rules |
| ACE 0.9.34 + ERG 2018 | HPSG + MRS semantics (rule-based) | MIT-like | candidate for build: transparent rules |
| Grammatical Framework 3.12 + RGL + gf-wordnet | abstract syntax trees (rule-based) | LGPL/BSD | candidate for build: real ASTs |
| EasyCCG, depccg | CCG derivations | MIT (trained on CCGbank, LDC) | tests |
| syntaxdot (Rust) | dependencies | MIT/Apache-2.0 | code to read; no English model published |

Code only (no pretrained English model fetched): Enju (HPSG), TurboParser, Hexatagger (`parsing-as-tagging`), DiaParser.

## What runs here now (15.10, with Java 21, pip, flex and bison installed)

Smoke test sentence: "The little dog that Alice found ran home quickly." Correct tree: root `ran`, `dog` is its subject, `found` is a relative clause on `dog`, `that` is the object of `found`. Test scripts and the fetch scripts are in `$MOVA_DATA/parsers/smoke/`.

| Parser | Result | How |
|---|---|---|
| UDPipe 1 | correct | `udpipe1/udpipe-1.4.0-bin/bin-linux64/udpipe --tokenize --tag --parse udpipe1/english-ewt-ud-2.5-191206.udpipe` |
| ACE + ERG 2018 | correct; MRS `_find_v_1 ARG1 Alice ARG2 dog`, 0.04 s | `ace-erg/ace-0.9.34/ace -g ace-erg/erg-2018-x86-64-0.9.34.dat -1T` |
| Link Grammar 5.12.5 | correct (16 linkages, first one right) | built with `--with-regexlib=c` into `link-grammar/root`; `LD_LIBRARY_PATH=link-grammar/root/lib link-grammar/root/bin/link-parser en` |
| GF 3.12 + RGL | runs on the RGL demo lexicon only (`little` missing) | `LD_LIBRARY_PATH=gf/root/usr/lib GF_LIB_PATH=gf/rgl/gf-rgl-20260403 gf/root/usr/bin/gf --run alltenses/LangEng.gfo`; wide coverage needs `src/gf-wordnet` compiled |
| MaltParser 1.9.2 + engmalt linear | correct | `java -jar maltparser/maltparser-1.9.2/maltparser-1.9.2.jar -c engmalt.linear-1.7 -m parse`; input must be POS-tagged CoNLL |
| CoreNLP 4.5.10 (neural depparse) | **wrong**: root `dog`, `ran` attached as `dep` | `java -mx4g -cp "corenlp/stanford-corenlp-4.5.10/*" edu.stanford.nlp.pipeline.StanfordCoreNLP -annotators tokenize,ssplit,pos,lemma,parse,depparse -outputFormat conllu` |
| spaCy sm, trf | correct | `venv` (Python 3.13, torch 2.6 cu124, GPU visible) |
| Stanza depparse + constituency | correct | `venv`; resources in `stanza/resources` |
| SuPar biaffine dep, CRF constituency | correct (dep: `home` → `quickly` instead of `ran`) | `venv` with `TORCH_FORCE_NO_WEIGHTS_ONLY_LOAD=1` (old checkpoints) |
| Trankit (XLM-R base) | correct | `venv310` (Python 3.10 via uv in `pythons/`); needs `huggingface_hub<0.25`; cache `trankit/cache` |
| benepar `benepar_en3` | correct | `venv310` with `transformers<4.36`; `NLTK_DATA` = `nltk_data` |
| EasyCCG | jar runs, no model: it is in a Google Drive folder that curl cannot list | — |
| BLLIP | `bllipparser` does not build (old C++/Python 2 era); models are downloaded | — |
| UDPipe 2 | models downloaded (9 GB, not unpacked); the runner is a separate TensorFlow code base (`udpipe-2` branch), not set up | — |
| UDify, AllenNLP, depccg, DiaParser | not set up (archived stacks, old allennlp/chainer) | — |
| Enju, TurboParser | not built | — |
| syntaxdot | Rust binary downloaded; no English model published | — |

## School corpus run (15.10): open parsers vs Haiku trees

Input: the whole school silver (`corpus/school/school-silver.conllu` in the data directory, 44 011 sentences, 1.17 M tokens, Haiku-patched trees), pretokenized, so trees align token by token. Results stay on disk: `$MOVA_DATA/runs/parse-school-2026-10-15/` (`out/`, `report.md`, `suspects.tsv`, `per-course.tsv`; runners in `bin/`). Comparison: `tools/parsecmp`.

Kept (the slow and weak ones dropped; the best CPU parser and all GPU parsers kept):

| parser | where | time, whole school | UAS vs Haiku | UAS−p | LAS |
|---|---|---|---|---|---|
| Stanza (UD) | GPU | 10 min | 85.5 | 87.5 | 81.2 |
| Trankit (UD, XLM-R) | GPU | 1.5 min | 83.2 | 87.9 | 78.6 |
| spaCy trf (ClearNLP) | GPU, CuPy 14 dlpack patch in `bin/neural.py` | 3 min | 52.7 | 57.2 | — |
| spaCy lg (ClearNLP) | CPU | 1 min | 52.0 | 56.3 | — |
| SuPar biaffine, CRF2o (Stanford deps) | GPU | 1.2 min | 58.7 | 63.0 | — |
| Stanza-con, benepar, SuPar-CRF (constituency) | GPU | 16, 2.5, 1.3 min | pairwise bracket F1 85–87 | | |

Dropped: ERG (stopped after 22 k sentences — 86% parsed, but ~6 h for the corpus), CoreNLP PCFG (single-threaded, hours), CoreNLP neural (wrong on the smoke test), UDPipe 1, Malt, spaCy sm/md (weak). Link Grammar finished: a full linkage for 43.5% of sentences.

Findings:
- Haiku's trees agree with Stanza (85.5) about as much as Stanza agrees with Trankit (86.5): the silver is roughly as good as a strong parser, not better.
- Low numbers for spaCy and SuPar are head-rule differences (ClearNLP / Stanford: prepositions, copulas head their phrases), not errors.
- Suspects: Stanza and Trankit agree on another head than Haiku — 87.7 k tokens (7.5%), in 66% of sentences. spaCy trf picks the same head for 40 k of them (`spacy_trf=yes` in `suspects.tsv`): this is the list to fix first. Manual check of 25 confirmed non-punct suspects: about half clear Haiku errors, 1 Haiku right, the rest ambiguous attachments; without spaCy's confirmation Haiku is right in ~15%.
- Haiku's habits: `cc` on the left conjunct and chained `conj` (old UD 1 style; deterministic fix), punct attachment, the root in long sentences with `;` (root ↔ conj/parataxis/advcl), PP attachment (obl/nmod), off-by-one `amod`/`compound`.
- Weak relations of the silver (UD parsers reproduce under 60%): `conj`, `parataxis`, `appos`, `flat`, `discourse`, `dep`.
- Worst courses: lessons, language, french, programming, ukrainian-street, chinese-street (10–11% suspects): arithmetic lines, foreign words, word lists — the parsers are weak there too. Best: everyday, ai, biology (6–7%).
