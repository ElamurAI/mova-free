# Collocational bootstrapping: A hypothesis about the learning of subject-verb agreement in humans and neural networks

**Authors:** Claire Hobbs, R. Thomas McCoy · **Year:** 2026 · **Venue:** Proceedings of the 30th Conference on Computational Natural Language Learning (CoNLL 2026)
**Link:** https://aclanthology.org/2026.conll-main.7/ (ACL Anthology 2026.conll-main.7; DOI 10.18653/v1/2026.conll-main.7)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper asks how statistical regularities in linguistic input could help a learner acquire syntax. It proposes "collocational bootstrapping": which words tend to occur together hints at which words are syntactically related. The test case is English subject-verb number agreement. In most natural sentences the correct rule (agree with the subject) and a wrong rule (agree with the nearest preceding noun) give the same answer, so the input is ambiguous between them. The authors train neural language models on synthetic sentences that cannot tell the two rules apart. The subject paired with each verb is drawn from a Zipfian distribution whose exponent controls how predictable the pairing is. Evaluation uses sentences where the two rules disagree. Models generalise correctly only when the variability is moderate (exponent around 1.4), and child-directed speech in CHILDES has almost exactly that value (about 1.43). The conclusion is that the frequency structure of real language may itself push both children and models toward the correct syntactic generalisation.

## How Mova uses it
- `ai/en/seeds/errors/singular-subject-plural-verb.md`: cited in the seed for the warning rules `en.errors.singular-subject-vbp` and `en.errors.singular-subject-vbp-auxcop` (a singular noun subject before a plural-present VBP verb, copula or auxiliary). The paper's "agree with the subject" versus "agree with the nearest noun" contrast frames why these rules key on the `nsubj` relation in the tree and not on the closest noun.
- The rules themselves come from error-annotation schemes and grammars (VERB:SVA, Fowler, Poutsma, Curme); this paper is supporting background.

## Effectiveness in Mova
Background reading, one supporting citation in one seed. Not measured separately. For reference, the seed reports the rules' behaviour on gold data: EWT 2.18, verb 54 checks / 22 hits, copula or auxiliary 129/37, mostly real text errors, with the rest quantity and collective nouns.

---

👨‍🔬💥
