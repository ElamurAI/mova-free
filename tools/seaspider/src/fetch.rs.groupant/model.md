# mova-seaspider-fetch

## model

# mova-seaspider-fetch

## Descriptant

One bounded HTTP(S) GET: rustls, gzip/brotli/deflate decoding, no cookies, no automatic redirects, bounded time and body, a fixed header subset. Source: `tools/seaspider/src/fetch.rs`.

## Forward approximant

client(): User-Agent, Accept, connect and whole-request timeouts, Policy::none, no idle pool, pinned resolver. get(url, limit): send, keep status, remote address and the KEPT_HEADERS (each <= 1000 chars), read decoded chunks until limit bytes, cut and mark truncated; errors flattened with causes to <= 400 chars with a '(timed out)' mark.

## Invariant

body.len() <= limit; truncated iff a chunk crossed the limit; no request follows a redirect by itself; no cookie store exists.

## Cost

One TCP+TLS handshake per request (no pooling) plus O(min(body, limit)) bytes.

## Composition

Called only through crawl::polite_get.

## behavior_complement

Excluded: JavaScript, cookies, authentication, POST/forms, HTTP/3, conditional requests, content sniffing beyond headers. Counterexample: a server stalling 6 s against a 1.5 s timeout yields an error, not a partial body (body_size_timeout_gzip_and_redirect_bounds). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From Got one recovers status, kept headers and the decoded prefix; the encoded transfer, dropped headers and bytes past the limit are not recoverable. No unique inverse is claimed.

## tests

### Checks

End-to-end: truncation at 10000 bytes, timeout, gzip decoding, User-Agent seen by the server.

## History

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
