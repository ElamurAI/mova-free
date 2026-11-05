# world-store-order

## descriptant

# Shuffled blocks

## Forward

Scan metadata, cut every book boundary or a paragraph boundary after the size threshold. Hash sentence count, books-file length, first version and UTC day. Reuse matching cache or shuffle with xorshift Fisher-Yates, then write order.bin and order.tsv directly. Prefix emits whole blocks until its requested sentence threshold is reached.

## Invariant

Fresh blocks partition metadata sentence IDs. Reused order.bin is trusted and not validated; truncated data can panic. Cache identity excludes size and corpus content beyond coarse measurements. This is a daily order, not cross-day reproducibility.

## Cost

Fresh order scans n metadata entries and shuffles b blocks. Cached order reads b records. Prefix expands selected blocks and reserves the requested capacity, which can exceed n; bounded finite percentages are assumed for safe probes.

## Epoch memory

## catalog

# world-store-order: typed model catalog

Domain: finite valid byte files, bounded IDs and successful allocation; an operation's arguments plus its initial filesystem and scheduler state are inputs. Observations include returns/errors and written bytes. No simultaneous writer, path traversal, malformed offsets or crash is assumed unless explicitly modeled. Source-derived conditional relation, not a proof of all Rust executions. Evidence: ../model.md#tests distinguishes executed probes from proposed oracles.

## nonbehavior

Type: behavior_complement.

U is valid metadata and size x block list. U minus R contains a positive block crossing a book boundary or splitting a paragraph solely because size was reached. Candidate inner bound: book boundary always cuts; size only cuts at paragraph end. Probe explicit paragraph fixture.

## antecedents

Type: reverse_reconstruction.

For observed block spans, every metadata sequence whose internal positions avoid the cut predicate and whose block ends satisfy book_end or(size reached and para_end) is a preimage. Book and paragraph numeric labels are not uniquely recovered. Oracle: relabeling preserves blocks.

## partition

Type: conservation.

For n>0, blocks partitions [0,n) into contiguous positive intervals; total length n. Fisher-Yates permutes these intervals, so expanded valid order covers each ID once. n=0 gives no blocks. Probe sizes 0,1,2,400 checks sum, contiguity and expansion.

## budget

Type: resource_preimage.

Prefix wants ceil(n*pct/100) but emits whole blocks: for 0<want<=n, want<=out_len< want+last_block_len; want>n emits n. pct<=0 casts to zero; NaN also becomes zero; infinity may cause capacity failure and is outside admitted probe domain. Probe bounded percentages and overshoot.

## History

### 2026-10-29 · Epoch memory

2026-10-29: a 1% prefix over six sentences in two three-sentence books emits three IDs. The exact-budget false model is a negative control. A size change after cache creation does not trigger rebuilding.

### Undated · document

Every entry is a separate observation/constraint. Status: partial source-derived conditional model. Subject: world/src/store.rs at SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Reverse operators retain all compatible antecedents; complements are candidate inner bounds, not complements of an empirically fitted forward function. Proposed oracles are not executed evidence.
