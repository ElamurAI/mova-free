# algorithm-suffix_automaton

## descriptant

### Combiner and epoch memory

Prospective prediction: deterministic exhaustive agreement, boundary correctness and a compiled semantic mutant rejected. Initial scope deliberately excludes throughput and downstream language quality. Preserve observations when later epochs revise these models. Full semantic AST, macro expansion, executable approximate substitutes and universal proof remain absent.

## model

# suffix_automaton mod.rs model

## Forward

contains(q) iff q is a contiguous substring of s, including empty. distinct_substrings is the cardinality of the set of nonempty substrings.

## Invariant

Root has longest=0 and no link. Every nonroot suffix link has strictly smaller longest. Transitions extend represented substrings by one byte. A state represents lengths (longest(link),longest]; clone splits this interval without changing the represented language.

## Cost

For n bytes and alphabet size sigma<=256, O(n log sigma) construction plus O(n sigma) clone-map copying (O(n) for fixed byte alphabet), O(n sigma) conservative storage. Query O(|q| log sigma), count O(n). No timings measured.

## nonbehavior

Type: behavior_complement. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Universe is (s,q,b,count). Exclude b disagreeing with interval occurrence (empty occurs), and counts unequal to the cardinality of nonempty interval values. State numbering and occurrence multiplicities are not observable.

## compatible_antecedents

Type: reverse_reconstruction. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

For a complete membership language L, compatible texts are exactly s with substring-set including empty equal to L. A realizable finite L has a unique longest word s, recovering the text. Count alone admits all texts with that count: ab and ba both give 3; a count does not recover symbols.

## link_partition

Type: structural_invariant. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Each nonroot state contributes longest(v)-longest(link(v)) distinct strings; contributions partition all nonempty substrings by end-position equivalence.

## empty_boundary

Type: boundary_exclusion. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Empty input accepts only the empty query and has count zero. Nonempty input has positive count; a query longer than the text cannot occur.

## reversal

Type: metamorphic_relation. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Reversal bijects nonempty substring values, preserving count; membership of q in s equals membership of reversed q in reversed s.

## injective_symbols

Type: metamorphic_relation. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Any injective byte relabeling preserves count and corresponding membership. Noninjective relabeling may collapse distinct words and reduce count.

## count_bounds

Type: output_constraint. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

For length n, n <= count <= n(n+1)/2 when n>0; constant words attain n. Arithmetic for bound and observation is mathematical or u128.

## projection_loss

Type: information_loss. Domain: Finite byte strings s and query q; count observation is u128; allocations must succeed.

Count equality is weaker than language equality: ab and ba have count 3 but differ on membership of ab. Full language includes the original text and is injective, unlike count.

## Limits and reference

Conceptual reference: Blumer et al., The smallest automaton recognizing the subwords of a text, Theoretical Computer Science 40 (1985), 31–55. Original implementation; no external code imported. Finite evidence is not a proof; partial syntax omits macro expansion and semantic resolution. No executable approximation profile is declared for these exact routines.

## evidence

### Evidence

Passed: two debug and two release executions of the 220-test library, including two new tests for this algorithm. Ordered successful test records are byte-identical across all four runs after removing timing and harness-order noise. Strict cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings and rustfmt --check on all six owned Rust files pass. One initial Clippy finding in the Huffman oracle was corrected to isolate_lowest_one before the final four-run verification.

Scope: 3,280 ternary strings through length 7; all four-byte-alphabet queries through length 4 and all present substrings against an independent BTreeSet of intervals. Explicit empty, banana, clone-heavy abcbc, UTF-8 bytes, 0/255 and a 10,000-byte constant text.

Negative control: Skipping the first nonroot state contribution (.skip(1) to .skip(2)) in a separate copy compiles with rustc --edition 2024 --test; tests::boundaries_and_projections rejects it with an assertion failure and exit 101. The production source is unchanged by mutation checks. Compile failure does not count as rejection.

Finite evidence is not a universal proof or throughput measurement. Exact routines have no approximate executable profile, and no downstream school gain is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Original std-only implementation and independent oracle. The combiner jointly relates raw Rust, source-bound partial syntax, forward target, ordered typed obernants, invariant, cost and measured assertions. Tests and implementation remain separate leaves; passing leaves do not certify integration wiring. Conceptual reference: Blumer et al., The smallest automaton recognizing the subwords of a text, Theoretical Computer Science 40 (1985), 31–55. Original implementation; no external code imported.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

Both Rust files have syntax projections generated twice by installed mova-modelcheck syntax and compared byte-for-byte; manifests bind exact source SHA-256. No macro expansion or semantic resolution is claimed. Logs, normalized records and Rust gate helper are under $MOVA_DATA/runs/octopus-wave5-strings-2026-10-30. Temporary lib.rs exports were used for local checks and restored before commit; parent registry/wiring validation belongs to integration.

### 2026-10-30 · prospective

#### Prediction recorded before final gates

2026-10-30. Predict suffix_automaton agrees with its independent oracle over the bounded domain; debug/release result records repeat exactly. Predict semantic mutant fails a real assertion. Cost prediction: For n bytes and alphabet size sigma<=256, O(n log sigma) construction plus O(n sigma) clone-map copying (O(n) for fixed byte alphabet), O(n sigma) conservative storage. Query O(|q| log sigma), count O(n). No timings measured.
