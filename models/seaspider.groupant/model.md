# seaspider

## model

# seaspider

## Descriptant

Composition of the public `mova-seaspider` crate (Sea Spider, one of Mova's animals): a polite link downloader running as the abstract robot MovaBot. A deployment supplies the robot's contact page, store path and queues (ours in the private tree); the crate supplies the mechanics. Adapted from the family Spider (Search repository), see `tools/seaspider/README.md`.

## Forward

config validates the robot and bounds -> crawl loads the queue, skips done URLs and orders hosts round-robin -> robots decides per origin (fetched through polite reservations) -> polite reserves IP and host slots and pins the IP -> fetch performs one bounded GET -> crawl follows redirects with a fresh robots decision and reservation per hop -> store writes the zstd frame, then the fsynced journal line -> links writes outgoing links and, in follow mode, same-host links are queued -> main exposes fetch/get/robots/stats.

## Invariant

No page request before an allowed robots decision for its origin; within one process two requests to one IP are separated by at least the interval (>= 2 s) from the end of the first to the start of the second; every journal body pointer refers to fsynced bytes whose sha256 the reader verifies; the processing order depends only on the queue and the responses.

## Cost

Wall time >= (requests to the busiest IP - 1) x 2 s plus latency; per page O(body) compression and O(html) link scan; one fsync per frame and per journal line.

## Composition

Leaves are file groupants under `tools/seaspider/src/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the nine end-to-end tests that run the real crawler against a local HTTP server and measure arrival times on the server side, and the CLI test that reads the store through the built binary.

## behavior_complement

Domain: all configured runs. The system does not: run JavaScript, keep cookies, log in or submit forms; fetch a page whose origin's robots.txt disallows it or is unreachable; follow links unless enabled, and then only on the same host within depth and page bounds; space requests across separate processes; exempt CDN addresses; honour crawl-delays above the configured cap (those URLs are deferred, not fetched); retry an uncertain fetch silently (it is recorded first). Counterexample: a redirect to a disallowed path on another host stops before that request.

## reverse_reconstruction

From `pages.jsonl` and the segments: the visit order, outcomes, redirect chains with IPs, robots decisions with deciding rules, and every stored decoded body (verified by sha256) are recoverable. Not recoverable: exact timings, encoded transfers, dropped headers, bytes beyond the body limit, robots.txt bodies of non-2xx responses. Evidence status: source-derived with finite checks.

## tests

cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs).

Negative controls: dropping the per-IP booking in polite.rs turns per_ip_two_second_spacing_across_hosts and three unit tests red; ignoring Disallow in robots.rs turns robots_disallow_longest_match_and_wildcards and six unit tests red; skipping the robots check on redirect hops in crawl.rs turns redirects_are_rechecked red; restored byte-identical afterwards.

Live smoke (once): robots + fetch against https://mova.elamur.ai/ (2 pages, 4 requests, all 200); no other external host.

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Sea Spider is a host-wide singleton (singleton.rs: flock on service.lock; a second serve/fetch/robots reports the holder and exits 5) and answers only callers on service.callers (default octopus, crab, whale and shark, the Whales' caller name before the Whale rename, admitted for one release), journaling every request in requests.jsonl. Other animals reach it through mova_octopus::relay (Crab's animal_ask, the Whales' ask_sea_spider).

### 2026-11-04 · tests

cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

Negative controls (2026-11-04): dropping the per-IP booking in polite.rs turns per_ip_two_second_spacing_across_hosts and three unit tests red; ignoring Disallow in robots.rs turns robots_disallow_longest_match_and_wildcards and six unit tests red; skipping the robots check on redirect hops in crawl.rs turns redirects_are_rechecked red; restored byte-identical afterwards.

Live smoke (2026-11-04, once): robots + fetch against https://mova.elamur.ai/ (2 pages, 4 requests, all 200); no other external host.
