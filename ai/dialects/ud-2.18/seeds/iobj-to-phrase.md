# A *to* phrase on a verb: nmod in the text, obl in the data

**Gist.** The `iobj` page says that *to me* in *she gave it to me* attaches as `nmod`. This is a UD v1 leftover. In v2 a nominal with a preposition on a verb is `obl`, and EWT writes only `obl`.

**Guideline.** `2.18:https://universaldependencies.org/en/dep/iobj.html:17–19`: «the _to me_ part is attached as [nmod]() although semantically it corresponds to the dative». The universal `obl` and `nmod` (rewritten in 2.18, `changes.md` No. 17): a dependent of a verb is `obl`, a dependent of a noun is `nmod`.

**EWT 2.18 data** (engine):
- a phrase with `case` *to* on a VERB: `obl` — 1285, `nmod` — 0.

**Where the discrepancy comes from.** The page text has not been updated since v1.

**Sources.** `2.18:https://universaldependencies.org/en/dep/iobj.html; `2.18:UD docs/changes.md` No. 17 «`nmod` and `obl`».

```rule
rule: ud218.to-phrase-obl
what: to phrase on a verb as obl — iobj.md says nmod
match: v[upos=VERB]; n[rel~obl, head=v]; c[lemma=to, rel=case, head=n]
require: n[rel~nmod]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/dep/iobj.html:18
```

```rule
rule: ud218.to-phrase-nmod
what: to phrase on a verb as nmod — as iobj.md says
match: v[upos=VERB]; n[rel~nmod, head=v]; c[lemma=to, rel=case, head=n]
require: n[rel~obl]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/dep/iobj.html:18
```
