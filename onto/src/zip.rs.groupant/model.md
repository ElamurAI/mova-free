# onto-zip

## descriptant

### onto-zip

open whole-file read, backwards EOCD within comment window, ZIP64 sentinel check, central signature/name/encryption records. names central order. read first matching name→local signature→payload slice→stored0/deflate8→size equality; other method refused.

## approximant

### Forward relation

open whole-file read, backwards EOCD within comment window, ZIP64 sentinel check, central signature/name/encryption records. names central order. read first matching name→local signature→payload slice→stored0/deflate8→size equality; other method refused.

#### Scoped equation

e=first entry name; start=e.off+30+local_namelen+local_extralen; raw=data[start..start+csize]. Require local magic; method0 copy or8 inflate else error; require |out|=size.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Invariant

CRC unchecked; no full bounds or central/local consistency checks. Malformed names/offsets can panic. Encryption bit1 refused open; unsupported method only read. Duplicate names first. Valid empty central accepted.

## Cost

O(archive bytes) retained; EOCD<=65557-byte window; entry/name scan; read lookup O(N), decode O(compressed+output). Declared size can allocate huge, no zip-bomb cap.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

## composition

### Composition

open whole-file read, backwards EOCD within comment window, ZIP64 sentinel check, central signature/name/encryption records. names central order. read first matching name→local signature→payload slice→stored0/deflate8→size equality; other method refused.

Fragment `entry-read` covers bytes [2524,3551); its equation contributes the indicated local relation. Other source branches/effects remain in the parent model; fragment does not inherit a whole-file proof.

Actual joins: wdx::Tsv emits rank-bearing unquoted schemas read by wdgraph and wdlex; wdgraph rebuilds local indices so wdwalk/rank_kinds must remap Q IDs. wdalign source masks serialize through src_names into wdwalk::n_sources_str (all16 categories checked). Wd uses first-step P31 even for present graph nodes, whereas wdwalk::run only adds P31 for absent graph Qs: these are distinct adapters. wdwalk candidates feed coresel confirmation; wdvmm parsing does not prove truth. wsd consumes model::Graph, kb::up/ancestors, query::lookup and core::of_concept; eval scores query::Answer rather than proof correctness. build::Builder consumes Zip::read; central/local offsets and method gates are not full ZIP validation.

## models

## behavior_complement

For in-bounds structures exclude success with encrypted central flag, unsupported read method, wrong decoded size. CRC corruption/duplicate names/panic on offsets not excluded.

## reverse_reconstruction

Payload/names admit different comments/CRC/extra fields/compression bytes. First duplicate controls lookup; decoded bytes cannot recover headers/method.

## header_join

Central offset/csize/size but local name/extra lengths determine payload.

## method_gate

0/8 only, match after slicing; open accepts unsupported method.

## name_projection

LossyUTF8 may collide, find picks first duplicate.

## failure_order

EOCD→ZIP64→central→name slice→encryption; read lookup→local→slice→method→size.

## tests

### Executed checks

Native command: `cargo test --offline -p onto --bin onto -- --exact --test-threads=1 zip::wave29_ontowiki::stored_archive_and_method_gate` (add `--release` before `--` for optimized profile). These exact names were included in the inspected selected command, twice per profile. All 21 sorted named outcomes matched across four runs. This source's compiled mutant returned exit101 with a failing assertion; exact source restoration followed by its passing test.

Tiny stored archive open/name/read, absent, method99 and encryption tested. Deflate/size mismatch/malformed offsets/FrameNet file unexecuted.

Syntax generated twice identically after restoring source. Evidence receipts: debug-1/debug-2/release-1/release-2 logs, outcomes and exit files; negative-zip, restored-zip, controls.json and native-commands.json in external wave29 ontowiki run.

No strict Clippy claim: inherited dependency/target warnings observed; no production logic changed, focused rustfmt applied to added test modules only.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Full source reviewed before changes; prospective predictions retained in the external wave29 receipt. Added tests preserve production behavior. Tiny stored archive open/name/read, absent, method99 and encryption tested. Deflate/size mismatch/malformed offsets/FrameNet file unexecuted. Registration is partial modeling, not semantic proof.
