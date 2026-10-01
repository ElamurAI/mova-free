# ATIS — participial phrase after a noun — `acl:relcl`, not `acl`

**Gist.** *flights leaving on wednesday* — a participle without a subject and without a relative word. EWT and `mova` give it `acl`, because there is no relative clause here. ATIS almost always uses `acl:relcl`: 662 times of 672. Hence the label proportion in ATIS: `acl:relcl` — 1177, `acl` — 43.

**Conditions and exceptions.**
- The rule counts only `acl:relcl` with `VerbForm=Part` that have no subject or `mark`. Full relative clauses (*a flight that arrives…*) are not included.
- EWT keeps such reduced phrases as `acl` and since 2.14 adds an enhanced edge and `Cxn=rc-red-…` in MISC to them.

**Examples.**
- `0007.dev` *…flights from new york to montreal canada leaving on wednesday*: *leaving* — `acl:relcl` → *flights*.
- `0003.dev` *round trip flights … leaving next tuesday and returning the day after* — *leaving* likewise.

**In UD.**

| rule | ATIS | EWT | GUM | LinES |
|---|---:|---:|---:|---:|
| `tb.atis.relcl-participle`: `acl:relcl` on a participle without subject and `mark` | 662 / 672 | 13 / 527 | 38 / 615 | 34 / 280 |

**Sources.** https://universaldependencies.org/en/dep/acl.html (participial modifiers), `dep/acl-relcl.md`; EWT README v2.14 (#392: enhanced for reduced relatives, `Cxn=rc-red-…`).

```rule
rule: tb.atis.relcl-participle
what: participle as acl:relcl without subject and mark — in EWT this is acl
match: v[rel=acl:relcl, feats.VerbForm=Part]
require: exists c[rel~nsubj, head=v] or exists c[rel=mark, head=v]
severity: warn
source: UD en acl, acl:relcl; counter
```
