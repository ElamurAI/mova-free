# layers — the conductor of the three levels

`layers` is level 3 of the [three levels of logic](../docs/architecture.md): it routes a query to the global level, then to a domain module, then marks it for escalation, and logs every decision with its scores and threshold.

## What it provides

- `Global` — the exact maths core (`math::calc`, with an independent check: if the check disagrees, the global level does not answer), principles (`global::judge`) and consequence chains (`global::consequences`).
- `Domain` — the trait of a domain module: `id`, `version`, `access` (`Public`, `Extended`, `Private`), `covers` (0..1 with a reason) and `solve`.
- `Context` — facts, registered domains, the coverage threshold (default 0.5) and the routing log. `route` tries the global level, then the best domain whose coverage reaches the threshold, else returns `Route::Escalate`.
- `Mode` — `Answer` (just the answer) or `Explain(Training | Debug | UserAsked)` (the answer with its steps).

## Test

```bash
cargo test -p layers
```

## Limits

`Escalate` only marks the query; this crate makes no model call. The global level answers maths and values only; other knowledge comes from the domains registered by the caller.
