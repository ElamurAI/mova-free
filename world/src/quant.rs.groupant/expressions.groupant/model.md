# world-quant-expressions

Tokenizer, precedence parser, evaluator and literal collection.

## forward

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

Tokenize operators/whitespace; parse term before expression; atoms are parentheses, rationals or lowercase dotted references. eval recursively applies exact rational arithmetic; nums_of traverses literals left-to-right.

## invariant

Accepted parse consumes all tokens. Undefined references and zero divisors fail. Direct Ex::Bin with an unknown operator falls into division; the model restricts to parser-produced operators.

## cost

O(L) tokenization and O(E) node construction, recursive rational costs dependent on bit length; depth may reach O(E).

## composition

Parsing feeds ordered execution; literal collection precedes evaluation against state, and check merges parse errors, execution diagnostics and gold inequality. Text extraction is an external exact dependency. Numeric-origin errors do not suppress effects, unlike evaluation and anchor errors; a parent must preserve this distinction. Dataset/teacher adapters do not establish script semantics.

## epoch-memory

Regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.

## checks

See ../model.md#tests and native quant::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

Universe: source strings and proposed parsed/evaluated results under fixed state. Trailing tokens, missing parentheses and unary minus exclude successful parsing; these form an inner absent set, not a full resource-safety model.

## reverse-reconstruction

For state s and rational r retain all accepted expressions evaluating to r. Equivalent parenthesizations and arithmetic identities produce multiple preimages; AST still loses whitespace.

## rational-oracle

For a,c in 0..8 and b in 1..8, ((a+c)/b)-c*a=(a+c-c*a*b)/b exactly. Tests compare 648 evaluations plus six malformed strings and zero division.

## reference-normalization

Case variants map to the same lowercase Ref, whereas Num retains original numeric spelling. Reverse formatting therefore has different fibers for reference and numeric atoms.

## History

### 2026-10-30 · epoch-memory

2026-10-30: regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.
