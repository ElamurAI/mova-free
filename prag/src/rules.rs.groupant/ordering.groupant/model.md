# prag-rules-ordering

## forward
For deletion G and old i, m(i)=None if i in G else i-count(g<i in G). On aligned F-frame inputs only g<F affects map. Keep survivors in order; remap quote,cause,next,link,roots,asked and discard removed/out-of-range targets. apply_rule snapshots hit membership then ordered actions/deferred drop. apply_at validates then clones and returns Some for actual changes.

## invariant
Aligned frames/pred stay aligned. Empty gone is early return (does not clean malformed links). Aliases/discourse unchanged. Direct drop_frames requires alignment because zip otherwise truncates; apply_at/apply_rule protect this domain.

## cost
O(F log |G|+F+Q+asked+roots) remapping and O(F) map/rebuilt vectors. Hit snapshot O(S). Direct-site validation O(1) before O(Read bytes) clone.

## behavior-complement
U=(R,G,candidate R'). Retained deleted roots, omitted survivors, unshifted cause/link/asked, or retained invalid quote99 are excluded for valid aligned reads. Absent direct frame Drop returning Some unchanged is excluded by witnessed repair.

## reverse-reconstruction
Survivor labels [0,2,3] and roots [0,1,2] cannot recover deleted frame1 payload. Given G and old identity mapping, rank inverse is unique only for survivors; deleted edges admit many targets. Count alone cannot recover deletion positions.

## specialization
All sixteen four-frame deletion subsets check survivor/root order; a separate fixture checks every reference field. Stateful multi-site candidate cascades and malformed ancestry are not claimed tested.

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
