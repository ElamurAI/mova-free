# qagen-select

## model

# Paragraph selection

## forward
Supplied ordered rows normalize to paragraphs; supplied paragraphs and filters produce weighted capped book quotas, balanced dialogue/narration pools, deterministic round-robin prefixes and source-ordered indices. See four scoped fragments. Live acquisition and loaders remain opaque I/O boundaries.

## behavior-complement
The composition excludes selection outside eligibility/caps/skip, cross-document context, floating remainder ambiguity and dependence on HashMap iteration order. It does not promise every requested count when eligible positive-weight capacity is insufficient. Duplicate keys may occur in supplied Para but indices stay distinct; skip suppresses every matching key.

## reverse-reconstruction
Relational converse composes compatible selected-index antecedents with pool/quota antecedents and normalized row antecedents. Metadata, rejected paragraphs, ord columns and licensing suffixes are generally unrecoverable; no right inverse or unique reconstruction claim.

## invariant
Finite ordered supplied inputs, 32/64-bit usize and representable allocations. q_i<=cap_i, sum(q)<=n; integer share floors sum<=left; shared-denominator remainders order exact. Each continuing clipped pass saturates an open key, so termination is independent of n magnitude. Selection cardinality equals quota sum for its own generated pools; selected indices strictly increase.

## cost
Let B=|size|, C=|cap|, P=paragraph count and R<=B+1. Quota initialization costs O(B log(B+1)+C) unit-comparison/scan work even when n=0; rounds add O(R B log(B+C+1)). Selection grouping/ranking adds O(P log(P+B+1)) comparisons; normalization adds linear row/text scanning. String comparison, key cloning and hashing bytes are additional costs. Temporary memory is O(P+B) entries plus owned/cloned text and key bytes. Internally generated selection caps have C<=B, while public quotas also accepts arbitrary cap-only keys. Exact u128 arithmetic replaces floating arithmetic for correctness; no performance improvement claimed.

## composition
Normalization feeds sentence counts, dialogue, words and context. Selection trusts supplied Para fields, so bypassing normalization need not satisfy derived-text invariants. Quota keys join pool book keys; cap construction bounds balance assignments. Two independent pool rankings merge only at final index sort. SQL ordering is a required external assumption, not verified by the pure fold.

## specialization
Observation vector: per-book quotas, selected index sequence, and normalized context strings. Exact source models with finite oracle evidence; no approximate substitution.

## checks
Six wave17 tests plus two existing select tests. Independent 45,056 three-book bounded tuples; large integer precision/overflow witnesses; synthetic context validation and balanced skip/round-robin checks.

## epoch-memory

Prospective model predicted exact Hamilton needed u128 products/shared denominators. Old code selected b0 instead of b1 for weights [2^53,2^53+1] with n=1, and overflowed cap sums for two usize::MAX entries. A hand-written capped expectation [1,5,2] was corrected to [1,4,3] after tracing two redistribution rounds; retain this as evidence against conflating iterative capped Hamilton with one global renormalization. Ordinary small outcomes agree with independent rational oracle. Private paras_from_rows enables pure source normalization tests; public API unchanged. Partial syntax omits macro bodies/name resolution/cfg semantics.

## evidence

### Native receipts

All inputs synthetic; no duckdb, corpus, teacher or external parser executed. Six new tests and two existing tests passed twice in debug and twice in release. Sorted named outcomes identical across all four runs. The independent small oracle checked 4^6 * 11 = 45,056 tuples using repeated subtraction for rational floor/remainder and repeated maximum scans for winner order. This is finite evidence.

Old precision and old overflow logs show assertion failure and usize sum panic respectively. Three compiled production mutants failed their intended assertions: reverse lexical tie order; take only one context sentence; omit skip membership. Exact positive bytes restored before four positive runs. The first draft capped expected result was incorrect, then corrected by tracing redistribution; first.log preserves that investigation. Existing prag unused_mut warning remained.

#### Commands

- cargo test -p qagen --lib select:: (twice)
- cargo test -p qagen --lib --release select:: (twice)
- mova-modelcheck syntax . qagen/src/select.rs NEW_JSON (twice, byte identical)
- mova-modelcheck groupants . EXTERNAL_SCAFFOLD NEW_JSON (valid, no diagnostics)

No benchmark or speed claim; suite elapsed times are not production timings. Partial syntax has 2,028 nodes, macro bodies opaque. Four fragments, five forward/invariant/cost scopes, fifteen typed obernants (five complement, five reverse, five specialization).

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model predicted exact Hamilton needed u128 products/shared denominators. Old code selected b0 instead of b1 for weights [2^53,2^53+1] with n=1, and overflowed cap sums for two usize::MAX entries. A hand-written capped expectation [1,5,2] was corrected to [1,4,3] after tracing two redistribution rounds; retain this as evidence against conflating iterative capped Hamilton with one global renormalization. Ordinary small outcomes agree with independent rational oracle. Private paras_from_rows enables pure source normalization tests; public API unchanged. Partial syntax omits macro bodies/name resolution/cfg semantics.

### 2026-10-30 · evidence

Heading `### Native receipts, 2026-10-30` stays in evidence as `### Native receipts`.

Source SHA-256: 0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554.

Raw evidence under $MOVA_DATA/runs/octopus-wave17-select-2026-10-30. SHA-256 receipts follow.

- prospective.md: `5ad9cf0df3aca1658776cb6b4e7c6932a7d12c0360f12de2e3c188d464175f6a`

- old-precision.log: `8eb59d80c79958c6493ca77f6aac093fd58cfa8597f5a4fb665eb90b2f576c14`

- old-overflow.log: `6445d5e5a73a69a0c90c4e7a0a2e8f947ceb34351f5f605a96047b91ab35f04c`

- mutant-tie.log: `271c9a14707201362028865c43c30028207e545cb31345e1d9c1e87e1c642767`

- mutant-context.log: `9d5111e78b01ac768d6dccdf972f278aab1167840b164054ae8d497d96cc1f11`

- mutant-skip.log: `f82992690e8cf6db7f5c32562d28a78267ebbb96630542469cc12242c3f29d06`

- debug-1.log: `9486a0dba1d6f4ad51a240123457b753da5bba49eeb88c76ae2f7302202097ab`

- debug-2.log: `9afafc23bbb2d8b6ae626f6cd5c295d4341847e090ec6c50b3bc8a2a8677b229`

- release-1.log: `3e962bca9ffbbe10568ea0e7fd1b7ae936a72fede8849a98ef535c4f7643c324`

- release-2.log: `16e5ff14a102dbdceea35bc8d9ab4a7c72a9149c7c1cf44fcff33e47a8be51eb`

- syntax-1.json: `e2ba9530f3630d3922e766245b6382c72efe1c9a18e570502fafa5bd05d32945`

- syntax-2.json: `e2ba9530f3630d3922e766245b6382c72efe1c9a18e570502fafa5bd05d32945`

- groupants-final.json: `bfb036cdcea6cea1e4188a02f3292303ec7152ca879f46d2ebf6ca9b71398ae4`
