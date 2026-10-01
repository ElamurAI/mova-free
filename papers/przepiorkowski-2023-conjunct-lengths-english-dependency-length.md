# Conjunct Lengths in English, Dependency Length Minimization, and Dependency Structure of Coordination

**Authors:** Adam Przepiórkowski, Michał Woźniak · **Year:** 2023 · **Venue:** Proceedings of the 61st Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers)
**Link:** https://aclanthology.org/2023.acl-long.864/ (ACL Anthology 2023.acl-long.864; DOI 10.18653/v1/2023.acl-long.864)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The authors test whether, in English binary coordinations, the left conjunct tends to be shorter than the right one, and whether this depends on the position of the external governor. They use a version of the Penn Treebank with explicit coordination annotation, locate governors with their own heuristic rules, and measure conjunct length in characters, syllables and words. The left conjunct is shorter regardless of governor position, so the hypothesis that the shorter conjunct simply gravitates toward the governor is rejected. However, the preference grows with the length difference only when the governor is on the left or absent, not when it is on the right. The authors explain this pattern by dependency length minimisation and show that the explanation works only for symmetric dependency analyses of coordination (Prague, London style), not for analyses headed by the first conjunct (Stanford, Moscow style). They take this as an argument in favour of enhanced UD over the basic UD treatment of coordination.

## How Mova uses it
- `ai/en/seeds/errors/coordination-structure.md` — cited in the seed on UD coordination as a caveat: UD's structure is asymmetric (the first conjunct is the head), and this paper shows such asymmetric models fail to explain conjunct-length data.
- The seed's rules still enforce the basic UD convention (conj rightward from the first conjunct, cc before its conjunct), since that is what the treebanks use; the paper documents the theoretical cost of that choice.

## Effectiveness in Mova
Not measured separately. Background reading for one knowledge seed; it does not change any rule's behaviour.

---

👨‍🔬💥
