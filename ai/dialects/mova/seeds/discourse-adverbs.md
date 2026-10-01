# `discourse` only for interjections, particles, numbering and emoticons — proposal

**Proposal: accept** the new definition of `discourse` (`dialects/ud-next/seeds/discourse-scope.md`). Do not introduce MISC marks for the 9 EWT adverbs.

**What exactly in `mova`.**
- ADV is never `discourse`, but `advmod`: *though*, *so*, *also*, *maybe*, *btw*;
- a prepositional phrase is never `discourse`, but `obl` (*in other words*);
- `discourse` remains for INTJ (*oh*, *well*, *like*, *um*, *yes*), item numbering (NUM, *1.*, *(a)*), emoticons (SYM), *thanks*, *ps*.

**Why.**
- **More consistent.** The criterion is the word class, visible without interpreting pragmatics. In EWT *though* is `advmod` 42 times and `discourse` 3; *actually* is `advmod` 83 times out of 83. In GUM *so* is `discourse` 57 times and `advmod` 622.
- **Lossless into the standard.** The 2.18 guideline already excluded adverbs ("non-adverbial discourse markers", `dialects/ud-2.18/seeds/discourse-adverbs.md`). So `advmod` is the correct export both in 2.18 and in `ud-next`.
- **What we pay.** The reverse run of EWT `mova → 2.18` diverges from gold in 9 tokens (train 4, dev 1, test 4), in GUM — 78. These are divergences from gold errors, not from the guideline, so it is not worth preserving them with a MISC mark. In the reverse-run report they should be shown as a separate line.

**Evidence from the data** (EWT 2.18): `discourse` — 1066: INTJ 775, SYM 123, NUM 113, NOUN 27, VERB 12, ADV 9, PROPN 3, ADJ 3, X 1. Prepositional phrases with `discourse` — 0.

**Converters.**
- EWT, GUM → `mova`: rule `mova.ud-next.discourse-adv` (`dialects/ud-next/convert.md`);
- `mova` → 2.18 / `ud-next`: no change;
- DEPS: in EWT `N:discourse` will remain. An action for DEPS or an EUD rebuild is needed (see `convert.md`).

**Check.** Gates `udnext.discourse-not-adv` and `udnext.discourse-not-pp` (error).

**Origin.** `dialects/ud-next/seeds/discourse-scope.md`, `dialects/ud-2.18/seeds/discourse-adverbs.md`.

Mova decides.
