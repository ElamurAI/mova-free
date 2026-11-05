# mova-seaspider-polite

## model

# mova-seaspider-polite

## Descriptant

Per-IP and per-host next-free moments (at least 2 s per IP), public-address filtering, cached DNS with overrides and a pinned reqwest resolver. Source: `tools/seaspider/src/polite.rs`.

## Forward approximant

reserve(host, ips, host_delay, now): over sorted unique ips pick the minimal free = max(now, host_next, ip_next[ip]) (ties: lowest address); book ip_next = start + interval and host_next = start + max(host_delay, interval). finish(host, ip, end): ip_next >= end + interval, host_next >= end + max(host_delay, interval). Dns.resolve: IP literal, override, fresh cache or tokio lookup (sorted, deduplicated), then drop non-public addresses unless allowed. is_public: IPv4 refuses loopback, private, link-local, shared 100.64/10, broadcast, multicast, documentation, benchmarking, 0/8 and 240/4; IPv6 is public only inside global unicast 2000::/3 minus 2001::/23 (Teredo, benchmarking, ORCHID), 2001:db8::/32 and 3fff::/20, while IPv4-mapped, NAT64 64:ff9b::/96 and 6to4 2002::/16 inherit the verdict of their embedded IPv4. pin(host, ip) makes PinResolver answer only that address.

## Invariant

For two requests reserved and finished through one Politeness on the same IP, start2 >= end1 + interval with interval >= 2 s (interval checked by config); a host's next start is >= its previous end + max(host_delay, interval). The resolver never returns an address that was not pinned. Without allow_private, no resolved address outside 2000::/3 survives except an IPv4-mapped or NAT64 address whose embedded IPv4 is public, and no embedded private IPv4 (mapped, NAT64, 6to4) survives.

## Cost

reserve/finish O(|ips| log n) BTreeMap operations; DNS one lookup per host per TTL.

## Composition

crawl::polite_get reserves, sleeps until the start, pins, fetches, finishes; fetch::client installs the resolver.

## behavior_complement

Domain: one process. Not covered: other processes or machines sharing an IP, IPv6 privacy addresses, CDN fan-out (no exception: a shared CDN address throttles all its hosts), DNS rebinding between resolve and connect is excluded by pinning. IPv6 special ranges are a closed list; a block IANA reserves later inside 2000::/3 is accepted until the list is updated. Teredo and IPv4-compatible forms are refused outright, so a public host reachable only through them is not crawled. Counterexample: the earlier IPv6 branch (loopback, unspecified, multicast, fc00::/7, fe80::/10, 2001:db8::/32 only) accepted ::127.0.0.1, 64:ff9b::10.0.0.1 and 2002:0a00:0001::1 (ipv6_special_and_embedded_ranges_are_not_public red). Removing the IP booking lets two hosts on one IP start 2 ms apart (negative control, per_ip_two_second_spacing_across_hosts red). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the request log (ip, start, end) the reservation sequence is recoverable in order; from a start time alone the cause of a wait (IP vs host vs crawl-delay) is not unique. No unique inverse is claimed.

## tests

Five unit tests (shared IP across hosts, host delay vs IP spread, finish from end, public ranges, IPv6 special and embedded ranges) and the end-to-end spacing tests. Negative control: with the earlier IPv6 branch restored, ipv6_special_and_embedded_ranges_are_not_public fails.

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs).

These are finite fixtures; the only live run was the smoke against the project's own site.

### Checks

## History

### 2026-11-04 · behavior_complement

Domain: one process. Not covered: other processes or machines sharing an IP, IPv6 privacy addresses, CDN fan-out (no exception: a shared CDN address throttles all its hosts), DNS rebinding between resolve and connect is excluded by pinning. IPv6 special ranges are a closed list as of 2026-11-04; a block IANA reserves later inside 2000::/3 is accepted until the list is updated. Teredo and IPv4-compatible forms are refused outright, so a public host reachable only through them is not crawled. Counterexample: the earlier IPv6 branch (loopback, unspecified, multicast, fc00::/7, fe80::/10, 2001:db8::/32 only) accepted ::127.0.0.1, 64:ff9b::10.0.0.1 and 2002:0a00:0001::1 (ipv6_special_and_embedded_ranges_are_not_public red, 2026-11-04). Removing the IP booking lets two hosts on one IP start 2 ms apart (negative control, per_ip_two_second_spacing_across_hosts red). Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · tests

Five unit tests (shared IP across hosts, host delay vs IP spread, finish from end, public ranges, IPv6 special and embedded ranges) and the end-to-end spacing tests. Negative control (2026-11-04): with the earlier IPv6 branch restored, ipv6_special_and_embedded_ranges_are_not_public fails.

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
