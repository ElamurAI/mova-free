# translation-meta-meta-lcs

## descriptant

### Description and epoch memory

Suffix DP D(i,j)=1+D(i+1,j+1) on equality else max skip. Trace advances both on equality, source on skip tie. gaps emits nonempty open intervals between anchors and sentinel.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Suffix DP D(i,j)=1+D(i+1,j+1) on equality else max skip. Trace advances both on equality, source on skip tie. gaps emits nonempty open intervals between anchors and sentinel.

## invariant

### Invariant and cost

#### Domain and preservation

Arrays with min(n,m)<=65535; allocation arithmetic representable; gaps anchors strictly increasing in each axis and within lengths.

#### Cost

(n+1)(m+1) u16 cells; O(nm) comparisons plus O(n+m) trace; gaps O(k) output.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Equal skip costs cannot prefer target advance. Empty vs nonempty input cannot produce anchors.

## reverse_reconstruction

Anchor output admits all key pairs with those equal entries and DP choices; unmatched values and nonselected equal alternatives are not recoverable.

## anchor_order

Every consecutive pair increases both indices.

## gap_partition

Anchors and intervening half-open gaps partition each input, excluding anchor elements.

## score_width

No claim beyond65535 LCS length: u16 addition overflows.

## empty_boundary

gaps(0,m,[]) emits one empty/nonempty pair iff m>0.

## tests

### Checks and evidence

- meta::tests::the_logic_gate_on_merges
- meta::tests::lexicon_tsv_back
- meta::tests::lcs_and_gaps
- meta::wave32::metadata_loss_and_lcs_ties

Exact cargo commands use package translate, binary translate, --offline --locked and -- --exact --test-threads=1. External native-commands.json lists every name; debug and release run twice, numerical status and named outcomes retained. Some tests target other portions of the file; this fragment has source reasoning plus only the overlapping assertions.

Compiled controls mutate production decision expressions and restore exact original bytes; see external mutations.json and restoration receipt. No assertion-only mutation is counted. The evidence summary is authoritative for completed receipts. Scoped source reasoning and finite tests do not establish total branch coverage.

## History

### 2026-10-31 · descriptant

#### Prospective epoch 2026-10-31

Models predicted metadata loss, bounded coordinate transport and deterministic tie selection before test changes. Production bytes are preserved; only five inline synthetic tests were appended across the group.

#### Observation epoch 2026-10-31

Selected named native tests are bound in model.md#tests; raw receipts are external under $MOVA_DATA/runs/octopus-wave32-tr_meta-2026-10-31. Syntax is generated twice and byte-compared. This is partial syntax without macro expansion or semantic resolution.
