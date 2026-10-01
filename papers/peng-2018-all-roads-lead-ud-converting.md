# All Roads Lead to UD: Converting Stanford and Penn Parses to English Universal Dependencies with Multilayer Annotations

**Authors:** Siyao Peng, Amir Zeldes · **Year:** 2018 · **Venue:** Proceedings of the Joint Workshop on Linguistic Annotation, Multiword Expressions and Constructions (LAW-MWE-CxG-2018)
**Link:** https://aclanthology.org/W18-4918/ (ACL Anthology W18-4918)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper examines how to move an existing English treebank from older annotation schemes to UD 2.2, using the multilayer GUM corpus (eight genres, with entity, coreference, information-status and RST discourse layers). Two sources are compared: gold Stanford Typed Dependencies and Penn-style constituency trees. The Stanford-to-UD conversion is rule-based (in the DepEdit tool) and is enriched with information from the other annotation layers, such as entity types, to make distinctions like compound versus flat or to identify dislocated elements. Using syntax alone the rules err on roughly 1.5% of tokens; with the extra layers the error rate drops to about 0.4%. Converting constituency trees through CoreNLP yields about 10% errors even from gold trees, mainly because phrase functions are underspecified. The work shows that multilayer corpora make it cheap to keep treebanks in step with new UD guideline versions, and it produced a new open English UD resource.

## How Mova uses it
- `ai/en/seeds/errors/coordination-structure.md` — cited for the observation that moving `cc` onto the following conjunct is the main source of non-projectivity in GUM after conversion; the seed's rules check that `conj` points rightwards from the first conjunct and that `cc` precedes its head.
- `ai/en/seeds/errors/gapping-orphan.md` — cited for the point that `orphan` cannot be reliably derived by conversion; the rule requires the head of `orphan` to be a promoted remnant (conj, root, advcl or parataxis).
- `ai/en/seeds/errors/obl-on-nominal.md` — cited for obl/nmod confusion being a leading cause of constituency-conversion errors; the rule flags `obl` under a noun or pronoun without a copula.

## Effectiveness in Mova
Not measured separately. The paper is background evidence for three annotation-error rules. The rules are validated on gold treebanks (for example, the obl-on-nominal check fires 243 times on EWT 2.18 with 15 violations, mostly letter headers; the orphan check has 0 violations out of 29 in EWT), but these counts measure the rules, not the paper's contribution.

---

👨‍🔬💥
