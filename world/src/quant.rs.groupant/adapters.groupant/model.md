# world-quant-adapters

Prompt/reply encoding, dataset readers, teacher batching and stats, including legacy tests.

## forward

prompt numbers sentences by ID; split_blocks overwrites duplicate IDs. load_set filter_maps valid records; load_done keeps last valid JSONL record per ID. vmm_run checks first replies, retries red records once and accepts retry only when green; stats counts green scripts.

## invariant

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

Teacher responses and I/O outcomes are external inputs. Green status derives from check; red retry retains original result. Durable exactly-once append is not guaranteed.

## cost

For positive batch, ceil(items/batch) chunks and at most one retry per processed chunk. Bytes and records govern parse/storage; latency and monetary cost depend on external results and scheduling.

## composition

Parsing feeds ordered execution; literal collection precedes evaluation against state, and check merges parse errors, execution diagnostics and gold inequality. Text extraction is an external exact dependency. Numeric-origin errors do not suppress effects, unlike evaluation and anchor errors; a parent must preserve this distinction. Dataset/teacher adapters do not establish script semantics.

## epoch-memory

Regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.

## checks

See ../model.md#tests and native quant::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

In adapter trace universe, red retry cannot replace an original as round-2 green. Exactly-once persistence and strict cap containment are not claimed absent failures; ignored write errors and concurrent calls remain possible.

## reverse-reconstruction

Saved script/round cannot recover discarded first scripts, failed responses or overwritten duplicate IDs. Retain all compatible prior records and reply preambles.

## retry-selection

Given first and optional retry observations, select retry iff present and errors empty, otherwise first. This is source inspection; no teacher calls were made.

## budget-coupling

Budget check and charge occupy separate mutex intervals; retry has no fresh check. Parent must retain in-flight spend, not infer future total solely from current spent.

## History

### 2026-10-30 · epoch-memory

2026-10-30: regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.
