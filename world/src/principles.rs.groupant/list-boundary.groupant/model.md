# world-principles-list-boundary

## forward

For requested n and candidate count L, list_step=max(1,floor(L/n)) when n>0 and 1 when n=0. CLI list takes n items from indices 0,stride,2stride,... below L. Therefore n=0 emits none; positive limits preserve the previous spacing. Other CLI branches perform round or status I/O.

## invariant

Stride is nonzero for all usize inputs. The selected iterator yields at most min(L,n) rows; output does not imply all pending facts were listed. A zero request still loads pending knowledge before selection; only the division panic is repaired.

## cost

Constant step calculation; O(min(L,n)) iterator observations after pending construction. No reduction of knowledge-loading cost is claimed.

## composition

Source callers: main dispatches main_args; family invokes round once per research round and reports returned counts/errors; round consumes pending and statement then external teacher/cache/world operations. Native public join: caller-supplied know::Fact -> statement. Selection tests call the exact private pure helper used by pending with supplied keys, avoiding filesystem state.

## epoch-memory

Initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.

## checks

cargo test -p world --lib principles::wave12_models -- --test-threads=1. File ../model.md#evidence binds repeated native profiles and compiled production controls. Partial syntax leaves macros opaque; source-bound syntax is not semantic AST verification.

## behavior-complement

Domain and observation scope: Pure list_step(L,n) and step_by/take index observations for usize L,n; CLI loading excluded.

In U={(L,n,s,J)}, exclude s=0, s differing from max(1,floor(L/n)) for n>0, s differing from 1 for n=0, or J differing from the first n multiples of s below L. Any nonempty J for n=0 or L=0 is absent. 400 finite cases check this scalar/index relation; CLI load failures and load avoidance are not covered.

## reverse-reconstruction

For (s,J), retain every (L,n) satisfying the stride equation and exact bounded-multiple prefix; J alone existentially retains compatible s. Empty J admits n=0 and/or L=0. Nonempty J's last index is below L, while truncation permits multiple larger L,n pairs. Fact values are outside the index observation.

## coupling

400 finite L,n pairs compare actual step_by/take against independently constructed arithmetic indices, including twenty zero limits. Old arithmetic panics at L=n=0; checked_div repair preserves all positive n. CLI I/O remains unexecuted.

Typed entries bind the manifest source revision, this section's domain/operator/observation, and the named finite checks. Status is partial source-derived model. No complete reconstruction search, universal proof or factual-truth guarantee is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30: initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.
