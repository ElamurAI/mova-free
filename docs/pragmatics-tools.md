# English pragmatics tools — local collection

15.10.2026: "while it learns, download all the open software for English pragmatics". Everything lives in `$MOVA_DATA/pragmatics-tools/` (13 GB): `src/` (62 git clones, depth 1), `hf/` (9 HuggingFace models), `manifest.tsv` (kind, repo, detected licence, size), `check.tsv`, `search.tsv` (GitHub search hits by stars).

**Licences.** Per the build-vs-test rule, GPL, NC and repositories without a licence file (all rights reserved) are for tests and measurements only. MIT, Apache and BSD are candidates for building, after a check of the data they were trained on. What the tools mostly give us is the *task definitions* and the *datasets they point to*; Mova's own pragmatics stays rule-based and transparent.

## RST discourse parsing

| repo | licence | use | size |
|---|---|---|---|
| [DMRST_Parser](https://github.com/seq-to-mind/DMRST_Parser) | none (no licence file) | tests only (no licence = all rights reserved) | 6.2M |
| [isanlp_rst](https://github.com/tchewik/isanlp_rst) | MIT License | candidate (permissive) | 3.3M |
| [DPLP](https://github.com/jiyfeng/DPLP) | MIT License | candidate (permissive) | 9.1M |
| [StageDP](https://github.com/yizhongw/StageDP) | MIT License | candidate (permissive) | 33M |
| [rstfinder](https://github.com/EducationalTestingService/rstfinder) | MIT License | candidate (permissive) | 12M |
| [feng-hirst-rst-parser](https://github.com/arne-cl/feng-hirst-rst-parser) | see LICENSE.txt | check the licence | 236M |
| [rstWeb](https://github.com/amir-zeldes/rstWeb) | MIT License | candidate (permissive) | 6.3M |
| [DisCoDisCo](https://github.com/gucorpling/DisCoDisCo) | none (no licence file) | tests only (no licence = all rights reserved) | 1.1M |
| [GumDrop](https://github.com/gucorpling/GumDrop) | Apache License | candidate (permissive) | 123M |
| [sharedtask2023](https://github.com/disrpt/sharedtask2023) | none (no licence file) | tests only (no licence = all rights reserved) | 571M |
| [sharedtask2021](https://github.com/disrpt/sharedtask2021) | none (no licence file) | tests only (no licence = all rights reserved) | 419M |
| [discourse-graphs](https://github.com/arne-cl/discoursegraphs) | see LICENSE | check the licence | 20M |
| [rst-converter](https://github.com/amir-zeldes/rst2dep) | Apache License | candidate (permissive) | 416K |

## PDTB / shallow discourse parsing, discourse relations

| repo | licence | use | size |
|---|---|---|---|
| [discopy](https://github.com/rknaebel/discopy) | MIT License | candidate (permissive) | 600K |
| [pdtb-parser](https://github.com/linziheng/pdtb-parser) | GNU GENERAL PUBLIC LICENSE | tests only (GPL) | 121M |
| [conll16st](https://github.com/attapol/conll16st) | none (no licence file) | tests only (no licence = all rights reserved) | 736K |
| [DisExtract](https://github.com/windweller/DisExtract) | MIT License | candidate (permissive) | 876K |
| [discourse-markers](https://github.com/synapse-developpement/Discovery) | Apache License | candidate (permissive) | 1.3M |
| [DiscourseSenser](https://github.com/WladimirSidorenko/DiscourseSenser) | MIT License | candidate (permissive) | 9.3M |
| [shallow-discourse-parser-2016](https://github.com/acoli-repo/shallow-discourse-parser-2016) | none (no licence file) | tests only (no licence = all rights reserved) | 149M |
| [TagNN-PDTB](https://github.com/yizhongw/TagNN-PDTB) | MIT License | candidate (permissive) | 356K |
| [BMGF-RoBERTa](https://github.com/HKUST-KnowComp/BMGF-RoBERTa) | MIT License | candidate (permissive) | 1.1M |
| [implicit-discourse-paragraph](https://github.com/ZeyuDai/paragraph-level_implicit_discourse_relation_classification) | none (no licence file) | tests only (no licence = all rights reserved) | 101M |

## Discourse markers

| repo | licence | use | size |
|---|---|---|---|
| [DiscSense](https://github.com/sileod/DiscSense) | none (no licence file) | tests only (no licence = all rights reserved) | 576K |
| [TSLM-DISCOURSE-MARKERS](https://github.com/IBM/TSLM-DISCOURSE-MARKERS) | Apache License | candidate (permissive) | 1.3G |

## Coherence

| repo | licence | use | size |
|---|---|---|---|
| [coherence-rst](https://github.com/grig-guz/coherence-rst) | MIT License | candidate (permissive) | 388K |
| [GCDC-corpus](https://github.com/aylai/GCDC-corpus) | none (no licence file) | tests only (no licence = all rights reserved) | 200K |
| [centering-neural-hds](https://github.com/sdeva14/emnlp20-centering-neural-hds) | none (no licence file) | tests only (no licence = all rights reserved) | 2.5M |
| [DiscoScore](https://github.com/AIPHES/DiscoScore) | none (no licence file) | tests only (no licence = all rights reserved) | 7.1M |

## Dialogue acts

| repo | licence | use | size |
|---|---|---|---|
| [DialogTag](https://github.com/bhavitvyamalik/DialogTag) | MIT License | candidate (permissive) | 248K |
| [MIDAS_dialog_act](https://github.com/DianDYu/MIDAS_dialog_act) | Apache License | candidate (permissive) | 1.6M |
| [swda](https://github.com/cgpotts/swda) | GNU GENERAL PUBLIC LICENSE | tests only (GPL) | 28M |
| [DialogueActClassification](https://github.com/NathanDuran/Switchboard-Corpus) | GNU GENERAL PUBLIC LICENSE | tests only (GPL) | 63M |
| [SPARTA_WSDM2022](https://github.com/LCS2-IIITD/SPARTA_WSDM2022) | none (no licence file) | tests only (no licence = all rights reserved) | 448K |
| [DialogueAct-Tagger](https://github.com/ColingPaper2018/DialogueAct-Tagger) | none (no licence file) | tests only (no licence = all rights reserved) | 230M |
| [CASA-Dialogue-Act-Classifier](https://github.com/macabdul9/CASA-Dialogue-Act-Classifier) | MIT License | candidate (permissive) | 5.9M |
| [Probabilistic-RNN-DA-Classifier](https://github.com/NathanDuran/Probabilistic-RNN-DA-Classifier) | GNU GENERAL PUBLIC LICENSE | tests only (GPL) | 259M |
| [CDAC](https://github.com/emory-irlab/CDAC) | none (no licence file) | tests only (no licence = all rights reserved) | 6.7M |

## Speech acts

| repo | licence | use | size |
|---|---|---|---|
| [SGNN](https://github.com/glicerico/SGNN) | Apache License | candidate (permissive) | 24M |

## Politeness

| repo | licence | use | size |
|---|---|---|---|
| [ConvoKit](https://github.com/CornellNLP/ConvoKit) | see LICENSE.md | check the licence | 27M |
| [politeness](https://github.com/sudhof/politeness) | Apache License | candidate (permissive) | 5.0M |
| [politeness-classifier](https://github.com/vinaykola/politeness-classifier) | none (no licence file) | tests only (no licence = all rights reserved) | 332M |

## Irony and sarcasm

| repo | licence | use | size |
|---|---|---|---|
| [tweeteval](https://github.com/cardiffnlp/tweeteval) | none (no licence file) | tests only (no licence = all rights reserved) | 27M |
| [sarcasm-detection](https://github.com/MirunaPislar/Sarcasm-Detection) | MIT License | candidate (permissive) | 165M |
| [SarcasmDetection](https://github.com/AniSkywalker/SarcasmDetection) | GNU LESSER GENERAL | tests; linking only | 12M |

## Metaphor

| repo | licence | use | size |
|---|---|---|---|
| [metaphor-tsvetkov](https://github.com/ytsvetko/metaphor) | Open Database License | check the licence | 308M |
| [metaphor-in-context](https://github.com/gao-g/metaphor-in-context) | none (no licence file) | tests only (no licence = all rights reserved) | 432K |
| [MelBERT](https://github.com/jin530/MelBERT) | none (no licence file) | tests only (no licence = all rights reserved) | 2.2M |
| [MetaphorFrame](https://github.com/liyucheng09/MetaphorFrame) | none (no licence file) | tests only (no licence = all rights reserved) | 39M |

## Implicature

| repo | licence | use | size |
|---|---|---|---|
| [ImpPres](https://github.com/facebookresearch/ImpPres) | Attribution-NonCommercial | tests only (NC) | 12M |
| [pragmatic-implicature-llm](https://github.com/LauraRuis/do-pigs-fly) | none (no licence file) | tests only (no licence = all rights reserved) | 229M |
| [rsa-cgpotts](https://github.com/cgpotts/pypragmods) | GNU GENERAL PUBLIC LICENSE | tests only (GPL) | 12M |
| [pragmeval](https://github.com/sileod/pragmeval) | none (no licence file) | tests only (no licence = all rights reserved) | 21M |
| [ludwig](https://github.com/ucl-dark/ludwig) | MIT License | candidate (permissive) | 292K |

## Presupposition

| repo | licence | use | size |
|---|---|---|---|
| [nope](https://github.com/nyu-mll/nope) | none (no licence file) | tests only (no licence = all rights reserved) | 468M |
| [CREPE](https://github.com/velocityCavalry/CREPE) | BSD | candidate (permissive) | 412K |

## Rational Speech Acts

| repo | licence | use | size |
|---|---|---|---|
| [webppl-rsa](https://github.com/wo/webppl-rsa) | none (no licence file) | tests only (no licence = all rights reserved) | 276K |

## Certainty

| repo | licence | use | size |
|---|---|---|---|
| [certainty-estimator](https://github.com/jiaxin-pei/certainty-estimator) | MIT License | candidate (permissive) | 336K |

## Hedging

| repo | licence | use | size |
|---|---|---|---|
| [cross_domain_hedge_detection](https://github.com/melozdemir/cross_domain_hedge_detection) | none (no licence file) | tests only (no licence = all rights reserved) | 24M |
| [BERTUncertaintyDetection](https://github.com/PeterZhizhin/BERTUncertaintyDetection) | none (no licence file) | tests only (no licence = all rights reserved) | 696K |

## Stance

| repo | licence | use | size |
|---|---|---|---|
| [stance-conditional](https://github.com/sheffieldnlp/stance-conditional) | none (no licence file) | tests only (no licence = all rights reserved) | 340K |
| [mdl-stance-robustness](https://github.com/UKPLab/mdl-stance-robustness) | MIT License | candidate (permissive) | 2.1M |

## Pretrained models (HuggingFace, `hf/`)

Smoke test 15.10 on the GPU, two sentences each (`transformers` pipeline in `$MOVA_DATA/parsers/venv`):

| model | task | licence | smoke test |
|---|---|---|---|
| `cardiffnlp/twitter-roberta-base-irony` | irony (TweetEval) | not stated on the card | "Great, another Monday. I just love being stuck in traffic." → irony 0.99; but "The train leaves at nine." → irony 0.72: over-eager outside tweets |
| `helinivan/english-sarcasm-detector` | sarcasm (news headlines) | not stated | missed "Oh wonderful, the printer broke again." (not sarcastic, 0.87) |
| `Intel/polite-guard` | politeness, 4 classes | Apache-2.0 | "Could you please pass me the salt?" → polite 0.91; "Give me the salt now." → impolite 0.99 ✓ |
| `pedropei/sentence-level-certainty`, `aspect-level-certainty` | certainty (Pei & Jurgens 2021) | not stated | a regression head: the classification pipeline output is meaningless; needs its own runner (`src/certainty-certainty-estimator`) |
| `sileod/roberta-base-discourse-marker-prediction` | discourse marker between two sentences (Discovery) | Apache-2.0 | "It was raining." / "we stayed home." → "mostly," 0.28 (expected "so"): weak |
| `diwank/silicone-deberta-pair` | dialogue acts (SILICONE) | MIT | not tested yet |
| `sileod/deberta-v3-base-tasksource-nli`, `cross-encoder/nli-deberta-v3-base` | NLI, for implicature/presupposition probes (ImpPres, NOPE) | Apache-2.0 | not tested yet |

## Not found

`probmods/problang` (the RSA book code), `hedgehog-nlp/hedgehog`, `vinodkpg/speech-act-classifier`, `tasnimmohiuddin/coherence-models`: no such repositories (checked with `git ls-remote`).

## Next

Nothing is wired into Mova. Candidates to measure against our pragmatics (`prag`), on the school and the pragmatics bench: polite-guard (requests, imperatives), discourse relations (discopy, DMRST — PDTB/RST over our trees), dialogue acts (SILICONE) for the speech-act frame, ImpPres/NOPE as implicature and presupposition tests.
