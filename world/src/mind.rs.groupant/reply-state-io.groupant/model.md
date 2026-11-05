# world-mind-reply-state-io

## forward

tell sequentially calls Story::read for P(text), counts Ok(None), and records Ok(Some(reason)) or error as gaps. reply trims once and branches on trailing '?': Story::ask answer '?' or error becomes unknown; other answers include explanation. Nonquestions call tell, reporting all gaps if any; partial successful reads are not rolled back. probe/serve create a Story then sequentially load texts; say exchanges one socket line.

## invariant

Empty P gives (0,[]) with no read calls. For returned calls, ok+gaps.len equals P length; error classification does not certify lack of Story effects. No durable state write is performed by this module's tell/reply, but in-memory Story can change. serve removes the socket path before bind and ignores response-write failures; this is not a transaction guarantee.

## cost

O(input bytes + total opaque read/ask costs + reply output); socket/filesystem waiting is unbounded. External Story parser and knowledge costs are not estimated.

## composition

Source callers: main dispatches probe/serve/say; family preserves each Member Story, sets identity before tell, broadcasts statements and ignores tell gaps; worldbook::perceive uses ok+gaps.len as projected sentence count then separately classifies gaps as knowledge. Native public join: set_me -> tell/reply on zero projected sentences -> unchanged Story::state_lines and preserved identity (same name accepted, different name rejected). Nonempty Story read/ask remains opaque; no parser has been invoked for these checks.

## epoch-memory

Initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.

## checks

cargo test -p world --lib mind::wave12_models -- --test-threads=1. File ../model.md#evidence binds repeated native profiles and compiled production controls. Partial syntax leaves macros opaque; source-bound syntax is not semantic AST verification.

## behavior-complement

Domain and observation scope: Returned tell/reply operations with finite text, Story state and actual compatible read/ask dependency trace; socket/probe effects inspected separately.

Exclude a tell result not equal to the ordered fold of supplied read results: Ok(None) increments success only, other returned cases append their sentence/reason, and final state is the dependency successor. P=[] excludes any state change, nonzero count or nonempty gaps. Exclude reply branch tags inconsistent with trim(line).ends_with('?'), ask output inconsistent with sentinel/success/error formatting, or tell wording inconsistent with exact counts, ordered gaps and singular iff count=1. These are conditional source-derived exclusions, not added restrictions on opaque Story transitions. No socket/persistence exclusion is claimed. Only seeded empty-projection identity/state cases execute natively.

## reverse-reconstruction

Retain every tagged input with the exact question predicate and compatible ask result/read fold producing the observed wording, counts, gaps and final state. Error/unknown strings can coincide; successful omitted sentences and intermediate states remain ambiguous. Runtime String observations without tags use unions of compatible fibers. Empty projected tell requires prior S=final S, but state_lines alone does not reconstruct S. No socket/filesystem inverse is implemented.

## coupling

Executed public tell/reply join uses only empty/heading-only input and Story::state_lines plus set_me identity preservation; it supports zero-read no-op behavior for those cases. Nonempty read/ask, probe/serve/say, parser, sockets and persistence are source inspected only. A response is system output, not truth evidence.

Typed entries bind the manifest source revision, this section's domain/operator/observation, and the named finite checks. Status is partial source-derived model. No complete reconstruction search, universal proof or factual-truth guarantee is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30: initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.
