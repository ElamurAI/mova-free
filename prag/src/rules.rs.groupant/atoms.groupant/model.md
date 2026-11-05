# prag-rules-atoms

## forward
frame_atoms projects kind, role presence, flags, links, previous kind, provenance and optional tree/lexicon fields. Pronoun atoms depend on proposed Resolve candidate. PrevAgent searches earlier frames backward; LineAgent uses frame0 for fi>0; Speaker uses nearest Say agent then before_speaker; Before uses number-compatible salience; Unresolve normalizes pron ID.

## invariant
Features can have multiple values (marks/adverbs); optional absence omits atoms. Candidate action execution sees evolving state although hits use pre-rule atoms. Lexical features may acquire global caches and are outside executed fixtures.

## cost
O(F log F) frame map per atoms call plus O(F+Q+T) scans and external lookup cost. Candidate previous search O(F), Before O(before length); clones cost entity bytes.

## behavior-complement
For empty-verb/entity Event fixtures, candidate atom sets lacking kind:event/has-agent:no or containing has-agent:yes are excluded. With earlier alpha,beta and She at frame2, PrevAgent result alpha is excluded in favor of beta.

## reverse-reconstruction
Kind/presence projections lose attrs, source and exact role identities. Events differing only in attrs share tested atoms. Same candidate ID can originate from different frames; output ID does not recover provenance.

## specialization
Resolve preserves original pron surface. Same candidate ID returns no change even when other metadata differs; this existing ID-based equality is narrower than full entity equality.

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
