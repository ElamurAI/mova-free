# math-calc-parser-lex-normalization

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

State (character vector, cursor, tokens) advances left to right. Numeric runs admit digits and dots, then an optional well-formed exponent suffix. Alphabetic/underscore starts identifiers; bracket/equality/multiplication/minus specials precede the ordered symbol table. Unknown characters return Err with no token vector. Recognized aliases collapse to canonical Sym tokens.

## Invariant

On successful lexing the cursor reaches the character count; identifiers preserve spelling and number strings preserve their scanned characters. Tokenization does not itself validate numeric spelling.

## Cost

O(C) scans and O(C) character/token storage with a fixed 22-entry symbol table; symbol candidate allocations are bounded constants per nonword character.

## behavior_complement

For successful normalized multiplication tokens, transitions emitting × or · rather than Sym(*) are excluded. Unknown nonwhitespace characters exclude Ok; malformed 1..2 is not excluded by lex and fails later in primary.

## reverse_reconstruction

A Sym(*) token admits *, × and · source spellings. A Sym(^) admits ^ and **. Whitespace insertion and = versus == are distinct antecedents; original UTF-8 offsets cannot be reconstructed from tokens.

## exponent_admission

Holding preceding digit run fixed, e/E joins Num only when followed by a digit or sign plus digit. 1e becomes Num(1),Id(e); 1e-2 is one Num. Oracle: inspect lex outputs on these bounded strings.

## unicode_coordinates

Cursor counts Rust chars, not UTF-8 bytes. Equivalent × and * advance one character but differing byte counts. Source anchors use bytes; lexical cost uses characters.

## numeric_validation_split

Num(1..2) is reachable from lex; parse_q in primary decides rejection. A lexical success therefore cannot certify a rational literal. Oracle: lex succeeds and parse fails.

## symbol_precedence

The ordered table tests ** before *, != before !, <= before <. Fixed suffix overlap determines token count; changing this order changes the E accepted by parse.

## Tests

Actual helper fixture: `calc::wave34::syntax_zero_and_binding_boundaries`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
