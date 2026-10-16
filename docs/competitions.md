# Competitions — where to submit the model

Plan (26.09): for now we just submit the model and, where possible, take part in linguistic competitions.

Candidates were collected from our library: papers about them are already downloaded; folder names are given in parentheses.

| competition | what it measures | where we fit |
|---|---|---|
| **CRAC** — multilingual coreference on CorefUD | mention chains. 2025: "Can LLMs Dethrone Traditional Approaches?", a separate LLM track with prompts and agentic systems (`novak-2025-findings-fourth-shared-task-multilingual`). 2026: distant entities, 4 of 10 systems LLM-based, CorPipe won (`novak-2026-findings-fifth-shared-task-multilingual`) | context and pronouns, the story layer. SLM (small language model) + LLM is exactly the 2025 question |
| **UniDive** — morphosyntactic parsing | parsing without the notion of "word": LAS only over content nodes, node features, abstract nodes (`goldman-2025-findings-unidive-2025-shared-task`) | the metadialect "not even UD", converters |
| **PARSEME 2.0** — multiword expressions | MWE identification based on UD (`bogdanova-2026-pmi-mwe-scorer-parseme-2`) | multiword expressions in seeds and domain snapshots |
| **DISRPT** — discourse | discourse units and relations (`ju-2025-dedisco-disrpt-2025-shared-task`) | the story layer, pragmatics |
| **SemEval** — new tasks every year | for example, intended sarcasm (`farha-2022-semeval-2022-task-6-isarcasmeval`), event abduction (`cao-2026-semeval-2026-task-12-abductive`) | pragmatics of "what was meant" |
| **BabyLM** — a small model on limited data | including Gricean pragmatics (`askari-2025-are-babylms-deaf-gricean-maxims`) | the SLM learner, the five-year-old test |

## What in the rules concerns us

- **Tracks.** The closed track accepts only the competition data; the open or LLM track accepts any resources. SLM + LLM goes to the open or LLM track; SLM alone goes to the closed one.
- **Measurement.** We already measure parsing with the CoNLL 2018 script (LAS, MLAS, BLEX), so the numbers are comparable.
- **Competition data is for research.** It sometimes has NC parts, as GUM does. So the competition build is separate and for research, like our GUM measurement. It does not go into the production model: `commercial_gate` will stop it.
- **A system paper** is usually mandatory. We write it from the usage log (`docs/citations.md`).
- **Calendar and deadlines** are checked when the time comes: "where possible".
