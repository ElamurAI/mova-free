# qagen-gate

## model

# Gate file model

## forward

G(text,paras,rule) = batch_classify followed, only when valid, by independent ordered paragraph folds. A fold composes lexical views, anchors and row predicates, canonical question deduplication, cap and accepted rank. Observation includes accepted pairs, rejection order/levels/reasons, junk and optional batch reason.

## behavior-complement

Excluded outputs include accepted rows with a batch format failure, k gaps within accepted paragraph rows, anchors outside local sentences and more than six accepted rows per paragraph. These exclusions are conditional on supplied Para and checked arithmetic.

## reverse-reconstruction

Accepted projection fiber contains all row streams whose validated distinct first-six subsequences equal the observation, with arbitrary ignorable header/fence/blank lines and row rejects that do not trigger a batch failure. Full Gated observation also constrains raw rejection lines, order, junk and reason strings. Canonical accepted rows provide a witness only if remaining paragraph coverage satisfies batch thresholds.

## specialization

batch_row_product: typed observation (Option<BatchReason>, Vec<Pair>, Vec<Reject>, usize); any BatchReason forces Vec<Pair>=empty.

## invariant

All accepted anchors are sorted unique local indices; accepted paragraph and question indices are 1-based. span_reason requires equal word/normalized sentence lengths and prevalidated in-range anchors; arbitrary direct callers can violate that precondition. Integer multiplication assumes representable counts; memory exhaustion and usize overflow are outside this model.

## composition

Lexical children produce distinct W and N views. Row checks consume both plus parsed anchors; batch classification precedes row checks and paragraph accumulation adds deduplication/cap. Accepted tuples cannot be fed back as full rejection logs: that projection loses information. The lexical node composes normalization followed by boundary matching; it does not identify word equality with span equality.

## cost

Let B be supplied bytes, R rows, S total paragraph sentence bytes and A answer length. Lexical normalization is O(B) scalar work/storage (lowercase expansion bounded by Unicode tables). Verbatim window comparisons cost O(total sentence tokens * answer tokens). Span matching with repeated suffix checks is conservatively O(S*A); anchors O(K log K). Batch indexing O(R log N), stored raw/column rows O(B); each row rebuilds answer views and scans relevant sentence views. No measured speed or approximation-error claim.

## epoch-memory

Prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

## checks

Eight gate::wave17_models tests: Unicode interval oracle 341 squared = 116281 comparisons plus two overlap witnesses; batch integer-ratio oracle 220 cases; literal normalization; empty anchors; single prefix; cap/dense numbering; span/implicit composition; accepted-projection reconstruction. Synthetic Para only, no live teacher/parser/corpus/database. Debug/release twice and compiled mutations are recorded in root model.md#evidence. Finite evidence does not establish all Unicode/grapheme semantics or full source correctness.

## evidence

### Gate evidence

Eight new tests passed twice debug and twice release after exact restoration of three compiled production mutations. Named outcomes were compared and identical across all four final runs. Independent span interval oracle: 341 strings over a/é/space/! at lengths zero through four, 116281 ordered pairs; plus two overlap witnesses. Integer batch ratio oracle: 220 cases over N=1..8, present=0..N, junk=0..4.

Old production failed empty_anchor_components_are_rejected and paragraph_prefix_is_single_optional_letter. The initial run also found a test-only reject-count mistake (six instead of eleven minus six = five); this expectation was corrected, not counted as a production defect. Compiled controls separately restored each old behavior and changed span left&&right to left||right. Each failed its intended assertion, not compilation.

Syntax projections were generated twice identically. Groupant structural reports from an external scaffold registry also compared identical and valid; global registry and group sidecars were not edited. Structural validity does not prove semantics.

Scope deviation: an extra qagen --lib regression ran all 26 existing/new tests, including lemma::tests::doubt_with_en_model, which conditionally loads a local en lemma model. That broader invocation exceeded the intended synthetic-only scope. No teacher, corpus, database or external parser process was invoked. The eight new tests and subsequent gate-only regression use supplied synthetic Para fixtures exclusively. Existing dependency unused/dead-code warnings remain.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806. 2628 partial syntax nodes; macro token bodies opaque, no semantic AST/proof claim. One file groupant and five nested fragments (lexical contains normalization and spans; rows and batch are siblings). Six forward models, six complements, six set-valued reconstruction models and six specialized typed observations. All five fragments select actual Fn syntax nodes and exact enclosing byte ranges.

Raw artifacts live at $MOVA_DATA/runs/octopus-wave17-gate-2026-10-30/. SHA-256 receipts:

- prospective.md: 7da41c925db5fcaf28a3fb46fd10bf3d9ffeebadcd59953cb9038e780a52e3e4

- before.log: 04c66000ca5f0528fad016d00677aed4fb72c42fbd4be2608ea6e0db21a2362e

- mutation-empty-anchor.log: 244df9ca6d86e7527a9d0a66dab601124fa70a1d37804f9670a5d330e822aac0

- mutation-repeated-prefix.log: a4e834282744b16c2fd452dadc1a52416caa90b42b88ab134ea94e0f8aeb8c65

- mutation-span-boundary.log: 3259978bb0051fb96721d96258db1db4b9107af073a8fda8d56b54b71d159c76

- final-debug1.log: 02d8980b8aaee3c72d94361de38e68f9da53dfe4dbaddcfa4edf74d7e0c626a3

- final-debug2.log: 23e373a8533d487ab0970ce1b45ed14d35d77c7cf86a90a6cb9287fbd8426167

- final-release1.log: 7964badc3eaf909d38e397d4d0a3a2cdd1276fcc8084554769de13c26a0266fa

- final-release2.log: bd77dd8cf480dcb34ec770f738443b262d883e56efeef91eb97ac13dbcdf5e82

- outcomes.txt: 803662df6790d69cbe90805ec9b5b5bd9a8de7629d6afb165e8bc5658bfc5649

- syntax1.json: 66f3f6356b79312ffeb02acd8268f3c5b73c01a749a6cc7e5370153c3c486729

- syntax2.json: 66f3f6356b79312ffeb02acd8268f3c5b73c01a749a6cc7e5370153c3c486729

- groupants1.json: e46a6c1f66b736c03e22d7484f3b4949f80c1646aacbc28fbd7a19324cc65fdc

- groupants2.json: e46a6c1f66b736c03e22d7484f3b4949f80c1646aacbc28fbd7a19324cc65fdc

- regression.log: c1585202e983017268719c5401e462f9d4d51f2f78786df7df3d332101a9661b

- gate-regression.log: c0956ceb6c68d2a745de9d9e1fb6397dcd4402e393ebe7fc73fa7bcf6a84efe7

### Undated · document

Source SHA-256: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806. Domain: finite supplied UTF-8 strings and synthetic Para values; rule V1/V2. Observation is the operation result specified below. Mathematical descriptions are source-level exact relations under stated assumptions, not a certified implementation theorem or numerical approximant.
