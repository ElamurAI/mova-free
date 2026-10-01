# Passive agent — obl:agent

**Gist.** In a passive sentence the *by*-phrase with the doer of the action is `obl:agent`; EWT has 376 of them. But *by* can also mark means or a time limit: *transported by land*, *by the 1920s* — then it is plain `obl`. Meaning tells them apart, and the formal hint is this: an agent usually has an article or possessive (*by the dog*, *by his wife*), while means is a bare noun (*by car*, *by hand*).

**Conditions and exceptions.** *by* with an article in a time sense (*by the end of the year*, *by the 1920s*) is `obl`. Proper names and pronoun agents (*by John*, *by them*) are not checked by the rule, since they have no article.

**Examples.**
- *The cat was chased by the dog* → obl:agent(chased, dog).
- *Arms are transported by land* → obl(transported, land).

**Check against gold.** EWT 2.18: plain `obl` with *by* and an article in a passive — 9, violations 1 (*by the 1920s*).

**Sources.** UD `_en/dep/obl.md` (passive agent), `_en/dep/obl-agent.md`.

```rule
rule: en.errors.by-agent
what: by-phrase with an article in a passive marked as plain obl — the agent should be obl:agent
match: v[xpos=VBN]; a[rel=aux:pass, head=v]; p[rel=obl, head=v]; c[rel=case, form=by, head=p]
require: none d[rel=det|nmod:poss, head=p]
severity: warn
source: UD _en/dep/obl.md, _en/dep/obl-agent.md
```
