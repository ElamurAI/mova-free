# world-nerves-scope

## descriptant

### Concept normalization and manual sentence extraction

Define A(s)=concatenate lowercase expansions of alphanumeric chars of s. If A is empty, in STOP/NUMBERS/ASKING, or all ASCII digits, N(s)=None. Otherwise, if byte_len(A)>4 and A ends ies, replace ies by y; else if byte_len(A)>3 and A ends s but not ss/us/is, drop final s; else retain A. content_words(t)=set(filter_map(N, split t at non-alphanumeric chars)). sentences_of trims lines, toggles fenced-code state on leading backticks, resets open at fences/blank/headings, skips code, and appends a line to the prior sentence iff open; open becomes false for final .!?»")]: punctuation.

## approximant

### Concept normalization and manual sentence extraction

Define A(s)=concatenate lowercase expansions of alphanumeric chars of s. If A is empty, in STOP/NUMBERS/ASKING, or all ASCII digits, N(s)=None. Otherwise, if byte_len(A)>4 and A ends ies, replace ies by y; else if byte_len(A)>3 and A ends s but not ss/us/is, drop final s; else retain A. content_words(t)=set(filter_map(N, split t at non-alphanumeric chars)). sentences_of trims lines, toggles fenced-code state on leading backticks, resets open at fences/blank/headings, skips code, and appends a line to the prior sentence iff open; open becomes false for final .!?»")]: punctuation.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

Stop filtering precedes suffix reduction. norm removes punctuation within its argument, while content_words splits punctuation before normalization: norm(cat-dog)=catdog but content_words(cat-dog) contains cat,dog. Sentence extraction discards headings and fenced code; it joins wrapped prose with one space and does not tokenize multiple sentences on a single line.

Cost: norm is O(input chars plus lowercase output bytes) with fixed cue-table scans; content_words adds ordered-set insertion O(W log U). sentences_of is linear in line/input/output bytes under amortized String growth. No input cap or fallible-allocation interface exists.

## composition

This fragment supplies norm/content_words to the parent graph seed/build/ranking code and norm to ideas::words. It also supplies manual sentence units to graph construction. The parent three-step diffusion recurrence is outside this child, so its activation arithmetic is not copied here.

## models

## behavior_complement

Exclude Some for empty, 123, where or three. N(BERRIES!) is berry and N(glass) is glass. N(ones)=Some(one) while N(one)=None: the source does not reapply stop filtering after suffix reduction. sentences_of cannot emit the body of a properly toggled code fence, a heading line, or a standalone empty line.

## reverse_reconstruction

N^-1(berry) contains berry, berries, BERRIES!, and punctuation-inserted b.e.r.r.y because norm filters before suffix handling. N^-1(None) contains empty strings, pure punctuation, ASCII digit strings and all removed cue words. The sentence output [One wrapped line.] admits one input line or two wrapped lines joined by one space, with arbitrary ignored headings/fenced examples; original wrapping is unrecoverable.

## normalization_order

A(s) -> removal predicates -> suffix rule is ordered and lossy. These rules are surface morphology, not linguistic lemmatization; alphanumeric Unicode digits outside ASCII are not removed by the ASCII-number predicate alone.

## token_boundary_transport

content_words deduplicates normalized tokens using BTreeSet. Repeating apples or adding case variants does not create multiple seed concepts. norm on an entire punctuation-separated phrase is not interchangeable with content_words.

## manual_state_machine

State=(code,open,out). Fence toggles code and clears open; blank/heading clears open; code content is skipped. A prose line either extends last(out) or creates a new element, then trailing punctuation controls open.

## terminal_punctuation

A colon or closing bracket ends wrapping, just like sentence punctuation. A line ending a comma stays open. An unmatched opening fence suppresses the remaining file; this parser does not report a fence error.


## tests

nerves::wave28_dialogue::normalization_and_manual_boundaries checks BERRIES!, glass, empty/digits/where/three, wrapping, heading removal and code fencing. synthetic_spread_units_and_dedup consumes content_words through spread, but diffusion itself is outside this child scope. No claim that finite examples prove all Unicode/suffix behavior.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. This fragment supplies norm/content_words to the parent graph seed/build/ranking code and norm to ideas::words. It also supplies manual sentence units to graph construction. The parent three-step diffusion recurrence is outside this child, so its activation arithmetic is not copied here.
