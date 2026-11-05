# world-lang-values

## descriptant

# world-lang-values

## Forward

Closed tags and Act use exact catalog membership; word grammar and field domains come from types. Text has at most 25 whitespace-separated words. Tag sets reject duplicate tags then normalize with V::tags. num accepts zero or one-to-six ASCII digits with no leading zero, or NUM_WORD tags, so 999999 is accepted; the narrower comment is not an enforced bound. plus_ids splits on '+', parses each nonempty component, rejects a repeated Id during scanning, then checks length <=6. Empty components and snapshot forms fail parsing before the final size check. feel_tokens validates the tag after its first Unicode character, checks duplicates across add/del, then validates + or -. value dispatches by Dom to Text, Words, Char, Place, Tags, TagSet or Num. Words splits comma and trims items but does not deduplicate. ABOUT and UNSET_KEYS are fixed admission sets. State-level key_ok compatibility is outside this parser file.

Scope: Source bytes [16968,20624): values; the operation-specific contracts below define observation projections.

## Invariant

The parser is pure and never executes a command. IDs are syntax-checked and registry-filtered; their existence is not checked. Cmd projection may lose spelling, while Line retains trimmed raw and supplied no; FormatErr retains original raw. At most one success or error is produced per nonempty input line. Rejected lines do not suppress later parse_all lines.

## Composition

The root composes interfaces, tokenization/Args, scalar/collection validators, v1/v2 dispatch, narrative dispatch and line admission. Output of each stage is the exact next-stage input: quotes and consumed-key state cannot be independently reconstructed without matching the same original line. parse_line orders stages as trim, tokens, reverse-tail anchors, misplaced-anchor check, Args, command, unused arguments, fallback anchor. parse_all preserves input positions while partitioning successes/errors and skipping blank lines. Six scoped fragments bind concrete source ranges; the remaining parse_all and test regions are modeled at root. This hierarchy does not include executor or query semantics.

## Cost

For total input bytes N and line count L, a conservative whole-file time bound is O(N^2+L) under fixed closed vocabularies: duplicate-key/list scans and vector removals can be quadratic. Tokenization itself is O(N); accumulated output and peak retained parse storage are O(N+L) excluding caller-owned input and allocator overhead. No runtime speedup or finite error budget is claimed.

## Limits

Partial syn-2 projection binds the whole file; macros remain opaque and no name resolution, type inference, semantic AST or formal proof is supplied. types::* supplies Id parsing, Closed catalogs, field domains, value normalization and text limits; prag::schema::Act supplies speech acts. No state existence, reference resolution, world mutation, temporal consistency, truth of claims or query behavior is certified. No live commands, annotator, environment mutations, filesystem or external services are used in native tests. No useful approximate semantics or faster Rust surrogate is declared for these categorical gates.

## Epoch memory

## catalog

# world-lang-values

Domain: Finite UTF-8 change-language lines with fixed types::* closed sets and prag::schema::Act; finite line numbers. Observation is Option<Line> or FormatErr. Inverse syntax fibers project to Cmd plus anchors and intentionally omit raw/no; full Line observations retain trimmed spelling and supplied line number. No world state or I/O is an input.

Scope: Source bytes [16968,20624): values; the operation-specific contracts below define observation projections.

Let U be admitted inputs paired with typed outputs for the named operation, R its fixed-source relation. Behavior complements below are candidate inner bounds of U minus R; source inspection and bounded witnesses do not prove global completeness. Reverse models retain all compatible antecedents in their declared domain, not one preferred spelling. No inverse recovers discarded whitespace unless raw text is observed.

## nonbehavior

plus_ids rejects repeated parsed identities and lists longer than six; IDs fail when outside allowed registries. text excludes empty trimmed content and more than 25 words. Tag sets reject duplicate values.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## antecedents

A trimmed Text value has all whitespace-padded originals with identical internal text as preimages. Sorted V::Tags can arise from any permutation of distinct admitted tags. IDs on this canonical syntax domain recover their string exactly, but allowed-registry context is an additional input.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## closed_values

Closed tags and Act use exact catalog membership; word grammar and field domains come from types. Text has at most 25 whitespace-separated words. Tag sets reject duplicate tags then normalize with V::tags. num accepts zero or one-to-six ASCII digits with no leading zero, or NUM_WORD tags, so 999999 is accepted; the narrower comment is not an enforced bound.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## joint

plus_ids splits on '+', parses each nonempty component, rejects a repeated Id during scanning, then checks length <=6. Empty components and snapshot forms fail parsing before the final size check. feel_tokens validates the tag after its first Unicode character, checks duplicates across add/del, then validates + or -.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## attribute_domain

value dispatches by Dom to Text, Words, Char, Place, Tags, TagSet or Num. Words splits comma and trims items but does not deduplicate. ABOUT and UNSET_KEYS are fixed admission sets. State-level key_ok compatibility is outside this parser file.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## tests

### Native bounded evidence

Six inline lang::bounded_models tests call pure parser/helpers only. Command inventory checks one fixture for all 25 variants and the six v3 names. Complete relation fibers exhaust 14 endpoint strings squared, two field orders and two separators: 784 strings, 16 admitted into four Cmd/anchor fibers and 768 rejected. The expected inverse sets are constructed from independent table indices, not parser outputs. The role matrix checks 5 roles against 9 registry prefixes; further fixtures check joint bounds, snapshots, malformed quotes, exact duplicate keys, first-error ordering, exclusive placement, cross-field gates, text limits, values and parse_all partition.

Run cargo test -p world --lib lang::bounded_models -- --test-threads=1, twice in debug and twice with --release. Normalize only these six named outcomes and compare. A compiled production mutant admits objects as the a endpoint of rel; the complete fiber assertion must fail, then restored source must pass. Generate mova-modelcheck syntax twice and compare. Reports and source/log hashes are in file-root evidence.json; raw logs live outside the repository under the named run directory. Existing tests elsewhere are not included. Finite evidence is not a full Rust proof or live executor test.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective model: relation endpoints must be characters, snapshot-looking C1@1 must fail ID parsing, repeated keys fail before command dispatch, quoted text groups whitespace, and explicit placement is exclusive. Complete bounded relation fibers use 784 strings and independent character-only expected antecedents. Current source admits ^s0 and duplicate anchors, allows distinct alias keys at/to to map to repeated Field::At values, and accepts numeric quantities through six digits (999999), despite num's 0-100000 comment. These are recorded behavior/policy gaps, not silently changed. An initial fixture used unsupported act=inform; corrected to the actual Act catalog's assert before final evidence. No production behavior was changed. Native measurements and the compiled endpoint mutant are recorded in evidence.json.
