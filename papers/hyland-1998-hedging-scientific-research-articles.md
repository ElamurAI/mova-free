# Hedging in Scientific Research Articles

**Authors:** Ken Hyland · **Year:** 1998 · **Venue:** John Benjamins (Pragmatics & Beyond New Series), book
**Link:** unknown
**License of the paper:** unknown (commercially published book, not openly licensed)

## Summary
The book is a corpus-based study of hedging, the linguistic means by which writers soften or qualify their claims, in scientific research articles. It argues that hedges are not signs of vagueness or weakness but an important rhetorical resource: they mark the writer's degree of commitment, anticipate objections and show respect for the reader and the research community. The author analyses which forms realise hedging (modal verbs, epistemic lexical verbs such as *suggest* or *appear*, adverbs such as *perhaps* or *possibly*, adjectives and certain constructions) and how frequently they appear. He also proposes a functional model that distinguishes hedges oriented towards the content (accuracy of the claim) from hedges oriented towards the writer and reader. The work became a standard reference for hedge lexicons in later computational research on uncertainty and hedge detection.

## How Mova uses it
- `ai/prag/data/cues.tsv` — a small hand-made list of pragmatic cue words and phrases; the hedge entries (*maybe, perhaps, probably, possibly, apparently, I think, I suppose, I guess*…) are compiled with this book and Lakoff 1973 as the conceptual basis. The list is Mova's own and not copied from any lexicon.
- `ai/prag/src/feats.rs` — the cues become symbolic feature slots (`hedge`, plus others) shared by the three pragmatic classifiers (averaged perceptron, IGTree, k-NN) in `ai/prag/src/model.rs`.
- The classifiers predict sentence-level labels including a binary `hedge` field.

## Effectiveness in Mova
The cue list is not ablated separately, so the contribution of this source is not measured on its own. The `hedge` field as a whole (from the `prag` README), five-fold cross-validation on 2,986 sentences, accuracy / macro-F1 %: majority baseline 97.3 / 49.3, perceptron 98.1 / 76.3, IGTree 98.0 / 75.5, k-NN 97.8 / 73.4. On the held-out test (538 sentences): baseline 96.8 / 49.2, IGTree best at 98.5 / 84.2. Shuffled-label controls stay at baseline level.

---

👨‍🔬💥
