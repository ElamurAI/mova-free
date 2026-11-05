# world-lang

## descriptant

# world-lang

## Forward

Interfaces define Role/Speaker/Obs/TRel/Ev2/Cmd, name projections and Line/FormatErr. tokens and Args produce checked positional/key-value states; scalar/list/value helpers enforce syntactic domains. parse_line dispatches legacy and narrative commands, rejects unused args, binds anchors and preserves provenance. parse_all partitions lines without execution. Every logical production region and the appended native-test region is covered at root; dependency validation remains explicitly conditional.

Scope: Whole source, including declarations, all helpers/dispatch, parse_all and native tests

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

# world-lang

Domain: Finite UTF-8 change-language lines with fixed types::* closed sets and prag::schema::Act; finite line numbers. Observation is Option<Line> or FormatErr. Inverse syntax fibers project to Cmd plus anchors and intentionally omit raw/no; full Line observations retain trimmed spelling and supplied line number. No world state or I/O is an input.

Scope: Whole source, including declarations, all helpers/dispatch, parse_all and native tests

Let U be admitted inputs paired with typed outputs for the named operation, R its fixed-source relation. Behavior complements below are candidate inner bounds of U minus R; source inspection and bounded witnesses do not prove global completeness. Reverse models retain all compatible antecedents in their declared domain, not one preferred spelling. No inverse recovers discarded whitespace unless raw text is observed.

## nonbehavior

Exclude rel endpoints in O/L/R/E/P/B/F/K, snapshot-form IDs, exact duplicate key spellings, unmatched quotes, misplaced anchors and multiple event placements. These exclusions are conditional on this grammar and fixed dependency catalogs; no exclusion of all semantically invalid worlds is claimed.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## antecedents

For observation (Cmd,anchors), reconstruct every line whose trim/token/Args/dispatch relation yields that pair. Whitespace between tokens, allowed key-order permutations and accepted quote variants form nonunique fibers; Args attrs preserve their source order, so not all key permutations are equivalent. The 784-string relation domain is completely enumerated with independent expected fibers; general UTF-8 reconstruction remains relational.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## registry_roles

Role Agent permits C; Patient C/O/L; To C/L; At L; Instr O. Relation a and b are C-only; event/causal/time IDs are E where requested. Identity parsing accepts uppercase registry prefix and one to six decimal digits without leading zero; snapshot @ forms are excluded by types::Id. Native 45-cell role matrix and complete endpoint fibers check correspondence.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## quotes_keys

tokens groups characters between literal double quotes and splits Unicode whitespace outside; there is no escape syntax. Args requires lowercase ASCII key names, splits the first '=', rejects repeated spellings and positions after any key, and validates complete outer quote pairs. text trims value edges; quote flags are enforced by ID/word consumers but intentionally ignored by some numeric/tag/text consumers.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## line_admission

parse_line returns None only on trimmed empty input. It scans trailing ^s tokens right-to-left before validating remaining anchors, Args, command, unused arguments and finally required anchor. Cursor sent supplies a default only without explicit anchors. Error stores untrimmed raw and supplied number, success stores trimmed raw.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## closed_values

Closed tags and Act use exact catalog membership; word grammar and field domains come from types. Text has at most 25 whitespace-separated words. Tag sets reject duplicate tags then normalize with V::tags. num accepts zero or one-to-six ASCII digits with no leading zero, or NUM_WORD tags, so 999999 is accepted; the narrower comment is not an enforced bound.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## command_modes

Twenty-five Cmd variants dispatch by exact lowercase command name; frame/claim/time/branch/know/reveal are the six v3 variants. Plan creation versus closure and belief creation versus modification are separate field-presence branches; parse acceptance does not guarantee referenced entities exist or mutations are allowed.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## placement

ev2 consumes before/after/during/same and rejects more than one. Ev2::placement on manually constructed state prioritizes before, after, during, same; parser-created Ev2 never needs that conflict resolution. reveal has its own placement parser; time requires exactly one relation or absolute at text.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## field_aliases

Args rejects duplicate textual keys, but attrs maps aliases at/to/in to the same Field::At without enforcing semantic uniqueness. Thus set C1 at=L1 to=L2 is admitted with two ordered Field::At entries; downstream policy is not modeled. This is not advertised as duplicate-field rejection.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## batch

parse_all enumerates physical text.lines positions from one, independently parses every line and appends success/error to separate order-preserving vectors. Blank lines disappear but numbering remains physical; no rollback, execution or stop-on-first-error behavior is introduced.

Domain and scope are fixed above. Evidence: model.md#tests names the independent finite native oracle; untested cases remain conditional source models. No semantic proof follows from a manifest.

## tests

### Native bounded evidence

Six inline lang::bounded_models tests call pure parser/helpers only. Command inventory checks one fixture for all 25 variants and the six v3 names. Complete relation fibers exhaust 14 endpoint strings squared, two field orders and two separators: 784 strings, 16 admitted into four Cmd/anchor fibers and 768 rejected. The expected inverse sets are constructed from independent table indices, not parser outputs. The role matrix checks 5 roles against 9 registry prefixes; further fixtures check joint bounds, snapshots, malformed quotes, exact duplicate keys, first-error ordering, exclusive placement, cross-field gates, text limits, values and parse_all partition.

Run cargo test -p world --lib lang::bounded_models -- --test-threads=1, twice in debug and twice with --release. Normalize only these six named outcomes and compare. A compiled production mutant admits objects as the a endpoint of rel; the complete fiber assertion must fail, then restored source must pass. Generate mova-modelcheck syntax twice and compare. Reports and source/log hashes are in file-root evidence.json; raw logs live outside the repository under the named run directory. Existing tests elsewhere are not included. Finite evidence is not a full Rust proof or live executor test.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective model: relation endpoints must be characters, snapshot-looking C1@1 must fail ID parsing, repeated keys fail before command dispatch, quoted text groups whitespace, and explicit placement is exclusive. Complete bounded relation fibers use 784 strings and independent character-only expected antecedents. Current source admits ^s0 and duplicate anchors, allows distinct alias keys at/to to map to repeated Field::At values, and accepts numeric quantities through six digits (999999), despite num's 0-100000 comment. These are recorded behavior/policy gaps, not silently changed. An initial fixture used unsupported act=inform; corrected to the actual Act catalog's assert before final evidence. No production behavior was changed. Native measurements and the compiled endpoint mutant are recorded in evidence.json.
