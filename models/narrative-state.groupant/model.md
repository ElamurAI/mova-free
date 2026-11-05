# narrative-state

## model

# narrative-state

## forward
Tagged composition of the owned child mechanisms through shared identities and explicitly named external dependencies. Ownership is not a pipeline. The catalog records actual call edges and each unexecuted boundary.

## invariant
Use one compatible input/state/version at every actual join. Preserve event world IDs, endpoint maps, observer scope and frame ancestry for narrative queries. Scope each invariant to its tagged entry point.

## cost
For a trace of L commands, sum the reached world transitions and graph operations. Each graph path query costs O(V+E); a relation may clone graph state and issue several queries. Frame routing is O(depth), and event lookup scans reached worlds/events. Closure uses repeated bitset propagation. No measured runtime or universal trace bound.

## composition
narr executes accepted world commands and registers their Event into story metadata/time; story observations use the same frames, IDs and world. world/lang/types remain external dependencies. Closing a New frame copies events but not its entity state, retaining the event's child world ID.

## checks
Native parent checks live in world/tests/groupant_wave9.rs: narrative fixtures cover chain direction, relation rejection, closure and New-frame close. Other parent catalog views are source inspection only; child fixtures provide their own narrower evidence. The integrated manifest binds final child revisions.

## epoch-memory

First parent model. An initial fixture wrongly predicted that New-frame close hides its events; source inspection and a failing assertion showed merge_from copies events before the Past-only entity merge. The corrected test preserves child wid and separately checks entity isolation. No production bug was inferred. The integrated manifest binds the final child source bytes; native parent joins and child fixtures retain separate evidence scopes.

## behavior-complement
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: On valid accepted v3 command traces, the model excludes a strict temporal cycle, a relation command across distinct world IDs, out-of-order frame close, and treating copied child events as root-axis events. These are conditional inner exclusions of U minus R; partial mutation before a rejected registration and all other command effects remain outside the claim.
Type: behavior_complement. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## reverse-reconstruction
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: For observed order O and frame/world projection P, B(O,P) contains every accepted command history and graph producing those observations. Before determines neither exact edges nor elapsed duration; closed child events copied to the root do not determine whether entity states were merged. Reverse composition must retain identical event IDs, world IDs and frame ancestry.
Type: reverse_reconstruction. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## endpoint-registration
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: narr::register maps event ID e to Pt::S(e), Pt::E(e), records wid and installs S(e)<=E(e). Query order must use this endpoint map, never assume event number equals point index. Native three-event fixture checks all six directed distinct pairs and closure.
Type: encoding_relation. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## line-placement
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: An explicitly placed event is off the main narration line; an unplaced happened event follows the current line tail by a strict edge. Registration and causal-stamp pending edges share frames[fi].line. Source inspection establishes the branch distinction; only unplaced root line is executed by this harness.
Type: priority_relation. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## transaction-boundary
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: relate saves the graph before adding the endpoint edges for one temporal relation and restores it on contradiction. The native reverse-cycle command checks unchanged edge count and surviving transitive order. This local rollback does not imply rollback of every register or exec_v3 side effect.
Type: state_transition. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## frame-world-routing
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: world_frame climbs parent links until the nearest owned world or root. Now shares parent line/world; New allocates a new world ID; Past projects the parent's world while sharing its axis. Assumes finite valid ancestry. Native new-world fixture checks world_frame, world count and OtherWorld.
Type: scope_relation. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## close-projection
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: merge_from copies missing child events for every frame kind before returning early for non-Past. New-world close therefore exposes E2 at the root with its original wid, but does not copy C2 entity state. Parent inspection and native lookup assertions reject conflating event storage location with world identity.
Type: information_loss. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## knowledge-time
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: Story::known_since takes earliest eligible telling/explicit/reveal observation, whereas time.order is partial story order. Earliest reader knowledge sentence is not an absolute event timestamp. Source inspection only; observer combinations, claims and reveals are unexecuted here.
Type: observation_relation. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## reason-witness
Subject/revision: inspected child sources of this epoch. Domain, observation and operator: TimeGraph::path carries edge indices and strictness; explain suppresses Span and renders remaining Why origins. reachability observation loses alternate paths and edge histories. Reconstruct explanations only against the same graph and frames; source inspection, no explanation-text oracle in this harness.
Type: provenance_relation. Approximation: partial source-derived relation, conditional on the stated domain and shared identities. Evidence: native checks only where explicitly named above; otherwise source inspection. Unsampled semantics remain unknown.

## catalog

# narrative-state specialized models

## narrative-state-wave12-narrative-query-order

Type: temporal_anchor_transport.

A three-event chain queried in permuted order retains E1,E2,E3 and anchors 1,2,3 through QA3 sort and grading; reversed expectation fails. Literal grading agrees with QA, while expected none intentionally differs. Missing events and empty sort abstain. Arbitrary narrative metadata remains outside the fixture.

Wave12 source-derived view; the supplied domain and observation are above. Executed scoped integration evidence is recorded in world/tests/groupant_wave12.rs.groupant/model.md#evidence. Fixed dependencies and shared identities are premises; finite native joins do not establish full-pipeline behavior or recover unique input history.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first parent model. An initial fixture wrongly predicted that New-frame close hides its events; source inspection and a failing assertion showed merge_from copies events before the Past-only entity merge. The corrected test preserves child wid and separately checks entity isolation. No production bug was inferred. The integrated manifest binds the final child source bytes; native parent joins and child fixtures retain separate evidence scopes.
