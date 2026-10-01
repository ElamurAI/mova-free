# Auxiliaries attach flat and have no dependents of their own

**Gist.** Several auxiliaries in one predicate (*might have been lying*) do not form a chain "one subordinate to another": each attaches directly to the main (content) verb. The auxiliary itself is a "leaf" of the tree: it has no dependents of its own.

**Conditions and exceptions.** Allowed dependents of an auxiliary: `fixed`, `goeswith`, `reparandum`, `conj` and `cc` (when auxiliaries are coordinated: *We can and will get there*), `punct`. UD generally allows negation (*not*, *n't*) on any function word, but in English EWT it attaches to the predicate (see `negation-not.md`). When the main verb is omitted (ellipsis), the auxiliary becomes the head and can have a subject — then it is no longer `aux` (see `vp-ellipsis.md`). Overlap: `errors/function-words-leaves.md` (`en.errors.aux-cop-leaf`) forbids a list of relations; the rule here is a permissive list, as in the UD validator.

**Examples.** *By that time, the story **would have been** revealed.* — all three (would, have, been) depend on *revealed*.

**In UD.** `aux(lying, might)`, `aux(lying, have)`, `aux(lying, been)` — not `aux(have, might)`. A node with `aux`/`aux:pass`/`cop` has no children except those listed.

**Sources.** https://universaldependencies.org/en/specific-syntax.html ("Auxiliaries", "Function words attaching to predicates"); UD 2.18 validator, test `leaf-aux-cop` (`udtools/level3.py`, check_functional_leaves); https://universaldependencies.org/en/dep/aux_.html.

```rule
rule: en.verbal.aux-leaf
what: an auxiliary or copula has no dependents except function-word exceptions (negation — separately)
match: f[rel=aux|aux:pass|cop]; d[head=f, !feats.Polarity]
require: d[rel~goeswith|fixed|reparandum|conj|cc|punct]
severity: error
source: UD 2.18 validator leaf-aux-cop; https://universaldependencies.org/en/specific-syntax.html, Auxiliaries
```
