# prag-schema

## model

# File composition

## forward

Tagged operations Closed(E,s), Get(f,L), Set(f,L,c), Parse(columns), Tsv(L), Json(L). Closed enums have cardinalities 8,16,3,3; six learned fields have cardinalities 8,16,2,2,3,3. Parsing seven supplied columns trims each, validates in form/act/indirect/hedge/polarity/voice/means order, and returns the first error or labels. TSV emits six names/bits and verbatim means. Default is decl/assert/false/false/none/none/dash.

## behavior-complement

U consists of tagged operation inputs paired with outputs of the matching operation, including errors or panic observations. An inner complement excludes successful parses with wrong count, unknown categorical strings, non-bits, empty trimmed means or more than 30 whitespace words; excludes setter outputs changing an unselected coordinate or means; excludes a successful invalid-class setter. These concrete exclusions are source-derived, not established for all Rust executions by the finite tests.

## reverse-reconstruction

Parse preimages retain all whitespace-padded columns producing the same trimmed labels. For a field-vector observation, means is unconstrained and cannot be reconstructed. TSV has a unique label preimage on canonical valid row labels, but parse(Tsv(L)) loses edge whitespace outside that domain and can fail on embedded TAB. JSON and TSV are distinct encodings.

## invariant

Shared field order and class number couple parser, setter, serializer and model consumers. Canonical means domain D requires nonempty trimmed text, <=30 words, and no TAB/CR/LF. Parse(split_tab(Tsv(L)))=L on D. It is not a law for every publicly constructible Labels.

## coupling

Shared field order and class number couple parser, setter, serializer and model consumers. Canonical means domain D requires nonempty trimmed text, <=30 words, and no TAB/CR/LF. Parse(split_tab(Tsv(L)))=L on D. It is not a law for every publicly constructible Labels.

## composition

The closed table, coordinate and codec fragments share exact enums and field order. Classifier indices require the corresponding classes list; schema is not directly used by infer.rs. Agree uses string tables and does not validate this schema. Native tests exercise schema operations; caller inspection is not an executed cross-file join.

## cost

O(B) parser/serializer time and output storage in input/output bytes B; fixed finite enum tables. Field operations O(1); classes allocates O(k), k<=16. No performance measurement or gain claim.

## checks

Six wave13_models tests plus one prior schema test run twice per debug/release profile; model.md#evidence at the file root binds receipts. Literal independent tables cover 4608 categorical rows, 34 valid coordinate updates, 12 invalid boundaries, 127 first-error masks, five word counts and Unicode whitespace fibers. JSON coverage executes Form variants and default Labels; arbitrary serde error cases are untested.

## epoch-memory

Prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.

## evidence

### Executed evidence

Structural checks use an external full-base registry with a validation-only wrapper over the unchanged base tree and this leaf. Both reports pass and are identical; ancestor integration remains the root agent's task. An earlier disconnected scaffold and a leaf-only registry were rejected; neither was a source/model failure or a passing check.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.

### 2026-10-30 · evidence

Heading `### Executed evidence, 2026-10-30` stays in evidence as `### Executed evidence`.

Seven schema tests (six new) pass twice in debug and twice in release with identical sorted names/results. Old Boolean-index witness fails on Indirect 2. Word-limit production mutant compiles and fails the named boundary assertion; exact source restoration passes. Source-bound partial syntax generated twice identically. Receipt set octopus-wave13-schema-2026-10-30. SHA-256:

- prospective.md: 146da37b18bb069863e3d875233ec4bab238cc87902ca73891293ad50e335a6c

- old-witness.log: 653294926e360f8a6732aaa3ecaa29aee535903003eae5677e3fac20b6bed14c

- check.log: bd1d1ba21ccf331675e78d92262af566f2e607cabf1162cf2e3f649c31905a28

- debug-1.names: 0d5dd60967f3bbbf9701c8f2661e048251f0bbbb27051b54223224122e7ceeda

- debug-2.names: 0d5dd60967f3bbbf9701c8f2661e048251f0bbbb27051b54223224122e7ceeda

- release-1.names: 0d5dd60967f3bbbf9701c8f2661e048251f0bbbb27051b54223224122e7ceeda

- release-2.names: 0d5dd60967f3bbbf9701c8f2661e048251f0bbbb27051b54223224122e7ceeda

- mutant.log: 424fccbe3806fca2c53282e3133c68367844ff8151bdcef29fa82a14995d9c46

- restored.log: 96fa063b1900e240b2188942b55156ead358cfdffc66c66a618230449342bad6

- syntax-1.json: e45ed306a43501ae0cec7b530b3d1a6e44b65075016262760661086a995a2ef4

- syntax-2.json: e45ed306a43501ae0cec7b530b3d1a6e44b65075016262760661086a995a2ef4

- scaffold-groupants-1.json and scaffold-groupants-2.json: 71cbb12c2ec7713af175a231824f67ecb1f4850755a98d7b7ffed788b3797938

- changed.json: aa15fca15804bfbb41890c2645b7879ce2f0b120f92bced920f993a0d9a2b719 (schema.rs registered, missing empty)
