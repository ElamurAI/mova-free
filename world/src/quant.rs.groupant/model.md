# world-quant

## model

# world-quant

Exact quantity grammar, rational state transitions, sentence-local literal provenance, answer gates and external adapters.

## forward

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

Fold parsed commands over (state, answer, errors). Invalid anchors skip. Extract numbers through largest visited anchor once; gate literals against their current anchor prefix plus IMPLIED, 0, 1 and reciprocal constants. Evaluate exact expressions, skipping on failure. set overwrites; arithmetic requires existing target; move debits and credits only with existing source; ask overwrites answer. check adds parse and gold errors.

## invariant

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

Exact rational state; ordered final keys. Green execution requires an ask and no diagnostics but does not prove semantic grounding. Same-key transfer is identity; distinct-key transfer conserves sum. Negative quantities are allowed.

## cost

L script chars, E expression nodes, C commands, N extracted numbers, H state keys, B evolving rational bit length: parsing O(L), literal gate O(E*N) rational comparisons, evaluation sum of rational operation costs at B. Prefix endpoints use O(Smax) integers; final state O(H log H). Recursion and allocations are not bounded.

## composition

Parsing feeds ordered execution; literal collection precedes evaluation against state, and check merges parse errors, execution diagnostics and gold inequality. Text extraction is an external exact dependency. Numeric-origin errors do not suppress effects, unlike evaluation and anchor errors; a parent must preserve this distinction. Dataset/teacher adapters do not establish script semantics.

## epoch-memory

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

## checks

See model.md#tests and native quant::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

Universe: parsed-command/sentence/result triples with fixed math dependencies and sufficient resources. Inner absent relation excludes green outputs with invalid anchors, undefined references, zero division or foreign literals; missing-source transfers cannot credit destinations. Malformed manually constructed Cmd and resource failure are outside this domain.

## reverse-reconstruction

For fixed sentences and observation O, retain every command sequence whose fold gives O. Answer-only fibers erase prior asks and overwritten state: set a; add b; ask yields 4 for all five a,b in 0..4 with a+b=4. Bounded fiber enumeration is complete only on that family.

## precedence

Multiply/divide bind tighter than add/subtract, each level left-associative. Unary minus is unsupported despite negative rational results. The 648-expression oracle independently rearranges the numerator.

## origin-prefix

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

At anchor j, accepted literal set is union of extracted numbers in sentences 1..j plus fixed allowances. A previous command at later anchor must not widen membership. Regression visits anchors 2,1,2 and detects one illegal 19 at anchor 1.

## diagnostic-effects

Foreign literal adds an error but can still modify state/answer. Invalid anchor or evaluation failure skips effects. check rejects any diagnostics, so an errorful run.state is not accepted knowledge.

## transfer-fiber

For distinct existing keys, fixed amount d maps (x,y) to (x-d,y+d), with inverse (x'+d,y'-d). Missing destination uses zero but its prior absence is lost; alias keys preserve x for every d. Sum alone leaves infinitely many rational antecedents.

## overwrite-loss

set erases prior target value/existence; ask erases prior answer. Converse retains all erased values compatible with earlier commands. Final state cannot identify overwritten command count or ordering.

## sentence-boundary

Split after .?! only before whitespace or end; $2.50 stays intact; trim each segment and keep unfinished tail. Whitespace information is lost. Four-segment fixture checks decimal and punctuation boundaries.

## reply-blocks

Trimmed === starts a block; preamble and code-fence lines are dropped; last duplicate ID wins. HashMap iteration supplies no ordering contract. Reverse reply fiber retains overwritten blocks and ignored text.

## external-budget

vmm_run checks cap before calls but retries and concurrent in-flight calls can exceed it. batch=0 panics, par=0 processes nothing. These are source observations only; no strict spending guarantee or live-call evidence.

## tests

### Bounded evidence

#### Initial measurement epoch

Run with MOVA_NOMEANS=1 MOVA_NOKINDS=1 MOVA_NOCLASSES=1 and cargo test -p world --lib wave8_models -- --test-threads=1; add --release for release. Synthetic mentions and prepared learner state avoid annotation, corpus, teacher and live state. The two existing quant::tests also pass.

Coverage: 648 rational expression evaluations against a rearranged independent formula; six malformed expressions and zero division; 162 transfer/alias cases over initial and amount 0..8; backward anchors 2,1,2; missing-source destination preservation; complete 25-pair answer fiber with five antecedents; decimal sentence boundaries and duplicate reply IDs.

Pre-fix native regressions failed: backward-anchor diagnostic count 0 instead of 1 and destination 9 instead of 7. Compiled production controls restored global seen membership and removed the missing-source continue; the respective named tests each failed, then source was restored.

Limits: finite CPU evidence does not prove all grammar/feature behavior, learner overflow safety, lexical correctness, persistent storage, teacher transport, budget adherence or full task accuracy. No speedup or numerical surrogate claim. Partial syntax projections repeat identically; structural checks validate bindings and references, not semantic correctness.

#### Artifact identities

Structural validation: an isolated Git-visible copy of this file and its complete sidecar tree passed mova-modelcheck groupants twice with identical reports and no diagnostics. Each root declares ten typed views and three fragments with four views each (22 views per file). Whole-repository registry and ancestor integration are delegated to the parent task; isolated success does not validate those inter-file bindings. Restored production sources passed all ten native tests after compiled controls.

#### Reference provenance scope correction

Follow-up source projection repeated byte-for-byte. Refreshed root and all three fragment hashes; existing fragment ranges remain valid because the new test follows their scopes. Isolated groupants reports repeat valid with no diagnostics; changed reports only world/src/quant.rs and no missing registrations. Full ancestor integration remains the parent task.

## History

### 2026-10-30 · epoch-memory

2026-10-30: regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.

### 2026-10-30 · tests

2026-10-30. Five native tests in quant::wave8_models; both files total ten.

Four focused runs (twice debug, twice release) pass all ten; sorted test execution uses --test-threads=1. Normalized named-result lines match byte-for-byte, SHA-256 dba383da848a3393a0766618919041d307b687a2a8b88d65b833bd9f77aa633d. Raw durations are excluded from identity comparison.

Source SHA-256: 3c192932d6786041d0d57962ec6736240e026985e99814429f99cabeffe16793.

Compiled prefix control raw-log SHA-256: ddba0f2bf24a1aebd5d15431fc265808b67e51abf31c5d6915727dfc8a142ad0.

Compiled transfer control raw-log SHA-256: 8af2d3983e0effd0afacb23dccd40c3c8bedca1938c3a59c43d8da11489e0536.

2026-10-30 follow-up: backward_anchor_references_use_command_order_state documents that references retain command-order state across decreasing anchors. Only literal membership is sentence-local. No production behavior changed in this follow-up. Six quant::wave8_models tests pass twice debug and twice release; normalized names/results match. Current source SHA-256: dd5cf73c5e8092c17566b87e0f6f465c3cdec5f1d87ac5a02ddc04c092987dbd. Earlier source hash and ten-test measurements above belong to the initial epoch.

Normalized six-test result SHA-256: 8daa317331c966e1080dd615414044901c9ac32240c8b5c093fcce936c43ddf8.
