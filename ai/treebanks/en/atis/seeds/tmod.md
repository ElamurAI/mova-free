# ATIS — `:tmod` and `:unmarked` mean "time", even with a preposition

**Gist.** In UD `:tmod`, and since 2.15 `:unmarked`, is a nominal without a preposition (docs#1028). ATIS uses these subtypes as a semantic "temporal modifier" label, so 81% of them have `case`:
- `nmod:tmod` on a noun — 1263, with a preposition 1022: *flights **on** mondays*;
- `obl:unmarked` on a verb — 851, with a preposition 688: *leave **in** the morning*.

ATIS did the 2.15 renaming only for `obl`: there is not a single `nmod:unmarked`, and `nmod:tmod` remained. In EWT `:unmarked` never has a preposition (0 of 2604). Temporal phrases with a preposition there are plain `obl`/`nmod`: 178 of 179 on the words *morning, pm, monday…*. EWT keeps the temporal meaning of an NP without a preposition in MISC as `TemporalNPAdjunct=Yes`.

**Conditions and exceptions.**
- ATIS is inconsistent here too: of 319 `obl`/`nmod` on time words, 210 have a preposition and remained without a subtype.
- ATIS has no `npmod` (measures, *a lot*): all 1263 `nmod:tmod` are temporal.
- MISC is empty, there is no `TemporalNPAdjunct`.

**Examples.**
- `0008.dev` *list the tower air flights on mondays*: *mondays* — `nmod:tmod` → *flights*, with `case` *on*.
- `0003.dev` *…leaving next tuesday…*: *tuesday* — `obl:unmarked` without a preposition, as in EWT.

**In UD.**

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 1263 / 1263 | 0 / 0 |
| `tb.atis.tmod-case`: `:tmod`/`:unmarked` with a preposition | 1710 / 2114 | 0 / 2604 |

For LAS without subtypes (`tools/eval.py`) this difference does not matter; for LAS with subtypes it does.

**Sources.** https://universaldependencies.org/en/dep/obl-unmarked.html, `dep/nmod-unmarked.md` («a modifier structured as an NP without case marking»); docs#1028; EWT README v2.15 (`TemporalNPAdjunct=Yes`); digest, §2 («ATIS… still have tmod/npmod»).

```rule
rule: tb.atis.tmod-case
what: :tmod/:unmarked (NP without a preposition) with a case preposition — a semantic time label
match: t[rel=nmod:tmod|obl:tmod|nmod:unmarked|obl:unmarked|nmod:npmod|obl:npmod]
require: none c[rel=case, head=t]
severity: warn
source: UD en obl:unmarked, nmod:unmarked; counter
```
