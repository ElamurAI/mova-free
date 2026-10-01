# LinES — structures before EWT 2.11–2.16: *when* as `mark`, *by* without `obl:agent`, no `cc:preconj` and `det:predet`

**Gist.** LinES was updated in UD selectively. It already has `:unmarked` (2.15) and `nmod:desc` (2.16): `obl:unmarked` 229, `nmod:unmarked` 162, `nmod:desc` 142. But some of EWT's older English customs are missing:
1. **Subordinate *when/where/how/why*** — SCONJ `mark` in 256 of 524 cases. EWT since 2.11 (#88) — ADV `advmod`, 0 of 1049.
2. **The passive agent *by X*** — plain `obl`, `obl:agent` only 4. EWT since 2.13 uses `obl:agent`: in LinES 152 of 166 such *by* phrases lack it.
3. **No `cc:preconj`:** *both/either/neither … and/or* — CCONJ `cc` (34). In EWT — `cc:preconj` (98).
4. **No `det:predet`:** *all the* — `det` (68), *such a* — `amod` (18), *half* — `amod` (3). In EWT *all the* and *such a* — `det:predet` (226).
5. **Not a single `advcl:relcl`, `goeswith`, `list`.**

**Conditions and exceptions.**
- `nsubj:outer` is present (23), `csubj:outer` — 1: LinES adopted this 2.11 change.
- `dislocated` — 131, in EWT — 7. Fiction has more fronted constituents, but an 18-fold difference looks like a broader interpretation of the label: PRON 60, NOUN 44.
- `acl` — 1070 versus `acl:relcl` 958. Among participles, `acl:relcl` without subject and `mark` — 34 of 280, almost as in EWT (13 of 527).

**Examples.**
- `en_lines-ud-dev-doc1-3177` *When a user connects to the SQL Server database…* — *When*: SCONJ `mark`.
- `en_lines-ud-dev-doc1-3204` *…a standards-based protocol that is governed by the World Wide Web Consortium (W3C).* — *Consortium*: `obl`, not `obl:agent`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 256 / 524 | 0 / 1049 |
| `tb.en.obl-agent`: *by* agent on a passive participle not `obl:agent` | 152 / 166 | 16 / 411 |
| `tb.lines.preconj`: *both/either/neither* CCONJ — not `cc:preconj` | 34 / 34 | 2 / 100 |
| `tb.lines.predet`: *all/such/half/both* before an article or possessive — not `det:predet` | 90 / 90 | 22 / 204 |

**Sources.** EWT README v2.11 (#88, `:outer`), v2.13 (`obl:agent`, #290), v2.16 (*such* as predet, docs#1114); https://universaldependencies.org/en/dep/cc-preconj.html, `dep/det-predet.md`, `dep/obl-agent.md`.

```rule
rule: tb.lines.preconj
what: both/either/neither before coordination — cc:preconj (LinES: cc)
match: p[form=both|either|neither, upos=CCONJ]
require: p[rel=cc:preconj]
severity: warn
source: UD en cc:preconj; counter
```

```rule
rule: tb.lines.predet
what: all/such/half before an article or possessive — det:predet
match: p[form=all|such|half|both]; d[upos=DET|PRON, rel=det|nmod:poss, next=p]
require: p[rel=det:predet]
severity: warn
source: UD en det:predet; EWT 2.16; counter
```
