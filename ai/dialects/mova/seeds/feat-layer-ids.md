# Feature layer identifiers — only `[a-z]+` — proposal

**Proposal: accept.** Name `mova`'s own layered features, if any appear (for example, for agreement with several participants in other languages), with lowercase Latin letters only, no digits.

**Why.**
- the post-2.18 guideline requires exactly this (https://universaldependencies.org/u/overview/feat-layers.html:80`);
- the validator also lets digits through (`utils.py:60`, `\[[a-z0-9]+\]`), but Old Georgian has already renamed `sauf2` → `dsauf`. So digits will eventually become an error;
- for English nothing changes now: there are no layers;
- no loss.

**Origin.** `dialects/ud-next/seeds/feat-layer-ids.md`.

Mova decides.
