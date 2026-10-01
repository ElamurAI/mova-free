# Get: passive auxiliary or lexical verb

**Gist.** Colloquial *get* with a past participle forms a passive (*The book got stolen*, *I got put on hold*) — then it is an auxiliary. In other uses *get* is an ordinary verb: "become" with an adjective (*Bill got rich*), causative (*I got it fixed* — "I had it fixed"), "receive" (*I got a letter*), *have got*.

**Conditions and exceptions.** Causative *get* + object + participle (*got it fixed*) is not a passive of *get* itself: *it* is the object, *fixed* is `xcomp`. *get* has no auxiliary function other than the passive one.

**Examples.** *He got shot.* (AUX, aux:pass) — *Bill got rich.* (VERB + xcomp) — *I got it fixed.* (VERB + obj + xcomp).

**In UD.** *get* as `AUX` — only `aux:pass` (or the head under ellipsis, `conj`).

**Sources.** https://universaldependencies.org/en/dep/aux_.html ("We allow the get-passive … not … become"); https://universaldependencies.org/en/pos/AUX_.html (get: usually VERB); https://universaldependencies.org/en/specific-syntax.html, Auxiliaries (*I got put on hold twice*).

```rule
rule: en.verbal.get-aux-passive-only
what: get as AUX is only a passive auxiliary
match: g[lemma=get, upos=AUX]
require: g[rel=aux:pass|conj|root]
severity: error
source: https://universaldependencies.org/en/dep/aux_.html; https://universaldependencies.org/en/pos/AUX_.html
```
