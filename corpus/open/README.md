# Mova corpus 0.4

**Licence: CC BY-SA 4.0** (https://creativecommons.org/licenses/by-sa/4.0/), for the whole corpus, as for Mova's
trained models. The code of Mova is under the Elamur Cosmic License (see the repository).

| Folder | What is inside |
|---|---|
| [books](books/) | the full texts of the books of Mova's school books course: *Alice's Adventures in Wonderland*, *Just So Stories* (Project Gutenberg) |
| [treebanks](treebanks/) | Universal Dependencies trees: the agreed school (42,759 sentences), Mova silver (348) and bronze (326) |
| [dictionary](dictionary/) | Mova's explanatory dictionary for pupils: 16,868 words with their senses, uses in the lessons and examples |
| [math-scripts](math-scripts/) | 2,680 world scripts for word problems from SVAMP and GSM8K |
| [contrast](contrast/) | contrast sets: bAbI with new names, places, things and verbs; StepGame phrasings the generator never produced |

Each folder's README says where its data comes from and credits the original sources.

**Download:** `train/get-corpus.sh` (or [corpus-0.4.tar.gz](https://research.elamur.ai/download/0/0.4/corpus-0.4.tar.gz),
unpacked with `tar xzf corpus-0.4.tar.gz --strip-components=1 -C corpus/open`). The data is not stored in git; this
folder holds the descriptions.

The previous release has its own page: [0.3.md](0.3.md).
