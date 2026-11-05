# octopus-animal-relay

## model

# octopus-animal-relay

## Descriptant

Asking another animal in English: the deputy capability handler animal_ask relays the quoted English of a prompt to a configured animal socket and returns its English answer; ask_animal is the shared client for animals with a configured socket. Source: `tools/octopus/src/relay.rs`.

## Forward approximant

validate: parameters exactly animal/socket/caller, socket absolute without /../, caller_name, one slot task kind text 1..1. execute: take socket and caller from parameters, the quoted task from the bound slot; ask_from(socket, id, run, task, normal, Some(caller), 30 s); ok -> {answer, request, request_id, failure, reply_sha256}; not ok or unreachable -> error.

## Invariant

Stateless; trusted parameters animal, socket (absolute), caller; one text slot task; request id = invocation id.

## Cost

One Unix-socket round trip bounded by 30 s plus the target animal answer cost.

## Composition

Parent: `native-octopus-deputy`.

## behavior_complement

never takes a socket from a prompt; never relays without a configured absolute socket; no retries; no urgency (normal priority); no authentication beyond the target allowlist and its 0700 directory. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

from the output one recovers the target, caller, request text and English answer; the target journal recovers the request by id. The full reply is kept only as its sha256. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; not a proof.

## History

### 2026-11-04 · tests

cargo test --manifest-path tools/octopus/Cargo.toml --offline with MOVA_OCTOPUS_TEST_MODEL (22 unit tests incl. 4 socket, 2 relay; 2026-11-04)

Negative control 2026-11-04: dropping the configured-socket checks in relay.rs turns configuration_requires_a_configured_socket red; restored afterwards.

cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04) (served Crab-style animal_ask relay against a real Sea Spider)
