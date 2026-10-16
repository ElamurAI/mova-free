-- Mova — annotation database (DuckDB). Schema v1, 26.09.2026.
--
-- Design note: a DuckDB database for parses of fairy-tale sentences — the same trees, pragmatics and everything
-- else go there; it lives in the dev workspace; the whole Mova corpus is moved into it as well.
--
-- Idea: different annotations from different sources and different layers lie on the same sentences. Text —
-- `docs` → `sentences`; each annotation of a sentence (or document) is a separate set `annsets` with
-- provenance (layer, source, run or treebank, dialect, time, how annotated). The UD layer stores CoNLL-U
-- rows verbatim (`tokens`), so a file is exported back byte for byte. Upper layers (entities,
-- mentions, events, pragmatics, passport, summaries) are separate tables with anchors to sentences and words.
--
-- Created by `db init`; loaded by `db load` / `db corpus` / `db tales` (TSV → COPY, `sql/load.sql`).
-- Table descriptions and example queries — `db/README.md`.

CREATE TABLE schema_info (
    version  INTEGER NOT NULL,
    created  TIMESTAMP NOT NULL DEFAULT now()
);
INSERT INTO schema_info (version) VALUES (1);

-- Runs and loads: who made the annotation, with what and when. A UD treebank, a corpus file, an `annot`
-- calibration run, a fairy-tale run — each gets its own row; reloading appends files to the same one.
CREATE TABLE runs (
    run      INTEGER PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,     -- 'ud-2.18:UD_English-EWT', 'corpus:mova-silver-en', 'annot:calib-ud-patch300-2026-09-26', 'tales-2026-09-26'
    kind     TEXT NOT NULL,            -- treebank | corpus | calib | tales | file
    tool     TEXT NOT NULL,            -- what produced the annotation: 'UD 2.18', 'annot', 'annot bronze', 'db tales' …
    path     TEXT,                     -- treebank or run directory, corpus file
    params   JSON,                     -- treebank card, mode and effort, Opus calls, training files …
    started  TIMESTAMP,                -- when the annotation was made (run date or file mtime)
    loaded   TIMESTAMP NOT NULL DEFAULT now(),
    note     TEXT
);

-- Loaded CoNLL-U files: everything needed for `db export file=…` to return the same file byte for byte
-- (block lines — in `annsets.comments` and `tokens`).
CREATE TABLE files (
    file          INTEGER PRIMARY KEY,
    run           INTEGER NOT NULL REFERENCES runs (run),
    path          TEXT NOT NULL UNIQUE,  -- absolute path at load time
    name          TEXT NOT NULL,         -- short: 'UD_English-EWT/en_ewt-ud-test.conllu'
    bytes         BIGINT NOT NULL,
    sha1          TEXT NOT NULL,
    blank_before  INTEGER NOT NULL,      -- blank lines before the first sentence
    final_newline BOOLEAN NOT NULL,      -- whether the file ends with LF
    sents         INTEGER NOT NULL,      -- blocks (sentences) in the file
    rows          INTEGER NOT NULL,      -- token rows: words, multiword tokens, empty nodes
    words         INTEGER NOT NULL,      -- syntactic words (ID is an integer)
    mwt           INTEGER NOT NULL,      -- multiword tokens (ID "3-4")
    empty         INTEGER NOT NULL,      -- empty nodes (ID "8.1")
    loaded        TIMESTAMP NOT NULL DEFAULT now()
);

-- Documents: text source with title, author, language, license and genre. A treebank document is a block
-- from `# newdoc` (or the whole file), a fairy tale is a separate document, Mova corpus sentences — by id prefix
-- (`tatoeba`, `papers:<dir>`, `seeds:<seed>`).
CREATE TABLE docs (
    doc      INTEGER PRIMARY KEY,
    key      TEXT NOT NULL UNIQUE,     -- 'UD_English-GUM:en_gum-ud-dev:GUM_academic_exposure', 'gutenberg:2591:hansel-and-gretel', 'tatoeba'
    source   TEXT NOT NULL,            -- 'UD 2.18 UD_English-GUM', 'Project Gutenberg #2591', 'tatoeba' …
    title    TEXT,
    author   TEXT,
    lang     TEXT NOT NULL,            -- ISO 639: 'en'
    license  TEXT,
    genre    TEXT,
    meta     JSON                      -- the rest: `newdoc id`, `meta::*` (GUM, fairy tales), `global.*`, translator, URL, source sha1 …
);

-- Sentences: position in the document and text. One sentence — many annotation sets.
CREATE TABLE sentences (
    sid      INTEGER PRIMARY KEY,
    doc      INTEGER NOT NULL REFERENCES docs (doc),
    para     INTEGER,                  -- paragraph in the document (from 1); NULL — the source does not mark paragraphs
    ord      INTEGER NOT NULL,         -- order in the document (from 1)
    sent_id  TEXT NOT NULL,            -- the source's `# sent_id`
    text     TEXT NOT NULL,            -- `# text`
    comments JSON                      -- other sentence comments from the original source: {"s_type": "decl", "newpar id": …}
);

-- Annotation sets. A sentence set (sid) is one view of a sentence: a UD tree from a treebank, Opus's first
-- view, a Rust draft, silver… A document set (sid NULL) is for document-level layers: entities,
-- passport, summaries.
CREATE TABLE annsets (
    aid         INTEGER PRIMARY KEY,
    doc         INTEGER NOT NULL REFERENCES docs (doc),
    sid         INTEGER REFERENCES sentences (sid),
    layer       TEXT NOT NULL,          -- ud | entities | events | pragmatics | passport | summaries | …
    source      TEXT NOT NULL CHECK (source IN ('gold', 'silver', 'bronze', 'opus', 'rust', 'udpipe')),
    tool        TEXT NOT NULL,          -- who annotated: human | opus | rust | udpipe
    run         INTEGER NOT NULL REFERENCES runs (run),
    origin      TEXT NOT NULL,          -- run or treebank: 'UD_English-EWT', 'calib-ud-patch300-2026-09-26/final', 'tales-2026-09-26/en-tales-pg2591-grimm'
    dialect     TEXT NOT NULL,          -- ud-2.18 | mova | …
    created     TIMESTAMP,              -- when annotated
    method      TEXT,                   -- how annotated: `# annotation`, run mode and effort …
    gate        TEXT,                   -- `en` gates (tree, UD 2.18 registry): 'ok' or the first reason; NULL — not checked
    file        INTEGER REFERENCES files (file),
    pos         INTEGER,                -- position of the block in the file (from 0)
    blank_after INTEGER,                -- blank lines after the block
    comments    JSON                    -- block comments verbatim, an array of lines starting with "#"
);

-- CoNLL-U rows of a set: ten raw columns, as in the file, including multiword tokens (ID "3-4")
-- and empty nodes (ID "8.1"). The typed view of words — `words`.
CREATE TABLE tokens (
    aid    INTEGER NOT NULL REFERENCES annsets (aid),
    k      INTEGER NOT NULL,           -- row number in the block (from 1)
    id     TEXT NOT NULL,
    form   TEXT NOT NULL,
    lemma  TEXT NOT NULL,
    upos   TEXT NOT NULL,
    xpos   TEXT NOT NULL,
    feats  TEXT NOT NULL,
    head   TEXT NOT NULL,
    deprel TEXT NOT NULL,
    deps   TEXT NOT NULL,
    misc   TEXT NOT NULL,
    PRIMARY KEY (aid, k)
);

-- ── Upper layers (docs/context.md). Empty for now. ──────────────────────────────────────────────
-- Anchor to sentences and words: sid + base (the ud set whose word IDs are meant) + w_from..w_to (word IDs
-- inclusive). A layer set is an `annsets` row with its own layer, source, run and method.

-- Document entities (layer 4): who is who. A document-level set, layer 'entities'.
CREATE TABLE entities (
    aid      INTEGER NOT NULL REFERENCES annsets (aid),
    eid      TEXT NOT NULL,            -- 'e3' — as in the md memory graph
    etype    TEXT,                     -- person | place | organization | object | abstract | time | event | animal | plant | substance … (GUM)
    name     TEXT,                     -- canonical name
    number   TEXT,                     -- Sing | Plur
    gender   TEXT,
    person   TEXT,
    animate  BOOLEAN,
    salience DOUBLE,                   -- attention weight
    facts    JSON,                     -- what is known: [{"text": …, "anchors": [sid, …]}]
    PRIMARY KEY (aid, eid)
);

-- Entity mentions (coreference, CorefUD `Entity=`): a word span and an entity, bridging relations.
CREATE TABLE mentions (
    aid      INTEGER NOT NULL REFERENCES annsets (aid),
    mid      INTEGER NOT NULL,
    eid      TEXT NOT NULL,            -- entity from `entities` of the same set
    sid      INTEGER NOT NULL REFERENCES sentences (sid),
    base     INTEGER REFERENCES annsets (aid),
    w_from   INTEGER NOT NULL,
    w_to     INTEGER NOT NULL,
    w_head   INTEGER,                  -- head of the mention
    infstat  TEXT,                     -- new | giv | acc (GUM)
    bridge   TEXT,                     -- bridging relation: eid of which the mention is a part or attribute (GUM `Bridge=`)
    PRIMARY KEY (aid, mid)
);

-- Events (narrative layer): who did what, with whom, when, where, why; event relations.
CREATE TABLE events (
    aid      INTEGER NOT NULL REFERENCES annsets (aid),
    evid     TEXT NOT NULL,            -- 'ev7'
    sid      INTEGER REFERENCES sentences (sid),
    base     INTEGER REFERENCES annsets (aid),
    w_pred   INTEGER,                  -- event predicate
    etype    TEXT,                     -- lemma or frame: 'sign', 'Commerce_buy' …
    roles    JSON,                     -- {"agent": "e3", "patient": "e12", "time": …, "place": …, "purpose": …}
    links    JSON,                     -- [{"rel": "cause" | "sequence" | "purpose", "to": "ev5"}]
    PRIMARY KEY (aid, evid)
);

-- Utterance pragmatics: sentence type and act, what was meant, presuppositions, stance.
-- First cut (docs/context.md, 26.09.2026; crate `prag`): form → s_type, act, indirect, hedge,
-- polarity, voice, means; sets — layer 'pragmatics', source 'silver' (LLM, gates) or 'rust' (SLM).
CREATE TABLE pragmatics (
    aid         INTEGER NOT NULL REFERENCES annsets (aid),
    sid         INTEGER NOT NULL REFERENCES sentences (sid),
    base        INTEGER REFERENCES annsets (aid),
    w_from      INTEGER,               -- NULL — the whole sentence
    w_to        INTEGER,
    s_type      TEXT,                  -- form: decl | q | wh | imp | excl | frag | intj | other (GUM `s_type` — also ger, inf, sub, multiple)
    act         TEXT,                  -- assert | narrate | ask | request | offer | suggest | promise | thank | apologize | greet | agree | disagree | evaluate | express | warn | other
    act_form    TEXT,                  -- act form: 'question' for a request phrased as a question
    means       TEXT,                  -- what was meant, paraphrased; '-' — nothing beyond the literal
    presupposes JSON,                  -- ["used to smoke", …]
    stance      TEXT,                  -- hedging, politeness, evaluation, irony
    speaker     TEXT,
    addressee   TEXT,
    indirect    BOOLEAN,               -- the form is not the usual one for the act ("Can you open the window?" — q, but request)
    hedge       BOOLEAN,               -- maybe, I think, sort of, perhaps …
    polarity    TEXT,                  -- speaker's evaluation: pos | neg | none
    voice       TEXT,                  -- who speaks in the text: narrator | character | none
    trace       JSON,                  -- SLM decision trace: {"act": {"v": "request", "why": [["w1=can", 1.2], …]}, …}
    PRIMARY KEY (aid, sid)
);

-- Document passport: domain, genre, style, narrator, purpose, mode, addressee. A document-level set.
CREATE TABLE passport (
    aid       INTEGER PRIMARY KEY REFERENCES annsets (aid),
    domain    TEXT,
    genre     TEXT,
    style     TEXT,                    -- academic, conversational, literary …; register
    narrator  TEXT,                    -- first or third person, position, number of speakers
    purpose   TEXT,                    -- inform, persuade, teach, entertain …
    mode      TEXT,                    -- narration, description, explanation, argumentation, instruction, dialogue
    addressee TEXT,
    time      TEXT,                    -- the document's time anchor
    place     TEXT,
    meta      JSON,
    anchors   JSON                     -- [sid, …] — where this is visible
);

-- Summaries by level (paragraph, chapter, book, series): claims only with anchors to sentences.
CREATE TABLE summaries (
    aid      INTEGER NOT NULL REFERENCES annsets (aid),
    k        INTEGER NOT NULL,         -- order in the set
    level    TEXT NOT NULL,            -- paragraph | chapter | doc | book | series
    sid_from INTEGER REFERENCES sentences (sid),
    sid_to   INTEGER REFERENCES sentences (sid),
    text     TEXT NOT NULL,
    anchors  JSON NOT NULL,            -- [sid, …] — source sentences of the claim
    PRIMARY KEY (aid, k)
);

-- ── Views ──────────────────────────────────────────────────────────────────────────────────────

-- Syntactic words with integer IDs and a head (without multiword tokens and empty nodes).
CREATE VIEW words AS
SELECT aid, k, CAST(id AS INTEGER) AS i, form, lemma, upos, xpos, feats,
       TRY_CAST(head AS INTEGER) AS h, deprel, deps, misc
FROM tokens
WHERE id NOT LIKE '%-%' AND id NOT LIKE '%.%';

-- Sets with names: run, file, document, sentence.
CREATE VIEW sets AS
SELECT a.aid, a.layer, a.source, a.tool, a.dialect, a.origin, r.name AS run_name, r.kind AS run_kind,
       f.name AS file_name, a.pos, d.key AS doc_key, s.sid, s.sent_id, s.text, a.gate, a.method, a.created
FROM annsets a
JOIN runs r ON r.run = a.run
JOIN docs d ON d.doc = a.doc
LEFT JOIN files f ON f.file = a.file
LEFT JOIN sentences s ON s.sid = a.sid;

-- ── Migrations ─────────────────────────────────────────────────────────────────────────────────
-- `db migrate <db>` runs everything from the marker line «-- migrate:vN» to the end of the file; every step
-- is idempotent, so on a new database (`db init` runs the whole file) it changes nothing.

-- migrate:v2
-- v2 (26.09.2026): pragmatics layer, first cut (docs/context.md; crate `prag`): indirect, hedge,
-- polarity, voice, SLM decision trace; view prag_labels.
ALTER TABLE pragmatics ADD COLUMN IF NOT EXISTS indirect BOOLEAN;
ALTER TABLE pragmatics ADD COLUMN IF NOT EXISTS hedge BOOLEAN;
ALTER TABLE pragmatics ADD COLUMN IF NOT EXISTS polarity TEXT;
ALTER TABLE pragmatics ADD COLUMN IF NOT EXISTS voice TEXT;
ALTER TABLE pragmatics ADD COLUMN IF NOT EXISTS trace JSON;

-- Pragmatics with provenance: set (source, run and stage), sentence and document.
CREATE OR REPLACE VIEW prag_labels AS
SELECT p.sid, s.sent_id, d.key AS doc_key, a.source, a.tool, a.origin, r.name AS run_name,
       p.s_type AS form, p.act, p.indirect, p.hedge, p.polarity, p.voice, p.means, p.trace, s.text, p.aid
FROM pragmatics p
JOIN annsets a ON a.aid = p.aid
JOIN runs r ON r.run = a.run
JOIN sentences s ON s.sid = p.sid
JOIN docs d ON d.doc = s.doc;

INSERT INTO schema_info (version) SELECT 2 WHERE NOT EXISTS (SELECT 1 FROM schema_info WHERE version = 2);

-- migrate:v3
-- v3 (26.09.2026): LLM training questions and answers per fairy-tale paragraph (docs/context.md, "Five-year-old
-- test" — "Training questions from the LLM"; crate `qagen`, loading — `db qa`). Question types and
-- the explicit/implicit mark — as in FairytaleQA (ACL 2022). Anchors — sentences of this same paragraph.
CREATE TABLE IF NOT EXISTS qa (
    qid      INTEGER PRIMARY KEY,
    run      INTEGER NOT NULL REFERENCES runs (run),
    doc      INTEGER NOT NULL REFERENCES docs (doc),
    para     INTEGER NOT NULL,          -- document paragraph (`sentences.para`)
    k        INTEGER NOT NULL,          -- question number in the paragraph (from 1)
    question TEXT NOT NULL,
    qtype    TEXT NOT NULL CHECK (qtype IN ('character', 'setting', 'action', 'feeling', 'causal relationship',
                                            'outcome resolution', 'prediction')),
    ex_or_im TEXT NOT NULL CHECK (ex_or_im IN ('explicit', 'implicit')),  -- answer stated in the text or inferred
    answer   TEXT NOT NULL CHECK (trim(answer) <> ''),
    anchors  JSON NOT NULL,             -- [sid, …] — source sentences of the answer, only from this paragraph
    source   TEXT NOT NULL CHECK (source IN ('opus', 'gold', 'silver', 'rust')),  -- who wrote it: LLM, teachers, …
    UNIQUE (run, doc, para, k)
);

-- Questions with names: document, run.
CREATE OR REPLACE VIEW qa_pairs AS
SELECT q.qid, d.key AS doc_key, d.title, q.para, q.k, q.qtype, q.ex_or_im, q.question, q.answer, q.anchors,
       q.source, r.name AS run_name, q.run, q.doc
FROM qa q
JOIN docs d ON d.doc = q.doc
JOIN runs r ON r.run = q.run;

INSERT INTO schema_info (version) SELECT 3 WHERE NOT EXISTS (SELECT 1 FROM schema_info WHERE version = 3);

-- migrate:v4
-- v4 (26.09.2026): replacing `qa` pairs (qagen, rule v2 — an explicit answer is a verbatim span of the anchor sentence).
-- A corrected row is a new run (`db qa --replaces <old run>`) with the same document, paragraph
-- and question number; `replaces` — qid of the old row. The old one is not deleted: `replaced_by` — qid of the new one.
-- Current pairs — view `qa_current`.
ALTER TABLE qa ADD COLUMN IF NOT EXISTS replaces INTEGER;     -- qid of the row this one replaces
ALTER TABLE qa ADD COLUMN IF NOT EXISTS replaced_by INTEGER;  -- qid of the row that replaced this one; NULL — current

CREATE OR REPLACE VIEW qa_pairs AS
SELECT q.qid, d.key AS doc_key, d.title, q.para, q.k, q.qtype, q.ex_or_im, q.question, q.answer, q.anchors,
       q.source, r.name AS run_name, q.run, q.doc, q.replaces, q.replaced_by
FROM qa q
JOIN docs d ON d.doc = q.doc
JOIN runs r ON r.run = q.run;

CREATE OR REPLACE VIEW qa_current AS
SELECT * FROM qa_pairs WHERE replaced_by IS NULL;

INSERT INTO schema_info (version) SELECT 4 WHERE NOT EXISTS (SELECT 1 FROM schema_info WHERE version = 4);
