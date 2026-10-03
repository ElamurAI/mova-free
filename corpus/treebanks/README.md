# Treebanks

Sentences annotated in [Universal Dependencies](https://universaldependencies.org/)
(CoNLL-U format: words, lemmas, tags, morphological features, dependency tree).

| File | Sentences | Texts | Annotation | License |
|---|---|---|---|---|
| `mova-silver-en.conllu` | 348 | Tatoeba sentences under CC0 (`# source` says which), example sentences from Mova seeds, and 6 sentences from ACL Anthology papers under CC BY 4.0 | parsed by Mova, checked and corrected by a large language model, with UD validation gates | annotation ours, CC BY 4.0 (because of the ACL sentences; attribution in `# source`) |
| `mova-bronze-en.conllu` | 326 | example sentences from Mova seeds | parsed by Mova only, not checked | ours, Apache-2.0 OR MIT |
| `mova-tales-silver-en.conllu` (new in 0.3) | 2,087 | sentences of 41 fairy-tale books from Project Gutenberg (public domain in the USA; book id in `# sent_id`) | parsed by Mova, patched by a large language model (Claude Opus, medium effort), UD 2.18 validation gates | annotation ours, Apache-2.0 OR MIT |
| `eslspok-mova/` | 2,320 (train 1,856, dev 232, test 232) | [UD English ESLSpok](https://github.com/UniversalDependencies/UD_English-ESLSpok) | the original with Mova's dialect rules applied: 9 rules changed 523 of 21,312 words | CC BY-SA 4.0, as the original |

**How Mova uses it:** `ai/en` trains and tests its tagger and parser on UD
treebanks; the silver and bronze sets test that the grammar seeds really parse
the way they describe. The training-data gate (`en::license::commercial_gate`)
only accepts sources whose license allows any purpose.

**The tales split** (how Mova's tales numbers are measured): `world conllu-split mova-tales-silver-en.conllu
even.conllu odd.conllu` — the odd half (1,043 sentences) is the test set, the even half (1,044) is for development;
Mova's self-training splits the even half further and never reads the odd one. LAS on the odd half: 76.85 in 0.2,
77.38 in 0.3.
