# octopus-deputy-socket

## model

# Deputy private socket — source-bound model

## Descriptant

`socket.rs` gives the native Octopus deputy a long-lived private transport. A request is one bounded NDJSON line
`{"schema":1,"id":ID,"mode":"plan"|"run","prompt":TEXT,"priority"?:"urgent"|"normal"}`; the prompt is arbitrary
ordinary text, understood only by the deputy. Every request goes through the same `Deputy::run` path, ledger
reservation and refusals as the CLI; the transport adds no capability.

## Forward

S = (listener in an owned 0700 directory, socket 0600, queue Q). Each step drains every waiting connection: admit(c) =
peer uid equals owner, request line ≤ 16384 bytes ending in newline, envelope fields ⊆ {schema,id,mode,prompt,priority},
schema 1, mode ∈ {plan,run}, priority ∈ {urgent,normal}. Q is ordered by (normal?, arrival); the head is answered by
handle(id, mode=run, prompt) and the reply line is bounded to 4 MiB (else an explicit budget refusal). STOP in the
socket directory ends the loop after the current answer and removes the socket.

## Invariant

At most one handler call runs at a time; a call that started always completes before the next request is chosen,
so a new prompt never interrupts or rewrites unfinished earlier work. Within a priority class order is arrival order.
Only the owner uid reaches the handler; malformed or foreign requests are answered with ok=false and never reach it.

## Cost

Per request: one accept, a bounded read (≤ 16 KiB), an O(|Q| log |Q|) sort, the handler cost (native parsing plus
configured adapters), one bounded write. Idle: 100 ms sleep between polls and a status write every 10 s.

## Composition

The CLI (`main.rs`) builds the Deputy from the pinned config and passes `Deputy::run` as the handler; `ask` and
`ask_with` are the client. Ledger identity (invocation IDs, no replay of uncertain reservations) stays in `lib.rs`.

## behavior_complement

Domain U: byte streams arriving on the socket. Complement of the admitted set: other-uid peers, lines over 16384 bytes
or without newline, non-object JSON, unknown fields (e.g. a `shell` field), modes other than plan/run, priorities other
than urgent/normal, and a non-private socket directory (refused before bind). None of these reach the handler.
Counterexample checked: `{"mode":"exec"}` and an extra `shell` field get ok=false.

## reverse_reconstruction

From a reply line one recovers the request id, ok flag, whether it was urgent and the handler's report; the full prompt
and mode are recoverable only through the deputy ledger keyed by id (the reply does not echo the prompt). Several
requests with the same id map to the ledger's single receipt (completed repeat) or a refusal (conflict); the
transport does not disambiguate them.

## no_preemption_order

Domain: concurrent clients while a handler call runs. Observation: handler call order. Assumption: requests arrive
during the running call. Relation: running call first, then all urgent in arrival order, then normal. Counterexample
(negative control): without the queue sort the order is first, normal, urgent and the test fails.

## angel_keepalive

`angel` keeps the deputy hanging permanently without being a bare daemon: one angel per directory (flock on
`angel.lock`); it builds the handler (model load) and runs `serve`; a load failure, a serve error or a panic is
written to `angel-status.json` (state restarting, error, restarts, backoff) and retried after a doubling backoff
1 s .. 60 s; only `STOP` ends it (state stopped). Inside `serve` a panicking handler fails only its own request
(ok=false) and serving continues. In-flight work is never replayed: the deputy ledger keeps uncertain reservations.
Domain: load/serve outcomes over time. Counterexample (negative control): without the per-request panic catch, the
panicking request gets no reply and the angel test fails.

## checks

### Checks

Three socket tests: round trip with 0700/0600 permissions, handler error, unknown field and wrong mode refused,
over-bound line refused, STOP removes the socket; urgent waits for the running call then precedes a waiting normal
request; a 0755 directory is refused before bind. Negative controls: removing the directory-privacy check makes the
refusal test hang (server binds), removing the queue sort fails the order test. Whole crate: 8 tests twice; with
MOVA_OCTOPUS_TEST_MODEL set, the native integration suite passes. Live: daemon on a private socket answered two
arbitrary prompts through the real deputy (both explicitly refused: question as instruction, unsupported predicate).

## History

### 2026-11-03 · checks

Heading `### Checks (2026-11-03)` stays in checks as `### Checks`.

2026-11-03 angel: a handler factory failing twice then succeeding is retried (restarts=2 recorded), a panicking
request answers ok=false and the next request is served, a second angel on the same directory is refused by the
lease, STOP records state stopped. Negative control: removing the per-request catch_unwind fails the test. Crate:
9 tests twice; native suite with the real model. Live: the running serve process was stopped via STOP and replaced
by `mova-octopus angel`, which answered a real Crab-health request.

### 2026-11-04 · document

#### Epoch 2026-11-04 (English)

2026-11-04: `serve_prioritized` and `angel_prioritized` pass the request's urgency to the handler (so an animal can order work it queues); `serve` and `angel` wrap them with unchanged behaviour and tests.

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: Optional envelope field `from` (caller name, 1..64 lowercase letters/digits/'-'); `serve_requests`/`angel_requests` hand the handler a `Request` with the declared caller and the SO_PEERCRED pid; `ask_from` names the caller and may bound the reply wait. Older entry points are wrappers with unchanged behaviour.

### 2026-11-04 · checks

2026-11-04 (English): cargo test --manifest-path tools/octopus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL (16 tests including the unchanged native controls; two identical runs on 2026-11-04). Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

2026-11-04: cargo test --manifest-path tools/octopus/Cargo.toml --offline with MOVA_OCTOPUS_TEST_MODEL (22 unit tests incl. 4 socket, 2 relay; 2026-11-04)
