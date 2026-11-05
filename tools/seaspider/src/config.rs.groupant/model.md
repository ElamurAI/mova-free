# mova-seaspider-config

## model

# mova-seaspider-config

## Descriptant

Configuration of the abstract robot MovaBot and every bound of the downloader; refuses unsafe or personal values. Source: `tools/seaspider/src/config.rs`.

## Forward approximant

Read <= 1 MiB JSON, deny unknown fields, fill defaults, validate: product token [A-Za-z_-]{1,64}, version [A-Za-z0-9._-]{1,32}, contact_url an http(s) page without '@', userinfo, mailto, spaces or parentheses (<= 200 chars); ip_interval_ms >= 2000; robots ttl 1..86400, unreachable retry >= 60, robots redirects 5..20; timeouts 1..300000 ms; body 1 B..1 GiB; redirects <= 20; segment 1 MiB..4 GiB; zstd 1..19; follow depth <= 10; dns ttl 1..86400. Resolve a relative store against the config directory. user_agent() = token/version (+contact_url).

## Invariant

A validated Config never has ip_interval < 2 s, never puts an e-mail address or user information into the User-Agent, and every numeric bound lies in its documented interval.

## Cost

O(config bytes) parse; O(1) validation.

## Composition

Read by crawl (Crawler::new), fetch (client timeouts, Accept) and main (load).

## behavior_complement

Domain: JSON documents. Excluded: unknown fields, ip_interval_ms < 2000, contact URLs with '@'/userinfo/mailto/non-http schemes, out-of-range bounds, files > 1 MiB. The module does not fetch, resolve or check that the contact page exists. Counterexample: ip_interval_ms 1999 is refused (config test ip_interval_never_below_two_seconds). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a valid Config the User-Agent is recoverable exactly; from a User-Agent only token, version and contact are recoverable, not the other bounds. Many JSON texts (field order, omitted defaults) map to one Config; no unique inverse. No unique inverse is claimed.

## tests

### Checks

Four unit tests: defaults and User-Agent, the 2 s floor, personal-data refusal, unknown-field refusal.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: New `service` section: lock (absolute; default $XDG_STATE_HOME or $HOME/.local/state + /mova/seaspider.lock, else temp dir) and callers (1..16 caller names; default octopus, crab, whale, shark — `shark` is the Whales' caller name before the Whale rename, admitted for one release); lock_path() and admits().

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04)
