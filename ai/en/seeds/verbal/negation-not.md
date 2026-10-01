# Negation not / n't

**Gist.** Clauses are negated with *not* (*n't*), placed after the first auxiliary: *I have **not** seen*, *She will**n't** go*. If there is no auxiliary, *do* is added: *I do not know*. Infinitives and participles take *not* before them: *not to go*, *not knowing*. *Not* applies to the whole predicate, so in UD it attaches to the main verb (or the predicative), not to the auxiliary.

**Conditions and exceptions.** *Not* as the remnant of an elliptical clause (*If not, …*, *Why not?*, *I hope not*) becomes the head or an adverbial. *Not only … but also* — `advmod`/`cc:preconj`; *whether or not* — *not* on *or*. The split *to not attempt* (split infinitive) is also normal. Other negative words (*never, no, nobody*) are separate parts of speech with `Polarity=Neg`.

**Examples.** *Kennedy has not been killed* — `advmod(killed, not)`. — *Don't go* — `advmod(go, n't)`. — *If not, is there someone else?*

**In UD.** *not/n't*: UPOS `PART`, XPOS `RB`, `Polarity=Neg`, relation `advmod` to the predicate. The validator allows negation to hang on a function word, but English treebanks attach it to the predicate.

**Sources.** Brown 1851, Part II, Ch. VI, "IV. Form of Negation" (not — "after the first auxiliary", while the infinitive and participles "take the negative first"); Jespersen, MEG V, ch. XXIII "Negation", §23.1x (text-1, p. 438); https://universaldependencies.org/en/pos/PART.html (predicate negation); https://universaldependencies.org/en/feat/Polarity.html; UD 2.18 validator, check_functional_leaves (exception for negation).

```rule
rule: en.verbal.not-part-neg
what: not as PART — Polarity=Neg and role advmod (or the head of an ellipsis)
match: n[lemma=not, upos=PART]
require: n[feats.Polarity=Neg]; n[rel=advmod|fixed|cc|conj|root|parataxis|reparandum|goeswith|orphan|advcl|ccomp]
severity: warn
source: https://universaldependencies.org/en/pos/PART.html; Brown 1851 Part II Ch. VI, Form of Negation
```

```rule
rule: en.verbal.negation-on-predicate
what: negation attaches to the predicate, not to an auxiliary or copula
match: f[rel=aux|aux:pass|cop]; n[head=f, upos=PART, feats.Polarity=Neg]
require: not n[rel=advmod]
severity: warn
source: https://universaldependencies.org/en/pos/PART.html; EWT practice (not → predicate)
```
