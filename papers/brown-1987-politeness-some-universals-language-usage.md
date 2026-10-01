# Politeness: Some Universals in Language Usage

**Authors:** Penelope Brown, Stephen C. Levinson · **Year:** 1987 · **Venue:** Cambridge University Press (Studies in Interactional Sociolinguistics 4)
**Link:** https://doi.org/10.1017/CBO9780511813085 (DOI 10.1017/CBO9780511813085)
**License of the paper:** unknown (publisher copyright)

## Summary
The book develops a general theory of politeness built on the notion of "face": a person's wish to be unimpeded (negative face) and to be approved of (positive face). Many speech acts, such as requests, criticism or disagreement, threaten face, and speakers choose strategies to soften them: doing the act baldly, using positive politeness (solidarity, compliments), negative politeness (hedging, apologising, deference), going off record (hints), or not doing the act at all. The weight of a face-threatening act is modelled as a function of social distance, relative power and the imposition of the act. The authors support the theory with detailed linguistic evidence from English, Tzeltal and Tamil and argue that these strategies are cross-culturally universal. It is a standard reference in pragmatics and in computational work on politeness.

## How Mova uses it
- `ai/prag/data/cues.tsv` — a small hand-written cue list for the pragmatics crate; politeness markers (e.g. "please", apology and thanks cues) are grounded in this book, alongside Lakoff 1973 and Hyland 1998 for hedges and Searle for illocutionary force indicators.
- `ai/prag/src/feats.rs` — the cues become features (`please`, `hedge`, act cues such as thank, apol, greet, offer) shared by the perceptron, IGTree and k-NN classifiers of sentence form, speech act, indirectness and hedging.
- The list is Mova's own and small; the book supplies the theoretical categories, not a lexicon.

## Effectiveness in Mova
One of four theoretical sources for a feature list; the politeness cues are not measured separately. The overall pragmatic classifiers using these features are reported in the development notes of `ai/prag` (e.g. averaged perceptron, speech act 68,8% accuracy vs 33,3% majority baseline on the held-out silver test).

---

👨‍🔬💥
