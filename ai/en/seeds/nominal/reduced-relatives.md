# Reduced relative clauses: participle and infinitive on a noun

**Gist.** A relative clause is often reduced to a participle or infinitive (Curme: "abridgment"): *the man [who is] sitting there*, *a book [that was] written in 1900*, *a chance to win*, *a bagel to eat*. A participle with dependents stands after the noun; a lone one usually before it, like an adjective (*a sleeping child*).
**Conditions and exceptions.** Curme: a participle with noticeable verbal force stands after the noun (*a man shot*), as a property — before (*an unheard-of crime*). UD distinguishes infinitives: with a gap corresponding to the noun (*a bagel to eat* — eat the bagel) — `acl:relcl`; without such a gap (*a way to get my discount*) — `acl`.
**Examples.** *sites offering booking* → `acl(sites, offering)`; *a parakeet named Cookie* → `acl`; *a suggestion to make* → `acl:relcl`.
**In UD.** `acl` with `VerbForm=Part|Ger|Inf` stands to the right of the noun (EWT 2.18: `acl` on a noun or pronoun — to the right 1 794 of 1 806; 12 to the left — mostly quotation modifiers like *a bang-your-head-against-the-wall moment*). A participle before the noun is `amod` (VERB with `amod`, 814 times to the left).
**Sources.** Curme 1931, Ch. XIV §23 II 11 "Abridgment of Relative Clause" (pp. 236–238), §10 I 1 (p. 64); Poutsma GLME vol. 2, participle clauses §2 (p. 438); https://universaldependencies.org/en/dep/acl.html, `acl-relcl.md` (Infinitival Relatives).

```rule
rule: en.nominal.acl-after-head
what: a participial or infinitival acl clause stands after the noun; a lone participle before the noun is amod
match: n[upos=NOUN|PROPN|PRON]; v[rel=acl, head=n]
require: v[after=n]
severity: warn
source: Curme 1931 §10 I 1, §23 II 11; UD en acl
```
