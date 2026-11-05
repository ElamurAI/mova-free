# prag-rules-sites

## forward
sites emits frames first, then eligible coreference pronouns in frame/slot order (agent,patient,recipient,place,obliques), then unframed VBG/VBN amod/acl tokens on noun/proper-noun heads. frame_at maps predicate pairs to last index except sentinel; head_frame follows bounded ancestry. real excludes unresolved/personal IDs.

## invariant
Coordinates are not entity identities. Pron recognition lowercases surface pron; resolved entities remain eligible. Token/ancestry branches assume valid supplied trees. Public atoms/drop_frames retain stronger caller preconditions than apply_at.

## cost
O(F+O+T+F log F) scan, O(S+F) space, O=oblique count. head_frame at most T steps with O(log F) map lookup.

## behavior-complement
For three frames with She in frame2 patient and first oblique, only [Frame0,Frame1,Frame2,Ent(2,1),Ent(2,4)] is permitted in candidate sequence universe. Absent slots cannot produce pronoun sites. Token complement is source-derived only.

## reverse-reconstruction
Same sites permit many entity IDs, texts and frame kinds; pronoun coordinate does not recover antecedent. Empty sites constrains no frames/qualifying tokens, not necessarily an empty tree list.

## specialization
actions_for restricts by site class; direct apply_at now rejects invalid coordinates before cloning. Token site creation is not executed.

## composition
Codec feeds selection without reordering. Sites and action-dependent atoms share the pre-rule Read; ordered action fold sees evolving state, then deletion remaps coordinates. Selection nested in codec retains status distinctions. Exact byte scopes constrain composition; no child alone proves the parent.

## checks
Ten rules::wave15_models tests, tiny supplied fixtures only; two debug and two release passes with identical named outcomes. File-root ../model.md#receipts retains hashes. Finite catalogs, nine transfer states and sixteen deletion subsets; compiled production vacancy-guard mutant fails finite_transfer_and_drop_relations.

## epoch-memory

First rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.

## limitations
Partial syntax, no expanded macros or semantic AST proof. No live parser, teacher, corpus, environment mutation, override or cache acquisition in tests. Empty verbs/entities avoid lexical matching acquisition; populated entity tests use direct actions. Token creation, full lexical/ancestry/action branches, IO/cache lifetime and all multi-site interactions remain unexecuted. Public atoms/drop_frames still require valid coordinates/alignment.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.
