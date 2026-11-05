# world-school-actors-world-event-play

## forward

Fold all initial holdings by actor. For each ordered event obtain (-1,0) for pay, else eff(verb) or return None. Quantity is e.n or (pre-event subject minus keep)*p/q. Update subject first; then explicit receiver, else first original holder different from subject if subject was not an original holder. Return the signed map without pruning.

## invariant

All products, sums and differences must fit i64 and fractional q must be nonzero; truncating integer division is retained. Equal subject/receiver identities observe both updates. Unknown effect aborts the local result, not external callback side effects.

## cost

H holdings,J events,A actors: O((H+J)log A+JH) plus event/string clones, O(A) map. Callback cost additional.

## composition

Parent calls play before answer_of. Fractions inspect state after prior events but before this event; fallback receiver consults original holds, not the updated map. eff is supplied in native tests; dictionary/parser calls are omitted.

## behavior-complement

Exclude a successful result if a consulted non-pay effect is None; exclude final clamping of negative actors; exclude computing a fraction from the post-event balance. For supplied pure eff, no mutation of Reading holdings/events occurs.

## reverse-reconstruction

Given final map and a fixed event sequence/effects, compatible initial holdings are every ordered list whose aggregated counts replay to that map. Fractions, integer truncation, implicit receiver choice and duplicate holdings make this fiber many-to-one; retain intermediate maps and original-holder identities.

## fraction_time

Event j quantity depends on W_(j-1)[subject], so transferring three before all-but-two differs from computing both quantities at the initial ten.

## transfer_conservation

When both parties exist, are represented and a+b=0, the event preserves sum of actor balances, including identical actor identities; absent implicit receiver need not conserve.

## receiver_selection

Fallback selects the first eligible original holder; reordering original holds may change receiver despite identical initial aggregated map.

## unknown_effect

A pure callback returning None suppresses the entire Option map, although earlier local updates occurred. pay bypasses callback and uses the fixed debit. External callback effects would not be rolled back.

## checks

Exact world library selections: school::wave12_models::supplied_reading_transfer_and_fraction_use_prior_state. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
