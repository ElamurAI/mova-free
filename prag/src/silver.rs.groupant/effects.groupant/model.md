# prag-silver-effects

## model

# prag-silver-effects

## forward
batches groups borrowed refs preserving member order. run checks ceiling, selects missing raw outputs, writes prompt, asks teacher, writes .part then rename, appends call, gates. Error stop flag halts new scheduling but not necessarily in-flight work. collect reads raw outputs, serializes key/label and rejection tables; read_silver skips header and overwrites duplicate keys.

## invariant
Only batches executed. File presence is resume condition, not validation. Rename and call-log append are separate; collect's two writes are not transactional. No live teacher/run/collect/read_silver invocation.

## cost
Grouping O(N log K), O(N) references. Wrapper IO/teacher costs unmeasured; retained raw/output bytes depend on external responses.

## behavior-complement
Pure grouping cannot duplicate/omit a reference or sort members by key. No-retry loop is a source observation, not tested external recovery or durability. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Batch refs lose original interbatch interleaving without extra trace. Raw files cannot recover complete prompt or call log. Last-wins reading loses earlier duplicate rows. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Effect boundary explicitly retains unexecuted teacher, filesystem, concurrency and aggregation behavior; legacy wrapper is never run.

## composition
Children share source identity. Preserve frame indices through alignment/link mapping and batch references through prompt/gate numbering. Independent marginal inverses can lose shared correlations; overlap costs are counted once.

## checks
Six fbench and five silver wave16_models tests: finite canonical role oracle, independent ten-role counts, old-code witnesses and compiled controls. See receipts at file root.

## epoch-memory

Prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.

## limitations
Partial syntax is not expanded semantic AST; macros opaque. Scoped models and finite observations, not full-file verification or proof. No live parser, teacher, corpus, filesystem aggregate, GPU, full course, speed or approximation-cap claim. External effects and resource-extreme malformed inputs remain unexecuted.

## evidence

### Evidence scope

See root ../model.md#receipts for source and raw-log hashes. Pure supplied fixtures only; effects are source inspected.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.
