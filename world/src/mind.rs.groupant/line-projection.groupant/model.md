# world-mind-line-projection

## forward

P(text) concatenates, in line order, babi::split_sentences(trim_start_matches('- ', trim(line))) for nonempty lines not starting '#'. Repeated bullet prefixes are stripped; a bare '-' remains text. Sentence splits require punctuation, ASCII space and an uppercase or supported quote successor. Headings are filtered before bullet removal.

## invariant

Output order preserves retained line/sentence order. No line crosses a newline. Trimmed headings contribute zero output; bullet-prefixed headings can survive. Sentence segmentation is a lexical heuristic, not an English parser.

## cost

Linear input scans plus allocated line/sentence bytes; babi splitting temporarily collects chars. No optimization or timing claim.

## composition

Source callers: main dispatches probe/serve/say; family preserves each Member Story, sets identity before tell, broadcasts statements and ignores tell gaps; worldbook::perceive uses ok+gaps.len as projected sentence count then separately classifies gaps as knowledge. Native public join: set_me -> tell/reply on zero projected sentences -> unchanged Story::state_lines and preserved identity (same name accepted, different name rejected). Nonempty Story read/ask remains opaque; no parser has been invoked for these checks.

## epoch-memory

Initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.

## checks

cargo test -p world --lib mind::wave12_models -- --test-threads=1. File ../model.md#evidence binds repeated native profiles and compiled production controls. Partial syntax leaves macros opaque; source-bound syntax is not semantic AST verification.

## behavior-complement

Domain and observation scope: project(text) over finite UTF-8 text and candidate sentence vectors with fixed babi splitter revision.

Let P be the forward line-filter/bullet/split concatenation. U contains (text,candidate-vector) pairs; U minus graph(P) excludes contributions from filtered blank/heading lines, reordered or missing retained segments, and segments differing from the fixed splitter output. '- # keep' survives because heading filtering precedes bullet removal; bare '-' survives. 49 pair fixtures and the heading-order case support these exclusions without claiming English-parser correctness.

## reverse-reconstruction

B(v)={text | P(text)=v}. Preserve exact segment order and splitter-boundary compatibility; permit ignored headings, blank lines, trimmed whitespace and removable bullet prefixes. Different physical-line layouts can share v; original formatting/text is not uniquely recovered.

## coupling

49 pairwise line fixtures use literal expected sentences, independently of split_sentences. Heading/bullet order has a separate fixture. Inserting filtered lines must preserve the output vector.

Typed entries bind the manifest source revision, this section's domain/operator/observation, and the named finite checks. Status is partial source-derived model. No complete reconstruction search, universal proof or factual-truth guarantee is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30: initial source-bound review. Prospective cost model predicts no asymptotic change: selection helper extraction preserves sorting and cloning; list repair replaces one scalar division with checked division. Old zero-limit native witness divides by zero. Final implementation returns a nonzero stride and take(0) produces no rows. Mind production behavior is unchanged. No surrogate, speed or full-system correctness claim.
