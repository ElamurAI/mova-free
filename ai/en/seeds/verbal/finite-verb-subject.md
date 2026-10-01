# A finite verb has its own subject

**Gist.** In English, unlike Ukrainian, a finite verb almost always has an overt subject: *I came, I saw, I conquered*. Even impersonal actions take formal *it*/*there*: *It is raining*, *There is a ghost*. Subjectless are: the imperative (*Go!*), the second of coordinated predicates (*He came and **saw***), and colloquial initial omission (*Hope you're well*, *Looks like rain*).

**Conditions and exceptions.** In coordinated predicates the shared subject attaches only to the first (*He came and saw* — `nsubj` on *came*); the second `conj` has no subject. For the structure of coordination (head — the first conjunct, `cc` — on the following one) see `errors/coordination-structure.md`. In relative clauses the subject is the relative pronoun (*the man **who** came*), and in free relatives (*What irritates me…*) the relative word is the head, and no subject is visible in the clause.

**Examples.** *It reminds me of Vietnam.* — *Seems as if there is a clear distinction* (colloquial subject omission). — *He was tired and went home* (second predicate without nsubj — normal).

**In UD.** A `VERB` with `Mood=Ind` in the role `root`, `ccomp`, `advcl`, `parataxis` usually has `nsubj`/`nsubj:pass`/`csubj`/`csubj:pass` or `expl`. Absence means colloquial omission or a lost subject (attached elsewhere).

**Sources.** Brown 1851, Rule XIV, Note VIII ("should have a separate nominative expressed", with exceptions for coordinated ones); Reed & Kellogg, Higher Lessons, Lesson 20 (compound predicate), Lesson 57 (Contraction of sentences); https://universaldependencies.org/en/dep/expl.html, https://universaldependencies.org/en/specific-syntax.html (Coordination).

```rule
rule: en.verbal.finite-has-subject
what: a finite indicative verb in a main or subordinate role has a subject or expl; except colloquial omission at the start of the sentence (Hope you're well), fixed thank/hope and clauses with as/than (as follows)
match: v[upos=VERB, feats.Mood=Ind, rel=root|ccomp|advcl]
require: exists s[rel~nsubj|csubj|expl, head=v]
unless: none c[before=v, upos!=PUNCT|CCONJ|ADV|INTJ]; v[lemma=thank|hope]; exists m[rel=mark, head=v, lemma=as|than]
severity: warn
source: Brown 1851 Rule XIV, Note VIII; exceptions — Jespersen 1924, Philosophy of Grammar, pp. 142, 310 (prosiopesis: Thank you, Hope I'm not boring you); Curme 1931 §5 b, d (Thank you, Hope to see you again; as follows, more than was expected)
```

```rule
rule: en.verbal.parataxis-has-subject
what: a finite verb in parataxis has its own subject or takes the subject of the preceding clause (asyndetic coordinated predicates: Staff is friendly, treat you as a friend)
match: h[]; v[upos=VERB, feats.Mood=Ind, rel=parataxis, head=h]
require: exists s[rel~nsubj|csubj|expl, head=v]
unless: exists s[rel~nsubj|csubj|expl, head=h, before=v]; none c[before=v, upos!=PUNCT|CCONJ|ADV|INTJ]; v[lemma=thank|hope]; exists m[rel=mark, head=v, lemma=as|than]
severity: warn
source: Brown 1851 Rule XIV, Note VIII (coordinated predicates); Reed & Kellogg, Higher Lessons, Lesson 57; Jespersen 1924, Philosophy of Grammar, p. 142; Curme 1931 §5 b, d
```
