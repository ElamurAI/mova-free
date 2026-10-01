# When, where, why, how in a subordinate clause — an adverb, not a conjunction

**Gist.** *When, where, why, how* introduce subordinate clauses (*He was upset **when** I talked to him*, *I know **where** she lives*), but, unlike *because* or *if*, they also play a role inside the subordinate clause — adverbial of time, place, reason, manner. So they are adverbs, not conjunctions.

**Conditions and exceptions.** In free relatives (*I looked **where** you were sitting*) *where* is the head, and the clause attaches to it as `advcl:relcl`. *Whenever, wherever* — the same.

**Examples.** *He was upset when I talked to him* — `advmod(talked, when)`, `advcl(upset, talked)`. — *I looked where you were sitting* — `advmod(looked, where)`, `advcl:relcl(where, sitting)`.

**In UD.** *when/where/why/how/whenever/wherever* — `ADV` (XPOS `WRB`), relation `advmod` (or the head of a free relative), never `mark`.

**Sources.** https://universaldependencies.org/en/dep/acl-relcl.html (WH-adverb relativizers attach as advmod; free relatives); https://universaldependencies.org/en/dep/advcl-relcl.html; https://universaldependencies.org/en/dep/advcl.html (*He was upset when I talked to him*); Reed & Kellogg, Higher Lessons, Lessons 59–60, 63 (conjunctive adverbs).

```rule
rule: en.verbal.wh-adverb-not-mark
what: when/where/why/how are adverbs (advmod), not mark
match: w[form=when|where|why|how|whenever|wherever]
require: not w[rel=mark]
severity: error
source: https://universaldependencies.org/en/dep/acl-relcl.html (WH-adverb relativizers → advmod)
```
