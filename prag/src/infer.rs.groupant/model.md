# prag-infer

## model

# Supplied-word inference

## forward

For immutable words W, earlier sentences E and punctuation text S, define I(W), P(W,E), M(W,S) as the three exported inference vectors. The command's observable vector is I ++ P ++ M; it then appends W to E. Each result includes kind, surface trigger, rendered content, explanation and licensed bit. Rules are deterministic lexical/dependency heuristics, not truth entailment. They do not depend on feats::Row, schema::Labels or model::Classifier. Those adjacent modules belong to learned pragmatic classification; mixing those APIs would erase the actual source boundary.

## behavior-complement

Subject is the bound source; universe U contains pairs ((W,E,S), arbitrary finite Inference vectors) under read-only input state. A source-derived inner bound on U minus graph(I++P++M) contains vectors that emit stronger-scale inferences after sentence-wide cancellation, license a predicate found only outside the last 50 prior tokens, emit a reflexive manner edge, or move an emitted maxim before an emitted scalar result. These are concrete exclusions in that observation scope; they do not classify every absent vector or linguistic possibility. Evidence is the scalar, history, malformed-head and chain fixtures.

## reverse-reconstruction

The converse fiber of an output O is all (W,E,S) satisfying I(W)++P(W,E)++M(W,S)=O. Retain shared W and its token order across all children. Different histories with the same last-50 predicate presence, punctuation texts with identical final suffix and words irrelevant to rules can give the same O. An empty vector may mean absent triggers or suppressed triggers; it cannot reconstruct a unique sentence, parse or actual intention. This is a relational specification, not an implemented complete inverse search.

## family-order

Hold W,E,S fixed. Observation projects each emitted result to kind and trigger. Its order relation is the concatenation of scalar token/scale order, presupposition token order, then question/exclamation/conjunction order. Duplicate scalar collapse can reduce multiplicity. The native chain fixture expects exactly Some, again, ?, ! for both mixed punctuation suffixes. It executes the same public calls as main but omits parsing, printing and irony.

## sentence-history

State transition T(E,W) = E appended with W occurs after the three calls in main. The present sentence cannot license its own presupposition unless the caller already supplied equivalent prior material. Given final history E', possible preceding states are prefixes with the final appended W; output-only observations retain many such states. Source inspection establishes main's call order; tests execute immutable supplied E and current W without running the CLI.

## invariant

No input mutation. Stable family order. Scalar and maxim results set licensed=true; presuppositions set it from lexical history membership. Invalid heads supply no direct predicate or valid event pair; dangling clause ancestry terminates false. This does not validate the entire dependency forest or exclude every malformed semantic inference.

## composition

Three immediate child scopes share the original Word coordinates. Presupposition owns a nested ancestry climb: it first chooses a complement index, then filters tokens against that same index, omits its Mark children and preserves source token order. Parent relation retains these coordinate equalities; composing independent projected fibers without them admits spurious histories.

## cost

Let n be sentence tokens and B total prior tokens. With fixed scale tables and bounded word lengths, scalar O(n²), presupposition O(B+n³), maxims O(n²). Whole function temporary history allocation is O(B), despite retaining only 50 tokens for licensing. Output and text lengths add their own storage costs. Guards are constant-time checks; no optimization or speed measurement is claimed.

## epoch-memory

Prospective expectations are retained in model.md#prospective. Old-source native witnesses showed an invalid ancestor panic and self-headed predicate/event claims. The narrow fix checks direct predicate heads, guards clause climbs and rejects reflexive or out-of-range conjunction heads. All valid legacy infer fixtures remain passing. Models retain substring cancellation, first-complement choice and lexical-only licensing as limitations. model.md#evidence binds finite executed observations, including the compiled wrong-window control. Macro bodies remain opaque; the syntax projection is partial.

## Epoch notes

## evidence

### Native inference evidence

Six wave13_models tests and four existing infer tests pass twice in debug and twice in release with identical normalized named outcomes. Three old-source failures establish malformed-ancestor panic, self-headed predicate inference and reflexive event precedence. A compiled production mutation changes take(50) to take(49), fails history_cutoff_is_fifty_tokens_not_sentences at padding=49, and is restored byte-for-byte before positive runs. Partial syntax generation repeats identically. No parser, teacher, GPU, corpus, filesystem loading or learned classifier was executed. Existing unrelated frames.rs compiler warning remains. No timing or universal correctness claim.

## prospective

### Prospective inference boundary review

Domain: supplied finite Word vectors, earlier sentence vectors and sentence text; no parser execution. Predictions before edits: scalar output follows token/scale order, duplicate some scales collapse, stronger/negation/cancellation block. Presupposition licensing depends only on predicate presence among the last 50 flattened prior tokens. Maxims emit question then exclamation then conjunction order. Invalid nonzero heads currently panic in presupposition and manner paths; clause traversal can encounter an invalid ancestor. Self-headed conjunction currently can claim an event precedes itself. Plan independent literal fixtures to witness these, then bounds-check and reject self heads while preserving valid behavior and public signatures. Negated is crate-private with valid index callers; no new API.

Invariant: output order is stable, inputs remain unchanged, presupposition licensing is contextual rather than truth verification; malformed unreachable ancestry cannot produce a clause member, and a manner relation must have distinct event coordinates. Complexity prediction unchanged: scalar O(n^2) for fixed scales; presupposition worst O(n^3) including per-trigger subtree climbs and O(B) earlier flattening; maxims O(n^2). Extra constant-time head guards do not establish a speedup. Independent oracles: literal content/ordering expectations, 49/50 token history cut, known clause membership with invalid unrelated branch, and finite scalar blocks. Negative compiled production control will alter the history cutoff, expected to fail the exact boundary assertion. No learned model/schema/feats dependency in this source; main chains public inference APIs and irony consumes negated.

## History

### 2026-10-30 · evidence

Heading `### Native inference evidence, 2026-10-30` stays in evidence as `### Native inference evidence`.

Receipt set: octopus-wave13-infer-2026-10-30. SHA-256:

- old-witness.log: 61b101d8a2fbf3999f748efe672dd02b7c4d60be0f311260137f17c9c1530834

- mutation.log: e7c38fc10b60941ca2742150ff129574b7fe57d3b1fce501b1b727f7bbb0d647

- check.log: aeb6832ed18f7e9eadc038d48ff921535793d1ff36174ced3b13e223d6bf424a

- native-debug-1-normalized.txt: e2a9324fa1661014e305a0f542647ba790d59275e0c7e7a29e04950c702c1ca4

- native-debug-2-normalized.txt: e2a9324fa1661014e305a0f542647ba790d59275e0c7e7a29e04950c702c1ca4

- native-release-1-normalized.txt: e2a9324fa1661014e305a0f542647ba790d59275e0c7e7a29e04950c702c1ca4

- native-release-2-normalized.txt: e2a9324fa1661014e305a0f542647ba790d59275e0c7e7a29e04950c702c1ca4

- syntax-1.json: 02936c95727327089af1756e0fd5ca76ca40155ee0d015eb787bddde2bfb7eae

- syntax-2.json: 02936c95727327089af1756e0fd5ca76ca40155ee0d015eb787bddde2bfb7eae

Source SHA-256: b1916d26ae0d46360e76e3f1fc00aefeddc6704c39e742e5dd37a52025e140cf

Full-base external registry scaffold validates twice with equal reports and no diagnostics. It retains all base ownership and adds a temporary unmodeled scaffold root; aggregate parent registration remains integration work. Both groupants-full-1.json and groupants-full-2.json SHA-256: 362632634086e11732019a08a09f7a0ab3f87a8fcb0f89421c7785115b10a069. The deliberately incomplete one-leaf registry attempt was rejected for missing other owners; it is not a semantic failure of this leaf. Final syntax outputs use fresh paths after the tokenized-cancellation test addition; earlier output-file-exists refusal and recovery logs are retained externally.

### 2026-10-30 · prospective

Heading `### Prospective inference boundary review, 2026-10-30` stays in prospective as `### Prospective inference boundary review`.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
