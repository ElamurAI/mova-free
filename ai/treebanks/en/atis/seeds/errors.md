# ATIS — known annotation errors

**Gist.** Besides the customs that distinguish ATIS from EWT (the other seeds in this folder), ATIS has errors that contradict the UD guidelines themselves. The converter does not fix them, because an error is not a dialect. They are candidates for tagger and parser suspicions and for edits.

**Conditions and exceptions.** Counters — the general rules of `../../seeds-overview.md` and the rules in this folder, violations / matches, all parts together.

**Examples and numbers.**
- **Passive subject is `nsubj`; there is not a single `nsubj:pass`.** `tb.en.pass-subj` — 10 of 10 passives with `aux:pass`: `1185.train` *what kind of aircraft is used on…* — *kind* `nsubj`. There is no `Voice=Pass` either (`verb-feats.md`).
- **A verb with UPOS NOUN.** `0539.dev` *flight will start from boston* — *start*: NOUN, head, has `aux` *will* and `obl` *boston*. `0220.train` *what cities does continental service* — *service*: NOUN, although it has `aux` *does*.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`): 22 of 30. Some are query fragments where EWT would give `nmod`: `0254.train` *flights from oakland to san francisco on january twenty first 1992*. Some are a consequence of the false NOUN above.
- **A preposition as `amod`** — 3 times: `0003.dev` *…returning the day after*: *after* — ADP, `amod` → *day*. In EWT postposed *after, before* are ADV `advmod`: *before* 41, *after* 5.
- **Relative *that* has three different UPOS in the same `nsubj` role**: ADP 42, DET 29, PRON 23 (`relative-that-mark.md`).
- **Demonstrative *this/that* with `PronType=Art`** — 67 (`pos-feats-from-ptb.md`).
- **The README does not match the data:** train 4224 sentences in the README, 4274 in the file.

**In UD.** Of these errors only `nsubj` instead of `nsubj:pass` is systematic: it is on all 10 passives. It can be converted by a rule, and in `convert.md` it is listed as a custom. The rest are isolated. For training, such sentences are better passed through suspicions than fixed by the converter.

**Sources.** https://universaldependencies.org/en/dep/nsubj-pass.html, `docs/_u/overview/…` (obl and nmod, UD 2.18: `obl-should-be-nmod`); UD docs/changes.md` No. 17; README `UD_English-Atis`.
