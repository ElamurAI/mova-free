# ESLSpok — UD 2.0 customs: *when* as `mark`, *so* as CCONJ, Japanese insertions as `dep`, no new subtypes

**Gist.** ESLSpok was annotated following UD 2.0 (Kyle et al. 2022) and has not been updated since. So it lacks the changes that EWT adopted in 2.10–2.17:
- subordinate *when/where/how* are ADV `advmod` in EWT since 2.11; in ESLSpok *when* is SCONJ `mark`;
- "X, so Y" is annotated by EWT since 2.10 as `parataxis` with *so* — ADV `advmod`; in ESLSpok *so* is often CCONJ `cc`;
- EWT removed `flat:foreign` in 2.13; ESLSpok has it, and Japanese words in an English sentence have `dep`;
- no new subtypes: `obl:agent`, `nsubj:outer`, `advcl:relcl`, `nmod:desc`, `:unmarked` — 0.

**Conditions and exceptions.**
- *when/where/how/why* in `mark` — 41 of 103 (`tb.en.wh-advmod`). All 41 are *when* SCONJ.
- *so*:
  - ADV `advmod` — 271;
  - CCONJ `cc` — 40;
  - CCONJ `advmod` — 20.

  In EWT *so*: ADV `advmod` 531, SCONJ `mark` 75, CCONJ — none.
- Japanese insertions: UPOS X, XPOS FW:
  - `dep` — 26;
  - `flat:foreign` — 13, plus 1 `flat:foreign` that is not X;
  - `root` — 4.

  In EWT UPOS X is only on `goeswith` fragments and other cases with the tags FW, LS, XX, ADD, AFX, GW (https://universaldependencies.org/en/pos/X.html).
- There are only 2 passive *by* phrases, both `obl`. Counted by the rule `tb.eslspok.by-agent` below: VBN + `aux:pass` + *by*, without FEATS, because ESLSpok has none. EWT has 205 such, of which 196 are `obl:agent`.

**Examples.**
- `file01145.txt_29` *And when I go to my grandmother house , she feel very happy…* — *when*: SCONJ, `mark`.
- *to Fuji jukai* — *jukai*: X/FW, `flat:foreign` → *Fuji*.

**In UD.**

| rule | ESLSpok | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 41 / 103 | 0 / 1049 |
| `tb.eslspok.so-cconj`: *so* — CCONJ | 60 / 339 | 0 / 657 |
| `flat:foreign` (`grep`) | 14 | 0 |

**Sources.** EWT README v2.10 (#313 `parataxis` for "X so Y"), v2.11 (#88 WH adverbs — `advmod`), v2.13 (#459 no `flat:foreign`); UD docs/changes.md` No. 13 (foreign insertions, 2.14); `docs/foreign.md`; Kyle et al. 2022, `2022.bea-1.7` (UD 2.0).

```rule
rule: tb.eslspok.so-cconj
what: so as a coordinating conjunction CCONJ (EWT: ADV advmod or SCONJ mark)
match: s[form=so]
require: not s[upos=CCONJ]
severity: warn
source: EWT README v2.10 (#313); counter
```

```rule
rule: tb.eslspok.by-agent
what: by phrase on VBN with aux:pass — obl:agent (without FEATS, by XPOS)
match: v[xpos=VBN]; a[rel=aux:pass, head=v]; o[rel=obl|obl:agent, head=v]; b[form=by, rel=case, head=o]
require: o[rel=obl:agent]
severity: warn
source: EWT README v2.13; counter
```
