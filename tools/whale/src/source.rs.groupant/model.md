# mova-whale-source

## Descriptant

Source adapters: bounded teacher context and full comparison text for UTF-8 text and EPUB. Source: `tools/whale/src/source.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

`text`: read at most max+4 bytes, keep the longest valid UTF-8 prefix within max bytes, mark partial when file size exceeds it. `epub`: container.xml rootfile -> OPF manifest/spine; visit spine XHTML members in order, drop script/style, keep entity references literal when unknown, stop at max bytes. `full_primary`: rehash, extract the whole UTF-8 file or all HTML/XHTML members sorted by name, rehash again.

## Invariant

Context length <= max bytes; no member path escapes the archive; no external resource or entity is loaded; full_primary returns text only when the file hash equals the pin before and after extraction.

## Cost

O(min(file, max)) for text context; EPUB O(expanded members) bounded at 64 MiB and 4096 members.

## Composition

Used by whale::prepare (context) and whale::publish (originality primary); the adapter per item format is chosen by the specification kind.

## behavior_complement

Excluded: files above 64 MiB, archives above 4096 members, absolute/percent/backslash/escaping hrefs, non-text spine items, invalid UTF-8 inside the kept prefix. Counterexample: a script element containing instructions never reaches the context (epub test). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a context one recovers a prefix of the visible text; all sources with the same visible prefix (any tail, any markup, any script content) are compatible antecedents. Partial flags separate complete from truncated observations. No unique inverse is claimed.
