# Get: passive auxiliary or lexical verb

**Gist.** Colloquial *get* with a past participle forms a passive (*The book got stolen*, *I got put on hold*) — then it is an auxiliary. In other uses *get* is an ordinary verb: "become" with an adjective (*Bill got rich*), causative (*I got it fixed* — "I had it fixed"), "receive" (*I got a letter*), *have got*.

**Conditions and exceptions.** Causative *get* + object + participle (*got it fixed*) is not a passive of *get* itself: *it* is the object, *fixed* is `xcomp`. *get* has no auxiliary function other than the passive one.

**Examples.** *He got shot.* (AUX, aux:pass) — *Bill got rich.* (VERB + xcomp) — *I got it fixed.* (VERB + obj + xcomp).

**In UD.** *get* as `AUX` — only `aux:pass` (or the head under ellipsis, `conj`).

**Sources.** UD `docs/_en/dep/aux_.md` ("We allow the get-passive … not … become"); UD `docs/_en/pos/AUX_.md` (get: usually VERB); UD `docs/_en/specific-syntax.md`, Auxiliaries (*I got put on hold twice*).

```rule
rule: en.verbal.get-aux-passive-only
what: get as AUX is only a passive auxiliary
match: g[lemma=get, upos=AUX]
require: g[rel=aux:pass|conj|root]
severity: error
source: UD docs/_en/dep/aux_.md; UD docs/_en/pos/AUX_.md
```
