//! Quantity mentions: every number of a problem with what it counts — its thing, the words that set it apart («salty
//! cookies», «bird families»), the verb it goes with, and its context («in the afternoon», «to africa») — after the
//! quantity schemas of Roy & Roth (2015, «Solving general arithmetic word problems»,
//! $MOVA_DATA/research/raw/compmath-2015-roy-solving-general-arithmetic-word-problems). A question picks the mentions it is
//! about by the words it shares with them: «how many more salty cookies than sweet cookies did he eat» picks the
//! salty cookies eaten and the sweet ones eaten, and takes one from the other.
//!
//! Deterministic: word sets are ordered, ties are refused (a question two mentions fit equally gets no answer).

use std::collections::BTreeSet;

/// One number of the problem and what it is about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mention {
    pub value: i64,
    pub thing: String,
    /// words between the number and its thing («salty», «bird»)
    pub adj: BTreeSet<String>,
    pub verb: String,
    /// the words around it in its clause, beyond the number and its thing
    pub ctx: BTreeSet<String>,
    pub sentence: usize,
    /// told in the question itself («… in 146 days?»)
    pub in_q: bool,
    /// a rate: what one of it is for («$78 off each t-shirt», «22 pages per day», «each plank needs 2 nails»)
    pub per: Option<String>,
    /// the verb and the thing as written («ate», «peaches»), for their kinds in WordNet
    pub verb_word: String,
    pub thing_word: String,
    /// whether the one who holds or acts on it (the sentence's name, or the name a pronoun stands for) is the one the
    /// question asks about; None when either is not known
    pub holder_asked: Option<bool>,
    /// what the dependency tree says about it (`tree_feats`; the maths student reads through the tree
    /// too): «role obj», «case of», «comp», «each», «verb asked true», «holder asked true»; empty unless MOVA_TREE=1
    pub tree: Vec<String>,
}

const STOP: [&str; 58] = [
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "at", "for", "with", "from", "by", "he", "she", "it", "they", "his", "her", "their", "its", "him", "them", "is", "are", "was", "were", "be", "been", "has", "have", "had", "do", "does", "did", "how", "many", "much", "more", "than", "that", "this", "there", "if", "then", "now", "also", "already", "some", "what", "who", "which", "will", "would", "can", "could", "those",
];

fn stop(w: &str) -> bool {
    STOP.contains(&w)
}

/// A word's stem for matching: irregular verbs to their base, then the endings cut («received» ~ «receive»,
/// «ate» ~ «eat», «cookies» ~ «cookie»).
pub fn stem(w: &str) -> String {
    let w = w.to_lowercase();
    let base = match w.as_str() {
        "ate" | "eaten" => "eat", "gave" | "given" => "give", "got" | "gotten" => "get", "found" => "find", "lost" => "lose", "bought" => "buy", "sold" => "sell",
        "made" => "make", "spent" => "spend", "took" | "taken" => "take", "grew" | "grown" => "grow", "threw" | "thrown" => "throw", "flew" | "flown" => "fly",
        "won" => "win", "ran" => "run", "drove" | "driven" => "drive", "wrote" | "written" => "write", "read" => "read", "put" => "put", "cut" => "cut",
        "left" => "leave", "went" | "gone" => "go", "came" => "come", "saw" | "seen" => "see", "broke" | "broken" => "break", "children" => "child", "people" => "person",
        "men" => "man", "women" => "woman", "feet" => "foot", "teeth" => "tooth", "mice" => "mouse", "geese" => "goose", "leaves" => "leaf",
        other => other,
    };
    let mut s = base.to_string();
    for suf in ["ing", "ies", "ed", "es", "s", "e"] {
        if s.len() > suf.len() + 2 && s.ends_with(suf) {
            s.truncate(s.len() - suf.len());
            if suf == "ies" {
                s.push('y');
            }
            break;
        }
    }
    s
}

fn words(text: &str) -> Vec<String> {
    // «$» is a word of its own («$ 99», «$99»); a hyphen splits («t-shirt»: joining them measured worse, 326 against
    // 340 of 700 in the two-fold check)
    text.replace('$', " $ ").split(|c: char| !(c.is_alphanumeric() || c == '$')).map(|w| w.trim_matches('-')).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect()
}

fn is_num(w: &str) -> Option<i64> {
    w.parse().ok()
}

/// A word that is a verb more likely than not: a known form of being or having, a past form, or a form the lexicon
/// tags only as a verb in its first sense (kept small and plain).
fn verbish(w: &str) -> bool {
    const VERBS: [&str; 40] = ["has", "have", "had", "is", "are", "was", "were", "ate", "eat", "eats", "gave", "give", "gives", "got", "get", "gets", "found", "find", "finds", "lost", "lose", "loses", "bought", "buy", "buys", "sold", "sell", "sells", "made", "make", "makes", "spent", "spend", "spends", "took", "take", "takes", "grew", "used", "uses"];
    VERBS.contains(&w) || (w.len() > 4 && w.ends_with("ed")) || matches!(w, "received" | "receive" | "receives" | "collected" | "picked" | "baked" | "flew" | "threw" | "won" | "scored" | "needs" | "wants" | "saw" | "drank" | "drew" | "wrote" | "read" | "ran" | "swam")
}

/// The mentions of a problem's story (the question's own numbers are left out).
pub fn mentions(body: &str) -> Vec<Mention> {
    read_mentions(body, false)
}

/// The mentions of the whole problem, the question's numbers too (marked).
pub fn mentions_all(text: &str) -> Vec<Mention> {
    read_mentions(text, true)
}

/// The counted facts the problem leans on without saying them (`global::measure`): «2 hours … how many minutes» brings
/// in 60 minutes per hour. A fact comes in only when the text names its word and the question asks in its unit (a pure
/// count — a dozen, a pair — needs only its word), and not when the text already tells that number.
fn known_facts(text: &str, told: &[Mention]) -> Vec<Mention> {
    let ws: Vec<String> = text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect();
    // the unit must be what the question asks in
    let question = text.rsplit_once(['.', '!']).map(|x| x.1).unwrap_or(text);
    let qs: Vec<String> = question.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect();
    let has_unit = |u: &str| qs.iter().any(|w| w == u || w.strip_suffix('s') == Some(u) || w.strip_suffix("es") == Some(u) || (u.ends_with('y') && w.strip_suffix("ies").is_some_and(|b| format!("{b}y") == u)) || (u == "foot" && w == "feet") || (u == "inch" && w == "inches"));
    let mut out: Vec<Mention> = Vec::new();
    let mut seen_words: BTreeSet<String> = BTreeSet::new();
    for w in &ws {
        // the global level's measures; what the word's explanation states («A week is seven days») only with
        // MOVA_DICTFACTS=1 — on 09.10 the explanations' numbers cost the two-fold check 24 of 2100 (old dictionary) and
        // 63 (the rich one), and won nothing in the lessons
        let facts: Vec<(i64, String)> = {
            let g = global::measure(w);
            if g.is_empty() && std::env::var("MOVA_DICTFACTS").is_ok() { crate::meanings::facts_of(w) } else { g.iter().map(|(v, u)| (*v, u.to_string())).collect() }
        };
        if facts.is_empty() || !seen_words.insert(w.trim_end_matches('s').to_string()) {
            continue;
        }
        for (v, unit) in &facts {
            let unit = unit.as_str();
            if (!unit.is_empty() && !has_unit(unit)) || told.iter().chain(out.iter()).any(|m| m.value == *v) {
                continue;
            }
            let mut adj = BTreeSet::new();
            adj.insert("known".to_string());
            out.push(Mention { value: *v, thing: stem(unit), adj, verb: String::new(), ctx: [stem(w)].into(), sentence: usize::MAX, in_q: false, per: Some(stem(w)), verb_word: String::new(), thing_word: unit.to_string(), holder_asked: None, tree: Vec::new() });
        }
    }
    out
}

/// A sentence's tree (the `en` annotator the world reads with), cached by text.
fn parsed(sentence: &str) -> Vec<en::annotate::Word> {
    static C: std::sync::Mutex<std::collections::BTreeMap<String, Vec<en::annotate::Word>>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    if let Some(w) = C.lock().unwrap().get(sentence) {
        return w.clone();
    }
    let w = crate::frames::parse_words(sentence);
    C.lock().unwrap().insert(sentence.to_string(), w.clone());
    w
}

/// The tree's word on each told number (on with MOVA_TREE=1; measured 15.10 on the two-fold check, base 1116: all
/// groups 1095, role 1071, case 1101, comp 1095, each 1110, verb asked 1077, holder asked 1104, role pairs 1068 — no
/// group helps the operation chooser, so it stays off): the noun it counts and that noun's role (subject,
/// object, oblique …) and preposition, a comparison («more», «fewer», «than», «times») or a rate («each», «per»,
/// «every») hanging on it, and whether its verb and its verb's subject are the question's.
fn tree_feats(body: &str, ms: &mut [Mention]) {
    if std::env::var("MOVA_TREE").as_deref() != Ok("1") {
        return;
    }
    let body = crate::numbers::normalize(body).0;
    let sentences: Vec<&str> = body.split_inclusive(['.', '!', '?']).collect();
    let base = |r: en::gram::Rel| r.base().name().to_string();
    let verb_of = |w: &[en::annotate::Word], mut i: usize| -> Option<usize> {
        for _ in 0..w.len() {
            if matches!(w[i].upos, en::gram::UPos::VERB | en::gram::UPos::AUX) || w[i].head == 0 {
                return Some(i);
            }
            i = w[i].head - 1;
        }
        None
    };
    let subj_of = |w: &[en::annotate::Word], v: usize| -> Option<String> { w.iter().find(|x| x.head == v + 1 && x.rel.base() == en::gram::Rel::Nsubj).map(|x| x.lemma.to_lowercase()) };
    // the question: its verb and its subject
    let (qverb, qsubj) = sentences.iter().rev().find(|s| s.trim_end().ends_with('?')).map(|q| {
        let w = parsed(q);
        let root = w.iter().position(|x| x.head == 0);
        (root.map(|r| w[r].lemma.to_lowercase()), root.and_then(|r| subj_of(&w, r)))
    }).unwrap_or((None, None));
    for (si, sentence) in sentences.iter().enumerate() {
        let w = parsed(sentence);
        let nums: Vec<usize> = (0..w.len()).filter(|&i| w[i].form.replace(',', "").parse::<f64>().is_ok()).collect();
        let mut used = 0usize;
        for m in ms.iter_mut().filter(|m| m.sentence == si) {
            let Some(&i) = nums.iter().skip(used).find(|&&i| w[i].form.replace(',', "").parse::<f64>().ok().is_some_and(|x| x as i64 == m.value)) else { continue };
            used = nums.iter().position(|&x| x == i).unwrap_or(used) + 1;
            // the noun it counts: its head when it is a number modifier, else itself
            let n = if w[i].rel == en::gram::Rel::Nummod && w[i].head > 0 { w[i].head - 1 } else { i };
            let mut f = vec![format!("role {}", base(w[n].rel))];
            for k in w.iter().filter(|x| x.head == n + 1 || x.head == i + 1) {
                let l = k.lemma.to_lowercase();
                if k.rel.base() == en::gram::Rel::Case {
                    f.push(format!("case {l}"));
                }
                if matches!(l.as_str(), "more" | "less" | "fewer" | "than" | "times" | "twice" | "half") {
                    f.push("comp".into());
                }
                if matches!(l.as_str(), "each" | "every" | "per" | "apiece") {
                    f.push("each".into());
                }
            }
            if let Some(v) = verb_of(&w, n) {
                f.push(format!("verb asked {}", qverb.as_deref() == Some(w[v].lemma.to_lowercase().as_str())));
                if let (Some(s), Some(q)) = (subj_of(&w, v), &qsubj) {
                    f.push(format!("holder asked {}", &s == q));
                }
            }
            f.sort();
            f.dedup();
            m.tree = f;
        }
    }
}

fn read_mentions(body: &str, with_question: bool) -> Vec<Mention> {
    let mut v = read_mentions_told(body, with_question);
    tree_feats(body, &mut v);
    if with_question && std::env::var("MOVA_NOFACTS").is_err() {
        let facts = known_facts(body, &v);
        if std::env::var("MOVA_SHOWFACTS").is_ok() && !facts.is_empty() {
            eprintln!("FACTS {:?} :: {}", facts.iter().map(|m| format!("{} {} per {:?}", m.value, m.thing, m.per)).collect::<Vec<_>>(), body.chars().take(120).collect::<String>());
        }
        v.extend(facts);
    }
    v
}

fn read_mentions_told(body: &str, with_question: bool) -> Vec<Mention> {
    let body = crate::numbers::normalize(body).0;
    // names: words written with a capital inside a sentence anywhere in the problem; they say who, not which quantity
    let names: BTreeSet<String> = body
        .split_inclusive(['.', '!', '?'])
        .flat_map(|s| s.split_whitespace().skip(1).filter(|w| w.chars().next().is_some_and(char::is_uppercase)).map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase()).collect::<Vec<_>>())
        .collect();
    let mut out: Vec<Mention> = Vec::new();
    // who a sentence is about: its first name, or, for «he», «she», «they», the name before
    let holder_in = |ws: &[String], upto: usize, last: &Option<String>| -> Option<String> {
        ws[..upto.min(ws.len())].iter().find_map(|x| if names.contains(x) { Some(x.clone()) } else if matches!(x.as_str(), "he" | "she" | "they") { last.clone() } else { None })
    };
    let asked_holder: Option<String> = {
        let mut last = None;
        let mut asked = None;
        for sentence in body.split_inclusive(['.', '!', '?']) {
            let ws = words(sentence);
            let h = holder_in(&ws, ws.len(), &last);
            if sentence.trim_end().ends_with('?') {
                asked = h.clone();
            }
            if h.is_some() {
                last = h;
            }
        }
        asked
    };
    let mut last_holder: Option<String> = None;
    for (si, sentence) in body.split_inclusive(['.', '!', '?']).enumerate() {
        let in_q = sentence.trim_end().ends_with('?');
        if in_q && !with_question {
            continue;
        }
        let w = words(sentence);
        let sentence_holder = holder_in(&w, w.len(), &last_holder);
        let mut verb = String::new();
        let mut verb_word = String::new();
        let mut last_num_end = 0;
        for i in 0..w.len() {
            if verbish(&w[i]) {
                verb = stem(&w[i]);
                verb_word = w[i].clone();
            }
            let Some(v) = is_num(&w[i]) else { continue };
            // the thing: up to the first plural after the number, or the word before «of» a measure
            let mut k = i + 1;
            let mut adj: BTreeSet<String> = BTreeSet::new();
            let mut thing = String::new();
            // «5 more birds», «3 fewer apples»: the comparison word stands before the thing it counts
            let mut compared = false;
            while k < w.len() && matches!(w[k].as_str(), "more" | "fewer" | "less" | "new" | "other" | "extra") {
                compared |= w[k] != "new" && w[k] != "other" && w[k] != "extra";
                k += 1;
            }
            if compared {
                adj.insert("more".into());
            }
            while k < w.len() && k <= i + 4 && is_num(&w[k]).is_none() && !stop(&w[k]) && !verbish(&w[k]) && !matches!(w[k].as_str(), "each" | "per" | "every" | "off" | "apiece") {
                let s = stem(&w[k]);
                if !thing.is_empty() {
                    adj.insert(thing.clone());
                }
                thing = s;
                let plural = w[k].ends_with('s') && !w[k].ends_with("ss");
                k += 1;
                if plural {
                    break;
                }
            }
            // «19 of the books», «301 of her cards»: the thing named after «of» and a determiner («15 of them» and a thing
            // left out stay without one: taking the thing of the number before measured worse, 368 against 377 of 700)
            if thing.is_empty() && w.get(k).is_some_and(|x| x == "of") {
                let mut j = k + 1;
                while j < w.len() && matches!(w[j].as_str(), "the" | "her" | "his" | "their" | "its" | "my" | "our" | "your" | "these" | "old" | "new") {
                    j += 1;
                }
                if let Some(t) = w.get(j).filter(|x| !matches!(x.as_str(), "them" | "those" | "it" | "these") && is_num(x).is_none() && !stop(x) && !verbish(x)) {
                    thing = stem(t);
                    k = j + 1;
                }
            }
            // «$ 99 off each jersey»: money
            if thing.is_empty() && i > 0 && w[i - 1] == "$" {
                thing = "dollar".into();
            }
            // «cups of flour», «pieces of candy», «bottles of soda»: the measured thing is named after «of»
            if w.get(k).is_some_and(|x| x == "of") && k + 1 < w.len() && is_num(&w[k + 1]).is_none() {
                adj.insert(thing.clone());
                let mut j = k + 1;
                while j < w.len() && j <= k + 3 && is_num(&w[j]).is_none() && !stop(&w[j]) && !verbish(&w[j]) {
                    if j > k + 1 {
                        adj.insert(thing.clone());
                    }
                    thing = stem(&w[j]);
                    j += 1;
                }
                k = j;
            }
            // the context: the words after the thing, up to the next number
            let next = (k..w.len()).find(|&j| is_num(&w[j]).is_some()).unwrap_or(w.len());
            // the words before belong to the first number of a sentence only; after it, they close the number before
            // (and of those, only the words after its verb: the subject says who, not which quantity)
            let from = if last_num_end == 0 { (0..i).rev().find(|&j| verbish(&w[j])).map(|j| j + 1).unwrap_or(0) } else { i };
            let before = &w[from..i];
            let ctx: BTreeSet<String> = before.iter().chain(w[k..next].iter()).filter(|x| !stop(x) && !verbish(x) && !names.contains(*x)).map(|x| stem(x)).collect();
            // the rate: «each / per / every / a» and a noun after the thing (up to the next number), or opening the
            // clause before the number («Each aquarium has 39 animals»)
            let unit_after = |from: usize, to: usize| -> Option<String> {
                (from..to.min(w.len())).find(|&j| matches!(w[j].as_str(), "each" | "per" | "every" | "a" | "an") && w.get(j + 1).is_some_and(|n| !stop(n) && is_num(n).is_none() && !verbish(n))).map(|j| stem(&w[j + 1]))
            };
            let clause_start = (0..i).rev().find(|&j| is_num(&w[j]).is_some()).map(|j| j + 1).unwrap_or(0);
            let per = unit_after(k, next).filter(|u| *u != thing).or_else(|| unit_after(clause_start, i).filter(|u| *u != thing));
            let thing_word = (i + 1..k.min(w.len())).rev().map(|j| w[j].clone()).find(|x| stem(x) == thing).unwrap_or_default();
            let holder = holder_in(&w, i, &last_holder).or_else(|| sentence_holder.clone());
            let holder_asked = holder.as_ref().zip(asked_holder.as_ref()).map(|(a, b)| a == b);
            out.push(Mention { value: v, thing, adj, verb: verb.clone(), ctx, sentence: si, in_q, per, verb_word: verb_word.clone(), thing_word, holder_asked, tree: Vec::new() });
            last_num_end = k;
        }
        if sentence_holder.is_some() {
            last_holder = sentence_holder;
        }
    }
    out
}

/// How well a mention fits a description: the thing counts most, then the words that set it apart, then the verb;
/// the context counts only with the words that belong to this part of the question (`own`), not with those the two
/// parts share (a shared «does the recipe call for» tells neither part apart). A mention whose own distinguishing
/// words the description does not have loses a little (it is about something else).
fn fit(m: &Mention, own: &BTreeSet<String>, shared: &BTreeSet<String>) -> i64 {
    let d = |w: &String| own.contains(w) || shared.contains(w);
    let mut s = 0;
    if d(&m.thing) {
        s += 4;
    }
    for a in &m.adj {
        s += if d(a) { 3 } else { -1 };
    }
    if !m.verb.is_empty() && d(&m.verb) {
        s += 2;
    }
    for c in &m.ctx {
        if own.contains(c) {
            s += 2;
        }
    }
    s
}

/// The mention that fits a description best, if one alone does.
fn best<'a>(ms: &'a [Mention], own: &BTreeSet<String>, shared: &BTreeSet<String>, not: Option<&Mention>) -> Option<&'a Mention> {
    let mut v: Vec<(i64, &Mention)> = ms.iter().filter(|m| Some(*m) != not).map(|m| (fit(m, own, shared), m)).collect();
    v.sort_by(|a, b| b.0.cmp(&a.0));
    match v.as_slice() {
        [(a, m), (b, _), ..] if a > b && *a > 0 => Some(m),
        [(a, m)] if *a > 0 => Some(m),
        _ => None,
    }
}

/// «How many more A than B …»: the two mentions the question is about, and their difference.
pub fn more_than(body: &str, question: &str) -> Option<(i64, String)> {
    let q = words(&crate::numbers::normalize(question).0);
    let more = q.iter().position(|w| w == "more" || w == "fewer" || w == "less")?;
    let than = q.iter().position(|w| w == "than")?;
    if than < more {
        return None;
    }
    let st = |v: &[String]| -> BTreeSet<String> { v.iter().filter(|x| !stop(x)).map(|x| stem(x)).collect() };
    // B's own words: after «than» up to a verb or a word of grammar; what follows («did he eat») is shared by A and B,
    // as is what comes before «more»
    let after = &q[than + 1..];
    let np_end = after.iter().position(|w| (stop(w) && w != "of") || verbish(w)).unwrap_or(after.len());
    let np_end = if np_end == 0 { after.iter().position(|w| verbish(w) || w == "did" || w == "does" || w == "do").unwrap_or(after.len()) } else { np_end };
    let mut shared = st(&q[..more]);
    let mut a = st(&q[more + 1..than]);
    // «than he did to build the house», «than those he gave to his friends»: B is told by a clause of its own
    let clause_b = after.first().is_some_and(|w| matches!(w.as_str(), "he" | "she" | "they" | "it" | "those" | "that" | "the" | "what" | "we" | "i" | "you"));
    let mut b = if clause_b { st(after) } else { st(&after[..np_end]) };
    if !clause_b {
        shared.extend(st(&after[np_end..]));
    }
    // «more emails in the morning than in the afternoon»: B names no thing of its own, so it is A's thing in B's place
    if !b.iter().any(|w| a.contains(w)) && b.len() <= 1 {
        let a_thing: Vec<String> = q[more + 1..than].iter().take_while(|w| !stop(w) && !verbish(w)).map(|x| stem(x)).collect();
        b.extend(a_thing);
        let a_rest = st(&q[more + 1..than]);
        b.extend(a_rest.into_iter().filter(|w| verbish(w)));
    }
    let ms = mentions(body);
    let ma = best(&ms, &a, &shared, None)?;
    let mb = best(&ms, &b, &shared, Some(ma))?;
    Some(((ma.value - mb.value).abs(), format!("{} {} {:?} ({}) − {} {} {:?} ({})", ma.value, ma.thing, ma.adj, ma.verb, mb.value, mb.thing, mb.adj, mb.verb)))
}

/// «How many X (and Y) … in all / altogether / in the day»: the sum of the told quantities the question is about —
/// those of its thing, narrowed to the ones its own words pick out (a colour, a day, a kind, a verb) when it names
/// any. At least two quantities, or no answer.
pub fn sum_of(text: &str) -> Option<(i64, String)> {
    let question = text.rsplit_once(['.', '!']).map(|x| x.1).unwrap_or(text);
    let qw = words(&crate::numbers::normalize(question).0);
    let low = qw.join(" ");
    let cue = ["in all", "altogether", "total", "together", "in the day", "combined", "both", "in all"].iter().any(|c| low.contains(c)) || qw.iter().filter(|w| *w == "and").count() >= 1;
    if !cue || low.contains(" more ") || low.contains(" than ") || low.contains(" left") || low.contains(" each") {
        return None;
    }
    let q: BTreeSet<String> = qw.iter().filter(|x| !stop(x)).map(|x| stem(x)).collect();
    let ms = mentions(text);
    let mut of_thing: Vec<&Mention> = ms.iter().filter(|m| q.contains(&m.thing)).collect();
    // a verb tells them apart only where they differ («received» emails, «sent» letters)
    let verbs: BTreeSet<&str> = of_thing.iter().map(|m| m.verb.as_str()).collect();
    if verbs.len() > 1 && of_thing.iter().any(|m| q.contains(&m.verb)) {
        of_thing.retain(|m| q.contains(&m.verb));
    }
    if of_thing.len() < 2 {
        return None;
    }
    // the question's own words that tell some of them apart
    let marks = |m: &Mention| -> usize { m.adj.iter().chain(&m.ctx).filter(|w| q.contains(*w)).count() };
    let top = of_thing.iter().map(|m| marks(m)).max().unwrap_or(0);
    let some_differ = of_thing.iter().any(|m| marks(m) < top);
    let chosen: Vec<&&Mention> = if top > 0 && some_differ { of_thing.iter().filter(|m| marks(m) > 0).collect() } else { of_thing.iter().collect() };
    if chosen.len() < 2 {
        return None;
    }
    let sum: i64 = chosen.iter().map(|m| m.value).sum();
    Some((sum, chosen.iter().map(|m| format!("{} {}", m.value, m.thing)).collect::<Vec<_>>().join(" + ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_two_mentions() {
        // the problem is read whole, as in the school: the story and its question
        let ask = |b: &str, q: &str| more_than(&format!("{b} {q}"), q).map(|x| x.0);
        let b = "Paco had 8 sweet cookies and 6 salty cookies. He ate 20 sweet cookies and 34 salty cookies.";
        assert_eq!(ask(b, "How many more salty cookies than sweet cookies did he eat?"), Some(14));
        let b = "Jack received 10 emails in the morning, 5 emails in the afternoon and 4 emails in the evening.";
        assert_eq!(ask(b, "How many more emails did Jack receive in the afternoon than in the evening?"), Some(1));
        let b = "Mary is baking a cake. The recipe calls for 9 cups of sugar 14 cups of flour and 40 cups of salt.";
        assert_eq!(ask(b, "How many more cups of salt than cups of flour does the recipe call for?"), Some(26));
        let sum = |b: &str, q: &str| sum_of(&format!("{b} {q}")).map(|x| x.0);
        assert_eq!(sum("5 red peaches, 14 yellow peaches and 6 green peaches are in the basket.", "How many green and yellow peaches are in the basket?"), Some(20));
        assert_eq!(sum("Julia played tag with 5 kids on monday, 9 kids on tuesday and 15 kids on wednesday.", "How many kids did she play with on monday and wednesday?"), Some(20));
        assert_eq!(sum("Jack received 6 emails in the morning, 3 emails in the afternoon and 5 emails in the evening.", "How many emails did Jack receive in the day?"), Some(14));
        // the negative control: a question two mentions fit equally gets no answer
        let b = "Randy has 95 blocks. He uses 20 blocks to build a house and 50 blocks to build a tower.";
        assert_eq!(ask(b, "How many more blocks did he use to build the tower than he did to build the house?"), Some(30));
        let b = "Tom has 5 apples. Tom has 7 apples.";
        assert_eq!(ask(b, "How many more apples than pears does Tom have?"), None);
    }
}

/// `world mentions "<problem>"`: the mentions, and the answer to a «more … than» question.
pub fn main_args(args: &[String]) -> anyhow::Result<()> {
    let text = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    for m in mentions(&text) {
        println!("{} {} adj {:?} verb {} ctx {:?}{}", m.value, m.thing, m.adj, m.verb, m.ctx, m.per.as_ref().map(|p| format!(" per {p}")).unwrap_or_default());
    }
    let question = text.rsplit_once(['.', '!']).map(|x| x.1).unwrap_or(&text);
    println!("{:?}", more_than(&text, question));
    Ok(())
}
