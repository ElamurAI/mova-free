# mova-whale-claims

## Descriptant

Numbered citations in a consultant's text: `split` (body sentences with their citation numbers, and the numbered reference list; shared with `mova-topic-corpus` sol.rs), `check` (every citation and reference entry must name a given source, and some sentence must cite), `claims` (each sentence followed to the sentences of the sources it cites) and `claim_key` (a claim's identity across epochs). Source: `tools/whale/src/claims.rs`.

## Forward approximant

split(text) -> ([(sentence without brackets, sorted citation numbers)], {n -> reference line}); a heading «References/Bibliography/Sources/Works cited» opens the list, the next heading closes it; ranges «1-3» expand when shorter than 20. check(text, given) = Err(Unknown(sorted numbers not given)) | Err(Uncited) | Ok(split). claims(prefix, body, given, texts, min_support): per sentence the best content-word Jaccard over the cited sources' sentences; status uncited (no number) | sourced (best >= min_support) | weak.

## Invariant

check never accepts a number outside the given set, in the body or the reference list; claims are in body order with ids prefix-c0001…; support names a cited source only; deterministic (BTreeMap/BTreeSet, ties keep the first sentence).

## Cost

O(text) to split; claims O(S × C × T) for S sentences, C citations each, T sentences per cited source (word sets built once per source).

## Composition

Parent: whale-workers. Used by epoch.rs (the consultant's text of an epoch) and, through `split`, by tools/topic-corpus sol.rs (draft claims → library sources).

## behavior_complement

Not done: entailment or factual checking (support is lexical overlap), resolving reference lines to library works (topic-corpus does that), reading citation styles other than bracketed numbers ([Smith 2020] or footnotes are ignored, so such a text counts as uncited). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a claim: its sentence text, cited ids and the supporting sentence with its overlap; from check's refusal: the offending numbers. The original text is not recoverable from claims (bracket contents and line breaks are dropped).

## History

### 2026-11-05 · document

#### Epoch 2026-11-05 (created)

`split` moved here from topic-corpus sol.rs unchanged (its tests there still pass); measured on fake answers of 4,500 words: 169–185 claims per paper.
