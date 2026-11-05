# math-calc-parser

## descriptant

### math-calc-parser

lex/P normalize aliases, implicit multiplication and precedence; power associates right. Comparison chains expand at most two comparisons. parse_equation extracts top equality or appends0. Preimages include whitespace/aliases, no raw-byte inversion. O(B) token storage, uncapped nesting.

## approximant

### Scoped forward and cost

lex/P normalize aliases, implicit multiplication and precedence; power associates right. Comparison chains expand at most two comparisons. parse_equation extracts top equality or appends0. Preimages include whitespace/aliases, no raw-byte inversion. O(B) token storage, uncapped nesting.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

lex/P normalize aliases, implicit multiplication and precedence; power associates right. Comparison chains expand at most two comparisons. parse_equation extracts top equality or appends0. Preimages include whitespace/aliases, no raw-byte inversion. O(B) token storage, uncapped nesting.

Source range[6098,17753) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

parse('2^3^2') excludes left-associated (2^3)^2; successful parse excludes unconsumed tokens. Unknown identifiers followed by parentheses exclude Call unless name is recognized.

## reverse_reconstruction

A canonical Call('binom',...) admits C/choose/nCr/binomial/binom spellings; equivalent whitespace and normalized multiplication symbols are lost. Output E does not recover original bytes.

## binding_precedence

pow consumes unary exponent recursively: 2^-1 works and -2^2 becomes Neg(Pow(2,2)). Postfix factorial/index binds before power.

## comparison_expansion

a<b<c becomes And(Lt(a,b.clone),Lt(b,c)); parser supports at most this second comparison, so a fourth comparison leaves tokens and errors.

## equation_projection

parse_equation unwraps only root Eq; every other accepted E maps to(E,0). An And chain involving equality remains an expression against zero.

## lexical_failure

Character scan recognizes exponent digits only when e/E is followed by valid signed/unsigned digits. Unknown symbol errors before parsing; malformed number text fails parse_q during primary.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

lex/P normalize aliases, implicit multiplication and precedence; power associates right. Comparison chains expand at most two comparisons. parse_equation extracts top equality or appends0. Preimages include whitespace/aliases, no raw-byte inversion. O(B) token storage, uncapped nesting.

## wave34-composition

# Parser descendant composition

The lex-normalization child runs before P exists: lexical Err prevents all parsing. Its canonical token stream feeds or/and/not/comparison/add/mul/unary and the power-association child. Power receives a cursor into that stream, consumes postfix and recursively unary exponents, then returns to the surrounding precedence layer. parse finally rejects unconsumed tokens. The two child ranges overlap the enclosing parser relation but do not exhaust it: function recognition, list parsing, comparison expansion and parse_equation remain modeled by the historical parent.

A compatible parser antecedent must satisfy both lexical spelling equivalence and the cursor/precedence relation; arbitrary lexer and power preimages cannot be paired without the same token stream. Exclusion at the parent includes failure ordering: lexical failure precedes numeric primary validation, which precedes trailing-token rejection when reached.

## Cost

Lexing O(C) plus parser work over tokens; power's recursive depth is part of parser stack use. Do not sum overlapping parent and child token traversal estimates. No nesting or allocation bound is proved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.
