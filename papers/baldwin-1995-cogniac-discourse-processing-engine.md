# CogNIAC: A Discourse Processing Engine

**Authors:** Breck Baldwin · **Year:** 1995 · **Venue:** PhD thesis, University of Pennsylvania
**Link:** unknown
**License of the paper:** unknown

## Summary
This doctoral thesis describes CogNIAC, a pronoun and coreference resolution engine built around a small set of high-confidence rules applied in order. The philosophy is precision first: a pronoun is resolved only when a rule fires unambiguously, otherwise it is left unresolved, so that downstream applications get few but reliable links. The rules rely on limited knowledge and shallow linguistic resources, such as unique antecedents in the current and previous sentence, reflexives, and patterns in quoted speech. The work was later summarised in a shorter 1997 workshop paper ("CogNIAC: high precision coreference with limited knowledge and linguistic resources"). Further details of the thesis contents are not confirmed here.

## How Mova uses it
- `ai/coref/src/sieve.rs` (`find_quote_authors`) — determining who speaks in a quotation: in a sentence containing a quote, the author is the subject of a speech verb outside the quotation marks; speech verbs come from a closed hand-written list. A quote continuing into the next sentence without a new author keeps the same author.
- The rule is cited together with Lee et al. 2013 (§3.3.1) and feeds the `speaker` sieve: first-person pronouns of one voice are one person, "you" addressed to someone links to that person's earlier "I".

## Effectiveness in Mova
The quote-author rule is part of the `speaker` sieve, which in the development notes of `ai/coref` (GUM test, gold trees, cumulative contribution) makes 771 links at 90.7% precision and adds +21.88 CoNLL F1 without singletons (+21.99 with Mova's own trees). That figure covers the whole sieve (also built on Lee et al. 2013); the contribution of the quote-author rule alone is not measured separately.

---

👨‍🔬💥
