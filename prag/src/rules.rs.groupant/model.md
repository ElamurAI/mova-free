# prag-rules

## model

# prag-rules

## forward
For supplied aligned Read R and Rule q, H=[s in sites(R) | kind-compatible(s,q) and q.atoms subset atoms(R,s,q.action)]. apply_rule snapshots H, folds actions over the changing Read in H order, collecting Drop indices, and remaps deletions last. Output is final Read and successful-site count. Multiple rules compose in supplied file order. active has process-global override and first-load cache; only its pure select_rules path is executed here.

## invariant
Frame coordinates and predicate pairs remain aligned on valid transitions. apply_at rejects missing/wrong-kind sites and mismatched frames/pred before cloning. Count is successful sites, not changed fields. Tests do not call active/set_override or lexical acquisition.

## cost
F frames, T tokens, S sites, Q quote edges, A rule atoms, H emitted atoms: site scan O(F+T+F log F); atoms rebuilds frame_at O(F log F), can scan F+Q and ancestry T. Rule envelope O(S*(F log F+F+Q+T+A*H)+action costs); apply_at clones O(Read bytes). No speed claim.

## behavior-complement
U contains supplied (R,q,candidate final Read,candidate count). Partial C contains candidates violating snapshot/fold: PatientToTo overwrites occupied recipient, Drop retains deleted roots, or invalid direct frame produces Some. External/lexical traces are unknown; finite tests do not prove all excluded transitions.

## reverse-reconstruction
B(final,count) contains all compatible (R,q) satisfying the forward fold. Fixed AgentNone maps otherwise equal reads with agents alpha and beta to identical agent=None output/count1. Drop loses entire removed frames. Reverse composition must preserve shared hit snapshot and remapping.

## specialization
Order intervention: Event followed by KindSay(Ask),KindEvent yields Event; reversed rules yield Say(Ask). Equal rule sets do not imply equal results.

## composition
Codec feeds selection without reordering. Sites and action-dependent atoms share the pre-rule Read; ordered action fold sees evolving state, then deletion remaps coordinates. Selection nested in codec retains status distinctions. Exact byte scopes constrain composition; no child alone proves the parent.

## checks
Ten rules::wave15_models tests, tiny supplied fixtures only; two debug and two release passes with identical named outcomes. File-root model.md#receipts retains hashes. Finite catalogs, nine transfer states and sixteen deletion subsets; compiled production vacancy-guard mutant fails finite_transfer_and_drop_relations.

## epoch-memory

First rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.

## limitations
Partial syntax, no expanded macros or semantic AST proof. No live parser, teacher, corpus, environment mutation, override or cache acquisition in tests. Empty verbs/entities avoid lexical matching acquisition; populated entity tests use direct actions. Token creation, full lexical/ancestry/action branches, IO/cache lifetime and all multi-site interactions remain unexecuted. Public atoms/drop_frames still require valid coordinates/alignment.

## receipts

### Wave15 rules evidence

Ten focused rules::wave15_models checks pass twice debug and twice release; normalized identities/results match. Initial old-code run: seven pass, invalid_direct_sites_are_no_change fails because an absent Drop returns Some. Compiled production mutation changes PatientToTo vacancy guard from OR to AND; finite_transfer_and_drop_relations and roles_transfer_preserves_fields_and_blocks_overwrite fail. Source restored byte-for-byte before all four positive passes.

Syntax projections are byte-identical across two fresh runs. Full structural reports pass twice identically with temporary root registration/binding restored afterward; root integration owns permanent registry changes. Structural checks do not prove semantics.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first rules tree. Review predicted unchanged valid-site transitions and O(1) validation before O(Read bytes) cloning. Old invalid_direct_sites_are_no_change failed because absent Drop returned Some; seven other initial tests passed. Repair rejects absent coordinates/unaligned reads. select_rules exposes the existing production filter; no selection policy change or speedup claimed.

### Undated · receipts

#### SHA-256 receipts

- old.log: 5256a7d999eb824e133af0e3580841587c817faa47479e2bc90688e3a50c568c
- mutant.log: ca726723fad1bf2ee67bc48bd527b7c6023950fe580f30f14315e9dca61f78d5
- debug-1.log: 21b497863ad2265cbca99942d8b0cbfe6c8f1872a66e2c31f4530acf8c1b4451
- debug-2.log: 96a6a5ac0a2743d1c2ceb926217f9575b9a6e97a7f6e2e1e5fc5a309b8678eb5
- release-1.log: c12bdeec48f71c2f0254cdb42f34a6c1c4e0936dd18e7452c6e047752cdd28cf
- release-2.log: c031968a13edb59a63973a71b0d04bfeb50a8a5a64a7c850eb1bb1e580911318
- normalized.txt: b65ea82b17ba8ec31566a176dc6366876fc780ed85566970ccf138a930936fb8
- syntax-1.json: f4181212c152500d980dd22fd04270ee09d0a1e839301e6c790ef34c8cbc931e
- syntax-2.json: f4181212c152500d980dd22fd04270ee09d0a1e839301e6c790ef34c8cbc931e
- full-groupants-3.json: 14c9726d9671c96d1f0c40158f9bab9f6c70860756d07d3792c1d30e4fa3cbf0
- full-groupants-4.json: 14c9726d9671c96d1f0c40158f9bab9f6c70860756d07d3792c1d30e4fa3cbf0
- rules.rs: ba82334c7c91aaaff7aed9651eae4b2d54b99d66c88cbe3f162fdda3407bc9f3

Evidence bundle: octopus-wave15-rules-2026-10-30 under the external runs directory. Source tree contains one file root and six fragments (codec includes nested selection). Each has forward/invariant/cost, concrete behavior complement, ambiguous reverse relation and specialized observation. Finite evidence, no live acquisition or performance claim.
