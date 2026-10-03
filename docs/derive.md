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
