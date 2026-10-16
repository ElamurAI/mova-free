# CHILDES — customs before EWT 2.11: *when/how* as SCONJ `mark`, relative *that* as `mark`, *that* as a DET pronoun

**Gist.** Part of CHILDES comes from the UD 1 guidelines (S+24, converted to 2.0 by a script) and from early UD 2. So several customs there are older than EWT 2.11:
1. **Subordinate *when/how/where/why*** — SCONJ `mark`. EWT since 2.11 (#88) uses ADV `advmod`: 676 of 3308 in CHILDES, 0 of 1049 in EWT.
2. **Relative *that*** — `mark` without a role in the clause: 61 of 271 relative *that/which* before an `acl:relcl` verb. In EWT — 1 of 909.
3. **Demonstrative *that/this* as a noun** — DET, not PRON: 304 of 5970. In EWT — 0 of 1531 (PRON since v1.2).

**Conditions and exceptions.**
- Among subordinate WH in `mark`: *when* SCONJ — 420, *how* SCONJ — 132. The other *where, why* are mostly ADV `advmod`, as in EWT: *where* 593, *why* 533, *how* 574.
- DET pronoun *that/this*: `nsubj` 111, `obj` 41, `advmod` 14, `obl` 7, `root` 4.
- CHILDES has no features, so `PronType=Rel` on relatives (`tb.en.rel-prontype` 242 of 242) is not a custom but a missing layer.
- The customs seem to follow the sources, like the lemmas in `pron-lemma.md`, but the `corpus_name` metadata do not show this directly.

**Examples.**
- `34984` *Hold it up so Adam can see how it looks.* — *how*: SCONJ `mark`; in EWT — ADV `advmod`.
- `34841` *And then after that we will play a long game right?* — *that*: DET as a noun with *after*; in EWT — PRON.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 676 / 3308 | 0 / 1049 |
| `tb.atis.relcl-mark`: relative *that/which* — `mark` | 61 / 271 | 1 / 909 |
| `tb.en.dem-pron`: *this/that…* as a noun not PRON | 304 / 5970 | 0 / 1531 |
| `tb.en.to-case`: *to* in `case` not ADP | 24 / 1104 | 0 / 2029 |
| `tb.en.pass-subj`: subject with `aux:pass` — `nsubj` | 22 / 263 | 0 / 1405 |

**Sources.** EWT README v1.2 (this/that → PRON), v2.11 (#88 WH adverbs — `advmod`, relatives); Yang et al. 2025, `2025.udw-1.6`, sec. 3 («As S+24 was annotated using the UD guidelines version 1.0, we convert the annotation using UD version 2.0 with a script…»); README `UD_English-CHILDES`, v2.17.
