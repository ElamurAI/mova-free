# translation-meta-ibm

## descriptant

### Description and epoch memory

Vocab assigns sorted unique IDs1.., missing0. train_prior builds sorted cooccurrence/prior rows, initializes uniform, repeats fractional counts against NULL then adds prior pseudocounts and normalizes positive rows. align maximizes t*exp(-2*relative-distance) over NULL*.3.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Vocab assigns sorted unique IDs1.., missing0. train_prior builds sorted cooccurrence/prior rows, initializes uniform, repeats fractional counts against NULL then adds prior pseudocounts and normalizes positive rows. align maximizes t*exp(-2*relative-distance) over NULL*.3.

## invariant

### Invariant and cost

#### Domain and preservation

Source IDs<=nf, nf+1 and vocab ids representable; alpha finite nonnegative, sums finite; no claim for NaN/negative priors. Prior duplicates intentionally counted.

#### Cost

Initial storage Q=sum (|f|+1)|e|+prior entries before dedup; sort each column. Iterations O(I*(sum |e|(|f|+1) log Rmax+R+P log Rmax)) time and O(Q+R+nf) memory; align O(|e||f|log Rmax).

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## composition

### Local composition

The child isolates the translation-meta-ibm-align relation inside this source. Its observation is a projection of the complete file relation described in model.md#approximant; costs overlap and must not be added twice. The remaining code supplies context/table inputs and consumes outputs. External joins: meta MTok feeds profile, Hebrew encoder, fwt and rules; meta LCS feeds logic and alignment-based training; Strong labels optionally override IBM ownership. No runtime chain between all children is asserted.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Unlisted row/column lookup returns0; zero score cannot beat NULL tie. Training does not load a linguistic model or corpus itself.

## reverse_reconstruction

Rows admit many repeated/reordered training pairs and priors yielding same normalized counts. Alignment output identifies inequality cells in row probabilities, not unique lexical meaning.

## row_mass

In ideal real arithmetic, nonempty initialized rows and positive-total normalized rows have mass1; zero-total rows retain their previous values. Actual f64 divisions and summation introduce rounding, so exact machine mass1 is not guaranteed. The synthetic1e-12 assertion is finite evidence for its fixture, not a universal error bound.

## null_competition

Best starts(t(0,e)*.3,None); strict greater keeps NULL ties.

## duplicate_prior

Each duplicate e in prior adds alpha again despite row deduplication.

## vocab_order

Sorted lexical ID map independent of occurrence order;0 aliases unknown and NULL.

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

#### Independent review clarification — 2026-10-31

Row-mass normalization is an ideal real-arithmetic relation; the implementation uses f64 and may round away from exact mass1. The synthetic normalization assertion uses1e-12 for its fixed fixture only. No universal machine-arithmetic error bound follows. This documentation correction follows the external wave32 rt_core cross-review; source and native receipts are unchanged.

### 2026-10-31 · tests

#### Recorded 2026-10-31 result

This source contributes 2 exact named checks to the 18-check group. Every name passes twice in debug and twice in release, with identical sorted outcomes. A compiled production mutation in ibm.rs was rejected by its selected runtime assertion; its original and restored SHA-256 match. All 18 checks pass again after all nine restorations. There were nine rejected production mutations, zero assertion-only mutations and zero surviving controls in this batch. Nine source projections matched byte-for-byte across two generations. No original production bytes changed.

The augmented-registry check has no owned source or fragment diagnostic. Only root-owned inference-core/mova child/source bindings await integration. A first isolated-registry experiment was unsuitable because the checker inspects unrelated existing sidecars; those failures are retained separately and are not counted as semantic controls.
