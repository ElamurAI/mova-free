# Sentence-to-quantity and question reading

## forward
Scope: kids, verb_of, subject, effect, read and read_sentences. read retains normalization, inclusive punctuation split and parse_words acquisition. Supplied read_sentences accepts ordered Word vectors and question flags. For each admitted integral numeric token, use nummod head as counted noun, climb to verb/root, resolve subject and previous last name, then attach rate/comparison/time/tags. Last PROPN nsubj selection actually uses the first matching subject in the sentence; update after quantities.

## invariant
Within valid-head domain, heads remain one-based and zero means root. verb_of is bounded by sentence length, subject by four conjunction hops. Empty supplied sentences increment sent without generating Q. Tags preserve duplicates/order; they are not a mathematical set. in_q is an observation, not a universal exclusion flag.

## cost
Child enumeration scans all n words. Verb ascent is O(n); tag expansion can nest scans and give O(n^2) per quantity, conservative O(n^3) per sentence plus external parsing. No corpus-speed claim.

## behavior-complement
Without numeric admitted-POS words, no Q is produced. A pronoun in the first sentence cannot recover an unobserved last name from this mechanism. The model does not reject malformed heads safely; out-of-range heads can panic.

## reverse-reconstruction
Q/Ask discard forms, many dependency edges and unselected words. All valid supplied sentence sequences inducing the same ordered observations remain compatible. A holder string may originate from a proper subject, possessive, earlier name or fallback, so it cannot identify one original syntax.

## boundary
Several question sentences can retain earlier Ask fields that are not overwritten; each public punctuation chunk can be a fragment. Correct supplied trees do not establish correctness of parse_words or language normalization.

## composition
Numeric child supplies token values; this child decorates them; schemas uses Q equality, tags and Ask flags. Same original text is passed separately for whole/part cues, so Q/Ask alone are not the complete schema input.

## checks
supplied_trees_preserve_holder_and_question_boundary checks a prior name, later pronoun, nummod unit, equality effect, empty-sentence numbering and question flags. Native tests never acquire an annotator.

## epoch-memory

Extracted private iterator seam without changing public acquisition or iteration order. Full parse quality and malformed graphs remain gaps. Partial syntax only.

## History

### 2026-10-30 · epoch-memory

2026-10-30: extracted private iterator seam without changing public acquisition or iteration order. Full parse quality and malformed graphs remain gaps. Partial syntax only.
