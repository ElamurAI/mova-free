# mlab-tests-suite-sha-helper

## descriptant

### mlab-tests-suite-sha-helper

Local SHA compression helper and abc self-check only.

## approximant

### Scoped forward relation

Pad message, schedule words, perform 64 rounds/block with wrapping add, hex encode8 words; self-check abc known digest.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

Append0x80 then zero until56 mod64 then 64-bit length.

#### Cost

O(B) compression and padded input copy, fixed64-word schedule; u32 arithmetic wraps intentionally and giant bitlength conversion remains unsupported.

## models

# Scoped typed relations

## behavior_complement

abc known digest excludes any changed output nibble for that input; this finite check does not exclude collisions or failures on untested messages.

## reverse_reconstruction

Digest collision/preimage ambiguity is not resolved by one known vector; output length is64 hex characters.

## padding

Append0x80 then zero until56 mod64 then 64-bit length.

## word_schedule

Words16..63 use rotates7/18 and17/19 with wrapping sums.

## compression

Eight working words fold into hash with wrapping addition.

## evidence

Only abc vector selected; not frozen-data hashes.


## tests

### Evidence scope

Parent selected tests: sha256_self_check, parse_error_runs_nothing, wave30_synthetic_suite_boundaries. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [647,3277); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
