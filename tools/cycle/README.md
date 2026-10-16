# cycle — the MMM teaching cycle

In each round, the MMM parses new school sentences, Haiku corrects the parse, the open parsers and `treeagree` check the correction, and the next MMM learns from the result. The teacher never sees a bare sentence: it always gets the MMM's parse with it, and the loop goes round.

```
cargo build --release
cycle run 2 10      # rounds 2..10, stops when the pool is used up
cycle round 4       # one round; steps already done are skipped (a broken round resumes)
cycle report        # cycle.tsv
```

## The GPU draft (15.10)

`CYCLE_GPARSE_MODEL=<gparse model dir>` makes the GPU parser re-parse the perceptron draft before the open parsers and treeagree. The tags, heads and relations come from `gparse`. LEMMA and FEATS stay where the tags did not change and are refilled from the agreed school where they did (`GP_LEX`).

Measured on round 6 (2 088 sentences). The referee is the 50 104 words where Stanza and Trankit give the same head and relation.

| draft | head | head + relation |
|---|---|---|
| `en` perceptron | 92.04 | 89.08 |
| `gparse` (agreed school) | 95.91 | 95.02 |
| treeagree over the perceptron draft (rounds 4–7) | 95.40 | 95.28 |
| **treeagree over the gparse draft** | **97.56** | **97.50** |
| Haiku's final trees | 96.24 | 95.87 |

Caveat: treeagree votes with Stanza and Trankit, so part of its lead is agreement with the referee itself.

## Fast learning (subsets, no full convergence)

- **The GPU MMM learns every round.** After each round, `gparse train` makes a warm start from the current model (`GP_INIT`). It runs 2 epochs over all of the round's trees (`GP_ALWAYS`) plus a fresh random 15% of the rest each epoch (`GP_SAMPLE`), at lr 5e-4. That takes about 30 s, versus 35 min from scratch. The result goes to `models/round<k>`, and the next round drafts with it. Measured on the held-out tests: school LAS 88.42 and EWT 85.62 after the fine-tune, against 88.48 and 85.75 for the 30-epoch model. Nothing is lost.
- **The perceptron is no longer retrained.** In the hybrid draft it only supplies tokens, lemmas and features, so `CYCLE_EN_MODEL` loads a saved `en` model (`ANNOT_EN_MODEL`). Drafting 2 200 sentences now takes 2 s instead of 3 min.

## Pools

- `new.tsv`: 4 344 primary-school sentences that are not in the 11.10 silver. Rounds 1–3 took 200 each; from round 6 (15.10) a round takes 5% of the corpus, 2 200 sentences.
- `pool-all.tsv`: all 47 103 primary-school sentences (no dictionary, no middle school). Use it once `new.tsv` is used up, with `CYCLE_LABEL=recheck`. The MMM has already trained on these sentences, so how much of its draft Haiku keeps is flattered; such rows are marked `pre/recheck`.

## Two flows

- **`CYCLE_FLOW=pre`** (the default from round 4, 15.10). The MMM drafts the sentences. Stanza, Trankit and spaCy trf parse the same sentences on the GPU. `treeagree` agrees the draft with them, learning retag FEATS and lemmas from the agreed school. Only this agreed tree goes to Haiku as the draft (`annot batch-prepare` with `ANNOT_DRAFTS`). Haiku's corrected trees then train the next MMM, and `treeagree check` only reports gate violations.
- **`CYCLE_FLOW=post`** (rounds 1–3). The MMM drafts go straight to Haiku, and `treeagree` agrees Haiku's trees afterwards. The steps below describe this flow.

## One round (post)

1. **Drafts.** `annot batch-prepare` retrains the MMM (`en`) on the base data (EWT, ESLSpok, the agreed school) plus everything the cycle has agreed so far. It then drafts N random pool sentences that are not yet in the cycle corpus.
2. **Haiku.** `annot batch-run` sends the drafts as patch requests through Message Batches (50% of the API price; 8 sentences per request; the fixed prompt is cached). `annot batch-apply` applies the patches, runs the UD gates and appends to `school-cycle.conllu`.
3. **Witnesses.** Stanza, Trankit and spaCy trf parse the round's trees on the GPU.
4. **Agreement.** `treeagree agree` produces the agreed trees and appends them to `school-cycle-agreed.conllu`, the corpus the next MMM trains on.
5. **Row in `cycle.tsv`:**
   - how much of the MMM draft Haiku kept (UAS/LAS). This is the number that should rise as the MMM learns;
   - how much `treeagree` still changed after Haiku;
   - the cost.

The defaults are for s2 (`$MOVA_DATA/runs/cycle-2026-10-15`, pool `new.tsv`: primary-school sentences that are not in the silver). Override them with `CYCLE_ROOT`, `CYCLE_POOL`, `CYCLE_N`, `CYCLE_SEED`, `CYCLE_BASE`, `CYCLE_CORPUS` and `CYCLE_AGREED`.

## Why the draft goes in: the anchoring check (round 1, 15.10)

`annot batch-blind` sends the same sentences again with no draft, so Haiku annotates them from scratch. The table covers 159 sentences that passed both runs. "Agrees with Stanza ∩ Trankit" means the share of words where Stanza and Trankit agree with each other and the tree takes the same head.

| | keeps the MMM draft (UAS / LAS) | agrees with Stanza ∩ Trankit | cost of 200 sentences | refused by the gates |
|---|---|---|---|---|
| Haiku from scratch | 73.7 / 67.5 | 86.7% | $0.180 | 34 |
| Haiku with the MMM draft | 96.1 / 93.7 | 92.4% | $0.082 | 12 |
| after treeagree | 94.0 / 90.4 | **95.6%** | — | — |

The draft does anchor Haiku. Without it, though, Haiku is worse by the open parsers' vote, refuses more gates and costs 2.2× more. So the draft stays, and `treeagree` after Haiku is mandatory.

The same comparison with the MMM of 11.10, trained on EWT only: Haiku kept its draft at UAS 88.3 / LAS 84.0 for $0.121. The current MMM, trained on the agreed school, is kept at 96.3 / 94.1 for $0.082.

## What it gives the MMM student (15.10)

`world knowledge course` runs the student (`PragStudent`, reading through frames over the `en` tree) through all 25 primary courses: 26 992 test questions, four options each. The test questions are never in the parsers' training data (only `explain:` and `fact:` lines are). Only the parser model (`WORLD_EN_MODEL`) changes. Runs are deterministic: repeated runs are byte-identical. Logs are in `$MOVA_DATA/runs/mmm-school-2026-10-15/`.

| parser the student reads with | tests right | questions understood | right among understood | formal-language exam |
|---|---|---|---|---|
| EWT + ESLSpok (the old one) | 39.8% | 56.3% | 50.5% | 37.3% |
| + agreed school (`en-agreed.bin`) | **43.3%** | **62.8%** | **53.5%** | **39.7%** |
| + parse database after the Haiku rounds (`en-db.bin`) | 43.4% | 62.9% | 53.6% | 39.7% |

That is +935 questions. The biggest gains are in chemistry (+10.2), physics (+8.9), myths (+7.7) and biology (+5.6 on 4 581 questions). Only english (−1.1) and euphemisms (−0.2) drop, where the questions are about words themselves. The Haiku rounds add nothing on top of the agreed school yet: they were mostly recheck rounds over sentences the parser already had.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
