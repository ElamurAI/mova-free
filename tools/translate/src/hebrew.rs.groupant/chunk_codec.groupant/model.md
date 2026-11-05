# translation-meta-hebrew-chunk_codec

## descriptant

### Description and epoch memory

Encode Entry as concept/tag/relation/head separated by slash and spaces. Decode rsplitn4 allowing slashes inside concept; malformed entries omitted; nonnumeric head becomes None. gloss splits dot/space/underscore, removes entries starting parenthesis.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Encode Entry as concept/tag/relation/head separated by slash and spaces. Decode rsplitn4 allowing slashes inside concept; malformed entries omitted; nonnumeric head becomes None. gloss splits dot/space/underscore, removes entries starting parenthesis.

## invariant

### Invariant and cost

#### Domain and preservation

Concepts cannot contain spaces for exact chunk roundtrip; valid Tag/Rel Display/parse; numeric heads interpreted zero-based within chunk.

#### Cost

O(B) scan/format plus allocated output bytes.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Malformed tag/relation fields cannot produce Entries; empty gloss parts never emitted.

## reverse_reconstruction

On delimiter-safe entries chunk_of(chunk_key(E))=E. General input strings alias through omitted malformed entries and arbitrary nonnumeric head strings.

## slash_transport

rsplitn4 preserves earlier slash characters inside concept.

## head_optional

Any head parse failure means None, not a rejection.

## gloss_filter

Only word starts '(' are filtered; this is not balanced-parenthesis parsing.

## empty_chunks

Empty encoded chunk yields no entries and models silent Hebrew words.

## tests

### Checks and evidence

- hebrew::wave32::gloss_chunks_and_owned_alignment

Exact cargo commands use package translate, binary translate, --offline --locked and -- --exact --test-threads=1. External native-commands.json lists every name; debug and release run twice, numerical status and named outcomes retained. Some tests target other portions of the file; this fragment has source reasoning plus only the overlapping assertions.

Compiled controls mutate production decision expressions and restore exact original bytes; see external mutations.json and restoration receipt. No assertion-only mutation is counted. The evidence summary is authoritative for completed receipts. Scoped source reasoning and finite tests do not establish total branch coverage.

## History

### 2026-10-31 · descriptant

#### Prospective epoch 2026-10-31

Models predicted metadata loss, bounded coordinate transport and deterministic tie selection before test changes. Production bytes are preserved; only five inline synthetic tests were appended across the group.

#### Observation epoch 2026-10-31

Selected named native tests are bound in model.md#tests; raw receipts are external under $MOVA_DATA/runs/octopus-wave32-tr_meta-2026-10-31. Syntax is generated twice and byte-compared. This is partial syntax without macro expansion or semantic resolution.
