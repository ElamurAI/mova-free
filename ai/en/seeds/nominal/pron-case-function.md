# Case of the personal pronoun and its role in the sentence

**Gist.** The nominative (*I, he, she, we, they*) is for the subject of a finite verb. The objective (*me, him, her, us, them*) is for an object, after a preposition, for the subject of an infinitive with *for*, and in "detached" positions not tied to a finite verb: *It's me*, *Me too*, *taller than me*.
**Conditions and exceptions.** Poutsma (XXXII §4, §8): colloquial speech often uses the objective instead of the nominative when the pronoun does not stand right by the finite verb; *Me and him went* is vernacular. The opposite skew is hypercorrection: *between you and I*. UD annotation records the form (case = form), so in such sentences the annotation is correct; the rule only suggests checking the relation.
**Examples.** ✓ *She saw me.*; ✓ *for him to go*; ~ *Me and her went* (vernacular); ~ *between you and I* (hypercorrection).
**In UD.** A pronoun with `Case=Nom` as `obj`, `iobj`, `obl`, `nmod` — not once in EWT 2.18. A pronoun with `Case=Acc` as subject of a finite verb — 3 times out of 4 842 such subjects (vernacular); another 45 are subjects of infinitives and participles, where the objective is correct.
**Sources.** Poutsma GLME vol. 4, Ch. XXXII §4–12 (pp. 29–41 = book pp. 709–721); Curme 1931 §8 IV "Case" (p. 61).

```rule
rule: en.nominal.nom-not-object
what: a pronoun in the nominative is never an object or a dependent of a preposition
match: p[upos=PRON, feats.Case=Nom]
require: not p[rel=obj|iobj]; not p[rel~obl]; not p[rel~nmod]
severity: warn
source: Poutsma GLME IV Ch. XXXII §4, §12
```

```rule
rule: en.nominal.finite-subject-nom
what: a personal or interrogative-relative pronoun (PRP, WP) with case that is the subject of a finite verb is nominative
match: v[feats.VerbForm=Fin]; p[upos=PRON, xpos=PRP|WP, feats.Case, rel~nsubj, head=v]
require: p[feats.Case=Nom]
severity: warn
source: Poutsma GLME IV Ch. XXXII §8; UD _en/feat/Case.md; P17-1074 (PRON)
```

Rule: `en.errors.finite-subject-nominative-auxcop` in `errors/subject-pronoun-case.md`.
