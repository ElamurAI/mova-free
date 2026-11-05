# translation-meta-ibm-align

## descriptant

### Description and epoch memory

For target center(j+.5)/max(k,1), scan source centers and choose strict largest attenuated score versus NULL*.3.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

For target center(j+.5)/max(k,1), scan source centers and choose strict largest attenuated score versus NULL*.3.

## invariant

### Invariant and cost

#### Domain and preservation

Finite nonnegative table scores; empty f or e allowed.

#### Cost

O(k*n*log Rmax) lookup comparisons and k output cells; exp per source/target pair.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

All-zero table produces None for every target. Empty source never produces a source index.

## reverse_reconstruction

A Some(i) output describes scores greater than NULL and all earlier scanned candidates, at least later scores; None describes no score strictly above NULL.

## center_geometry

Distances use separate source and target lengths floored at1.

## source_tie

Equal maximal source scores choose earliest source index.

## null_tie

NULL survives equality because update uses strict greater.

## index_domain

Returned Some index always less than source length, independent of vocabulary ID.

## tests

### Checks and evidence

- ibm::tests::learns_a_toy_dictionary
- ibm::wave32::zero_prior_and_normalization

Exact cargo commands use package translate, binary translate, --offline --locked and -- --exact --test-threads=1. External native-commands.json lists every name; debug and release run twice, numerical status and named outcomes retained. Some tests target other portions of the file; this fragment has source reasoning plus only the overlapping assertions.

Compiled controls mutate production decision expressions and restore exact original bytes; see external mutations.json and restoration receipt. No assertion-only mutation is counted. The evidence summary is authoritative for completed receipts. Scoped source reasoning and finite tests do not establish total branch coverage.

## History

### 2026-10-31 · descriptant

#### Prospective epoch 2026-10-31

Models predicted metadata loss, bounded coordinate transport and deterministic tie selection before test changes. Production bytes are preserved; only five inline synthetic tests were appended across the group.

#### Observation epoch 2026-10-31

Selected named native tests are bound in model.md#tests; raw receipts are external under $MOVA_DATA/runs/octopus-wave32-tr_meta-2026-10-31. Syntax is generated twice and byte-compared. This is partial syntax without macro expansion or semantic resolution.
