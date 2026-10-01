# Open complement (xcomp): no subject of its own

**Gist.** Some verbs take a complement whose "subject" is not its own but borrowed from the main clause: *Sue asked George **to respond*** (George responds), *You like **to swim*** (you swim), *I consider him **honest***, *She looks **beautiful***, *He painted the barn **red***. This is `xcomp`: it is always non-finite and has no subject of its own.

**Conditions and exceptions.** This covers: raising verbs (*seem, appear, tend*), control (*want, try, persuade*), copula-like verbs (*become, remain, look*), resultatives (*paint red, make them martyrs*). When the subordinate clause has its own finite subject, it is `ccomp`. If an `xcomp` has its own `nsubj`, it is an error; the exception is raising with *there* (*there seems to be a problem*: the notional subject *problem* attaches to *be*).

**Examples.** *He says that you like to swim* — `xcomp(like, swim)`. — *The cat seems to be in pain.* — *I became very upset.*

**In UD.** `xcomp` without `nsubj`/`nsubj:pass`/`csubj`/`csubj:pass`; if the `xcomp` is a verb, it is not `VBZ`, `VBP`, `VBD`, `MD`.

**Sources.** https://universaldependencies.org/en/dep/xcomp.html ("always non-finite"); https://universaldependencies.org/en/specific-syntax.html, Functional control and Resultatives; https://universaldependencies.org/u/dep/xcomp.html; Jespersen, MEG V, ch. XVIII "Subject + Infinitive as Object of Main Verb", §18.5x (text-1, p. 304); PropBank 3.1 `seem.xml` (seem.01 without ARG0), `persuade.xml` (persuade.01: ARG2 — "impelled action").

```rule
rule: en.verbal.xcomp-no-subject
what: xcomp has no subject of its own
match: x[rel=xcomp]
require: none s[rel=nsubj|nsubj:pass|csubj|csubj:pass, head=x]
severity: warn
source: https://universaldependencies.org/en/dep/xcomp.html; UD 2.18 validator (check_single_subject, comment on xcomp)
```

```rule
rule: en.verbal.xcomp-nonfinite
what: a verbal xcomp is non-finite
match: x[rel=xcomp, upos=VERB|AUX]
require: not x[xpos=VBZ|VBP|VBD|MD]
severity: error
source: https://universaldependencies.org/en/dep/xcomp.html; https://universaldependencies.org/en/specific-syntax.html, Clausal core arguments
```
