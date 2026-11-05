# value-set-transition

## descriptant

# External causal set transition

## Forward

Freeze h=prior harmed entry exists, s=current agent belongs to prior harm history, p=current directed help pair exists, q=earlier reverse help exists. Eligibility requires a resolved event, character target and agent, and distinct identities. Ignore cause/goal/belief/name/trait keys. H means a harm field exists; R means a relief-but-not-harm field exists, preserving harm-first classification. Then dH=eligible AND H AND NOT s; dP=eligible AND R AND h AND NOT p; mercy=dP AND s; reciprocity=dP AND q.

Append the harm agent iff dH; append (agent,target,current line) to helped iff dP. These updates do not require principle rules to exist. Emit each judgment only if its rule exists. The first lexical qualifying key selects each explanation; merge the harm emission and help bundle according to their key positions. The help bundle orders relief, mercy, reciprocity.

## Invariant

Histories are monotone and unique per target/agent or directed help pair. New same-command harm cannot qualify as prior harm. Reachable harmed entries are nonempty; arbitrary fabricated empty entries are outside this model's domain.

## Composition

The parent records goals and self-achievement before this child's external-cause guard. This child freezes eligibility before ordered field traversal. Names/events supply explanations; global state classes and principle availability remain external dependencies. Reverse reconstruction retains every reachable prior history satisfying successor union/append and ordered emission equations. If only judgments are observed, successor history is existentially hidden; empty output can conceal rule-independent history changes.

## Cost

Snapshot membership costs O(H_x). A conservative bound for F fields and P helped tuples is O(F(H_x+P)), excluding string and rule lookup costs. Parsing and achievement are parent work, not additional child work. No timing or speedup claim.

## Epoch memory

## catalog

# Command-start set-transition typed models

Current revision is bound by manifest.json. Domain: one external causal set command, reachable histories, successful allocation, fixed global tables and finite nonoverflowing inputs. Source relations are partial; bounded correspondence does not establish universal semantics.

## nonbehavior

Type: behavior_complement. U contains reachable prior histories, a command and successor histories/ordered observations. Excluded transitions include new help without prior target harm, mercy without prior harm by this agent, repeated helped pairs and history changes under ineligible cause. This is a candidate inner bound on U minus actual behavior.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## antecedents

Type: reverse_reconstruction. For fixed command c and observation y, retain every reachable S with successor H'=H union {(x,a) if dH}, P'=P append [(a,x,n) if dP] and the ordered emission equation in the forward model. Projected output existentially hides successor state; missing rules may hide history changes. Preserve all compatible event/name histories and overwritten inputs. No unique inverse.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## command-snapshot

Type: snapshot_boundary. Freeze h=prior harmed entry exists and s=agent belongs to prior harmed entry. With eligible external cause, dH=Hfield AND NOT s; dP=Rfield AND h AND NOT priorHelp; mercy=dP AND s. New same-command harm cannot qualify help or mercy.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## lifetime-idempotence

Type: history_idempotence. Harm is suppressed once this agent is in prior harm history; help is suppressed once the directed helped pair exists. Repeating an identical eligible command stabilizes after at most two applications: an initially unharmed mixed command can harm first, help second, then emit neither. Relief never clears history. This child statement excludes achievement and other judgment branches.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## representative-selection

Type: lexical_minimum. K_H contains nonignored final fields classified harm. K_R contains nonignored relief fields not classified harm, preserving else-if precedence. The lexical minimum of each set supplies its explanation. Merge harm and help/mercy/reciprocity bundles by these key positions.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## field-permutation

Type: input_equivalence. Permuting distinct assignments preserves the final BTreeMap and complete ordered output under fixed prior state. Duplicate assignments preserve output only if their final values remain equal; changing final writes may change behavior.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## reciprocity-time

Type: temporal_precondition. A newly emitted help bundle includes reciprocity when an earlier reverse helped tuple (target,agent,l) has l<n and the rule exists. The just-appended tuple cannot satisfy the strict earlier-line condition.

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## single-field-refinement

Oracle: independent finite snapshot probe and source tests; see ../../model.md#current-evidence. Overlapping global classes and absent principles remain source-derived.

## tests

### Checks

The file-root set-transition-probe.rs.txt includes actual values.rs and enumerates reachable histories, two field assignments, identity/cause gates and token orders. Expected judgments use closed-form eligibility bits, literal field representatives, and complete output tuples rather than a copy of the production loop. See ../../model.md#current-evidence for execution counts, dependencies and negative controls.

The reverse relation is specified but no executable general inverse is supplied. Absent-principle behavior and overlapping global classes remain source-derived. Finite correspondence does not establish every command trace.

## History

### 2026-10-29 · Epoch memory

2026-10-29: the historical mixed-field witness showed differing membership, even after sorting. Canonical order alone would select one accidental within-command causal interpretation. Frozen command-start history repairs that boundary while preserving lifetime suppression. Syntax is a current source-bound partial projection; semantic AST analysis and universal correspondence remain absent.

### Undated · single-field-refinement

Type: restricted_refinement. Against historical source f450843ba1c92f35884eb71667d432f36a34f6316c9b8a252811e0dac87ddbe6, at most one classified nonignored state field preserves this child's harm/help membership and explanations under fixed prior history/tables. Canonical goal-peer order is outside this child. Mixed-field equivalence is not claimed.
