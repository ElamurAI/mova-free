# modelcheck-library-discovery-changes

## descriptant

### Sorted Git discovery, binary membership, inventory and changed-source gate groupant

Discovery order is strict Rust String order and duplicate paths collapse. Membership equals the old existential scan only under sorted order. Directory registered<=total; missing=total-registered. Deleted diff paths are omitted by is_file; inventory may already fail on tracked missing files.

Useful prior epochs remain in the parent descriptant. This child uses exact byte anchors and the same whole-source hash: editing unrelated bytes also invalidates its binding. The fragment is a partial structural/behavior model, not a proof. No executable percentage surrogate is supplied: categorical gates have no defined useful loss metric; any future profile must use supported caps 1, 2, 5 or 10 percent with explicit domain and evidence.

## catalog

Wave33 historical scope and evidence follow. Current source hashes and refreshed ranges are in manifest.json; wave34 refinements and evidence are at the source root.

# Sorted Git discovery, binary membership, inventory and changed-source gate typed models

Subject/revision: source SHA-256 in manifest; scope [3699 , 8919). Domain is one completed scoped call on a fixed snapshot with adequate resources, unless narrowed below. Observation includes return/error and declared filesystem effects. Models are source-derived partial relational constraints; complements are inner exclusion hypotheses, converse relations retain compatible-input ambiguity. No universal proof or inverse executor is claimed. Evidence: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Mutation evidence and limits are recorded in ../model.md#wave33-evidence.

## behavior_complement

Exclude successful changed calls with empty or option-prefixed base and inventory success with registered paths absent from discovery or canonical source outside root. Checks are never executed.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reverse_reconstruction

A report constrains discovered paths, source digests, model IDs and directory counts; ignored files, source bytes beyond digest identity, Git history and filesystem timing remain ambiguous.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## sorted_membership_equivalence

For strictly sorted F from rust_files, binary_search(F,q).is_ok iff exists i:F[i]=q. Empty, first/last, gaps, prefixes and Unicode ordering obey the same relation; arbitrary unsorted callers are outside the helper contract.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## directory_count_conservation

Each discovered path adds one to every lexical ancestor directory and adds registered indicator to the second count. Totals are sums over ancestry incidence, not disjoint directory partitions.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## changed_set_projection

For fixed repository snapshot, changed=(diff(base) union untracked) intersect existing_file. missing_changed=changed minus registered_sources. A report cannot recover which producer supplied a path occurring in both.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## comparison_budget

For Q queries and P sorted paths, binary search requires O(Q log(P+1)) comparisons and constant search workspace. Timings compare the actual helper with old linear predicate; discovery/hash/JSON/ancestor costs are excluded.

Oracle: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## tests

### Native correspondence

Exact named checks: wave33_tests::sorted_membership_matches_linear_reference_and_measures_cost; tests::changed_gate_rejects_revision_options_before_touching_a_repository.

See ../model.md#wave33-evidence for current repeated profiles, actual production mutants and scope limits. AST generation alone is not behavioral evidence. Each reference test body was read before execution.

## forward

### Scoped forward model

Sorted Git discovery, binary membership, inventory and changed-source gate

Git ls-files -z cached/others exclude-standard feeds a BTreeSet. Membership returns binary_search success on that sorted list. Inventory verifies registered paths, confines and hashes files, and accumulates ancestor directory counts. Changed inventory rejects option-like base before inventory, then unions surviving diff/untracked .rs paths.

## invariant-and-cost

### Invariant and cost

Discovery order is strict Rust String order and duplicate paths collapse. Membership equals the old existential scan only under sorted order. Directory registered<=total; missing=total-registered. Deleted diff paths are omitted by is_file; inventory may already fail on tracked missing files.

Discovery sorting O(P log P) comparisons; membership Q queries O(Q log P) rather than O(QP), multiplied by compared prefix lengths. Inventory hashes total bytes B and visits directory depth D per path. Changed invokes inventory plus two Git processes; no IO speed gain follows.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: materialized a scoped view after full-source review. Git ls-files -z cached/others exclude-standard feeds a BTreeSet. Membership returns binary_search success on that sorted list. Inventory verifies registered paths, confines and hashes files, and accumulates ancestor directory counts. Changed inventory rejects option-like base before inventory, then unions surviving diff/untracked .rs paths.
