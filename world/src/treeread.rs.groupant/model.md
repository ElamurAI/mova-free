# world-treeread

## model

# Quantitative supplied-tree reading and ordered schemas

## forward
read normalizes number words, splits on .!? inclusively, obtains Words through frames::parse_words, then read_sentences consumes ordered (Words,question-bit) pairs. Each integral numeric surface in an admitted POS yields Q with counted noun, bounded ancestor verb, subject/last-name holder, rate/comparison/time and tags. Question sentences update Ask. answer passes read output and original text to answer_reading. The first applicable schema wins: comparison, part/whole, total, rate, each, left, unknown change; otherwise None. This path does not build or mutate World.

## invariant
For valid one-based heads and bounded arithmetic, quantity order follows sentence then token order; empty sentences still advance sent. Last name updates after each sentence, so pronouns resolve to the previously seen subject. in_q records the supplied question bit; it is not globally filtered from every schema. Each division requires positive divisor and exact divisibility. Left returns only if at least two eligible quantities and nonnegative result. Public parser behavior and malformed heads are outside these invariants.

## cost
For S sentences with n_i words and q quantities, kids scans all n_i words; verb climb costs O(n_i), subject scans at most four levels. Nested tag children can cost O(n_i^2) per quantity, yielding conservative O(sum n_i^3) reading work plus lexical lookup, normalization and parsing. Schema scans are O(q times tag-comparison cost), allocation O(total stored tag bytes). Unit is word visits/string bytes, not elapsed latency. No benchmark or asymptotic speedup claim.

## behavior-complement
In the supplied bounded arithmetic domain, each cannot return a quotient for zero divisor or nondivisible operands, and an empty reading with default Ask returns None. Left cannot return a negative balance. This partial forbidden-pair relation does not exclude wrong linguistic matches: same_thing accepts substring matches (pineapple/apple), question quantities can influence total/rate, and known schema failures remain possible.

## reverse-reconstruction
A returned scalar and reason admit all Q/Ask/text triples satisfying the first reached schema. Sum 8 over two nonnegative values <=8 has nine ordered antecedents. Reasons retain operand values but lose parse heads, original numeral spelling, discarded modifiers and sometimes holder context. None has many fibers: empty data, insufficient operands, incompatible tags, nondivisible each and negative left. It is not unique inverse parsing or reconstruction of the original problem.

## numeric-surface
num removes every comma and dollar sign, parses f64, requires zero fractional part then casts to i64. Thus 1,200, $1200, 12e2 and 1200.0 collide. This is not exact decimal parsing: large integers can round and casts saturate. Tests cover small admitted values, negative values, fractional rejection and nonfinite rejection, not all i64 precision edges.

## discourse-order
Subject prefers PROPN/PRON among direct nsubj/expl children, then the first subject, then up to four conjoined ancestors, else last name. he/she/they/it substitute last; possessive subjects prefer possessor. Supplied two-sentence fixture checks prior-name carryover and empty-sentence numbering. This is a heuristic anaphora model, not discourse identity resolution.

## schema-priority
Total precedes rate and each; explicit rate precedes sharing. With rate=3 and count=12, each=true still returns product 36. Comparison picks strict best positive tag overlap, preserving first tie, and excludes equal Q values through structural equality. Part/whole uses largest nonquestion quantity and body cue; repeated structurally equal quantities can be excluded together. These are implementation observations, not justified arithmetic semantics.

## arithmetic-domain
Tests restrict operands to 0..12 and ledger to 0..6. Production i64 addition, multiplication, subtraction and abs are unchecked; overflow can panic in debug or wrap in release. No cross-profile equivalence is promised outside the bounded safe domain. Integer division does not approximate fractional answers.

## question-leakage
Comparisons and part/whole exclude in_q, but total, rate, each and left do not apply a universal question filter. Ask fields update on each question sentence; branches without resets can preserve prior fields (e.g. right_tags if no later than). The model deliberately retains these order effects; no broad semantic repair is claimed.

## acquisition-boundary
frames::parse_words calls the annotator and Tree::parse_cased, returning an empty Word vector on acquisition failure; public treeread then silently returns an empty reading/None rather than exposing an acquisition error. Its dependency behavior remains external; supplied read_sentences and answer_reading isolate deterministic mechanics without model files. Public read/answer retain the same acquisition and sentence split. School and knowledge callers invoke this optional heuristic path; historical train measurements in the source are not new evaluations and no held-out set was run.

## composition
reading.groupant maps words to quantity and question observations; schemas.groupant maps those observations with original textual whole/part cues to answer; numeric.groupant models numeric surfaces and coarse thing matching. Shared Q equality, ordered tags and Ask flags constrain composition. Numeric validity does not imply correct units or correct parser attachment. Recommend a quantitative inference parent distinct from the QA index/anchor reader.

## checks
Native filter treeread::wave8_models has five tests: numeral fibers, supplied-tree discourse, 81 total cases, 169 each cases plus priority, and 343 ledgers plus refusals. These compare real functions with cardinality/divisibility/balance oracles. See model.md#evidence for profile repetition and compiled mutation.

## epoch-memory

First source-bound model and private supplied-input seams, preserving production acquisition. Predictions: sums retain nine bounded antecedents; each must reject nondivisible data; explicit rate wins over each. Tests are finite correspondence, not complete algorithm or reading-quality proof. No executable approximant: unsupported semantic metric. Partial Rust syntax projection is source-bound but has no macro expansion, name resolution or semantic AST.

## evidence

### Executed bounded correspondence

Five native tests in `treeread::wave8_models`; ten combined `wave8_models` tests passed twice in debug and twice in release with `--test-threads=1`, and once after restoring both compiled controls. The normalized ten test identities/results match across all five runs. No ignored tests, annotator acquisition, GPU, live teacher, corpus or held-out evaluation. Existing unrelated compiler warnings remain.

Reader control raises the production confidence word threshold from two to three; `confidence_partition_and_fibers` fails equality. Treeread control replaces the production total sum with product; `total_oracle_and_sum_fibers` fails equality. Both compiled, exited with failing assertions, and were restored before the final native pass. Separately, `anchor_zero_and_range_are_absent` failed with subtraction overflow before the checked-subtraction repair. These are test sensitivity observations, not proof of all branches.

Raw logs are external under run-relative names `debug-1.log`, `debug-2.log`, `release-1.log`, `release-2.log`, `restored.log`, `reader-mutation.log`, `treeread-mutation.log`, and `before-fix.log`; no host paths are committed. A separate one-file registry validates each root and its three fragments; the main registry and inter-file ancestry are integrated by the parent task. Models deliberately retain parser, overflow, entity identity and reading-quality gaps.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound model and private supplied-input seams, preserving production acquisition. Predictions: sums retain nine bounded antecedents; each must reject nondivisible data; explicit rate wins over each. Tests are finite correspondence, not complete algorithm or reading-quality proof. No executable approximant: unsupported semantic metric. Partial Rust syntax projection is source-bound but has no macro expansion, name resolution or semantic AST.

### 2026-10-30 · evidence

Heading `### Executed bounded correspondence — 2026-10-30` stays in evidence as `### Executed bounded correspondence`.

- Source SHA-256: `854e8c3e3a7e1abd56800d0003cf2e1b92cb207ce1d07aa5fef43e2d3b08fed8`.

- Partial syntax SHA-256: `95293fbec1daa9d2d8b88668f1932a7362145248ef20f9f70e27aa37b565ec7e`; two generations byte-identical.

- Combined normalized result SHA-256: `a3d9d7b1ab323e2b7450b0080a8719c9182c20a69724b0bb8378fb8b754c1071`.

- Compiled control log SHA-256: `7aadc078f2f0fb1d42b5e582334809e8e98a1d5d0f43707ffaac6b938d20da6a`.

Isolated structural validation: copied this exact source/sidecar into a temporary Git repository with its one-node registry; all root/three-fragment declarations passed twice, with byte-identical reports and zero diagnostics. Report SHA-256: `453230e62ab8c21c0ee50cd7d921b0d183fb573f30509368cefa8acb22c49250`. The full worktree cannot use a one-node registry because existing unrelated sidecars retain their own owners; this isolated check is deliberately separate from the parent's complete integration check.
