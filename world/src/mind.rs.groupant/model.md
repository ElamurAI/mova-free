# world-mind

## model

# world-mind

## forward

Define P(text) by trimming lines, dropping empty lines and lines beginning '#', removing repeated '- ' prefixes, then concatenating babi::split_sentences outputs in line order. For tell let P=[s1,...,sk], S0=S and D contain actual returned read(Si-1,si)=(Si,ri). Fold Ok(None) into a success count; append (si,reason) for Ok(Some(reason)) or Err(error), with errors converted by Display. T(S,text,D)=(Sk,ok,gaps); no rollback is added.

For reply let q=trim(line). If q ends in '?', ask(S,q): Ok((a,w)) with a!='?' renders `a. Because: w`; sentinel a='?' renders `I do not know: w`; error renders `I do not know: e`. Otherwise T supplies the outcome: no gaps renders `Understood (k sentence[s]).`, singular exactly when k=1; gaps render `I do not understand: ` followed by ordered `«sentence» — reason` entries joined with `; `. Tags and D are model witnesses, not runtime String fields.

Nonempty Story paths are conditional source inspection. Native checks execute P and empty-projection tell/reply with seeded identity and state-lines observations.

## invariant

P retains segment order. Returned tell has ok+|gaps|=|P| and exact sentence/reason pairings. P=[] invokes no read, so T=(S,0,[]) and a nonquestion reply is exactly `Understood (0 sentences).`. Nonempty final state follows dependency transitions, including any effects on error/gap outcomes. Text output establishes neither truth nor durable state.

## cost

For B input bytes and k projected sentences, O(B+output bytes) projection/formatting plus k opaque read costs or one ask cost. Projection/gap/output storage is O(B+output bytes), excluding dependency state. Parser cost and I/O waiting are unmeasured.

## composition

Source callers: main dispatches probe/serve/say; family preserves each Member Story, sets identity before tell, broadcasts statements and ignores tell gaps; worldbook::perceive uses ok+gaps.len as projected sentence count then separately classifies gaps as knowledge. Native public join: set_me -> tell/reply on zero projected sentences -> unchanged Story::state_lines and preserved identity (same name accepted, different name rejected). Nonempty Story read/ask remains opaque; no parser has been invoked for these checks.

## epoch-memory

Follow-up modeling epoch: source-specific tagged transition exclusions and ambiguous preimages replace generic root statements. Source bytes and native evidence are unchanged.

## checks

cargo test -p world --lib mind::wave12_models -- --test-threads=1. File model.md#evidence binds repeated native profiles and compiled production controls. Partial syntax leaves macros opaque; source-bound syntax is not semantic AST verification.

## behavior-complement

Domain and observation scope: Tagged project(text), tell(S,text,D), reply(S,line,D), with finite text and actual returned Story dependency trace D at a fixed revision. Socket/probe effects lie outside this relational slice.

U consists of the tagged scoped inputs, actual dependency witnesses and arbitrary candidate outputs/states. R is given by the forward equations; U minus R is the scoped complement conditional on D matching the real dependency. Relative to the full module this is an inner absent-behavior slice, with socket/probe and unreturned calls excluded.

Excluded candidates include contributions from filtered blank/heading lines, reordered retained segments, a tell count/gap pairing inconsistent with the returned read trace, state changes or nonzero counts/gaps for P=[], an ask branch on a nonquestion or tell branch on a trailing '?', and reply counts/singular suffix/gap order differing from the formatting equations. Read errors are gaps, not counted successes. This imposes no additional state semantics on opaque nonempty Story calls.

49 projection pairs, heading-order fixtures and three seeded empty-projection cases exercise selected exclusions. Nonempty read/ask/error/question behavior remains source-inspected.

## reverse-reconstruction

For tagged y retain every (operation,input,S,D) with the same tag and R(input,S,D,y). A projected vector admits all texts with exactly that P, including arbitrary discarded headings/blank lines and compatible whitespace/bullet variants. A tell result (S',ok,gaps) retains all segment vectors and compatible read traces ending at S', containing exactly ok successful outcomes and exactly the observed ordered sentence/reason pairs. Successful sentences and intermediate states remain ambiguous; P=[] requires S'=S and (0,[]).

Reply fibers preserve the input's exact trailing-question predicate and corresponding ask-result or tell-fold equation, including sentinel handling and formatting. A runtime String lacks the model tag: reconstructing from only text takes the union over compatible branches/results, never infers a unique branch merely from English phrasing. Identity/state-lines are partial state observations. No complete solver or recovery of hidden Story facts is claimed.

## coupling

Source callers: main dispatches probe/serve/say; family preserves each Member Story, sets identity before tell, broadcasts statements and ignores tell gaps; worldbook::perceive uses ok+gaps.len as projected sentence count then separately classifies gaps as knowledge. Native public join: set_me -> tell/reply on zero projected sentences -> unchanged Story::state_lines and preserved identity (same name accepted, different name rejected). Nonempty Story read/ask remains opaque; no parser has been invoked for these checks.

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

Final source SHA-256: `d83c8c3b03e9b7c29f29db9c67dcd15b7c1c5d2a413ef6bb93958cbaf4b78936`.

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
