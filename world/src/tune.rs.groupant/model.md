# world-tune

## model

# Neighbor proposals and adoption with explicit effect boundary

## forward

Build defaults into base, score base dev/validation, enumerate neighbors, order by positive journal history then delta label, reuse or score each result, sort by dev gain/index, and optionally adopt a positive-dev/nonnegative-validation/p<0.05 result for at most gens iterations. Nonadopt stops after first generation. run(domains) may persist hot config.

## behavior-complement

Universe: successful finite orchestration traces under callback, IO and arithmetic premises. Exclude more than gens adoptions, adopting dev<=0/validation<0/p>=0.05 or NaN, changing two map keys per proposal, and adoption with adopt=false. Do not exclude journal writes without adoption or side-effectful callbacks.

## reverse-reconstruction

Final base cannot identify trial count, journal hits, callbacks, initial state or rejected alternatives. Zero generations/nonadoption returns defaults for many traces. Chosen tuple cannot recover scores, bootstrap samples or configuration without deltas.

## invariant

Original public signatures remain; pure seams do not mutate inputs. Selection equivalence assumes unique candidate indices, true in production. Score-vector alignment, i64 representability, callback safety and IO success are unvalidated premises.

## cost

Per generation neighbors O(sum list lengths+D*K+bytes); history sorting O(D log D * J) journal scans, external score/bootstrap costs, result sort O(D log D), selection O(D). No speed claim; requested threads are uncapped.

## validation-reuse

Every new candidate queries score_held and adoption requires nonnegative held gain: it is a tuning validation set, not untouched final-test evidence. Adaptive reuse prevents a final accuracy interpretation.

## journal-identity

Keys combine code/component, hash(show(base)), data_key and delta label. Hash collisions and path+size data keys can alias distinct content; cached observations do not establish same callbacks or data.

## composition

run loads files and builds Component; tune joins neighboring_configs with history sorting before assigning indices, then selected index back to deltas. Only pure seams execute in tests. Journal/scoring/IO orchestration remains source-inspected.

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

- tune-syntax-1.json: (see History)

- tune-syntax-2.json: (see History)

### Evidence

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 932ebce85aebb89467cb9da0d82b136debed8b77f6a9f0b8e852336480db0adb.

Receipt set octopus-wave14-formats-2026-10-30. Six frames_md and four tune tests pass twice debug/twice release with identical ten named outcomes. Compiled production controls: 0591 -> 0592 fails text_escape_and_accent_projection_have_distinct_fibers; p<0.05 -> p<=0.05 fails rejection_thresholds_and_configuration_order. Sources restore exact bytes before positives. Both partial syntax projections repeat identically. Finite families: 169 escape pairs, 27 neighbor cases, 60 selection cases in both orders, explicit boundaries/duplicates/NaN and exact columns. No repair, performance or universal correctness claim; unrelated existing compiler warnings remain.

- mutant-frames.log: 6070f06e77741ee9df00c1b2eecc1c17558b76bddc7030618f5097bea1b34e2e

- mutant-tune.log: b5dd80aaed1d0cc654d35efcc033ad265d219b32ae0751324e6624f1a2421aba

- debug-1-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- debug-2-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- release-1-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- release-2-named.txt: ec94e18440b60901cf29185164fc19db8b0ad5f14e3ae956ec8b9a9b0b6e1162

- tune-syntax-1.json: 3c680420be12e82d36c6f25f855c49b0ec1609b5708f2d13af33f4234026dc73

- tune-syntax-2.json: 3c680420be12e82d36c6f25f855c49b0ec1609b5708f2d13af33f4234026dc73

### Undated · document

Subject: world/src/tune.rs SHA-256 932ebce85aebb89467cb9da0d82b136debed8b77f6a9f0b8e852336480db0adb. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
