# translation-meta-strong

## descriptant

### Description and epoch memory

number trims leading H and parses positive decimal prefix. words scans USFM markers, assigns Strong attribute to each marked word and retains plain chunks. read removes footnotes and groups sorted files by verse. on_tokens lowercases alnum characters, advances bounded lookahead cursor on first match.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

number trims leading H and parses positive decimal prefix. words scans USFM markers, assigns Strong attribute to each marked word and retains plain chunks. read removes footnotes and groups sorted files by verse. on_tokens lowercases alnum characters, advances bounded lookahead cursor on first match.

## invariant

### Invariant and cost

#### Domain and preservation

Finite UTF-8 inputs, allocations and pos+40/start+len representable; Strong number u32>0. No real USFM directory or corpus accessed. Filesystem parser path remains source-model-only.

#### Cost

Marker processing O(B) ordinary scans plus string allocation; token mapping O(B+40 sum token_chars) comparisons and O(B) character storage. Directory sorting O(F log F) and full input reads.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## composition

### Local composition

The child isolates the translation-meta-strong-token_alignment relation inside this source. Its observation is a projection of the complete file relation described in model.md#approximant; costs overlap and must not be added twice. The remaining code supplies context/table inputs and consumes outputs. External joins: meta MTok feeds profile, Hebrew encoder, fwt and rules; meta LCS feeds logic and alignment-based training; Strong labels optionally override IBM ownership. No runtime chain between all children is asserted.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Zero/overflow/nondecimal number prefixes produce None. Failed token and punctuation cannot advance cursor. Match at offset40 is outside search window.

## reverse_reconstruction

Numbers alias H prefixes/leading zeros/trailing nondigits. Token labels lose punctuation, spacing, case, unmatched text and per-character mixed ownership; first matched char label does not identify complete source word boundaries.

## number_prefix

trim_start_matches H removes repeated uppercase H, not lowercase h.

## cursor_window

Start range excludes pos+40, includes pos; failure retains pos.

## character_owner

Whole target token inherits first source character's Strong label even crossing source words.

## marker_fallback

Missing closing w marker consumes remainder; unknown markers strip marker syntax but retain body.

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

### 2026-10-31 · tests

#### Recorded 2026-10-31 result

This source contributes 2 exact named checks to the 18-check group. Every name passes twice in debug and twice in release, with identical sorted outcomes. A compiled production mutation in strong.rs was rejected by its selected runtime assertion; its original and restored SHA-256 match. All 18 checks pass again after all nine restorations. There were nine rejected production mutations, zero assertion-only mutations and zero surviving controls in this batch. Nine source projections matched byte-for-byte across two generations. No original production bytes changed.

The augmented-registry check has no owned source or fragment diagnostic. Only root-owned inference-core/mova child/source bindings await integration. A first isolated-registry experiment was unsuitable because the checker inspects unrelated existing sidecars; those failures are retained separately and are not counted as semantic controls.
