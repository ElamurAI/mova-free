# "It is easy to X" and "X is easy to do" — different structure

**Gist.** Two similar sentences have different trees.
- **Extraposition** (*It is easy to make money*). The subject is the infinitive clause, and *it* is an empty placeholder `expl`. The infinitive here is `csubj`.
- **"Tough" construction** (*This book is easy to read*). The subject is *book*, the infinitive complements the adjective: in EWT it is `ccomp`, sometimes `advcl`. `csubj` is impossible here, because there is already a subject.

BLiMP has separate "tough vs raising" paradigms for this; language models make mistakes on them.

**Conditions and exceptions.** Tough adjectives: *easy, hard, difficult, impossible, tough, simple, pleasant*. *too early to say* is a different construction with *too*.

**Examples.**
- *It is hard to say* → expl(hard, It), csubj(hard, say).
- *Fish are the easiest to take care of* → nsubj(easiest, Fish), ccomp(easiest, take).

**Check against gold.** EWT 2.18:
- `expl` + infinitive not as `csubj` — 4 of 107;
- noun subject + infinitive as `csubj` — 0 of 113.

**Sources.** UD `_en/dep/csubj.md`, `_en/dep/expl.md`; `2020.tacl-1.25` (BLiMP: CONTROL/RAISING, paradigms tough vs raising 1–2).

```rule
rule: en.errors.extraposed-infinitive-csubj
what: it-extraposition (It is easy to X) — the infinitive is csubj of the adjective
match: a[upos=ADJ]; e[rel=expl, head=a]; v[upos=VERB, head=a, after=a]; t[form=to, rel=mark, head=v]
require: v[rel~csubj]
severity: warn
source: UD _en/dep/csubj.md, _en/dep/expl.md; 2020.tacl-1.25 (tough vs raising)
```

```rule
rule: en.errors.tough-infinitive-not-csubj
what: tough construction (X is easy to do) — there is already a subject, the infinitive cannot be csubj
match: a[upos=ADJ]; s[rel=nsubj, head=a, upos=NOUN|PROPN]; v[upos=VERB, head=a, after=a]; t[form=to, rel=mark, head=v]
require: not v[rel~csubj]
severity: warn
source: UD _en/dep/csubj.md; 2020.tacl-1.25 (tough vs raising)
```
