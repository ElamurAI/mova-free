# mova-nautilus-sampling

## model

# mova-nautilus-sampling

## Descriptant

The sampled Wikipedia layer (source `wikipedia-examples`): which words Nautilus asks about, in what order, and which
of a word's sampled examples the sense question shows. Words come from the large corpus's tier-1 word frequencies
(`<corpus>/tier1/words.tsv`, from the private `wikicorpus words`), examples from its sampler (`wikicorpus
examples-batch`, called as a binary, so the public crate never depends on private code), kept whole in an examples
file outside the repository. Source: `tools/nautilus/src/sampling.rs`.

## Forward approximant

words.tsv -> lemma -> {content UPOS -> occurrences} (lower-case, admissible, not junk); order: f = sum over content
UPOS, s = numbered senses of the pupils' meaning; key (group(s), -ln(1+f)*max(s,1), word bytes) with group 0 for
s >= 2, 1 for s = 1, 2 for s = 0; words of `skip` layers and f < min_count left out; the UPOS asked is the main one
when it holds 80% of f. fetch: chunks of 200 words -> one `examples-batch` process -> exactly one JSON line per word
in input order, the lemma echoed -> appended to the examples file. choose: candidates = examples 20..220 characters
using the word by Mova's lemmas, not boilerplate; base = length quality (1 within 160 characters, else 0.5; -1 for
a list) + 0.5 for a clear sense (>= 2 shared lemmas, more than the next) + 0.5/(1+i/10); greedy: + 1.5 new clear
sense, + 1 new UPOS, + 1 new article, + 0.5 new category, + 0.5 new head, - 3 * max word Jaccard, - 2 * max topic
Jaccard (content lemmas in at most 30% of the word's examples, expanded by lemmas co-occurring with them in >= 2
examples); word Jaccard >= 0.6 never chosen. row: 3 chosen examples `text «sent_id»`, the pupils' part of speech
and meaning, the sampling evidence in the sense column.

## Invariant

Deterministic: BTreeMap/BTreeSet, total_cmp on floats, ties by bytes or index; the same words.tsv, base, skip and
examples file give byte-identical rows. Nothing is appended unless the sampler answered a whole chunk. The examples
file is streamed, never held whole.

## Cost

order: O(W log W) over tier-1 words (783 181 keys, about 45 000 candidates). choose: O(n^2) set operations over at
most 100 examples a word plus O(n * L^2) co-occurrence pairs (L content lemmas per example). fetch: about 0.06 s a
word on 4 threads (200 words with 100 examples each in 12 s). The examples file is about 35 kB a word.

## Composition

Parent: `nautilus`. Called by `animal::make` for a `wikipedia-examples` layer; its rows are read by `work::DictJob`
like any draft (`senses::plan` on the row's examples, two shown). The licence gate is `animal::LayerConfig::check`
(CC BY-SA forced, not shipped, outside any Git work tree). State: reads words.tsv, the base and skip layers; appends
the examples file; spawns the sampler.

## behavior_complement

Does not annotate, count or read Wikipedia itself (the sampler does); does not pick senses (the loop and its
teacher do); does not retry a failed sampler call or repair a short answer; does not dedupe examples across words;
never writes inside a Git work tree (checked by the caller). The per-example sense signal is weak: a clear sense
needs two shared lemmas, so most examples have none, and the variety rests on UPOS, article, topic and head.

## reverse_reconstruction

From a row: the word, its interest and group (in the sense column) and the sentence ids of the chosen examples; from
the examples file: every sampled sentence of the word (title, sentence id). Which candidates were dropped as
near-duplicates or too long is recomputed, not stored. No unique inverse is claimed.

## tests

`cargo test --release --offline --locked` with MOVA_OCTOPUS_TEST_MODEL=(see History): tools/nautilus
46 unit + 2 English tests, two identical runs. sampling::tests: the_order_puts_ambiguous_common_words_first,
the_choice_shows_different_senses_and_no_near_duplicates, topics_join_examples_of_one_sense,
rows_and_the_examples_file, a_failing_sampler_appends_nothing.

Negative controls (each red, restored): skip ignored; group order off; near-duplicate filter off; a short
sampler answer accepted; common lemmas kept as topics. Finite fixtures; no teacher or model was called; not a proof.

### Checks

## History

### 2026-11-05 · Cost

order: O(W log W) over tier-1 words (783 181 keys, about 45 000 candidates). choose: O(n^2) set operations over at
most 100 examples a word plus O(n * L^2) co-occurrence pairs (L content lemmas per example). fetch: about 0.06 s a
word on 4 threads (200 words with 100 examples each in 12 s, 2026-11-05). The examples file is about 35 kB a word.

### 2026-11-05 · document

#### Epoch 2026-11-05

First model, written with the source. Prospective prediction kept for checking: on the 200 first words, with native
picks off, every word with examples is asked (8 questions of 25); measured 2026-11-05: 200 asked, 0 native, 0 thin.
With native picks on the same rows Mova settled 52 words; the pick names one sense of examples chosen to show
several, so the deployment turns native picks off for this layer (`native_picks: false`).

### 2026-11-05 · tests

`cargo test --release --offline --locked` with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin: tools/nautilus
46 unit + 2 English tests, two identical runs (2026-11-05). sampling::tests: the_order_puts_ambiguous_common_words_first,
the_choice_shows_different_senses_and_no_near_duplicates, topics_join_examples_of_one_sense,
rows_and_the_examples_file, a_failing_sampler_appends_nothing.

Negative controls 2026-11-05 (each red, restored): skip ignored; group order off; near-duplicate filter off; a short
sampler answer accepted; common lemmas kept as topics. Finite fixtures; no teacher or model was called; not a proof.
