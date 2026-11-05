# prag-silver-gate

## model

# prag-silver-gate

## forward
Ignore empty/fence lines; nonnumeric first field increments junk. Extra 0/>N or duplicate in-range number rejects entire batch before schema/voice decisions. Otherwise visit 1..N: missing/schema/voice errors reject that item, good labels enter ordered map.

## invariant
Batch rejection is one None bad entry and empty ok. Junk alone does not reject. Tale forbids voice none; unquoted Tatoeba forbids non-none; Gum has no voice restriction; quoted Tatoeba bypasses source voice gate.

## cost
O(B+N log N) time, O(B+N) row/error storage; schema parsing additional per-row text cost.

## behavior-complement
Duplicate-number acceptance is excluded, as is rejecting an unrelated valid item due only to missing neighbor. Treating Gum voice policy as Tale silently would change existing semantics. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Same accepted labels can come from permuted answer lines and ignored fences/junk; whole-batch failure can have duplicate, extra or both causes. Preserve reason/junk observations in fibers. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Error precedence: alignment ambiguity dominates schema/voice. Removing duplicate detection compiles and fails gates_partition_numbers_and_keep_junk_separate.

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
