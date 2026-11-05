# mova-whale-epoch

## Descriptant

The epoch driver (`mova-whale epoch CONFIG`): per document and epoch N, Mova writes a long cited text from the paper and its neighbourhood (`compose::compose_epoch`), registers it in the dossier, the Whale's own terminal writes from the given sources by path under a pinned prompt, its text is checked (citations only to given numbers, word bounds), registered (never public) and turned into claims; a per-document stop rule marks saturation. Source: `tools/whale/src/epoch.rs`.

## Forward approximant

run(c, N, only, terminal): for each document of `documents(c)` (list | batches' assignments in order | batch::candidates), in order: skip if saturated before N; final states unchanged; sol_asking -> uncertain; else register original (epoch 0), read paper and neighbourhood texts, compose, cite, gate; fail -> mova_short|mova_gates; pass -> register mova -> (live) sol_inputs (original, mova, own earlier texts newest first, nearest key texts; hard caps) -> stage -> render -> status sol_asking -> ask -> error: sol_<kind>, stop the run | answer: check -> refused: sol_rejected | register sol, claims, (topic corpus), stop rule -> done|saturated.

## Invariant

status.json is saved before every step; a question is asked at most once per (document, epoch) in a run root; a text below mova.words.min is never registered and nothing is asked for it; a consultant text with a citation outside the given numbers is never registered; generated texts are never public; one driver per run root (epoch.lock) and the terminal's holder.lock for a live run; documents are never ordered alphabetically.

## Cost

Per document: Mova reads the paper and each neighbour text once (≈ 0.5–2 s per text; 7–27 s per paper with 12 neighbours) and composes in O(sentences × clauses) with the grammar gate per kept clause; one terminal question (minutes, the model's time); claims O(S × C × T).

## Composition

Parent: whale-workers. Uses compose.rs (compose_epoch), reader.rs, gates.rs (Guard), knowledge.rs (optional knowledge), dossier.rs, claims.rs, terminal.rs (MovaTerminal, TerminalLease), batch.rs (candidates), topic.rs (sol-import/sol-claims child). Called from main.rs (`epoch`, `epoch-status`).

## behavior_complement

Not done: retrying anything; re-asking after a limit, timeout, failure or an unrecorded answer (final for that document's epoch in its run root); building the world or the dossier's site-old/web texts; parallel documents or several terminals in one run; judging the consultant's text beyond citations, words and lexical support; saturation without a terminal step. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the run root: per epoch and document the status (state, metrics, gates, refused reason, terminal error), the prompt and the given/left-out sources, the raw answer and the claims; from the dossier: each text with its inputs. The texts can be reproduced from the same inputs (Mova's text byte for byte; the consultant's not).

## History

### 2026-11-05 · Cost

Per document: Mova reads the paper and each neighbour text once (≈ 0.5–2 s per text; 7–27 s per paper with 12 neighbours, 2026-11-05) and composes in O(sentences × clauses) with the grammar gate per kept clause; one terminal question (minutes, the model's time); claims O(S × C × T).

### 2026-11-05 · document

#### Epoch 2026-11-05 (created)

Measured on five batch papers: Mova 5,993–6,000 words on four, 25 on a paper without part headings; two byte-identical runs; fake consultant 169–185 claims per paper, 97–99 % cited.

#### Epoch 2026-11-05 (world tool)

Own material comes from the world's distance-0 entry (texts other than the original, of earlier epochs) or else from the dossier; consultant inputs are `Input`s (a plain file or a text) and are staged into the terminal root whenever they are texts (always for the world store). Negative control: ignoring the distance-0 entry turns `the_world_tool_gives_dossier_and_neighbourhood` red.
