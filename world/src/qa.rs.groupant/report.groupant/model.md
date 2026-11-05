# world-qa-report

## forward

answers_md evaluates rows in supplied order. CONTROL rows add ctrl and ctrl_ok from abstention only, bypassing verdict and ordinary totals. Other rows increment exactly one of hit/miss/none. TSV document IDs are retained but answers_md relies on caller filtering.

## invariant

hit+miss+none equals noncontrol input count; ctrl equals control input count; ctrl_ok <= ctrl. Ordinary expected text never changes world state. Supplied-world reporting is assessment, not learning or retention.

## cost

Q rows: sum of answer and verdict costs plus O(rendered bytes). Markdown table retains row order; text replacement allocates escaped copies.

## composition

Public joins: World::apply on constructed Cmd/Line values -> answer -> verdict; read_questions -> answers_md on supplied frozen expectations. No neural parser, teacher, corpus, CLI persistence or courses executed.

## behavior-complement

Domain/universe: terminating calls on consistent supplied worlds and well-formed observations in this scope. Inner absent-behavior relation excludes transitions violating the invariant above. This is a source-derived partial exclusion; unknown paths and malformed world states remain outside it. It is not a complement of merely sampled tests.

## reverse-reconstruction

A score tuple admits every row sequence with the same outcome multiset; permutations, IDs and question meanings cannot be recovered. Markdown is not a reversible TSV encoding.

## projection-boundary

Native read_questions -> answers_md join yields (1,1,1,1,1) over four frozen rows. main caller filters q.doc == doc before use; this is source-inspected, not CLI execution. IDs/kinds/newlines are not all escaped and cross-document misuse remains a caller precondition.

All three typed entries bind the manifest revision and scope. Operators are invariant exclusion, output preimage and observation constraint respectively; status is partial source modeling with finite native evidence. Reverse relations preserve all described ambiguity; no complete solver or universal inverse is provided.

## checks

cargo test -p world --lib qa::wave12_models -- --test-threads=1. See the file ../model.md#evidence for actual repeated profiles and compiled controls. Partial syntax projection leaves macro bodies opaque; no semantic AST proof.

## epoch-memory

Prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.
