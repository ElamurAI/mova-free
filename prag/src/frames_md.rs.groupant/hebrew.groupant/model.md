# Hebrew tolerant columns and participant identity joins

## forward

hwords skips comments/fences, short rows, range/decimal IDs and unparsable integer IDs. Keep columns 0,1,2,6,5,9; malformed head becomes zero. kv uses first exact key=. bare deletes U+0591..U+05AF. MId map strips leading nondigits, last duplicate wins. Semicolon Frame parts inherit current role; unresolved IDs skip. Negation joins integer head to verb.id for four exact lemmas.

## behavior-complement

Universe: supplied text/words and proposed projections. Exclude accepted range/decimal IDs, preserving deleted cantillation, resolving an absent MId, or a row with empty Frame. First-key/last-MId collision policies exclude their opposites.

## reverse-reconstruction

hwords loses ignored columns and rejected lines; invalid and explicit zero heads coincide. bare loses accent positions. Rendering loses MId prefixes and shadowed duplicates. Full original verse reconstruction is ambiguous.

## invariant

Accepted row order and field indices persist despite extra columns. Current role persists after an unresolved ID. Unique MIds and valid dependency trees are not enforced.

## cost

O(B+W log W+VW+P log W+output). Multiple roles may render one word repeatedly.

## mid-collision

For normalized key 2, a later record shadows earlier old for all participant references. References themselves are not prefix-normalized. Exact-row native fixture checks later new and inherited A0.

## field-transport

Observe six HWord fields; extra/ignored columns remain arbitrary in reverse. Invalid head maps to zero whereas invalid id drops the row. Synthetic CoNLL-U checks feats/misc transport without parser execution.

## composition

Integer dependency-head keys drive negation while normalized MId strings drive roles. hwords and hebrew_table are separate public APIs joined by inspected site callers.

## epoch-memory

Wave14 first modeling epoch: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

## limitations

Partial syntax binds source bytes; macro bodies remain opaque and semantic AST is absent. Native/structural checks are not universal proofs or full-system coverage. No approximation error budget or automatic substitution is claimed. File evidence distinguishes executed pure paths and inspected effects.

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### Undated · document

Subject: prag/src/frames_md.rs SHA-256 023ca98fa461e9d1545915abd8071ecb542cc61999bbf5a054b712e5edf9b5e3. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
