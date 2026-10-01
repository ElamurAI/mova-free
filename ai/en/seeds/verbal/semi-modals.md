# Have to, be going to, used to — not auxiliaries

**Gist.** The phrases *have to* (must), *be going to* (intend), *used to* (habitual past) are close to modals in meaning, but grammatically they are ordinary verbs with an infinitive: *have* here inflects (*has to, had to*), takes do-support (*doesn't have to*), has an infinitive (*will have to*). *Ought to*, on the contrary, is a true modal.

**Conditions and exceptions.** *Be* in *be going to* is an ordinary `aux` on *going*. *Gonna* is tokenized as *gon* + *na*: *gon* is `VERB`, *na* is `PART` (mark). *Had better* is a special fixed combination; *better* is not a verb here.

**Examples.** *I have to leave.* — *I'm going to take a nap.* — *It used to snow here.* — *You ought to go* (ought — AUX, MD).

**In UD.** *have/going/used* — `VERB`, the head; the infinitive is `xcomp` with `mark(to)`. Not `aux`.

**Sources.** https://universaldependencies.org/en/pos/AUX_.html (*Now I have to buy some more* — VERB); https://universaldependencies.org/en/specific-syntax.html (*I 'm going to take a nap*, *gon na kick it*); Poutsma 1923, *The Infinitive…*, §5, Obs. II (ought "almost regularly" with to; vol. 2, p. 20), §32 (had better, had need without to; p. 42); Brown 1851, Rule XVII, Note XII (would rather, not had rather).

```rule
rule: en.verbal.have-to-verb
what: have in have to + infinitive is VERB, not an auxiliary
match: v[lemma=have]; x[rel=xcomp, head=v]; t[form=to, rel=mark, head=x]
require: v[upos=VERB]
severity: error
source: https://universaldependencies.org/en/pos/AUX_.html
```

```rule
rule: en.verbal.going-to-verb
what: going (gon) with an infinitive is VERB
match: g[form=going|gon]; x[rel=xcomp, head=g]
require: g[upos=VERB]
severity: error
source: https://universaldependencies.org/en/specific-syntax.html, Predicates; Complementizers
```

```rule
rule: en.verbal.used-to-verb
what: used in used to + infinitive is VERB
match: v[form=used]; x[rel=xcomp, head=v]; t[form=to, rel=mark, head=x]
require: v[upos=VERB]
severity: error
source: UD 2.18 aux registry (used is not among the auxiliaries); https://universaldependencies.org/en/pos/AUX_.html
```
