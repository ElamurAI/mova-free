# mathematical-twin-library

## catalog

# Mathematical twin library

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

For d=1..32, constructor initializes theta=prior, P=I/ridge, n=SSE=0 and zero box bounds. Prediction validates state then computes px=P*x, h=x^T px and seconds=x^T theta in f64. Observation of finite y>0 predicts first, clones state, applies the rank-one correction, validates the candidate and commits it.

## invariant

Validation requires finite vectors/SSE>=0 and approximate symmetry plus successful finite positive Cholesky pivots. It does not certify an exact symmetric matrix, box ordering, n/SSE consistency or observational reachability. Constructor does not call validate: subnormal positive ridge may yield infinite P. Predict may return negative seconds. n+1 overflow is not guarded: debug panic or release wrapping is outside the n<u64::MAX model.

## cost

At dimension d<=32, validation uses O(d^3) arithmetic and O(d^2) allocation. Predict adds O(d^2); observe calls predict and another validate, hence O(d^3), not merely the O(d^2) rank-one kernel. State memory O(d^2). No wall-clock estimate is calibrated.

## composition

Initial-state covers constructor/validation; forecast uses validation and dot; update calls forecast then clone/validate/commit. Thus R_observe composes their shared M,x and preserves the first forecast for SSE. R_parent is not a union of child complements. Validation is overlapping work: cost it twice for observe, not once per catalog entry. dot, public data declarations and tests remain file-level scope.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks (catalog)

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U contains one API call with arbitrary f64 vectors and Result/state observations. R is the guarded transition in forward. A source-derived inner exclusion is success from predict with a dimension mismatch, nonfinite feature or failed state validation; another is an observe Err that commits a candidate. Constructor Ok with invalid P is deliberately not excluded. Proposed oracle enumerates malformed dimensions and checks unchanged state on Err; existing invalid-input test supplies a bounded witness.

## reverse-reconstruction

For a successful observed prediction z, B(z)={(M,x):validate(M), matching dimensions, f64 predict(M,x)=z}. Fixing x and seconds alone leaves a hyperplane x^T theta=z.seconds in the real approximation, with P/history/bounds constrained only by observed fields. Rounded execution can enlarge fibers. Proposed oracle enumerates small one-dimensional states and groups identical predictions; unique inversion is not claimed.

## ridge-prior

For finite prior length 1..32 and finite positive lambda, new returns theta0=prior and diagonal fl(1/lambda). This is the ideal regularizer lambda||theta-prior||², not a zero-centered ridge. A successful constructor need not pass validate when reciprocal overflows. Proposed boundary oracle uses smallest positive subnormal and maximum finite ridge.

## validation-domain

Let V be the ordered dimension, vector-finiteness, SSE and matrix/Cholesky predicates in validate. Acceptance means this floating-point procedure succeeds, not mathematical SPD for every tolerance-accepted asymmetric P. Each row length is checked before symmetric indexing. Proposed oracle varies near-asymmetry and pivot cancellation independently.

## forecast-fiber

At fixed valid M, seconds=dot(x,theta), h=dot(x,Px), observations=n. Equal seconds need not imply equal leverage or equal x. Historical RMSE is None for n=0 and sqrt(SSE/(n as f64)) otherwise. Proposed oracle constructs equal-dot feature pairs with unequal leverage; real level sets do not prove bitwise equality.

## box-support

For n>0, outside iff some coordinate lies below min or above max, with boundary equality inside. For n=0 it is always true. The box can contain unseen joint combinations, so inside does not establish interpolation in the convex hull. Proposed oracle trains on (0,0),(1,1) then probes (0,1); malformed externally supplied reversed bounds are accepted by validate.

## rank-one-update

In exact arithmetic with symmetric SPD P and y>0: q=P*x, h=x^Tq, e=y-x^Ttheta, theta'=theta+q*e/(1+h), P'=P-q*q^T/(1+h). Rust recomputes q after predict and performs each multiply/divide in f64. Finite inputs can overflow intermediates; candidate validation may reject. Existing independent ridge test compares a four-row 2D sequence within 1e-12.

## information-history

Under ideal arithmetic from constructor, P_n inverse=lambda*I+sum x_i*x_i^T and theta_n=P_n*(lambda*prior+sum x_i*y_i). Different ordered histories can share these sufficient statistics; floating order changes equality. Existing 2D ridge oracle supports only its bounded sequence. No guarantee for arbitrary ill-conditioned or malicious loaded states.

## prospective-residual

SSE'=SSE+(y-predict_before.seconds)^2; n'=n+1 below counter overflow. This accumulated online error differs from residuals recomputed under theta_final and does not retrospectively alter earlier Prediction values. Existing repeated-corrections test has prior0, x1,y2: first forecast0, SSE4, next forecast1. RMSE is historical, not a confidence interval.

## transactional-update

The externally visible self assignment occurs only after next.validate succeeds. Within returning Result errors, self is unchanged; a panic or process abort has no returned Result trace. Temporary clone updates and floating failures are permitted. Proposed oracle uses large finite y causing e² overflow and compares every state field before/after Err.

## update-antecedent

Given x,y and ideal successor P',theta', h'=x^T P' x<1, candidate P=P'+P'x*x^T P'/(1-h') and theta=theta'-P*x*(y-x^Ttheta') recover algebraic predecessor parameters. Also n=n'-1 and SSE=SSE'-(y-x^Ttheta)^2 must be admissible; old min/max are many-to-one under expansion. This formula is conditional on exact symmetric arithmetic, not a stable f64 inverse. Proposed oracle checks forward reconstruction on bounded rational examples.

## resource-budget

At fixed d, admissible budget candidates satisfy measured T_validate(d)+T_kernel(d)+allocation overhead<=B for predict; observe includes two validations. Asymptotic O(d^3) yields no absolute time guarantee. Proposed benchmark would fit separate validation/kernel costs, but no new timing run is executed here.

## checks

### Twin library checks

- `cargo test -p mova-twin --locked` passed three library and three local ledger integration tests. No live benchmark, teacher, training or GPU run was invoked. Existing ledger tests execute bounded local shell fixtures.
- A separate external Rust probe confirmed four boundary cases: tiny positive ridge returns a model with infinite P that validation rejects; validation permits reversed bounds and nonzero SSE at n=0 while prediction can return negative seconds; finite huge duration causes update rejection without mutating state; zero features update count/SSE while leaving theta unchanged. This probe used an independent offline Cargo lock, not the repository lock, so the locked suite remains the primary reproducible package check.
- Rust generation hashed the actual source and calculated UTF-8 byte ranges. The file root has twelve typed models; its three fragments have four each. Behavior complement and reverse reconstruction are first and second at every node.
- `mova-modelcheck groupants` passed with no diagnostics and two initial runs yielded byte-identical reports. A stale fragment source hash made the check fail; original bytes were restored. These results establish declaration integrity, not mathematical correctness.
- The parent task integrates registry-node.json and runs the final registry/tree checks. At this branch stage the two files are explicitly unregistered; no shared registry/group artifacts were edited.

The source review does not establish stable inversion, statistical calibration, arbitrary-state reachability, or overflow-safe counters. Proposed catalog oracles remain proposals unless covered by the four boundary cases above or named existing tests. No timing improvement is claimed.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/lib.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

### 2026-10-29 · checks

Executed 2026-10-29 against source bound by manifest.json.
