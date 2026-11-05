# mova-json value, layout and path library

## domain
UTF-8 JSON documents (RFC 8259 grammar, nesting <= 512, no lone surrogates) as read from Mova configs, registries, groupant manifests and JSONL logs; paths in the `a.b[2].c[k=v]` grammar; edit arguments already parsed into values. Files are trusted, local and not concurrently written.

## forward
parse: text -> ordered Value; object keys keep order (duplicates kept), numbers keep their lexeme, strings keep the source escape text when it differs from the canonical escape. load_str detects the layout by rendering the value in candidate styles (detected N-space pretty, pretty 2/1/4, compact, spaced) with the input's trailing newline and accepts the first that reproduces the input exactly (canonical = true); otherwise canonical = false with the detected style. render(v, style) is the Python `json.dumps(indent=N, ensure_ascii=False)` / serde_json pretty layout (empty containers `[]`, `{}`), compact `,`/`:` or spaced `, `/`: `. Paths: get/get_mut walk keys, indices (negative from the end) and selectors that must match exactly one object element (string value or scalar lexeme). set replaces or inserts a final key (sorted place when the object's keys are sorted, else appended); del removes; push appends; insert_before splices before the single element equal to the anchor; remove_value removes every equal element; replace_str substitutes in string values (not keys) and counts strings and occurrences. load_for_edit refuses a non-canonical document unless reformat is requested; save renders and writes through a sibling temp file and rename only when bytes changed. jsonl_count tallies the value at a path per line (missing -> `<missing>`, invalid lines listed); jsonl_filter returns the matching lines.

## invariant
- For a canonical document, render(parse(t), detected style) == t byte for byte.
- An edit changes only the rendered bytes of the edited values (and their separators); all other strings, numbers and key orders render as read.
- A selector or anchor that matches zero or several elements is an error, never a silent first match; remove_value with no match is an error.
- No write happens for a non-reproducible layout without explicit reformat; an unchanged rendering never rewrites the file.

## cost

parse and render are O(bytes); load_str renders at most six candidates for a non-canonical input and one for a canonical input in the detected layout (2.5 MB contracts.json: about 25 ms release for check or get). Path steps are O(container length) each; set on a sorted object is O(keys). Memory is the value tree plus one rendering.

## behavior-complement
Not done: duplicate-key detection, number normalization (1.0 and 1 differ), comments or trailing commas, JSON5, tab or mixed indentation, arrays printed inline inside a pretty layout (such files are reported non-canonical and refused for edit), locking against concurrent writers, keys edited by replace_str, schema validation.

## reverse-reconstruction
From the output file and the command one recovers the edited location and new value; the previous value is recovered only for del (printed) and for replace_str when NEW does not already occur (inverse replace). set over an existing value, remove_value and a reformat are many-to-one: equal outputs arise from different prior values or layouts, so no unique inverse is asserted.

## layout-identity

Type: round_trip_identity. Domain: tracked JSON files of the repository., 4793 of 4863 tracked `*.json` files were reproduced byte for byte; the other 70 (mostly `algorithms/src` sidecars written with blank lines in empty arrays) are refused for edit without --reformat. Negative control: a one-byte layout change (`": "` -> `": "`) turns canonical false; wrong indent or newline renderings differ.

## checks
`cargo test --release` in tools/mova-json: round trip of models/contracts.json, two groupant manifests, the private crab.json and deputy.json (when present; not in the public tree) with layout-mutation red controls; escapes/lexemes; path edits on a crab-shaped fixture with ambiguous-selector, missing-anchor, absent-key and bad-path red controls; sorted insertion; replace_str; JSONL tallies.

## epoch-memory

Written to replace the Octopus's ad-hoc Python JSON edits with one deterministic Rust tool. Own value type instead of serde_json because the workspace lock has serde_json without preserve_order, and enabling that feature would reorder maps in every other crate; keeping lexemes and raw escapes also gives byte identity that a re-serializing library does not.

## History

### 2026-11-05 · cost

parse and render are O(bytes); load_str renders at most six candidates for a non-canonical input and one for a canonical input in the detected layout (2.5 MB contracts.json: about 25 ms release for check or get, 2026-11-05). Path steps are O(container length) each; set on a sorted object is O(keys). Memory is the value tree plus one rendering.

### 2026-11-05 · layout-identity

Type: round_trip_identity. Domain: tracked JSON files of the repository. On 2026-11-05, 4793 of 4863 tracked `*.json` files were reproduced byte for byte; the other 70 (mostly `algorithms/src` sidecars written with blank lines in empty arrays) are refused for edit without --reformat. Negative control: a one-byte layout change (`": "` -> `":  "`) turns canonical false; wrong indent or newline renderings differ.

### 2026-11-05 · epoch-memory

2026-11-05: written to replace the Octopus's ad-hoc Python JSON edits with one deterministic Rust tool. Own value type instead of serde_json because the workspace lock has serde_json without preserve_order, and enabling that feature would reorder maps in every other crate; keeping lexemes and raw escapes also gives byte identity that a re-serializing library does not.
