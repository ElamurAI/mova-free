# prag-rules-actions

## forward
PatientToTo exists iff patient=p is Some and recipient=None, then patient=None,recipient=p with remaining fields fixed. ToToPatient is symmetric. Oblique promotions consume first entry (first by for AgentFromBy) only if destination vacant. Swap exchanges unequal slots. KindEvent changes Say only; KindSay changes non-Bare unequal kinds. Links write frame coordinates, Resolve copies candidate with original pron, token actions append Event/predicate.

## invariant
Promotion destinations are not overwritten. Surviving obliques retain order. AgentFromPatient intentionally overwrites agent. Drop is deferred. Not every action family is executed.

## cost
Role moves O(1) except clones; oblique removal O(O) shifts; SaysRemove O(F+Q); ancestry adds bounded traversal. apply_at adds Read clone and frame index build.

## behavior-complement
U=(patient,recipient,candidate outcome) over None,a,b. PatientToTo excludes success when recipient exists/patient absent, or resulting patient nonempty/recipient unequal to prior patient. For [(with,tool),(by,worker),(by,other)], AgentFromBy excludes choosing tool/other.

## reverse-reconstruction
Successful PatientToTo is invertible when both slots/remainder observed: prior patient=result recipient, prior recipient=None. AgentNone loses alpha versus beta. Oblique promotion loses consumed preposition/position unless side information retained.

## specialization
Nine patient/recipient states tested. Swap involution holds for successful unequal-slot domain; equal slots report no change. Compiled incorrect vacancy guard must fail the finite relation.

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
