# rule-derivation-packing

## descriptant

# rule-derivation-packing

## Forward

scan_store captures latest version, selects ordered IDs, reads raw records and interns classes in first-encounter order. It copies UPOS/relation/head bytes and computes finite flags from tags. unpack creates synthetic S with empty forms, class-lemma projection, widened heads and decoded enums.

## Invariant

Equal Packed vector lengths, valid enum/class indexes and fewer than 65536 classes are assumptions. Downstream heads must be in range. Missing lemma IDs become empty strings, whereas malformed enum/class indexes can panic. Class IDs are dictionary-relative, not stable cross-run identities.

## Cost

W selected tokens and C classes: O(W log C + class-string bytes), plus selection/raw lookup. Unpack O(W+lexical bytes). Numeric vectors use 6 bytes/token plus one bool byte, vector allocation overhead and class strings. Seven bytes/token describes logical element sizes, not total retained or allocated memory.

## Composition

Closed-class lemma changes affect class and unpacked lemma; tag membership in VBD/VBZ/VBP/MD affects finite and potentially VERB root choice. Store identity is external. scan_store fixes hash then calls lemma_table(dir), which separately reopens latest; concurrent ref changes require compatible lexical tables. No transaction/snapshot guarantee follows.

## Epoch memory

Initial epoch: 90 assertions over twelve extracted pure source items with minimal enum dependencies. Closed-class erasure mutant rejected. No full learner, parser, GPU, corpus, filesystem-training or external state execution. Source-derived models and proposed oracles are distinct from executed bounded probes. See ../model.md#tests.

## catalog

# rule-derivation-packing typed models

## nonbehavior

Type: behavior_complement.

U consists of valid record/table projections and successful results. Inner absent behaviors include unpack recovering original surface forms, obtaining a nonempty lemma from a colon-free class, and finite=true for a decoded store tag outside VBD/VBZ/VBP/MD. Form/lemma cases executed; real decode_tag inspected only.

## antecedents

Type: reverse_reconstruction.

Given unpacked S with fixed dictionary/enums, Packed must preserve observed enum/finite/u16-head values; each class slot may be any slot whose split(':').nth(1).unwrap_or(empty) equals the lemma. Nonempty forms or heads exceeding u16 have no preimage. Multiple slots and suffixes share observations; original forms are unrecoverable. Probe witnesses ADP:x:y versus ADP:x:z and open-class collisions.

## lexical-projection

Type: information_loss.

Closed ADP/AUX/PART/SCONJ/CCONJ/DET/PRON map to POS:lowercase(lemma), others to POS. unpack retains only colon component one, so ADP:x:y yields x: class/unpack is not an identity for colon-containing lemmas. Missing lexical IDs yield empty suffix. Probe covers lowercase/open-class/colon collapse and to/of propagation; no full Unicode normalization claim.

## finite-projection

Type: state_projection.

scan_store finite is decoded-tag membership in VBD/VBZ/VBP/MD; parsed scan instead checks feature text for VerbForm=Fin. These are distinct dependency predicates. Finite=true admits four decoded tags, false all others; root preference further requires VERB POS. Proposed exhaustive real Tag/decode oracle not executed by stub probe; store sibling fixtures are separate evidence.

## widths

Type: admission_constraint.

New class gets classes_m.len() as u16. After 65536 classes, the next index wraps to zero and dictionary inversion overwrites a slot: injectivity needs the capacity assumption. Raw head little-endian u16 widens without validation; probe retains 65535. Book scan narrows parser heads to u16. Width recovery needs range constraints. Capacity-boundary allocation oracle remains prospective.

## History

### 2026-10-29 · document

Partial source-derived groupant, 2026-10-29. Full source inspected; this node specializes its explicit syntax envelope. Macro expansion and semantic AST are absent.

Subject: derive.rs SHA-256 2726211da9538f3f5c225292219dfda4d4fd6f15d6b68deaaaf53c834ba60842. Domain: finite admitted inputs with fixed dependency/schedule state. First type is a candidate inner behavior-complement bound in explicit input/output universe, second is an ambiguous compatible-preimage relation. Other models specialize observations or costs. See descriptant for assumptions and ../model.md#tests for evidence; structural acceptance is not semantic proof.
