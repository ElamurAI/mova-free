# modelcheck-sections

## Descriptant

`sections.rs` holds the narrow Markdown section grammar that groupant references and the one-file consolidation share: line splitting with fence state, plain H2 headings, section byte selection, `file#Section` splitting, GitHub-style anchors and heading demotion. It is not a Markdown renderer: no setext headings, HTML blocks, lists, containers or inline markup.

## Forward

For text T split at `\n` (CRLF terminators stripped from the line text, kept in spans):

- `lines(T)`: per line the text, byte span and `fenced`, true for fence delimiters and every line inside a fence. A fence opens on 0–3 spaces then ≥3 backticks or tildes (a backtick info string has no backtick) and closes on the same marker, at least as long, followed only by spaces or tabs. An unclosed fence marks every remaining line.
- `h2_names(T)`: rest of each unfenced line starting with `## `; `heading_count(T,S)`: number of unfenced lines equal to `## S`.
- `section_bytes(T,S)`: when `heading_count=1`, the bytes from the heading line start to the end of the last nonblank line before the next unfenced `## ` line (or end of text), without its terminator; otherwise none.
- `section_has_body(T,S)`: when `heading_count=1`, whether `section_bytes(T,S)` holds, after its heading line, a fenced line or a nonblank line that is not an ATX heading; otherwise none.
- `split_reference(r)`: `(file, Some(section))` at the first `#`, else `(r, None)`.
- `anchor(name)`: trimmed, lowercased; spaces become `-`; only alphanumerics, `-` and `_` stay.
- `heading_level(line)`: ATX level 1–6 after 0–3 spaces, followed by end, space or tab. `min_heading_level(T)` over unfenced lines.
- `demote(T,k)`: adds k `#` after the indent of every unfenced ATX heading line; other bytes unchanged. Returns the count of headings pushed past level 6.

## Invariant

Only unfenced exact `## S` lines select sections; a missing or duplicate heading never yields bytes. `section_bytes` is a substring of T that starts with `## S`. `demote` changes only unfenced heading lines and only by inserting `#` characters, so removing the same k characters from those lines restores T. Fence state after a closed fence equals the initial state.

## Cost

O(B) time over the B bytes of T for each call, O(L) line records for L lines; `section_bytes` scans twice (count, then selection). No allocation beyond the line vector, the heading string and the demoted output.

## Composition

`lib.rs` uses `heading_count`, `split_reference` and `section_bytes` (through `reference_bytes`) for component sections and hash-bound section references, and `section_has_body` for the empty-selection gate. `consolidate.rs` uses all of the module: verbatim/wrapped decisions, collision anchors, demotion, unclosed-fence refusal and section rebinding. `mova-correspondence` uses `split_reference` and `section_bytes` for catalogue model bindings. The child `section-fences.groupant` refines the fence automaton range.

## Tests

`sections::tests::sections_follow_the_plain_h2_grammar` (fenced heading ignored, duplicates rejected, unclosed fence detected, split and anchor) , `sections::tests::a_body_is_more_than_headings_and_blank_lines` (a heading-only section and one holding only a sub-heading have no body, a text or fenced line is a body, a missing section is none) and `sections::tests::demotion_is_reversible_and_skips_fences` (fenced lines, non-headings and overflow). Integration: `consolidate::tests` and the groupant validation controls in `lib.rs`.

## behavior_complement

The module never selects a section for zero or several matching headings, never treats a fenced, indented (≥4 spaces), padded or `##`-without-space line as a heading, never rewrites non-heading lines in `demote`, and never resolves paths, reads files or interprets inline Markdown, HTML or setext headings.

## reverse_reconstruction

Given `section_bytes(T,S)=X`, every T whose unique `## S` section has exactly those bytes is compatible; the other sections, the preamble and trailing blank lines are unconstrained. Given `demote(T,k)=D` with known k, T is unique (remove k `#` from each unfenced heading line of D); with unknown k, every k with all heading levels of D above k is compatible. Given `anchor(n)=a`, all names differing only in case, removed punctuation or surrounding whitespace are compatible.

## History

### 2026-11-04 · Descriptant

Epoch 2026-11-04 (source revision in the manifest): created when each groupant became one `model.md` with blocks. The fence automaton moved here from `lib.rs`; its fragment is `section-fences.groupant`.
