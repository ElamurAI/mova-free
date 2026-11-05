# en-model-graph

## descriptant

### model: graph

GNode bytes encode fixed fields and zero reserved bytes. Graph sym resolves dictionary first, else deduplicates local strings by linear search. str requires valid symbol index; local heap state is not reconstructible from discarded text whitespace.

## approximant

### graph forward scope

GNode bytes encode fixed fields and zero reserved bytes. Graph sym resolves dictionary first, else deduplicates local strings by linear search. str requires valid symbol index; local heap state is not reconstructible from discarded text whitespace.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Model combines Lexicon,PTagger,Morph,Parser,Linearizer and retained base sentences. Graph has ordered GNodes, local string heap, question and positional capitalization bits.

Raw article escapes cannot undergo a/an regeneration. Graph local symbols cannot reconstruct omitted arbitrary whitespace; lossless means stored token/order/space information. Encode head cast to u16 can truncate beyond65535. adapt leaves parser/tagger/base unchanged but may replace lin even at weight0.

encode includes delegated tokenizer/tagger/parser plus O(n*L) local symbol linear search (L local heap length), string work. decode adds linearizer cost only when generative. adapt allocates base+weight*pseudo tokens, then linearizer training. u16 head cast and usize allocations lack checked bounds.

## models

## behavior_complement

Raw article escapes cannot undergo a/an regeneration. Graph local symbols cannot reconstruct omitted arbitrary whitespace; lossless means stored token/order/space information. Encode head cast to u16 can truncate beyond65535. adapt leaves parser/tagger/base unchanged but may replace lin even at weight0.

## reverse_reconstruction

Given Graph g and model M, compatible source texts are those encoding to g; local heap and raw cases constrain strings but whitespace normalization can yield multiple antecedents. decode(g,true) is a canonical reconstruction, not a universal byte-inverse. Serialized Model state cannot identify training order/history uniquely.

## node_wire

GNode.bytes writes lemma4,tag1,case1,rel1,space1,head2 little endian and two zero reserved bytes; layout check is separate from language semantics.

## article_escape

Only DT a/an in lowercase or title case can normalize when vowel-letter rule reproduces input. Exceptions and other case patterns use Raw.

## capital_position

First Title node becomes AsIs plus cap_first unless proper noun, I, or inherently uppercase lemma; decode transfers title to first emitted node.

## adaptation_effect

Pseudo labels are own predictions; weight0 retrains from base, empty domain supplies no extra evidence. No external teacher or persistence occurs.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: model::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
