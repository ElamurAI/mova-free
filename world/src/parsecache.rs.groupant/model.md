# parse-cache

## descriptant

# parse-cache

## Forward

Cache state is (key,path,T,refresh,hits,misses,repaired). open computes a dependency identity then reads or discards the envelope; parse counts input occurrences, parses missing occurrences, validates batch cardinality, overwrites by text, and returns final-map lookups. finish rechecks identity before conditional publication.

## Invariant

Successful parse outputs preserve input cardinality and text order. Parser failure preserves the tree map but not counters. Invalid envelopes cannot populate it; repaired does not imply disk repair has already occurred.

## Cost

Identity costs total hashed dependency bytes. For n input occurrences, u cached keys, k missing occurrences and parsed/output Word size W: lookup/insert O((n+k) log(u+k)) string comparisons plus parser cost and cloning O(W); hashing/serialization O(serialized map bytes). BTreeMap comparisons include text length. Write clones T and serializes it twice, increasing peak memory by O(T bytes).

## Composition

Identity, batch and envelope children share exact key and tree-map serialization. Composition is open -> zero or more parse calls -> finish; each successful parse updates T, failed attempts retain T but advance counters. The root alone covers open/refresh/repaired state and finish coupling. Children overlap only through called functions; do not add their costs to a root cost that already includes them. No claim of complete inter-file parser dependency coverage.

## Epoch memory

## catalog

# parse-cache typed models

## nonbehavior

Type: behavior_complement. Status: partial_source_derived_model.

Domain and observation: U is the set of one-call (cache state, text sequence, parser outcome, returned outcome, final map/counters) tuples; R is parse_with on that state.

Operator and approximation: Candidate inner approximant C subset U minus R: successful return with cardinality unequal to texts; invoking parser on an all-hit non-refresh batch; inserting a partial parser result; counting a cached occurrence as a miss with refresh=false. Counter effects on Err are intentionally not excluded.

Assumptions: Normal termination, no mutation from outside the cache; parser effects are opaque.

Evidence and falsifiable oracle: Finite batching oracle and existing partial-parser test; mutation changing refresh disjunction to conjunction must fail. See model.md#tests for executed bounds versus prospective work.

## antecedents

Type: reverse_reconstruction. Status: partial_source_derived_model.

Domain and observation: Observe successful output Y for a fixed ordered text sequence X; previous map T, refresh r, and parser answers P are unknown.

Operator and approximation: Preimage is all (T,r,P) whose missing occurrence list M=[x in X : r or x notin dom(T)] and last-write map T'=T overwritten by zip(M,P) satisfy len(P)=len(M) when M nonempty and Y_i=T'[X_i]. Preserve every unconstrained prior key and every overwritten earlier duplicate parse; no unique parser/input recovery follows.

Assumptions: Pure finite Word values; opaque parser may return different results for identical texts.

Evidence and falsifiable oracle: Bounded probe enumerates two unknown cached values and two per-occurrence parser values, checks exact fibers for fixed duplicate text sequence. See model.md#tests for executed bounds versus prospective work.

## dependency-key

Type: identity_projection. Status: partial_source_derived_model.

Domain and observation: Executable/model bytes, latched execution mode, whitelisted environment, optional files and GPU dependencies at identity() call.

Operator and approximation: K=SHA256(JSON(BTreeMap(parts))). Parts always include schema, executable digest, en-model digest and debug mode; GPU adds six named files and driver bytes (unreadable driver becomes empty). Only listed environment values and optional file contents enter. Unread fields are outside this declared dependency projection; no global parser-completeness claim.

Assumptions: Filesystem reads succeed except specified optional absence/driver fallback; stable env; SHA collision resistance only when treating unequal content as unequal identity.

Evidence and falsifiable oracle: Prospective isolated CPU/GPU dependency perturbation matrix; current probe does not call identity. See model.md#tests for executed bounds versus prospective work.

## optional-fibers

Type: absence_projection. Status: partial_source_derived_model.

Domain and observation: optional_hash(path) on metadata/read outcome.

Operator and approximation: NotFound metadata maps to literal absent; other metadata errors propagate; existing metadata proceeds to streaming hash and can still fail. All nonexistent paths share a fiber; existing equal-byte files share digest fibers, independent of their path spelling.

Assumptions: No concurrent create/delete between metadata and open; digest equality does not prove equal bytes mathematically.

Evidence and falsifiable oracle: Probe checks two absent paths and two existing equal-byte paths; missing-to-present boundary. See model.md#tests for executed bounds versus prospective work.

## occurrence-ledger

Type: multiset_accounting. Status: partial_source_derived_model.

Domain and observation: Pre-call counters h,m and X of n occurrences, not distinct strings.

Operator and approximation: k=sum_i [refresh or X_i notin dom(T)] is computed before any insert. Counters become (h+n-k,m+k) before parser is called and remain advanced on Err. Missing order is the original occurrence order.

Assumptions: No usize overflow, including accumulated historical counters.

Evidence and falsifiable oracle: Exhaustive length-0..3 batches over two texts, all initial key subsets, both refresh flags; parser-error and partial-result controls. See model.md#tests for executed bounds versus prospective work.

## duplicate-overwrite

Type: last_write_fiber. Status: partial_source_derived_model.

Domain and observation: A successful batch with repeated missing text t and position-dependent parser output.

Operator and approximation: Final T'[t]=P_j at the last occurrence j of t in missing; every returned occurrence of t clones that final value. Earlier P_j for the same key are unconstrained antecedents of the observed output. Missing entries are not deduplicated.

Assumptions: Parser returns exactly one Word vector per missing occurrence.

Evidence and falsifiable oracle: Probe returns different head markers for duplicate occurrences and enumerates all marker fibers; a unique-key miss-count hypothesis is rejected. See model.md#tests for executed bounds versus prospective work.

## error-memory

Type: failure_partition. Status: partial_source_derived_model.

Domain and observation: parse_with failure after accounting, before insertion.

Operator and approximation: Parser Err or Ok with wrong batch length leaves trees unchanged while adding n-k hits and k misses. A normal successful complete result inserts all zipped entries; no per-item validation rejects malformed tree heads. Parser external side effects are unconstrained.

Assumptions: No panic/abort; iterator operations and clones finish normally.

Evidence and falsifiable oracle: Probe compares complete tree maps and counters after explicit parser Err and short Ok batch. See model.md#tests for executed bounds versus prospective work.

## refresh-policy

Type: counterfactual_policy. Status: partial_source_derived_model.

Domain and observation: Fixed X,T,P comparing refresh=false and true in parse_with; open_key refresh at construction.

Operator and approximation: Refresh open skips reading even invalid existing bytes and starts empty, repaired=false. In each later refresh parse every occurrence is missing, including keys inserted earlier; successful parsing overwrites those keys while preserving keys not requested since this cache object opened.

Assumptions: Same parser observation when comparing policies; no filesystem effects from parse_with itself.

Evidence and falsifiable oracle: Probe repeated calls in refresh mode plus preexisting corrupt envelope; no repair is reported during refresh open. See model.md#tests for executed bounds versus prospective work.

## envelope-admission

Type: acceptance_preimage. Status: partial_source_derived_model.

Domain and observation: Serialized input bytes b and requested key k at decode.

Operator and approximation: Accept iff serde deserialization succeeds, schema=1, identity=k and checksum=H(JSON(deserialized trees)). Inverse contains every accepted JSON representation (whitespace, member order, accepted unknown fields) inducing the same tree map; checksum binds canonical reserialization, not raw input bytes.

Assumptions: Uses current serde Word representation; checksum is unkeyed, so deliberate edits with a recomputed checksum are accepted.

Evidence and falsifiable oracle: Probe admits whitespace/unknown fields and rechecks mutated checksum/schema/key; existing Word test covers high feature bit and POS tag. See model.md#tests for executed bounds versus prospective work.

## open-repair

Type: first_failure_partition. Status: partial_source_derived_model.

Domain and observation: open_key(root,k,false), assuming directory creation succeeds.

Operator and approximation: Missing file yields empty map/repaired=false; successful read plus decode failure yields empty map/repaired=true; read errors other than NotFound propagate. Decode rejection does not delete or overwrite invalid disk bytes. Key k is a synthetic safe basename in probes; public open uses digest keys.

Assumptions: Stable isolated directory; no path traversal in caller-provided synthetic keys.

Evidence and falsifiable oracle: Probe missing/valid/corrupt cases and compares corrupt bytes after open; unreadable-file OS matrix remains prospective. See model.md#tests for executed bounds versus prospective work.

## finish-barrier

Type: publication_precondition. Status: partial_source_derived_model.

Domain and observation: finish on a cache state, environment and dependency read outcomes.

Operator and approximation: Identity is recomputed even with misses=0. Error or unequal identity rejects before write; equality and misses>0 invokes write; equality and misses=0 leaves disk untouched. End-point equality cannot detect an intermediate dependency change followed by restoration and is not a filesystem snapshot.

Assumptions: No concurrent dependency changes during identity call; trusted file contents; identities may collide.

Evidence and falsifiable oracle: Prospective isolated environment dependency-change fixture; no live parser/model fingerprinting executed. See model.md#tests for executed bounds versus prospective work.

## publication-protocol

Type: persistence_relation. Status: partial_source_derived_model.

Domain and observation: write with in-memory T,key and delegated store::write_atomic.

Operator and approximation: Envelope is (1,key,H(JSON(T)),T); serialize then write a same-directory temporary path with process-id extension and rename to final key.json. Conditional success makes decode recover T. Publication replaces the entire snapshot: two stale cache objects writing sequentially lose the first object's additions rather than unioning maps. Crash durability/fsync and concurrent same-process temporary-file races are outside this relation; equal key does not authenticate an untrusted envelope.

Assumptions: Single writer per key, successful writes/rename, no competing modification; store dependency inspected separately.

Evidence and falsifiable oracle: Existing round-trip test uses actual store::write_atomic; probe persistence uses that exact source function extracted into a harness, documented in model.md#tests. See model.md#tests for executed bounds versus prospective work.

## tests

### Executed bounds and gaps

Evidence is recorded in model.md#evidence; probe.txt and probe-driver.txt reproduce the bounded observations without changing runtime source. Existing parsecache::tests cover two cases in the real world test binary. Extended probes cover actual parsecache source with explicit external dependency stubs and actual extracted write_atomic function; they never call public open, parse, finish or real parser/model paths. Identity and finish models remain source-derived. AST and registry checks establish structure only.

## evidence

### Parse-cache evidence

#### Executed checks

The Rust driver in probe-driver.txt copies the bound parsecache source into an
external harness, changing only leading module documentation comments to plain
comments. It appends probe.txt within that module so private helpers can be
exercised. It links the actual en, serde, serde_json, sha2 and anyhow artifacts
from the isolated target. It extracts the actual store::write_atomic function
from store.rs; it does not replace that function with a hand-written fake.
Execution-mode, model-path and public parser dependencies are panic stubs: any
attempt to use identity/open/parse/finish would fail. This bounds correspondence
to open_key/parse_with/write/decode/hash/file_hash/optional_hash and the extracted
write_atomic function, rather than full world integration.

Eight tests passed twice, including the two source tests and six appended tests.
The two complete positive logs were byte-identical, including their observed
0.04-second test duration. This is a small reproducibility observation, not a
performance estimate. The finite observations were:

- 120 batches: every length 0 through 3 over two text keys, four initial key
  subsets, and both refresh flags; exact missing occurrence list, call count,
  counters, final map and returned sequence were checked.
- 16 duplicate-text antecedents: two prior values for each of two cache keys and
  two values for each of two parser occurrences. Both output fibers contained
  all eight expected antecedents; unused-key preservation was also checked.
- Eight schema/identity/checksum guard assignments, exactly one accepted, with
  first-error category checks and malformed JSON rejection.
- Two whitespace/unknown-field representation changes, checksum rejection after
  a tree edit, and acceptance after checksum recomputation. The checksum does
  not establish tree semantic validity or authentication.
- Three failed parse attempts preserving the map but advancing counters; two
  refresh calls, corruption discard without disk rewrite, and returned-value
  mutation without changing the cached value.
- Six file-hash sizes: 0, 1, 65535, 65536, 65537 and 131073 bytes; absent paths,
  equal-content paths and directory rejection.
- Two stale in-memory snapshots published sequentially: the second replaces the
  first, with no union of independent additions. This demonstrates lost-update
  semantics without racing threads or touching an active cache.

Negative control: the driver changes only the external source copy's missing
predicate from `refresh || !contains` to `refresh && !contains`. The finite
batch oracle fails with `missing parse`. A false unique-key miss-count formula
is independently rejected by the duplicate observation. Production Rust is never
mutated by either control.

#### Structural evidence

The installed mova-modelcheck generated syntax.json from actual source. A second
projection was byte-identical. Fragment anchors are identity nodes
6/16/73/95/302 at [384,3551), batch node 495 at [5254,6324), and envelope nodes
620/657 at [6590,7254). These are partial syn syntax envelopes, not semantic
coverage or proof. Macro bodies remain opaque.

#### Reproduction and dependency bindings

From the repository root, first create a fresh Cargo target using
`CARGO_TARGET_DIR=TARGET cargo test -p world --lib parsecache::tests -- --test-threads=1`.
Compile the std-only driver with
`rustc --edition=2024 --crate-name cache_probe_driver world/src/parsecache.rs.groupant/probe-driver.txt -o DRIVER`.
Run `DRIVER REPO TARGET/debug/deps NEW_OUTPUT_DIR`; that directory must not exist.
The driver creates isolated cache subdirectories and sets TMPDIR for original
source tests. No active MOVA_DATA cache/state is read or written.

Bound dependency observations, SHA-256:

No real identity/finish dependency perturbation, GPU, linguistic parser,
concurrent-writer race, crash durability, whole-repository test suite or semantic
AST proof was executed. Future checks must retain these distinctions.

## transitions

### Executable transition observations

#### Behavior complement

Fix initial map T, refresh r, ordered text occurrences X and complete supplied
parser results P. Let M be the subsequence of X whose keys are absent from T,
or every occurrence if r. n=|X|, k=|M|. Project a transition onto
z=(parser-call bit, hit increment, miss increment). Its universe is
U={0,1} x {0,...,n} x {0,...,n}. The declared complement is exactly
U minus {(1[k>0],n-k,k)} for this projection. It says nothing about opaque parser
side effects. The native batch test enumerates every candidate in U for every
length 0..3 binary text sequence, initial key subset and refresh flag, comparing
membership with the actual call count and counter deltas. Final maps and output
order/cardinality are checked separately. This finite complement is not a
universal complement of all possible Rust traces.

#### Reverse reconstruction

For fixed X=[a,a], refresh=true, and initial T={a:old_a,b:old_b},
let each old value and each of the two parser markers range over {0,1}.
Observing only output [q,q] has exactly the eight antecedents
(old_a,old_b,first,q) when q is 0 or 1; target q=2 has no antecedents in this
declared domain. The native test enumerates all sixteen real transitions and
compares the complete fibers with that formula. It separately verifies that b
is preserved. Observing the full final map would constrain old_b too: these
eight-element fibers apply to the returned output projection only.

#### Attempt accounting and failure frames

The two-step history domain fixes X=[a,b,b], initial T[a]=marker9 and absent b.
Each call independently selects parser Err, short Ok, long Ok or complete Ok,
under both refresh policies: 32 histories, 64 steps. The injected outcome is
unobserved when no parser call is required. Both wrong cardinalities reject
before any map insertion.

For each call h'=h+n-k and m'=m+k, including rejected attempts. On rejection
T'=T. Path, identity, repaired flag and existing file bytes are unchanged by
parse_with; parser external effects, panics and resource failures are outside
the frame. On success T' overwrites in occurrence order and outputs are final
map lookups. Ghost committed occurrences increase by k only on successful
nonempty batches. Distinct committed keys count the union of their keys.
These are observational test ledgers, not new production fields:
distinct committed keys <= committed occurrences <= attempted misses.
A miss counter alone cannot recover either committed ledger. finish currently
uses attempted misses as its write trigger, so failed attempts may trigger
publication of an unchanged snapshot if a caller later invokes finish and
identity revalidation succeeds. Actual finish is not executed here.

#### Extensional parser premise

Equality to direct per-occurrence parsing requires a fixed identity-scoped
function f(text), independent of occurrence, neighboring texts, batch shape and
call history; retained entries must already equal f for their keys; each
successful parser batch must return f in input order. Under these premises,
final-map lookup yields f for every input position, including duplicates and
refresh calls. The native test checks mixed hits/misses under both policies,
then violates the premise with distinct duplicate markers and observes
last-occurrence overwrite. It also checks output ownership by mutating a
returned Word without changing the cached Word. No actual parser is asserted
to satisfy text extensionality; finite fixtures cannot establish that premise.

#### Envelope observations

The native envelope test checks all eight schema/identity/checksum truth
assignments, only one of which is admitted, with identity failure preceding
checksum failure. Typed Envelope values preserve the high u128 feature bit.
Compact, pretty and unknown-field JSON representations decode to the same
tree map. Thus accepted output does not recover original bytes uniquely.
Changing a Word without updating the checksum rejects; recomputing the checksum
admits the change. The checksum binds reserialized tree content, not semantic
tree validity, authenticity, raw JSON spelling or an independently verified
parser identity. The requested identity must separately match the envelope.

#### Boundaries and predictions

Predicted before running: zero disagreements for these finite relations in
debug and release; the OR-to-AND missing-predicate mutant must fail the native
batch oracle. No production change is proposed: occurrence overwrite and
attempt counters are existing policy, not defects to silently redesign.

Successful allocation/cloning, no usize overflow and isolated stable filesystem
operations are assumptions. Real dependency fingerprints, finish, concurrency,
crash durability and parser quality remain outside native test evidence.
Existing historical probes retain their separate source bindings and evidence.

## History

### 2026-10-29 · Epoch memory

2026-10-29, source SHA-256 da30cef6ff680f3c2853c158dada7d661307d0bbf0464bf058624bdd63d2c384. First source-bound instance; runtime source unchanged. Inspection exposed two important non-rollback effects: duplicate missing occurrences are sent separately and the final occurrence wins; hit/miss counters advance before parsing succeeds. Refresh is a persistent flag rather than a one-call invalidation. The source-bound partial AST is not semantic proof: no macro expansion, name resolution, cfg evaluation or type inference. General real identity/finish checks and concurrent-writer behavior remain untested. No speedup or complete parser-dependency guarantee is claimed.

### 2026-10-29 · evidence

Date: 2026-10-29. Base revision: 4279da541. Runtime source is unchanged.

Source SHA-256: da30cef6ff680f3c2853c158dada7d661307d0bbf0464bf058624bdd63d2c384.

`cargo test -p world --lib parsecache::tests -- --test-threads=1` passed
both existing tests, with 131 unrelated tests filtered out, in a separate CPU
target. No live parser or model was loaded by these two tests. Build warnings
were existing unrelated world warnings. Evidence log:
`$MOVA_DATA/work/octopus-next-cache-tests.log`.

Successful logs: `$MOVA_DATA/work/octopus-next-cache-probe-v2/positive-0.log` and
`positive-1.log`. Rejected mutant:
`$MOVA_DATA/work/octopus-next-cache-probe-v2/negative-0.log`.
The first exploratory probe used serde_json::Value to edit an envelope with a
128-bit feature bit. That generic-number intermediary changed its representation
and caused deserialization failure before the expected guards. The fixture was
corrected to mutate typed Envelope values and preserve numeric bytes. Production
cache round trips already retained the feature bit; no runtime fix was needed.
The initial failed fixture log remains outside the repository under
`$MOVA_DATA/work/octopus-next-cache-probe/positive-0.log`.

The independent groupants report with the unchanged registry was valid, with
19 file sidecars, 6 group sidecars and 513 missing file sidecars. The new source
remains an unregistered leaf until the parent integrates registry-node.json
under inference-core. `changed` against 4279da541 reports no changed Rust source;
it does not establish any mathematical claim about the new sidecar.
Reports: `$MOVA_DATA/work/octopus-next-cache-groupants-final.json` and
`$MOVA_DATA/work/octopus-next-cache-changed.json`.

- world/src/store.rs: 6db0ddecd1453238a4de3ca60f9ae8282ebfebe31f1310ffb1891e4d3d8f6980.

- en/src/annotate.rs: 4000a32591a0e5a6387883925998f726997d6c398e46a6328862e83eef74879c.

- en/src/gram.rs: 73c2629c907bd875b3a3082d7809b3b9fc4801002727e05f4101db665a8617c8.

- Cargo.lock: 97a06ccf83083b384be8b6dc12b2a88296c76d9e17515784f0fc0c83287bb9f9.

### 2026-10-30 · Epoch memory

2026-10-30, appended native transition tests; production prefix unchanged.
The finite models now execute in the actual world test binary: projected
complement membership, complete output-only preimages including an unreachable
target, two-call failure/refresh frames and serialization fibers. See
[transition relations and premises](model.md#transitions). Attempted misses must not
be described as committed parses. Exact direct-parser equivalence requires
identity-scoped text extensionality and compatible cache entries; a duplicate
counterexample explicitly refutes unconditional per-occurrence preservation.
No runtime bug was established and no cache policy or schema changed.

### 2026-10-30 · document

Subject: world/src/parsecache.rs at SHA-256 6545414761854d33b7b41f8a864a7f4c59eebeaee721eef68772bb4a7a652479; epoch 2026-10-30. Scope: runtime and original tests bytes [0,10023); appended native tests are described in model.md#transitions. Transition horizon is one invocation unless an entry names a call history. Successful ordinary allocation; no usize counter overflow; a stable filesystem during each modeled read; no claim of parser linguistic correctness. Source inspection is evidence of a conditional specification, not a semantic proof. Evidence status is bounded_execution only where model.md#tests names the exact checked observation; all other oracles are prospective.

### 2026-10-30 · publication-protocol

Native 2026-10-30 refinements and executable finite domains: [transitions](model.md#transitions). Historical evidence retains its original source hash.

### 2026-10-30 · tests

#### Native transition epoch 2026-10-30

Run `cargo test -p world --lib parsecache:: -- --test-threads=1 --nocapture`
and the same command with `--release`. The wider `parsecache::` filter includes
both the two original tests and four new `transition_tests` tests.
They call actual private functions through injected parser closures/open_key.
[transitions](model.md#transitions) states exact domains and premises.
The four deterministic CHECK records agree across two debug, two release and
two external-harness runs. Native six-test runs all pass. The external driver
now runs twelve positive tests and its compiled mutant targets the native
batch/complement oracle. Existing historical evidence remains dated below.

### 2026-10-30 · evidence

#### Native transition epoch 2026-10-30

Base ae2b64669. Current source SHA-256:
6545414761854d33b7b41f8a864a7f4c59eebeaee721eef68772bb4a7a652479.
The original 10023-byte source prefix is byte-identical to base; only a
test module was appended. Four new tests plus both original tests passed
twice in debug and twice in release. Four CHECK records matched across all
four runs and two external positive harness executions; timing and build
diagnostics are excluded from the deterministic record comparison.

Observed finite domains:

- 120 input-map-refresh combinations and 2768 candidates in the projected
  call/count complement universe; missing order, duplicate overwrite, final
  map and output cardinality/order checked.
- Sixteen antecedents, output-only fibers of eight, eight and zero for markers
  0, 1 and out-of-domain 2. Two extensional-parser fixtures and one explicit
  occurrence-dependent duplicate counterexample.
- 32 two-step histories (64 steps) over Err, short, long and complete parser
  outcomes under both refresh flags. Attempted counters, successful-commit
  ghost ledgers, map/file/identity/repaired frames and repeated refresh checked.
- Eight envelope guard assignments with one acceptance; three equivalent JSON
  representations, high feature-bit preservation, checksum corruption rejection
  and admission after deliberate checksum recomputation.

The external driver ran twelve tests twice against the source-bound harness,
then compiled the missing-predicate OR-to-AND mutant. The native batch oracle
failed with missing parse as expected. The driver changed only its extracted
external source copy. Actual parser/model functions remain panic stubs there;
native tests use private injected functions. No CPU/GPU parser, network or
teacher was invoked.

The installed mova-modelcheck generated syntax twice with identical files.
Every prior syntax node except root node 0 is unchanged; the root extent grew
with the appended tests. Existing fragment byte ranges and node selections
therefore remain current. The changed-file gate reports one changed source and
no missing registration. The groupants report reports only the expected stale
parse-cache bindings in inference-core and mova ancestors; root integration
owns those two updates. No leaf/fragment diagnostic was reported.

Unrestricted world Clippy fails on existing modulo_one and never_loop errors
outside this file. Structured diagnostics contain no parsecache.rs warning or
error. A scoped rerun allows those two known lint categories; other repository
warnings remain separate. This is not a claim of repository-wide strict Clippy.

External logs and reproduction tools:
`$MOVA_DATA/runs/octopus-next-cache-2026-10-30/` contains debug-1/2.log,
release-1/2.log, probe-driver.log, probe/positive-0/1.log,
probe/negative-0.log, check.log, clippy.jsonl, clippy-scoped.jsonl,
syntax-1/2.json, changed.json and groupants.json.
The checked-in driver reproduces the external positive/mutant runs as described
in the earlier reproduction section, now using the current source hash.
The adjacent registry-node.json supplies the parent with the updated leaf
description/check command. No full suite, actual dependency identity/finish,
concurrency, crash durability or semantic parser equivalence is inferred.

Scoped Clippy completed successfully after allowing only the two named existing
error categories. Formatting and git diff whitespace checks passed.

Current artifact/evidence SHA-256 bindings:

- probe-driver.txt: 728d1e44d04802cebf0c100677c28b606127528e4211ad7f215ecef708554e21.
- debug-1.log: acc960c98c9a76eef6ac92a4b24ab86ec86ce94e00b7eaf5cc427f4819e9584f.
- release-1.log: 0d7604db687c24ffd01aff58cec6d4ec50e61e00c8ff6ea08c5493f06bccfc82.
- check.log: 9bc1f538afb567828f6e8b0966260216a878b01a8fd92f6bafc32db31dcbe230.
- probe/negative-0.log: 34c3a8e79f7c18d8304d232e7323343fd747e401196386a5b10d1200f227eb0e.

### 2026-10-30 · transitions

Epoch 2026-10-30. The four `transition_tests` tests appended to parsecache.rs
execute the actual private cache operations in the world test binary. These are
finite exact correspondence oracles, not approximate parser replacements or
percentage-error profiles. No linguistic parser, teacher, network, model-file
identity computation or GPU is invoked. The existing typed catalog remains the
model inventory; this document deepens its relations rather than adding labels.
