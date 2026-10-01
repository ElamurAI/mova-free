# so big a house: adjective before the indefinite article

**Gist.** When an adjective is intensified by *so, as, too, how, however*, the indefinite article stands **after** the adjective: *so big a house, too good a chance, as good a scholar as he, how long a wait*. Without such an intensifier the adjective cannot stand before the article.
**Conditions and exceptions.** An alternative order with the adjective after the noun is also used: *a power so strong* (Curme). *Such a big house* — *such* is a predeterminer, and the adjective comes after the article. *Quite a, rather a* are predeterminers. Poutsma: likewise *no* + comparative (*no better a man*).
**Examples.** ✓ *too costly a sacrifice*; ✓ *so harsh an answer*; ✗ *big a house*.
**In UD.** `amod(house, big)`, `det(house, a)`, `advmod(big, so)`: the adjective is to the left of `det`. EWT 2.18: `amod` before *a/an* of the same noun — 4 times, all with `advmod` *as/how*.
**Sources.** Poutsma GLME vol. 1, Ch. VIII §151, §153 (pp. 554–556); Curme 1931 §10 I 1 (p. 65: *too costly a sacrifice*, *so harsh an answer*).

```rule
rule: en.nominal.adj-before-article
what: an adjective before the article a/an — only with an intensifier so/as/too/how/however
match: n[]; d[lemma=a, rel=det, head=n]; a[upos=ADJ, rel=amod, head=n, before=d]
require: exists c[head=a, rel=advmod, lemma=so|as|too|how|however|this|that|no]
severity: warn
source: Poutsma GLME I Ch. VIII §151; Curme 1931 §10 I 1
```
