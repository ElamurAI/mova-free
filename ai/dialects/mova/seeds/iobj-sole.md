# `iobj` without `obj` and with `xcomp` — proposal

**Proposal: accept** the 2.12 "Sole iobj" amendment and the EWT custom, not the text of `_en/dep/iobj.md` and `_en/specific-syntax.md` (`dialects/ud-2.18/seeds/iobj-sole.md`).

**What exactly in `mova`.**
- `iobj` can be the only internal argument (*remind me*, *ask Bush*);
- `iobj` can stand next to `xcomp` when the verb takes an addressee or recipient: *ask/tell/allow/convince/urge/persuade/cause/trust/teach/remind/warn **X** to do*;
- with *let, make, get, have, keep, find, want, help* + `xcomp` — `obj`;
- the boundary between these two classes is set by the lexicon (`en/seeds/lexicon/verb-iobj-licensors.md`, VerbNet), not by examples;
- *permit* and *recommend* have both variants in EWT. `mova` takes whichever the lexicon says once its rule matures, and until then — as in gold.

**Why.**
- **Matches the universal guideline.** The 2.12 amendment is of type AMENDMENT and cancels the condition on which the `_en` text rests.
- **More informative.** `iobj` with *ask X to* distinguishes the addressee from the "causee" `obj` with *make X do*. Merging into `obj`, as `_en` says, would lose this difference.
- **Lossless.** Export to EWT 2.18 is identity. If the literal `_en` text is ever needed (with `xcomp` — only `obj`), the `iobj` → `obj` transition is deterministic. The reverse transition goes through the lexicon.

**Evidence from the data** (EWT 2.18, engine):
- `iobj` — 795; without `obj`/`ccomp` — 243; with `xcomp` — 93;
- the same verb class with `obj` + `xcomp` — 39 (*encourage*, *enable*, *order*, *invite*, *force*, *require*);
- `iobj` is present in GUM (482), LinES (101), ParTUT (33), GENTLE (31).

**Origin.** `dialects/ud-2.18/seeds/iobj-sole.md`; `2.18:UD docs/changes.md#sole-iobj`; report `data/runs/bones-2026-09-25/verbal.md`.

Mova decides.
