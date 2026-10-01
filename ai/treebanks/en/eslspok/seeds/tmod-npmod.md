# ESLSpok — the old subtypes `:tmod` and `:npmod` (before UD 2.15)

**Gist.** ESLSpok was annotated following UD 2.0 and not updated after 2.12. So NPs without a preposition acting as adverbials or modifiers have the old labels:
- `obl:tmod` — 77;
- `obl:npmod` — 50;
- `nmod:tmod` — 20;
- `nmod:npmod` — 4.

151 in total. In `mova`, as in EWT since 2.15, both subtypes are merged into `:unmarked`, and the temporal meaning is kept by `TemporalNPAdjunct=Yes` in MISC.

**Conditions and exceptions.**
- `:tmod` — temporal NPs: *week* 17, *day* 16, *time* 10, *today* 6, *tomorrow* 5, *night* 5, *morning* 5.
- `:npmod` — measures and quantity: *bit* 11, *years* 10, *times* 4, *little* 4.
- Only 4 of 151 have a preposition — real NPs without case, as the guidelines require. Compare with ATIS: 81% there.
- ESLSpok has not a single `:unmarked`.

**Examples.**
- `file00207.txt_69` *he runs every morning around here* — *morning*: `obl:tmod`.
- `file01070.txt_33` *last weekend my best friend in Tokyo came to Osaka…* — *weekend*: `obl:tmod`.

**In UD.**

| rule | ESLSpok | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 151 / 151 | 0 / 0 |
| `tb.atis.tmod-case`: of them with a preposition | 4 / 151 | 0 / 2604 |

**Sources.** https://universaldependencies.org/en/dep/obl-unmarked.html, `dep/nmod-unmarked.md`; docs#1028; EWT README v2.15 (`TemporalNPAdjunct=Yes`); README `UD_English-ESLSpok` (UD 2.0, changes — only 2.12).
