# world-store-lookup

## descriptant

# Version lookup

## Forward

open captures meta, version rows and main. records binary-searches floor(dict bytes/16) entries, reads an exact byte extent, and traverses parent links on miss. text reads positional byte ranges and performs lossy UTF-8 conversion. lemma_file fills sparse indices, treats malformed index as zero and lets later rows overwrite.

## Invariant

Nearest explicit match wins, including zero-length records. No sortedness, cycle, metadata-tail or enum-range validation is promised. The selected syntax envelope includes all Store methods; decoder correspondence remains outside the probe.

## Cost

Cold dictionary storage and reads are linear in visited dictionary bytes; warm lookup is logarithmic per dictionary plus linear version-parent searches. Each store retains its cache indefinitely.

## Epoch memory

## Composition

The identity child resolves numeric lemma IDs before comparing the stored projection, while the lexical child projects effective records into feature counts. Record equality is finer than feature equality; unchanged features cannot recover tags or complete words. Parent lookup must preserve inherited ID meanings across child tables. Dense unique canonical tables and representable IDs are explicit conditions. lemma_table_for uses the exact version table or the base table, without an ancestor-table walk. No automatic converse composition or whole-update proof is claimed.

## catalog

# world-store-lookup: typed model catalog

Domain: finite valid byte files, bounded IDs and successful allocation; an operation's arguments plus its initial filesystem and scheduler state are inputs. Observations include returns/errors and written bytes. No simultaneous writer, path traversal, malformed offsets or crash is assumed unless explicitly modeled. Source-derived conditional relation, not a proof of all Rust executions. Evidence: ../model.md#tests distinguishes executed probes from proposed oracles.

## nonbehavior

Type: behavior_complement.

U is finite sorted well-formed acyclic version stores x lookup outcomes. U minus R includes returning a later ancestor's payload bytes different from the nearer matching payload and latest ignoring a captured nonempty main. Candidate inner bound; payload equality cannot reveal lookup provenance. Probe builds conflicting records.

## antecedents

Type: reverse_reconstruction.

For observed records r at (h,i), all chains with nearest matching dictionary entry pointing to r and every earlier dictionary omitting i are compatible. Arbitrary farther ancestors and unrelated IDs remain free. Empty records can be a genuine zero-length entry, not absence. Probe tests zero-length and inherited records.

## termination

Type: reachability_precondition.

Miss traversal terminates on absent parent or '-' in a finite acyclic chain. A parent cycle with no match loops; no visited set/depth cap exists. Invalid dictionaries can return wrong misses because sorted IDs are assumed. Probe tests finite miss only; cycles intentionally not run.

## cache-cost

Type: resource_preimage.

For V visited versions, cold bytes D and returned L bytes, records costs O(D + sum(log n_v) + V*m + L), m=versions row count due to linear parent search; warm calls avoid D. Cache retains all loaded dictionaries/handles without eviction. Oracle proposed: instrument file opens; no timing claim.

## History

### 2026-10-29 · Epoch memory

2026-10-29: fixed main and meta snapshots coexist with lazy file opens; a handle is not a globally atomic snapshot. Even append-only publication can interleave open's versions read and later main read: captured old rows plus a new delta main can fail inherited-record fallback. That scenario is source-derived and not executed. Successful nearest-match claims require a coherent captured chain; equal ancestor/child payloads are observationally indistinguishable.

### Undated · document

Every entry is a separate observation/constraint. Status: partial source-derived conditional model. Subject: world/src/store.rs at SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Reverse operators retain all compatible antecedents; complements are candidate inner bounds, not complements of an empirically fitted forward function. Proposed oracles are not executed evidence.
