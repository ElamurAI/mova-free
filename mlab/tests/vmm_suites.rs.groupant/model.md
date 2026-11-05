# mlab-tests-vmm-suites

## descriptant

### mlab-tests-vmm-suites

Frozen VMM400 integrity/baseline harness and SHA helper; this epoch runs synthetic SHA inputs only.

## approximant

### Forward relation

sha256_hex writes PID/atomic-sequence temporary bytes, invokes sha256sum, removes file, returns first stdout token. load would verify ten frozen files, check would run400 and compare exact sorted known-red IDs, VM gate compares captured output.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

FROZEN and KNOWN_RED unchanged. Atomic counter plus PID distinguishes helper temporary names within process; cleanup occurs after subprocess output, not after every panic path.

#### Failure ordering

Write/launch errors panic; subprocess exit status is not checked; empty output gives empty digest. Load would hash before parsing; scenario count checked before suite-run baseline comparison.

#### Cost

Synthetic helper O(input bytes + subprocess startup); unexecuted load O(total frozen bytes) and scenario interpretation. Temporary file effects only for abc and empty inputs.

## composition

### Composition

sha256_hex writes PID/atomic-sequence temporary bytes, invokes sha256sum, removes file, returns first stdout token. load would verify ten frozen files, check would run400 and compare exact sorted known-red IDs, VM gate compares captured output.

Digest is first whitespace-separated lossy-UTF8 stdout token, not structurally validated. Red IDs sorted before equality; independent gate has same=400 assertion.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-tests-vmm-suites

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Synthetic SHA helper execution excludes reading frozen scenario files because load/check are not called. The baseline check would exclude either newly red or newly green IDs; no such claim is established by this epoch.

## reverse_reconstruction

SHA output admits all byte preimages under digest relation; mathematical collision possibility prevents unique reconstruction. Equal red sets admit many changed individual outputs and do not alone prove byte identity.

## state_invariant

FROZEN and KNOWN_RED unchanged. Atomic counter plus PID distinguishes helper temporary names within process; cleanup occurs after subprocess output, not after every panic path.

## failure_precedence

Write/launch errors panic; subprocess exit status is not checked; empty output gives empty digest. Load would hash before parsing; scenario count checked before suite-run baseline comparison.

## representation_transport

Digest is first whitespace-separated lossy-UTF8 stdout token, not structurally validated. Red IDs sorted before equality; independent gate has same=400 assertion.

## resource_cost

Synthetic helper O(input bytes + subprocess startup); unexecuted load O(total frozen bytes) and scenario interpretation. Temporary file effects only for abc and empty inputs.

## tests

### Evidence and limitations

Scope: FROZEN and KNOWN_RED unchanged. Atomic counter plus PID distinguishes helper temporary names within process; cleanup occurs after subprocess output, not after every panic path.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

### 2026-10-31 · tests

Selected named outcomes: wave30_synthetic_hash_helper. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "wave30_synthetic_hash_helper": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
