# In a copular clause the head is the predicative

**Gist.** UD makes the head of the clause not the copula *be* but what is asserted about the subject: a noun, adjective, prepositional phrase, pronoun. So the subject, negation and adverbials attach to the predicative: *Bill is **honest*** — `nsubj(honest, Bill)`. A noun or pronoun as predicate cannot have a direct object — only verbs have objects (and *worth, like, unlike*).

**Conditions and exceptions.** A verb under a copula occurs only in a predicate clause (*The important thing is **to keep** calm*) — then there is an outer subject `nsubj:outer`. In questions the head is the question word: *What is that?* — `cop(What, is)`.

**Examples.** *Sue is a true patriot.* — *The light is on* (head — the preposition *on*). — *The plan is to leave* (`nsubj:outer`).

**In UD.** A `cop` head with UPOS `NOUN`, `PROPN`, `PRON`, `NUM` has no `obj`/`iobj`; a `cop` head with UPOS `VERB` has `nsubj:outer` or `csubj:outer`.

**Sources.** https://universaldependencies.org/en/dep/cop.html; https://universaldependencies.org/en/specific-syntax.html, Copulas and Core arguments ("exclusive to verbal predicates and a handful of adjectives"); Brown 1851, Rule VI "Same Cases" (a noun after an intransitive is in the same case, not an object); Reed & Kellogg, Higher Lessons, Lesson 29.

```rule
rule: en.verbal.cop-nominal-no-object
what: a nominal predicate with a copula has no direct or indirect object
match: h[upos=NOUN|PROPN|PRON|NUM]; c[rel=cop, head=h]
require: none o[rel=obj|iobj, head=h]
severity: error
source: https://universaldependencies.org/en/specific-syntax.html, Core arguments; Brown 1851 Rule VI
```

```rule
rule: en.verbal.cop-verb-head-outer
what: a verb under a copula only in a predicate clause with an outer subject
match: h[upos=VERB]; c[rel=cop, head=h]
require: exists o[rel=nsubj:outer|csubj:outer, head=h]
severity: warn
source: https://universaldependencies.org/en/dep/cop.html (predicate clause); https://universaldependencies.org/en/dep/nsubj-outer.html
```
