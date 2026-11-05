# world-lang-validation

## descriptant

# world-lang-validation

## Forward

parse_line returns None only on trimmed empty input. It scans trailing ^s tokens right-to-left before validating remaining anchors, Args, command, unused arguments and finally required anchor. Cursor sent supplies a default only without explicit anchors. Error stores untrimmed raw and supplied number, success stores trimmed raw. Tail tokens starting ^s parse as u16, so zero and repeated values are admitted, and integer parsing also permits accepted standard u16 spellings. Explicit anchors override cursor sent without equality checking. know's from is a separate stricter positive check. Failure order is token quotes, tail-anchor parse from rightmost, misplaced anchor, no command, Args scanning, command-specific field checks, leftover arguments, missing-anchor. Oracle uses lines with multiple defects to distinguish these phases; no generic lexical-versus-semantic priority is assumed.

Scope: Source bytes [20624,21873): validation; the operation-specific contracts below define observation projections.

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

# world-lang-validation

Domain: Finite UTF-8 change-language lines with fixed types::* closed sets and prag::schema::Act; finite line numbers. Observation is Option<Line> or FormatErr. Inverse syntax fibers project to Cmd plus anchors and intentionally omit raw/no; full Line observations retain trimmed spelling and supplied line number. No world state or I/O is an input.

Scope: Source bytes [20624,21873): validation; the operation-specific contracts below define observation projections.

Let U be admitted inputs paired with typed outputs for the named operation, R its fixed-source relation. Behavior complements below are candidate inner bounds of U minus R; source inspection and bounded witnesses do not prove global completeness. Reverse models retain all compatible antecedents in their declared domain, not one preferred spelling. No inverse recovers discarded whitespace unless raw text is observed.

## nonbehavior

Nonempty successfully tokenized input with only anchors cannot yield a command. A valid non-cursor command without anchors is rejected only after argument/command checks. Any remaining token starting '^' is a misplaced-anchor error.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## antecedents

For successful (Cmd,anchors), retain all trimmed command strings satisfying prior stages and either exact explicit anchor tail or cursor-sent fallback. Repeated/zero anchors are retained and do not collapse. For FormatErr, raw and no reconstruct supplied inputs, but msg alone has broad error fibers.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## line_admission

parse_line returns None only on trimmed empty input. It scans trailing ^s tokens right-to-left before validating remaining anchors, Args, command, unused arguments and finally required anchor. Cursor sent supplies a default only without explicit anchors. Error stores untrimmed raw and supplied number, success stores trimmed raw.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## anchor_domain

Tail tokens starting ^s parse as u16, so zero and repeated values are admitted, and integer parsing also permits accepted standard u16 spellings. Explicit anchors override cursor sent without equality checking. know's from is a separate stricter positive check.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## error_trace

Failure order is token quotes, tail-anchor parse from rightmost, misplaced anchor, no command, Args scanning, command-specific field checks, leftover arguments, missing-anchor. Oracle uses lines with multiple defects to distinguish these phases; no generic lexical-versus-semantic priority is assumed.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## tests

### Native bounded evidence

Six inline lang::bounded_models tests call pure parser/helpers only. Command inventory checks one fixture for all 25 variants and the six v3 names. Complete relation fibers exhaust 14 endpoint strings squared, two field orders and two separators: 784 strings, 16 admitted into four Cmd/anchor fibers and 768 rejected. The expected inverse sets are constructed from independent table indices, not parser outputs. The role matrix checks 5 roles against 9 registry prefixes; further fixtures check joint bounds, snapshots, malformed quotes, exact duplicate keys, first-error ordering, exclusive placement, cross-field gates, text limits, values and parse_all partition.

Run cargo test -p world --lib lang::bounded_models -- --test-threads=1, twice in debug and twice with --release. Normalize only these six named outcomes and compare. A compiled production mutant admits objects as the a endpoint of rel; the complete fiber assertion must fail, then restored source must pass. Generate mova-modelcheck syntax twice and compare. Reports and source/log hashes are in file-root evidence.json; raw logs live outside the repository under the named run directory. Existing tests elsewhere are not included. Finite evidence is not a full Rust proof or live executor test.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective model: relation endpoints must be characters, snapshot-looking C1@1 must fail ID parsing, repeated keys fail before command dispatch, quoted text groups whitespace, and explicit placement is exclusive. Complete bounded relation fibers use 784 strings and independent character-only expected antecedents. Current source admits ^s0 and duplicate anchors, allows distinct alias keys at/to to map to repeated Field::At values, and accepts numeric quantities through six digits (999999), despite num's 0-100000 comment. These are recorded behavior/policy gaps, not silently changed. An initial fixture used unsupported act=inform; corrected to the actual Act catalog's assert before final evidence. No production behavior was changed. Native measurements and the compiled endpoint mutant are recorded in evidence.json.
