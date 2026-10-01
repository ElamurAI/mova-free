# Do, have, be with an object — lexical verbs

**Gist.** *Have, do, be* can be both auxiliary and lexical. Lexical *have* means "possess", *do* — "make, perform", *be* — "exist" (*There are magicians*). The sign: an auxiliary has no object of its own; if *have/do* has an object (*I have a car*, *do the dishes*), it is `VERB`.

**Conditions and exceptions.** Under ellipsis the auxiliary becomes the head and can "inherit" a fronted object of the omitted verb: *how many we can [reach]*, *the things that we do [feel]* — then `AUX` with `obj` is possible. *Have to* (*I have to go*) is `VERB` with `xcomp`.

**Examples.** *I have eaten* (AUX). — *I have a dog* (VERB). — *Do your homework* (VERB). — *Mary didn't leave, John did* (AUX head, no object).

**In UD.** `AUX` with a dependent `obj`/`iobj` — suspicious: most likely it is `VERB`.

**Sources.** https://universaldependencies.org/en/pos/AUX_.html (Ambiguity with VERB: have, do); Brown 1851, Obs. 4 to the definition of auxiliary ("do, be, have, being also principal verbs"); https://universaldependencies.org/en/specific-syntax.html, VP ellipsis.

```rule
rule: en.verbal.aux-no-object
what: a word with UPOS AUX has no direct or indirect object (otherwise it is VERB or ellipsis)
match: h[upos=AUX]; o[rel=obj|iobj, head=h]
require: not h[upos=AUX]
severity: warn
source: https://universaldependencies.org/en/pos/AUX_.html; Brown 1851 Part II Ch. VI, Obs. 4
```
