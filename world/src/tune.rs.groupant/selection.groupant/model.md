# Eligible maximum selection over supplied observations

## forward

Eligibility for (i,gd,gh,p,remembered) is gd>0, gh>=0 and p<0.05. Return eligible tuple with maximal gd then minimal i; None if no eligible row. remembered is carried but not ranked.

## behavior-complement

Universe: unique-index result arrays with proposed None/tuple. Exclude ineligible selection, None despite an eligible row, or a row outranked by greater eligible dev gain or smaller tied index. NaN p is ineligible; negative/infinite-negative p remains allowed by legacy comparison.

## reverse-reconstruction

Chosen row admits arbitrary rejected and lower-ranked extra rows. None admits every array of ineligible tuples, including empty. Original sample scores and journal validity cannot be reconstructed.

## invariant

Return an input tuple unchanged. Unique indices give permutation invariance. Statistical interpretation assumes finite p in [0,1] but code does not enforce it.

## cost

O(R) time, O(1) extra storage. Caller still sorts for reporting, so extraction adds a scan rather than a speed gain.

## tie-stability

Distinct indices and fixed observations make all input permutations choose the same row. Finite oracle checks 60 combinations in forward/reverse orders; production indices derive from history-sorted proposals.

## threshold-partition

dev<=0, validation<0 and p>=0.05/NaN each forbid adoption. Boundary p=0.05 is rejected. Compiled <= mutant fails rejection_thresholds_and_configuration_order.

## composition

Actual caller sorts results for printing then invokes selector. With unique indices this equals former sorted-first eligible selection.

## epoch-memory

Wave14 first modeling epoch: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

## limitations

Partial syntax binds source bytes; macro bodies remain opaque and semantic AST is absent. Native/structural checks are not universal proofs or full-system coverage. No approximation error budget or automatic substitution is claimed. File evidence distinguishes executed pure paths and inspected effects.

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### Undated · document

Subject: world/src/tune.rs SHA-256 932ebce85aebb89467cb9da0d82b136debed8b77f6a9f0b8e852336480db0adb. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
