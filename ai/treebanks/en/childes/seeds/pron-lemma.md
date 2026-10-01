# CHILDES — pronoun lemmas depend on the subcorpus

**Gist.** Since 2.11 (docs#517) EWT has a rule for pronoun lemmas:
- oblique case → nominative: *me* → *I*, *them* → *they*;
- possessive → the possessive itself: *your* → *your*, *mine* → *my*.

CHILDES was merged from three sources, and each carries its own custom:
- Adam (S+24 + LP23) and Eve (LP21): *your* → *your*, but *me* → *me*;
- the other children (LP23): *your* → *you* — the old EWT rule, and *me* → *I*.

Merging without a converter gives the model two opposite "form → lemma" pairs.

**Conditions and exceptions.**
- *me*: → *I* 995, → *me* 256 (Adam 194, Eve 62).
- *your* PRON: → *your* 784 (Adam 682, Eve 102), → *you* 1093 (all others).
- *them* → *them* 223 versus → *they* 460; *his* → *he* 286 versus → *his* 214.
- *mine, yours, hers, ours, theirs* — the lemma is the form itself: *mine* 135, *yours* 81. In EWT *mine* → *my*, *yours* → *your*.
- *i* → lowercase *i* — 53 times, the rest → *I* (7955).

**Examples.** Adam: *give me* — *me*: lemma *me*. Laura: *give me* — *me*: lemma *I*.

**In UD.**

| rule | CHILDES | EWT |
|---|---:|---:|
| `tb.en.pron-lemma`: personal or possessive lemma not per EWT 2.11 | 1940 / 6594 | 9 / 5039 |

**Sources.** EWT README v2.11 («revised guidelines for English pronouns (lemmas, features)», docs#517); https://universaldependencies.org/en/pos/PRON.html; Yang et al. 2025, `2025.udw-1.6`, sec. 3 (three sources, each with its own guideline).
