# Infinitive of purpose — an adverbial (advcl), not a complement

**Gist.** An infinitive can be a complement of the verb (*I want **to help***) or an adverbial of purpose (*I came **to see** you* = in order to see). A complement is required by the verb itself; a purpose adverbial can be added to almost any action and replaced with *in order to*. A phrase with *in order to* is always an adverbial.

**Conditions and exceptions.** An infinitive of purpose after verbs of motion (*come, go, sit down*) is `advcl`; *go to see* ≠ *want to see*. An infinitive on a noun (*a house to live in*) is `acl`.

**Examples.** *He talked to him **in order to secure** the account* — `advcl`. — *I came to tell you* — `advcl`. — *I tried to finish it* — `xcomp`.

**In UD.** A clause with `mark(in)` + `fixed(order)` is not `xcomp`, not `ccomp`, not `csubj` (usually `advcl`).

**Sources.** UD `docs/_en/dep/advcl.md` (purpose clause: *in order to secure the account*); UD `docs/_en/dep/xcomp.md` (complements, not purpose clauses); Reed & Kellogg, Higher Lessons, Lesson 40 ("Frequently the infinitive phrase expresses purpose"); Poutsma 1923, *The Infinitive…*, §3 (to has expressed purpose from of old: *I came to tell you*; vol. 2, p. 15).

```rule
rule: en.verbal.in-order-to-advcl
what: a clause with in order to is an adverbial, not a complement
match: x[]; m[form=in, rel=mark, head=x]; f[form=order, rel=fixed, head=m]
require: not x[rel=xcomp|ccomp|csubj]
severity: warn
source: UD docs/_en/dep/advcl.md; UD docs/_en/dep/xcomp.md
```

## «So» of purpose before a finite clause

**Gist.** *So* (or *so that*) before a finite clause with *can* or *could* gives the purpose of the action: *I left the key under the mat **so** she can get it* (= in order that she can get it). It is then a subordinating conjunction: in UD EWT, SCONJ (XPOS IN) attached to the clause's predicate as `mark`. The same *so* after a comma gives a result and stays an adverb: *It was raining, **so** we stayed home* (ADV, `advmod`); so does *so* of degree (*so tired*).

**Conditions and exceptions.** EWT 2.18 train is not uniform: in the context «a word (no comma or coordinator), *so*, a pronoun or a determiner and a noun, *can* or *could* not negated» it has 18 SCONJ/`mark` and 4 ADV/`advmod`; with *will, would, may, must* the adverb (result) is more frequent, and with a comma before *so* the adverb prevails (16 ADV, 6 SCONJ before a modal). `en::tagrule` (R1) gives the tagger IN only in the first context; the others are left to the model.

**Examples.** *Please let me know whether you can attend **so** I can order enough food* (SCONJ, `mark`). *Extend the protection **so** wolves can move* (SCONJ in EWT; a bare noun subject is not in R1). *I was busy **so** I would wait* (ADV in EWT).

**In UD.** SCONJ, IN, `mark` on the predicate of the purpose clause, which is `advcl` of the main predicate; *so that* is `mark` + `fixed`.

**Sources.** UD `docs/_en/pos/SCONJ.md` (subordinating conjunctions introduce adverbial clauses); UD `docs/_en/dep/mark.md`; UD English EWT 2.18 train (counts above); Quirk, Greenbaum, Leech & Svartvik 1985, *A Comprehensive Grammar of the English Language*, ch. 15 (adverbial clauses: purpose with *so that* or *so* and a modal; result with *so (that)*, typically after a comma).

```rule
rule: en.verbal.so-purpose-mark
what: «so» + subject pronoun + can/could right after a word introduces a purpose clause: SCONJ mark
match: p[upos!=PUNCT|CCONJ]; s[form=so, next=p]; n[form=i|you|he|she|it|we|they, next=s]; m[form=can|could, next=n]
require: s[upos=SCONJ, rel=mark]
unless: exists x[form=n't|not|nt, next=m]
severity: warn
source: UD docs/_en/pos/SCONJ.md; UD docs/_en/dep/mark.md; Quirk et al. 1985 ch. 15 (clauses of purpose)
```
