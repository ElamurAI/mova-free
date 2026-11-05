# translation-meta-meta

## descriptant

### Description and epoch memory

Learn candidate gap links, optionally judge residual/logic cost, union clusters and choose greatest occurrence canonical member (lexical smallest tie). read greedily chooses longest phrase, absorbs selected function/joiner spans, derives representative head and optionally coarsens. tnodes drops span; TSV saves clusters and explanations only.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Learn candidate gap links, optionally judge residual/logic cost, union clusters and choose greatest occurrence canonical member (lexical smallest tie). read greedily chooses longest phrase, absorbs selected function/joiner spans, derives representative head and optionally coarsens. tnodes drops span; TSV saves clusters and explanations only.

## invariant

### Invariant and cost

#### Domain and preservation

Finite token arrays; phrase lengths bounded by max_len. LCS length<=65535, products and u32/u64 counts representable; well-formed UTF-8 keys without TSV delimiters for clean serialization. Read handles out-of-range direct heads as root; arbitrary learned callbacks excluded from native execution.

#### Cost

LCS O(nm) time/space; candidate phrase joining O(n L²) token visits plus bytes/map comparisons. Clause ancestry O(n²). Judge includes cross-cluster |A||B| pairs, repeated occurrence aggregation and up to12 sampled reader callbacks per memoized pair; no corpus speed claim.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## composition

### Local composition

The child isolates the translation-meta-meta-lcs relation inside this source. Its observation is a projection of the complete file relation described in model.md#approximant; costs overlap and must not be added twice. The remaining code supplies context/table inputs and consumes outputs. External joins: meta MTok feeds profile, Hebrew encoder, fwt and rules; meta LCS feeds logic and alignment-based training; Strong labels optionally override IBM ownership. No runtime chain between all children is asserted.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Within valid counts and finite inputs, LCS cannot output unequal or nonmonotone anchor pairs; TSV cannot preserve feature switches, max_len, refusal history or unclustered mappings. Concept equality excludes no semantic contradictions by itself.

## reverse_reconstruction

A meta token admits every surface span whose longest-match and representative-head rules yield it. tnodes preimage contains arbitrary spans. TSV reconstructs cluster rows/why under clean delimiters, with caller max_len; switches and refusal list remain free antecedent state.

## lcs_tie

Equal skip scores advance source i, so [a,b] vs [b,a] chooses (1,0).

## metadata_projection

from_tsv(to_tsv(L),k) fixes k and default switches, regardless of L switches.

## merge_gate

Judged merge accepted iff pair_ok and logic_ok and saved-cost>=51; 17-bit estimates are coding proxies, not probabilities.

## phrase_coordinate

Spans partition source token positions; heads and relations taken from first external-head nonjoiner representative, fallback first nonjoiner.

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

### 2026-10-31 · tests

#### Recorded 2026-10-31 result

This source contributes 4 exact named checks to the 18-check group. Every name passes twice in debug and twice in release, with identical sorted outcomes. A compiled production mutation in meta.rs was rejected by its selected runtime assertion; its original and restored SHA-256 match. All 18 checks pass again after all nine restorations. There were nine rejected production mutations, zero assertion-only mutations and zero surviving controls in this batch. Nine source projections matched byte-for-byte across two generations. No original production bytes changed.

The augmented-registry check has no owned source or fragment diagnostic. Only root-owned inference-core/mova child/source bindings await integration. A first isolated-registry experiment was unsuitable because the checker inspects unrelated existing sidecars; those failures are retained separately and are not counted as semantic controls.
