# GUM — XPOS without the EWT web tags: `ADD`, `NFP`, `AFX`

**Gist.** EWT tags XPOS following the LDC Webtext addendum (Mott et al. 2012), which added these tags to PTB:
- `ADD` — addresses and e-mail;
- `NFP` — emoticons and decorative punctuation;
- `AFX` — a detached prefix;
- `GW` — part of a split word.

GUM tags following classic PTB with the BioMedical addendum: `HYPH` is present, but there is not a single `ADD`, `NFP`, `AFX`. `GW` is present (24). The same tokens get ordinary PTB tags in GUM.

**Conditions and exceptions.**
- URLs and e-mail: EWT — `ADD` (475 in total, 364 of them URL/e-mail by form), GUM — `NNP` (33).
- Emoticons *:) :( ;-)*: EWT — `NFP` (96), GUM — `SYM` (1).
- Ellipsis *...*: EWT — `,` (235) or `.` (95), GUM — `:` (94).
- Prefixes before a hyphen (*post-, non-, pre-*): EWT — `AFX` (75: *over* 12, *mid* 10, *post* 4, *non* 4), GUM — `NN`/`NNP`/`VB`/`IN` (*post* 11 NN, 4 NNP, 3 VB).
- UPOS mostly agrees: URL — PROPN, emoticon — SYM. The difference is only in XPOS.
- GENTLE and GUMReddit (the same pipeline) also lack `ADD`, `NFP`, `AFX`. PUD lacks `ADD`, `NFP`, `LS`.

**Examples.** EWT *spahnn@hnks.com*: `ADD`, PROPN. In GUM a URL like *http://…*: `NNP`, PROPN.

**In UD.** For `mova` XPOS are EWT tags, so GUM → `mova` should restore `ADD`, `NFP`, `AFX`. `ADD` can be set by form: `://`, `www.`, `@` and a dot. For `NFP` there is a list of emoticons. `AFX` — before `HYPH`, for a closed list of prefixes. For the tagger this matters little: GUM has dozens of such tokens.

**Sources.** Digest, §1.4 (Webtext addendum: GW, NFP, ADD, AFX) and §1.1 (BioMedical addendum: HYPH, AFX); https://universaldependencies.org/en/pos/X.html (ADD, AFX, GW → X), `pos/SYM.md` (NFP → SYM), `pos/PUNCT.md`; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 4 (GUM v2.8: HYPH, merging `-LSB-`/`-RSB-` with `-LRB-`/`-RRB-`).
