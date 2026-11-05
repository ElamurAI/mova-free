# mova-whale-contract

## Descriptant

Configured output contract: length measure and projection, field rules, verbatim guard and bundle rendering. Source: `tools/whale/src/contract.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

`validate`: length field count in [min,max] in the configured unit without forbidden substrings (error prefix `length gate:`), then each field rule (equals; nonempty bounded text; bounded nonempty string lists; object lists with text/list/enum fields, distinctness and required member values), reply byte budget and verbatim context run. `project`: for an oversized field cut at the last paragraph break, else last sentence end, whose prefix count is admitted; record a trace. `rewrite_statuses` keeps the claimed value and downgrades the rest; `artifacts` renders text, bullet lists, JSON projections and the metadata file.

## Invariant

A validated body satisfies every configured rule; a projection output is a byte prefix of the input whose count is in [min,max] and never pads or rewrites; status rewrites retain the teacher claim; artifact order follows the specification.

## Cost

Word counting O(n); projection O(n * boundaries * n) worst case from recounting prefixes; validation O(reply); verbatim guard O(tokens) BTreeSet.

## Composition

Called by whale::admit and whale::project_quarantined_lengths; publish renders artifacts after validation.

## behavior_complement

Excluded: out-of-range length, blank or oversized fields, duplicates by distinct_by, missing required members, values outside enums, equality mismatches, replies above budget, configured verbatim runs, oversized fields without an admitted boundary or with forbidden substrings. Counterexample: an oversized field with no paragraph/sentence boundary is not projected. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a projected field all originals with the same prefix and a longer tail are compatible; the trace pins input/output hashes and boundary, so the original is recovered only with the retained reply. A validated count admits many texts. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: tests of word bounds on a fixture spec (4000..5000: 3999 and 5001 refused; 5002 with a sentence end projected to 5000; 5001 without one refused) and an opt-in MOVA_WHALE_SPEC_CHECK test of a deployment spec. No production change.
