# mova-seaspider-http

## model

# mova-seaspider-http

## Descriptant

End-to-end tests of the crawler against a local threaded HTTP/1.1 server on 127.0.0.1 with a.test/b.test/c.test pinned to it. Source: `tools/seaspider/tests/http.rs`.

## Forward approximant

Start a per-test server that logs (host, path, User-Agent, arrival instant) and answers from a closure; build a Config with private addresses allowed and host overrides; run Crawler::run; assert outcomes in pages.jsonl, which paths reached the server, server-side spacing >= 2 s (or >= crawl-delay), redirect chains, truncation/timeout/gzip, journal projections of two runs, resume counts, links and follow results, CLI get/stats/usage, private-address refusal.

## Invariant

Tests never contact a host other than 127.0.0.1; every negative assertion is a path the server must not have seen.

## Cost

About 12 s wall time (dominated by the 2 s per-IP waits); tests run in parallel, each with its own server and Politeness.

## Composition

Exercises config, crawl, polite, fetch, robots, store, links and the main binary together.

## behavior_complement

Not covered: TLS, IPv6, real DNS, servers that ignore Connection: close, very large stores, concurrent writers. Counterexample role: each of the three negative controls turns one of these tests red. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A passing run shows the fixture behaviours held on that run; it does not reconstruct server timings or prove behaviour on other servers. No unique inverse is claimed.

## tests

### Checks

Nine tests; negative controls listed in the group model.md#tests.

## History

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
