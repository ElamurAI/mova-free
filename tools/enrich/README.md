# mova-enrich — grounded corpus enrichment and bounded self-study

`mova-enrich` turns a cycle round's agreed annotations (`pre/agreed.conllu`, `pre/prag-agreed.tsv`, `world.tsv`) into reviewed teaching material: a draft dictionary supplement, a books lesson, exercises, tests, exams and course-gap candidates. Mova prepares bounded proposals and course excerpts; the teacher (Codex CLI through [mova-teacher](../../teacher/README.md)) reviews them; Rust gates validate and stage the result. It also runs Mova's own bounded study/propose/review/learn/assess step (`improve`).

How it fits the training: [docs/training.md](../../docs/training.md#corpus-enrichment). The self-study step: [docs/school.md](../../docs/school.md#mova-driven-improvement).

## Build

```bash
cargo build --release -p mova-enrich
```

## Commands

Every output path must be new; nothing is overwritten.

| Command | What it does |
|---|---|
| `prepare ROUND DICTIONARY SCHOOL NEW_RUN MAX_SENTENCES` | build the source-bound packet and review prompt from a cycle round |
| `review RUN` | one teacher call on the prepared prompt |
| `stage RUN` | validate the review and stage the drafts |
| `run ROUND DICTIONARY SCHOOL NEW_RUN MAX_SENTENCES` | `prepare`, `review` and `stage` in one go |
| `check-source RUN ROUND DICTIONARY SCHOOL` | check that the dictionary, the school lessons and the agreed trees still match the packet's fingerprints |
| `status RUN` / `profile RUN` | print the run's state / prompt and packet sizes as JSON |
| `require-staged RUN` | exit non-zero unless the run is a current, validated staged result |
| `refine OLD_RUN NEW_RUN` | one targeted repair of a review whose only failure is an over-long answer span |
| `restage OLD_RUN NEW_RUN` | stage an older run again under the current validators |
| `simulate NEW_JSON` | write the artifact-state model check (completion needs all three artifact gates) and an idealized worker/wave grid; not a runtime forecast |
| `improve STAGED NEW_ROOT [BASE_MODEL]` | the bounded study → propose → review → learn → assess step |
| `study STAGED NEW_STUDY [BASE_MODEL]` | the study part of `improve` alone |
| `prepare-next STAGED STUDY NEW_REVIEW` | prepare the next review from a study |
| `finish-improve ROOT` | finish an interrupted `improve` from its saved parts |
| `study-exam STATE EXAM NEW_REPORT` | answer an exam file from a saved retained state (fresh process, no lesson replay) |

## Environment

| Variable | Default | Meaning |
|---|---|---|
| `ENRICH_MODEL` | `gpt-6.1-terra` (via `MOVA_TEACHER_MODEL` first) | teacher model |
| `ENRICH_PROMPT_MAX_BYTES` | 150000 | refuse a review prompt above this size |
| `ENRICH_CURRENT_DICTIONARY` (or `CYCLE_ENRICH_DICTIONARY`) | — | the current dictionary whose meanings must be preserved |

Effort is medium. The binary prints `wall_seconds` on stderr.

## Limits

- Lexical overlap retrieves related lessons; it does not prove a missing topic.
- Fiction does not become fact: narrative-driven additions and course gaps wait for independent evidence.
- A staged result is a draft for admission, not admitted knowledge; a whole pass is never marked seen after a partial failure.
- Different test scores or more frames alone are not evidence of improvement; compare the same questions before and after.
