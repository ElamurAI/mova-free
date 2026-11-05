//! macula — MACULA Hebrew lowfat trees into dependency trees (operator, 17.10: «the Torah, in English and Ancient
//! Hebrew, the Jewish recension»).
//!
//!   macula <lowfat-dir> <out-dir> [BOOK,…]
//!
//! The Aramaic sentences go to `<out-dir>/arc/` (UD `arc`, ids `wlc-arc:<BOOK>:<n>`); every Aramaic word has `Lang=arc`,
//! a Hebrew word in an Aramaic sentence `Lang=hbo`. The targums and the three-language index: module `tanakh`.
//!
//! MACULA Hebrew (Clear Bible / Biblica, CC BY 4.0: "MACULA Hebrew Linguistic Datasets, available at
//! https://github.com/Clear-Bible/macula-hebrew/") gives the Westminster Leningrad Codex as word groups: clauses with
//! the roles of their parts (s, v, o, pp, adv, p …), noun and preposition phrases, coordinations. A dependency tree is
//! read off them by head rules: in a clause the verb heads, or the predicate when there is no verb; in a preposition
//! phrase the noun heads and the preposition is its `case`; in a noun phrase the first content word heads (the
//! construct chain's first noun, the noun after the article or the object marker); in a coordination the first
//! member heads and the others are `conj`, each conjunction a `cc` of the member after it. Then PTNK's readings
//! (UD Ancient Hebrew, measured against it, README): הָיָה with a predicate is a copula, לֵאמֹר is a subordinator
//! headed by its ל, a clause before the clause of `ClCl2` hangs on it, a number before a noun is its nummod, and more.
//!
//! Out: `<out-dir>/<BOOK>.conllu` (`# sent_id = wlc:<BOOK>:<n>`, one document per book, `# verse = GEN.1.1`) and
//! `<out-dir>/verses.tsv` (id → verses). Words are MACULA's (prefixes and suffixes as words of their own, a written word
//! of several a multiword token, the swallowed article written ה); FORM is the pointed text (NFC), LEMMA MACULA's
//! lemma, FEATS gender, number, person, PronType and the verb stem; marks of punctuation are tokens; MISC the English gloss, the word
//! reference, Strong's number and the argument frame (`A0:…`) where MACULA has one. The semantic-domain fields from the
//! Semantic Dictionary of Biblical Hebrew are not carried. Every tree is checked: one head per word, one root, no
//! cycle; a tree that fails is reported and not written.
//!
//!   macula compare <out-dir> <ud.conllu>…
//!
//! Measures our trees against a UD treebank of Biblical Hebrew whose words carry `Ref[MACULA]` (UD PTNK, CC BY-NC: a
//! test only). Words are matched by MACULA id; a UD head without an id of its own (punctuation, a split article) is
//! followed up to the nearest one that has it. UAS over the matched words; LAS on the relation without its subtype,
//! once as is and once with construct (`compound:smixut`) read as our `nmod`.
//!
//!   macula lessons requests <trees-dir> <en-pool.tsv> <parashot.tsv> <work-dir>
//!   macula lessons post <work-dir> <trees-dir> <en-pool.tsv> <school-dir>
//!
//! The Torah course of the Judaism school (operator, 17.10: «grow the religious materials, lessons, dictionaries»): one
//! lesson per weekly portion (parashah, 54), written by a teacher from the portion's whole text in English (JPS 1917,
//! aliyot marked) — Sonnet, and Opus for every eighth portion (operator: religious texts 80% Sonnet, 20% Opus). The
//! teacher also names three key verses and the tradition's reading of each; `post` writes each as a fact in layers,
//! the Hebrew verse (MACULA), word for word (its glosses), the JPS 1917 verse, and the idea — the layers taken from the
//! texts, not from the teacher. Lessons go to `<school-dir>/torah/NNN-<parashah>.lesson`; then the dictionary builder
//! (closed) marks them.
//!
//!   macula prag <trees-dir> <out.tsv> [--with rule,…] [--without rule,…]
//!   macula prag-compare <out.tsv> <trees-dir>/verses.tsv <en prag.tsv> <en pool.tsv.verses>
//!   macula prag-quotes <en pool.tsv>
//!
//! The pragmatic reading of each sentence by rules, its speech cut into frame and quote and carried from verse to
//! verse; its agreement with the English reading of the same verses, with the quotes of JPS 1917 and with the English
//! negation words (module `prag`, README «Pragmatics»).

use quick_xml::events::{BytesStart, Event};
use serde_json::json;
use quick_xml::Reader;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;


mod tanakh;
use macula::{consonants, english_verses, hebrew_verses, prag, unescape, verse_key, BOOKS, SAYING};

#[derive(Clone, Debug, Default)]
struct Word {
    /// MACULA's word id (`o010010010011`)
    mid: String,
    form: String,
    after: String,
    class: String,
    role: String,
    lemma: String,
    feats: BTreeMap<&'static str, String>,
    gloss: String,
    wref: String,
    strong: String,
    frame: String,
    proper: bool,
    /// MACULA marks the Aramaic words `lang="A"` (Daniel 2:4b–7:28, Ezra 4:8–6:18 and 7:12–26, Jeremiah 10:11, two words of
    /// Genesis 31:47); they get `Lang=arc` in MISC
    aramaic: bool,
    /// MACULA's `type`: the verb form (wayyiqtol, imperative, jussive …) or the kind of a particle (interrogative …)
    kind: String,
}

#[derive(Clone, Debug)]
enum Node {
    W(usize),
    G { role: String, class: String, rule: String, kids: Vec<Node> },
}


fn attrs(e: &BytesStart) -> BTreeMap<String, String> {
    e.attributes().flatten().map(|a| (a.key.as_ref().to_string(), unescape(&a.value))).collect()
}

fn word(a: &BTreeMap<String, String>) -> Word {
    let g = |k: &str| a.get(k).cloned().unwrap_or_default();
    let mut feats = BTreeMap::new();
    let map = |v: &str| -> Option<String> {
        Some(
            match v {
                "masculine" => "Masc",
                "feminine" => "Fem",
                "common" => "Com",
                "singular" => "Sing",
                "plural" => "Plur",
                "dual" => "Dual",
                "first" => "1",
                "second" => "2",
                "third" => "3",
                _ => return None,
            }
            .to_string(),
        )
    };
    for (k, f) in [("gender", "Gender"), ("number", "Number"), ("person", "Person")] {
        if let Some(v) = a.get(k).and_then(|v| map(v)) {
            feats.insert(f, v);
        }
    }
    if let Some(s) = a.get("stem").filter(|s| !s.is_empty()) {
        feats.insert("HebBinyan", s.to_uppercase());
    }
    Word {
        mid: g("xml:id"),
        form: g("unicode"),
        after: g("after"),
        class: g("class"),
        role: g("role"),
        lemma: g("lemma"),
        feats,
        gloss: g("gloss"),
        wref: g("ref"),
        strong: g("strongnumberx"),
        frame: g("frame"),
        proper: a.get("type").is_some_and(|t| t == "proper"),
        kind: g("type"),
        aramaic: a.get("lang").is_some_and(|l| l == "A"),
    }
}

/// The language of a sentence: Aramaic (`A`) when most of its words are, else Hebrew (`H`). Only two sentences mix
/// them: Daniel 2:4 (the frame in Hebrew, the Chaldeans' words in Aramaic) and Genesis 31:47 (the name יְגַר שָׂהֲדוּתָא).
fn sent_lang(s: &Sent) -> &'static str {
    if s.words.iter().filter(|w| w.aramaic).count() * 2 > s.words.len() { "A" } else { "H" }
}

/// One MACULA sentence: its id, its words in order and its top nodes.
struct Sent {
    id: String,
    words: Vec<Word>,
    top: Vec<Node>,
}

fn read_book(xml: &str) -> Vec<Sent> {
    let mut r = Reader::from_str(xml);
    let mut out = Vec::new();
    let mut cur: Option<Sent> = None;
    // the open groups: (role, class, rule, kids)
    let mut stack: Vec<(String, String, String, Vec<Node>)> = Vec::new();
    let mut open_word: Option<Word> = None;
    let push = |stack: &mut Vec<(String, String, String, Vec<Node>)>, cur: &mut Option<Sent>, n: Node| match stack.last_mut() {
        Some(g) => g.3.push(n),
        None => cur.as_mut().unwrap().top.push(n),
    };
    loop {
        match r.read_event() {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                "sentence" => cur = Some(Sent { id: attrs(&e).get("id").cloned().unwrap_or_default(), words: Vec::new(), top: Vec::new() }),
                "wg" if cur.is_some() => {
                    let a = attrs(&e);
                    let g = |k: &str| a.get(k).cloned().unwrap_or_default();
                    stack.push((g("role"), g("class"), g("rule"), Vec::new()));
                }
                "w" if cur.is_some() => open_word = Some(word(&attrs(&e))),
                _ => {}
            },
            Ok(Event::Empty(e)) if e.name().as_ref() == "w" && cur.is_some() => {
                let s = cur.as_mut().unwrap();
                s.words.push(word(&attrs(&e)));
                let i = s.words.len() - 1;
                push(&mut stack, &mut cur, Node::W(i));
            }
            Ok(Event::Text(t)) => {
                if let Some(w) = open_word.as_mut() {
                    if w.form.is_empty() {
                        w.form = unescape(t.as_ref()).trim().to_string();
                    }
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                "w" => {
                    if let (Some(w), Some(s)) = (open_word.take(), cur.as_mut()) {
                        s.words.push(w);
                        let i = s.words.len() - 1;
                        push(&mut stack, &mut cur, Node::W(i));
                    }
                }
                "wg" => {
                    if let Some((role, class, rule, kids)) = stack.pop() {
                        push(&mut stack, &mut cur, Node::G { role, class, rule, kids });
                    }
                }
                "sentence" => {
                    if let Some(s) = cur.take() {
                        out.push(s);
                    }
                    stack.clear();
                }
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("xml at {}: {e}", r.buffer_position()),
            _ => {}
        }
    }
    out
}

const FUNCTION: [&str; 7] = ["cj", "art", "om", "prep", "rel", "ptcl", "ij"];

fn is_fn_word(n: &Node, w: &[Word]) -> bool {
    matches!(n, Node::W(i) if FUNCTION.contains(&w[*i].class.as_str()))
}

fn class_of<'a>(n: &'a Node, w: &'a [Word]) -> &'a str {
    match n {
        Node::W(i) => &w[*i].class,
        Node::G { class, .. } => class,
    }
}

fn role_of<'a>(n: &'a Node, w: &'a [Word]) -> &'a str {
    match n {
        Node::W(i) => &w[*i].role,
        Node::G { role, .. } => role,
    }
}

/// The words under a node, in order.
fn leaves(n: &Node) -> Vec<usize> {
    match n {
        Node::W(i) => vec![*i],
        Node::G { kids, .. } => kids.iter().flat_map(leaves).collect(),
    }
}

/// A relative clause with no noun before it, made a noun or a clause (`Relp2Np`, `Relp2CL`).
fn headless_relative(n: &Node) -> bool {
    matches!(n, Node::G { rule, .. } if rule.starts_with("Relp2"))
}

/// Prepositions that are nouns in construct in PTNK.
const NOUN_PREPS: [&str; 4] = ["תחת", "נגד", "עמד", "נכח"];

fn noun_prep(n: &Node, w: &[Word]) -> bool {
    matches!(n, Node::W(i) if w[*i].class == "prep" && NOUN_PREPS.contains(&consonants(&w[*i].lemma).as_str()))
}

/// The verb הָיָה («to be») as a clause's verb; in Aramaic הֲוָה.
fn copula(n: &Node, w: &[Word]) -> bool {
    matches!(n, Node::W(i) if w[*i].role == "v" && w[*i].class == "verb" && (consonants(&w[*i].lemma) == "היה" || (w[*i].aramaic && consonants(&w[*i].lemma) == "הוה")))
}

fn rule_of(n: &Node) -> &str {
    match n {
        Node::W(_) => "",
        Node::G { rule, .. } => rule,
    }
}

/// A group of function words only, as MACULA's compound prepositions (`PpPp`: מֵ + עִם, «from with») and runs of
/// conjunctions and adverbs (`CjpCjp`: וְ + עַתָּה, «and now»): its words are spliced into the group above, so each
/// one hangs on the phrase it introduces (UD: both prepositions are `case` of the noun).
fn function_group(n: &Node, w: &[Word]) -> bool {
    match n {
        Node::G { class, kids, .. } if class == "pp" => kids.iter().all(|k| matches!(k, Node::W(i) if w[*i].class == "prep")),
        Node::G { class, kids, .. } if class == "cjp" => kids.iter().all(|k| matches!(k, Node::W(i) if matches!(w[*i].class.as_str(), "cj" | "adv" | "ptcl"))),
        _ => false,
    }
}

fn splice(n: &Node, w: &[Word]) -> Node {
    match n {
        Node::W(_) => n.clone(),
        Node::G { role, class, rule, kids } => {
            let mut out = Vec::new();
            for k in kids {
                let k = splice(k, w);
                match &k {
                    Node::G { kids: inner, .. } if function_group(&k, w) => out.extend(inner.iter().cloned()),
                    _ => out.push(k),
                }
            }
            Node::G { role: role.clone(), class: class.clone(), rule: rule.clone(), kids: out }
        }
    }
}

/// Which child heads a group, and whether a rule picked it (false: the fallback, the first child).
fn head_child(class: &str, rule: &str, kids: &[Node], w: &[Word]) -> (usize, bool) {
    let find = |p: &dyn Fn(&Node) -> bool| kids.iter().position(p);
    let last = |p: &dyn Fn(&Node) -> bool| kids.iter().rposition(p);
    let pick = match (class, rule) {
        // a clause before a clause that heads (UD: the later one is the main clause): a hanging noun (casus pendens,
        // `dislocated`), הִנֵּה (`discourse`), a condition (`advcl`); «if …, and then …» heads on the clause after ו
        ("cl", "ClCl2") | ("cl", "CLandCL2") => last(&|n| !is_fn_word(n, w)),
        // a number, an adverb or an adjective before a noun: the noun heads («seven days», «yet three days»)
        ("np", "NumpNP") | ("np", "AdvpNp") | ("np", "AdjpNp") | ("pp", "AdvpPp") => last(&|n| !is_fn_word(n, w)),
        // a preposition or the object marker before a headless relative clause («from what», «that which»): the
        // function word heads, as in PTNK, and the clause is its `acl:relcl`
        ("pp", "PpRelp") | ("np", "OmpRelp") => Some(0),
        // … and so before a relative clause made a noun or a clause («כַּאֲשֶׁר», «as [what] he spoke»)
        _ if kids.len() == 2 && matches!(&kids[0], Node::W(i) if matches!(w[*i].class.as_str(), "prep" | "om")) && headless_relative(&kids[1]) => Some(0),
        // a preposition that is a noun in construct in PTNK (תַּחַת «under, instead of», נֶגֶד «before», עִמָּד «with»,
        // נֹכַח «opposite»): it heads its phrase
        ("pp", _) if find(&|n| noun_prep(n, w)).is_some() => last(&|n| noun_prep(n, w)),
        // הָיָה with a predicate MACULA marks as its object («and the earth was [o: without form]») is a copula in
        // UD: the predicate heads
        ("cl", _) if find(&|n| copula(n, w)).is_some() && find(&|n| role_of(n, w) == "o").is_some() => find(&|n| role_of(n, w) == "o"),
        // … and with a prepositional predicate, unless it is the narrative וַיְהִי («and it came to pass in …»): the
        // phrase with ל and a noun («shall be for a sign»), else one with ל, else the first heads
        ("cl", _) if find(&|n| copula(n, w) && !matches!(n, Node::W(i) if w[*i].kind == "wayyiqtol")).is_some() && find(&|n| role_of(n, w) == "pp").is_some() => {
            let pp_l = |n: &Node, noun: bool| role_of(n, w) == "pp" && leaves(n).first().is_some_and(|&i| w[i].class == "prep" && consonants(&w[i].lemma) == "ל") && (!noun || leaves(n).len() > 2 || leaves(n).last().is_some_and(|&i| w[i].class != "pron"));
            find(&|n| pp_l(n, true)).or_else(|| find(&|n| pp_l(n, false))).or_else(|| find(&|n| role_of(n, w) == "pp"))
        }
        ("cl", _) => find(&|n| role_of(n, w) == "v")
            .or_else(|| find(&|n| role_of(n, w) == "p"))
            .or_else(|| find(&|n| class_of(n, w) == "cl"))
            .or_else(|| find(&|n| !is_fn_word(n, w))),
        // the object of the preposition heads; a coordination of phrases, its first member
        ("pp", _) => find(&|n| !matches!(n, Node::W(i) if w[*i].class == "prep") && !is_fn_word(n, w)),
        ("relp", _) => find(&|n| class_of(n, w) == "cl").or_else(|| find(&|n| !is_fn_word(n, w))),
        _ => find(&|n| !is_fn_word(n, w)),
    };
    match pick {
        Some(i) => (i, true),
        None => (0, false),
    }
}

/// The relation of a non-head child to its group.
/// Conjunctions that open a subordinate clause: `mark` (SCONJ) in UD, not `cc`.
const SUBORDINATORS: [&str; 8] = ["כי", "אם", "למען", "פן", "טרם", "יען", "עקב", "לבלתי"];
/// Particles MACULA classes as conjunctions that are adverbs in UD: «also» (גַּם, אַף), «but» (אוּלָם), «truly» (אָמְנָם).
const CJ_ADVERBS: [&str; 4] = ["גם", "אף", "אולם", "אמנם"];

/// Demonstratives MACULA classes as adjectives (זֶה, זֹאת, אֵלֶּה, the rare אֵל): `det` after a noun, as the pronouns.
const DEMONSTRATIVES: [&str; 4] = ["זה", "זאת", "אלה", "אל"];
/// The same in Biblical Aramaic (MACULA's `lang="A"`): דְּנָה, דָּא, אִלֵּין, דֵּךְ, אִלֵּךְ, דִּכֵּן.
const ARC_DEMONSTRATIVES: [&str; 6] = ["דנה", "דא", "אלין", "דך", "אלך", "דכן"];
/// Aramaic subordinators MACULA classes as conjunctions: הֵן «if».
const ARC_SUBORDINATORS: [&str; 1] = ["הן"];

/// A demonstrative MACULA classes as an adjective, in the word's own language.
fn demonstrative(w: &Word) -> bool {
    let c = consonants(&w.lemma);
    w.class == "adj" && (DEMONSTRATIVES.contains(&c.as_str()) || (w.aramaic && ARC_DEMONSTRATIVES.contains(&c.as_str())))
}

/// A conjunction that opens a subordinate clause, in the word's own language.
fn subordinator(w: &Word) -> bool {
    let c = consonants(&w.lemma);
    w.class == "cj" && (SUBORDINATORS.contains(&c.as_str()) || (w.aramaic && ARC_SUBORDINATORS.contains(&c.as_str())))
}

/// Verbs whose infinitive with ל is their complement (`xcomp` in PTNK): «could», «commanded», «gave», «kept»,
/// «finished», «did again», «refused», «chose», «swore», «began», «learned», «ceased», «hurried», «was willing».
const XCOMP_VERBS: [&str; 14] = ["יכל", "צוה", "נתן", "שמר", "כלה", "יסף", "מאן", "בחר", "שבע", "חלל", "למד", "חדל", "מהר", "אבה"];

/// A list rule: MACULA's names for members of one kind side by side, with or without a conjunction — `NpNpNp`,
/// `2PpaPp`, `NP3NP`, `PpPp`, `Conj4Np` … — not a head with a modifier (`2Pp` is an apposition in PTNK).
fn list_rule(rule: &str) -> bool {
    if rule == "2Pp" {
        return false;
    }
    let (mut kinds, mut members, mut rest) = (0u8, 0, rule);
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix("Conj").or_else(|| rest.strip_prefix("and")) {
            rest = r;
        } else if let Some(r) = ["NP", "Np", "np"].iter().find_map(|p| rest.strip_prefix(p)) {
            kinds |= 1;
            members += 1;
            rest = r;
        } else if let Some(r) = ["PP", "Pp", "pp"].iter().find_map(|p| rest.strip_prefix(p)) {
            kinds |= 2;
            members += 1;
            rest = r;
        } else if let Some(r) = rest.strip_prefix('a') {
            rest = r;
        } else if rest.starts_with(|c: char| c.is_ascii_digit()) {
            members += 1;
            rest = &rest[1..];
        } else {
            return false;
        }
    }
    members >= 2 && (kinds == 1 || kinds == 2)
}

/// The rules of a construct chain: «the sons of Laban» (`NPofNP`), «all the land» (`QuanNP`), «broad of hands»
/// (`AdjpofNp`); the word after the first is `compound:smixut`.
fn construct(rule: &str) -> bool {
    matches!(rule, "NPofNP" | "QuanNP" | "AdjpofNp")
}


/// The relation of a non-head child to its group (UD conventions for Hebrew where they apply: the construct chain is
/// `compound:smixut`, a subordinating conjunction `mark`, a demonstrative `det`, an infinitive after a preposition
/// `advcl`, the parts of a compound number `flat`).
fn relation(child: &Node, child_head: usize, group_class: &str, group_rule: &str, head: &Node, after_cj: bool, before: bool, w: &[Word]) -> &'static str {
    let cls = class_of(child, w);
    // a clause before the clause that heads
    if before && cls == "cl" && matches!(group_rule, "ClCl2" | "CLandCL2") {
        return match rule_of(child) {
            "Intj2CL" => "discourse",
            "Np2CL" | "PP2CL" => "dislocated",
            "cjpCLx" => "advcl",
            _ => "parataxis",
        };
    }
    // a later member of a list with no conjunction («a bull, a ram, a lamb»; «from good to bad») is `conj`
    if !before && list_rule(group_rule) && !is_fn_word(child, w) {
        return "conj";
    }
    if headless_relative(child) && !before {
        return "acl:relcl";
    }
    // the object of a noun-like preposition is its construct (a suffix pronoun its possessor)
    if noun_prep(head, w) && !before && !is_fn_word(child, w) {
        return if cls == "pron" { "nmod:poss" } else { "compound:smixut" };
    }
    let head_is_verb = matches!(head, Node::W(i) if w[*i].class == "verb") || role_of(head, w) == "v";
    let child_is_verb = w.get(child_head).is_some_and(|x| x.class == "verb");
    match role_of(child, w) {
        "s" => return "nsubj",
        // an object clause («he saw that …») is `ccomp`
        "o" if cls == "cl" => return "ccomp",
        "o" => return "obj",
        // the second object is what the first is made or called («he called its name Gilead»): `xcomp` in PTNK
        "o2" => return "xcomp",
        "io" => return "iobj",
        // an infinitive clause after a preposition («when he came», «to see») is an adverbial clause; after a verb
        // that takes it as its complement («he could not see», «he refused to be comforted») it is `xcomp`
        "pp" if child_is_verb && matches!(head, Node::W(i) if w[*i].class == "verb" && XCOMP_VERBS.contains(&consonants(&w[*i].lemma).as_str())) => return "xcomp",
        "pp" => return if child_is_verb { "advcl" } else { "obl" },
        // an adverbial: a noun phrase («that day», «home») is `obl`; the infinitive absolute that strengthens its verb
        // («surely die») is `obl:npmod`
        "adv" => {
            return if cls == "pp" && child_is_verb {
                "advcl"
            } else if cls == "pp" || cls == "np" || cls == "noun" {
                "obl"
            } else if cls == "verb" {
                "obl:npmod"
            } else if cls == "ij" {
                // נָא «please» and other interjections in an adverb's place
                "discourse"
            } else if cls == "cl" {
                "advcl"
            } else {
                "advmod"
            }
        }
        "p" => return if head_is_verb { "xcomp" } else { "nsubj" },
        "v" if copula(child, w) => return "cop",
        "v" | "vc" => return "conj",
        _ => {}
    }
    if let Node::W(i) = child {
        return match w[*i].class.as_str() {
            "cj" if CJ_ADVERBS.contains(&consonants(&w[*i].lemma).as_str()) => "advmod",
            "cj" if subordinator(&w[*i]) => "mark",
            "cj" => "cc",
            "pron" if w[*i].kind == "demonstrative" => "det",
            // «thirty and a hundred»: the parts of a number joined by ו are a coordination in PTNK, without it `flat`
            "num" if group_class == "nump" && after_cj => "conj",
            "num" if group_class == "nump" => "flat",
            "adj" if demonstrative(&w[*i]) => "det",
            "noun" | "num" | "verb" | "adj" if construct(group_rule) && !after_cj => "compound:smixut",
            "art" => "det",
            "om" | "prep" => "case",
            "rel" => "mark",
            "ptcl" | "adv" => "advmod",
            "ij" => "discourse",
            "adj" => "amod",
            "num" => "nummod",
            "pron" if group_class == "np" => "nmod:poss",
            _ if after_cj => "conj",
            _ if group_rule.contains("Appos") => "appos",
            _ if group_class == "cl" || group_class.is_empty() => "dep",
            _ => "nmod",
        };
    }
    if after_cj && cls == class_of(head, w) {
        return "conj";
    }
    match (group_class, cls) {
        ("np", "np") | ("np", "nump") | ("adjp", "np") if construct(group_rule) => "compound:smixut",
        ("nump", "nump") => "flat",
        ("cl", "cl") | ("", "cl") if head_is_verb && !after_cj => "ccomp",
        ("cl", "cl") | ("", "cl") | ("", _) => "conj",
        (_, "relp") => "acl:relcl",
        (_, "cl") => "acl",
        // «הַזֶּה», «הַהִוא»: an article and a demonstrative after a noun is a determiner
        (_, "adjp") if w.get(child_head).is_some_and(|x| x.class == "pron" || demonstrative(x)) => "det",
        // a participle after a noun («the goats going up») is a relative clause in UD
        (_, "adjp") if child_is_verb => "acl:relcl",
        (_, "adjp") => "amod",
        (_, "nump") => "nummod",
        (_, "advp") => "advmod",
        _ if group_rule.contains("Appos") => "appos",
        _ if after_cj => "conj",
        _ => "nmod",
    }
}


/// Fill heads and relations for one node; returns its head word.
fn attach(n: &Node, w: &[Word], heads: &mut [(usize, &'static str)], fallback: &mut usize) -> usize {
    match n {
        Node::W(i) => *i,
        Node::G { class, rule, kids, .. } => {
            if kids.is_empty() {
                return usize::MAX;
            }
            let (hc, ruled) = head_child(class, rule, kids, w);
            if !ruled {
                *fallback += 1;
            }
            let hw: Vec<usize> = kids.iter().map(|k| attach(k, w, heads, fallback)).collect();
            let h = hw[hc];
            let mut after_cj = false;
            let coord = class.is_empty() || kids.iter().all(|k| is_fn_word(k, w) || (class_of(k, w) == "cl" && role_of(k, w).is_empty()));
            // the last clause seen, for speech: (its head word, a conjunction since)
            let mut last_clause: Option<(usize, bool)> = None;
            // the head of a quotation once it has begun: the clauses after it continue what is said (UD: they are
            // `conj` or `parataxis` of the quotation's first clause, not of the verb of saying)
            let mut quote: Option<usize> = None;
            for (j, k) in kids.iter().enumerate() {
                let is_clause = class_of(k, w) == "cl" && role_of(k, w).is_empty();
                if j == hc || hw[j] == usize::MAX {
                    after_cj |= is_fn_word(k, w) && class_of(k, w) == "cj";
                    if is_fn_word(k, w) && class_of(k, w) == "cj" {
                        if let Some(l) = last_clause.as_mut() {
                            l.1 = true;
                        }
                    }
                    if is_clause && hw[j] != usize::MAX {
                        last_clause = Some((hw[j], false));
                    }
                    continue;
                }
                let is_cj = matches!(k, Node::W(i) if w[*i].class == "cj");
                // a conjunction belongs to the member after it (UD), the group's head when nothing follows; inside a
                // clause it belongs to the clause's head (the ו of «וַיֹּאמֶר» hangs on the verb, not on the subject)
                // a group of clauses only (the sentence, «CLaCL», «ClCl» …) coordinates: its conjunctions belong to the
                // clause after them; in a clause with roles a conjunction belongs to the clause's head
                let target = if is_cj && class == "cl" && !coord {
                    h
                } else if is_cj {
                    // the next member that is not itself a conjunction — the group's head too, when it comes next
                    (j + 1..kids.len()).find(|&x| hw[x] != usize::MAX && !matches!(&kids[x], Node::W(i) if w[*i].class == "cj")).map_or(h, |x| hw[x])
                } else {
                    h
                };
                let target = if target == hw[j] { h } else { target };
                // a narrative wayyiqtol takes the story up again: the quotation has ended
                let narrative = matches!(w.get(hw[j]), Some(x) if x.kind == "wayyiqtol");
                let target = match quote {
                    Some(q) if coord && is_clause && !narrative => q,
                    _ => target,
                };
                if coord && is_clause && narrative {
                    quote = None;
                }
                // what was said: a clause right after a clause of saying, no conjunction between
                let speech = (class.is_empty() || class == "cl") && is_clause && last_clause.is_some_and(|(lh, cj)| !cj && SAYING.contains(&consonants(&w[lh].lemma).as_str()) && lh != hw[j]);
                // the word before this member: a clause after a subordinating conjunction (כִּי, אִם …) is an adverbial
                // clause; a clause after a clause with no conjunction between is set beside it (UD: `parataxis`)
                let prev = j.checked_sub(1).map(|p| &kids[p]);
                let prev_cj = prev.and_then(|p| match p {
                    Node::W(i) if w[*i].class == "cj" => Some(subordinator(&w[*i])),
                    _ => None,
                });
                if speech {
                    heads[hw[j]] = (last_clause.unwrap().0 + 1, "ccomp");
                    quote = Some(hw[j]);
                } else if coord && is_clause && j > hc && prev_cj == Some(true) {
                    heads[hw[j]] = (target + 1, "advcl");
                } else if coord && is_clause && j > hc && prev_cj.is_none() {
                    heads[hw[j]] = (target + 1, "parataxis");
                } else {
                    heads[hw[j]] = (target + 1, relation(k, hw[j], class, rule, &kids[hc], after_cj && j > hc, j < hc, w));
                }
                if is_cj {
                    if let Some(l) = last_clause.as_mut() {
                        l.1 = true;
                    }
                }
                after_cj |= is_cj;
                if is_clause {
                    last_clause = Some((hw[j], false));
                }
            }
            h
        }
    }
}

fn upos(wd: &Word) -> &'static str {
    match wd.class.as_str() {
        "noun" if wd.proper => "PROPN",
        "noun" => "NOUN",
        "verb" => "VERB",
        "prep" if NOUN_PREPS.contains(&consonants(&wd.lemma).as_str()) => "NOUN",
        "prep" | "om" => "ADP",
        "cj" if CJ_ADVERBS.contains(&consonants(&wd.lemma).as_str()) => "ADV",
        "cj" if subordinator(wd) => "SCONJ",
        "cj" => "CCONJ",
        "pron" => "PRON",
        "art" => "DET",
        "adv" => "ADV",
        "num" => "NUM",
        "adj" => "ADJ",
        "rel" => "SCONJ",
        "ij" => "INTJ",
        "ptcl" => "PART",
        _ => "X",
    }
}

/// One head per word in range, exactly one root, no cycle.
fn check(heads: &[(usize, &'static str)]) -> Result<(), String> {
    let n = heads.len();
    let roots = heads.iter().filter(|h| h.0 == 0).count();
    if roots != 1 {
        return Err(format!("{roots} roots"));
    }
    for (i, &(h, _)) in heads.iter().enumerate() {
        if h > n || h == i + 1 {
            return Err(format!("word {} has head {h}", i + 1));
        }
        let (mut x, mut steps) = (i + 1, 0);
        while x != 0 {
            x = heads[x - 1].0;
            steps += 1;
            if steps > n {
                return Err(format!("cycle through word {}", i + 1));
            }
        }
    }
    Ok(())
}

/// GEN 1:1!2 → GEN.1.1
fn verse_of(r: &str) -> String {
    let r = r.split('!').next().unwrap_or(r);
    let (b, cv) = r.split_once(' ').unwrap_or((r, ""));
    format!("{b}.{}", cv.replace(':', "."))
}

/// One row of the CoNLL-U output: a word of MACULA, a further part of a name MACULA keeps as one word, or a mark of
/// punctuation MACULA keeps in `after` (paseq ׀, sof pasuq ׃, the paragraph marks ס and פ).
struct Row {
    form: String,
    lemma: String,
    upos: &'static str,
    feats: String,
    head: usize,
    rel: &'static str,
    misc: Vec<String>,
    space: bool,
    /// the article swallowed by a preposition (בַּ = בְּ + הַ): written ה, as in PTNK, inside a multiword token
    elided: bool,
}

/// FEATS values are letters and digits only (UD): «QAL PASSIVE» → QALPASSIVE; MACULA's «UNKNOWN: V» is left out.
fn feat_value(v: &str) -> Option<String> {
    let x: String = v.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    (!x.is_empty() && !x.starts_with("UNKNOWN")).then_some(x)
}

/// The words in the order of the text. MACULA gives them in the order of its groups, and in about 3% of sentences
/// a group takes a word from further on (Ruth 1:1: «וַיֵּלֶךְ אִישׁ מִבֵּית לֶחֶם» has אִישׁ in the subject group after the
/// place). The heads are read in MACULA's order; the words are then sorted by their id (book, chapter, verse, word,
/// part; the swallowed article after its preposition) and the heads renumbered.
fn surface_order(s: Sent, heads: Vec<(usize, &'static str)>) -> (Sent, Vec<(usize, &'static str)>) {
    let mut order: Vec<usize> = (0..s.words.len()).collect();
    order.sort_by(|&a, &b| s.words[a].mid.cmp(&s.words[b].mid).then(a.cmp(&b)));
    if order.iter().enumerate().all(|(i, &k)| i == k) {
        return (s, heads);
    }
    let mut new_of = vec![0usize; order.len()];
    for (new, &old) in order.iter().enumerate() {
        new_of[old] = new;
    }
    let words = order.iter().map(|&k| s.words[k].clone()).collect();
    let heads = order.iter().map(|&k| (if heads[k].0 == 0 { 0 } else { new_of[heads[k].0 - 1] + 1 }, heads[k].1)).collect();
    (Sent { id: s.id, words, top: Vec::new() }, heads)
}

fn conllu(s: &Sent, id: &str, heads: &[(usize, &'static str)]) -> String {
    let mut verses: Vec<String> = s.words.iter().map(|w| verse_of(&w.wref)).collect();
    verses.dedup();
    // the rows of each word; first[i] is the row of word i
    let mut rows: Vec<Row> = Vec::new();
    let mut first = Vec::with_capacity(s.words.len());
    for (i, w) in s.words.iter().enumerate() {
        let mut feats: Vec<String> = w.feats.iter().filter_map(|(k, v)| feat_value(v).map(|v| format!("{k}={v}"))).collect();
        // PronType (UD): the article, demonstratives, interrogatives, personal pronouns
        let is_dem = w.kind == "demonstrative" || demonstrative(w);
        let pron_type = match w.class.as_str() {
            "art" if heads[i].1 == "det" => Some("Art"),
            "pron" | "adj" if is_dem => Some("Dem"),
            "pron" if w.kind == "interrogative" => Some("Int"),
            "pron" => Some("Prs"),
            _ => None,
        };
        // the head of a fixed expression (לֵאמֹר, כִּי אִם) says what it works as: a subordinator
        if heads.iter().any(|&(h, r)| h == i + 1 && r == "fixed") {
            feats.push("ExtPos=SCONJ".into());
            feats.sort();
        }
        if let Some(p) = pron_type {
            feats.push(format!("PronType={p}"));
            feats.sort();
        }
        let feats = if feats.is_empty() { "_".to_string() } else { feats.join("|") };
        let mut misc = vec![format!("Gloss={}", w.gloss.replace(['|', ' '], "_")), format!("Ref={}", w.wref.replace(' ', "_")), format!("MId={}", w.mid)];
        if !w.strong.is_empty() {
            misc.push(format!("Strong={}", w.strong));
        }
        if w.aramaic {
            misc.push("Lang=arc".to_string());
        }
        if !w.frame.is_empty() {
            misc.push(format!("Frame={}", w.frame.replace(' ', "").replace('|', "_")));
        }
        if !w.kind.is_empty() && w.kind != "common" && w.kind != "proper" {
            misc.push(format!("Type={}", w.kind.replace(' ', "_")));
        }
        // a Hebrew word in an Aramaic sentence (the frame of Daniel 2:4); every Aramaic word has Lang=arc above
        if !w.aramaic && sent_lang(s) == "A" {
            misc.push("Lang=hbo".to_string());
        }
        // the paseq is in MACULA's form after a space: a mark of its own
        let (base, paseq) = match w.form.strip_suffix('׀') {
            Some(b) => (b.trim_end().to_string(), true),
            None => (w.form.clone(), w.after.contains('׀')),
        };
        let elided = base.is_empty();
        let base = if elided && w.class == "art" { "ה".to_string() } else if elided { "_".to_string() } else { base };
        // a place name MACULA keeps as one word (בְּאֵר שָׁבַע): a word per part, the later ones `flat`
        let parts: Vec<&str> = base.split(' ').filter(|x| !x.is_empty()).collect();
        let lemmas: Vec<&str> = w.lemma.split(' ').filter(|x| !x.is_empty()).collect();
        let mut up = upos(w);
        if demonstrative(w) {
            up = if heads[i].1 == "det" { "DET" } else { "PRON" };
        }
        // הָיָה as a copula is an auxiliary; the relative ה (the article on a verbal participle) a subordinator
        match heads[i].1 {
            "cop" if w.class == "verb" => up = "AUX",
            "mark" if w.class == "art" => up = "SCONJ",
            _ => {}
        }
        first.push(rows.len());
        for (k, part) in parts.iter().enumerate() {
            let lemma = if lemmas.len() == parts.len() {
                lemmas[k].to_string()
            } else if k == 0 && !lemmas.is_empty() {
                lemmas.join("־")
            } else {
                "_".to_string()
            };
            rows.push(Row { form: part.nfc().collect(), lemma: lemma.nfc().collect(), upos: up, feats: if k == 0 { feats.clone() } else { "_".into() }, head: usize::MAX, rel: "flat", misc: if k == 0 { misc.clone() } else { vec![format!("Ref={}", w.wref.replace(' ', "_"))] }, space: true, elided, });
        }
        // the space after the word: none before a maqaf's next word or a prefix's host
        let maqaf = w.after == "־" || (!w.after.is_empty() && w.form.ends_with(w.after.as_str()) && !paseq);
        let mut marks: Vec<&str> = Vec::new();
        if paseq {
            marks.push("׀");
        }
        if w.after.contains('׃') {
            marks.push("׃");
        }
        if w.after.contains('ס') {
            marks.push("ס");
        }
        if w.after.contains('פ') {
            marks.push("פ");
        }
        let last = rows.len() - 1;
        rows[last].space = !maqaf && !w.after.is_empty() && (paseq || w.after.contains(' ') || marks.is_empty());
        if !marks.is_empty() {
            // a space before the paseq; sof pasuq and the paragraph marks are written close
            rows[last].space = paseq;
            for m in &marks {
                rows.push(Row { form: m.to_string(), lemma: m.to_string(), upos: "PUNCT", feats: "_".into(), head: first[i] + 1, rel: "punct", misc: vec![format!("Ref={}", w.wref.replace(' ', "_"))], space: false, elided: false });
            }
            let l = rows.len() - 1;
            rows[l].space = true;
        }
    }
    // heads of the words, by their new numbers
    for (i, &(h, rel)) in heads.iter().enumerate() {
        let r = first[i];
        rows[r].head = if h == 0 { 0 } else { first[h - 1] + 1 };
        rows[r].rel = rel;
        let end = first.get(i + 1).copied().unwrap_or(rows.len());
        let mut k = r + 1;
        while k < end && rows[k].head == usize::MAX {
            rows[k].head = r + 1;
            rows[k].rel = if rows[r].upos == "PROPN" { "flat:name" } else { "flat" };
            k += 1;
        }
    }
    // a mark hangs on the word before it, or on that word's head when it is part of a fixed expression or a name
    for k in 0..rows.len() {
        if rows[k].upos == "PUNCT" {
            let mut h = rows[k].head;
            while h != 0 && matches!(rows[h - 1].rel, "fixed" | "flat" | "flat:name") {
                h = rows[h - 1].head;
            }
            rows[k].head = if h == 0 { rows.iter().position(|r| r.head == 0).map_or(0, |r| r + 1) } else { h };
        }
    }
    let n = rows.len();
    rows[n - 1].space = false;
    // tokens: the written words — rows with no space, maqaf or mark between; a token of more than one word (a
    // prefix, a suffix, a swallowed article) is a multiword token whose form is the written word, as in PTNK
    let brk = |k: usize| rows[k].space || rows[k].upos == "PUNCT" || rows[k + 1].upos == "PUNCT" || rows[k].form.ends_with('־');
    let mut text = String::new();
    let mut body = String::new();
    let mut t = 0;
    while t < n {
        let mut e = t;
        while e + 1 < n && !brk(e) {
            e += 1;
        }
        let surface: String = rows[t..=e].iter().filter(|r| !r.elided).map(|r| r.form.as_str()).collect::<String>().nfc().collect();
        text.push_str(&surface);
        if rows[e].space {
            text.push(' ');
        }
        let mwt = e > t;
        if mwt {
            body.push_str(&format!("{}-{}\t{surface}\t_\t_\t_\t_\t_\t_\t_\t{}\n", t + 1, e + 1, if rows[e].space { "_" } else { "SpaceAfter=No" }));
        }
        for (k, r) in rows.iter().enumerate().take(e + 1).skip(t) {
            let mut misc = r.misc.clone();
            if !mwt && !r.space && k + 1 < n {
                misc.push("SpaceAfter=No".into());
            }
            let misc = if misc.is_empty() { "_".to_string() } else { misc.join("|") };
            let lemma = if r.lemma.is_empty() { "_" } else { &r.lemma };
            body.push_str(&format!("{}\t{}\t{lemma}\t{}\t_\t{}\t{}\t{}\t_\t{misc}\n", k + 1, r.form, r.upos, r.feats, r.head, r.rel));
        }
        t = e + 1;
    }
    format!("# sent_id = {id}\n# verse = {}\n# text = {}\n{body}\n", verses.join(" "), text.trim_end())
}

/// לֵאמֹר («saying») is a fixed subordinator in PTNK: the ל heads and אמר is its `fixed`; the ל is the `mark` of the
/// quoted clause after it, and that clause hangs on the verb of saying (`ccomp`; `advcl` after a verb that is not of
/// saying, as «he commanded … saying»). With no clause after it in the sentence, לֵאמֹר is the verb's `ccomp` itself.
fn lemor(w: &[Word], heads: &mut [(usize, &'static str)]) {
    let n = heads.len();
    let up = |heads: &[(usize, &'static str)], x: usize, anc: usize| -> bool {
        // is `anc` an ancestor of `x` (1-based)?
        let (mut y, mut steps) = (x, 0);
        while y != 0 && steps <= n {
            if y == anc {
                return true;
            }
            y = heads[y - 1].0;
            steps += 1;
        }
        false
    };
    for l in 0..n.saturating_sub(1) {
        let a = l + 1;
        if !(w[l].class == "prep" && consonants(&w[l].lemma) == "ל" && w[a].class == "verb" && consonants(&w[a].lemma) == "אמר" && w[a].kind == "infinitive construct") {
            continue;
        }
        // the verb the infinitive hangs on
        let v = heads[a].0;
        if v == 0 || v == l + 1 {
            continue;
        }
        // the clause after: climb from the next word while its head lies after the infinitive
        let mut q = None;
        if a + 1 < n {
            let (mut x, mut steps) = (a + 2, 0);
            while heads[x - 1].0 > a + 1 && steps <= n {
                x = heads[x - 1].0;
                steps += 1;
            }
            let clause = matches!(heads[x - 1].1, "ccomp" | "parataxis" | "conj" | "advcl" | "dep");
            if heads[x - 1].0 != 0 && clause && !up(heads, v, x) {
                q = Some(x);
            }
        }
        let saying = SAYING.contains(&consonants(&w[v - 1].lemma).as_str());
        heads[a] = (l + 1, "fixed");
        match q {
            Some(x) => {
                heads[x - 1] = (v, if saying { "ccomp" } else { "advcl" });
                heads[l] = (x, "mark");
            }
            None => heads[l] = (v, "ccomp"),
        }
    }
}

fn tree(s: &Sent) -> (Vec<(usize, &'static str)>, usize) {
    let mut heads = vec![(0usize, "dep"); s.words.len()];
    let mut fallback = 0;
    let top = if s.top.len() == 1 { s.top[0].clone() } else { Node::G { role: String::new(), class: String::new(), rule: String::new(), kids: s.top.clone() } };
    let top = splice(&top, &s.words);
    let root = attach(&top, &s.words, &mut heads, &mut fallback);
    if root < heads.len() {
        heads[root] = (0, "root");
    }
    lemor(&s.words, &mut heads);
    // «all the wealth which God took»: a relative clause on the noun after כָּל hangs on כָּל, the head of the chain
    for i in 0..heads.len() {
        let (y, rel) = heads[i];
        if rel == "acl:relcl" && y != 0 && heads[y - 1].1 == "compound:smixut" {
            let z = heads[y - 1].0;
            if z != 0 && consonants(&s.words[z - 1].lemma) == "כל" {
                heads[i].0 = z;
            }
        }
    }
    // כִּי אִם («but rather», «except») is one subordinator: אִם is the `fixed` of כִּי
    for i in 1..heads.len() {
        let (a, b) = (&s.words[i - 1], &s.words[i]);
        if a.class == "cj" && b.class == "cj" && consonants(&a.lemma) == "כי" && consonants(&b.lemma) == "אם" && heads[i - 1].0 != i + 1 {
            heads[i] = (i, "fixed");
        }
    }
    // the article on a participle that acts as a verb — it modifies a noun, or has dependents of its own — is the
    // relative ה («the goats [which are] going up»): `mark`, as in PTNK; on a participle used as a noun it stays `det`
    for i in 0..heads.len() {
        let (h, rel) = heads[i];
        if s.words[i].class != "art" || rel != "det" || h == 0 || s.words[h - 1].class != "verb" {
            continue;
        }
        let verbal = matches!(heads[h - 1].1, "acl:relcl" | "acl") || (0..heads.len()).any(|j| j != i && heads[j].0 == h);
        if verbal {
            heads[i].1 = "mark";
        }
    }
    (heads, fallback)
}

/// MACULA id → (head's MACULA id or "ROOT", relation), from CoNLL-U whose MISC has the id under `key`.
fn heads_by_id(text: &str, key: &str) -> BTreeMap<String, (String, String)> {
    let mut out = BTreeMap::new();
    for block in text.split("\n\n") {
        let rows: Vec<Vec<&str>> = block.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').collect::<Vec<_>>()).filter(|c| c.len() == 10 && !c[0].contains('-') && !c[0].contains('.')).collect();
        let id_of = |c: &Vec<&str>| -> Option<String> {
            c[9].split('|').find_map(|kv| kv.strip_prefix(key)).map(String::from).filter(|x| x.len() > 1 && x[1..].chars().all(|d| d.is_ascii_digit()))
        };
        let by_pos: BTreeMap<&str, &Vec<&str>> = rows.iter().map(|c| (c[0], c)).collect();
        for c in &rows {
            let Some(me) = id_of(c) else { continue };
            // follow the head up to a word with an id
            let (mut h, mut steps) = (c[6], 0);
            let head = loop {
                if h == "0" {
                    break Some("ROOT".to_string());
                }
                let Some(hc) = by_pos.get(h) else { break None };
                if let Some(x) = id_of(hc) {
                    break Some(x);
                }
                h = hc[6];
                steps += 1;
                if steps > rows.len() {
                    break None;
                }
            };
            if let Some(head) = head {
                out.insert(me, (head, c[7].to_string()));
            }
        }
    }
    out
}

fn compare(args: &[String]) {
    let mut ours = BTreeMap::new();
    let mut files: Vec<_> = fs::read_dir(&args[0]).expect("out dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
    files.sort();
    for f in files {
        ours.extend(heads_by_id(&fs::read_to_string(f).expect("ours"), "MId="));
    }
    for gold_path in &args[1..] {
        let gold = heads_by_id(&fs::read_to_string(gold_path).expect("gold"), "Ref[MACULA]=");
        let (mut n, mut uas, mut las, mut las_map) = (0usize, 0usize, 0usize, 0usize);
        let mut confusion: BTreeMap<(String, String), usize> = BTreeMap::new();
        let base = |r: &str| r.split(':').next().unwrap_or(r).to_string();
        for (id, (gh, gr)) in &gold {
            let Some((oh, or)) = ours.get(id) else { continue };
            n += 1;
            if gh == oh {
                uas += 1;
                let (g, o) = (base(gr), base(or));
                if g == o {
                    las += 1;
                }
                if g == o {
                    las_map += 1;
                } else {
                    *confusion.entry((gr.clone(), or.clone())).or_default() += 1;
                }
            }
        }
        let pc = |x: usize| 100.0 * x as f64 / n.max(1) as f64;
        let _ = las_map;
        println!("{gold_path}: {n} words matched of {} in gold; UAS {:.2}  LAS {:.2}", gold.len(), pc(uas), pc(las));
        let mut c: Vec<_> = confusion.into_iter().collect();
        c.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        println!("  right head, other relation (gold → ours): {}", c.iter().take(12).map(|((g, o), k)| format!("{g}→{o} {k}")).collect::<Vec<_>>().join(", "));
    }
}

const LESSON_RULES: &str = "You write one lesson of the Torah course of a Judaism school for a bright child of 9 or 10 who reads English well. The lesson is about one weekly portion (parashah) of the Torah; you get its whole text in the JPS 1917 translation, verse by verse, with its seven readings (aliyot) marked.

Write the lesson in exactly this line format (no other lines, no blank lines, no markdown):
title: <the parashah's name>: <what it is about, in a few words>
explain: … (4 or 5 lines: what happens in the portion, in order, in plain words; where it begins and ends)
fact: … (11 to 14 lines: who, what, where, the commandments or laws given, numbers, names, what the tradition keeps from it — each one true to the text, with chapter and verse where it helps, e.g. (Genesis 12:1))
exercise: <question> | *<right answer> | <wrong> | <wrong> | <wrong>   (15 to 18 lines)
test: <question> | *<right answer> | <wrong> | <wrong> | <wrong>   (10 to 12 lines, in casual talk, as friends would ask)
exam: <question> | *<right answer> | <wrong> | <wrong> | <wrong>   (4 lines, in stiff formal exam language)
key: <verse, e.g. GEN.12.1> | <the tradition's reading of this verse, one or two plain sentences, stated as the tradition's reading>   (3 lines: the three verses that matter most in this portion)

Rules: the right answer first in each question, marked with *; three wrong answers, plausible to a child (common misreadings make good wrong answers); everything true to the text; what the tradition or the commentators read into the text is said as their reading (\"the rabbis teach\", \"Rashi explains\"), never as plain fact; contested points named as contested; respectful to Jews and to every faith; no tabs.";

fn lessons_requests(trees: &str, pool: &str, parashot: &str, work: &Path) {
    let en = english_verses(pool);
    let he = hebrew_verses(trees);
    fs::create_dir_all(work).expect("work dir");
    let rows: Vec<Vec<String>> = fs::read_to_string(parashot).expect("parashot").lines().skip(1).map(|l| l.split('\t').map(String::from).collect()).filter(|c: &Vec<String>| c.len() >= 6).collect();
    let mut lines = Vec::new();
    let mut k = 0usize;
    for whole in rows.iter().filter(|c| c[2] == "whole") {
        let (lo, hi) = (verse_key(&whole[4]), verse_key(&whole[5]));
        let aliyot: Vec<&Vec<String>> = rows.iter().filter(|c| c[0] == whole[0] && c[2] == "aliyah").collect();
        let mut body = format!("{LESSON_RULES}\n\nPARASHAH: {} ({} {} to {})\n", whole[0], whole[1], whole[4], whole[5]);
        let mut verses: Vec<(&String, &String)> = en.iter().filter(|(v, _)| (lo..=hi).contains(&verse_key(v))).collect();
        verses.sort_by_key(|(v, _)| verse_key(v));
        for (v, text) in &verses {
            if let Some(a) = aliyot.iter().find(|a| &a[4] == *v) {
                body.push_str(&format!("\n[reading {}]\n", a[3]));
            }
            body.push_str(&format!("{v} {text}\n"));
        }
        body.push_str("\nWrite the lesson now.");
        let model = mova_teacher::model("MACULA_MODEL");
        lines.push(json!({"custom_id": format!("{:03}-{}", k + 1, whole[0].to_lowercase().replace(['\'', ' '], "-")), "params": {"model": model, "max_tokens": 16000, "messages": [{"role": "user", "content": [{"type": "text", "text": body}]}], "output_config": {"effort": "medium"}}}).to_string());
        k += 1;
    }
    fs::write(work.join("batch-requests.jsonl"), lines.join("\n") + "\n").expect("write requests");
    let missing_he = en.keys().filter(|v| !he.contains_key(*v)).count();
    println!("{k} lessons in {} requests → {} ({} English verses; {missing_he} with no one-verse Hebrew sentence)", lines.len(), work.display(), en.len());
}

fn lessons_post(work: &Path, trees: &str, pool: &str, school: &Path) {
    let en = english_verses(pool);
    let he = hebrew_verses(trees);
    let dir = school.join("torah");
    fs::create_dir_all(&dir).expect("course dir");
    let (mut written, mut bad) = (0usize, Vec::new());
    for l in fs::read_to_string(work.join("batch-results.jsonl")).expect("results").lines() {
        let v: serde_json::Value = serde_json::from_str(l).expect("json");
        let id = v["custom_id"].as_str().unwrap_or("").to_string();
        if v["result"]["type"] != "succeeded" {
            bad.push(format!("{id}: request failed"));
            continue;
        }
        let text: String = v["result"]["message"]["content"].as_array().into_iter().flatten().filter_map(|b| b["text"].as_str()).collect();
        let n: usize = id[..3].parse().unwrap_or(0);
        let mut out = format!("course: torah\nlesson: {n}\n");
        let mut count: BTreeMap<&str, usize> = BTreeMap::new();
        let mut keys = Vec::new();
        for line in text.lines().map(str::trim).filter(|x| !x.is_empty()) {
            let Some((k, rest)) = line.split_once(": ") else { continue };
            match k {
                "title" | "explain" | "fact" | "exercise" | "test" | "exam" => {
                    if k == "title" {
                        out.push_str(&format!("title: {rest}\nlevel: judaism\nstars: 1\n"));
                    } else if matches!(k, "exercise" | "test" | "exam") {
                        // the right answer comes first with its «*»; the dictionary builder's `lessons mark` wants it first and bare, and
                        // moves it to a drawn place with the mark
                        let mut parts: Vec<String> = rest.split(" | ").map(String::from).collect();
                        if parts.len() < 3 || !parts[1].starts_with('*') || parts.iter().skip(2).any(|o| o.starts_with('*')) {
                            *count.entry("bad question").or_default() += 1;
                            continue;
                        }
                        parts[1] = parts[1][1..].trim_start().to_string();
                        out.push_str(&format!("{k}: {}\n", parts.join(" | ")));
                    } else {
                        out.push_str(&format!("{k}: {rest}\n"));
                    }
                    *count.entry(k).or_default() += 1;
                }
                "key" => keys.push(rest.to_string()),
                _ => {}
            }
        }
        // the key verses in layers, from the texts
        for kline in &keys {
            let Some((verse, idea)) = kline.split_once(" | ") else { continue };
            let verse = verse.trim();
            let (Some(e), Some((h, w))) = (en.get(verse), he.get(verse)) else { continue };
            let (b, cv) = verse.split_once('.').unwrap_or((verse, ""));
            let book = match b { "GEN" => "Genesis", "EXO" => "Exodus", "LEV" => "Leviticus", "NUM" => "Numbers", _ => "Deuteronomy" };
            out.push_str(&format!("fact: {book} {} in Hebrew: {h} — word for word: {w} — in English (JPS 1917): {e} — the idea: {}\n", cv.replacen('.', ":", 1), idea.trim()));
            *count.entry("layers").or_default() += 1;
        }
        let ok = count.get("title") == Some(&1) && count.get("explain").copied().unwrap_or(0) >= 3 && count.get("fact").copied().unwrap_or(0) >= 8 && count.get("exercise").copied().unwrap_or(0) >= 10 && count.get("test").copied().unwrap_or(0) >= 6 && count.get("exam").copied().unwrap_or(0) >= 2;
        if !ok {
            bad.push(format!("{id}: {count:?}"));
            continue;
        }
        fs::write(dir.join(format!("{id}.lesson")), out).expect("write lesson");
        written += 1;
    }
    println!("{written} lessons → {}; {} not written", dir.display(), bad.len());
    for b in bad {
        println!("  {b}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|a| a == "prag") && args.len() >= 3 && args.len() % 2 == 1 {
        let (mut with, mut without): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
        for pair in args[3..].chunks(2) {
            let list = pair[1].split(',');
            match pair[0].as_str() {
                "--with" => with.extend(list),
                "--without" => without.extend(list),
                o => {
                    eprintln!("usage: macula prag <trees-dir> <out.tsv> [--with rule,…] [--without rule,…]; not «{o}»");
                    std::process::exit(2);
                }
            }
        }
        if let Some(bad) = with.iter().chain(&without).find(|r| !prag::RULES.contains(r)) {
            eprintln!("no rule «{bad}»; the rules: {}", prag::RULES.join(", "));
            std::process::exit(2);
        }
        return prag::prag(&args[1], &args[2], &prag::Rules::new(&with, &without));
    }
    if args.first().is_some_and(|a| a == "prag-quotes") && args.len() == 2 {
        return prag::quotes(&args[1]);
    }
    if args.first().is_some_and(|a| a == "prag-compare") && args.len() == 5 {
        return prag::compare(&args[1], &args[2], &args[3], &args[4]);
    }
    if args.first().is_some_and(|a| a == "lessons") {
        match (args.get(1).map(String::as_str), args.len()) {
            (Some("requests"), 6) => return lessons_requests(&args[2], &args[3], &args[4], Path::new(&args[5])),
            (Some("post"), 6) => return lessons_post(Path::new(&args[2]), &args[3], &args[4], Path::new(&args[5])),
            _ => {
                eprintln!("usage: macula lessons requests <trees-dir> <en-pool.tsv> <parashot.tsv> <work-dir> | post <work-dir> <trees-dir> <en-pool.tsv> <school-dir>");
                std::process::exit(2);
            }
        }
    }
    if args.first().is_some_and(|a| a == "targums") && args.len() == 3 {
        return tanakh::targums(&args[1], &args[2]);
    }
    if args.first().is_some_and(|a| a == "tanakh3") && args.len() == 4 {
        return tanakh::index(&args[1], &args[2], &args[3]);
    }
    if args.first().is_some_and(|a| a == "compare") && args.len() >= 3 {
        return compare(&args[1..]);
    }
    if args.len() < 2 {
        eprintln!("usage: macula <lowfat-dir> <out-dir> [BOOK,…]");
        std::process::exit(2);
    }
    let (dir, out) = (Path::new(&args[0]), Path::new(&args[1]));
    let only: Vec<String> = args.get(2).map(|x| x.split(',').map(String::from).collect()).unwrap_or_default();
    fs::create_dir_all(out).expect("out dir");
    let mut files: Vec<_> = fs::read_dir(dir).expect("lowfat dir").flatten().map(|e| e.path()).filter(|p| p.to_string_lossy().ends_with("-lowfat.xml")).collect();
    files.sort();
    // Hebrew (hbo) and Aramaic (arc, `<out-dir>/arc/`) apart: a sentence goes by the language of most of its words
    let mut books: BTreeMap<String, String> = BTreeMap::new();
    let mut verses = String::new();
    let mut n_of: BTreeMap<String, usize> = BTreeMap::new();
    let mut arc_books: BTreeMap<String, String> = BTreeMap::new();
    let mut arc_verses = String::new();
    let mut arc_n_of: BTreeMap<String, usize> = BTreeMap::new();
    let (mut arc_sents, mut arc_words, mut mixed) = (0usize, 0usize, 0usize);
    let (mut sents, mut words, mut fallbacks, mut bad) = (0usize, 0usize, 0usize, 0usize);
    let mut rels: BTreeMap<&str, usize> = BTreeMap::new();
    // files are named by book order, then chapter: 01-Gen-001 … so reading order within a book holds
    for f in files {
        for s in read_book(&fs::read_to_string(&f).expect("xml")) {
            let book = s.id.split_whitespace().next().unwrap_or("").to_string();
            if s.words.is_empty() || (!only.is_empty() && !only.contains(&book)) {
                continue;
            }
            let (heads, fb) = tree(&s);
            if let Err(e) = check(&heads) {
                eprintln!("{}: {e} — left out", s.id);
                bad += 1;
                continue;
            }
            let (s, heads) = surface_order(s, heads);
            let n_arc = s.words.iter().filter(|w| w.aramaic).count();
            if n_arc > 0 && n_arc < s.words.len() {
                mixed += 1;
            }
            if 2 * n_arc > s.words.len() {
                let n = arc_n_of.entry(book.clone()).or_default();
                let id = format!("wlc-arc:{book}:{n}");
                *n += 1;
                let mut vs: Vec<String> = s.words.iter().map(|w| verse_of(&w.wref)).collect();
                vs.dedup();
                arc_verses.push_str(&format!("{id}\t{}\n", vs.join(" ")));
                arc_books.entry(book).or_default().push_str(&conllu(&s, &id, &heads));
                arc_sents += 1;
                arc_words += s.words.len();
                continue;
            }
            let n = n_of.entry(book.clone()).or_default();
            let id = format!("wlc:{book}:{n}");
            *n += 1;
            let mut vs: Vec<String> = s.words.iter().map(|w| verse_of(&w.wref)).collect();
            vs.dedup();
            verses.push_str(&format!("{id}\t{}\n", vs.join(" ")));
            books.entry(book).or_default().push_str(&conllu(&s, &id, &heads));
            sents += 1;
            words += s.words.len();
            fallbacks += fb;
            for h in &heads {
                *rels.entry(h.1).or_default() += 1;
            }
        }
    }
    for (b, t) in &books {
        fs::write(out.join(format!("{b}.conllu")), t).expect("write book");
    }
    fs::write(out.join("verses.tsv"), verses).expect("write verses");
    if !arc_books.is_empty() {
        let arc = out.join("arc");
        fs::create_dir_all(&arc).expect("arc dir");
        for (b, t) in &arc_books {
            fs::write(arc.join(format!("{b}.conllu")), t).expect("write book");
        }
        fs::write(arc.join("verses.tsv"), arc_verses).expect("write verses");
    }
    println!("{sents} sentences, {words} words in {} books → {}; {bad} trees failed the check; {fallbacks} groups had no rule for their head (first child)", books.len(), out.display());
    println!("Aramaic (arc, apart in arc/): {arc_sents} sentences, {arc_words} words in {} books; {mixed} sentences mix the two languages (they go by the most of their words)", arc_books.len());
    let mut r: Vec<(&&str, &usize)> = rels.iter().collect();
    r.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("relations: {}", r.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
}

#[cfg(test)]
mod tests {
    use super::*;

    const GEN_1_1: &str = r#"<chapter id="GEN 1"><sentence id="GEN 1:1"><p>x</p>
<wg class="cl" rule="PP-V-S-O">
 <wg role="pp" class="pp" rule="PrepNp"><w class="prep" ref="GEN 1:1!1" lemma="b">בְּ</w><w class="noun" ref="GEN 1:1!1" after=" ">רֵאשִׁית</w></wg>
 <w role="v" class="verb" ref="GEN 1:1!2" after=" ">בָּרָא</w>
 <w role="s" class="noun" ref="GEN 1:1!3" after=" ">אֱלֹהִים</w>
 <wg role="o" class="np" rule="NpaNp">
  <wg class="np" rule="OmpNP"><w class="om" ref="GEN 1:1!4" after=" ">אֵת</w><wg class="np" rule="DetNP"><w class="art" ref="GEN 1:1!5">הַ</w><w class="noun" ref="GEN 1:1!5" after=" ">שָּׁמַיִם</w></wg></wg>
  <w class="cj" ref="GEN 1:1!6">וְ</w>
  <wg class="np" rule="OmpNP"><w class="om" ref="GEN 1:1!6" after=" ">אֵת</w><wg class="np" rule="DetNP"><w class="art" ref="GEN 1:1!7">הָ</w><w class="noun" ref="GEN 1:1!7">אָרֶץ</w></wg></wg>
 </wg>
</wg></sentence></chapter>"#;

    #[test]
    fn genesis_1_1() {
        let s = read_book(GEN_1_1);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].words.len(), 11);
        let (h, fb) = tree(&s[0]);
        assert_eq!(fb, 0);
        assert_eq!(check(&h), Ok(()));
        // 1 בְּ 2 רֵאשִׁית 3 בָּרָא 4 אֱלֹהִים 5 אֵת 6 הַ 7 שָּׁמַיִם 8 וְ 9 אֵת 10 הָ 11 אָרֶץ
        let want = [(2, "case"), (3, "obl"), (0, "root"), (3, "nsubj"), (7, "case"), (7, "det"), (3, "obj"), (11, "cc"), (11, "case"), (11, "det"), (7, "conj")];
        assert_eq!(h, want);
    }

    fn one(body: &str) -> (Sent, Vec<(usize, &'static str)>) {
        let xml = format!(r#"<chapter id="GEN 1"><sentence id="GEN 1:1"><p>x</p>{body}</sentence></chapter>"#);
        let mut s = read_book(&xml);
        let s = s.remove(0);
        let (h, _) = tree(&s);
        assert_eq!(check(&h), Ok(()));
        (s, h)
    }

    #[test]
    fn surface_order_and_aramaic() {
        // «and went a man from Bethlehem»: MACULA's group puts the subject (word 2) after the place (word 3)
        let (s, h) = one(r#"<wg class="cl" rule="V-PP-S"><w role="v" class="verb" xml:id="o080010010011" after=" ">הָלַךְ</w>
            <wg role="pp" class="pp" rule="PrepNp"><w class="prep" xml:id="o080010010031">מִ</w><w class="noun" xml:id="o080010010032" after=" ">בֵּית</w></wg>
            <w role="s" class="noun" xml:id="o080010010021" lang="A" after=" ">אִישׁ</w></wg>"#);
        assert_eq!(s.words.iter().map(|w| w.form.as_str()).collect::<Vec<_>>(), ["הָלַךְ", "מִ", "בֵּית", "אִישׁ"]);
        let (t, g) = surface_order(s, h.clone());
        assert_eq!(t.words.iter().map(|w| w.form.as_str()).collect::<Vec<_>>(), ["הָלַךְ", "אִישׁ", "מִ", "בֵּית"]);
        // the same arcs, renumbered: אִישׁ is still the verb's subject and מִ still the case of בֵּית
        assert_eq!(h, [(0, "root"), (3, "case"), (1, "obl"), (1, "nsubj")]);
        assert_eq!(g, [(0, "root"), (1, "nsubj"), (4, "case"), (1, "obl")]);
        assert_eq!(check(&g), Ok(()));
        assert_eq!(t.words.iter().map(|w| w.aramaic).collect::<Vec<_>>(), [false, true, false, false]);
        // negative control: words already in order come back as they were
        let (u, k) = surface_order(t, g.clone());
        assert_eq!((u.words[1].form.as_str(), k), ("אִישׁ", g));
    }

    #[test]
    fn number_before_noun() {
        // «seven days»: the noun heads, the number is its nummod (the first version took the number as the head)
        let (_, h) = one(r#"<wg class="cl" rule="V-O"><w role="v" class="verb" after=" ">עָשָׂה</w>
            <wg role="o" class="np" rule="NumpNP"><w class="num" after=" ">שִׁבְעַת</w><w class="noun">יָמִים</w></wg></wg>"#);
        assert_eq!(h, [(0, "root"), (3, "nummod"), (1, "obj")]);
    }

    #[test]
    fn clause_before_the_head_clause() {
        // «behold, the goats [are] striped»: the later clause heads, הִנֵּה is its discourse
        let (_, h) = one(r#"<wg class="cl" rule="ClCl2"><wg class="cl" rule="Intj2CL"><w class="ij" after=" ">הִנֵּה</w></wg>
            <wg class="cl" rule="S-P"><w role="s" class="noun" after=" ">עַתֻּדִים</w><w role="p" class="adj">עֲקֻדִּים</w></wg></wg>"#);
        assert_eq!(h, [(3, "discourse"), (3, "nsubj"), (0, "root")]);
        // negative control: in a plain «ClCl» the first clause still heads
        let (_, h) = one(r#"<wg class="cl" rule="ClCl"><wg class="cl" rule="Intj2CL"><w class="ij" after=" ">הִנֵּה</w></wg>
            <wg class="cl" rule="S-P"><w role="s" class="noun" after=" ">עַתֻּדִים</w><w role="p" class="adj">עֲקֻדִּים</w></wg></wg>"#);
        assert_eq!(h[0], (0, "root"));
    }

    #[test]
    fn copula() {
        // «and the land shall be a desolation»: הָיָה with a predicate MACULA marks `o` is a copula, the predicate heads
        let (_, h) = one(r#"<wg class="cl" rule="V-S-O"><w role="v" class="verb" lemma="הָיָה" type="weqatal" after=" ">הָיְתָה</w>
            <w role="s" class="noun" after=" ">הָאָרֶץ</w><w role="o" class="noun">שְׁמָמָה</w></wg>"#);
        assert_eq!(h, [(3, "cop"), (3, "nsubj"), (0, "root")]);
        // with a phrase «ל + noun» («shall be for a sign») the phrase heads
        let (_, h) = one(r#"<wg class="cl" rule="V-PP"><w role="v" class="verb" lemma="הָיָה" type="weqatal" after=" ">הָיָה</w>
            <wg role="pp" class="pp" rule="PrepNp"><w class="prep" lemma="לְ">לְ</w><w class="noun">אוֹת</w></wg></wg>"#);
        assert_eq!(h, [(3, "cop"), (3, "case"), (0, "root")]);
        // negative control: the narrative וַיְהִי («and it came to pass in …») stays the head
        let (_, h) = one(r#"<wg class="cl" rule="V-PP"><w role="v" class="verb" lemma="הָיָה" type="wayyiqtol" after=" ">יְהִי</w>
            <wg role="pp" class="pp" rule="PrepNp"><w class="prep" lemma="בְּ">בְּ</w><w class="noun">עֵת</w></wg></wg>"#);
        assert_eq!(h, [(0, "root"), (3, "case"), (1, "obl")]);
    }

    #[test]
    fn saying() {
        // «and he spoke …, saying:» with the quote in the next sentence: the ל of לֵאמֹר heads, אמר is fixed
        let (_, h) = one(r#"<wg><w class="cj" lemma="וְ">וַ</w><wg class="cl" rule="V-S-PP"><w role="v" class="verb" lemma="דָּבַר" after=" ">יְדַבֵּר</w>
            <w role="s" class="noun" after=" ">יְהוָה</w>
            <wg role="pp" class="pp" rule="PrepCL"><w class="prep" lemma="לְ">לֵּ</w><wg class="cl" rule="V2CL"><w role="v" class="verb" lemma="אָמַר" type="infinitive construct">אמֹר</w></wg></wg></wg></wg>"#);
        assert_eq!(h, [(2, "cc"), (0, "root"), (2, "nsubj"), (2, "ccomp"), (4, "fixed")]);
    }

    #[test]
    fn lists() {
        for r in ["NpNpNp", "2PpaPp", "NP3NP", "PpPp", "Conj4Np", "NpaNp", "2Np"] {
            assert!(list_rule(r), "{r}");
        }
        // negative control: a head with a modifier is no list
        for r in ["NpPp", "2Pp", "NPofNP", "Np-Appos", "NumpNP", "PrepNp", "NpRelp"] {
            assert!(!list_rule(r), "{r}");
        }
    }

    #[test]
    fn valid_conllu() {
        // a swallowed article (בַּ = בְּ + הַ), a paseq, a stem with a space, a name of two words
        let (s, h) = one(r#"<wg class="cl" rule="V-PP-S"><w role="v" class="verb" stem="qal passive" unicode="יְמָאֵן ׀" after="׀">יְמָאֵן</w>
            <wg role="pp" class="pp" rule="PrepNp"><w class="prep" lemma="בְּ">בַּ</w><wg class="np" rule="DetNP"><w class="art" lemma="הַ"></w><w class="noun" after=" ">מָּקוֹם</w></wg></wg>
            <w role="s" class="noun" type="proper" lemma="בְּאֵר שֶׁבַע" unicode="בְּאֵר שָׁבַע" after="׃">בְּאֵר שָׁבַע</w></wg>"#);
        let out = conllu(&s, "t:1", &h);
        let rows: Vec<Vec<&str>> = out.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).map(|l| l.split('\t').collect()).collect();
        assert!(rows.iter().all(|c| c.len() == 10 && !c[1].contains(' ') && !c[2].contains(' ') && !c[1].is_empty()));
        assert!(rows.iter().all(|c| c[5] == "_" || c[5].split('|').all(|f| f.split_once('=').is_some_and(|(_, v)| v.chars().all(|x| x.is_ascii_alphanumeric())))));
        assert!(out.contains("HebBinyan=QALPASSIVE"));
        // the article is written ה inside the multiword token בַּמָּקוֹם; the paseq and the sof pasuq are marks
        assert!(rows.iter().any(|c| c[0].contains('-') && c[1] == "בַּמָּקוֹם".nfc().collect::<String>()));
        assert!(rows.iter().any(|c| c[1] == "ה" && c[7] == "det"));
        assert!(rows.iter().any(|c| c[1] == "׀" && c[7] == "punct") && rows.iter().any(|c| c[1] == "׃" && c[7] == "punct"));
        assert!(rows.iter().any(|c| c[1] == "שָׁבַע".nfc().collect::<String>() && c[7] == "flat:name"));
        // the text is the written tokens
        let text = out.lines().find_map(|l| l.strip_prefix("# text = ")).unwrap();
        assert_eq!(text, "יְמָאֵן ׀ בַּמָּקוֹם בְּאֵר שָׁבַע׃".nfc().collect::<String>());
    }

    #[test]
    fn aramaic() {
        // «this king»: an Aramaic sentence, its demonstrative a det; the one Hebrew word is marked Lang=hbo, the Aramaic ones Lang=arc
        let (s, h) = one(r#"<wg class="np" rule="NpAdjp"><w class="noun" lang="A" lemma="מֶלֶךְ" after=" ">מַלְכָּא</w><w class="adj" lang="A" lemma="דְּנָה" after=" ">דְנָה</w><w class="noun" lang="H" lemma="אִישׁ">אִישׁ</w></wg>"#);
        assert_eq!(sent_lang(&s), "A");
        assert_eq!(h[1], (1, "det"));
        let out = conllu(&s, "t:1", &h);
        assert!(out.contains("Lang=hbo") && out.contains("Lang=arc"));
        // negative control: the same lemma in a Hebrew word is no demonstrative, and a Hebrew sentence stays Hebrew
        let (s, h) = one(r#"<wg class="np" rule="NpAdjp"><w class="noun" lang="H" lemma="מֶלֶךְ" after=" ">מֶלֶךְ</w><w class="adj" lang="H" lemma="דְּנָה">דְנָה</w></wg>"#);
        assert_eq!(sent_lang(&s), "H");
        assert_ne!(h[1].1, "det");
    }

    #[test]
    fn check_shows_red() {
        // negative control: two roots, a cycle, a head out of range
        assert!(check(&[(0, "root"), (0, "root")]).is_err());
        assert!(check(&[(0, "root"), (3, "dep"), (2, "dep")]).is_err());
        assert!(check(&[(0, "root"), (9, "dep")]).is_err());
        assert!(check(&[(0, "root"), (1, "dep")]).is_ok());
    }

    #[test]
    fn verses() {
        assert_eq!(verse_of("GEN 1:1!2"), "GEN.1.1");
        assert_eq!(verse_key("EXO.12.3"), (1, 12, 3));
        assert_eq!(verse_key("DEU.34.12"), (4, 34, 12));
        assert!(verse_key("GEN.50.26") < verse_key("EXO.1.1"));
    }
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn majority_tie_and_empty_parser() {
        assert!(read_book("<chapter/>").is_empty());
        let s = read_book(
            r#"<sentence id="X"><w class="noun" unicode="א" lang="A"/><w class="noun" unicode="ב" lang="H"/></sentence>"#,
        );
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].words.len(), 2);
        assert_eq!(sent_lang(&s[0]), "H");
        assert_eq!(feat_value("UNKNOWN: V"), None);
        assert_eq!(feat_value("QAL PASSIVE"), Some("QALPASSIVE".into()));
        assert_eq!(check(&[]), Err("0 roots".into()));
        assert_eq!(check(&[(1, "dep")]), Err("0 roots".into()));
        assert!(check(&[(0, "root"), (2, "dep")]).is_err());
    }
}
