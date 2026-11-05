# world-md

## model

# world-md

Human Markdown projections of immutable world snapshots and narration metadata.

## forward

For fixed Run and FormatErr inputs, world_md appends text, cursor, location/object/character/relation registries, optional intention/belief registries, events, working memory and gate diagnostics in source order. story_md falls back exactly to world_md without Story; otherwise it prepends narration views and appends the complete root report. cell maps pipe to backslash-pipe and LF to space. snap_str omits Type/Class and renders unknown Gender/Age for characters.

## behavior-complement

Universe U consists of input/output pairs for the scoped deterministic observations with fixed dependency state and successful allocation. N={(x,y) in U | y != F(x)} is the exact complement of the stated model. Correspondence with Rust is a source-derived hypothesis, supported only on named finite tests; unknown IO/dependency outcomes are not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining all compatible state/input antecedents. x belongs to B(F(x)); unique recovery requires injectivity, which the lossy projections lack. Bounded fibers are complete only within their enumerated input domain.

## cell-fiber

Type: reverse_reconstruction. For strings a\na and a a, cell yields the same a a. General preimage chooses LF or space for each output space, respecting pipe escaping and original backslashes. 259 strings over six scalars through length three supply bounded correspondence. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## escape-boundary

Type: encoding_relation. cell replaces every pipe and LF, retaining all other Unicode scalars. This is a table-cell convention, not a CommonMark encoder. Raw labels/time can bypass it. Independent scalar oracle and non-idempotence witness cover the helper only. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## snapshot-loss

Type: information_loss. snap_str hides Type/Class and resolves IDs through World labels/place chains; report equality cannot recover IDs, histories or hidden fields uniquely. Unknown base fields are displayed only for characters. This is source-derived, not universally tested. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## fallback-law

Type: constraint_relation. Without Story, story_md(r,e,n)=world_md(r,e,n) byte for byte. Native empty-world fixture checks this equality and report diagnostics. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## story-root-coupling

Type: composition_relation. With Story, the result ends with world_md on the same Run/errors/line count. Frame counts and qa3 responses use that same world, not independent worlds. Empty v3 fixture checks suffix and eight section positions. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## diagnostic-projection

Type: encoding_relation. Format errors contribute their count plus raw/no/msg; they are displayed without executing commands. No causal claim that lines_total equals accepted plus rejected is enforced here; supplied counters are printed. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## invariant

Rendering borrows Run without mutation. Section order and fallback are deterministic conditional on ordered dependency observations. cell output has no LF; no universal Markdown sanitization or roundtrip invariant holds.

## composition

Fragments share exact source state; helper output feeds rendering/features/rules in source order. Compose relations by existential intermediate state, preserving shared identities and caller options. Do not take independent products of projected fibers or add overlapping costs. Inline tests are additional root scope.

## cost

Let B be emitted bytes, S sentences, C cursor entries, E entities, H states/history entries. Base output allocation O(B); cursor lookup O(S*C); history/snapshot/context and qa3 costs are external terms, not assumed constant. Story scans event metadata per frame O(F*V), then calls qa3 and root renderer. Memory includes intermediate strings and output; no IO occurs in this file.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1. File-specific filters md::wave8_models and tabular::wave8_models. Tests use native pure functions and synthetic World/Text records, never an annotator or teacher. Evidence and negative controls are in root model.md#evidence.

## epoch-memory

First epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.

## limitations

Labels, title, time and diagnostic code spans are not uniformly escaped. cell leaves CR/backticks/backslashes intact and is not idempotent. Sentence count casts to u16; large documents are outside tested domain. Dependency semantics and nonempty complex story graphs remain untested.

## evidence

### md correspondence evidence

#### Finite domains

Markdown: 259 unique strings over six scalars through length three, independent
scalar expansion oracle, exact two-element newline/space fiber, and a
non-idempotence witness. Empty v2/no-Story and v3 reports check fallback,
diagnostics and eight ordered sections/root suffix. These do not exercise
nonempty registry/state/history rendering or arbitrary qa3 behavior.

CSV: 343 quoted three-field records over seven LF-free fields roundtrip through
an independently defined encoder. Four encodings/unbalanced quote forms share
one parsed result; multiline helper acceptance is explicitly distinct from
physical-line file reading. Feature checks cover 12/13/20 distinct finite
numeric values, categorical unknown loss, raw age override, a three-person
family graph and nine ternary known-outcome assignments. A twenty-row decision
list witness checks tie priority, one-rule cap, default and first-match behavior.
These tests do not evaluate corpus quality, IO, full CV or external models.

#### Source and structural checks

#### Epoch correction and limits

The first debug run passed eight tests and failed the fallback fixture:
World::new initializes Story even in v2. The fixture now explicitly removes
Story; production behavior was correct and unchanged. The model conditions
fallback on Story absence, not v2 mode. Preserve that distinction in parent wiring.

Labels, title, time and diagnostic code spans are not uniformly escaped. cell leaves CR/backticks/backslashes intact and is not idempotent. Sentence count casts to u16; large documents are outside tested domain. Dependency semantics and nonempty complex story graphs remain untested.
No live teacher, parser loading, GPU, training data or network operation was used.
Mathematical views are source-derived hypotheses supported by these finite
observations; no executable percentage-error surrogate is justified.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.

### 2026-10-30 · evidence

2026-10-30, first epoch. Production behavior is unchanged; only native tests were appended.
Source binding and partial syntax are in manifest.json and syntax.json. Fragment
ranges bind named function envelopes; gaps and cross-fragment calls are described
in their models. This is not semantic AST analysis or general verification.

#### Repeated profiles

Command: `cargo test -p world --lib wave8_models -- --test-threads=1`,
and the same command with `--release`. Two successful debug runs and two
release runs each passed nine tests (four md and five tabular). Normalizing
to ordered test identities and results yielded identical bytes in all four:
SHA-256 `7c21f5bccd10b254d87e87ac68a2a048ccb9841f27afb70034f84fd6539bf404`.
Durations and compiler warnings were excluded; no speed claim follows.

Relative external run names: `octopus-wave8-formats-2026-10-30/debug1-restored.log`,
`debug2.log`, `release1.log`, `release2.log` and corresponding
`.normalized` files. Raw logs are outside the repository.

#### Compiled controls

One compiled test binary contained two production mutations: cell LF replacement
space to underscore, and quartile comparison greater-than to greater-or-equal.
Named assertions `md::wave8_models::cell_bounded_oracle_and_loss_fibers` and
`tabular::wave8_models::numeric_quartiles_and_category_threshold` failed;
the remaining seven tests passed. Both sources were restored before any result
assertion; a subsequent native run passed all nine.
Raw `mutants.log` SHA-256:
`7f4658d1349eba594b5d73f11d047f592ef8941cec1f52143e75cd83284b71e3`.

Two independently generated syntax projections per file matched byte-for-byte.
This file's syntax SHA-256:
`d4088d3c894f971b4335258cac57a0ec8d2645d0f033bff750a408c5aad18174`.
An external copy of the complete registry with these two entries found no
diagnostics in either file tree. Its only three diagnostics were the expected
root child-list mismatch and two missing transitive root source bindings.
Ancestor/registry integration belongs to the parent task; global validity is
not claimed here. A minimal registry was unsuitable because validation scans
other existing sidecars; that failed setup was replaced with the full copy.
