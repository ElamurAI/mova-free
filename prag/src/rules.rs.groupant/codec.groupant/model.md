# prag-rules-codec

## forward
Feat lookup uses 46 names. Atom splits at first colon, retaining subsequent colons. Action lookup searches canonical frame/pronoun/token codes. parse_rules skips the first line, admits rows with >=4 tab fields and known action/all nonempty ampersand atoms, retains status verbatim, takes optional reason column7. Empty atom segments vanish.

## invariant
Codes/names are unique. Unknown feature/action rejects row; unknown status remains parsable. Empty conjunction is valid. say() is prose and no TSV escaping or byte-preserving serializer exists.

## cost
O(B+L*C) for B bytes,L rows,constant action catalog C; feature lookup scans 46 names. Space is retained atoms/strings; rendering linear in output bytes.

## behavior-complement
U=(text,candidate rule sequence). Including an unknown-action row, retaining first-line data, truncating value x:y to x, or reordering admitted records is excluded. Unknown status alone does not exclude a parsed row.

## reverse-reconstruction
Compatible file antecedents include arbitrary headers, skipped rows, ignored columns and empty atom separators. Empty, & and && yield the same empty conjunction. Canonical action code has unique recovery in its finite catalog; whole file bytes do not.

## specialization
All feature and action names round-trip; exact reason and extra-colon value tested. Parsing and status selection are separate nested stages.

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
