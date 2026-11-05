# mova-seaspider-robots

## model

# mova-seaspider-robots

## Descriptant

RFC 9309 robots.txt parsing, product-token group selection and the longest-match allow/disallow decision with the deciding rule. Source: `tools/seaspider/src/robots.rs`.

## Forward approximant

Strip BOM, cut at 500 KiB on a char boundary, drop comments, group consecutive User-agent lines; tokens lowercased with any '/version' suffix dropped. rules_for(token): merged own groups, else '*' groups, else none; empty Allow/Disallow ignored; patterns percent-canonicalised and compiled to anchored regexes ('*' -> '.*', final '$' anchors); crawl-delay = max of finite non-negative values. decide(path): '/robots.txt' allowed; longest matching pattern wins, Allow wins a tie, no match allows.

## Invariant

decide is a pure function of (rules, canonical path); a path matched by no rule is allowed; for matched rules the verdict equals the allow flag of a maximal-length pattern, Allow preferred at equal length.

## Cost

Parse O(bytes); rules_for O(lines) plus regex compilation; decide O(rules x path) with linear-time regex matching.

## Composition

Used by crawl::fetch_robots (once per origin per TTL) and robots_decision (per URL and redirect hop).

## behavior_complement

Domain: arbitrary text and paths. Not modelled: non-standard directives other than crawl-delay (Sitemap, Host, Clean-param are ignored), the Spider's peer-crawler consent, case-insensitive path matching. Counterexample: with 'User-agent: *\nDisallow: /' and an own group allowing '/', the own group wins and '/page' is allowed (own_group_replaces_star_and_ignores_version). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a verdict with its rule one recovers the deciding pattern and its kind; the robots.txt text is not recoverable (other rules, comments and groups are lost). Every robots.txt containing that maximal rule and no longer matching rule is a compatible antecedent. No unique inverse is claimed.

## tests

### Checks

Eight unit tests: star fallback, own group with version suffix and a negative control for another robot, grouped and merged agents, longest match and Allow tie, '*' and '$', crawl-delay and empty Disallow, /robots.txt and percent forms, no groups.

## History

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
