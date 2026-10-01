# Retrain the English tagger and parser

## Gist

`ai/en` is Mova's English annotator. It produces full UD output: tokens, PTB tag (XPOS), UPOS,
FEATS, lemma, head and relation. The learned parts are an averaged-perceptron PTB tagger, a
transition-based (arc-standard) parser with a beam, and back-off tables for UPOS and FEATS. They
train from CoNLL-U treebanks in seconds to about a minute on one CPU core. The result is one binary
model file. Training is deterministic: the same files give the same model.

Before training, a **license gate** checks that every training file comes from a source whose
license allows commercial use. NC or unknown sources stop `en train`.

## When to use

- You have an English UD treebank of your own (or a domain sample) and want a model that knows it.
- You changed the dialect converters or the dictionary and need a fresh model.
- A model fails to load with "retrain the model". The format version, the enum sizes or the
  built-in dictionary changed.

To experiment without saving a model, use the `*-eval` commands (below). They train in memory and
have no license gate. Use them for research data such as NC treebanks.

## What the trainer needs

- **CoNLL-U, 10 tab-separated columns.** Lines `# sent_id = ...` and `# text = ...` are kept. Other
  comments, multiword-token ranges (`3-4`) and empty nodes (`5.1`) are skipped.
- **A PTB tag in XPOS on every token.** The tag set is the EWT set (`NN`, `VBZ`, `HYPH`, `NFP`,
  `ADD`, ...). A sentence with even one token without a known PTB tag is **dropped silently**.
  Treebanks without PTB XPOS give the trainer nothing.
- **Known relations in DEPREL.** These are the UD 2.18 English relations listed in
  `ai/en/src/gram.rs` (`Rel`). An unknown subtype falls back to its base (`det:poss` -> `det`). An
  unknown base relation is a read error with the file and line number.
- **English.** The dictionary compiled into the binary (`ai/en/data/lemmas.tsv`, `forms.tsv`) is
  English.
- Ideally the data is in Mova's dialect, `mova`. See [dialects-and-converters.md](dialects-and-converters.md).

Public UD treebanks come from the Universal Dependencies release (universaldependencies.org). The
English treebanks are named `UD_English-*` and their files `en_<name>-ud-{train,dev,test}.conllu`.

## Steps

1. **Name the file after its source.** The gate reads the source from the file name. It strips a
   leading `en_` and then the suffixes `-ud-train`, `-ud-dev`, `-ud-test`, `-train`, `-dev`, `-test`:

   | file | source |
   | --- | --- |
   | `en_ewt-ud-train.conllu` | `ewt` |
   | `gum-train.conllu` (converted to `mova`) | `gum` |
   | `en_mytb-ud-train.conllu` | `mytb` |
   | `train.conllu` | `train` (unknown, so training stops) |

   A bare `train.conllu` is rejected. Keep the source name in the file name.

2. **Register the license** in `ai/en/data/train-licenses.tsv`. Use one line per source, three
   tab-separated columns:

   ```
   source<TAB>license<TAB>commercial
   mytb	CC BY 4.0	yes
   ```

   - `source`: the name from step 1.
   - `license`: free text for people, e.g. `CC BY-SA 4.0`.
   - `commercial`: commercial use allowed. Only the exact word **`yes`** opens the gate.
     Anything else (`no`, `Yes`, empty) counts as "no".
   - Lines starting with `#` and blank lines are ignored. Extra columns are ignored.
   - The file is compiled into the binary (`include_str!`), so the next `cargo` build picks up the
     change.
   - Be honest here. The gate checks names, it does not enforce anything. It exists so that a
     commercially usable model is never trained on NC data by accident. Treebanks under CC BY-NC-SA
     (GUM, GENTLE, LinES, ParTUT) are registered with `no` on purpose.

3. **Measure first (optional, no gate).** This trains in memory and scores against gold tokens
   with CoNLL 2018 metrics (UPOS, XPOS, UFeats, Lemmas, UAS, LAS, CLAS, MLAS, BLEX):

   ```sh
   cargo run --release --manifest-path ai/en/Cargo.toml -- \
     ud-eval data/my-treebank/en_mytb-ud-train.conllu -- data/my-treebank/en_mytb-ud-test.conllu
   ```

   Set `EN_PRED_DIR=<dir>` to also write the predicted CoNLL-U files. Narrower evals with the same
   argument shape (`<train>... -- <test>...`): `tok-eval`, `tag-eval`, `morph-eval`, `parse-eval`,
   `lin-eval`, `rt-eval`, `beam-curve`.

4. **Train and save the model.** Training files go before `--`. After `--` comes the model path,
   then optional check files:

   ```sh
   cargo run --release --manifest-path ai/en/Cargo.toml -- \
     train data/ud/en_ewt-ud-train.conllu data/my-treebank/en_mytb-ud-train.conllu \
     -- models/en-mytb.bin data/my-treebank/en_mytb-ud-dev.conllu
   ```

   - All training files are gated, and the model learns from all of them together.
   - Check files are not used for training and are not gated. When given, the saved model is loaded
     back and must annotate them token for token the same as the in-memory model. Otherwise the
     command fails.
   - `EN_BEAM` sets the parser beam. The default is 8, with 12 epochs. `EN_BEAM=1` trains a greedy
     parser with 10 epochs. It is much faster, at a lower LAS.

5. **Use the model.**

   ```sh
   cargo run --release --manifest-path ai/en/Cargo.toml -- \
     annotate models/en-mytb.bin data/my-treebank/en_mytb-ud-test.conllu notes.txt > out.conllu
   ```

   - A `.conllu` input is annotated on its gold tokens.
   - Any other file is plain text, one sentence per line, tokenized by the model.
   - Output is CoNLL-U on stdout. Load and speed figures go to stderr.
   - Other crates load `ai/en/models/ud-ewt-eslspok.bin` by default; to use another model set
     `WORLD_EN_MODEL` (`ai/world`), `MATH_EN_MODEL` (`ai/math`) or `QAGEN_EN_MODEL` (`ai/qagen`), or replace
     that file.

## The model file

- **Location:** wherever you point `en train`. No fixed directory exists. Model files are build
  artifacts, so keep them out of the source tree.
- **Format** (`ai/en/src/store.rs`): a custom binary serde encoding.
  - It is not self-describing: fields are written in order, numbers are little-endian, and lengths
    and enum variants are LEB128.
  - **Header:** the magic `MOVA-EN\0`, the format version (`u32`, currently 1), the sizes of the
    `Tag`/`Rel`/`UPos`/`Feat` enums, a fingerprint of the built-in dictionary, the body length and
    a checksum.
  - **Body:** the `Annotator`, which holds the tokenizer lexicon, tagger, parser, linearizer and
    UPOS/FEATS tables.
- **Loading is strict.** These cases are load errors, never silently different output: wrong magic,
  another format version, different enum sizes, another dictionary, a truncated or padded file, or
  a bad checksum. The fix is always to retrain.
- **Writes are atomic.** The model is written to `<path>.part` and then renamed.
- **License:** a trained model keeps the licenses of its training data.

## Example: from a fresh treebank to a model

```sh
# 1. register the source (tab-separated; "yes" = commercial use allowed)
printf 'mytb\tCC BY 4.0\tyes\n' >> ai/en/data/train-licenses.tsv

# 2. optional: bring an older or treebank-specific dialect into mova first
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  convert ai/dialects/ud-2.14 ud-2.14 mova data/my-treebank/en_mytb-ud-train.conllu \
  > data/mova/mytb-train.conllu

# 3. train on EWT + your data, check the round trip on dev
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  train data/ud/en_ewt-ud-train.conllu data/mova/mytb-train.conllu \
  -- models/en-mytb.bin data/my-treebank/en_mytb-ud-dev.conllu
```

The converted file in step 2 is named `mytb-train.conllu`, so the gate still sees the source `mytb`.

## Pitfalls

- **"source 'train' is unknown".** The file name lacks the source; rename it (step 1).
- **Zero or few sentences learned.** XPOS is missing or not PTB. Check that column 5 holds PTB tags.
- **Gate passes on `ud-eval` but not on `train`.** This is by design. Evals are for research and save
  nothing.
- **Don't train on test or dev.** Pick settings on dev and score on test once.
- **More data of a different kind helps less.** Doubling same-style data gave about +2.5 to 3 LAS.
  Out-of-domain additions gave much less, sometimes nothing. Prefer text from the domain where the
  model will run.
- **Rebuilding the dictionary invalidates every model.** `en lexicon <agid infl.txt> <train.conllu>...
  [-- <data-dir>]` regenerates `data/lemmas.tsv` and `data/forms.tsv` (relative to the current
  directory unless you pass a dir). The dictionary fingerprint changes, so old models refuse to load.
  Lemma numbers are append-only.
- **Silver data counts as data.** Annotation produced by a program or an LLM needs its own row in
  `train-licenses.tsv` (e.g. `mova-silver-en`), and the license of its source texts. Always measure on
  independent gold, never on your own silver.
