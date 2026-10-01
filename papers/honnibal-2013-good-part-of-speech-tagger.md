# A Good Part-of-Speech Tagger in about 200 Lines of Python

**Authors:** Matthew Honnibal · **Year:** 2013 · **Venue:** blog post (technical article, not peer-reviewed)
**Link:** https://explosion.ai/blog/part-of-speech-pos-tagger-in-python (republication of the original 2013 blog post)
**License of the paper:** unknown

## Summary
This article is a practical walkthrough of how to build a fast, accurate part-of-speech tagger with very little code. It argues that a greedy, left-to-right averaged perceptron with a well-chosen set of features gets close to state-of-the-art accuracy, while being much simpler and faster than CRFs or beam-search taggers. Each word is tagged once, using features of the current word, its neighbours, its suffixes and prefixes, and the tags already assigned to the previous words. To work around the train/test mismatch for previous-tag features, training uses the model's own predictions rather than gold tags. The article also stresses small engineering choices that matter in practice, such as a tag dictionary for frequent unambiguous words, word normalisation, and weight averaging. The overall message is that a simple, well-understood linear model is often a better engineering choice than a more complex one.

## How Mova uses it
- `ai/en/src/ptag.rs` — the Penn Treebank tagger follows this recipe: an averaged perceptron (Collins 2002) with a greedy left-to-right pass, as described in the article.
- Features are integers (hashed feature parts) with per-tag weights; neighbouring words, prefixes/suffixes, word shape and a "previous tag + word" conjunction are used, close to the article's feature set.
- Adapted: instead of a separate tag dictionary shortcut, the built-in lexicon's set of possible tags for a word form is itself a feature (a tag mask), so the model learns how far to trust the lexicon.
- Extended beyond the article: for unknown words a TnT-style prior P(tag | suffix, lexicon) from `ai/en/src/tag.rs` is added to the perceptron score, and a beam of 8 replaces the purely greedy pass in the final configuration.

## Effectiveness in Mova
Measured on UD English EWT test (from `ai/en/src/ptag.rs` and the `en` README):
- TnT baseline: 93.12%;
- greedy averaged perceptron (this recipe): 93.74%;
- plus TnT prior for unknown words: 94.13%;
- plus neighbour word shape and beam 8: 94.35% (PUD 94.04%, GUM 93.82%, vs TnT 93.12 / 92.84 on those).
The dependency parser trained on these tags improved from LAS 77.27 to 79.00. Two variants did not help: a separate feature per lexicon-mask tag (unknown words dropped from 71% to 65%) and training rare words without word-identity features (±0.1).

---

👨‍🔬💥
