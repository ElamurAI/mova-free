# prag-silver-prompt

## model

# prag-silver-prompt

## forward
RULES then Sentences; Tale/Gum pos=0 emits paragraph header and ctx unless immediately preceding supplied Item shares src/doc and checked successor paragraph. Tatoeba header emitted once. Every input item gets sequential number/text.

## invariant
checked_add prevents MAX->MIN overflow/wrap. Predecessor is previous item, not corpus search; pos!=0 emits no context/header. Valid same-source adjacent paragraphs preserve original rendering.

## cost
O(N+B) time/output; O(1) predecessor state.

## behavior-complement
Missing ctx after cross-source collision and panic/wrap at i64::MAX are excluded after repair. Inserting ctx before every sentence is also outside relation. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
key,batch,ord,plen and suppressed ctx are lost. Multiple Item sequences yield identical prompts; no unique inverse. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Old code failed source-scoped Tale1/Gum2 context count and MAX/MIN adjacency witness. Equality on src plus checked successor repairs both with no teacher invocation.

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
