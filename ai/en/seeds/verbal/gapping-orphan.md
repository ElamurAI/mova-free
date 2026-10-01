# Gapping: omitted predicate and orphan

**Gist.** In coordinated clauses the predicate of the second is often omitted, leaving two elements contrasted with the first: *Marie went to Paris and Miriam [went] to Prague*. A tree without the omitted word is built like this: one of the remnants (usually the subject) is promoted to the predicate's place, and the others attach to it with the special relation `orphan`.

**Conditions and exceptions.** `orphan` — only when there is no predicate at all. If an auxiliary remains, it is VP ellipsis (the head is the auxiliary, no `orphan` needed). With right-node raising (*John bought and ate an apple*) — ordinary coordination. The promoted remnant usually has the relation `conj` (more rarely `parataxis`, `advcl` in comparisons: *He buys companies like my mother [does] vegetables*).

**Examples.** *Marie went to Paris and Miriam to Prague* — `conj(went, Miriam)`, `orphan(Miriam, Prague)`. — *He's not against gays in the bedroom, just at the altar.*

**In UD.** The head of `orphan` has the relation `conj`, `parataxis`, `root`, `csubj`, `ccomp`, `advcl`, `acl` or `reparandum` (with subtypes).

**Sources.** https://universaldependencies.org/en/dep/orphan.html; https://universaldependencies.org/en/specific-syntax.html, Gapping / Stripping, Right-node raising; UD 2.18 validator, test `orphan-parent` (warning); Reed & Kellogg, Higher Lessons, Lesson 57 (Contraction: omitting the verb).

```rule
rule: en.verbal.orphan-parent
what: the head of orphan is a promoted remnant in the role conj/parataxis/advcl etc.
match: h[]; o[rel=orphan, head=h]
require: h[rel~conj|parataxis|root|csubj|ccomp|advcl|acl|reparandum]
severity: warn
source: UD 2.18 validator orphan-parent; https://universaldependencies.org/en/dep/orphan.html
```
