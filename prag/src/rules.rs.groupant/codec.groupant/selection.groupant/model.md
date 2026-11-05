# prag-rules-codec-selection

## forward
select_rules(text,mode)=[] for off; otherwise stable filter parse_rules(text), keeping accepted or (mode=proposed and status=proposed). active returns an override unchanged first; absent override, OnceLock captures first environment/file selection with embedded fallback. Only pure selection is executed.

## invariant
Duplicates and file order survive. Unknown modes select accepted only; rejected/unknown statuses never survive. Override intentionally bypasses filter; environment changes after cached load do not recompute.

## cost
O(B+L*C) parse plus O(L) filtering; off O(1). Cache locking, IO and scheduling unmeasured.

## behavior-complement
For statuses [accepted,proposed,rejected,other,accepted] and IDs [z,a,b,c,z], U candidate selected IDs excludes off != [], default != [z,z], proposed != [z,a,z]. This does not constrain overrides.

## reverse-reconstruction
Selected [z,z] permits arbitrarily many rejected/unknown rows between them and different unknown modes. Off collapses every text to []; retain all compatible texts in inverse.

## specialization
File position is priority, not ID sort/dedup. Fixture deliberately contains z before a and repeated z.

## composition
Codec feeds selection without reordering. Sites and action-dependent atoms share the pre-rule Read; ordered action fold sees evolving state, then deletion remaps coordinates. Selection nested in codec retains status distinctions. Exact byte scopes constrain composition; no child alone proves the parent.

## checks
Ten rules::wave15_models tests, tiny supplied fixtures only; two debug and two release passes with identical named outcomes. File-root ../../model.md#receipts retains hashes. Finite catalogs, nine transfer states and sixteen deletion subsets; compiled production vacancy-guard mutant fails finite_transfer_and_drop_relations.

## epoch-memory

First rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.

## limitations
Partial syntax, no expanded macros or semantic AST proof. No live parser, teacher, corpus, environment mutation, override or cache acquisition in tests. Empty verbs/entities avoid lexical matching acquisition; populated entity tests use direct actions. Token creation, full lexical/ancestry/action branches, IO/cache lifetime and all multi-site interactions remain unexecuted. Public atoms/drop_frames still require valid coordinates/alignment.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.
