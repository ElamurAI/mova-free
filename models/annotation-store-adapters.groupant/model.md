# annotation-store-adapters

## descriptant

### annotation-store-adapters: wave31

Real joins: main->load/export/prag/qa/tales; load/export/prag/qa->Duck+Stage+Tsv; load/export/tales->en::conllu or tokenizer/annotator; util supplies SHA1/path/time. TSV write includes header while read expects COPY output without header: direct write/read is not identity unless header separated. Prag eight/nine-column input and QA eight-column input are distinct tagged schemas. Integration test fixtures exercise real CoNLL representation but SQL branches remain unexecuted. SQL include_str files are external dependencies, not selected Rust sources.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave31 report. Registration is not a universal proof or whole-system runtime claim.

## model

# annotation-store-adapters

## forward

CLI chooses one tagged command. load: CoNLL Doc -> identity match against prior state -> six TSV tables -> Duck exec -> post-count reconciliation. export: SQL TSV materialization -> raw Doc -> text. prag/qa: schema-specific TSV parser -> wanted identities -> state validation -> staged records -> exec -> postcounts. tales text segmentation is a separately tagged preprocessing path; production train/load path is not executed. Tests join supplied fixtures with actual Doc parsing, not a fake DuckDB interpreter.

## invariant

TSV NULL/empty encoding remains distinct within admitted cells; row/table identity references agree only under state snapshot and SQL/schema assumptions. Loader sentence identity is id AND text, QA identity is doc/para/k plus anchor doc/para. Exact export additionally needs file-only filter, one file row, preserved ordering/comments/blanks/newline. A post-commit error must not be interpreted as rollback.

## cost

Rust helper work is mostly linear in materialized bytes/rows with hash-map expected lookups, BTreeSet O(u log u) wanted identities, duplicate key suffix probing and ordered heading scans. Materialized Docs/TSV results occupy proportional memory. CLI startup, SQL execution, filesystem effects and annotation training/inference are separate costs; no throughput measurement or speedup claim.

## composition

Real joins: main->load/export/prag/qa/tales; load/export/prag/qa->Duck+Stage+Tsv; load/export/tales->en::conllu or tokenizer/annotator; util supplies SHA1/path/time. TSV write includes header while read expects COPY output without header: direct write/read is not identity unless header separated. Prag eight/nine-column input and QA eight-column input are distinct tagged schemas. Integration test fixtures exercise real CoNLL representation but SQL branches remain unexecuted. SQL include_str files are external dependencies, not selected Rust sources.

## domain

Finite CPU inputs; valid UTF-8 and representable signed IDs/counters; temp filesystem fixtures under isolated run TMPDIR; no production DB/corpus/jobs or annotator training. Database relations are conditional on real CLI/schema/SQL ordering and writer isolation, not established by this helper evidence.

## limitations

DuckDB executable absent from PATH and bounded known bin paths; no download/install, no mock subprocess SQL evidence. Source-derived transaction statements are not observed database guarantees. Tests inspected before selection. All production prefixes preserved byte-for-byte; additions are cfg(test) checks only. Partial syntax excludes macro expansion/name/type resolution.

## checks

- 16 exact target/name cases twice debug and twice release; sorted actual outcomes identical per profile
- 6 compiled production-behavior mutants assertion-red, exact restoration, positive green
- 12 source projections generated twice identically; 9 narrow fragments share parent projections
- Added blocks formatted separately; original production prefixes checked against base

## catalog

## behavior-complement

In the admitted helper trace universe, invalid TSV width/sentinel, invalid parser headers/k/anchors and invalid pre-dispatch CLI flags exclude successful adapter results. Database post-check failure does not exclude committed effects; helper exclusions cannot establish SQL rollback.

## reverse-reconstruction

Output text/table rows induce compatible-input/state relations across each join, preserving unknown IDs/provenance/unused records. NULL and empty are distinguishable in staged TSV, but bool spellings, stripped anchor whitespace, paragraph whitespace and overwritten CLI duplicates form many-to-one fibers. Exact database recovery requires ordered raw rows/comments/blank flags and external SQL assumptions.

## representation-transport

Tsv::row encodes Null as backslash-N, rejects sentinel string, preserves empty string and writes header at creation. COPY-to read has no header skip. JSON serializes anchor arrays and comments before TSV so control characters travel as JSON escapes.

## identity-coordinate-join

load matches (sent_id,text) against pre-state while qa maps doc_key then insists each anchor has unique sid with same doc and Some(para). Equal annotation bytes alone cannot identify a sentence; matching sent_id alone can admit an incorrect text.

## effect-frontier

Parse/duplicate/state checks and TSV validation precede write exec. Post TSV parsing/counts follow exec, and batch file loops preserve earlier effects after later failure. Partial TSV row buffers may contain prefixes even when count remains zero.

## tagged-independent-paths

Prag label schema, QA question schema, CoNLL load/export and tales segmentation are separate dispatched paths sharing transport utilities. One schema's parser success does not imply another's admission or DB table constraints.

## fixture-observation-limit

Existing DB integration tests are modeled but not run; new safe tests in those files exercise actual en::conllu Doc parsing and fixture layout only. They do not evaluate query strings, views, migrations or transactional behavior.

## cost-composition

Sum actual invoked helper costs plus process/SQL costs on the selected tagged path; do not add all CLI branches as if executed together. Full input/result materialization and sequential per-file DB calls remain, with no measured optimization claim.
