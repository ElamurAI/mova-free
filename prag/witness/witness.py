# Open pragmatic witnesses on the GPU, for the pragmatics agreer.
# usage: witness.py <in.conllu> <out.tsv>
# Per sentence (documents read in order, as prag does):
#   - two zero-shot NLI models (deberta tasksource, cross-encoder deberta): act, polarity, uncertainty — the
#     sentence is the premise, one hypothesis per class, softmax entailment over the class hypotheses;
#   - certainty (Pei & Jurgens 2021, regression head; low = hedged);
#   - Intel polite-guard (politeness), cardiffnlp irony (with the two sentences before as context).
import sys, torch
from transformers import AutoTokenizer, AutoModelForSequenceClassification

H = __import__("os").environ.get("MOVA_BIG", "/data/mova") + "/pragmatics-tools/hf/"  # $MOVA_BIG: the heavy-data directory
inp, out = sys.argv[1], sys.argv[2]
dev = "cuda"
# WITNESS_FAST=1 or MOVA_EXEC=gpu (GPU-enhanced approximant, docs/gpu-exec.md): inputs sorted by length (less padding), fp16 weights,
# larger batches; the scores move a little, the agreement with the fp32 run is measured there
_env = __import__("os").environ
FAST = _env.get("WITNESS_FAST") == "1" or _env.get("MOVA_EXEC") == "gpu"
BS = 256 if FAST else 64

def load(model_dir):
    m = AutoModelForSequenceClassification.from_pretrained(model_dir)
    if FAST:
        m = m.half()
    return m.to(dev).eval()

def run(tok, m, a, b, bs):
    """Logits for text pairs (b may be None), in input order; with FAST sorted by length for the batches."""
    order = sorted(range(len(a)), key=lambda i: len(a[i]) + (len(b[i]) if b else 0)) if FAST else list(range(len(a)))
    res = [None] * len(a)
    for k in range(0, len(order), bs):
        idx = order[k:k + bs]
        if b:
            x = tok([a[i] for i in idx], [b[i] for i in idx], truncation=True, max_length=256, padding=True, return_tensors="pt").to(dev)
        else:
            x = tok([a[i] for i in idx], truncation=True, max_length=256, padding=True, return_tensors="pt").to(dev)
        with torch.no_grad():
            lg = m(**x).logits.float().cpu()
        for j, i in enumerate(idx):
            res[i] = lg[j]
    return torch.stack(res) if res else torch.zeros(0)

sents = []  # (id, text)
sid = text = None
for line in open(inp, encoding="utf-8"):
    line = line.rstrip("\n")
    if line.startswith("# sent_id = "):
        sid = line[12:]
    elif line.startswith("# text = "):
        text = line[9:]
    elif not line and sid:
        sents.append((sid, text or ""))
        sid = text = None
if sid:
    sents.append((sid, text or ""))

def doc_key(i):
    p = i.split(":")
    if len(p) >= 4 and p[0] == "school":
        lesson = p[2]
        num = int(lesson.split("-")[0]) if lesson.split("-")[0].isdigit() else 0
        n = int(p[-1]) if p[-1].isdigit() else 0
        if p[1] == "books":
            return ("books:" + lesson.split("-")[1], num, n)
        return (p[1] + ":" + lesson, num, n)
    return (i.rsplit(":", 1)[0], 0, 0)

order = sorted(range(len(sents)), key=lambda i: doc_key(sents[i][0]))
before = {}
prev_doc, hist = None, []
for i in order:
    d = doc_key(sents[i][0])[0]
    if d != prev_doc:
        hist, prev_doc = [], d
    before[i] = " ".join(hist[-2:])
    hist.append(sents[i][1])

ACTS = {
    "narrate": "This sentence tells what happened in a story.",
    "assert": "This sentence states a fact or explains something.",
    "ask": "This sentence asks a question.",
    "request": "This sentence asks or tells someone to do something.",
    "evaluate": "This sentence gives an opinion or judgement about something.",
    "warn": "This sentence warns about a danger.",
    "express": "This sentence expresses a feeling.",
    "promise": "This sentence makes a promise.",
    "thank": "This sentence thanks someone.",
    "greet": "This sentence greets someone.",
    "apologize": "This sentence apologizes.",
}
POL = {"pos": "The speaker is positive about something.", "neg": "The speaker is negative about something.", "none": "The speaker is neutral."}
UNC = {"1": "The speaker is uncertain.", "0": "The speaker is certain."}

def nli(model_dir, groups, bs=BS):
    tok = AutoTokenizer.from_pretrained(model_dir)
    m = load(model_dir)
    lab = {v.lower(): int(k) for k, v in m.config.id2label.items()}
    ent = lab.get("entailment", 0)
    res = {g: [None] * len(sents) for g in groups}
    for g, hyps in groups.items():
        names = list(hyps)
        pairs = [(s[1], hyps[n]) for s in sents for n in names]
        scores = run(tok, m, [p for p, _ in pairs], [h for _, h in pairs], bs)[:, ent].tolist() if pairs else []
        for i in range(len(sents)):
            sc = torch.tensor(scores[i * len(names):(i + 1) * len(names)]).softmax(0)
            j = int(sc.argmax())
            res[g][i] = (names[j], float(sc[j]))
    return res

groups = {"act": ACTS, "pol": POL, "unc": UNC}
w1 = nli(H + "sileod__deberta-v3-base-tasksource-nli", groups)
w2 = nli(H + "cross-encoder__nli-deberta-v3-base", groups)

def classify(model_dir, texts, pairs=None, bs=BS, regression=False):
    tok = AutoTokenizer.from_pretrained(model_dir)
    m = load(model_dir)
    lg = run(tok, m, texts, pairs, bs)
    if regression:
        return lg[:, 0].tolist()
    out = []
    for r in lg.softmax(-1):
        out.append((m.config.id2label[int(r.argmax())], float(r.max())))
    return out

texts = [s[1] for s in sents]
cert = classify(H + "pedropei__sentence-level-certainty", texts, regression=True)
polite = classify(H + "Intel__polite-guard", texts)
irony = classify(H + "cardiffnlp__twitter-roberta-base-irony", [(before[i] + " " + texts[i]).strip() for i in range(len(sents))])

with open(out, "w", encoding="utf-8") as f:
    f.write("sent_id\tnli1_act\tnli1_act_p\tnli2_act\tnli2_act_p\tnli1_pol\tnli1_pol_p\tnli2_pol\tnli2_pol_p\tnli1_unc\tnli1_unc_p\tnli2_unc\tnli2_unc_p\tcertainty\tpolite\tpolite_p\tirony\tirony_p\n")
    for i, (sid, _) in enumerate(sents):
        row = [sid]
        row += [w1["act"][i][0], f"{w1['act'][i][1]:.2f}", w2["act"][i][0], f"{w2['act'][i][1]:.2f}"]
        row += [w1["pol"][i][0], f"{w1['pol'][i][1]:.2f}", w2["pol"][i][0], f"{w2['pol'][i][1]:.2f}"]
        row += [w1["unc"][i][0], f"{w1['unc'][i][1]:.2f}", w2["unc"][i][0], f"{w2['unc'][i][1]:.2f}"]
        row += [f"{cert[i]:.2f}", polite[i][0], f"{polite[i][1]:.2f}", irony[i][0], f"{irony[i][1]:.2f}"]
        f.write("\t".join(row) + "\n")
print(f"witness: {len(sents)} sentences → {out}", flush=True)
