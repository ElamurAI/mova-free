# normalization

## forward
Nine-column ordered rows fold contiguous equal (doc,para) into Para; derive book from first two colon components, words from whitespace and dialogue from has_quote. Context is immediately previous same-doc paragraph's final two sentences in source order.

## behavior-complement
Reject row length !=9, license without public-domain prefix, null/invalid paragraph. Never cross document boundaries for context. Prefix acceptance is actual behavior, not legal validation.

## reverse-reconstruction
For each normalized Para, compatible rows concatenate its sentence IDs/text under matching doc/para, first-row metadata and accepted license prefix. The ignored ord column is arbitrary; later repeated metadata is lost. This describes a set-valued preimage and presumes contiguous SQL order.

## invariant
Source-bound operational relation over supplied finite inputs; deterministic observations under fixed order and seed. No corpus/distribution assumption. Integer quota arithmetic assumes 32/64-bit usize.

## cost
One sequential row fold plus sentence/text scans; O(text bytes+rows) work, O(text bytes+rows) storage including cloned context.

## specialization
Type normalization_projection: scoped observation above, exact intended relation, no approximation-error budget asserted.

## checks
normalization_groups_context_and_rejects_bad_rows. See ../model.md#evidence for actual profiles and negative controls.

## epoch-memory

Model written from the existing mechanism before numerical repair. Source hash "(see History)". Finite witnesses constrain this relation, not a general proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30: model written from the existing mechanism before numerical repair. Source hash "0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554". Finite witnesses constrain this relation, not a general proof.
