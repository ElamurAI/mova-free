# Hedges: A Study in Meaning Criteria and the Logic of Fuzzy Concepts

**Authors:** George Lakoff · **Year:** 1973 · **Venue:** Journal of Philosophical Logic, vol. 2, no. 4
**Link:** unknown
**License of the paper:** unknown

## Summary
This paper introduced "hedges" as a linguistic term: words and phrases whose job is to make things more or less fuzzy, such as *sort of*, *kind of*, *technically*, *loosely speaking* or *par excellence*. Starting from the observation that category membership is a matter of degree (a robin is a more typical bird than a penguin), the author argues that natural-language concepts have fuzzy boundaries. He uses fuzzy-set ideas to model degrees of membership and shows that hedges operate on these degrees, for example by restricting a predicate to its central members or extending it to peripheral ones. He also argues that some hedges are sensitive to which properties of a concept are considered defining versus characteristic. The paper had a lasting influence on semantics, cognitive linguistics and, later, on computational work on hedging and uncertainty.

## How Mova uses it
- `ai/prag/data/cues.tsv` — the hedge entries in Mova's small hand-made cue list (*sort of*, *kind of*, *maybe*, *I think* and similar) take this paper, together with Hyland 1998, as their conceptual basis; the list itself is Mova's own.
- `ai/prag/src/feats.rs` — the cue becomes the `hedge` feature slot used by all three pragmatic classifiers in `ai/prag/src/model.rs`.
- Only the notion of a hedge and typical examples are used; the fuzzy-logic formalism of the paper is not implemented.

## Effectiveness in Mova
Not measured separately (the cue list is not ablated). For context, the `prag` README reports for the `hedge` field, five-fold cross-validation on 2,986 sentences, accuracy / macro-F1 %: majority baseline 97.3 / 49.3, perceptron 98.1 / 76.3, IGTree 98.0 / 75.5, k-NN 97.8 / 73.4.

---

👨‍🔬💥
