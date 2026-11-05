# Sea Spider — `mova-seaspider`

Sea Spider is one of Mova's animals (Rust on Mova). It takes requests in English from the Octopus — a large language model (LLM) — or from Crab. See [the sea animals of Mova](../../docs/animals.md).

Sea Spider is Mova's polite link downloader, named after the sea spiders (Pycnogonida), marine
arthropods of every ocean: a Rust library and a CLI (`seaspider`). It fetches a queue of URLs as the abstract robot **MovaBot**, honours robots.txt,
keeps at most one request per IP every 2 seconds, and stores raw pages compressed.

It does not run JavaScript, keep cookies, log in, submit forms or follow links unless asked.

`seaspider serve` takes arbitrary English prompts on a private socket,
understands them with Mova's native deputy (`tools/octopus`) and answers in English, with the
structured result alongside (see «In English» below).

## Commands

    seaspider fetch CONFIG QUEUE     fetch a queue, print a summary (JSON)
    seaspider get STORE URL          write the latest stored body of URL (or sha256:HEX) to stdout
    seaspider robots CONFIG URL      fetch robots.txt politely and explain the decision for URL
    seaspider stats STORE            counts of the journal and segments (JSON)
    seaspider serve CONFIG DEPUTY_CONFIG SOCKET   answer English prompts on a private socket (STOP ends it)
    seaspider ask [--from CALLER] SOCKET ID [urgent] < text
                                     send arbitrary English; prints the English answer, then JSON

`fetch`, `robots` and `serve` take the singleton lock first (see «One Sea Spider per host»); while
another Sea Spider holds it they print who runs and exit with status 5.

QUEUE has one URL per line: a JSON object with `"url"`, or a TSV/plain line whose first field is
the URL. Empty lines and `#` lines are skipped; duplicates are fetched once.

Build and test (standalone crate with its own workspace):

    cargo build --release --manifest-path tools/seaspider/Cargo.toml --offline
    cargo test --manifest-path tools/seaspider/Cargo.toml --offline

## How it works

- **MovaBot.** The identity is configuration: `robot.token` (RFC 9309 product token, default
  `MovaBot`), `robot.version` (default the crate version) and `robot.contact_url`, a public page
  about the robot. The User-Agent is `MovaBot/<version> (+<contact URL>)`. The contact must be an
  http(s) page with no `@`, no user information and no `mailto:`; no personal data goes into
  headers or URLs. The public example uses the placeholder `https://example.invalid/movabot`.
- **robots.txt** (RFC 9309). Fetched per origin (scheme, host, port) before the first request and
  cached for `robots.ttl_secs` (at most 24 h). The groups naming the product token apply (a
  version suffix such as `MovaBot/1.0` matches); otherwise the `*` groups; otherwise nothing.
  Longest matching pattern wins, Allow wins a tie, `*` and a final `$` work, percent-encoding is
  compared in one form, at most 500 KiB is parsed, `/robots.txt` is always allowed. HTTP 4xx
  (except 429) means unavailable: everything is allowed. HTTP 5xx, 429 or a network failure means
  unreachable: the origin is closed for `robots.unreachable_retry_secs`. Up to
  `robots.max_redirects` redirects are followed. `Crawl-delay` raises the host delay to
  `max(crawl-delay, 2 s)`; above `politeness.max_crawl_delay_secs` the URL is recorded as
  `deferred` instead of waited for.
- **Politeness.** Every request — robots.txt, page, redirect hop — reserves a slot: the resolved
  IP must have been idle for `politeness.ip_interval_ms` (minimum and default 2000; lower values
  are refused) since the end of its previous request, and the host must be past its own delay.
  Hosts on one IP share the IP's limit. A host with several IPs uses the one free first; the
  connection goes to exactly that IP (pinned resolver, no idle connections kept). Non-public
  addresses (loopback, private, link-local, …) are refused unless
  `network.allow_private_addresses` is true.
- **Fetch.** rustls, gzip/brotli/deflate decoding, no cookies, redirects handled by the crawler:
  every redirect target is checked against its own robots.txt and reserved again, up to
  `fetch.max_redirects`. Bounded connect and whole-request timeouts; the decoded body is cut at
  `fetch.max_body_bytes` and marked `truncated`.
- **Order.** Deterministic: URLs keep file order within a host; hosts are visited round-robin in
  sorted order; followed links are appended in page order. Timing changes only the sleeps.
- **Store.** A store directory holds `seg/<k>.seg` (bodies as zstd frames with frame checksums,
  appended; a new segment once the current one reaches `storage.segment_max_bytes`),
  `pages.jsonl` (the journal) and `links.jsonl`. Identical bodies are stored once (by sha256).
- **Journal.** One JSON line per event: `kind` `robots` (a robots.txt fetch, body stored when
  2xx), `intent` (written before a page's first request), `page` (the outcome), `uncertain`.
  A page line has `url`, `final_url`, `outcome` (`fetched`, `disallowed`, `deferred`, `error`),
  `status`, `chain` (each hop's URL, status and IP), `headers` (content-type, content-length,
  content-encoding, content-language, last-modified, etag, location, retry-after, x-robots-tag,
  date), `content_type`, `fetched_unix`, `ms` (wall time of the visit including politeness waits),
  `seg`/`off`/`len`, `body_bytes`, `truncated`, `sha256` of the raw (decoded) body, `robots`
  (robots URL, source, group, decision, deciding rule, crawl-delay), `depth`, `parent`,
  `user_agent`.
- **Crash safety.** A body frame is written and its segment fsynced before the journal line that
  points at it; every journal line is fsynced before the next request; directories are fsynced
  when files are created. A crash leaves at most unreferenced bytes at a segment end (harmless)
  and a torn last journal line, which the writer cuts on open. An `intent` with no later outcome is
  an uncertain fetch (the request may have reached the server): the next writer records an
  `uncertain` line for it once, and the URL is fetched again.
- **Resume.** A URL whose latest page record is not an `error` is skipped, unless
  `resume.refetch_after_secs` is set and that record is older.
- **Links.** Every fetched HTML page adds a line to `links.jsonl`: absolute normalised links
  (http(s), no fragment, no `utm_*`, no image/media/archive extensions) with a nofollow mark
  (`rel` nofollow/ugc/sponsored, `<noindex>`, meta robots nofollow, `X-Robots-Tag: nofollow`).
  Following is off by default; `follow.enabled` follows same-host, non-nofollow links up to
  `follow.max_depth` and `follow.max_pages` per run.

## In English

`seaspider serve CONFIG DEPUTY_CONFIG SOCKET` runs Sea Spider as a persistent animal. The prompt is
arbitrary English text plus an optional priority (`urgent`/normal); there is no command language.
Mova's native deputy (`mova_octopus::Deputy`: Mova's annotator, the world tree reader and the
configured capabilities) reads it; URLs and host names reach a capability only as typed arguments
bound through the parse (see `tools/octopus/README.md`, «Typed arguments»). The answer is English
realized deterministically (`answer.rs` over `mova_octopus::realize`: number agreement, plurals and
past forms from Mova's morphology, lists, «I could not … because …» with the robots rule or HTTP
status). No language model is involved.

Capabilities (handler `seaspider`, parameter `operation`; vocabulary in `deputy.example.json`):

| operation | slot | example prompts | answer |
|---|---|---|---|
| `fetch` | `urls`: 1–16 URLs | "Fetch https://a.example/.", "Please fetch these pages: A, B", "Could you download A?" | queued at once: «I have queued 2 pages for fetching (octopus-ID-0): A and B. 1 job is ahead of it.» |
| `robots` | `target`: one URL or host | "Are we allowed to fetch mova.elamur.ai?", "Can you read A?", "May we fetch A?" | «Yes, …/robots.txt allows MovaBot to fetch A (rule Allow: /).» / «No, … disallows A for MovaBot (rule Disallow: /x).» |
| `stats` | `host`: optional | "How many pages have you stored?", "… stored from mova.elamur.ai?" | «I have stored 3 pages from a.test. I could not fetch 1 more page: robots.txt disallowed 1.» |
| `page` | `url`: one URL | "What did you get from A?" | «I fetched A at 2026-11-04 07:52 UTC: HTTP 200, text/html, 139 bytes. Its title is “Mova”. It links to 1 page.» |
| `status` | — | "What is the status of the current run?", "Is the queue empty?", "Are you busy?" | «I am working on … : 0 of 1 page is done, now A. 2 jobs are waiting.» and the last job in the past tense |

"Can …?" and "May …?" questions ask for permission and are answered from robots.txt; "Could you …?"
and imperatives are requests. Only `robots`, `stats`, `page` and `status` (declared `observes`) answer
questions: "Is URL fetched?" or "Did you fetch URL?" never fetches. A host target means `https://HOST/`. A refused prompt (negation,
condition, unsupported verb such as "delete", no URL, two URLs where one is expected, a URL outside
the requested action's parse, an invalid or non-public address) does nothing and answers «I did
not act on this request because …», keeping a review candidate in the deputy's queue: teaching
material, not a workaround.

- **One fetch stream.** One worker thread owns the only `Crawler` (the store's writer lock, the
  robots cache, the per-IP timers) and runs jobs one at a time: urgent first, then by arrival. A
  prompt never interrupts running work; priority only orders the waiting queue (at most 64 jobs).
- **Answers never wait for fetches.** Fetch prompts are queued and answered at once. Status, store
  counts and page summaries come from the job state, the job journal and the store journal. A
  robots question is answered from the latest stored robots.txt of the origin while it is within
  its TTL, without a request; otherwise a check is queued and awaited (at most 15 s) only when the
  worker is idle, else the answer says it was queued.
- **Journals.** The deputy ledger keeps each request id (a repeated id returns the recorded answer,
  never a new job); `jobs.jsonl` in the socket directory gets one fsynced line per queued, started,
  url and finished event. A job left queued or started by a restart is recorded as interrupted and
  not replayed. `STOP` in the socket directory ends the server after the current answer; the worker
  finishes the URL it is fetching and records the rest as not started.
- **Socket.** The Octopus transport: an owned 0700 directory, a 0600 socket, SO_PEERCRED owner
  check, bounded NDJSON frames, the angel with restarts and `angel.lock`.
- **Reply.** `{"answer": English, "report": {plan, results, failure, followups, …}}`; `seaspider
  ask` prints the answer, then the JSON line.

### One Sea Spider per host

Sea Spider is off by default: nothing starts it unless a deployment does (the public
[`crab-worker.example.json`](crab-worker.example.json) has `autostart: false`). It is a singleton.
`serve`, `fetch` and `robots` first take an exclusive `flock` on `service.lock` (default
`$XDG_STATE_HOME/mova/seaspider.lock`, else `$HOME/.local/state/mova/seaspider.lock`) and write
their pid, command and start time into it; a second start reads that, answers «Sea Spider is
already running (pid N, serve SOCKET since …; lock …). I did not start a second one.» and exits
with status 5, before it loads the deputy, opens the store or sends a request. The kernel releases
the lock when the holder exits, so a crash leaves no stale lease. The holder runs a few threads
(the socket server, the single fetch worker) but one fetch stream, so robots.txt and the per-IP
timers are never split between processes that share the lock.

### Who may ask

A request may name its caller in the envelope (`"from": "crab"`; `seaspider ask --from crab`); a
request without one is the Octopus's. `service.callers` lists who is answered — default `octopus`,
`crab`, `whale` and `shark` (a compatibility alias of `whale`, admitted by default
for one release). Another caller gets «I did not act on this request because fish is not
among the callers I answer (octopus, crab, whale, shark).» and nothing runs. `requests.jsonl` in the
socket directory gets one fsynced line per request: id, caller, whether it was declared, the
connecting pid and its executable (SO_PEERCRED, `/proc/PID/exe`), priority, the prompt's sha256 and
whether it was admitted; queued jobs carry `asked_by`. The caller name is a declaration by a process
of the socket's owner, not an authentication: the allowlist keeps animals to their roles; the 0700
directory and the owner check keep everyone else out.

Other animals use one shared client, `mova_octopus::relay`:
- **Crab** relays with the deputy capability handler `animal_ask`. Its trusted parameters name the
  animal, its socket and the caller name; the prompt supplies only the quoted English:
  "Ask Sea Spider: «Is the queue empty?»" → Sea Spider's English answer. The socket is never taken
  from a prompt, and a capability without a configured absolute socket is refused when the
  configuration loads. The relay waits at most 30 s for the answer.
- **Whales** call `mova_whale::seaspider::ask_sea_spider` (caller `whale`) with the optional
  `seaspider_socket` of their animal configuration (unset by default: the Whales then do not talk
  to Sea Spider).

## Configuration

JSON, unknown fields refused; relative `store` paths resolve against the configuration file.
`config.example.json` lists every field with its default. Only `robot.contact_url` and `store` are
required.

| field | default | bound |
|---|---|---|
| `robot.token` | `MovaBot` | 1–64 ASCII letters, `-`, `_` |
| `robot.version` | crate version | `[A-Za-z0-9._-]`, ≤ 32 |
| `robot.contact_url` | — | http(s) page, no `@`/userinfo/`mailto` |
| `politeness.ip_interval_ms` | 2000 | ≥ 2000 |
| `politeness.max_crawl_delay_secs` | 60 | ≤ 86400 |
| `robots.ttl_secs` | 86400 | 1–86400 |
| `robots.unreachable_retry_secs` | 3600 | ≥ 60 |
| `robots.max_redirects` | 5 | 5–20 |
| `fetch.connect_timeout_ms`, `fetch.timeout_ms` | 10000, 30000 | 1–300000 |
| `fetch.max_body_bytes` | 10 MiB | 1 B–1 GiB |
| `fetch.max_redirects` | 5 | ≤ 20 |
| `storage.segment_max_bytes` | 256 MiB | 1 MiB–4 GiB |
| `storage.zstd_level` | 9 | 1–19 |
| `resume.refetch_after_secs` | null (never) | |
| `follow.enabled` / `max_depth` / `max_pages` | false / 1 / 1000 | depth ≤ 10 |
| `network.allow_private_addresses` | false | |
| `network.resolve` | {} | host → IP overrides |
| `network.dns_ttl_secs` | 3600 | 1–86400 |
| `service.lock` | `$XDG_STATE_HOME` or `$HOME/.local/state` + `/mova/seaspider.lock` | absolute |
| `service.callers` | `octopus`, `crab`, `whale`, `shark` | 1–16 caller names |

## Checks

`cargo test --manifest-path tools/seaspider/Cargo.toml --offline` runs the unit, end-to-end, singleton
CLI and English tests against a local HTTP server on 127.0.0.1 (no external network). They cover
robots.txt matching and status handling, crawl-delay, the per-IP spacing, redirect re-checks, body,
timeout and redirect bounds, the store and its crash repair, resume, links and follow mode, refusal
of private addresses, the singleton lock and the caller allowlist. The English tests
(`tests/english.rs`) need `MOVA_OCTOPUS_TEST_MODEL` set to a Mova model such as `en.bin`; without it
they report a skip. Negative controls (for example dropping the per-IP booking, ignoring Disallow,
admitting every caller) must turn the corresponding tests red.

## Limits

- One writer per store (an exclusive lock); one request at a time (sequential).
- English understanding is the deputy's bounded skeleton reader: configured vocabulary and native
  UD relations, not general language understanding; unparsed or unusual phrasings are refused.
  Store answers read the whole journal each time (O(journal)). Host robots questions check
  `https://` only. Answers are English only.
- robots.txt and the per-IP/host timers live in one process: the singleton lock keeps two from
  running at once, but a new run fetches robots.txt again, and two runs started back to back are
  not spaced against each other. The lock is per lock file: two configurations naming different
  `service.lock` paths are not excluded from each other.
- Caller names are declarations by processes of the socket's owner, not authentication. A relayed
  request is sent at normal priority.
- No CDN exception: hosts behind one shared CDN address all wait for that address.
- No conditional requests (ETag/If-Modified-Since), no sitemaps, no JavaScript, no cookies.
- `X-Robots-Tag` and meta robots `noindex` are recorded, not acted on; pages are stored raw.
- Content is stored whatever its type; non-HTML bodies are not link-scanned.
- Licences of fetched content are not judged here; what may be used for what is decided
  downstream (`licence/DATA.md`).

## Attribution

Adapted from an internal crawler:
`robots.rs` (RFC 9309 matching), `net.rs` (per-IP next-free-moment scheduling), `links.rs` (the
HTML link scanner, unchanged apart from English comments), `store.rs` (store v2 layout: zstd
frames in append-only segments with a JSONL journal) and `fetch`/`visit`/`robots` of `main.rs`
(bounded GET, manual redirects, robots.txt status handling). Not carried over: the internal crawler's
consent levels and peer-crawler policy, CDN exception, address-pool harvesting and health bans,
AIMD pacing, per-domain dictionaries and deltas, block store and dump, conditional requests.
New in Sea Spider: the configurable MovaBot identity, the 2-second floor, the pinned resolver,
redirect re-checks against the target origin, intent/uncertain records, deduplication by sha256,
the reader, and follow mode.
