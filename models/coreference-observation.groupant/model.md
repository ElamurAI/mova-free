# coreference-observation

## descriptant

# coreference-observation

## epoch-memory

The reusable algorithm is static total-priority sorting for the existing greedy policy. The leading overlap matrix [[5,4],[4,0]] exposes why global assignment is not a substitute: greedy obtains 5 while optimal assignment obtains 8. The hybrid now reserves sorting for residual sides at least 128 with a bounded edge budget. Reused Hungarian scratch is a separate resource optimization and must preserve per-augmentation reset. These observations predict local costs only.

## model

# coreference-observation

## forward

score_doc filters entities, flattens mention spans, invokes match_mentions, constructs matched/spurious partition identities and evaluates metrics. match_mentions groups by the selected key, consumes exact spans first in stable vector order, then chooses maximal-overlap residual pairs with prediction index before gold index as tie breakers. When both residual sides are at least 128, checked edge count is <=262144 and reservations succeed, heap_sort orders fixed total-priority edges and endpoint flags select the same greedy matching; otherwise scanning remains. assign_max is a separate maximum-weight assignment operation; its Hungarian scratch is allocated once and reset for each augmentation.

## invariant

The greedy result retains exact-match priority, injective gold use, prediction-index output order, and matching even on zero-overlap residual edges. Sorted and scanned selectors have identical complete match vectors on their shared domain. A refused sorted attempt leaves output unchanged for fallback. Hungarian potentials persist as required by the algorithm while per-augmentation minima/visited scratch is reset; objective semantics are unchanged for regular finite matrices.

## cost

For residual P by G groups, sorted work is O(PG log(PG)) and O(PG+P+G) auxiliary storage only within min(P,G)>=128 and PG<=262144; fallback scanning is O(PG min(P,G)) plus vector-removal work. Grouping, exact-span search/removal and partition metrics remain additional reached costs. For r=min(rows,columns), c=max(rows,columns), Hungarian assignment costs O(r^2 c) with O(r+c) augmentation scratch; original rows>columns additionally materializes an O(rows*columns) time/storage transpose. Scratch reuse removes repeated per-augmentation buffer allocation, not the cubic-order arithmetic. Reservation or count-limit fallback may retain legacy cost.

## composition

The actual scoring pipeline joins mention identities to metric partition IDs. Greedy mention matching and global maximum assignment are distinct operations within the score module; replacing one with the other is invalid. Harness observations independently compare heap-sort total keys with stable greedy identities and compare integer maximum objectives with a negated-cost minimum-assignment oracle only on exactly representable integer matrices. No runtime call from match_mentions to Hungarian is introduced.

## checks

["Source-local score tests for selector equivalence, threshold/cap fallback and scratch reuse","Five groupant_ independent coreference observations","Repeated focused checks, negative controls and paired syntax: see model.md#evidence"]

## limitations

Native evidence is finite and revision-specific; ragged/nonfinite matrices, malformed partitions, numerical overflow and linguistic annotation quality are not newly established. Thresholds/caps and scratch reset semantics must match merged source. Integer objective equality does not imply assignment-vector equality under ties or equivalence on arbitrary fractional matrices. Real-arithmetic assignment optimality is a model target; general floating-point optimality is not established.

## wave27-coreference-pipeline

doc supplies kids/depth/sub/quote to mention and it; mention fields feed sieve masks/head keys/candidate order; sieve returns full membership (including internal mentions) to eval; visible tests full cluster length before filtering. eval::sys_entities recomputes head via corefud::span_head, whereas out_mentions writes detector head, so no universal head equality is claimed. score is existing external sibling ownership. main::load joins strict CoNLL-U reading and optional retree, then documents. tests assertions cross actual joins and do not substitute copied algorithms.

## catalog

# Typed transfer relations

Source-derived models; native evidence pending.

## excluded-identity-changes

Type: behavior_complement.

In U of tagged lawful matcher calls and complete output vectors, excluded observations include two predictions consuming one gold mention, a residual overlap pair displacing an available exact-span match, a zero-overlap residual pair being omitted while both sides remain free, and swapping equal-overlap ties against prediction/gold order. In the separate real-arithmetic assignment model, an objective below the maximum is excluded. For machine arithmetic, this epoch establishes preservation of the existing rounded observation and bounded exact-integer oracle results, not universal numerical optimality. Neither statement imposes maximum weight on greedy mention matching.

## compatible-spans-partitions

Type: reverse_reconstruction.

An observed match vector admits all mention lists and group keys whose exact-first and greedy phases produce those identities; arbitrary unmatched spans and many overlap magnitudes remain possible. A scalar metric or assignment objective admits multiple partitions, matrices and tied assignments. Preserve the operation tag, dimensions, matching policy and ordering as conditions; neither mention spans nor unique assignments are reconstructed.

## exact-before-residual

Type: stage_priority.

Exact spans are consumed before residual edge construction. Residual indices refer to the remaining stable vectors, so removing duplicate exact spans must not reorder survivors or allow sorted edges to reuse them. The independent duplicate-span fixture observes the exact-first boundary and the residual continuation together.

## total-key-ties

Type: tie_order_preimage.

Sort keys are (Reverse(overlap), residual_prediction_index, residual_gold_index). Selecting an edge only when both endpoints are free reproduces repeated maximum-overlap scanning, including zero weights. Equal objectives alone do not constrain identities: the test compares actual stable match identities and separately inspects the ordering of unique total keys.

## bounded-selector

Type: resource_fallback.

Sorting is attempted only when min(P,G)>=128 and checked P*G<=262144. Multiplication overflow, excess edge count, small side or failed reservation returns to scanning before output mutation. Cost and allocation behavior are piecewise; source-local threshold and cap cases are required in addition to the independent supplied fixtures.

## greedy-not-assignment

Type: policy_non_equivalence.

For a residual group containing [[5,4],[4,0]] and zero padding, greedy takes the leading 5 then 0 while global assignment can take 4+4. This rejected replacement is part of the model, even though assign_max is a legitimate separate metric component. Integer negation joins maximum and minimum assignment objectives only under exact numeric and rectangular shape obligations.

## augmentation-reset

Type: state_reset_obligation.

Hungarian scratch minima and visited flags must be reset at every augmentation; potentials and the partial matching retain their intended state. Reusing buffers without this split can leak a previous search into the next. Empty, rectangular/transposed, negative and repeated-call observations constrain this seam; no allocation-count or runtime gain is asserted without measurements.

## wave27-coreference-pipeline-span-head-transport

MSpan=(span, span_head(tree,span)); OutMention=(span,detected_head,visible_cluster_ordinal,note). Equal spans need not imply identical heads across gold/predicted trees. retree requires equal sentence/word forms but can change head matching.

## wave27-coreference-pipeline-partition-visibility

Let C partition [0,M). Visible output filters v(m,C)=!internal(m) && !(coord(m)&&|C|=1&&!override), drops empty projected clusters, then assigns consecutive ids. A group plus one invisible member has |C|=2 and may survive; filtering and singleton detection do not commute.

## wave27-coreference-pipeline-failure-effects

CLI eval mkdir precedes read. run gold parse/self-check precede length check and report; diagnostic file write precedes leak check; CoNLL-U write precedes caller report-file write. Failure may leave earlier effects; there is no cross-file transaction.

## wave27-coreference-pipeline-micro-metrics

Aggregate Counts=sum over aligned documents, then compute ratios/F1. Averaging per-document F1 is a different function. Empty denominators in scorer map0, whereas CLI mention-stat raw recall/precision can print NaN; do not unify their zero semantics.

## evidence

# coreference-observation evidence

## checks (evidence)

See models/mova.groupant/model.md#transfer-evidence, transfer-benchmark.md and transfer-validation.md for the combined receipts. Source revisions are bound by this manifest and paired partial syntax. Finite checks do not establish universal equivalence, floating-point optimality, parser quality or whole-system speed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 finite transfer integration evidence.

### 2026-10-30 · checks (evidence)

2026-10-30: 14 focused checks passed twice in debug and twice in release with identical normalized observations. Nine source-local score checks and five independent joins. The integrated release runner also passed 2,694 differential cases per invocation. Stable-tie and scratch-reset source controls and the root sign-adapter control failed and were restored.
