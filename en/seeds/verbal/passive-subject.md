# Passive subject

**Gist.** In the passive the subject is what was the object in the active: *he loves her* → *she is loved (by him)*. Such a subject is not the doer but the one the action is directed at, so UD marks it separately — `nsubj:pass` (or `csubj:pass` when the subject is a whole clause). Conversely, a passive verb cannot have a plain `nsubj`.

**Conditions and exceptions.** The indirect object can also become the passive subject: *I was given a horse* (the direct object *horse* remains). Not every verb with an object has a passive: *cost, weigh, last, resemble* (see `lexicon/verb-no-passive.md`). A passive subject also occurs without an auxiliary: in absolute constructions (*with some used as building material*), in headlines (*Man arrested*) — so the third rule only warns.

**Examples.** *Dole was defeated by Clinton.* — *That she lied was suspected by everyone* (csubj:pass). — *Armed militias, many staffed by former soldiers, …* (no aux:pass).

**In UD.** Only heads with `Voice=Pass` have `nsubj:pass`/`csubj:pass`; a head with `Voice=Pass` has no `nsubj`/`csubj`; usually an `aux:pass` is present.

**Sources.** Jespersen, MEG III, ch. XV "Subject of Passive Verb", §15.11 (books/j/je/jespersen-1914-modern-english-grammar-historical, text-3, p. 313); Reed & Kellogg, Higher Lessons, Lesson 129; UD `docs/_en/dep/nsubj-pass.md`, `docs/_en/dep/csubj-pass.md`, `docs/_en/feat/Voice.md` ("Only Voice=Pass verbs may have passive subject dependents").

```rule
rule: en.verbal.passive-subject-voice
what: nsubj:pass and csubj:pass — only with Voice=Pass
match: h[]; s[rel=nsubj:pass|csubj:pass, head=h]
require: h[feats.Voice=Pass]
severity: error
source: UD docs/_en/feat/Voice.md
```

```rule
rule: en.verbal.passive-no-active-subject
what: a verb with Voice=Pass has no active subject nsubj/csubj
match: h[feats.Voice=Pass]
require: none s[rel=nsubj|csubj, head=h]
severity: error
source: UD docs/_en/feat/Voice.md ("it will be of the passive variety")
```

```rule
rule: en.verbal.passive-subject-aux
what: a passive subject usually has an aux:pass next to it (otherwise an absolute construction, a headline or an error)
match: h[]; s[rel=nsubj:pass, head=h]
require: exists a[rel=aux:pass, head=h]
unless: h[rel=advcl]; exists w[lemma=with, head=h]; none f[feats.VerbForm=Fin]
severity: warn
source: UD docs/_en/feat/Voice.md; Jespersen MEG III §15.11; exceptions from the seed: absolute construction (advcl; with some used as…), headline without a finite verb (Man arrested)
```
