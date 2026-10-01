# The agent in the passive: by + noun

**Gist.** Who performed the action in a passive sentence is shown by a *by* phrase: *The cat was chased **by the dog***. A test from Reed & Kellogg's grammar: if *by + agent* can be added to the form *be + participle* without changing the meaning, it is a passive; if not, it is an adjective after a copula (*The coat was badly worn*).

**Conditions and exceptions.** Not every *by* is an agent: *by the window* (place), *by Friday* (time) are plain `obl`. An agent also occurs with a passive participle without an auxiliary: *paintings eaten by moths*.

**Examples.** *We were delighted by the snow.* — *It has been eaten by moths.* — *He sat by the fire* (not an agent, obl).

**In UD.** The agent is `obl:agent`, the preposition *by* is its `case`; the head is a verb with `Voice=Pass`.

**Sources.** https://universaldependencies.org/en/dep/obl-agent.html, https://universaldependencies.org/en/feat/Voice.html ("Only Voice=Pass verbs may have obl:agent dependents"); Reed & Kellogg, Higher Lessons, Lesson 129, Remark (the by-agent test); Poutsma 1923, *The Infinitive…*, §85 (a by-phrase makes the passive inevitable; vol. 2).

```rule
rule: en.verbal.agent-passive-by
what: obl:agent — only with Voice=Pass and with the preposition by
match: h[]; g[rel=obl:agent, head=h]
require: h[feats.Voice=Pass]; exists c[form=by, rel=case, head=g]
severity: error
source: https://universaldependencies.org/en/dep/obl-agent.html; https://universaldependencies.org/en/feat/Voice.html
```
