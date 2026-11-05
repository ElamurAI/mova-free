# Question-word observations

## forward
Scope: qlemmas and cover. Concatenate main content, main subject, subordinate content and subordinate subject; drop stop words and byte length <2. qlemmas unions lemma expansions. cover counts each remaining occurrence whose expansions intersect sentence content-token forms/lemmas, divided by occurrence count (zero if empty).

## invariant
Coverage is in [0,1]. Repeating a word changes its multiplicity, not the lemma set. Lexical expansion belongs to ans and the compiled lexicon, not this fragment.

## cost
Expected O((T+W)L) hash work for T sentence tokens and W question occurrences with L expansion cost; string bytes and allocations remain relevant.

## behavior-complement
For fixed expansions, an empty retained question cannot yield positive coverage, and an absent sentence match cannot add a numerator hit. This says nothing about paraphrase adequacy.

## reverse-reconstruction
The lemma-set fiber contains every permutation and repetition yielding the same union. For coverage h/n, all ordered occurrence lists with h matching entries among n are compatible; original order and multiplicity are not recoverable from the scalar alone.

## boundary
Non-ASCII word length is measured in bytes; lexical equivalence is whatever ans::lemmas supplies. A set-based ratio is not an interchangeable implementation.

## composition
This observation feeds both state routing and span extraction; confidence and coverage are independent gates and cannot be multiplied as calibrated probabilities.

## checks
coverage_occurrences_and_set_loss tests 36 count pairs, stop words and subordinate/subject inclusion. lemma_skip_removes_forms_and_aliases checks the separate candidate-word skip helper outside this fragment's source range.

## epoch-memory

Source-derived occurrence/set distinction, finite tests; no full parser evaluation. Syntax projection is partial, semantic AST absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30: source-derived occurrence/set distinction, finite tests; no full parser evaluation. Syntax projection is partial, semantic AST absent.
