# Outer subject with a predicate clause

**Gist.** Sometimes the predicate of a copular sentence is a whole subordinate clause: *The problem is **that this has never been tried***, *The important thing is **to keep calm***. Then the head of the whole is the predicate of the inner clause (*tried*, *keep*), the copula *is* attaches to it, and the subject of the outer clause is marked as "outer".

**Conditions and exceptions.** The subtype `:outer` is only for such predicate clauses. If the predicate is a noun or adjective (*The title is Green Eggs and Ham*, *That book is very good*), the subject is a plain `nsubj`. In pseudoclefts (*What John did was to play tennis*) the subject *What* is also `nsubj:outer`.

**Examples.** *The problem is that this has never been tried.* — *To hike in the mountains is to experience the best of nature* (csubj:outer). — *It was because Bill is honest.*

**In UD.** Only heads that have a `cop` have `nsubj:outer`/`csubj:outer`.

**Sources.** https://universaldependencies.org/en/dep/nsubj-outer.html, https://universaldependencies.org/en/dep/csubj-outer.html, https://universaldependencies.org/en/dep/cop.html (predicate clause); https://universaldependencies.org/en/dep/acl-relcl.html, Pseudoclefts; UD 2.18 validator, is_inner_subject (outer is not counted).

```rule
rule: en.verbal.outer-subject-needs-cop
what: an outer subject occurs only with a predicate clause with a copula
match: h[]; o[rel=nsubj:outer|csubj:outer, head=h]
require: exists c[rel=cop, head=h]
severity: warn
source: https://universaldependencies.org/en/dep/nsubj-outer.html; https://universaldependencies.org/en/dep/cop.html
```
