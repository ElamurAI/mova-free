# en-morph-codec

## descriptant

### en-morph-codec
Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.

## approximant

### Scoped forward relation
Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
No executable percentage approximant attached.

## invariant

### Scoped assumptions and costs
No general lemma/inflect inverse. Tree arrays aligned and conjunction ancestry acyclic required; non-self cycles may loop. Rules not linguistic proof.
Record scans/string candidates; tree adjacency O(n), conjunction walks up to quadratic or cyclic nontermination; u32 sums unchecked.

## composition

### Parent composition
Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent combines this fragment with the other named source fragments and helpers outside their ranges. No coverage claim for unscoped code.

## models

## behavior_complement
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: No general lemma/inflect inverse. Tree arrays aligned and conjunction ancestry acyclic required; non-self cycles may loop. Rules not linguistic proof.

## reverse_reconstruction
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: decode(encode(f,t),t)=f for terminating calls; decode preimages include Raw and inflection/case equivalents; lemma alone many-to-one.

## escape_roundtrip
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: Encode tests exact inflection then title then uppercase; Raw stores original bytes and bypasses morphology on decode.

## dictionary_ties
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: UD max_by_key chooses last equal count; reverse AGID chooses first compatible slot else first record, so different tie policies coexist.

## archaic_priority
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: ONLY_VERBS overrides even noun tag; remaining archaic irregular forms require verb/MD, known forms block ending rules.

## agreement_coordinates
Within this fragment's byte scope: Encode computes lemma and inflection, then exact/Title/Upper tests in that order; otherwise Raw carries original form. Decode Raw returns input directly; remaining cases apply inflection and chosen case. This composition preserves bytes when calls terminate.
Parent relation constraining compatible states: Tree heads are 1-based with0 root, child vector keeps token order; first nsubj wins; pronoun conjunction chain has no general visited set.


## tests

### Bounded parent evidence
See ../model.md#tests. Actual check: morph::wave27_tests::lossless_escape_and_features; shared source evidence, not independent exhaustive fragment testing.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-scoped partial model; parent tests provide finite shared evidence.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): source rebound to be759bd5cb5e; byte range 13688..14516 → 14166..14994. The scope text is unchanged (moved only).
