# coref-mention-features

## descriptant

### coref-mention-features

Lexical/FEATS projection, modifier locality and mask precedence.

## approximant

### Scoped forward relation

Lexical/FEATS projection, modifier locality and mask precedence.

Parent transition model (restricted to this byte scope): is_head applies UPOS/role exclusions and optional idiom/compound switches. span_of traverses the head's allowed children then descendants; take first/last nonpunctuation/symbol and retain matching edge brackets. Coordination may emit full group and own span. build chooses pronoun table then ordered FEATS overrides, with number FEATS only when table number unknown; nonpronoun coordination forces PL and titles OR gender. Sort (sent,start,-end,head), dedup equal spans.

## invariant

### Scoped invariant/cost

Each detected valid-domain mention contains its head and has unique span after dedup; coordination and own spans may share head. Mask0 means unknown, not negation. Relative/interrogative are internal. No name-based gender inference and no gold lookup. Bounds are interval hulls and can contain unrelated tokens.

Conservative O(sum n³ + M log M + emitted text bytes): each head can traverse closure and each modifier can walk n ancestors; retained mention strings/features can be O(N²) bytes. Recursive graph walks use explicit stacks but require acyclicity; no dimension cap.

Scope: Lexical/FEATS projection, modifier locality and mask precedence.. Parent bounds are conservative; no separate timing measured.

## models

Scope: Lexical/FEATS projection, modifier locality and mask precedence.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

At default settings an expl pronoun, flat nominal child, title or excluded idiom cannot become an independent detected head. Output excludes duplicate spans, but not overlap or duplicate heads. Event helper creates Kind::Other SG/INAN and does not require ordinary nominal-head eligibility.

## reverse_reconstruction

For mention vector V, compatible inputs are all documents/environment snapshots yielding V after projection and span dedup; ignored Entity, unrelated comments and some syntax/FEATS can vary. Text lowercasing and removed title information prevent unique source reconstruction.

## feature_precedence

Pron type: Reflex > Rel > Int/IntRel > Dem > Ind/Tot/Neg > Rcp > Poss. Person1 > Person2 > Person3 overrides table. Known pronoun number is not overwritten by contradictory Number FEATS.

## coordination

For a nominal conjunct child, own span cuts direct conj/cc children; full span emitted only if different. Full group number PL; dedup by span after sort may discard a different head.

## span_geometry

Head-level cuts do not recur for lower descendants. Hull includes intervening positions; bracket edge retention requires an inside matching mate, not a general balanced-parentheses proof.

## modifier_transport

words retains content POS except titles; mods/proper_mods exclude flat name and paths through subordinate/appos/conj edges. head_key is last flat-name word, unlike head_low used by strict head sieves.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- tests::story_chains_and_reasons
- tests::wave27_coordination_mentions_and_visibility

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: No exhaustive closed-list validation or malformed/cyclic public documents; production rule accuracy is unmeasured.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::wave27_coordination_mentions_and_visibility. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-mention-compile.log, control-mention-red.log, control-mention-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
