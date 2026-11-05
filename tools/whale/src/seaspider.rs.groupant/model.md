# mova-whale-seaspider-client

## Descriptant

The Whales asking Sea Spider in English: ask_sea_spider sends one request as caller whale to the configured seaspider_socket through mova_octopus::relay::ask_animal; off unless configured. Source: `tools/whale/src/seaspider.rs`.

## Forward approximant

ask_sea_spider(cfg, id, english, urgent): no socket -> error "not configured"; else ask_animal(socket, "whale", id, english, urgent).

## Invariant

Stateless; reads AnimalConfig.seaspider_socket.

## Cost

One relay round trip (<= 30 s).

## Composition

Parent: `whale-workers`.

## behavior_complement

no Whale capability calls it yet; no fetching by the Whales themselves; Sea Spider decides admission. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Sea Spider's requests.jsonl recovers each Whale request by id and caller. No unique inverse is claimed.
