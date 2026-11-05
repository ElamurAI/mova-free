# world-school-actors-world-marginal-election

## forward

For each side c, aggregate T_c(x)=sum votes(e) where e_c=x and x!=9 in i128. Elect the smallest signed x attaining maximal T_c. Missing verb or absent subject evidence yields None; absent receiver evidence defaults to zero.

## invariant

The two elections are independent and can produce a pair absent from the joint vote map. Sentinel 9 never competes; negative vote totals still compete. Absent subject evidence does not mean zero effect.

## cost

E joint effects: two ordered marginal aggregations O(E log E), O(E) storage, then linear maxima. i128 totals assume the supported finite address space.

## composition

Actors codec first merges duplicate joint effects and clamps them; election sees those retained i64 counts, not the original unbounded row totals. play consumes the elected pair; an absent subject can abort play despite receiver evidence.

## behavior-complement

Exclude a selected coordinate 9, a nonmaximal retained marginal, or a larger signed winner on equal votes. Exclude Some when the verb is absent or every subject coordinate is sentinel; a missing receiver alone cannot force None.

## reverse-reconstruction

For an observed (a,b), retain every joint map whose non-sentinel subject marginal elects a and whose receiver elects b, or is empty with b=0. Winner inequalities are strict against smaller coordinates and nonstrict against larger ones. None retains both absent verb and no-subject fibers.

## correlation_loss

Joint maps differing only in association but sharing both weighted marginals are observationally equivalent for effect; elected pair need not be an observed joint effect.

## sentinel_asymmetry

A receiver-only map ((9,1),n) yields None, whereas ((-1,9),n) yields Some((-1,0)); evidence absence is directional.

## signed_tie

Among equal marginal sums, numerical signed order chooses -1 before 0 before 1. Map insertion order cannot break the tie.

## codec_boundary

Summation of marginal retained counts occurs after per-joint-codec saturation; summing original duplicate TSV votes first across a marginal is not interchangeable with this pipeline.

## checks

Exact world library selections: school::wave12_models::actor_retention_and_partial_vote_projection. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
