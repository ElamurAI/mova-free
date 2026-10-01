# Learning to Solve Arithmetic Word Problems with Verb Categorization

**Authors:** Mohammad Javad Hosseini, Hannaneh Hajishirzi, Oren Etzioni, Nate Kushman · **Year:** 2014 · **Venue:** Proceedings of the 2014 Conference on Empirical Methods in Natural Language Processing (EMNLP)
**Link:** https://aclanthology.org/D14-1058/ (ACL Anthology D14-1058; DOI 10.3115/v1/D14-1058)
**License of the paper:** CC BY-NC-SA 3.0 — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper presents ARIS, an early system for addition and subtraction word problems that does not depend on equation templates. The text is split into fragments, each with a verb, an entity (the noun a number belongs to), a quantity, attributes (modifying adjectives) and one or two containers (an owner or a location). The world state is the amount of each entity in each container, and every sentence is a transition between states. The key learned step classifies each verb into one of seven categories: observation, positive and negative change, positive and negative transfer between two containers, construction and destruction. An SVM predicts, for each (verb, container) pair, whether the quantity goes up or down. The system then updates the states and drops irrelevant sentences by matching entities and attributes against the question. It fills missing containers under a circumscription assumption and builds an equation with one unknown from two adjacent states. The entity-container-transition model is the paper's lasting idea: each state change is an elementary, interpretable step.

## How Mova uses it
- `ai/math/src/steps.rs`: the math solver's main non-lexical features are verb classes (have, get, give, make, lose, move, compare) in the spirit of ARIS's verb categorisation, extended with Roy & Roth 2018. The model sees "how the quantity changes" instead of a bag of words, and question-relevance features follow ARIS and Patel 2021.
- `ai/global/seeds/shortcuts/possession.md`: the seed on possession and quantity change ("give" decreases the giver and increases the receiver; the question decides which one is asked) is the compiled first-level source of these verb classes. Its `verbs` block lists the members of each class, and the solver, the story world and pragmatics all share it.
- `ai/math/src/qworld.rs`: the solver writes the quantitative world of a problem itself, deterministically and ARIS-style. Each number becomes a world command (owner = subject, thing = unit, action from the verb class, rate "X per Y"), each question becomes a query, and an exact rational-number core does the arithmetic. If the world is unknown, it answers "don't know" instead of guessing.

## Effectiveness in Mova
Measured only together with other ideas or as whole modules:
- Replacing words and lemmas with non-lexical features (verb classes after ARIS and Roy & Roth 2018, rate features after UnitDep 2017, subject match after Patel 2021) raised SVAMP from 44.7% to 48.0%. The project notes record this combined gain; the share due to ARIS alone is not separated.
- The ARIS-like quantitative world `math::qworld` v1 answered 31 of 300 SVAMP problems correctly, with 102 "don't know". It was too weak as a third opinion. Trained on verified scripts it reached 34/300, then 97/300 with a learned query template, then 103/300 with question-driven cells for differences.

---

👨‍🔬💥
