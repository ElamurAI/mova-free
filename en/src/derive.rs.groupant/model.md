# en-derive

## descriptant

### en-derive

Derived words: a form the dictionary does not know read as a known open-class word with productive affixes (prefix, suffix, inflection on a derived word, British spelling variant); rules are seed data (```derive blocks), spelling by join.

## approximant

### Forward approximant

parse reads ```derive lines (prefix/suffix/inflect/variant with options after=, except=, block=, start=, nostart=, part=) into Rules in seed order. go(w, depth) memoised: a dictionary word of an open class with >=4 letters (>=5 when UD lacks it) is a base with its dictionary tags; else each rule cuts its affix: variant rewrites the British ending and recurses; prefix (optional hyphen) recurses on the rest (shortest stem, start/nostart, except, block by rival prefix + rest in the dictionary) and keeps the stem tags of its classes (part= maps participles); suffix/inflection try stems(rest) and keep a stem s only when join(s, affix) contains w, s ends in after= and not in except=, no rival block suffix makes a dictionary word, the stem base-class matches, and an inflection only on a derived stem. Analyses sorted by (steps, prefix-final). analyses() is empty for dictionary words and Early Modern endings (morph::ending_verb). tags() unions the tags of analyses of the best rank; hint() is tags unless EN_DERIVE_TAGS=0.

Domain/state: Compiled seed texts, OnceLock rule table, built-in dictionary; EN_DERIVE_TAGS read once.

Executable surrogate: unsupported. The outputs are categorical (analyses, tags); no calibrated percentage metric. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

Every analysis ends in an in_dictionary base with letters >= MIN_BASE and (in UD or >= MIN_RARE_BASE); steps.len() <= DEPTH; each step word = the next step stem; the last step word = the analysed form; tags nonempty and sorted. Deterministic: rules in seed order, BTreeMap memo, stable sort.

Cost: Per unknown word: memoised recursion over (string, depth) with R rules (~80), each cut O(len) and <= 9 stem candidates; dictionary probes O(1) expected; worst case O(R^DEPTH) bounded by distinct suffixes of the word in the memo. Rules parsed once (OnceLock).

Limits: The dictionary (AGID) lacks many non-comparable adjectives (directional, objective, lateral only as NN); the lexicon extension adds their JJ reading to a base, a base neither has keeps its dictionary tags; Rare false analyses of names and misspellings remain (poppler = popple + -er, spooner, unlogical); Compounds (treebank) are out of scope, and irregular derivatives (grammatical, artificial) come from the lexicon extension, not from rules; No meaning is derived; glosses are explanations only

## composition

### Composition

parse reads ```derive lines (prefix/suffix/inflect/variant with options after=, except=, block=, start=, nostart=, part=) into Rules in seed order. go(w, depth) memoised: a dictionary word of an open class with >=4 letters (>=5 when UD lacks it) is a base with its dictionary tags; else each rule cuts its affix: variant rewrites the British ending and recurses; prefix (optional hyphen) recurses on the rest (shortest stem, start/nostart, except, block by rival prefix + rest in the dictionary) and keeps the stem tags of its classes (part= maps participles); suffix/inflection try stems(rest) and keep a stem s only when join(s, affix) contains w, s ends in after= and not in except=, no rival block suffix makes a dictionary word, the stem base-class matches, and an inflection only on a derived stem. Analyses sorted by (steps, prefix-final). analyses() is empty for dictionary words and Early Modern endings (morph::ending_verb). tags() unions the tags of analyses of the best rank; hint() is tags unless EN_DERIVE_TAGS=0.

Consumers: morph::known_form and Morph::lemmatize (derived lemma), ptag::dict_mask and tag::Tagger::unknown (dictionary hint), main derive and lexicon-cover; readers outside en (tools/whale reader, papers qd) see more known words through known_form.

## models

## behavior_complement

No analysis for: a dictionary word; a non-ASCII-lowercase/hyphen string; a bottom word of a closed class, a proper noun, shorter than 4 letters, or of 4 letters known only from AGID; a prefix rest shorter than the rule minimum or with a second hyphen; a spelling join does not give back (misspellings happyness, noticable, arguement, basicly); a blocked rival (unpossible, readible, inpossible, foregetting); an inflection on a dictionary word; more than 4 steps; an EModE ending on a known verb (biteth).

## reverse_reconstruction

compose(stem, rule) = prefix + stem (or with hyphen), the British ending for a variant, join(stem, affix) for suffix/inflection: every step word is in compose(step.stem, step.rule) (test steps_compose_back). The reverse of go is many-valued: one form has several analyses (downsampled = down- + sampled = downsample + -ed); the analysis does not recover meaning or the lexical bracketing, only compatible stems.

## spelling_join

join(stem, affix): -ue loses e; -e dropped before a vowel except c/g + a/o and ee before non-e; consonant+y → i (dropped before -ic, -ical, -ist, -ism, -ize, -ify; kept before i); CVC may double before a vowel; -able/-ible + -ity → -ability/-ibility; -ly: -le → -ly, -ll → -lly, none after -ic; -s/-es by sibilant or consonant+y. Accepted analyses satisfy w ∈ join(stem, affix).

## ranking

Analyses ordered by (steps, last step is a prefix); tags come from the analyses of the best rank, so intensely is intense + -ly (RB), not in- + tensely.

## tests

### Evidence

Checks: derive::tests::rules_parse; derive::tests::derived_words_of_papers; derive::tests::conservative; derive::tests::steps_compose_back; derive::tests::spelling; derive::tests::suffix_before_prefix; derive::tests::deterministic.

cargo test --release -p en --lib: all en library tests passed (exit 0), run twice with identical outcomes. Negative controls are inside the tests (malformed derive lines must fail to parse; misspellings, blocked rivals, short or rare bases, names and EModE endings must give no analysis; contexts with a comma, a coordinator, a result modal or a negation must not force IN). A compiled mutation control is recorded in the branch report. No corpus, training, held-out set, GPU, teacher or API was run by the tests.

## History

### 2026-11-04 · descriptant

2026-11-04 modeling epoch (en-morph): new source; tests written with the code, each with negative controls; measurements in en/README.md (Measurements 2026-11-04). A registered leaf is partial coverage, not a proof.

2026-11-04 epoch (lexicon growth): go takes a word of the lexicon extension (en-lexext) as a base below the form, with its classes added to a dictionary word's (directional gains JJ, so bidirectional is JJ; supervised gains JJ); analyses() is empty for an extension word, derivations() still explains it (en derive); hint() is derivation only, the taggers ask lexext::hint first. Tests now read derivations(); the negative controls run on derivations, so the extension cannot hide a false analysis.

### 2026-11-04 · tests

Heading `### Evidence — 2026-11-04` stays in tests as `### Evidence`.
