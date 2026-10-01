# "The fact that…": a content clause on a noun is acl

**Gist.** Some nouns (*fact, idea, hope, belief, news, question, fear, doubt, rumor*) take a clause that spells out their content: *the fact that he left*, *the question whether we can do it*. This is not a relative clause: it has no gap, and *that* is a conjunction (it cannot be replaced by *which*). Curme calls it an "attributive substantive clause" in an appositive relation.
**Conditions and exceptions.** Curme: introduced by *that, whether*, after *fear* — *lest*, colloquially — *as*; it can be reduced to an infinitive (*the time to act*, *his plan to go*). Content clauses are mostly finite.
**Examples.** *the hope that he may recover*; *the rumor that she quit*; *no doubt that she was lovely*.
**In UD.** The head of the clause is `acl` to the noun (not `acl:relcl`, not `appos`, not `ccomp`); *that* is SCONJ with `mark`. `mark` *that* under `acl:relcl` is an error (EWT 2.18: 1 time). A finite `acl` without `mark` can also be legitimate (EWT: 12 of 61): omitted *that* in colloquial speech (*the fact those horses lost…*; Curme: the clause is "sometimes without such introduction"), *no matter what…*, a quotation used as a modifier (*a bang-your-head-against-the-wall moment*) — so there is no separate rule for a missing `mark`.
**Sources.** Curme 1931, Ch. XIII §23 I "Attributive Substantive Clause" (pp. 199–203); https://universaldependencies.org/en/dep/acl.html (content clauses, as in CGEL); Poutsma GLME vol. 2, Ch. XV §1 (substantive clauses; p. 106).

```rule
rule: en.nominal.relcl-no-mark-that
what: acl:relcl has no mark that/whether — such a clause is a content clause (acl)
match: v[rel=acl:relcl]
require: none m[rel=mark, lemma=that|whether, head=v]
severity: error
source: Curme 1931 §23 I; UD en acl (content clauses)
```
