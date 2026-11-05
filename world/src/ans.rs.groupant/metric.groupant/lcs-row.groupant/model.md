# Rolling longest-common-subsequence row

## forward

For each a token keep saved previous-row diagonal; for each b token set cell to diagonal+1 on equality, else max(previous-row same column, current-row left). Return final cell.

## invariant

Returned length cannot exceed min(n,m); unequal singleton streams cannot return 1. Reordering tokens is not generally score-preserving.

## behavior-complement

Returned length cannot exceed min(n,m); unequal singleton streams cannot return 1. Reordering tokens is not generally score-preserving.

## reverse-reconstruction

A length k admits every sequence pair with longest common subsequence k; many matching witnesses and token spellings may realize it. k=0 contains disjoint alphabets and either empty sequence, without unique reconstruction.

## parent-interaction

rouge_l tokenizes first, bypasses empty streams, then consumes LCS for harmonic precision/recall; rouge_q takes best nonblank reference. DP length does not establish semantic answer quality.

## boundary

After column j, dp[j+1] is current-prefix optimum while prev is the previous-row diagonal for the next cell; overwriting prev too early destroys dependencies.

## information

Symmetry holds for returned length though space cost depends on the second input. Repeated equal tokens need distinct subsequence positions.

## cost

Exactly n*m inner iterations, m+1 usize cells, O(n*m) time and O(m) auxiliary memory excluding token input; no task-error surrogate.

## checks

Existing independent subset enumeration over 961 binary sequence pairs; finite test is not general correctness proof.

Exact selected native boundary: `cargo test --offline --locked -p world --lib ans::wave8_models::lcs_matches_subsequence_oracle -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-ans-metric-lcs-row, current source SHA d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
