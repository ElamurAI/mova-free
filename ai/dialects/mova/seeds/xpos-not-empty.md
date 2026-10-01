# XPOS is never empty — proposal

**Proposal: accept** the validator 0.2.8 rule (`empty-string-in-xpos`) as a CoNLL-U writer gate for all export targets, not only `ud-next`.

**What exactly in `mova`.** An unknown XPOS is written as `_`; an empty string is never written. This mainly concerns future languages with string XPOS (`train/dialects-and-converters.md`, section "Training on the metadialect"). In `en`, XPOS is always a PTB tag.

**Why.**
- the 2.19 validator will reject a file with an empty XPOS as a level 2 error;
- for 2.18 `_` is also correct, so there is no loss;
- right now there is no empty XPOS anywhere: EWT, 13 other English 2.18 treebanks and `corpus/mova-*-en.conllu` — 0 lines (grep).

**Check.** This cannot be expressed in the seed rule language: it does not see raw columns. The gate is in the writer code, with a negative control: a line with an empty XPOS must produce an error.

**Origin.** `dialects/ud-next/seeds/xpos-not-empty.md`.

Mova decides.
