# world-principles

## model

# world-principles

## forward

S(F) prefixes `a {subj} ` and applies link/polarity templates: IsA -> `is a`/`is not a`; Has -> `has`/`does not have`; Can -> `can`/`cannot`; Does -> `does this:`/`does not do this:`. Other links use optional `not ` followed by know::link_name(link). Append space+obj, uppercase the body's first character, and append '.'.

key(F)=subj+'|'+Debug(link)+'|'+obj+'|'+negation. E(K,C) retains encounter-order Facts whose source starts `derived from` and whose key is absent from C. Q(K,C,n)=take_n(first-per-key(stable-sort-by-key(E))). Equal keys retain the first encounter's full Fact. The cache wrapper obtains C from checked(), an unexecuted filesystem boundary.

For list count L and request n, stride=max(1,floor(L/n)) when n>0, otherwise 1. I(L,n) takes the first n multiples of stride below L; I(L,0)=[]. round/write_world consume selected batches and cached verdicts; teacher T is a judgment, not a truth predicate.

## invariant

Q preserves full chosen Facts, excludes checked/nonprefix candidates, orders keys ascending, emits each key at most once and returns at most n. Sorting is not confidence ranking; source prefixes do not enforce lesson/exam separation. statement/key omit frequency/count/provenance, and pipe-key collisions remain possible. Stride is nonzero; zero requests select nothing but still load CLI knowledge.

## cost

F supplied facts and D checked keys require O(F log(D+1)) membership comparisons and O(F log(F+1)) sorting comparisons, including allocated-key/string work. Cloning/storage is O(total candidate bytes). Statement is linear in rendered bytes. Step calculation is constant-time; index observation costs O(min(L,n)). Knowledge/cache/teacher cost remains opaque; no measured speed claim.

## composition

Source callers: main dispatches main_args; family invokes round once per research round and reports returned counts/errors; round consumes pending and statement then external teacher/cache/world operations. Native public join: caller-supplied know::Fact -> statement. Selection tests call the exact private pure helper used by pending with supplied keys, avoiding filesystem state.

## epoch-memory

Follow-up modeling epoch: source-specific tagged transition exclusions and ambiguous preimages replace generic root statements. Source bytes and native evidence are unchanged.

## checks

cargo test -p world --lib principles::wave12_models -- --test-threads=1. File model.md#evidence binds repeated native profiles and compiled production controls. Partial syntax leaves macros opaque; source-bound syntax is not semantic AST verification.

## behavior-complement

Domain and observation scope: Tagged statement(F), select(K,C,n), list(L,n) on finite supplied Facts/Knowledge, checked-key set C and usize limits; fixed link_name dependency. Teacher/cache/publication traces form a separate source-inspected conditional slice.

U is the disjoint union of tagged statement, supplied-selection and index-list input/output candidates; R comprises S,Q,I. U minus R excludes wrong statement template/polarity, subject/object bytes or punctuation; Q outputs containing ineligible/checked Facts, duplicate or descending keys, more than n entries, the wrong sorted truncated prefix or a later duplicate's full Fact instead of the first; and nonempty I for n=0, zero stride or incorrect sampled indices.

Eight statement rows, 40 subset/limit cases and 400 stride cases support selected source-derived exclusions. This relation does not establish truth of derived-prefix Facts or teacher T. Conditional teacher-retention formatting/selection is inspected only; atomic publication, correct external verdicts, concurrency-safe budgets and failure recovery are not asserted.

## reverse-reconstruction

For statement y retain all F with S(F)=y, including arbitrary frequency/count/provenance and compatible subject/object/link template decompositions. For selected full-Fact vector V retain all K,C,n with Q(K,C,n)=V: V's eligible keys are unique/ascending, its full Facts are the first encounters, and additional eligible keys obey the exact truncation boundary. Checked/nonderived records, duplicate alternatives and tail candidates remain ambiguous; neither K,C nor n is uniquely recoverable.

For index J retain all (L,n) with I(L,n)=J; empty J admits n=0 or L=0. Source-inspected retained teacher rows admit all compatible final numbered verdict parses and publication histories; a T label cannot recover true-world facts or complete antecedent knowledge. Tags are model observations; text alone retains the union of compatible tagged fibers.

## coupling

Source callers: main dispatches main_args; family invokes round once per research round and reports returned counts/errors; round consumes pending and statement then external teacher/cache/world operations. Native public join: caller-supplied know::Fact -> statement. Selection tests call the exact private pure helper used by pending with supplied keys, avoiding filesystem state.

Typed entries bind the manifest source revision, this section's domain/operator/observation, and the named finite checks. Status is partial source-derived model. No complete reconstruction search, universal proof or factual-truth guarantee is claimed.

## evidence

### Native evidence

Six selected native tests (three per source) pass twice in debug and twice in release with identical normalized test names/results. Command: `cargo test -p world --lib [--release] wave12_models -- --test-threads=1`. This is not the full world suite. Existing unrelated compiler warnings remain; no whole-world Clippy claim.

Mind checks include 49 pairwise literal segmentation fixtures, heading/bullet order and three zero-projection public tell/reply inputs preserving supplied identity and state-lines observations. Principles checks include eight polarity templates with provenance/frequency/count information-loss probes, 40 subset/limit selections preserving first duplicate source, and 400 list-stride cases including zero limits.

Old-code witness `principles::wave12_models::zero_list_limit_has_no_division` panics on division by zero after extracting the unchanged arithmetic. The repair uses checked division and preserves positive-limit sampling. Zero-limit CLI loading remains opaque; no speedup is inferred.

Compiled production controls: removing mind's heading filter fails `mind::wave12_models::line_projection_finite_oracle`; reversing principles' key comparison fails `principles::wave12_models::pending_finite_subset_order_and_stable_provenance`. Each fails an equality assertion, not compilation; exact source bytes are restored and the named test passes.

Repeated syntax projections and structural reports are byte-identical. External full-base registry scaffold validates both file trees with no diagnostics; its absent scaffold sidecar is intentional. Changed-source registration covers both owned sources. Root owns integrated ancestry. New test modules were rustfmt-formatted independently; broad legacy formatting was avoided; diff whitespace passes.

No parser, teacher, socket, live course or durable-state operation was executed. Public pure joins are supplied Fact to statement, and Story set_me plus empty tell/reply plus state_lines. Private pending_with tests use supplied checked keys; they do not validate checked-file reads. Family/worldbook call sites are inspected only. Teacher T is a judgment rather than an independent truth oracle; key/prose projection loses frequency, count and provenance. Unescaped pipe-key collisions and multi-file partial commits remain explicit limitations.

## History

### 2026-10-30 · epoch-memory

2026-10-30: initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Final source SHA-256: `cdb5ef0db3da9ca5761580326e66abbbfbbb8e69414a57485adf5fc5b1aa8c42`.

Raw logs are retained under `$MOVA_DATA/runs/octopus-wave12-mind-2026-10-30/`. Log hashes:

- `final-debug1.log`: `0cbbbb8401fe96d544a442052b3d0d7a3be5c7d05db8f8ae2604194cadf7d761`

- `final-debug2.log`: `b98e07739321432f58c9695f232147a9b35750ea77689065b2107e529b935274`

- `final-release1.log`: `69449e455ac0ec346457ada540373ec07bb4f1c70955fd83a2c666d5238e0f82`

- `final-release2.log`: `813d1bb163cad9572f2cc16631529e0b5a03c7427b761d6c1c6fdb733c532bec`

- `old-witness.log`: `0318bb3c85d5788d176dd55f2d292d0eb60c331693ed1d6bbc62303643ba3836`

- `mutant-mind.log`: `20c9d2b3f65a6d968a2566e699417c51fbb58c59edee36d4be826a4cebc024a8`

- `mutant-principles.log`: `12b3036aa3279cd09150154c73f091547ee500f29ab59944a75d3b73cd81bf25`

- `restored-mind.log`: `9b6644eda28d38931969b429528e99ed9d08205c86aa37420d09996f549fd9e1`

- `restored-principles.log`: `4fbb1d8076541c63a8909a56c92db63a7b266f479cd9630089f6eb5cf7d27653`

- `final-groupants1.json`: `7217ce6f629c6b4acc6cd89f0452593a8eae084f0274c88720c42d070335b150`

- `changed.json`: `d9456b7ed39861a4ff996507c17d6f43dc38f58387a418a72c61f5b075cdb0ac`
