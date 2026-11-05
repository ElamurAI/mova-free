# octopus-english-realizer

## model

# octopus-english-realizer

## Descriptant

Small deterministic English realizer for animals' answers: counted nouns, verb agreement, past forms from Mova's morphology, lists and sentences. Source: `tools/octopus/src/realize.rs`.

## Forward approximant

noun(n,l)= l if n==1 else Morph.inflect(l,NNS); noun_count = 'n noun'; be/was/has agree with n; past/participle = Morph.inflect(l,VBD/VBN); list joins with ', ' and ' and '; sentence capitalises the first letter and adds a full stop unless the text ends with . ! ?; could_not(v,o,b) = 'I could not v o because b.'; paragraph joins non-empty sentences.

## Invariant

Output states only the caller's values; deterministic; number agreement follows the given count.

## Cost

O(output length) plus dictionary lookups.

## Composition

Parent: `native-octopus-deputy`. Pure functions; plural and past forms from en::morph::Morph (built-in dictionary plus suffix rules).

## behavior_complement

Not done: articles, irregular plurals absent from the dictionary beyond suffix rules, pronoun choice, tense other than present and simple past, any content selection. Counterexample checked: past('get') = 'got' from the dictionary, not 'getted'. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A realized sentence determines the counts and items it names; the function and inputs are recoverable only up to equivalent phrasings (e.g. 0 and 2 both give plurals). No unique inverse is claimed.

## tests

Command: cargo test --manifest-path tools/octopus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL (16 tests including the unchanged native controls; two identical runs).

Negative controls: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site; not a proof.

Cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs)

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: shared thousands(), utc() and report(Report) for animals' answers (used by the Whales; Sea Spider keeps its own copies).

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/octopus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL (16 tests including the unchanged native controls; two identical runs on 2026-11-04).

Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site (2026-11-04); not a proof.

2026-11-04: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs on 2026-11-04)
