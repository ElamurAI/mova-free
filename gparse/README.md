# gparse — the MMM parser trained by gradient descent on the GPU

The perceptron parser in `en` learns on the CPU. `gparse` is its GPU sibling: the same input (gold tokens) and the same output (XPOS, UPOS, head, relation), but trained by gradient descent on the RTX 4000 (20 GB). It is written in Rust on candle with CUDA.

- **Input per word:** a word embedding (lowercase, words seen at least twice), hashed prefix-3 and suffix-3, word shape and position. There is no tagger in front: the network predicts the tags itself.
- **Encoder:** a pre-LN transformer, 4 layers, d 256, 8 heads, ff 1024, dropout 0.2, 23.6 M parameters. It is trained from scratch, with no pretrained weights and no foreign licences inside.
- **Heads:** the biaffine arc scorer of Dozat & Manning (2017), a bilinear relation scorer, and XPOS/UPOS classifiers. The loss is arc + relation + ½(XPOS + UPOS).
- **Decoding:** Eisner (1996), giving the best projective tree with exactly one word under the root.
- **Training:** AdamW (lr 2e-3, β2 0.98, wd 0.01), warmup over the first epoch, then a linear decay. Batches hold about 1 000 words of sentences of similar length, with word dropout 0.15. The epoch with the best dev LAS is saved. A run is deterministic for a given seed.

```
cargo build --release
GP_LR=0.002 GP_TOKENS=1000 GP_EPOCHS=30 gparse train <model dir> <train.conllu>... -- <dev.conllu>...
gparse eval <model dir> <test.conllu>...
gparse annotate <model dir> <in.conllu> <out.conllu>
```

A model directory holds `model.safetensors`, `cfg.txt`, the vocabularies (`words.txt`, `xpos.txt`, `upos.txt`, `rels.txt`) and `train.log`.

## Traps in candle 0.9 (tests guard them)

Two fused kernels pass **no gradient back**, and they fail silently:
- `candle_nn::LayerNorm`;
- `ops::softmax_last_dim`.

With them in the network, the embeddings and the encoder stayed at their random init and only the output heads learned. The tagger stalled at 66–75% and the parser at 48 UAS. The model therefore uses its own layer norm built from plain tensor ops, and `ops::softmax`. The tests `embeddings_get_gradients` and `which_ops_backprop` assert both facts. If candle ever fixes the kernels, the tests fail and the fused versions may come back.

Embedding tables start at σ 0.02. At σ 1, a word seen a few times never moves away from its random vector under Adam's small steps.

## Measurements (2026-10-15)

The test is held out by lesson (every 10th school lesson, never trained on) plus EWT test. Training runs for 30 epochs; the model is chosen by EWT dev LAS. LAS counts our own predicted tags for both parsers.

| training data | parser | school, agreed trees | school, Haiku silver | EWT test |
|---|---|---|---|---|
| EWT + ESLSpok | `en` perceptron (CPU) | 80.52 | 84.59 | 79.97 |
| EWT + ESLSpok | **gparse** (GPU) | 74.92 | 73.25 | 82.49 |
| + school, Haiku silver | `en` perceptron | 83.13 | 82.85 | 80.16 |
| + school, Haiku silver | **gparse** | 86.44 | 84.40 | 85.04 |
| + school, agreed | `en` perceptron | 84.54 | 82.41 | 80.65 |
| + school, agreed | **gparse** | **88.24** | 83.88 | **85.57** |

- The GPU parser trained on the agreed school is the best MMM parser so far: +3.7 LAS on the school and +4.9 on EWT over the perceptron trained on the same data. Its XPOS is 97.5 on the school.
- Without school data the perceptron fits the Haiku silver better (84.6 vs 73.3), because that silver started from its own drafts.
- Speed: one epoch on 54 k sentences takes 58 s. Parsing runs at 140–170 k words/s on the GPU.

Kept on disk in `$MOVA_DATA/runs/mmm-parse-2026-10-15/` (`log/`, `gp-*`).

## Pragmatics read back (15.10)

`prag roles` turns the reader's frames into (dependent, predicate, role) rows. `gparse` reads them as input: a role/predicate embedding is added to each token, and a learned score per role is added to the arc it links. Training gets the roles of the gold trees (`GP_PRAG_TRAIN`), hidden for half the sentences (`GP_PRAG_DROP`) so the first pass works without them. At inference `GP_PRAG` gives the roles of the previous pass. Old models load with the new weights fresh (`load_vars`).

The loop: parse → `prag roles` → parse with roles → …. On the held-out sets (`$MOVA_DATA/runs/prag-loop-2026-10-15`) it converges in 2–3 passes. Changed words per pass on the school test: 755, 47, then 30 oscillating. The gain is nearly zero: school LAS 88.43 → 88.48, EWT 85.68 → 85.69. The roles are still derived from the same tree, so they carry little new information. They will matter once the reader adds what the tree lacks (VerbNet classes, animacy, coreference, idioms). The limit is 3 passes.

With the discourse state shared over whole documents (`prag roles` reads a lesson, or a whole book, in order; 8% of the role rows change, mostly pronouns resolved from earlier sentences), the loop gives 88.38 → 88.42 on the school test. Still nothing. As a parser feature, the reader's roles restate the tree.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
