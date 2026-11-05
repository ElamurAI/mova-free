# onto-hs

## descriptant

### onto-hs

read_codes skips header, requires two CSV fields/six digits/increasing code. Tree deduplicates prefixes2/4/6. bounds scans comparator hits, borrows next then previous unit, scales Rat, joins opposite endpoints of last compatible attr/base interval. parse uses ordered chunk branches, colon levels and parenthesis state; main first deepest Item else Head.

## approximant

### Forward relation

read_codes skips header, requires two CSV fields/six digits/increasing code. Tree deduplicates prefixes2/4/6. bounds scans comparator hits, borrows next then previous unit, scales Rat, joins opposite endpoints of last compatible attr/base interval. parse uses ordered chunk branches, colon levels and parenthesis state; main first deepest Item else Head.

#### Equations

x_base=x_Rat*f_unit. overlap(a,b)=¬below(a.hi,b.lo)∧¬below(b.hi,a.lo), below(h,l)=(h<l)∨(h=l∧¬(h_closed∧l_closed)). main=first argmax_level(Item) ⊕ first(Head).

#### Domain

Rat conversion exact when representable; scaling overflow skips hit. overlaps ignores dimensions/basis. CSV quote scanner permissive; Tree assumes valid ASCII. Parenthesis state restored.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

## invariant

### Invariant and domain

Rat conversion exact when representable; scaling overflow skips hit. overlaps ignores dimensions/basis. CSV quote scanner permissive; Tree assumes valid ASCII. Parenthesis state restored.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

## cost

### Cost approximation

CSV O(B); bounds nearest-attribute/unit scans O(T²) worst-case; NP/tail recursion may quadratic. u8 colon level overflow beyond255; no allocation bound.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

x_base=x_Rat*f_unit. overlap(a,b)=¬below(a.hi,b.lo)∧¬below(b.hi,a.lo), below(h,l)=(h<l)∨(h=l∧¬(h_closed∧l_closed)). main=first argmax_level(Item) ⊕ first(Head).

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- thresholds: endpoint=x*factor; complete last interval on opposite missing side with same attr/base

- chunks: (tokens,exclude,cont,level)→ordered branch features; parenthesis restores exclude/cont

## models

## behavior_complement

No comparator excludes numeric threshold; open touching endpoints exclude overlap unless both closed. Whether clause excludes distinguishing features. Bad code order excludes Ok.

## reverse_reconstruction

Bound preimages include synonyms and rescaled kg/g, cc/l values; Parsed loses punctuation/filler/optional content. Tree loses descriptions.

## unit_transport

base=value*Rat factor; percent magnitude retained, cc factor1/1e6; volume basis precedes weight.

## interval_topology

below iff h<l or equal and not both inclusive; caller aligns attr/base/basis.

## chunk_state

Exclude/Of/Has/Purpose persist per level; parentheses restore exclude/cont after nested parse.

## main_projection

First maximal-level Item else first Head; empty parse None/zero counts.


## tests

### Actual-code checks

- `hs::t::bound_overlap`
- `hs::t::csv_and_tree`
- `hs::t::features_of_telegraphic_style`
- `hs::t::thresholds_exact`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-hs.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
