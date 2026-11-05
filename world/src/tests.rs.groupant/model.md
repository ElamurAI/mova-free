# world-tests

## descriptant

### world-tests

text generates numbered sentences; line requires one accepted command; world parses all commands then applies sequentially with panic on first rejection. gate_of requires Err and returns its Gate. Selected checks cover strict format, rollback after bad have/give, immutable versions after later deltas, and ordered/alternative verdict matching. Other v1/v2 examples and natural-language reader cases remain unexecuted. Observation relation for rejection: O(apply(W,c))=O(W) for the tested rejected c, O=(character versions,event count,links-valid). Historical relation snap(W_after,C1,v_saved)=snap(W_before,C1,v_saved).

## approximant

### Forward relation

text generates numbered sentences; line requires one accepted command; world parses all commands then applies sequentially with panic on first rejection. gate_of requires Err and returns its Gate. Selected checks cover strict format, rollback after bad have/give, immutable versions after later deltas, and ordered/alternative verdict matching. Other v1/v2 examples and natural-language reader cases remain unexecuted. Observation relation for rejection: O(apply(W,c))=O(W) for the tested rejected c, O=(character versions,event count,links-valid). Historical relation snap(W_after,C1,v_saved)=snap(W_before,C1,v_saved).

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

Version comparisons are value equality, not pointer identity; format failures precede execution. New current snapshot differs while historical snapshot/delta stays equal.

## Cost

Finite fixtures plus delegated parse/apply/snapshot/link costs; no timing/scalability claim.

## composition

### Composition

Scoped children: rollback, format. text generates numbered sentences; line requires one accepted command; world parses all commands then applies sequentially with panic on first rejection. gate_of requires Err and returns its Gate. Selected checks cover strict format, rollback after bad have/give, immutable versions after later deltas, and ordered/alternative verdict matching. Other v1/v2 examples and natural-language reader cases remain unexecuted. Observation relation for rejection: O(apply(W,c))=O(W) for the tested rejected c, O=(character versions,event count,links-valid). Historical relation snap(W_after,C1,v_saved)=snap(W_before,C1,v_saved). Other functions and test additions are modeled only at this parent. bAbI Story and command World have different representations: command-language fixtures do not certify the UD parser adapter. StepGame lexical/load helpers connect directly to tests03; CLI helpers construct a separate world process but this wave does not spawn it. No composition-wide percent accuracy follows.

## models

## behavior_complement

Selected fixture relation excludes successful mismatched have C2 O1, mutation of an old C1 snapshot, accepting duplicate to= fields and matching fox>crow against crow then fox evidence. These finite assertions do not prove all states.

## reverse_reconstruction

Passing rollback constrains observed character versions, event count and link validity for two rejected commands, leaving unobserved fields unconstrained. Verdict Hit admits any answer containing the matched ordered alternatives; original question cannot be reconstructed.

## rollback_projection

Reject test compares character version vector/event count and check_links, not every World field.

## immutable_history

Saved state equals old state after changes; latest delta omits redundant Type/Name.

## ordered_verdict

crow>fox hit, fox>crow miss; comma conjunction and bar alternatives differ.

## execution_boundary

qframe/annotator/read paths require model; stored Wen derives from held-out data. Exact selected names exclude them.

## tests

### Bounded checks

Named checks associated with this source (not every fragment branch):

- tests::format_is_strict
- tests::rejected_command_leaves_world_unchanged
- tests::old_states_stay_immutable
- tests::verdict_respects_order_and_alternatives

17 unique named tests in total passed twice debug and twice release; sorted named outcomes equal across all four runs. Four compiled controls cover bAbI identity, StepGame tie ordering, parser-command state removal and bench-command CPU selection; each exits101 at a runtime assertion, exact source restored then positive exit0. Other sources have finite adversarial fixtures but no separately compiled mutant.

CLI integration coverage is command construction and temporary cleanup only; legacy school/model/cadence paths are not executed. No speed benchmark, GPU, live teacher, datasets or held-out replay.

Limits:

- Only four synthetic tests executed; no model, stored VMM replay or held-out fixtures
- Finite assertions not executor proof; executable percentage approximant unsupported

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Read full assigned source before editing; prospective predictions retained in external wave29 receipts. Stored/live/parser-dependent test paths are excluded from execution. Evidence is limited to named bounded fixtures; registration is partial modeling. Production behavior unchanged; only inline synthetic tests added to four assigned sources.

### 2026-10-31 · tests

External receipt root: $MOVA_DATA/runs/octopus-wave29-worldeval-2026-10-31.
