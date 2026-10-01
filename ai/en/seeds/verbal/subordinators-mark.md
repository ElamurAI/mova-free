# Subordinating conjunctions — mark

**Gist.** A subordinate clause is introduced by a conjunction: for complement clauses — *that, whether, if*; for adverbial clauses — *because, although, though, if, unless, while, since, until, before, after, as, once, so that*. In UD they are all `mark`, attached to the predicate of the subordinate clause (not the main one). The infinitival *to* and a preposition introducing a clause belong here too (*on **whether** users are at risk*: *on* and *whether* are both `mark`).

**Conditions and exceptions.** `mark` is a function word: not a noun, pronoun or adjective. Relative pronouns (*who, which, that* in a relative clause) are not `mark` but clause members (`nsubj`, `obj`…). A leaf conjunction can only have an adverbial modifier (*just because*, *right after*), `fixed` (*so that*, *even though*), coordination. The head of `mark` is a clause: `advcl`, `ccomp`, `csubj`, `acl`, `xcomp`, `root`, `conj`, `parataxis`, not a noun object or subject. Overlap: `errors/function-words-leaves.md` (`en.errors.function-word-leaf`) is a prohibitive list; `en.verbal.mark-leaf` here is permissive, as in the UD validator.

**Examples.** *Forces engaged in fighting after insurgents attacked* — `mark(attacked, after)`. — *He says that you like to swim* — `mark(like, that)`. — *because of the rain* — *because* here is `case` (a preposition), not `mark`.

**In UD.** `mark` does not have UPOS `NOUN`, `PROPN`, `ADJ`, `PRON`, `DET`, `NUM`, `AUX`, `INTJ` (unless there is `ExtPos`); the children of `mark` are only `advmod`, `obl`, `fixed`, `goeswith`, `reparandum`, `conj`, `cc`, `punct` (and negation); `SCONJ` usually has the relation `mark` (or `fixed`).

**Sources.** https://universaldependencies.org/en/dep/mark.html; https://universaldependencies.org/en/specific-syntax.html, Complementizers, subordinating conjunctions and the infinitival marker; UD 2.18 validator, tests `rel-upos-mark`, `leaf-mark-case` (`udtools/level3.py`); Reed & Kellogg, Higher Lessons, Lessons 63–65 (adverb clause), 71–72 (noun clause), 100–107 (connectives); Brown 1851, Part II, Ch. IX "Of Conjunctions", Rule XXII.

```rule
rule: en.verbal.mark-upos
what: mark is a function word, not a noun, pronoun, adjective or auxiliary
match: m[rel=mark, !feats.ExtPos]
require: not m[upos=NOUN|PROPN|ADJ|PRON|DET|NUM|AUX|INTJ]
severity: error
source: UD 2.18 validator rel-upos-mark
```

```rule
rule: en.verbal.mark-leaf
what: mark has no dependents except allowed function ones
match: f[rel=mark]; d[head=f, !feats.Polarity]
require: d[rel~advmod|obl|goeswith|fixed|reparandum|conj|cc|punct]
severity: error
source: UD 2.18 validator leaf-mark-case
```

```rule
rule: en.verbal.mark-head-clausal
what: the head of mark is a clause, not a nominal clause member
match: h[]; m[rel=mark, head=h]
require: not h[rel=obj|iobj|nsubj|nsubj:pass|nmod|nmod:poss|amod|det|case|compound|nummod|cc|punct|aux|cop|mark|fixed|flat]
severity: warn
source: https://universaldependencies.org/en/dep/mark.html ("The mark is a dependent of the subordinate clause head")
```

```rule
rule: en.verbal.sconj-is-mark
what: a subordinating conjunction (SCONJ) usually has the relation mark
match: s[upos=SCONJ]
require: s[rel=mark|fixed|goeswith|reparandum|conj|root|flat|dep]
severity: warn
source: https://universaldependencies.org/en/pos/SCONJ.html; https://universaldependencies.org/en/dep/mark.html
```
