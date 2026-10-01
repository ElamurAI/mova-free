# Mapping to Declarative Knowledge for Word Problem Solving

**Authors:** Subhro Roy, Dan Roth · **Year:** 2018 · **Venue:** Transactions of the Association for Computational Linguistics (TACL)
**Link:** https://aclanthology.org/Q18-1012/ (ACL Anthology Q18-1012; DOI 10.1162/tacl_a_00012)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
Instead of learning each arithmetic operation directly from text, this solver first picks a mathematical concept for every operation in the expression tree and then a specific declarative rule under that concept, which fixes the operation. Four concepts are covered: transfer of objects (verb classes HAVE, GET, GIVE, CONSTRUCT, DESTROY combined with coreference of participants), dimensional analysis (units and rate components), part–whole relations (hyponym, hypernym, siblings) and explicit math phrases ("more than", "times"). Rules operate on pairs of numbers, with a subtree represented by a heuristically chosen representative quantity. The choice of rule is treated as a latent variable, so no expensive rule-level annotation is needed; training uses latent structured SVM followed by a standard structured SVM. The authors also expose dataset biases (e.g. "give" appearing only with subtraction) and collect a minimally perturbed problem set to show that knowledge-based rules generalize better than purely lexical learning.

## How Mova uses it
- `ai/global/seeds/shortcuts/possession.md`: a first-level knowledge seed with verb classes `have`, `get`, `give`, `make`, `lose`, `move` (adapted from the HAVE/GET/GIVE/CONSTRUCT/DESTROY scheme, together with ARIS, Hosseini et al. 2014) and their links to increase/decrease of a possessor's quantity; "give" decreases for the giver and increases for the receiver, and the question decides which side is asked.
- `ai/math/src/steps.rs`: the verb class of each quantity is read from these compiled links and used as a non-lexical feature of the solver's transition model.
- The verb lists are wired in at build time as constants rather than learned latent rules; there is no latent-variable SVM.

## Effectiveness in Mova
Non-lexical features (verb classes, rates, binding to the question; together with the Unit Dependency Graph paper, Roy & Roth 2017): SVAMP 44.7% → 48.0% (improvement). The contribution of the verb classes alone is not measured separately.

---

👨‍🔬💥
