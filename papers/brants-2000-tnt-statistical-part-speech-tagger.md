# TnT – A Statistical Part-of-Speech Tagger

**Authors:** Thorsten Brants · **Year:** 2000 · **Venue:** Sixth Applied Natural Language Processing Conference (ANLP 2000)
**Link:** https://aclanthology.org/A00-1031/ (ACL Anthology A00-1031; DOI 10.3115/974147.974178)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
TnT ("Trigrams'n'Tags") shows that a carefully engineered second-order hidden Markov model is as accurate as more complex taggers of its time. Tag trigram probabilities are smoothed by linear interpolation of unigram, bigram and trigram estimates, with the interpolation weights set by deleted interpolation rather than tuned by hand. Unknown words are handled by a suffix model that estimates tag distributions from word endings of rare training words, with separate statistics for capitalised and lowercase words. Decoding is exact Viterbi search, with beam pruning for speed. The tagger is fast, language-independent and reaches state-of-the-art accuracy on English and German treebanks.

## How Mova uses it
- `ai/en/src/tag.rs` — a direct reimplementation: trigram Markov model over tag classes with a dense transition table, deleted-interpolation smoothing, a word-to-tags lexicon for known words, and suffix models (up to 10 characters, separately for upper- and lowercase words, trained on words seen at most 10 times). Exact Viterbi over tag pairs.
- `ai/en/src/ptag.rs` — the main tagger is now an averaged perceptron with a beam; TnT's lexicon and suffix model still supply prior tag classes for unknown words.
- TnT remains a baseline and a fast mode (e.g. greedy parsing with TnT tags, faster generation).

## Effectiveness in Mova
Measured in the development notes of `ai/en`: TnT alone scores 93,12% (EWT), 93,12% (PUD), 92,84% (GUM) tagging accuracy; the perceptron tagger that uses TnT priors for unknown words reaches 94,35 / 94,04 / 93,82%. Downstream, greedy parsing with TnT tags gives EWT UAS/LAS 82,3/77,3 versus 83,4/79,0 with the perceptron tags. TnT-based generation is about 4 times faster (~10 500 vs ~2 600 sentences/s) at slightly lower quality (54,6% vs 56,6% exact, chrF 92,4 vs 92,8).

---

👨‍🔬💥
