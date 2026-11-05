# cycle — the SLM teaching cycle

In each round, the SLM parses new school sentences, the teacher corrects the parse, the open parsers and `treeagree` check the correction, and the next SLM learns from the result. The teacher never sees a bare sentence: it always gets the SLM's parse with it, and the loop goes round.

```
cargo build --release -p cycle
cycle run 2 10      # rounds 2..10, stops when the pool is used up
cycle round 4       # one round; steps already done are skipped (a broken round resumes)
cycle report        # cycle.tsv
```

Other commands:

| command | what it does |
|---|---|
| `cycle pipeline <from> <to>` | the asynchronous cycle: our hardware prepares rounds while the teacher works; finished rounds go into the parse database and fine-tune the GPU parser |
| `cycle enrich <k>` | one enrichment round of the pipeline |
| `cycle trainer` | the GPU parser learns on its own: whenever the parse database changed, a short fine-tune from the newest model |
| `cycle db-init <seed.conllu>` | a fresh parse database: the seed corpus, then every finished round |
| `cycle db-export <out>` | the database's training export (`train-db.conllu` content) into `<out>`, minus every development and held-out sentence by id and normalized text (`db::Heldout`; sources and dropped count logged; `CYCLE_HELDOUT` adds sources) |
| `cycle heldout-filter <out-dir> <file.conllu>...` | copies of any training files without development and held-out sentences (the same `db::Heldout`), as `<out-dir>/<i>/<file name>` (names kept for `en`'s licence gate); kept and dropped counts per file. The annotator's `CYCLE_BASE` (and the cycle's agreed corpus) reach `annot drafts` and `annot batch-prepare` only through such copies (`<round>/base-inputs/`) |
| `cycle db-agree` | agree every sentence's versions by a weighted vote per word; `CYCLE_VOTE_MISC=1` keeps each word's vote in MISC (`Vote`, `VoteHead`, `VoteAlt`) for training on confident arcs ([gparse](../../gparse/README.md#confident-arcs)); off, `current.conllu` is unchanged |
| `cycle db-same-text <file>` | give sentences the tree of the same text stored under another id |
| `cycle promote-reviewed <a> <b> <c>`, `cycle rollback-reviewed <tx>` | promote reviewed annotation into the parse database as one transaction, and roll it back ([training.md](../../docs/training.md)) |
| `cycle changes <draft.conllu> <final.conllu> [top]` | what the teacher changed in a draft: moved heads and relations, UPOS changes |

## The teacher

The teacher is Codex CLI through the shared [`mova-teacher`](../../teacher) crate, authenticated with ChatGPT (default model `gpt-6.1-terra`, medium effort; `CYCLE_MODEL` or `MOVA_TEACHER_MODEL` set another `gpt-` model). `round`, `run`, `pipeline` and `enrich` run only with `CYCLE_TEACHER=codex` (the default) and `CYCLE_TIERS=0`; any other teacher or tier setting stops with an error. There is no fallback to the Claude API. Start every new run in a fresh run directory. See [docs/training.md](../../docs/training.md), "Current teacher transport".

## The GPU draft

`CYCLE_GPARSE_MODEL=<gparse model dir>` makes the GPU parser re-parse the perceptron draft before the open parsers and treeagree. The tags, heads and relations come from `gparse`. LEMMA and FEATS stay where the tags did not change and are refilled from the agreed school where they did (`GP_LEX`).

When `CYCLE_GPARSE_MODEL` is set, `treeagree` agrees the gparse draft rather than the perceptron draft. `treeagree` votes with Stanza and Trankit, so measuring drafts against those two parsers partly measures agreement with the referee itself.

## Fast learning (subsets, no full convergence)

- **The GPU SLM learns every round.** After each round, `gparse train` makes a warm start from the current model (`GP_INIT`). It runs 2 epochs over all of the round's trees (`GP_ALWAYS`) plus a fresh random 15% of the rest each epoch (`GP_SAMPLE`), at lr 5e-4, instead of training from scratch. The result goes to `models/round<k>`, and the next round drafts with it.
- **The perceptron is not retrained.** In the hybrid draft it only supplies tokens, lemmas and features, so `CYCLE_EN_MODEL` loads a saved `en` model (`ANNOT_EN_MODEL`) instead of retraining it for each draft.

## Pools

- `new.tsv`: primary-school sentences that are not yet in the silver corpus. A round takes `CYCLE_N` sentences (default 200).
- `pool-all.tsv`: all primary-school sentences (no dictionary, no middle school). Use it once `new.tsv` is used up, with `CYCLE_LABEL=recheck`. The SLM has already trained on these sentences, so how much of its draft the teacher keeps is flattered; such rows are marked `pre/recheck`.

## Two flows

- **`CYCLE_FLOW=pre`** (the default). The SLM drafts the sentences. Stanza, Trankit and spaCy trf parse the same sentences on the GPU. `treeagree` agrees the draft with them, learning retag FEATS and lemmas from the agreed school. Only this agreed tree goes to the teacher as the draft (`annot batch-prepare` with `ANNOT_DRAFTS`). The teacher's corrected trees then train the next SLM, and `treeagree check` only reports gate violations.
- **`CYCLE_FLOW=post`**. The SLM drafts go straight to the teacher, and `treeagree` agrees the teacher's trees afterwards. The steps below describe this flow.

## One round (post)

1. **Drafts.** `annot batch-prepare` retrains the SLM (`en`) on the base data (EWT, ESLSpok, the agreed school) plus everything the cycle has agreed so far. It then drafts N random pool sentences that are not yet in the cycle corpus.
2. **The teacher.** `annot batch-run` sends the drafts as patch requests to the local Codex batch worker (8 sentences per request, `ANNOT_BATCH=8`). `annot batch-apply` applies the patches, runs the UD gates and appends to `school-cycle.conllu`.
3. **Witnesses.** Stanza, Trankit and spaCy trf parse the round's trees on the GPU.
4. **Agreement.** `treeagree agree` produces the agreed trees and appends them to `school-cycle-agreed.conllu`, the corpus the next SLM trains on.
5. **Row in `cycle.tsv`:**
   - how much of the SLM draft the teacher kept (UAS/LAS). This is the number that should rise as the SLM learns;
   - how much `treeagree` still changed after the teacher;
   - a cost column, kept for reading older runs; it is zero or empty for Codex rounds, whose subscription cost is not reported.

The defaults: run root `$MOVA_BIG/runs/cycle-codex-2026-10-28`, pool `<root>/new.tsv`, `CYCLE_N=200`, `CYCLE_SEED=1015`. For a new run set `CYCLE_ROOT`, `CYCLE_POOL` and `CYCLE_CORPUS` explicitly (a fresh root has no `new.tsv`); `CYCLE_BASE`, `CYCLE_AGREED`, `CYCLE_FLOW`, `CYCLE_GPARSE_MODEL`, `CYCLE_EN_MODEL` and `CYCLE_LABEL` are the other settings.

## Why the draft goes in: the anchoring check

`annot batch-blind` sends the same sentences again with no draft, so the teacher annotates them from scratch. Comparing the two runs (how much of the SLM draft the teacher keeps, agreement with words where Stanza and Trankit agree, gate refusals) checks how much the draft anchors the teacher. The draft stays in the request; because it anchors the teacher to the SLM's parse, `treeagree` after the teacher is mandatory.

## What it gives the SLM student

`world knowledge course` runs the student (`PragStudent`, reading through frames over the `en` tree) through the primary courses with only the parser model (`WORLD_EN_MODEL`) changed, so a parser from a cycle can be compared with the base parser on the school. The test questions are never in the parsers' training data (only `explain:` and `fact:` lines are). School results are in [docs/school.md](../../docs/school.md#results).

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
