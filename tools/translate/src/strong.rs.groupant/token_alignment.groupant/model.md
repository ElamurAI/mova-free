# translation-meta-strong-token_alignment

## descriptant

### Description and epoch memory

Flatten source alnum lowercase chars with per-word labels. For each normalized token scan up to40 start positions; first contiguous match emits start label and advances by token char length; failure emits None.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Flatten source alnum lowercase chars with per-word labels. For each normalized token scan up to40 start positions; first contiguous match emits start label and advances by token char length; failure emits None.

## invariant

### Invariant and cost

#### Domain and preservation

Character coordinates, not UTF-8 byte indices; finite strings and representable index arithmetic.

#### Cost

O(source_chars+40 sum normalized_token_chars), output O(tokens) and flattened source storage.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Empty normalized token returns None without search; a match starting40 ahead cannot be selected.

## reverse_reconstruction

Labels admit every alnum string/word partition with matching first-character labels and valid cursor matches; removed case/punctuation cannot be recovered.

## unicode_lower

flat_map lowercase may expand a Unicode character; coordinates follow expanded chars.

## failure_state

Absent token leaves cursor unchanged, allowing a later token to match same remaining text.

## first_label

A token spanning multiple source words inherits only first char label.

## lookahead_boundary

Offset39 accepted, offset40 rejected by half-open range.

## tests

### Checks and evidence

- strong::tests::strong_words_and_tokens
- strong::wave32::window_and_numeric_boundaries

Exact cargo commands use package translate, binary translate, --offline --locked and -- --exact --test-threads=1. External native-commands.json lists every name; debug and release run twice, numerical status and named outcomes retained. Some tests target other portions of the file; this fragment has source reasoning plus only the overlapping assertions.

Compiled controls mutate production decision expressions and restore exact original bytes; see external mutations.json and restoration receipt. No assertion-only mutation is counted. The evidence summary is authoritative for completed receipts. Scoped source reasoning and finite tests do not establish total branch coverage.

## History

### 2026-10-31 · descriptant

#### Prospective epoch 2026-10-31

Models predicted metadata loss, bounded coordinate transport and deterministic tie selection before test changes. Production bytes are preserved; only five inline synthetic tests were appended across the group.

#### Observation epoch 2026-10-31

Selected named native tests are bound in model.md#tests; raw receipts are external under $MOVA_DATA/runs/octopus-wave32-tr_meta-2026-10-31. Syntax is generated twice and byte-compared. This is partial syntax without macro expansion or semantic resolution.
