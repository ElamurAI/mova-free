# Yom HaDin — the Last Judgment

> **Prototype.** This is an experiment beside Mova, not part of its core yet: it works, but its interfaces and data may change at any time.

> **The machine does not sentence. It makes arbitrary sentencing harder to hide.**

Yom HaDin is not a court but an anti-court: it judges no one; it shows how every court could judge, and where a verdict leaves the space the law allows. It is an open, deterministic study of legal reasoning and a second opinion. It is not a court and not legal advice.

Crate `yomhadin`. The small model (the SLM) does all the reasoning: laws are read out of texts by Mova's own stack (en parser → prag frames → world), and a large model is never asked, neither to read the laws nor to judge.

## What it does

1. **Reads laws out of texts as rules.** Every sentence of a source goes through the parser (or its agreed tree from the parse database) into prag frames. A frame with a deontic marker becomes a rule:
   - **force**: obligation («shall», «must», «is obligated»), prohibition («shall not», «may not», «is forbidden»), permission («may», «is permitted»), liability («shall pay», «shall be put to death», «is liable», «is guilty»), exemption («is exempt», «there shall be no bloodguiltiness for him», «no claim can be made»);
   - the **act** (the frame under it, or the frame itself: «thou shalt not steal»);
   - the **conditions** («if», «when»: prag's `cond` and irrealis clauses, with «or» kept as alternatives);
   - the **exceptions** («unless», «except», and a following «But if …» sentence on the same verse);
   - the **penalty** (the sanction as said: «pay five oxen and four sheep for ox», «be put to death»);
   - the **authority**: a «Rebbi X said» frame above it, «the Sages», a verse quoted («it is written»), else the source's default («Torah», «Mishnah (anonymous)», «Halakhah (anonymous)», a code's article);
   - **disputes**: rules on the same halakhah and the same act, by different authorities, with different force or penalty.
2. **Keeps jurisdictions as data.** A jurisdiction is a folder under `jurisdictions/`: place, era, body of law, kind (modern or historical), the rules, what its terms cover, how heavy its penalties are, whose word prevails, its own precedence between conflicting rules, and its provenance. A new jurisdiction comes in as files, with no code change.
3. **Judges a case by enumerating every way to judge it.** The case is read into frames by the same stack. For each party who did something (each defendant) and each admissible jurisdiction, the rules whose act or condition covers the defendant's facts are matched, and their **choice points** are found:
   - **choice of law**: lex loci (a place the jurisdiction claims is named), personal law (a status word it claims), lex fori (any forum);
   - the **reading of a term**: a rule that covers the case only on the broad reading of a word («took» as «steal», «ox» as «property») has a point, broad or narrow;
   - each **condition** the case does not settle: met or not proven; one the case contradicts («at night» against «if the sun be risen») sets the rule aside;
   - each **exception** the case does not settle: applies or not;
   - each **dispute**: whose word prevails;
   - **precedence**: the principle that settles conflicting rules (specific, rank, later, strict, lenient; each jurisdiction lists its own, the declared one first);
   - **suspension**, where the jurisdiction allows probation.

   A **path** is one choice at every point. Every path is counted exactly (a rule fires only when all its points hold, so paths are counted by which rules fire, times the global points), and each verdict keeps its first path with the full chain of why: every rule with its id, source and authority, what fired, what was set aside and why, which principle settled the conflict.

   The **honest path** takes the plain choice everywhere: terms as the jurisdiction reads them plainly, what the case does not say as not proven, the majority in a dispute, the declared precedence, no suspension.

## The report

`yomhadin judge "<case>"` prints, for each defendant, never a single number:

- **side by side**: per jurisdiction, the choice of law, rules matched, choice points, paths, the honest verdict and its decisive rule, the harshest and mildest severity in the lattice, the corruption sensitivity, the mice to acquittal, and the flagged rules among those matched;
- **(a) the forum**: the admissible space under the law of the chosen jurisdiction (`--forum`, or the place named in the case): every verdict with its count of paths, and the honest chain;
- **(b) the modern reference**: the honest verdicts of the modern jurisdictions on one scale, with the weights printed, the modal class, the median and the spread; every row traces to its rule, source and provenance;
- **(c) historical and religious traditions**: the same for the historical and religious jurisdictions, weighted apart and never mixed into (b): counterfactual judges and a source of arguments, not votes;
- **(d) slices by principle**: in each jurisdiction, the plausible verdict (within `--depth` tipped points) that best fits proportionality (nearest the harm the case tells of), harm, intent, restitution, rehabilitation;
- **(e) the strictest and the most lenient plausible positions**, modern and historical;
- **(f) the facts that flip the conclusion**: one fact proven or left unproven that changes an honest verdict;
- **(g) the value-choice points**: readings, disputes, precedence and suspension, where a judge must choose and the law does not decide, with what each option gives;
- **symmetric goals**: convict, acquit, reduce, reclassify (the same facts read as another offence or none), and elsewhere (each jurisdiction's honest verdict); for each, the paths ranked by the number of contested points, the rule they rest on, and what must be established or argued; a goal no path reaches within the depth is reported as unreachable, not forced;
- **the corrupt judge counterfactual** (`--lens corrupt`, alias `bribed`): how much the outcome rests on an honest procedure. From the honest path, the fewest points a judge must tip to reach acquittal, a suspended sentence, or the harshest verdict, and which points they are; the bribe is counted in mice, one 🐭 per point («with mice as a bribe, the snake finds the probation clause and gives a suspended sentence»). The **sensitivity** is the share of the lattice's other verdicts reachable by tipping one point: 0 when there is nothing to tip; at or above 0.5 the jurisdiction's verdict is fragile here. It is a measure of where discretion lives, not advice;
- **a given verdict** (`--given S`, a severity on the shared scale): its percentile against the modern reference, whether it is atypical, the assumptions one must accept to reach it in the forum (or each modern jurisdiction), and the alternative chains that lead elsewhere; a verdict no path reaches is «outside the space this law allows here».

- **legal fictions** (`--lens fiction`, also in the full report when a device is in the case) and **absurdity** (`--lens absurd`): see below.

Detail lenses: `paths` (every verdict with its count and first chain), `honest` (alias `majority`), `strict` (the harshest reading, forgetting the others), `lenient`, `loopholes` (one point from the honest path to acquittal), or `detail` for all of them.

**The shared scale of penalties** (`_common/penalties.tsv`, ordinal, a convention printed with every reference): 0 none, 1 restitution or a duty, 2 a fine or a single payment (also a prohibition with no sanction stated), 3 a fine as a multiple of the harm, 4 corporal, 5 liberty, 6 exclusion (karet, «cut off»), 7 capital. The heaviest pattern found in a penalty sets its rank; each jurisdiction may override the patterns.

**Flags** (`_common/flags.tsv`). A rule is flagged when a word of its text names a status that sets a party apart (slave or free, gender, religion or people, foreigner, social class), or when its penalty is one the main human-rights instruments forbid or work to abolish: corporal and mutilating punishment (ICCPR art. 7, the Convention against Torture), cruel modes of execution, and the death penalty (the ICCPR's Second Optional Protocol, ECHR Protocol 13). Flagged rules stay in their jurisdiction's own column (marked ⚑) and are left out of the references by default; `--flagged` includes them. The flags are words, not judgments: they show where to look and err on the side of flagging.

**Weights** are equal per jurisdiction by default and printed in every reference; `--weights ukraine=2,us-federal=0.5` changes them.

**Protocols.** Every `judge` run writes its protocol to `$MOVA_DATA/runs/yom-hadin/protocols/<fnv64 of the case and options>.txt` (`--protocol DIR` elsewhere, `--no-protocol` none): the case, the options and weights, each jurisdiction's rules-file fingerprint and provenance, the full report, and every lattice's points and counts. It holds no time and no machine path, so the same input gives the same protocol byte for byte.

## Legal fictions (`--lens fiction`)

A legal fiction (ha'aramah) is a verdict or a permission that rests on a formal device rather than on the substance of the case. Here a device is a rule of its own kind (`devices.tsv`): it changes the classification of a place, a thing, a transaction or an agent without changing the facts. When every trigger word of a device is in the case, the lens judges the case twice, with the device and without it, side by side; in the chain with the device, ⟐ marks each step whose standing (fires, does not fire, set aside) or settling changes with the device. A case with no device gets «none».

| device | what it reclassifies | source named |
|---|---|---|
| the eruv | a place: the streets inside a wire or wall around a city become one shared private domain (a courtyard), so carrying on the Sabbath is permitted; the Manhattan eruv is one | tractate Eruvin; Mishnah Eruvin |
| the prozbul | a transaction: a debt handed to the court becomes the court's bond, which the sabbatical year does not release | Mishnah Shevi'it 10:3–4 (Hillel) |
| the sale of chametz | a thing: chametz sold to a gentile before Passover is the gentile's goods while it stays in the house | Tosefta Pesachim; Shulchan Aruch, Orach Chayim 448 |
| heter iska | a transaction: an interest-bearing loan written as a partnership, the interest as a share of profit | Bava Metzia 104b (iska); the heter iska formula of early 17th-century Krakow |
| the Sabbath gentile | an agent: the act is the gentile's, not the Jew's (asking him stays forbidden rabbinically, and that rule is kept) | Shabbat 150a; Shulchan Aruch, Orach Chayim 307 |

The laws these devices act on are written by hand beside the Talmud's rules (`jurisdictions/yerushalmi/text.tsv`, ten sentences, our plain paraphrase of the sources named in each line, three of them with the reason their source gives: carrying in the street and in a courtyard on the Sabbath, lighting a fire and asking a gentile to, demanding a debt after the seventh year and a bond of the court, keeping chametz and a gentile's goods during Passover, interest and a partner's profit), and read by `extract` like every other rule.

The eruv case, as the lens prints it (other rules left out):

```text
yerushalmi — the eruv (place; tractate Eruvin; Mishnah Eruvin)
 defendant man: with the device permitted: «carry» [0]; without it guilty: forbidden «carry» [2] — the verdict rests on the device
   with the device:
     ⟐ yerushalmi:Shabbat.1.1.hand:1 [Shabbat.1.1.hand; Mishnah Shabbat 1:1, with the reason of Exod 23:12 (paraphrase)]: does not fire — if «carry in:street»: the case does not say: not proven
     ⟐ yerushalmi:Eruvin.6.hand:1 [Eruvin.6.hand; Mishnah Eruvin (paraphrase)]: fires (permission) — act «man carry book place:courtyard on:sabbath inside:eruv» ~ «carry thing in:courtyard place:sabbath»
     ⟐ 4 rules fire with different verdicts; by «rank» yerushalmi:Eruvin.6.hand:1 prevails
     ⟐ verdict: permitted: «carry» (severity 0)
   without it:
     · yerushalmi:Shabbat.1.1.hand:1 […]: fires (prohibition) — act «man carry book place:street on:sabbath inside:eruv» ~ «carry thing in:street place:sabbath»
     · verdict: guilty: forbidden «carry» (severity 2)
```

The lens shows the geometry; the tradition has its own words for it. Nachmanides, on Leviticus 19:2 («Ye shall be holy»): one can keep every letter of the Torah and still be *naval birshut haTorah*, «a scoundrel within the bounds of the Torah»; holiness asks for more than the letter. And the principle *middah keneged middah*, «measure for measure» (Mishnah Sotah 1:7: «with the measure a man measures, it is measured to him»): as one deals, so one is dealt with. Many devices were made for the substance (Hillel made the prozbul because people stopped lending to the poor before the seventh year); the lens does not judge which. It only shows where a verdict rests on a device and what it would be without it.

## Absurdity (`--lens absurd`)

The lens shows the moments where reasoning turns absurd, by measurable criteria only. It judges paths, never people. Each flag carries its chain and sources and a score 0–4 by a fixed rule, on the scale of Mova's absurdity matrix (`docs/knowledge.md`); a path's score is its highest flag, and a case with no flag scores 0.

| criterion | when it is flagged | score |
|---|---|---|
| purpose defeated | the rule's own stated reason (read with the rule: «for ye were strangers», «because the Sabbath is a rest») is there in the case, and the path frees the defendant all the same: by a device, or by the narrow reading of a word | 3 by a device, 2 by a reading |
| disproportion | a penalty imposed on the path is 3 or more classes from the harm the case tells of (a death 7, a body hurt 4, property 3, else 1) | 3 classes 2, 4 classes 3, 5 or more 4 |
| contradiction | on the honest path, two rules of one jurisdiction give a guilty and a free verdict on the same act, and the declared principle ties: the order of the body decides | 3 |
| literalism | the honest verdict turns between free and guilty on the width of one word alone (in either direction) | 2; 3 when the verdicts are 5 or more classes apart |
| status-only | with only a party's status swapped (Jew/gentile, Israelite/stranger, slave/free, man/woman, citizen/foreigner) and the facts the same, the honest verdict changes | 3; 4 when 3 or more classes apart |

Disproportion is checked on the honest path and on every verdict a judge reaches within `--depth` tipped points. An exemption or an acquittal imposes no penalty and is not checked for disproportion. A rule's stated reason is read at extraction: a clause marked «for», «because», «since» or «lest», or «that» / «so that» with may, might or shall, is kept as the rule's reason rather than its act (`reason` column). «The case shows the reason» means a word of the reason (its verb or what it names) is covered by a fact of the case.

The heter iska case, as the lens prints it:

```text
yerushalmi / man: score 3 — 2 flags
  [3] contradiction: rules of this jurisdiction give a guilty and a free verdict on the same facts, and its declared principle ties: the order of the body decides
  [3] purpose defeated: yerushalmi:BavaMetzia.5.hand:1 [BavaMetzia.5.hand] gives its reason «because: brother live», which the case shows; heter iska keeps its letter and the verdict becomes permitted: «take» instead of guilty: forbidden «lend»
```

`measure` prints each case's highest score and number of flags per jurisdiction (`absurd` column, `score:flags`).

## Jurisdictions

| id | kind | status | body | provenance |
|---|---|---|---|---|
| `torah` | historical | filled | Exodus 21–23, Leviticus, Numbers 30 and 35, Deuteronomy, JPS 1917 | derived by `extract` (public domain text) |
| `yerushalmi` | historical | filled | Mishnah and the Jerusalem Talmud: Berakhot and Peah, Guggenheimer's English; inherits the Torah's rules; five legal devices and ten hand-written base laws they act on | derived by `extract` (CC BY, see below); the base laws hand-written |
| `hammurabi` | historical | sample | 20 laws, our paraphrase after L. W. King's translation (1910, public domain), simplified and not quoted | hand-written text, rules by `extract` |
| `ukraine` | modern | sample | 11 provisions of the Criminal Code (2001) and Civil Code (2003) on theft, found things, killing, damage, defence; our English, simplified | hand-written text, rules by `extract` |
| `us-federal` | modern | sample | 8 provisions of 18 U.S.C. and 36 C.F.R. in plain sentences, simplified | hand-written text, rules by `extract` |
| `orthodox`, `masorti`, `reform`, `karaite` | religious | slot | the same Jewish texts as each movement reads them; each says in one line what it admits as a source and whether it admits legal devices (`admits`, `devices yes|no|moot`), which the fiction lens and forum shopping read | none yet |
| `talmud-bavli`, `twelve-tables`, `justinian`, `ruska-pravda`, `magna-carta`, `england-wales`, `eu`, `israel`, `us-state` | — | slot | metadata only, waiting for open texts | none |

The hand-written samples are marked as such in their `jurisdiction.tsv`; the official texts decide, not our summaries.

Files of a jurisdiction (`src/juris.rs`):

```text
jurisdiction.tsv  id, name, kind (modern|historical|religious), place, era, body, status, admits, devices (yes|no|moot),
                  places, persons, principles, inherits, suspend,
                  source (pool <filter> <pool> <trees> | text <file>), provenance
rules.tsv         the rules (written by `extract`): id, jur, source, sent, authority, force, penalty, acts, conds,
                  excs, parties, dispute, tree (db|slm), reason, text
terms.tsv         head, narrow, broad, plain (narrow|broad), opposite, note
penalties.tsv     pattern, rank, gloss
authorities.tsv   pattern, class, rank (lower prevails)
text.tsv          address, authority, text (for a jurisdiction written by hand)
devices.tsv       id, name, trigger, field (place|thing|transaction|agent), substance, form, source, note
```

`_common/` holds the shared terms, penalties and flags; a jurisdiction's own rows override them.

## Run

```sh
cargo build --release -p yomhadin
B=target/release/yomhadin
$B jurisdictions                                  # place, era, body, status, rules of each
MOVA_BIG=<heavy data dir> $B extract              # read the rules of every jurisdiction with sources
$B extract hammurabi ukraine us-federal           # the hand-written ones need only the en model
$B rules yerushalmi --authority rebbi --max 5     # rules by authority, in order of precedence
$B judge "A man stole an ox and sold it."         # the full layered report
$B judge "A thief broke into a house at night and the owner killed him." --lens corrupt
$B judge "In Kyiv a man took a wallet from the pocket of another man." --given 5 --lens outlier
$B judge "A man carried a book in the street of Manhattan on the Sabbath inside an eruv." --lens fiction
$B judge "A man lent money to his neighbour and took interest under a heter iska." --lens absurd
$B sample                                         # the sentences of the precision sample
$B measure                                        # rule counts, precision, lattices of the cases, two runs
cargo test --release -p yomhadin                  # the court's gates and their negative controls
```

`extract` reads the Torah and the Talmud from the pools and parse databases named in `jurisdiction.tsv`
(the Tanakh and Talmud pools under `$MOVA_BIG/runs/`, with their `.verses` maps, and the agreed trees in
`$MOVA_BIG/corpora/judaism/parsedb*/current.conllu`). Sentences without an
agreed tree are parsed by the en SLM. The extracted `rules.tsv` files are kept in the repo, so `judge` needs only the
en model (`$MOVA_DATA/en/models/en.bin`) to read the case.

## Checks

`measure` scores the extractor on a hand-checked sample (`data/gold.tsv`, Torah and Talmud sentences; `sample`
prints them) and prints the lattices of the example cases and the negative control (`data/cases.tsv`): every
case × defendant × jurisdiction with its rules, points, paths, verdicts, sensitivity, mice to acquittal and absurdity.
The gold was written by hand from the sentences alone, before the extracted rules were looked at, and it is strict: a
rule is right when its force matches and its act (or the act it is conditioned on) is one the sentence truly holds; a
promise, a curse foretold, a prayer or a rejected hypothesis holds none. Recall is the share of the gold's rules that
some extracted rule of the same sentence matches. The control scores each sentence's rules against the gold of the
next sentence; the force-and-act control is the one that goes red (the authority control stays high because the Torah
has one authority). Gold rows that do not say who speaks are not scored for authority.

`measure` also renders every case's report twice in one process and compares; the same input gives the same output and
the same `rules.tsv`, byte for byte.

`cargo test --release -p yomhadin` runs `tests/court.rs` (by hand, no model needed): absurdity flags, status-only
verdicts, legal devices, uncovered cases, path counting, term readings, sensitivity and goals, precedence, determinism,
flagged rules and the reference statistics, each with its negative control.

A chain as `judge` prints it (the honest path, Ukraine, the ox case; the rules that do not fire left out):

```text
· ukraine:CrC.185.1:1 [CrC.185.1; Criminal Code art. 185]: fires (liability) — act «man steal ox» ~ «steal property»
· ukraine:CrC.186.1:1 [CrC.186.1; Criminal Code art. 186]: fires (liability) — act «man steal ox» ~ «take property»; act «steal» as «take»: broad
· 2 rules fire with different verdicts; by «specific» ukraine:CrC.185.1:1 prevails
· verdict: liable: liable to fine or community service (severity 2)
```

and the corrupt judge on the owner in the burglary case:

```text
ukraine — sensitivity 0.50: fragile
  · acquittal: exempt — 3 🐭🐭🐭: reading «act «kill» as «harm»» of ukraine:CrC.36:1 [CrC.36]: narrow → broad; reading «if «harm place:defence»: «kill» as «harm» (broad reading)» of ukraine:CrC.36:1 [CrC.36]: narrow → broad; precedence «conflicting rules settled by»: specific → later
  · suspended: liable: be punished by imprisonment — suspended — 1 🐭: suspension «suspend a sentence up to severity 5 (Criminal Code art. 75: release on probation for imprisonment up to five years)»: no → suspend
  · harshest: unreachable within 3 points
```

## Limits

- The extractor reads every «shall» of the Torah as a rule, so ritual duties (the priest shall take, shall burn) outnumber the laws of damages; promises and curses («ye shall perish») come out as obligations.
- The absurdity criteria are mechanical: the harm scale has four steps (death, body, property, other), a statutory maximum stands for the penalty («a fine or imprisonment» counts as liberty), and a reason counts as shown when one of its words is covered by a fact.
- The Talmud is the hardest: questions are skipped, but hypotheses («one might think»), quotations of verses inside an argument and narrative imperatives become rules; the speaker of «he said to him» is resolved only within one segment.
- Penalties are read from the frame and its obliques: «restraint of liberty» loses «of liberty», «a fine or community service or correctional labour …» keeps only the first two conjuncts. Amounts and terms of years are not compared, only the class on the shared scale.
- Matching is by role lemmas and the terms dictionaries, which are small and hand-written; a word the dictionary does not know covers only itself. A case is read by the en SLM, which can misread it; the facts it read are printed first.
- At most 16 rules per jurisdiction and defendant enter the lattice (those that speak to most of the defendant's conduct, sanctions and exemptions first); the rest are counted as left out. The goal search tips at most `--depth` points (3).
- Intent, value and time thresholds («worth more than 1 000 dollars», «within six months») are not read as quantities; they become conditions the case may or may not prove.
- The samples of Hammurabi, Ukraine and the US are small and simplified; the slots wait for open texts.
- The Torah lattices are large, so `measure` is slow.
- A device is found by its trigger words among the case's role fillers; a case that names the device in a phrase the parser drops («with a prozbul» as a loose oblique) is not seen. The device's renaming is word for word.

## Licences and attribution

The code (`src/`, `tests/`): the Elamur Cosmic License (`LICENSE.md` at the repo root). The data (`jurisdictions/`, `data/`): CC BY-SA 4.0 with its sources cited in [LICENSE.md](LICENSE.md). The Torah: The Holy Scriptures, Jewish Publication Society 1917, public domain. The Jerusalem Talmud: Heinrich W. Guggenheimer, *The Jerusalem Talmud* (Berlin, De Gruyter, 1999–2015), English translation via the Sefaria export, which gives its licence as CC BY: the rules in `jurisdictions/yerushalmi/rules.tsv` quote it, with this attribution. Hammurabi: our paraphrase after L. W. King's translation (1910, public domain). Ukraine: our English of official texts, which Ukrainian law leaves outside copyright. United States: our plain sentences after the U.S. Code and the Code of Federal Regulations, works of the U.S. Government. No NC, research-only or no-derivatives text is built in.

The Elamur team
