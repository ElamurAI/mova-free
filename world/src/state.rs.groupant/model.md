# hot-state

## model

### Bounded state reconstruction model

Scope: diff, apply, ancestry_path/walk, materialise ordering, and the ancestry
projections of story/other_branch. Remaining legacy behavior is unmodeled.
Filesystem atomicity, path validation, save ID collisions, current-state races,
frozen policy, brain inheritance and story contents are not certified.

#### Forward

For source lines a and operations o, decode starts at cursor 0.
A parsed unsigned count n consumes min(n, remaining length). Keep appends those
lines; drop discards them; insertion appends its payload without consumption.
Malformed counts consume zero; unknown prefixes do nothing. Unconsumed tails
are omitted. This is intentionally permissive parsing, not validation.
Cursor addition is bounded by |a| and cannot overflow.

diff builds a longest common subsequence table and chooses insertion on ties.
For feasible allocations and table counts within u32, apply(a,diff(a,b))=b.
Edit count is |a|+|b|-2 LCS(a,b). The deterministic delta need not be the
only minimal script.

ancestry_walk follows a fixed map from id to origin, returning leaf-first IDs.
Repeated IDs are errors. Strict mode rejects missing nodes; permissive mode
retains the first missing ID and stops. Origin terminates even with a map row.
materialise uses strict mode, then reads deltas root-first. story uses
permissive mode, propagating cycle errors. other_branch uses permissive mode,
returning None on cycle. Other branch selection and story formatting are opaque.
Successful materialise starts six empty files and folds file sections in order.
Duplicate sections update earlier output; unknown names are accepted. File
section behavior is source-derived, not independently executed in this epoch.

#### Composition

Validation precedes all materialise delta reads. With successful reads,
d1..dk gives Fk=DecodeFile(dk,...DecodeFile(d1,F0)).
The relation requires BOTH valid ancestry and this ordered fold. The complement
is relative to that conjunction, not a union of child complements.
Concurrent filesystem mutations violate the fixed-snapshot assumption.
Reverse composition retains all intermediate Fi compatible with adjacent deltas,
including arbitrary deleted lines. Fixed chain and deltas determine forward
output without making the transformation invertible.

#### Invariant

Decoder cursor lies in [0,|a|]. Only keeps and inserts append lines.
Accepted strict paths contain distinct existing IDs and end at origin.
Permissive paths may have one terminal missing ID. Bounds are |t| and |t|+1.
Metadata time/metric/note does not affect ancestry.
Encoder output retains target order and minimum insert/delete count.
These are source-derived claims with finite correspondence evidence.

#### Cost

diff allocates (|a|+1)(|b|+1) u32 cells and performs O(|a||b|) line comparisons;
string comparison/copy costs and allocation failure are additional.
Decode scans operation bytes plus copied output bytes and stores output bytes.
Ancestry takes O(h log |t| + h log h) ordered-map/set comparisons, O(h) IDs,
plus string costs. Cycle detection adds a set; no speed gain is claimed.
Materialise adds delta I/O and per-section base clones.
No executable approximate surrogate: a useful nonzero categorical error budget
for state restoration has not been established.

## tests

### Bounded evidence

Production state.rs contains three native unit tests. Each passed twice in
debug and twice in release with cargo test -p world --lib bounded_state_tests
-- --test-threads=1. The three normalized test-name/result records match across
all four runs. No process environment changes or live state reads/writes occur.
Compilation includes actual world module wiring; tests call the pure helpers.

- 961 ordered binary-line sequence pairs (lengths 0..4): exact roundtrip and
  minimum edit count against independent exhaustive subsequence enumeration.
- 9,317 line/script cases: seven source sequences of length 0..2 and all
  length-three products of eleven tokens. Independent iterator consumption
  checks valid, overlong, malformed, zero, empty insertion and usize::MAX counts.
- 125 three-node functional maps, five starts, strict and permissive modes:
  1,250 path/error comparisons against a bounded pigeonhole oracle. Explicit
  self-cycle and missing-node diagnostics also checked.

A source-extracted rustc test harness additionally compiles production diff,
apply, ancestry_path/walk and the same tests in both profiles. Two compiled
controls restore unchecked drop advancement or suppress keep output. Both must
fail in debug and release; these controls do not run the full world module.
The old arithmetic witness is source ["a"], operations ["-usize::MAX","=1"]:
unchecked addition can panic in debug or wrap and panic/misdecode in release.
Cycle behavior before the repair is source-inspected nontermination, not an
executed unbounded test. All cycle checks use finite pure maps after the repair.

Two source-bound syn syntax projections compare byte-identically. Projection
does not include macro expansion, name resolution or semantic proof.
Root plus two local fragments declare seven viewpoints each: 21 models, three
nodes, one Rust source. Count is inventory, not verification coverage.

## History

### 2026-10-30 · model

#### Epoch memory

2026-10-30 prospective prediction: clamping consumption preserves valid and
oversized non-overflow semantics while removing debug overflow/release wrap.
Repeated-ID rejection should bound malformed traversal. Independent iterator,
subsequence and finite graph oracles test these predictions; finite success
does not prove a universal claim. Raw cursor arithmetic previously overflowed;
ancestry cycles previously allocated indefinitely. Evidence is in model.md#tests.

### 2026-10-30 · tests

Heading `### Bounded evidence, 2026-10-30` stays in tests as `### Bounded evidence`.

External evidence uses the run directory named octopus-state-wave6-2026-10-30:
debug1/2.log, release1/2.log, matching .normalized files, check.log,
reference/cursor-mutant/keep-mutant debug/release logs and syntax2.json.
The standalone check.rs extraction driver is retained with those run artifacts.
No timing claim, full-suite claim, filesystem fixture claim or global safety
claim follows. The source-derived materialise fold and story/branch adapters
remain runtime coverage gaps; concurrent snapshots and filesystem effects are
outside the modeled universe.

Evidence SHA-256: source adc5fb462499c61d475c4e15b6b10d1dcadde7731dfb13078b69f9a34f0a4ade;
check.log 5e7315d50b71c3954d78fd3a32042a9e5b9958eff693d4dbdaa8f2b64e02dab6;
each native normalized record 3c4efa6a9eb389a32e498e2a820125368740b01bf29ad04e3ce2836dca6df63e.
Isolated inventory validation covers exact source/sidecar copies. Full repository
ancestor rebinding belongs to parent integration, not the isolated result.

### 2026-11-05 · public export

CENTRAL_MANUALS is gated by cfg(mova_legacy_worlds), which the new world/build.rs sets only when the feature worlds is on and peta/world/legacy/worlds exists; otherwise the empty table. A public checkout therefore compiles with --all-features. Behaviour with the folder present is unchanged.
