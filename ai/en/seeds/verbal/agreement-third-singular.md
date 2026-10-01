# Agreement: the -s form — only with a singular subject

**Gist.** A finite verb agrees with its subject in person and number. The third person singular form (*goes, has, is, does*; tag `VBZ`) and *was* are used only with a singular subject. The verb agrees with the head of the subject, not with its dependents: *The progress of his forces **was** impeded*, *The ship, with all her furniture, **was** destroyed*.

**Conditions and exceptions.** The singular is also taken by names with a plural form (*The United States **is***, *"Friends" **is** a show*), measures and sums (*Three hours isn't far*, *Six months' interest was due*), names of sciences (*Physics is*). In inversion (*From that flows all the tributaries*, *Attached is the files*) and in colloquial *there's* + plural, agreement is often broken in the text itself. So the rule only warns: it catches either a subject attachment error (the wrong word is marked as subject) or textual non-agreement.

**Examples.** *The car **is** red.* — *The list of items **is** long* (the subject is list, not items). — *\*The dogs barks* (text error).

**In UD.** A subject (`nsubj`, `nsubj:pass`) with `Number=Plur` does not go with a `VBZ` predicate or with `VBZ`/*was* as `cop`/`aux` — except clauses with `expl` (there's) and with an outer subject (`nsubj:outer` — then *is* belongs to the outer clause).

**Sources.** Brown 1851, Rule XIV "Finite Verbs" ("must agree with its subject… in person and number") and Note II ("The adjuncts of the nominative do not control its agreement"; ); Reed & Kellogg, Higher Lessons, Lesson 142 (Construction of Number and Person Forms); UD docs/changes.md` "Morphosyntactic Features" (with notional agreement the feature is taken from the word form); Santorini 1990 (PTB), VBZ.

```rule
rule: en.verbal.vbz-plural-subject
what: a VBZ predicate with a plural subject — suspicious (except there's)
match: v[xpos=VBZ]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PRON, feats.Number=Plur]
require: exists e[rel=expl, head=v]
unless: exists q[rel=nummod, head=s]
severity: warn
source: Brown 1851 Rule XIV, Note II; Santorini 1990, VBZ; exception: measures and sums with a numeral take the singular (Three hours isn't far; 5 kg per gun means): verbal/agreement-third-singular.md, verbal/agreement-notional.md
```

```rule
rule: en.verbal.vbz-aux-plural-subject
what: a VBZ copula or auxiliary with a plural subject — suspicious (except a predicate clause)
match: h[]; a[xpos=VBZ, rel=cop|aux|aux:pass, head=h]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PRON, feats.Number=Plur]
require: exists o[rel=nsubj:outer|csubj:outer, head=h]
unless: exists q[rel=nummod, head=s]
severity: warn
source: Brown 1851 Rule XIV; https://universaldependencies.org/en/dep/nsubj-outer.html; exception: measures and sums with a numeral take the singular (Three hours isn't far; 5 kg per gun means): verbal/agreement-third-singular.md, verbal/agreement-notional.md
```

```rule
rule: en.verbal.was-plural-subject
what: was with a plural subject — suspicious
match: h[]; a[form=was, rel=cop|aux|aux:pass, head=h]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PRON, feats.Number=Plur]
require: exists o[rel=nsubj:outer|csubj:outer, head=h]
unless: exists q[rel=nummod, head=s]
severity: warn
source: Brown 1851 Rule XIV; exception: measures and sums with a numeral take the singular (Three hours isn't far; 5 kg per gun means): verbal/agreement-third-singular.md, verbal/agreement-notional.md
```
