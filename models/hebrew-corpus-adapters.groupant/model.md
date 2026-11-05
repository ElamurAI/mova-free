# hebrew-corpus-adapters

## descriptant

### hebrew-corpus-adapters: wave31

Real joins: cue form/name->features->count/logp columns; tokenize byte spans->segments->talmud slices; MACULA Node0-based -> head1-based -> surface permutation -> emitted-row head mapping; macula html_text->tanakh NFC edition; read_sentence open_after->next open_before within book. Counterexamples: copying MACULA ordinal to Tanakh order misaligns books; treating Segment.end as talmud slice end loses punctuation; union of component complements is not complement of composed relation. L/M are tagged siblings only.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave31 report. Registration is not a universal proof or whole-system runtime claim.

## model

# hebrew-corpus-adapters

## forward

Tagged product L+M: L performs tokenize->features(cues)->Counts/Model->Segmenter, corpus::clean->talmud::original and Segmenter->talmud::segments. M performs XML->Sent->tree->check->surface_order->conllu; main shares library consonants/keys and dispatches prag/tanakh. Prag consumes supplied10-column rows plus prior speech state. Tanakh edition composes library html_text with NFC. No runtime L->M call asserted.

## invariant

Preserve UTF-8 byte coordinates in langseg until explicit text copying, column order HEB,1-based MACULA head transport through both permutations/row expansion, sparse1-based verse coordinates and tagged book-code identity. Both model families are lossy: labels/text cannot recover erased markup/groupings. Valid nonempty row/tree, finite signed scores and finite nonnegative switch costs are explicit.

## cost

Add only reached sibling paths; do not sum independent commands or overlapping fragments as serial stages. Langseg tokenizer O(B²) worst-case unmatched '<', ordinary bounded markup O(B), scoring O(F log V), Viterbi O(nK),K8. Corpus cleaner bounded60 lookahead O(B), registry quota can rescan O(rows*E). Macula repeated scans/ancestor walks O(n²) and depth-dependent subtree traversal; joins O(R log V), output/storage proportional to bytes. No end-to-end timing or speedup inferred.

## composition

Real joins: cue form/name->features->count/logp columns; tokenize byte spans->segments->talmud slices; MACULA Node0-based -> head1-based -> surface permutation -> emitted-row head mapping; macula html_text->tanakh NFC edition; read_sentence open_after->next open_before within book. Counterexamples: copying MACULA ordinal to Tanakh order misaligns books; treating Segment.end as talmud slice end loses punctuation; union of component complements is not complement of composed relation. L/M are tagged siblings only.

## domain

Small synthetic strings/XML/arrays and tempfile model roundtrip under dedicated temp root. Scoring requires representable counts, column bounds, finite alpha>0 and nonnegative switch costs for objective model; rows nonempty10 fields, heads valid for tree/CoNLL-U helpers. Raw adversarial input outside those premises is not certified.

## limitations

Source inspection covers CLI effects, but actual live corpus, teacher, frozen assessments, production DB/model data, downloads and lesson workflows excluded. No parser completeness, language accuracy or universal safety inferred from finite checks. Existing audit-before-guard and transitive-invalid-head caveats retained.

## checks

- 30 exact native test names twice debug/twice release with identical sorted outcomes; native-commands.json
- 6 compiled source-behavior mutants red, exact byte/hash restore, focused green; mutation-results.json
- 10 syntax projections generated twice and byte compared
- 10 source leaves,10 fragments,120 typed views with explicit cost components

## catalog

## behavior-complement

Under admitted states, L cannot emit neutral-only tokens or invalid path columns, M cannot choose Aramaic on majority tie, PD edition cannot admit wrong licence. The sibling universe is tagged transitions; neither path implies execution/effects of the other. These are partial source-derived exclusions, not exhaustive complements.

## reverse-reconstruction

Reverse each actual join by retaining compatible intermediates: segment label fibers of feature/count/score states; CoNLL-U fibers of group trees and permutations; cleaned verse fibers of markup and sparse input coordinates. Keep tag L/M and book code; dropped metadata and source order are ambiguous. Unique inverse only when original permutation/data retained.

## byte-coordinate

Token boundaries are UTF-8 source offsets; Segmenter spans last token end but talmud slices to next start/text length. Punctuation stays preceding segment; no leading-neutral recovery law.

## dependency-coordinate

Old word head h maps0->0 else p(h-1)+1, then emitted-row head first[h-1]+1. Valid heads and shared word sequence are required; interpreting Node0-based ids as heads is invalid.

## book-code

MACULA library and Tanakh order differ, so cross-reader identity is book code plus chapter/verse, not integer ordinal. Missing/invalid coordinates map0 and unknown code39, creating noninjective fibers.

## speech-carry

Prag read_sentence(row,open,rules) produces next open; quote recognition precedes closure/carry/bare. Book boundary resets externally. Langseg has no caller relationship to this speech state.

## effect-boundary

Model save truncates before rows; CLI talmud writes original/English before optional map read; converter writes accumulated books/maps separately. No atomic rollback or no-read claim for audit guard. These effect traces are source-modeled, not executed.
