# Trees by logical derivation

The SLM builds dependency trees step by step, each arc from a rule with a reason, as it solves arithmetic.

- `world derive <train.conllu>... --test …` — decision lists of attachment rules induced from trees: "a DET
  attaches to the nearest NOUN to its right as det (precision 97%)"; the root by the root rule; a rule that would
  close a cycle is skipped. From EWT gold: EWT test UAS 56.65. An easy-first variant (most certain steps first,
  attached words hidden in their phrase) was worse (53.5) and is kept for comparison.
- `world derive-learn <book-list> --test …` — the teacher is the SLM parser itself: random book sentences parsed
  by the working pipeline; the deriver learns transformation rules (Brill-style) that take its decision-list tree
  towards the parser's tree, no gold. 18 000 trees, 80 rules (2026-10-02):

| measure | decision lists | + learned transformations |
|---|---|---|
| agreement with the parser, held-out book sentences | UAS 51.90 | 57.49 |
| EWT test, gold (SLM tags) | UAS 55.55 | 60.61 |
| tales test, gold (SLM tags) | UAS 56.78 | 61.54 |

  Learned rules read like a grammar: NOUN before NOUN → compound; ADV before ADJ → advmod; "be" before ADJ → cop
  of the adjective; NOUN after "and" → conj of the previous noun; "and" between adjectives → cc. A first run chose
  one rule twenty times (its moves closed cycles and were skipped); fixed — such moves are no fixes.
- `world derive-scan <book-list> [--rounds R]` — the whole corpus (5.78 M sentences) parsed once and packed (~3 GB);
  each round shows what the deriver already derives (agreement by part of speech, biggest disagreements) and learns
  several non-interfering rules in two parallel passes.

The parser is far ahead (~80 LAS); the point is a deriver whose every arc has a reason, which can be inspected,
hidden rule by hidden rule, and improved by self-play.

Whole-corpus scan (2026-10-02, 5.52 M sentences, 139.7 M words): agreement with the parser after each round
(8 rules per round, two parallel passes) — 55.42 → 58.90 → 60.51 → 61.59 → 62.07 → 62.71 (rounds 0–5; the run hit
the 2-hour limit of a foreground task after round 5, 30 rules learned, kept in the run log). Next runs go detached
through `tools/jobs` and start from a `prefix=1%` sample of the tree store for minute-long checks.

Incremental learning (2026-10-02, after Ngai & Florian 2001): fix counts are updated only for the sentences a rule
touched, and an index from condition sets to words gives a candidate's breaks without a pass over the data. On
2 700 parser trees, 30 rules: the same agreement as the full rescoring (train UAS 62.92, held-out 63.66, EWT 60.53),
5× faster (10.2 s against 50.0 s); equivalent formulations of a few rules differ (ties). `DERIVE_FULL_RESCORE=1`
keeps the old path for comparison.
