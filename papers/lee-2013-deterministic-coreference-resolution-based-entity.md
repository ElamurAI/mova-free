# Deterministic Coreference Resolution Based on Entity-Centric, Precision-Ranked Rules

**Authors:** Heeyoung Lee, Angel Chang, Yves Peirsman, Nathanael Chambers, Mihai Surdeanu, Dan Jurafsky · **Year:** 2013 · **Venue:** Computational Linguistics, 39(4)
**Link:** https://aclanthology.org/J13-4004/ (ACL Anthology J13-4004; DOI 10.1162/COLI_a_00152)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
This journal article is the full description of the Stanford multi-pass sieve coreference system, which won the CoNLL-2011 shared task. Instead of a learned pairwise model, a sequence of deterministic "sieves" is applied from the most precise (speaker identification, exact string match) to the least precise (pronoun resolution), so that high-confidence links are made first. The model is entity-centric: each mention can use attributes pooled over its whole cluster (number, gender, animacy, NER type), and unknown attribute values are compatible with anything. Only the first mention of a cluster is resolved, indefinite mentions are not linked except by exact match, and candidate antecedents are ordered by a Hobbs-like syntactic traversal. The paper analyses the contribution of each sieve and shows that this simple, modular and interpretable design is competitive with machine-learned systems across several corpora.

## How Mova uses it
- `ai/coref/src/sieve.rs` — overall architecture: an ordered list of sieves from most to least precise (speaker, exact, relaxed, appositive, predicate nominative, relative pronoun, acronym, reflexive, strict head match variants, proper head, pronoun), each link carrying the sieve id and its evidence.
- Entity-centric rules taken from the paper: only the first mention of a cluster is resolved; indefinites (a/an, quantified, bare plurals) are linked only on exact match (`ai/coref/src/mention.rs`); cluster features are the union of mention features, and unknown values match anything.
- Speaker constraints (§3.3.1): "I" of one voice is one person, the author of a quotation is the subject of the speech verb outside the quotes, and clusters with "I" of different voices never merge.
- Pronoun sieve window of at most 3 sentences, and a Hobbs-style candidate order taken from the paper's description (Hobbs 1978 itself is not in our library).
- Adapted to UD: features come only from UD FEATS plus small hand lists (no NER, no name-based gender guessing), and mentions are matched by head as in the CRAC shared tasks.

## Effectiveness in Mova
Measured on GUM test (UD 2.18, head match), from the development notes of `ai/coref`. CoNLL F1 of the default sieve (12 sieves): 73.04 with singletons / 60.31 without on gold trees, 67.90 / 54.03 on Mova's own trees. Baselines on gold trees: exact string match only 53.07 / 27.88; pronoun → nearest agreeing mention 57.73 / 32.67. Cumulative sieve contributions (CoNLL F1 without singletons, gold trees): speaker +21.88, exact +20.94, strict_a +5.18, pronoun +6.38. Dev decisions: Hobbs order vs pure recency gave pronoun precision 64.2% vs 55.4% (CoNLL 73.43 / 63.34 vs 72.23 / 61.33); a pronoun window of 1–4 sentences made no difference (±0.1), so 3 was kept as in Lee et al.; dropping indefinites from exact match was worse (−0.4 / −0.9), so the paper's rule was kept. For reference, the CRAC 2025 mBERT baseline on en_gum is 61.7 (settings differ, comparison is only indicative).

---

👨‍🔬💥
