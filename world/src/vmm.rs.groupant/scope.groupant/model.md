# world-vmm-scope

## descriptant

### Sentence partitions, continuation summaries and physical-line repair

parts: for n<=max return [(1,n as u16)], including empty (1,0). Otherwise k=ceil(n/max), starts={i in 2..=n: para[i]!=para[i-1]}, from=1; for j=1..k-1 ideal=floor(n*j/k)+1, cut=earliest minimizer of |s-ideal| among starts s>from, falling back to ideal; emit (from,cut-1), set from=cut; finally emit (from,n), casting endpoints to u16. splice: split original into Rust str::lines; scan fixes in order; trim, require @ then first ASCII space, parse usize N and require 1<=N<=L; replace line N-1 with trim(command), increment count. Return joined lines with LF and count of accepted assignments.

## approximant

### Sentence partitions, continuation summaries and physical-line repair

parts: for n<=max return [(1,n as u16)], including empty (1,0). Otherwise k=ceil(n/max), starts={i in 2..=n: para[i]!=para[i-1]}, from=1; for j=1..k-1 ideal=floor(n*j/k)+1, cut=earliest minimizer of |s-ideal| among starts s>from, falling back to ideal; emit (from,cut-1), set from=cut; finally emit (from,n), casting endpoints to u16. splice: split original into Rust str::lines; scan fixes in order; trim, require @ then first ASCII space, parse usize N and require 1<=N<=L; replace line N-1 with trim(command), increment count. Return joined lines with LF and count of accepted assignments.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

Duplicate replacement indices are last-wins and every accepted assignment increments count, even if repeated or unchanged. Splice does not validate grammar, anchors or original rejection membership. parts can exceed max and fallback cuts can move backwards. world_brief includes ordered entity summaries, last 12 events and next-free IDs; body_v2 includes prior sentences/optional brief only when a>1, and task sentences whose 1-based numbers fall in inclusive [a,b].

Cost: parts scans n paragraph labels then scans candidate starts for each of k-1 cuts: O(n+k*S). splice holds original line strings and accepted replacements, O(original+fix+output bytes). World-summary cost depends on entities/events and formatting; prompt construction copies rule/example/story/brief bytes. No remote call occurs in this scoped range.

## composition

The parent call_v2 composes this partitioner/body builder with teacher output and parse_all, then repair_prompt/splice; call_v3 also reuses parts/splice but its body_v3 is outside this scope. Physical replacement acceptance is distinct from sentence anchoring and executor gates. Scope ends before call_v2, so no call-budget or cache-state transition belongs here.

## models

## behavior_complement

Splice cannot grow the number of original logical lines through a valid index assignment alone, rejects index 0 and indices>L, and cannot count ignored malformed fixes. It does not preserve the original final newline. parts has no universal maximum-size or monotonic partition guarantee. Prompt/repair formatting in this scope cannot invoke teacher transport or persist fixed files.

## reverse_reconstruction

For splice result [a,last,c] and count=2, compatible histories include original [a,b,c] with @2 first then @2 last; original overwritten middle content is unconstrained. Count=2 does not imply two distinct changed lines. Empty output can come from empty original plus ignored fixes. A partition [(1,2),(3,4)] admits many sentence texts and paragraph assignments; original prose cannot be recovered from endpoints.

## line_coordinate_overwrite

Line numbers refer to physical output lines, not sentence anchors. All non-replaced logical lines preserve content; accepted replacement text is trimmed. A valid @N assignment may replace a line that was not rejected by parse_all.

## partition_fallback

For n=10,max=3 with only section start 9, cuts are 9,6,9 and ranges become (1,8),(9,5),(6,8),(9,10). This source-derived counterexample invalidates a blanket contiguous-partition contract. max=0 with nonempty input divides by zero; u16 casts truncate larger indices.

## continuation_projection

world_brief summarizes current entity fields/relations/plans/beliefs, at most last 12 events, cursor and next-free IDs; it is a lossy view, not a serialization of world history. body_v2 prepends earlier sentences as context and limits task lines using [a,b].

## repair_prompt_transport

repair_prompt numbers every original logical line from 1 and lists supplied FormatErr entries, requesting @N replacements. The later splice parser trusts any valid N. Request wording does not enforce that returned edits correspond exactly to the error list.


## tests

vmm::wave28_dialogue::splice_coordinates_duplicates_and_invalid_rows exercises repeated @2, invalid 0/out-of-range/non-numeric/noise and empty original. parts_and_prompt_anchors exercises parts(4,2) and empty text; its body_v3 and estimate assertions lie outside this child. done_guard_precedes_teacher_and_partition is parent-only evidence. Pathological partition example is source-derived, not a newly executed case.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. The parent call_v2 composes this partitioner/body builder with teacher output and parse_all, then repair_prompt/splice; call_v3 also reuses parts/splice but its body_v3 is outside this scope. Physical replacement acceptance is distinct from sentence anchoring and executor gates. Scope ends before call_v2, so no call-budget or cache-state transition belongs here.
