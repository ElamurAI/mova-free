# octopus-typed-slots

## model

# octopus-typed-slots

## Descriptant

Typed prompt arguments (URL, host) for configured deputy capabilities: strict validation, placeholder masking before parsing and binding through the native parse. Source: `tools/octopus/src/slots.rs`.

## Forward approximant

mask(prompt): split on whitespace, trim punctuation (a closing parenthesis stays while balanced); a word with '://' must validate as http/https URL without user information, valid host, port 1..65535, printable ASCII; a dotted host shape or IPv4 literal is a host; host/path without scheme is refused; reserved zurl*/zhost* words refused; non-public hosts (loopback, private, link-local, CGNAT, reserved names localhost/.local/.internal/.lan/.home.arpa/.test/.invalid) refused unless policy allows; each argument k becomes placeholder Zurl<k>/Zhost<k>. bound_head(w,node,heads): walk up while the node's relation base is obj/iobj/obl/nmod/appos/conj/compound/flat/list/dep/nsubj and no dependent is an exclusion marker; return the first request head reached.

## Invariant

No argument value enters a plan unless its placeholder node reaches a request head through argument relations; every masked value passed validation; text without URL/host shapes is unchanged by mask.

## Cost

O(prompt bytes) masking; bound_head O(n^2) for n native words (n<=160).

## Composition

Parent: `native-octopus-deputy`. Slot rules bounded (<=4 per capability, disjoint kinds, max<=16); arguments validated before parsing; binding only through argument relations to a request head.

## behavior_complement

Not done: IPv6 literals, schemeless URLs, internationalised domain names in Unicode form, DNS resolution (non-public addresses behind public names are left to the fetcher), coreference ('fetch it'), quoted or negated scopes (refused by the resolver). Counterexample: 'Fetch A because B has a copy.' leaves B under an adverbial clause, so the prompt is refused (red when binding ignores the parse). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a plan's arguments one recovers the validated values (scheme and host lowercased, '/' added to an empty path) and their native nodes; the original spelling (case, trailing punctuation) is recoverable only from the stored prompt. Many prompts map to one argument list; no unique inverse. No unique inverse is claimed.

## tests

`a_colon_hands_the_quotation_to_the_preceding_request` (vocative, parataxis and obj attachments bind; nearest of two heads; negative controls: comma instead of colon, no request head, a word after the span, a span before the request, an exclusion marker). Native Dolphin-pod test with en.bin 2483516a: "Ask the X Dolphin: «Check this diff.»" and the "…" form bind for four Dolphins; quotations before the request, in a later sentence or after "Note:" are never sent. Removing the colon rule turns the native test red (restored). Two identical runs.

### Checks

## History

### 2026-11-04 · document

#### Dolphin pod (2026-11-04)

Slot kind `text`: quoted spans in «», “” or "" are masked first and whole as Ztext placeholders; their words never reach the parser; reserved placeholder words outside spans, unclosed or nested quotes, empty or over-6000-byte spans and control characters refuse with argument_invalid.

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: SlotKind::Id (catalog slug: >=3 hyphen-joined lowercase alphanumeric parts with a letter and a digit, <=128 bytes), placeholder Zslug*, reserved prefix zslug; mask_with(prompt, policy, ids) masks ids only when enabled; mask() keeps its behaviour.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/octopus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL (16 tests including the unchanged native controls; two identical runs on 2026-11-04).

Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site (2026-11-04); not a proof.

2026-11-04: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs on 2026-11-04)

### 2026-11-05 · document

#### Epoch 2026-11-05: colon-introduced quotations

Observation: en.bin 2483516a and the model before it (bff784ac) attach the placeholder of "Ask the Opus Dolphin: «…»" as `vocative` of «ask» (with «consult» as `obj`), so `bound_head` refused it (`argument_unbound`); a trailing word ("…» Thanks.") changed the attachment. Forward: colon_head(w, node, heads) = max{h ∈ heads : h < node−1} when w[node−1] is the PUNCT «:», every word after node is PUNCT, and no exclusion marker hangs from node; else None. The deputy uses it for text arguments only and only when bound_head gives None (the parse decides first). Invariant: the bound head precedes the colon and belongs to an action of the same sentence; the slot-kind and slot-count checks still apply, so a request without a text slot (crab status) still refuses. behavior_complement: a quotation before the request, a colon with no request before it, words after the span ("…» now."), URLs/hosts after a colon, and a colon in another sentence are not bound by this rule. reverse_reconstruction: from a bound text argument one recovers the span value and its node; whether the parse or the colon rule bound it is recoverable only by re-running bound_head on the stored native words.

#### Epoch 2026-11-05: colon rule for every kind

`colon_head` is position-based and was already kind-agnostic; `bind` used it for text only. Under en.bin 2483516a "Survey this paper: paper-2023-gamma." hangs the identifier placeholder as `vocative` of «survey» ("Describe this paper: X." gives `obl`), so the request lost its argument. Now every kind uses the rule; slot kind and count checks still apply. Red control: text-only restriction fails the Whale English test.

### 2026-11-05 · tests

2026-11-05: `a_colon_hands_the_quotation_to_the_preceding_request` (vocative, parataxis and obj attachments bind; nearest of two heads; negative controls: comma instead of colon, no request head, a word after the span, a span before the request, an exclusion marker). Native Dolphin-pod test with en.bin 2483516a: "Ask the X Dolphin: «Check this diff.»" and the "…" form bind for four Dolphins; quotations before the request, in a later sentence or after "Note:" are never sent. Removing the colon rule turns the native test red (restored). Two identical runs.
