# prag-frames-md

## model

# English/Hebrew supplied-frame Markdown transport

## forward

English scans non-Bare frames in order and renders ten columns with original one-based indices. Hebrew projects accepted CoNLL-U fields, joins normalized MIds, and renders six columns. Empty inputs produce the respective no-frame note. These observations are bytes, not linguistic truth.

## behavior-complement

Universe: finite supplied frame/word vectors paired with proposed bytes. Excluded families include a Bare frame rendered as its own row, dense renumbering after skipped Bare frames, a Hebrew row without nonempty Frame metadata and an earlier duplicate MId winning a participant lookup. These are source-derived inner exclusions, not exhaustive absent behavior.

## reverse-reconstruction

For output y retain every vector whose rendered projections yield y. Varying English src/cause/next/quote fields alone leaves y unchanged. Hebrew accent removal, rejected CoNLL-U lines and overwritten MIds create additional fibers. Empty notes admit arbitrary Bare-only or frameless vectors. No original parse is uniquely recoverable.

## invariant

Inputs are unchanged; visible English coordinates retain original indices. Escaping does not remove line breaks; complete Markdown sanitation is not guaranteed.

## cost

English O(sum entity subtree bytes across recursive ancestors + output + oblique/role lookup work); chain copying can be quadratic. Hebrew O(B+W log W+VW+P log W+output) for bytes B, words W, verbs V and participant IDs P.

## column-transport

Fix roles and flags: who/what/to-whom carry agent/patient/recipient respectively; unsaid applies only to filled slots. Native exact-column checks preserve this wiring.

## provenance-loss

Holding rendered fields fixed while changing src/cause/next/quote leaves output equal. The explicit two-antecedent native fixture falsifies unique provenance recovery.

## composition

English composes phrase, ent, TAM and escaping. Hebrew composes hwords with hebrew_table. Inspected roundtrip site_code::tables removes the exact trailing note; its Hebrew sites call hwords then hebrew_table. External caller execution is not claimed.

## epoch-memory

Wave14 first modeling epoch: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

## limitations

Partial syntax binds source bytes; macro bodies remain opaque and semantic AST is absent. Native/structural checks are not universal proofs or full-system coverage. No approximation error budget or automatic substitution is claimed. File evidence distinguishes executed pure paths and inspected effects.

## evidence

Receipt set (see History). Six frames_md and four tune tests pass twice debug/twice release with identical ten named outcomes. Compiled production controls: 0591 -> 0592 fails text_escape_and_accent_projection_have_distinct_fibers; p<0.05 -> p<=0.05 fails rejection_thresholds_and_configuration_order. Sources restore exact bytes before positives. Both partial syntax projections repeat identically. Finite families: 169 escape pairs, 27 neighbor cases, 60 selection cases in both orders, explicit boundaries/duplicates/NaN and exact columns. No repair, performance or universal correctness claim; unrelated existing compiler warnings remain.

- mutant-frames.log: (see History)

- mutant-tune.log: (see History)

- debug-1-named.txt: (see History)

- debug-2-named.txt: (see History)

- release-1-named.txt: (see History)

- release-2-named.txt: (see History)

- frames-syntax-1.json: (see History)

- frames-syntax-2.json: (see History)

### Evidence

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 023ca98fa461e9d1545915abd8071ecb542cc61999bbf5a054b712e5edf9b5e3.

Receipt set octopus-wave14-formats-2026-10-30. Six frames_md and four tune tests pass twice debug/twice release with identical ten named outcomes. Compiled production controls: 0591 -> 0592 fails text_escape_and_accent_projection_have_distinct_fibers; p<0.05 -> p<=0.05 fails rejection_thresholds_and_configuration_order. Sources restore exact bytes before positives. Both partial syntax projections repeat identically. Finite families: 169 escape pairs, 27 neighbor cases, 60 selection cases in both orders, explicit boundaries/duplicates/NaN and exact columns. No repair, performance or universal correctness claim; unrelated existing compiler warnings remain.

- mutant-frames.log: 6070f06e77741ee9df00c1b2eecc1c17558b76bddc7030618f5097bea1b34e2e

- mutant-tune.log: b5dd80aaed1d0cc654d35efcc033ad265d219b32ae0751324e6624f1a2421aba

- debug-1-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- debug-2-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- release-1-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- release-2-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- frames-syntax-1.json: fabdf640e4edc7c32b8f5d9a466f9cbc6184524aacc60592e7dbb6277b887bff

- frames-syntax-2.json: fabdf640e4edc7c32b8f5d9a466f9cbc6184524aacc60592e7dbb6277b887bff

### Undated · document

Subject: prag/src/frames_md.rs SHA-256 023ca98fa461e9d1545915abd8071ecb542cc61999bbf5a054b712e5edf9b5e3. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
