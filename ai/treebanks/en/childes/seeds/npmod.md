# CHILDES — `:tmod` renamed, but `:npmod` remained

**Gist.** In 2.17 CHILDES replaced all `:tmod` with `:unmarked`, as UD 2.15 asks (docs#1028). `:npmod` was not touched: `obl:npmod` — 226, `nmod:npmod` — 15. In 2.15 both subtypes were merged, so CHILDES now has two labels for an NP without a preposition:
- `obl:unmarked` 494 / `nmod:unmarked` 110 — the former `:tmod`, temporal *today, morning, yesterday*;
- `obl:npmod` / `nmod:npmod` — measures and quantity *a little bit, a lot, way too…*.

In EWT since 2.15 both are `:unmarked`, and the temporal ones also have `TemporalNPAdjunct=Yes` in MISC. CHILDES does not set this trace.

**Conditions and exceptions.**
- `:npmod`: *way* 53, *bit* 46, *little* 42, *lot* 13, *years* 9, *time* 8, *minute* 8, *while* 7, *yourself* 6.
- `:unmarked`: *today* 163, *morning* 69, *yesterday* 59, *time* 49, *night* 48, *tomorrow* 40, *day* 36.
- *time* and *years* occur in both groups: the "time / measure" boundary in CHILDES is semantic, not formal.
- 35 of 845 such modifiers have a `case` preposition — for an NP without a preposition this is an error (`tb.atis.tmod-case`).

**Examples.** *a little bit* — *bit*: `obl:npmod`; *today* on a verb — `obl:unmarked`.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 241 / 241 | 0 / 0 |
| `tb.atis.tmod-case`: `:tmod`/`:npmod`/`:unmarked` with a preposition | 35 / 845 | 0 / 2604 |

**Sources.** README `UD_English-CHILDES`, v2.17 («Replace all `:tmod` with `:unmarked`»); https://universaldependencies.org/en/dep/obl-unmarked.html; docs#1028; EWT README v2.15.
