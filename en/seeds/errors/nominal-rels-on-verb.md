# Nominal relations do not come off a verb

**Gist.** Some UD relations by definition belong to the noun phrase: `amod` is an adjective on a noun, `acl` a clause on a noun, `case` a preposition inside a noun phrase. If the head of such a relation is a verb, the error is either in the relation or in the part of speech of the head. The most frequent cases:
- a preposition before a gerund or clause is `mark`, not `case` (*by using it*);
- a clause on a verb is `advcl`, not `acl`;
- an adjective on a verb is `xcomp` (*got rich*) or `advmod`, not `amod`.

**Conditions and exceptions.**
- EWT has the VERB *following* in *the following* with `det` (20 cases) — an established peculiarity of the gold.
- A title of a work that is a sentence (*"What We've Lost", published by…*) gives `acl` from a VERB.

**Examples.**
- *by using it* → mark(using, by).
- *the man sitting there* → acl(man, sitting).
- *He left, crying* → advcl(left, crying).

**In UD.** The guideline for `mark`: if a preposition introduces a clause, it is `mark`, because "case" is inappropriate for a clause. `case` is "any preposition" in an extended nominal projection.

**Check against gold.** EWT 2.18: `acl` under VERB — 1, `amod` under VERB — 5, `case` under VERB — 3. ESLSpok: `case` under VERB — 14 (a treebank convention).

**Sources.** UD `_en/dep/case.md`, `_en/dep/mark.md` (the paragraph about a preposition before a clause), `_en/dep/acl.md`, `_en/dep/amod.md`; english-banks §2 (IN → ADP by `case`, → SCONJ by `mark`); `2025.law-1.14` (LLM annotation: tags better than UDPipe, dependencies worse).

```rule
rule: en.errors.acl-on-verb
what: acl under a verb — a clause on a verb should be advcl, acl is only for nouns
match: h[upos=VERB]; a[rel=acl, head=h]
require: not a[rel=acl]
severity: warn
source: UD _en/dep/acl.md, _en/dep/advcl.md
```

```rule
rule: en.errors.amod-on-verb
what: amod under a verb — an adjective on a verb is xcomp or advmod, not amod
match: h[upos=VERB]; a[rel=amod, head=h]
require: not a[rel=amod]
severity: warn
source: UD _en/dep/amod.md, _en/dep/cop.md (Bill got rich → xcomp)
```

```rule
rule: en.errors.case-on-verb
what: case under a verb — a preposition before a gerund or clause is mark
match: h[upos=VERB]; a[rel=case, head=h]
require: not a[rel=case]
severity: warn
source: UD _en/dep/mark.md, _en/dep/case.md
```
