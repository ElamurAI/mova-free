# modelcheck-publication

## model

# Publication boundary model

Subject: adjacent Rust source at the manifest digest. The current caller-pin, evidence-disjointness and narrative-role refinements are specified in model.md#pinned-policy; the initial finite checkpoint below is retained as history. Domain: bounded, immutable, readable filesystem snapshots and a trusted explicit editorial-policy input. Observation: a new catalog or one manifest admission report. Models are source-derived and partial; finite Rust controls establish neither scientific truth, authorship nor licence validity.

## Forward approximant

For manifest M, exact digest h, catalog entry E and allowed list A, let I(M) identify canonical algorithm bundles, Mova-bound papers and algorithm/source-bound surveys. Default scope is implementation if I(M), otherwise unreviewed. Self-declared global labels have no authority.

Completed admission is the conjunction:
G = complete_current_inventory AND caller_pinned_reviews AND normalized_policy_entry_equality
AND complete_catalog AND exact_entry(M,h) AND explicit_global(E)
AND NOT I(M) AND supported_paper_survey(M)
AND independent_review(E) AND affirmative_rights(E,M)
AND pinned_evidence(E) AND nonempty_exact_subset(A,artifacts(M))
AND current_reader_integrity(M).

Successful output contains A, never the entire directory. The catalog builder has no repository input and counts only policy decisions: publication-check must run the actual repository-aware reader before export. The supported global kind is the existing schema1 generated/unverified paper survey without implementation bindings. Editorial approval remains a separate trusted assertion; unknown future exposition kinds are excluded pending reader support.

## behavior_complement

Domain: completed fixed-snapshot calls. Observation: public_export_allowed and allowed_artifacts. Missing catalog/entry, stale edition, unsupported scope, incomplete inventory, restrictive immutable rights, absent evidence/allowlist and implementation material exclude admission. Counterexample to integrity implies global: an intact algorithm bundle defaults to implementation. Counterexample to length implies review: repetitive prose remains unreviewed without policy. Evidence: source inspection and named native negative fixtures.

## reverse_reconstruction

G=true constrains every conjunct and exact artifact hash. It cannot reconstruct a unique reviewer, editorial history, licence agreement, narrative or directory. Many evidence texts can yield the same admitted observation. No inverse executor or causal reconstruction is implemented.

## version_partial_exclusion

Domain: one manifest path with changing bytes. A decision on h1 does not admit h2 when h1 differs from h2. A new manifest after catalog construction has no entry. Counterexample to path identity suffices: change metadata without changing the path. Stale-policy, stale-catalog and new-manifest fixtures demonstrate this finite boundary. Stale/unknown decisions become unreviewed.

## artifact_allowlist_fiber

Domain: declared artifacts D and selected A. Require nonempty A subset D with exact hashes, excluding raw source snapshots. Output is A, not D. Counterexample to allowlist alone proves integrity: corrupt an unselected declared auxiliary artifact; the independent reader rejects it. Unknown extra files never enter the allowed output. Duplicate, raw-code, stale-evidence and reader-corruption controls distinguish these cases.

## immutable_rights_partition

Domain: immutable manifest flags and affirmative policy. Policy cannot override local_only=true, license.local_only=true, public_export_allowed=false or license.public_export_allowed=false. Missing affirmative top-level permission/local-only declarations also exclude. Counterexample: a policy permits a local-only edition. Four restrictive-flag controls establish this finite boundary; a new reviewed edition is needed.

## error_partition

Malformed duplicate/path-unsafe decisions return errors; absent/stale/unapproved ordinary admission yields a denied report; policy-approved input can still fail actual reader integrity. Multiple simultaneous violations have no error-priority guarantee. Visible symlinks, JSON/count/depth excess prevent a complete catalog, rather than emitting an incomplete inventory marked complete.

## Invariant and cost

Bundles, source bindings and collection indexes are unchanged. New catalog metadata may live at store root or outside it, never in a bundle subdirectory. The CLI supplies exclusive output creation. Sorted inventory caps visited entries/manifests at 50000, depth 16 and each read at 16MiB; evidence and allowed lists cap 256 each. A directory listing is materialized before per-entry checks, so the cap is not a strict hostile-directory allocation bound. Cost follows decoded metadata, ordered-map comparisons and bytes read/hashed; no speedup or percentage error claim is made.

The fixed-snapshot assumption matters: no atomic snapshot is acquired and symlink checks do not prove TOCTOU security. Hashes prove byte identity, not honest review, rights ownership or science.

## Composition and evidence

CLI arity/new-output behavior composes with this gate. The legacy library reader remains an independent integrity-only gate and its manifest digest is compared with the policy-pinned digest. Syntax projection is partial syntax. Implementation integrity, editorial review, rights, artifact scope and scientific proof remain separate observations.

## pinned-policy

# Caller-pinned editorial policy and narrative-only admission

Subject: adjacent source at the current manifest digest. Domain: fixed trusted local filesystem snapshots, bounded supported manifest inventory and a caller-supplied expected SHA-256 for the reviews file. Observation: exact admitted artifacts plus separate editorial and mathematical status. Filesystem concurrency, authenticated authorship and honest review are assumptions or gaps, not proved properties.

## caller_policy_converse

For positive observation G, recover the compatible-input relation:
Preimage(G) is the set of (store, catalog, reviews, expected_digest) for which the caller's expected digest equals the reviews bytes, equals the catalog policy pin and equals the recomposed policy pin; the current complete inventory digest and normalized selected decision also agree. This constrains a policy edition, not a unique reviewer or narrative. A catalog self-assertion supplies no caller trust root. Counterexample: edit only its reason or allowed list while preserving schema and manifest digest. Finite controls reject the altered entry, wrong expected digest and changed reviews file.

The old publication-check call without reviews and expected digest remains a negative-only compatibility path. It can inspect implementation/unreviewed/current negative catalogs but cannot emit positive publication permission. Positive CLI arity is publication-check REPO STORE CATALOG REL_MANIFEST REVIEWS EXPECTED_REVIEWS_SHA NEW_REPORT.

## inventory_version_exclusion

A catalog's inventory pin is SHA-256 of sorted manifest path/digest pairs. Positive admission recomposes the entire bounded current catalog from the pinned reviews file; a missing/unknown inventory digest, new manifest, changed manifest or incomplete artifact inventory excludes the old positive decision. Fresh recomposition does not mutate the store. Counterexample: add a second intact manifest after catalog construction and request the original global entry; finite evidence rejects the stale inventory. No incremental append permission or cache shortcut is inferred.

## review_artifact_disjointness

Evidence paths must be normal visible STORE-relative paths inside reserved reviews/. No directory ancestor through the store root may contain manifest.json. Each evidence byte digest must match and neither its path nor digest may equal a declared artifact of any inventoried manifest. Before any global policy decision the complete bounded artifact metadata inventory is built, with at most 256 declared artifacts per manifest and 50000 total. Incomplete/invalid artifact metadata prevents positive catalog construction.

Counterexamples: generated description as its own evidence; the same description copied into reviews/ under another name; an evidence file under a manifest ancestor; a reserved evidence path appearing in the artifact inventory. Named controls reject each. Disjoint metadata hashes do not establish human review, editorial independence, copyright ownership or original authorship. The caller accepts these declarations as a trusted editorial assertion; mathematical status is unchanged.

## narrative_role_partition

The only currently admitted narrative role is the existing paper-survey components.description.path. Allowed artifacts remain a nonempty exact subset of declared artifacts, and each selected path must have that role. An artifact digest equal to source.sha256 is excluded regardless of extension or name. Counterexamples: allowlist auxiliary.json or a renamed primary-paper copy as description. Controls reject nonnarrative roles and raw-source digest identity. Hash inequality cannot prove originality or rule out modified copying; substantive editorial/originality review remains necessary. Unknown future narrative roles are unsupported.

## error_and_assurance_partition

No-policy positive request, bad caller pin, catalog/policy disagreement and stale/incomplete recomposition yield denial. Malformed duplicate/path-unsafe inputs remain errors. Current reader integrity is still independently required after policy checks; a corrupted unselected declared asset fails that reader. Editorial_status comes from the pinned decision; mathematical_status preserves generated_source_survey_unverified, and proof remains false. Review is not mathematical verification and does not supersede that status.

Visible-path consistency excludes every dot-prefixed component, matching inventory exclusion. Any Git marker in a store ancestor rejects the store. All file reads assume a fixed trusted local filesystem; symlink check/open races and lack of atomic snapshot are explicit. The existing integrity reader may permit internal symlinks in unselected assets while the publication path checks are stricter. No portable race-free filesystem guarantee is claimed.

## Evidence and cost

Positive calls recompute a bounded inventory and catalog; they are intentionally more expensive than integrity-only or ordinary negative lookups. Ordered sets contain bounded declared artifact metadata; file/directory allocation limits and fixed-snapshot assumptions remain explicit. There is no performance or numerical-approximation claim. A synthetic isolated CLI fixture also produced byte-identical catalogs twice, admitted the pinned positive command and denied the same catalog through the old no-policy command. Actual collection/site metadata writes are outside this module change.

## History

### 2026-11-01 · Composition and evidence

On 2026-11-01, 12 focused native tests passed debug/release, offline/locked with at most two jobs. Controls cover deterministic defaults, actual reader success, implementation rejection, stale/absent/unknown/incomplete/new-manifest exclusion, missing conjuncts, immutable restrictions, evidence versus reader corruption, duplicates, traversal/symlinks, unknown kind/raw-code and scan/output bounds. This is finite evidence. Actual 293-entry catalog generation and deployment are outside this change.

### 2026-11-01 · Evidence and cost

On 2026-11-01, 18 publication controls and all 26 binary controls passed debug and release, offline/locked with at most two build jobs. The initial 12-test checkpoint remains historical. Added controls cover caller-policy pins and entry equality, normalized evidence ordering, policy/inventory changes, reserved artifact-disjoint evidence, narrative roles/raw-source digest, hidden paths, Git ancestors and missing artifact inventories.
