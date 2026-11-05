# parsecmp-main

## descriptant

### Descriptant

read_conllu ignores read errors, short/range/dotted rows, defaults malformed head to u32::MAX and lowercases relation. Score counts head matches, UD-only base LAS and gold-PUNCT exclusion. spans strips functional label suffixes, excludes root/preterminals and sorts half-open word spans. bracket_f1 counts multiset intersection. Main aligns by counts/order, requires two UD voters, ties smaller head, flags changed head at >=0.75 support; writes three reports sequentially. agree uses first system punctuation mask for all pairs.

## approximant

# Forward approximant

read_conllu ignores read errors, short/range/dotted rows, defaults malformed head to u32::MAX and lowercases relation. Score counts head matches, UD-only base LAS and gold-PUNCT exclusion. spans strips functional label suffixes, excludes root/preterminals and sorts half-open word spans. bracket_f1 counts multiset intersection. Main aligns by counts/order, requires two UD voters, ties smaller head, flags changed head at >=0.75 support; writes three reports sequentially. agree uses first system punctuation mask for all pairs.

## Composition

Child fragments model one narrow helper each; their correctness alone does not establish orchestration.

The containing source wires this scoped relation to its callers. Treeagree proposal→gate→commit and treebanks parse→match→collect→render are actual joins; parsecmp is an independent diagnostic path. The external group model distinguishes these tags.

## catalog

# Typed models for parsecmp-main

## behavior_complement

Non-UD systems never contribute LAS or majority votes. Span duplicates contribute at most minimum multiplicity. Token count alignment does not validate matching forms/ids or safe heads.

Operator: inner exclusion on admitted domain. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## reverse_reconstruction

Scores constrain numbers of matching positions but cannot recover trees; spans lose words, POS/preterminal labels and root wrapper; multiset F1 loses identities of missed spans.

Operator: compatible antecedent relation, retaining ambiguity. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## punctuation_mask

Score excludes by gold UPOS only; agree applies first parser mask even to later pair comparisons, so swapping parser order can change denominator.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## zero_denominator

pct(0,0) returns em dash; agree reports 0.00 with max(1) denominator; constituency empty pair F1 is zero.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## multiset_join

hit=sum_s min(count_a(s),count_b(s)); returned denominators preserve each full multiplicity.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## read_failure

Missing file becomes empty sentences, indistinguishable from empty file at read_conllu boundary; main can still fail on course read.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## tests

### Bounded evidence

- `wave33_tests::score_and_bracket_multiset_boundaries`
- `wave33_tests::conllu_fixture_filters_and_missing_file`

These names passed twice in debug and twice in release; the complete treebank/comparison slice has 25 target-qualified names and identical sorted outcomes in all four runs. Ten compiled behavior mutations (one per owned source) failed named runtime assertions; exact original/restored SHA-256 equality and positive reruns were checked. Zero assertion-only mutants, zero surviving exploratory mutants, zero production fixes. Fresh raw logs, numeric exits, commands and mutation hashes are external in the wave33 trees receipt set.

#### Scope and gaps

Only named finite assertions cover selected inputs; other branches and invalid-domain behavior remain source-derived. No corpus, network, downloads, annotation model, live teacher, full release build or site execution. The treebanks join is entirely in memory and parsecmp uses a tiny uniquely named temporary directory. Treeagree Registry::load uses the embedded UD registry. Tests are finite CPU observations, not universal semantic proof, title-match quality estimates or corpus accuracy.

## invariant-and-cost

### Invariant and cost

Read O(bytes); spans O(chars+spans log spans) memory O(chars+depth+spans), bracket intersection O((a+b)log distinct). Main pairwise comparisons O(system²*tokens), holds all outputs; no parser/model invocation. Numeric sums assume no overflow.

Non-UD systems never contribute LAS or majority votes. Span duplicates contribute at most minimum multiplicity. Token count alignment does not validate matching forms/ids or safe heads.

The complement and specialized catalog constrain preserved coordinates and effect boundaries. Empty/malformed domains are only guaranteed where explicitly admitted. No whole-program cost bound follows from a helper bound.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source reading identified conditional input alignment, information loss and failure order before tests. Existing production code is retained; added tests use actual helpers. Finite results update this source-bound model, not all parser/corpus semantics.

No executable percentage surrogate has a justified observation metric for these categorical/effectful APIs. Future profiles require metric, domain and real-reference evidence at caps 1/2/5/10%; no higher cap. Syntax is partial lexical Rust projection without macro expansion or semantic resolution.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

Source `tools/parsecmp/src/main.rs` at SHA-256 `8d5c1b3d4b13c46e7f0b31fad87b1197b29574d987a2ea1c89ed012961a9a9e7`. Selected actual-source tests in `parsecmp`:

### Undated · document

Subject: source SHA-256 `8d5c1b3d4b13c46e7f0b31fad87b1197b29574d987a2ea1c89ed012961a9a9e7`; manifest supplies exact source/byte scope. Domain: admitted states in forward model, nonoverflow arithmetic and successful allocations. Observation: one terminating invocation, except explicitly described file trace. Source-derived partial relations; named finite tests constrain selected cases only. Each entry holds arguments and environment fixed except its stated intervention.
