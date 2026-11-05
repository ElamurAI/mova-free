# translation-meta-logic

## descriptant

### Description and epoch memory

Project frames to lowercased events and numbers; lexicon maps verbs/entities. Multiset counts shared events/facts, LCS aligns ordered events. Count negation, both-present modal mismatch, numbers deficit, crossing transitive nonpronoun role or named-role mismatch.

#### Gaps

No real annotator/model loads, corpus translation, teacher calls, GPU, full training/evaluation, or language-accuracy claim. No executable percentage surrogate is supplied: an appropriate reference metric/domain is not established. Supported future caps are1/2/5/10 percent only. Residual reconstruction is conditional on retained diagnostics and is not persistent learning.

## approximant

### Forward relation

Project frames to lowercased events and numbers; lexicon maps verbs/entities. Multiset counts shared events/facts, LCS aligns ordered events. Count negation, both-present modal mismatch, numbers deficit, crossing transitive nonpronoun role or named-role mismatch.

## invariant

### Invariant and cost

#### Domain and preservation

Valid role slots0..3, finite tokens, u64/i64 counters no overflow, LCS safe width. perturb numeric increment excludes u64::MAX. These are parser-derived diagnostics, not proof of textual entailment.

#### Cost

O(nm) LCS cells, O(F log F) fact maps with string comparisons; matched role scans O(sum Rx*Ry). probe runs up to4 comparisons plus membership scans.

Counters and allocation arithmetic must remain representable. String/map costs include compared bytes; finite evidence is not a throughput estimate.

## composition

### Local composition

The child isolates the translation-meta-logic-logical_words relation inside this source. Its observation is a projection of the complete file relation described in model.md#approximant; costs overlap and must not be added twice. The remaining code supplies context/table inputs and consumes outputs. External joins: meta MTok feeds profile, Hebrew encoder, fwt and rules; meta LCS feeds logic and alignment-based training; Strong labels optionally override IBM ownership. No runtime chain between all children is asserted.

## models

# Typed models

Subject: exact source hash in manifest; observation is one call or the stated finite training pass. All models are partial source-derived relations, not universal proof. Domain and resource assumptions are in model.md#invariant. The baseline exclusion is an inner source-derived claim on that domain; reverse is a compatible-antecedent relation. Specialized relations hold fixed the parameters and tables unless stated otherwise. Evidence uses the named checks in model.md#tests; individual assertions cover only finite parts, other branches remain prospective.

## behavior_complement

Unmatched events are gaps, not contradictions. Missing modal alone cannot contradict. Permuting complete events preserves the verb-multiset gap term. Contradictions depend on the ordered verb LCS and its tie-selected pairs: repeated verbs with different negation or roles can pair differently and increase or decrease distance. No permutation-invariance law is asserted for contradiction distance.

## reverse_reconstruction

Logic totals admit many event sequences and fact assignments. Zero contradiction permits omitted events, missing modals and pronoun/name uncertainty; original sentences cannot be reconstructed.

## distance_domain

distance=contra+(ev_left+ev_right-2*shared)/2, nonnegative under valid aggregate relation.

## crossing_precedence

Both transitive and a nonpronoun crossed identity yields one contradiction then skips named-role comparison.

## numeric_multiset

Number contradiction=max(lengths)-multiset_intersection; number normalization aliases sixty/threescore.

## sensitivity_scope

perturb only matched event positions; absence of applicable perturbation is no evidence of sensitivity.

## tests

### Checks and evidence

- logic::tests::every_kind_of_change_is_seen
- logic::tests::contradictions_and_their_control

Exact cargo commands use package translate, binary translate, --offline --locked and -- --exact --test-threads=1. External native-commands.json lists every name; debug and release run twice, numerical status and named outcomes retained. Some tests target other portions of the file; this fragment has source reasoning plus only the overlapping assertions.

Compiled controls mutate production decision expressions and restore exact original bytes; see external mutations.json and restoration receipt. No assertion-only mutation is counted. The evidence summary is authoritative for completed receipts. Scoped source reasoning and finite tests do not establish total branch coverage.

## History

### 2026-10-31 · descriptant

#### Prospective epoch 2026-10-31

Models predicted metadata loss, bounded coordinate transport and deterministic tie selection before test changes. Production bytes are preserved; only five inline synthetic tests were appended across the group.

#### Observation epoch 2026-10-31

Selected named native tests are bound in model.md#tests; raw receipts are external under $MOVA_DATA/runs/octopus-wave32-tr_meta-2026-10-31. Syntax is generated twice and byte-compared. This is partial syntax without macro expansion or semantic resolution.

#### Independent review correction — 2026-10-31

The initial complement incorrectly claimed that reordering a fully matched event multiset cannot raise distance. Independent review supplied an actual-source compiled witness: with identity Lexicon, [say(false), say(true)] compared with its reversal has ev=(2,2,2), gaps=0, contra=2 and distance2, while self-comparison has distance0. The verb-only LCS pairs repeated verbs positionally and therefore changes which negations are compared. The corrected model preserves only the verb-multiset gap term under permutation; contradiction distance depends on selected LCS pairs.

Evidence: $MOVA_DATA/runs/octopus-wave32-rt_core-2026-10-31/cross-review-tr_meta.md and tr-meta-review-witness.log; build and witness statuses are0. The witness uses an external exact source copy plus one appended synthetic test. It is independent evidence, not an additional test in this worker's18-name native slice. Production/source hashes and final native receipts remain unchanged.

### 2026-10-31 · tests

#### Recorded 2026-10-31 result

This source contributes 2 exact named checks to the 18-check group. Every name passes twice in debug and twice in release, with identical sorted outcomes. A compiled production mutation in logic.rs was rejected by its selected runtime assertion; its original and restored SHA-256 match. All 18 checks pass again after all nine restorations. There were nine rejected production mutations, zero assertion-only mutations and zero surviving controls in this batch. Nine source projections matched byte-for-byte across two generations. No original production bytes changed.

The augmented-registry check has no owned source or fragment diagnostic. Only root-owned inference-core/mova child/source bindings await integration. A first isolated-registry experiment was unsuitable because the checker inspects unrelated existing sidecars; those failures are retained separately and are not counted as semantic controls.
