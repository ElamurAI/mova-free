# Expletive it: extraposition, cleft, weather

**Gist.** *It* can be an empty filler. (1) Extraposition: the real clausal subject stands at the end, and *it* takes its place: *It is important **that your students respect you***, *It's hard **to make money***. (2) Cleft — focusing: *It was **Joseph Goebbels** who said that*. (3) Weather and time: *It is raining*, *It's late*. (4) In the object slot: *I find it best not to think about that*.

**Conditions and exceptions.** In "tough" sentences without *it* (*This problem is hard to solve*) the infinitive is no longer a subject but an `xcomp` on the adjective; with *it* — `csubj`. *It* as an ordinary pronoun (*I bought a car; it is red*) is `nsubj`. Rarely, the filler in a cleft is *that* (*that was 2 days ago that I called*).

**Examples.** *It is rare to find such nice workers* (expl + csubj). — *It's John who we want to help* (expl + advcl:relcl). — *It is raining* (expl without a clause).

**In UD.** Expletive *it* — `expl`, `PRON`. Next to it (on the same head) there is usually a clause: `csubj`, `csubj:pass`, `ccomp`, `xcomp`, `advcl`, `advcl:relcl`; without one it is weather *it* or an error. With an adjective with `expl` *it*, the infinitive is `csubj`, not `xcomp`.

**Sources.** https://universaldependencies.org/en/dep/expl.html; https://universaldependencies.org/en/specific-syntax.html, Core arguments (*It is raining*, *It was Joseph Goebbels who said that*, *I find it best…*) and *Tough*-constructions; https://universaldependencies.org/en/dep/acl-relcl.html, It-clefts; https://universaldependencies.org/en/dep/csubj.html; UD 2.18 validator, `rel-upos-expl`.

```rule
rule: en.verbal.expl-it-has-clause
what: expletive it usually has a clause on its head (except weather it)
match: h[]; e[form=it, rel=expl, head=h]
require: exists c[rel=csubj|csubj:pass|ccomp|xcomp|advcl|advcl:relcl, head=h]
severity: warn
source: https://universaldependencies.org/en/dep/expl.html; https://universaldependencies.org/en/specific-syntax.html, Tough-constructions
```

```rule
rule: en.verbal.tough-expl-csubj
what: with an adjective with expletive it the infinitive is csubj, not xcomp
match: a[upos=ADJ]; e[form=it, rel=expl, head=a]; x[rel=xcomp, head=a]
require: not x[rel=xcomp]
severity: warn
source: https://universaldependencies.org/en/specific-syntax.html, Tough-constructions ("the lower predicate must be a csubj")
```

```rule
rule: en.verbal.expl-is-pron
what: expl is a pronoun
match: e[rel=expl]
require: e[upos=PRON]
severity: error
source: UD 2.18 validator rel-upos-expl
```

```rule
rule: en.verbal.expl-it-or-there
what: expl in English is it or there (rarely that in a cleft)
match: e[rel=expl]
require: e[lemma=it|there]
severity: warn
source: https://universaldependencies.org/en/dep/expl.html
```
