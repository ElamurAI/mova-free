# Configuration serialization and neighbor enumeration

## forward

show emits lexical BTreeMap order. For each knob select first value position p with abs(value-base[k])<1e-9; emit existing p-1,p+1 or all values if no match. Each output clones base and replaces only k; label includes current/new values. Default from the knob triple is ignored here.

## behavior-complement

Universe: knob/base/candidate-list triples with every knob key present. Exclude nonadjacent emitted positions after a match, reordered knob declarations, or changing any other base key. Empty value lists exclude proposals. Duplicate/no-op maps are allowed.

## reverse-reconstruction

Proposals lose unchosen list entries, ignored knob defaults and some declaration histories when maps/labels coincide. Nearby current values can share tolerance match. NaN never matches and preserves fallback behavior.

## invariant

Pure read-only inputs; same map key set per result. Missing knob key panics. Duplicate/nonfinite values are retained, not validated. No global size cap.

## cost

O(sum list lengths+D*K+output bytes), output O(D*K+bytes), transient list at most candidate-list length.

## tolerance-domain

Strict tolerance uses the first match. Overlapping values can emit a no-op neighbor. NaN and infinity-minus-infinity do not match; supplied tests expose nonfinite fallback rather than asserting numeric safety.

## one-key-transport

For selected k, c[j]=base[j] for all j!=k and c[k] is declared emitted value. Independent finite oracle checks 27 length/current combinations, labels and untouched y.

## composition

Caller builds base from defaults first; helper enumeration precedes journal ordering. Therefore helper order is not final candidate index order.

## epoch-memory

Wave14 first modeling epoch: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

## limitations

Partial syntax binds source bytes; macro bodies remain opaque and semantic AST is absent. Native/structural checks are not universal proofs or full-system coverage. No approximation error budget or automatic substitution is claimed. File evidence distinguishes executed pure paths and inspected effects.

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### Undated · document

Subject: world/src/tune.rs SHA-256 932ebce85aebb89467cb9da0d82b136debed8b77f6a9f0b8e852336480db0adb. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
