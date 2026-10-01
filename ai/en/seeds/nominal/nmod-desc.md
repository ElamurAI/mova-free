# Title or description with a name: nmod:desc; with an article — appos

**Gist.** When a "bare" description stands before a name — a position, title or role without an article (*President Obama, spokesman John Smith, actor Martin Sheen*) — it is a descriptor. When the description has an article, possessive or numeral (*my friend Nick, the poet Burns*), it is a full apposition. Poutsma (vol. 1, IV §6 b) describes a similar type *the planet Mars, the man Moses*: the first word alone does not give the full meaning.
**Conditions and exceptions.** A descriptor can be dropped without harming the grammar. Suffixes *Jr., Inc., LLC, Corp.* after a name are also descriptors (to the right).
**Examples.** *Dr. Smith* → `nmod:desc(Smith, Dr.)`; *my friend Nick* → `appos(friend, Nick)`; *Apple Inc.* → `nmod:desc(Apple, Inc.)`.
**In UD.** `nmod:desc` — without `det`, `nmod:poss`, `nummod` (EWT 2.18: 437 of 439); the head is PROPN (439 of 439). A description with a determiner before a name is `appos` from the description to the name (EWT: 58 of 73 such *appos* have `det`/`nmod:poss`).
**Sources.** https://universaldependencies.org/en/dep/nmod-desc.html; Poutsma GLME vol. 1, Ch. IV §6 b (p. 291); Curme 1931 §10 III "Close apposition" (p. 91).

```rule
rule: en.nominal.desc-bare
what: the descriptor nmod:desc is a "bare" noun without determiner, possessive or numeral
match: t[rel=nmod:desc]
require: none c[rel~det, head=t]; none c[rel=nmod:poss, head=t]; none c[rel=nummod, head=t]
severity: error
source: UD en nmod:desc
```

```rule
rule: en.nominal.desc-head-propn
what: nmod:desc depends on a proper name
match: h[]; t[rel=nmod:desc, head=h]
require: h[upos=PROPN]
severity: warn
source: UD en nmod:desc
```
