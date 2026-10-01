# Direct object: only on a verb, only one

**Gist.** A direct object is a noun that complements a transitive verb without a preposition: *Washington captured **Cornwallis***. Only verbs have it (and three adjectives: *worth, like, unlike*): *It's worth **the money***. A predicate has one direct object; a second "object" is either an indirect one (*gave **me** a raise*), or a predicative on the object (*made him **king***; see `objective-complement.md`), or an adverbial without a preposition (*walked **three miles***, *arrived **this morning***).

**Conditions and exceptions.** Coordinated objects (*captured Cornwallis and his army*) are one `obj` with `conj`. Lexical *have/do* with an object is `VERB`. Under ellipsis the object can hang on an auxiliary that has become the head.

**Examples.** *She gave me a raise* — `obj(gave, raise)`, `iobj(gave, me)`. — *They elected him president* — `obj(him)`, `xcomp(president)`. — *\*a man honesty* (a noun with obj is an error).

**In UD.** `obj`/`iobj` have a head `VERB` (`AUX` only under ellipsis) or `ADJ` *worth/like/unlike*; a head has at most one `obj`.

**Sources.** Reed & Kellogg, Higher Lessons, Lesson 28 (Object complement: "names that which receives the act"); Brown 1851, Rule V ("object of an active-transitive verb"); https://universaldependencies.org/en/dep/obj.html; https://universaldependencies.org/en/specific-syntax.html, Core arguments ("exclusive to verbal predicates and … worth, like, unlike"); UD 2.18 validator, `too-many-objects`; Poutsma 1923, *The Infinitive…*, §81 (worth with an object; vol. 2, p. 97).

```rule
rule: en.verbal.single-object
what: a predicate has at most one direct object
match: h[]; o[rel=obj, head=h]
require: none p[rel=obj, head=h, after=o]
severity: error
source: UD 2.18 validator too-many-objects
```

```rule
rule: en.verbal.object-head-predicate
what: the head of obj/iobj is a verb, an adjective or (under ellipsis) an auxiliary
match: h[]; o[rel=obj|iobj, head=h]
require: h[upos=VERB|AUX|ADJ]
severity: error
source: https://universaldependencies.org/en/specific-syntax.html, Core arguments
```

```rule
rule: en.verbal.object-head-adjective
what: among adjectives only worth, like, unlike take a direct object
match: h[upos=ADJ]; o[rel=obj|iobj, head=h]
require: h[lemma=worth|like|unlike]
severity: warn
source: https://universaldependencies.org/en/specific-syntax.html (after Huddleston & Pullum 2001); Poutsma 1923 Infinitive §81
```
