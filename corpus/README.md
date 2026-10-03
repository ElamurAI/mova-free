# Mova corpus

Data Mova is developed and tested on. Everything here is open for any purpose;
each folder says where the data comes from, what we changed, its license, and
how Mova uses it. Benchmarks with restrictive licenses (non-commercial,
no-derivatives) are not included: the code only measures against them.

**Download:** run `train/get-corpus.sh` (or get [corpus-0.3.tar.gz](https://research.elamur.ai/download/0/0.3/corpus-0.3.tar.gz) from [research.elamur.ai](https://research.elamur.ai/corpus/) and unpack it with `tar xzf corpus-0.3.tar.gz --strip-components=1 -C corpus`). The data is not stored in git; this folder holds the descriptions.

| Folder | What | License |
|---|---|---|
| [tales](tales/) | 42 books of fairy tales and fables, Project Gutenberg texts | public domain in the USA |
| [tale-summaries](tale-summaries/) | one-line summaries of 51,070 paragraphs of those books | ours: Apache-2.0 OR MIT |
| [tale-questions](tale-questions/) | 12,251 short questions with answers copied from the text | ours: Apache-2.0 OR MIT |
| [treebanks](treebanks/) | sentences annotated in Universal Dependencies: our silver and bronze sets, and a corrected ESLSpok | per file, see its README |
| [math-scripts](math-scripts/) | 2,680 world scripts for word problems from SVAMP and GSM8K | problems MIT (original sets); scripts ours |
| [contrast](contrast/) | contrast sets: bAbI stories with new names, places, things and verbs; StepGame phrasings the generator never produced | bAbI part CC BY 3.0 (as bAbI); phrasings ours: Apache-2.0 OR MIT |

New in 0.3: the tales treebank (`treebanks/mova-tales-silver-en.conllu`, 2,087 sentences, the test set of Mova's
tales parsing numbers) and `contrast/`. The 0.2 archive stays available:
[corpus-0.2.tar.gz](https://research.elamur.ai/download/0/0.2/corpus-0.2.tar.gz) (`train/get-corpus.sh 0.2`).
