# translation-flow-bible

## descriptant

### translation-flow-bible

Book identity enumerates39 OT+27 NT+19 DC codes; split_of uses book%10 test3/8, dev5, otherwise train. vkey parses first three dot components and ignores suffix. read_pool reads main file then companion .verses, maps duplicate ids last-wins, joins sentences per verse in pool order with single separator when accumulator nonempty and drops trim-empty results.

## approximant

### Forward relation

Book identity enumerates39 OT+27 NT+19 DC codes; split_of uses book%10 test3/8, dev5, otherwise train. vkey parses first three dot components and ignores suffix. read_pool reads main file then companion .verses, maps duplicate ids last-wins, joins sentences per verse in pool order with single separator when accumulator nonempty and drops trim-empty results.

Scope: source-derived conditional transition model at the manifest SHA. VKey u8/u16/u16; known book codes case-sensitive, zero chapter/verse accepted. code outside0..85 yields ?. Numeric parse failure returns None. Filesystem/USFM decompression source-derived only; no real pool read.

## composition

### Composition

Scoped children are projections of this file, not independent executables. Main population filtering feeds aggregation; order/text helpers feed measure's exact decoder; many's agreement and storage read the same MTok with intentionally different punctuation treatment, streaming accounting consumes reader costs, clustering consumes normalized PairM. Uncovered branches remain source-derived; finite helper checks do not establish orchestration wiring.

## catalog

# Typed models

All entries bind the source SHA and optional byte range in manifest.json. Observation is one admitted helper call or the explicitly described orchestration trace. Relations are partial source-derived models, not universal proofs. Finite test names and unexecuted effects are in model.md#tests.

## behavior_complement

Unknown book labels cannot produce vkey; extra dot suffix cannot alter parsed first three components. Split membership depends only book index, never chapter/verse. Missing companion file prevents returned verse map.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## reverse_reconstruction

show restricted known-book keys has vkey(show(k))=Some(k); vkey preimages include arbitrary ignored suffixes and numeric spellings accepted by parse. Joined pool output cannot recover sentence boundaries or duplicate-id history without side information.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## split_period

Residues3 and8 test,5 dev over all u8 values; NT/DC continue global index rather than restarting split.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## coordinate_parse

Parse exactly first book/chapter/verse, accepts0 and rejects overflow above65535; trailing components are ignored, so this is not full identifier validation.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## pool_precedence

Companion BTreeMap resolves duplicate ids to last row; pool lines lacking tab or valid mapped key skipped, input order controls concatenated sentence order.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## compressed_order

usfm_files admits .usfm/.usfm.zst and sorts by filename without .zst; equal normalized names retain unspecified read_dir-derived tie order. read_usfm chooses decompression by final extension.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.


## tests

### Evidence and gaps

#### Native checks

- `bible::tests::split_by_book`

#### Controls

Five compiled source-behavior mutants: main gate counter suppressed; many strict chapter comparison relaxed; measure noncanonical-gap recording disabled; pages Meta filter reversed; blocks literal-change predicate reversed. All failed the corresponding assertion and passed after original bytes/hash restoration. Zero assertion-only mutants; zero surviving exploratory mutants.

#### Limitations

CLI argument/effect paths, live corpus/model/annotation/training, full Run::verse, page filesystem and reports remain source-derived only. Source syntax projections generated twice and byte-compared. No corpus-quality, persistent-learning, throughput or universal semantic coverage claim.

## invariant-and-cost

### Domain, invariant and cost

#### Invariant

VKey u8/u16/u16; known book codes case-sensitive, zero chapter/verse accepted. code outside0..85 yields ?. Numeric parse failure returns None. Filesystem/USFM decompression source-derived only; no real pool read.

#### Cost

Book lookup bounded85 comparisons; key parse O(input prefix bytes); pool O(B+Nlog V) memory O(input bytes+output). usfm_files sorts paths by stripped zst suffix O(Flog F); decompression proportional decoded bytes with unbounded external file resource risk, not exercised.

Integer additions/casts require representability; finite floating inputs where comparisons are modeled. These symbolic costs are not measurements of corpus throughput. No useful cheaper executable percentage surrogate established; future supported caps only 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective predictions saved before edits in the external wave32 tr_flow receipts. Original production bytes preserved; only appended synthetic test modules added. Selected actual helpers exercised; five compiled behavior controls rejected with exact byte restoration and positive reruns. Source syntax projection is partial, not expanded semantic Rust. Diagnostic reconstruction with residual is not semantic understanding or persistent learning.

VKey u8/u16/u16; known book codes case-sensitive, zero chapter/verse accepted. code outside0..85 yields ?. Numeric parse failure returns None. Filesystem/USFM decompression source-derived only; no real pool read.

### 2026-10-31 · tests

These selected names ran twice debug and twice release with exact filtering and one test thread; raw logs and numeric status files are in the external octopus-wave32-tr_flow-2026-10-31 receipt directory. All test bodies inspected before selection. Inline cases use only synthetic in-memory records; the existing blocks filesystem test was not selected. Tests listed at file level do not execute every function or this entire scoped fragment.
