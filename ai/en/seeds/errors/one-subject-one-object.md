# One subject and one direct object per predicate

**Gist.** A predicate has at most one subject (`nsubj`, `nsubj:pass`, `csubj`) and at most one direct object (`obj`). Two subjects mean the roles are mixed up. The typical case is an object relative clause (*the book that the students read*): *that* is obj, *students* is nsubj, but the annotator makes both subjects. Another is a subordinate-clause subject attached to the main verb. Object relative clauses and long-distance dependencies are generally the hardest for both humans and language models.

**Conditions and exceptions.**
- The "outer" subject of a copular sentence with a clausal predicate is a separate subtype `nsubj:outer` and is not covered by the rule (*The problem is that this has never been tried*).
- Coordinated subjects are one `nsubj` plus `conj`.

**Examples.**
- *the book that the students read* → obj(read, that), nsubj(read, students).
- *The problem is that this has never been tried* → nsubj:outer(tried, problem), nsubj:pass(tried, this).

**Check against gold.** EWT 2.18: two subjects — 0 of 21 677; two obj — 0 of 12 254.

**Sources.** UD `_en/dep/nsubj.md`, `_en/dep/nsubj-outer.md`, `_en/dep/obj.md`, `_en/dep/cop.md`; `2023.udw-1.7` (v2.11: nsubj:outer); `2020.tacl-1.25` (BLiMP: filler-gap and islands are the hardest for models).

```rule
rule: en.errors.one-subject
what: the predicate has two subjects (nsubj, nsubj:pass, csubj) — roles mixed up (except nsubj:outer)
match: v[]; s[rel=nsubj|nsubj:pass|csubj|csubj:pass, head=v]
require: none x[rel=nsubj|nsubj:pass|csubj|csubj:pass, head=v]
severity: error
source: UD _en/dep/nsubj.md, _en/dep/nsubj-outer.md
```

```rule
rule: en.errors.one-object
what: the predicate has two direct objects — the second should be iobj, xcomp or belong to another predicate
match: v[]; o[rel=obj, head=v]
require: none x[rel=obj, head=v]
severity: error
source: UD _en/dep/obj.md, _en/dep/iobj.md
```
