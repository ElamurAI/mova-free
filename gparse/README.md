# gparse — the SLM parser trained by gradient descent on the GPU

The perceptron parser in `en` learns on the CPU. `gparse` is its GPU sibling: the same input (gold tokens) and the same output (XPOS, UPOS, head, relation), but trained by gradient descent on a CUDA GPU. It is written in Rust on candle with CUDA. How it sits beside the other parsers: [docs/parsers.md](../docs/parsers.md); GPU stages and reference switches: [docs/gpu-exec.md](../docs/gpu-exec.md#stages).

- **Input per word:** a word embedding (lowercase, words seen at least twice), hashed prefix-3 and suffix-3, word shape and position; with `GP_SUBWORD=1`, also the mean of the word's hashed character n-grams (see [Open-vocabulary input](#open-vocabulary-input)). There is no tagger in front: the network predicts the tags itself.
- **Encoder:** a pre-LN transformer, 4 layers, d 256, 8 heads, ff 1024, dropout 0.2. It is trained from scratch, without pretrained weights. Its training data keeps its licence: a model trained on the UD English treebanks is CC BY-SA 4.0 ([the data and model licences](../licence/DATA.md)).
- **Heads:** the biaffine arc scorer of Dozat & Manning (2017), a bilinear relation scorer, and XPOS/UPOS classifiers. The loss is arc + relation + ½(XPOS + UPOS).
- **Decoding:** Eisner (1996), giving the best projective tree with exactly one word under the root.
- **Training:** AdamW (lr 2e-3, β2 0.98, wd 0.01), warmup over the first epoch, then a linear decay. Batches hold about 1 000 words of sentences of similar length, with word dropout 0.15. The epoch with the best dev LAS is saved. A run is deterministic for a given seed.

```
cargo build --release -p gparse    # not a default workspace member; or through -p world --features gpu
GP_LR=0.002 GP_TOKENS=1000 GP_EPOCHS=30 gparse train <model dir> <train.conllu>... -- <dev.conllu>...
gparse eval <model dir> <test.conllu>...      # overall, then by length, seen/unseen words, GUM genre
gparse annotate <model dir> <in.conllu> <out.conllu>
gparse plan <model dir> <in.conllu> <token budget>    # work plan for the cost twin, see below
```

Further environment variables (others are described in their sections below):

| variable | default | effect |
|---|---|---|
| `GP_SEED` | 42 | training seed |
| `GP_LAYERS`, `GP_DROP` | from the configuration | encoder layers and dropout of a new model |
| `GP_INIT=<model dir>` | — | continue from that model (its configuration and vocabularies; new words are unknown) |
| `GP_ALWAYS=<file,file>` | — | sentences that go into every epoch whole (a cycle round's new trees); the rest is sampled |
| `GP_SAMPLE` | 1.0 | fraction of the other training sentences each epoch sees (a fresh random subset per epoch) |
| `GP_CPU` | unset | run on the CPU instead of CUDA |
| `GP_TIMING` | unset | print prediction timings (forward, Eisner decoding, relations) |
| `GP_SCORE` | unset | `annotate` writes each sentence's tree score as a `# gp_score = …` comment |
| `GP_LEX=<corpus.conllu>` | — | `annotate` refills LEMMA and FEATS from that corpus where the tags changed and limits relations to the corpus's |
| `GP_SUBWORD`, `GP_NGRAM_BUCKETS` | 0, 2^18 | a new model with the character n-gram input and its bucket count ([below](#open-vocabulary-input)) |
| `GP_PA`, `GP_SOFT`, `GP_PA_CONTROL` | — | training on the words the agreement vote is sure of ([below](#confident-arcs)) |
| `GP_EVAL_SCRAMBLE` | unset | `eval` scrambles the letters of words the model has not seen (control of the subword input) |

A model directory holds `model.safetensors`, `cfg.txt`, the vocabularies (`words.txt`, `xpos.txt`, `upos.txt`, `rels.txt`) and `train.log`.

## Evaluation by kind of text

After the overall line of each file, `gparse eval` prints indented UAS and LAS by sentence length (1–10, 11–20, 21–40,
41+ words), for words the model has seen and has not seen (an unseen word is outside its vocabulary and reads as
`<unk>`), and by GUM genre for sentence ids `GUM_<genre>_…`. The first line stays the same, so readers of the overall
score are unaffected. GUM is non-commercial: measurement only, never training or checkpoint selection.

## Confident arcs

`cycle db-agree` with `CYCLE_VOTE_MISC=1` keeps the vote over each word's (head, relation) in MISC:
`Vote` (the share of the weight behind the chosen pair), `VoteHead` (behind the chosen head), `VoteAlt` (other heads
with their shares). Training reads them:

- `GP_PA=<share>`: a word whose head share is below the threshold leaves the arc loss, one whose pair share is below
  it leaves the relation loss. Tags keep full supervision. Words without a vote (gold treebanks, single-version trees)
  always stay, so gold data is never masked. `GP_PA=0` keeps every word and trains byte for byte as without it.
- `GP_SOFT=1`: a voted word learns the vote's distribution over heads (cross entropy against the shares) instead of
  the single winner.
- `GP_PA_CONTROL=random` (negative control): the same number of words leave each loss, drawn at random.

Partial annotation in this form follows the locally normalized arc loss: dropping a word from the loss is exactly
training on a partial tree (Zhang et al. 2017, partial annotation for dependency parsing). The training log gets a
`partial` line per epoch with the share of words each loss kept.

Limit: in a parse database whose versions are the seed and teacher rounds, most disagreement is one round against the
seed (share 2/3), so a threshold above 2/3 removes exactly the words a round corrected. The vote measures agreement
between versions, not between independent witnesses.

## Open-vocabulary input

`GP_SUBWORD=1` makes a new model read each word also as the mean of the embeddings of its character n-grams of length
3–6, with boundary marks (`<wo`, `ord>`), lowercased, hashed by FNV-1a into `GP_NGRAM_BUCKETS` buckets (default
2^18), at most 32 per word (Bojanowski et al. 2017, fastText's subword vectors). The word id, prefix-3, suffix-3 and
shape inputs stay. Word dropout hides only the word id, so the model learns to read a word it does not know from its
spelling; a word outside the vocabulary of a continued model (`GP_INIT`) still has a vector.

The setting is saved in `cfg.txt` (`ngram=…`; absent for the reference input) and a continued model keeps it; asking
`GP_SUBWORD` for a continued model with the other input is an error. A subword model needs a training run from
scratch and a `gparse` that knows the setting: an older binary ignores the `ngram` line and the table, and would parse
without the n-grams. The n-gram table adds `buckets × 256` parameters (2^18: 67 M, 256 MB in fp32), and training is slower: the table's
gradient is dense and AdamW updates every bucket at every step.

`GP_SUBWORD` unset or `0` is the reference input. `GP_EVAL_SCRAMBLE=1` is its negative control: an unseen word's
letters are put in a fixed pseudo-random order at evaluation, so the subword model's gain on unseen words must
shrink.

## Traps in candle 0.9 (tests guard them)

Two fused kernels pass **no gradient back**, and they fail silently when used for training:
- `candle_nn::LayerNorm`;
- `ops::softmax_last_dim`.

With them in the network, the embeddings and the encoder stay at their random init and only the output heads learn. The training path therefore uses its own layer norm built from plain tensor ops. CUDA softmax uses a wrapper with an analytical backward pass, described below; `ops::softmax` remains the reference. The tests `embeddings_get_gradients` and `which_ops_backprop` assert both facts. The no-backward operations are explicit in Candle 0.9.2; if their contract changes, these tests require review. The upstream no-backward entry point is used only for inference. The training wrapper has separate gradient tests.

Embedding tables start at σ 0.02. At σ 1, a word seen a few times never moves away from its random vector under Adam's small steps.

## Pragmatics read back

`prag roles` turns the reader's frames into (dependent, predicate, role) rows. `gparse` reads them as input: a role/predicate embedding is added to each token, and a learned score per role is added to the arc it links. Training gets the roles of the gold trees (`GP_PRAG_TRAIN`), hidden for half the sentences (`GP_PRAG_DROP`) so the first pass works without them. At inference `GP_PRAG` gives the roles of the previous pass. A model saved without these weights loads with them fresh (`load_vars`).

The loop: parse → `prag roles` → parse with roles → …, at most 3 passes. `prag roles` can also read a lesson or a whole book in order, so the discourse state (for example resolved pronouns) is shared across sentences. Limit: the roles are derived from the same tree, so they add information only where the reader adds what the tree lacks (VerbNet classes, animacy, coreference, idioms).

## Fused inference softmax

CUDA prediction selects Candle's fused row softmax by default. `GP_FUSED_SOFTMAX=0` selects the
reference operations; CPU prediction uses the reference by default.
`Net::forward` keeps differentiable attention during training (the analytical
backward wrapper below); `forward_inference` is the inference-only entry point.

The attention equation is from Vaswani et al. (2017), section 3.2.1,
[Attention Is All You Need](https://arxiv.org/abs/1706.03762).
The optimization reuses [Candle 0.9.2's implementation](https://github.com/huggingface/candle/blob/0.9.2/candle-nn/src/ops.rs):
`SoftmaxLastDim::cuda_fwd` evaluates row softmax without the separate intermediate
tensors of max/subtract/exp/sum/divide. This is kernel fusion; QK and AV remain
ordinary matrix products, and it is not a new attention algorithm. The kernel uses `apply_op1_no_bwd`, so it must
not replace training softmax without supplying a backward pass.

Because floating-point rounding differs, the fused path is an approximant. The agreement probe `tests/inference_softmax.rs` compares both paths (timings, trees, tags, encoder and arc
differences). Reproduce with external data:

```sh
GP_TEST_MODEL="$MOVA_DATA/en/models/gparse" \
GP_TEST_CORPUS="$MOVA_DATA/en/ud-mova-ewt/train-mova.conllu" \
GP_TEST_SENTENCES=2048 GP_BATCH=6000 \
cargo test --release -p gparse --test inference_softmax -- --ignored --nocapture --test-threads=1
cargo test --release -p gparse --test gpu_gradients -- --ignored --nocapture
```

The CUDA gradient guard checks query, key, value, embeddings, normalization and
arc weights. Its negative control demonstrates zero query/key gradients through
the fused path while value gradients still exist: testing only for any QKV
gradient would miss this error.

Empty tokenized sentences keep their output position and zero score while
being omitted from GPU batches, so root-only batches and the empty Eisner case
do not reach CUDA or the decoder; `tests/empty_sentences.rs` covers empty-only
and mixed batches on CUDA.

## Fused softmax with backward

CUDA training uses `softmax::last_dim` by default. It launches the same
Candle F32 row kernel as inference and supplies the vector-Jacobian product:

```text
p = softmax(z), g = dL/dp
dL/dz = p * (g - sum_last_dim(p * g))
```

The derivative follows the softmax Jacobian in Goodfellow, Bengio and
Courville (2016), [Deep Learning, section 6.2.2.3](https://www.deeplearningbook.org/contents/mlp.html).
The code cites this derivation and Candle's kernel ABI. Forward is fused;
backward uses ordinary tensor operations. CPU and other dtypes use the
reference implementation. `GP_TRAIN_FUSED_SOFTMAX=0` restores reference
training; `GP_FUSED_SOFTMAX=0` separately restores reference prediction.
Layer normalization retains its existing differentiable implementation.

The CUDA check compares probabilities and gradients with reference autodiff
at widths 1, 3, 17, 33 and 257, and independently checks gradients using f64
central finite differences (step 1e-4, absolute tolerance 2e-6). It covers
offset slices, transposes, large logits and masked columns. The full-network
guard checks query, key and value gradients separately. The upstream
no-backward softmax remains the negative control.

The training-step probe `tests/training_speed.rs` compares the two paths on a fixed batch (same weights and seed,
warm-up, alternating trials). Reproduce it with `GP_TEST_MODEL` and `GP_TEST_CORPUS`
pointing to the external model and training corpus:

```sh
cargo test --release -p gparse --test softmax_backward -- --include-ignored --nocapture
cargo test --release -p gparse --test training_speed -- --ignored --nocapture --test-threads=1
GP_TRAIN_TEST_SENTENCES=128 cargo test --release -p gparse --test training_speed -- --ignored --nocapture --test-threads=1
```

## GELU approximation option

`GP_GELU=tanh` selects the approximation
`0.5*x*(1+tanh(sqrt(2/pi)*(x+0.044715*x^3)))` from Hendrycks & Gimpel,
[Gaussian Error Linear Units (2016), section 2](https://arxiv.org/abs/1606.08415).
Candle supplies both forward and backward. `GP_GELU=erf` selects the exact
`0.5*x*(1+erf(x/sqrt(2)))`. Without an override, the saved model's `gelu`
setting is used; old configurations without this field retain `erf`.
Training saves the effective selection in `cfg.txt`, so reloads preserve it.
An unknown saved activation or override is an error.

The option is experimental: **erf is the default**. `tests/gelu_paths.rs` compares
prediction agreement and timing of both activations, and checks the tanh
derivative against a finite difference; `GP_TRAIN_TEST_COMPARE=gelu` compares
training steps.

`formal/Activation.lean` proves the generic gated-activation derivative and a
conditional error-transfer bound; `formal/Softmax.lean` proves the real-valued
softmax derivative and dense-to-reduced reverse-mode formula. See
[formal/README.md](../formal/README.md#neural-mathematics) for the
scope, commands and false-formula controls.

With `GP_TEST_MODEL` and `GP_TEST_CORPUS` set to external training resources:

```sh
cargo test --release -p gparse --test gelu_paths -- --include-ignored --nocapture --test-threads=1
GP_TRAIN_TEST_COMPARE=gelu GP_TRAIN_TEST_SENTENCES=128 cargo test --release -p gparse --test training_speed -- --ignored --nocapture --test-threads=1
```

## Work plan for the cost twin

`gparse plan MODEL_DIR CORPUS.conllu TOKEN_BUDGET` emits JSON with eligible
sentences/words, skipped sentences, batch count, padded tokens, attention cells
and decoding length cubes. It reuses `data::plan_of`, honors the model's maximum
length and skips empty sentences. It does not load weights or initialize CUDA.
These are inference work features, not training or elapsed-time measurements.
The [updating cost twin](../docs/training.md#the-cost-twin)
fits and checks their relationship to actual process time.


## Periodic development evaluation

For fast multi-epoch experiments, set `GP_DEV_EVERY=5 GP_DEV_QUICK=256`.
Intervening epochs evaluate a fixed, evenly spaced subset of each development
file; every fifth and the final epoch evaluate the full files. Only full
scores may select a checkpoint, and proxy scores are labeled in the log.
Both settings must be positive. The reference default is `GP_DEV_EVERY=1`.
Training updates and their random sequence are unchanged. Training with no development files retains its existing
save behavior.

The interval can miss an intermediate best checkpoint; use full checks to
confirm a candidate. See [fast probes and full runs](../docs/training.md#fast-probes-and-full-runs).

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
