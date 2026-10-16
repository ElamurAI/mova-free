//! The LLM writes world changes: prompt (change language, closed sets, gates, an example on another story) and
//! numbered fable sentences → one Opus call (`prag::opus`, high). The output is parsed; if
//! the format gate rejected lines — one retry with the same prompt and an explanation of the rejection, no more.
//! Finished commands (`cmds-<doc>.txt`) are not overwritten: a repeated run does not call the LLM again.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context as _, Result, bail};
use prag::opus::{Opus, append_call};
use prag::schema::Act;

use crate::lang::{FormatErr, parse_all};
use crate::types::*;
use crate::world::Text;

/// Example (format only; a different story, not from the run's fables).
pub const EXAMPLE_TEXT: &[(u16, &str)] = &[
    (1, "A girl lived with her grandmother in a hut at the edge of a forest."),
    (1, "One morning she climbed an apple tree in the garden and picked an apple, but it slipped from her hand and fell into the grass."),
    (1, "A hungry dog ran up, snatched the apple and ate it."),
    (2, "\"Bad dog!\" cried the girl, but her grandmother said: \"Be kind to him, and he will guard our house.\""),
    (2, "So the girl fed the dog every day, and he became her best friend."),
    (3, "Kindness wins friends."),
];

pub const EXAMPLE_CMDS: &str = r#"loc L1 type=forest ^s1
loc L2 type=house class=hut in=L1 ^s1
char C1 type=human class=girl name="the girl" at=L2 gender=female age=young ^s1
char C2 type=human class=grandmother name="the grandmother" at=L2 gender=female age=old ^s1
rel R1 kind=kin a=C1 b=C2 ^s1
loc L3 type=garden in=L1 ^s2
loc L4 type=tree class=apple_tree in=L3 ^s2
move C1 to=L4 ^s2
event E1 verb=climb agent=C1 to=L4 ^s2
obj O1 type=food class=apple ^s2
have C1 O1 ^s2
event E2 verb=pick agent=C1 patient=O1 at=L4 ^s2
event E3 verb=slip patient=O1 how="slipped from her hand" ^s2
move O1 to=L3 cause=E3 ^s2
cursor at=L3 ^s3
char C3 type=animal class=dog name="the dog" at=L3 gender=male status=hungry ^s3
event E4 verb=snatch agent=C3 patient=O1 at=L3 why="he was hungry" ^s3
move O1 to=C3 cause=E4 ^s3
event E5 verb=eat agent=C3 patient=O1 at=L3 ^s3
set O1 status=eaten cause=E5 ^s3
set C3 status=fed cause=E5 ^s3
say C1 to=C3 act=evaluate id=E6 ^s4
set C1 feeling=angry cause=E4 ^s4
rel R2 kind=adversary a=C1 b=C3 cause=E4 ^s4
say C2 to=C1 act=suggest means="feed the dog and he will protect the house" why="she wants the girl to be kind" id=E7 ^s4
event E8 verb=feed agent=C1 patient=C3 why="she followed her grandmother's advice" ^s5
set C1 feeling=happy cause=E8 ^s5
rel R2 kind=friend cause=E8 ^s5
say narrator act=assert means="being kind to others makes friends" id=E9 ^s6"#;

/// Rules of the change language for the LLM.
pub fn rules() -> String {
    let acts: Vec<&str> = Act::ALL.iter().map(|a| a.name()).collect();
    format!(
        r#"You turn a short story into commands of a world-state language. A deterministic program executes the commands and checks them with strict gates; a line that breaks a gate is rejected.

OUTPUT
- One command per line, in story order. Only command lines: no comments, explanations, headers or code fences.
- Every line ends with the anchor of its sentence: ^s<N>. A speech spanning two sentences may end with both: ^s3 ^s4. Anchors never go backwards. Every sentence gets at least one command.
- Ids: locations L1 L2 ..., objects O1 ..., characters C1 ..., relations R1 ..., events E1 ... (events and speeches share E numbers). Number them in order of appearance; create before use; never reuse an id.
- <word> is lowercase English: letters, digits, _ or - (pick_up). "text" is plain English in double quotes, at most 20 words, no double quotes inside. <words> is word or word,word.

COMMANDS
loc L<n> type=<loctype> [class=<word>] [name="text"] [in=L<m>]
  A place. Anything a character can be in or on (a nest, a boat, a bridge) is a location; put it inside the larger place with in= (a nest in a tree, a boat on a river).
obj O<n> type=<objtype> [class=<word>] [name="text"] [at=L<m>|C<m>]
  A thing; at=C<m> means the character holds or owns it. Omit at= if its place is not known yet.
char C<n> type=<chartype> class=<word> [name="text"] [at=L<m>] [<key>=<value> ...]
  A character; class = species or role (wolf, king, miller). A group acting together (the villagers) is one character.
move C<n> to=L<m> [cause=E<k>]
  A character goes to another location.
move O<n> to=L<m>|C<m> [cause=E<k>]
  An object is dropped or put down (to=L), or picked up or snatched by a character (to=C).
have C<n> O<m> [cause=E<k>]
  First possession of an object that has no place yet.
give C<n> O<m> to=C<k> [cause=E<j>]
  A character hands an object to another character.
set C<n>|O<n>|L<n> <key>=<value> [<key>=<value> ...] [cause=E<k>]
  Attributes (objects and locations: name, trait, status only).
rel R<n> kind=<relkind> a=C<x> b=C<y> [cause=E<k>]
  A relation between two characters starts.
rel R<n> kind=<relkind> cause=E<k>
  The relation changes.
rel R<n> fact=E<k>
  An event that matters for the relation.
event E<n> verb=<word> [agent=C<x>] [patient=C<x>|O<x>|L<x>] [to=C<x>|L<x>] [at=L<x>] [instr=O<x>] [why="text"] [how="text"]
  Something that really happens. verb = English base form. why = the agent's motive or the reason it happened.
say C<n>|narrator [to=C<m>] act=<act> [indirect=0|1] [sincere=0|1] [means="text"] [why="text"] id=E<k>
  An utterance, direct or reported; narrator = the narrator's own comment or moral. indirect=1: the act is done through another form (flattery that is a trick, a statement meant as a request). sincere=0: the speaker lies or flatters falsely. means = what the speaker really means or wants. why = the speaker's motive.
cursor at=L<n> [time=<word>]
  The narration moves to another place, or to another story time (morning, next_day).

CLOSED SETS (use only these values)
loctype: {loc}
objtype: {obj}
chartype: {chr}
relkind: {rel}
act: {acts}
keys: name="text" | gender=male|female | age=young|adult|old | mother=C<n> | father=C<n> | trait=<words> (lasting qualities) | feeling=<feeling> | sleep=awake|asleep | freedom=free|captive | life=alive|dead | goal="text" (what the character wants now) | belief="text" (what the character believes) | status=<words> (a state or role: hungry, sick, king)
feeling: {feel}

GATES
- Reference only ids that already exist; create each id once.
- Places change only by move or give; set never changes a place.
- Links stay two-way: have is only for an object that has no place yet. An object held by one character reaches another only by give; an object lying somewhere is picked up by move O to=C.
- Picking up, giving, dropping: only nearby - the same location, or locations nested with in=. A dropped object lands where the holder is, or in a location that contains it.
- An attribute that is already known changes only with cause=E<k> (an event or say written earlier). An unknown attribute may be set without cause.
- A relation changes kind only with cause=E<k>.
- event at=L must be where its agent is.
- Values only from the closed sets.

Write what really happens. Wishes, plans, conditions and hypotheticals are not events: put them into goal, belief, means or why.
Give motives (why) to important actions and speeches, feelings when the text shows them, and relations (rel) between characters who meet, with their changes and causes.
"#,
        loc = LOC_TYPE.list(),
        obj = OBJ_TYPE.list(),
        chr = CHAR_TYPE.list(),
        rel = REL_KIND.list(),
        acts = acts.join(" "),
        feel = FEELING.list(),
    )
}

fn story(o: &mut String, title: &str, sents: &[(u16, &str)]) {
    let _ = writeln!(o, "STORY \"{title}\"");
    for (i, (p, s)) in sents.iter().enumerate() {
        let _ = writeln!(o, "s{} (para {p}): {s}", i + 1);
    }
}

/// Prompt body without the `Commands:` tail (so a retry can insert the rejection explanation).
fn body(t: &Text) -> String {
    let mut o = rules();
    o.push_str("\nEXAMPLE (format only; a different story)\n");
    story(&mut o, "The Girl and the Dog", EXAMPLE_TEXT);
    o.push_str("Commands:\n");
    o.push_str(EXAMPLE_CMDS);
    o.push_str("\n\nNOW THE TASK\n");
    let sents: Vec<(u16, &str)> = t.sents.iter().map(|(p, s)| (*p, s.as_str())).collect();
    story(&mut o, &t.title, &sents);
    o
}

pub fn prompt(t: &Text) -> String {
    format!("{}Commands:\n", body(t))
}

/// Format rejection explanation for a retry.
pub fn feedback(errs: &[FormatErr]) -> String {
    let mut o = format!("Your previous answer to this task was rejected by the format gate ({} lines):\n", errs.len());
    for e in errs {
        let _ = writeln!(o, "- line {}: `{}` - {}", e.no, e.raw, e.msg);
    }
    o.push_str("Write the whole command list again, following the grammar and the closed sets exactly.\n");
    o
}

// ── v2 ─────────────────────────────────────────────────────────────────────────────────────────────

/// Example v2 (a different, invented story; not from FairytaleQA): shows intentions, beliefs, multiple acts,
/// joint agents, feelings as a set, `done`/`unset`, a body part, `neg`, `about`, `when`, `before`,
/// `num`, `here=0`, `mention`, a directed relation.
pub const EXAMPLE2_TEXT: &[(u16, &str)] = &[
    (1, "A poor miller and his wife lived in a mill by a river."),
    (1, "They had an old goose with a lame wing, and the wife planned to cook it for dinner."),
    (1, "\"Do not kill me,\" begged the goose, \"and I will bring you a golden egg.\""),
    (1, "The miller agreed, but his wife did not believe the goose."),
    (2, "The next morning the miller found a golden egg in the straw, which the goose had laid in the night, and he gave it to his wife."),
    (2, "She was ashamed and glad at once, and she gave up her plan."),
    (3, "That night a fox crept into the mill and bit off the goose's lame wing, but the miller and his wife drove it away together."),
    (3, "The goose laid many more golden eggs, and the old couple were never poor again; they even bought a farm in the next village."),
    (3, "The river still flows past the mill."),
];

pub const EXAMPLE2_CMDS: &str = r#"loc L1 type=river ^s1
loc L2 type=mill in=L1 ^s1
char C1 type=human class=miller name="the miller" at=L2 gender=male status=poor ^s1
char C2 type=human class=wife name="his wife" at=L2 gender=female status=poor ^s1
rel R1 kind=spouse a=C1 b=C2 ^s1
char C3 type=animal class=goose name="the goose" at=L2 age=old ^s2
obj O1 type=body_part class=wing name="a lame wing" part=C3 ^s2
plan P1 kind=plan who=C2 what="cook the goose for dinner" ^s2
say C3 to=C1+C2 act=request+promise means="do not kill me and I will bring you a golden egg" why="it does not want to be killed" id=E1 ^s3
set C3 goal="not be killed" ^s3
plan P2 kind=promise who=C3 to=C1 what="bring a golden egg" src=E1 ^s3
say C1 to=C3 act=agree cause=E1 id=E2 ^s4
believe B1 who=C2 claim="the goose will not bring a golden egg" about=P2 from=E1 true=0 ^s4
event E3 verb=believe agent=C2 patient=C3 neg=1 ^s4
cursor at=L2 time=next_morning ^s5
obj O2 type=treasure class=egg name="a golden egg" at=L2 ^s5
event E4 verb=find agent=C1 patient=O2 at=L2 how="in the straw" ^s5
event E5 verb=lay agent=C3 patient=O2 at=L2 when=night before=E4 ^s5
move O2 to=C1 cause=E4 ^s5
plan P2 status=kept by=E5 ^s5
event E6 verb=give agent=C1 patient=O2 to=C2 at=L2 ^s5
give C1 O2 to=C2 cause=E6 ^s5
feel C2 +ashamed +happy cause=E5 ^s6
believe B1 status=dropped by=E5 ^s6
event E7 verb=give_up agent=C2 about=P1 why="the goose kept its promise" cause=E5 ^s6
plan P1 status=dropped by=E7 ^s6
done C3 cause=E7 ^s6
char C4 type=animal class=fox name="a fox" ^s7
event E8 verb=creep agent=C4 to=L2 when=night ^s7
move C4 to=L2 cause=E8 ^s7
event E9 verb=bite_off agent=C4 patient=O1 at=L2 ^s7
detach O1 to=C4 cause=E9 ^s7
feel C3 +afraid cause=E9 ^s7
event E10 verb=drive_away agent=C1+C2 patient=C4 at=L2 how="together" cause=E9 ^s7
move C4 to=L1 cause=E10 ^s7
obj O3 type=treasure class=egg name="many more golden eggs" num=many ^s8
event E11 verb=lay agent=C3 patient=O3 at=L2 ^s8
have C1 O3 cause=E11 ^s8
rel R2 kind=benefactor a=C3 b=C1 cause=E11 ^s8
unset C1 status cause=E11 why="they were never poor again" ^s8
unset C2 status cause=E11 why="they were never poor again" ^s8
loc L3 type=farm name="a farm in the next village" here=0 ^s8
mention L1 L2 ^s9"#;

/// Rules of the change language v2 for the LLM.
pub fn rules_v2() -> String {
    let acts: Vec<&str> = Act::ALL.iter().map(|a| a.name()).collect();
    format!(
        r#"You turn a story into commands of a world-state language. A deterministic program executes the commands and checks them with strict gates; a line that breaks a gate is rejected, and later lines that depend on it fail too. Questions about the story (who, where, what happened, why, how someone felt, what they wanted, promised or believed, how it ended) will later be answered ONLY from this world state, so record what a careful reader would be asked about.

OUTPUT
- One command per line, in story order. Only command lines: no comments, explanations, headers or code fences.
- Every line ends with the anchor of its sentence: ^s<N>. A speech spanning two sentences may end with both: ^s3 ^s4. Anchors never go backwards. Every sentence gets at least one command; if nothing changes, mention the ids it speaks about.
- Ids: locations L1 L2 ..., objects O1 ..., characters C1 ..., relations R1 ..., events E1 ... (events and speeches share E numbers), plans P1 ..., beliefs B1 .... Number them in order of appearance; create before use; never reuse an id.
- <word> is lowercase English: letters, digits, _ or - (pick_up). "text" is plain English in double quotes, at most 20 words, no double quotes inside; reuse the story's own words where you can. <words> is word or word,word. C1+C2 joins several characters.
- Be economical: about 2-4 lines per sentence. Skip trivia; keep who, where, what happened and why, feelings, wants, promises, beliefs and outcomes.

COMMANDS - world
loc L<n> type=<loctype> [class=<word>] [name="text"] [in=L<m>] [here=0]
  A place; anything a character can be in or on. Put it inside the larger place with in=. The narration moves to a new place, except with here=0: the place is only mentioned (a far country, the place of a plan).
obj O<n> type=<objtype> [class=<word>] [name="text"] [at=L<m>|C<m>] [part=C<m>|O<m>] [num=<number>|some|few|several|many|all|pair]
  A thing; at=C<m>: the character holds or owns it; omit at= if its place is unknown. part=: a part of a body or thing (a nose, a wen, a tail, a door); a part is not held, it goes with its whole.
char C<n> type=<chartype> class=<word> [name="text"] [at=L<m>] [<key>=<value> ...]
  A character; class = species or role (wolf, king, miller). A group acting together (the villagers) is one character with num=.
move C<n> to=L<m> [cause=E<k>]           a character goes to another place
move O<n> to=L<m>|C<m> [cause=E<k>]      an object is dropped or put down (to=L), or picked up or taken by a character (to=C)
have C<n> O<m> [cause=E<k>]              first possession of an object that has no place yet
give C<n> O<m> to=C<k> [cause=E<j>]      a character hands an object to another
detach O<n> to=L<m>|C<m> cause=E<k>      a part is cut or taken off: it now lies somewhere or is in someone's hands
attach O<n> to=C<m>|O<m> cause=E<k>      a thing becomes part of a body or thing
mention X [Y ...]                        existing ids a sentence speaks about when nothing changes; the narration does not move
cursor at=L<n> [time=<word>]             the narration moves to a place, or to another story time (morning, next_day)

COMMANDS - characters
set C<n>|O<n>|L<n> <key>=<value> [<key>=<value> ...] [cause=E<k>]
  Attributes (objects: name, trait, status, num; locations: name, trait, status).
feel C<n> +<feeling> [+<feeling> ...] [-<feeling> ...] [cause=E<k>]
  Feelings are a set: + adds, - removes, the others stay.
done C<n> cause=E<k>                     the character's goal is reached
unset C<n> <key> cause=E<k> [why="text"] a goal is given up or fails, a feeling, status or belief ends
rel R<n> kind=<relkind> a=C<x> b=C<y> [cause=E<k>]    a relation between two characters starts
rel R<n> kind=<relkind> cause=E<k>                     the relation changes
rel R<n> fact=E<k>                                     an event that matters for the relation
  Directed kinds read "a is <kind> of b": benefactor (a helped b, b owes a), master, parent, ruler, captor, suitor, mentor. The others are mutual.

COMMANDS - happenings
event E<n> verb=<word> [agent=C<x>[+C<y>]] [patient=C<x>|O<x>|L<x>] [to=C<x>|L<x>] [at=L<x>] [instr=O<x>] [why="text"] [how="text"] [cause=E<k>] [neg=1] [about=X] [when=<word>] [before=E<k>]
  Something that happens. verb = English base form. why = the agent's motive or reason; how = the manner; cause = the earlier event it results from; neg=1: the text says it did NOT happen (he did not stop); about = the past event, plan, belief or thing it concerns; when = story time (night, next_day, long_ago); before=E<k>: it happened before E<k> although it is told later (a memory, a flashback). agent, patient and to may join several characters.
say C<n>[+C<m>]|narrator [to=C<m>[+C<k>]] act=<act>[+<act>] [indirect=0|1] [sincere=0|1] [means="text"] [why="text"] [cause=E<k>] [about=X] [when=<word>] id=E<k>
  An utterance, direct or reported; narrator = the narrator's own comment or moral. act may join up to 3 acts (request+promise). indirect=1: the act is done through another form (flattery that is a trick). sincere=0: a lie or false flattery. means = what is said or really meant; why = the speaker's motive.

COMMANDS - minds
plan P<n> kind=<plankind> who=C<x>[+C<y>] [to=C<z>] what="text" [src=E<k>]
  A promise, plan, agreement, wish, threat, order, task, curse, prophecy, bargain or bet; it is open now. who = whose it is; to = the one it is made to; src = the speech that made it.
plan P<n> status=kept|broken|dropped by=E<k>
  The plan closes: kept (fulfilled, came true) by event E<k>, broken, or dropped. Close each plan once, when the story shows it.
believe B<n> who=C<x> claim="text" [about=X] [from=E<k>] [true=0|1]
  What a character believes or knows. from = the event or speech it comes from; true=0 if it is false in the story (a lie believed, a mistake).
believe B<n> status=dropped by=E<k>      the character stops believing it
believe B<n> true=0|1 by=E<k>            the story shows whether it is true

CLOSED SETS (use only these values)
loctype: {loc}
objtype: {obj}
chartype: {chr}
relkind: {rel}
plankind: {plan}
act: {acts}
keys: name="text" | gender=male|female | age=young|adult|old | mother=C<n> | father=C<n> | trait=<words> (lasting qualities) | feeling=<feeling>[,<feeling>] | sleep=awake|asleep | freedom=free|captive | life=alive|dead | goal="text" (what the character wants now) | belief="text" | status=<words> (a state or role: hungry, sick, king, rich) | num=<number>|some|few|several|many|all|pair
feeling: {feel}

GATES
- Reference only ids that already exist; create each id once.
- Places change only by move, give, detach or attach; set never changes a place.
- Links stay two-way: have is only for an object that has no place yet. An object held by one character reaches another only by give; an object lying somewhere is picked up by move O to=C. A part moves with its whole until detach.
- Picking up, giving, dropping, detaching, attaching: only nearby - the same location, or locations nested with in=.
- An attribute that is already known changes only with cause=E<k> (an event or say written earlier). An unknown attribute may be set without cause. Known feelings change only with cause=.
- A relation changes kind or direction only with cause=E<k>.
- event at=L must be where each agent is.
- done, unset, detach, attach and closing a plan need an event that happened: cause=E<k> or by=E<k>; kept needs an event without neg=1. Things never move because of an event with neg=1.
- A plan closes once; a belief is dropped once.
- Values only from the closed sets.

Write what really happens as events. Wishes, plans, promises, orders, threats, curses and conditions are not events: write them as plan (or goal, belief), and close them when they come true or fail.
Give motives (why) to important actions and speeches, feelings when the text shows them, causes (cause=) between events, and relations between characters who meet, with their changes and causes.
"#,
        loc = LOC_TYPE.list(),
        obj = OBJ_TYPE.list(),
        chr = CHAR_TYPE.list(),
        rel = REL_KIND.list(),
        plan = PLAN_KIND.list(),
        acts = acts.join(" "),
        feel = FEELING.list(),
    )
}

/// Parts of a tale for the LLM: up to `max` sentences each; split at section boundaries, closer to the middle.
pub fn parts(t: &Text, max: usize) -> Vec<(u16, u16)> {
    let n = t.sents.len();
    if n <= max {
        return vec![(1, n as u16)];
    }
    // section boundaries: sentence i (1-based) where a new section starts
    let starts: Vec<usize> = (2..=n).filter(|i| t.sents[i - 1].0 != t.sents[i - 2].0).collect();
    let k = n.div_ceil(max);
    let mut out = Vec::new();
    let mut from = 1usize;
    for j in 1..k {
        let ideal = n * j / k + 1;
        let cut = starts.iter().copied().filter(|s| *s > from).min_by_key(|s| s.abs_diff(ideal)).unwrap_or(ideal);
        out.push((from as u16, (cut - 1) as u16));
        from = cut;
    }
    out.push((from as u16, n as u16));
    out
}

/// Short view of the world for continuation: registries, open intentions, last events, free ids.
pub fn world_brief(w: &crate::world::World, upto: u16) -> String {
    let mut o = format!("WORLD SO FAR (commands for s1-s{upto} were executed; these ids exist)\n");
    let brief = |id: Id| -> String {
        let Some(s) = w.now(id) else { return id.to_string() };
        let mut parts = vec![w.label(id)];
        let k = w.kind_of(id);
        if !k.is_empty() {
            parts.push(format!("({k})"));
        }
        for f in [Field::At, Field::Parent, Field::PartOf] {
            if let Some(x) = s.id(f) {
                parts.push(format!("{} {}", f.name(), x));
            }
        }
        for f in [Field::Feeling, Field::Goal, Field::Status, Field::Trait, Field::Num] {
            if let Val::Known(v) = s.get(f) {
                parts.push(format!("{}={v}", f.name()));
            }
        }
        parts.join(" ")
    };
    for (reg, title) in [(Reg::L, "locations"), (Reg::O, "objects"), (Reg::C, "characters")] {
        let v: Vec<String> = w.ents(reg).map(|e| brief(e.id)).collect();
        if !v.is_empty() {
            let _ = writeln!(o, "{title}: {}", v.join(" | "));
        }
    }
    let rels: Vec<String> = w
        .ents(Reg::R)
        .map(|e| {
            let s = e.now();
            format!("{} {}", w.label(e.id), s.get(Field::Kind))
        })
        .collect();
    if !rels.is_empty() {
        let _ = writeln!(o, "relations: {}", rels.join(" | "));
    }
    let plans: Vec<String> = w
        .ents(Reg::P)
        .map(|e| {
            let s = e.now();
            let who = s.ids(SetF::Owners).iter().map(|c| c.to_string()).collect::<Vec<_>>().join("+");
            format!("{} {} by {who} {} [{}]", e.id, s.get(Field::Kind), s.get(Field::What), s.get(Field::Stage))
        })
        .collect();
    if !plans.is_empty() {
        let _ = writeln!(o, "plans: {}", plans.join(" | "));
    }
    let bel: Vec<String> = w.ents(Reg::B).map(|e| format!("{} {} {} [{}]", e.id, e.now().ids(SetF::Owners).iter().map(|c| c.to_string()).collect::<Vec<_>>().join("+"), e.now().get(Field::What), e.now().get(Field::Stage))).collect();
    if !bel.is_empty() {
        let _ = writeln!(o, "beliefs: {}", bel.join(" | "));
    }
    let ev: Vec<String> = w.events().iter().rev().take(12).rev().map(|e| format!("{} {} ({})", w.label(e.id), e.anchors(), w.roles_str(e))).collect();
    if !ev.is_empty() {
        let _ = writeln!(o, "last events: {}", ev.join("; "));
    }
    if let Some(c) = w.cursor().last() {
        let _ = writeln!(o, "cursor: at {} time={}", c.at.map(|l| l.to_string()).unwrap_or("?".into()), c.time.clone().unwrap_or("-".into()));
    }
    let free: Vec<String> = w.next_free().iter().map(|(r, n)| format!("{}{n}", r.letter())).collect();
    let _ = writeln!(o, "next free ids: {}", free.join(" "));
    o
}

/// Prompt body v2 without the `Commands:` tail: rules, example, (for continuation — the world and the preceding
/// text), the part's sentences.
pub fn body_v2(t: &Text, part: (u16, u16), prev: Option<&crate::world::World>) -> String {
    let mut o = rules_v2();
    o.push_str("\nEXAMPLE (format only; a different story)\n");
    story(&mut o, "The Miller's Goose", EXAMPLE2_TEXT);
    o.push_str("Commands:\n");
    o.push_str(EXAMPLE2_CMDS);
    o.push_str("\n\nNOW THE TASK\n");
    let (a, b) = part;
    if a > 1 {
        let _ = writeln!(o, "The story is long and goes in parts. Sentences s1-s{} were already turned into commands; do NOT write commands for them.", a - 1);
        let _ = writeln!(o, "STORY SO FAR \"{}\" (context only)", t.title);
        for (i, (p, s)) in t.sents.iter().enumerate().take(a as usize - 1) {
            let _ = writeln!(o, "s{} (para {p}): {s}", i + 1);
        }
        if let Some(w) = prev {
            o.push('\n');
            o.push_str(&world_brief(w, a - 1));
        }
        let _ = writeln!(o, "\nCONTINUE with sentences s{a}-s{b}: keep the existing ids, start new ids from the next free ones, first anchor ^s{a}.");
    }
    let _ = writeln!(o, "STORY \"{}\"{}", t.title, if a > 1 || (b as usize) < t.sents.len() { format!(" (sentences s{a}-s{b})") } else { String::new() });
    for (i, (p, s)) in t.sents.iter().enumerate() {
        let n = i as u16 + 1;
        if n >= a && n <= b {
            let _ = writeln!(o, "s{n} (para {p}): {s}");
        }
    }
    o
}

/// Request to fix only the lines rejected by the format gate: `@<line> <command>` for each.
pub fn repair_prompt(body: &str, out: &str, errs: &[FormatErr]) -> String {
    let mut o = format!("{body}\nYOUR ANSWER (numbered lines):\n");
    for (i, l) in out.lines().enumerate() {
        let _ = writeln!(o, "{}: {l}", i + 1);
    }
    let _ = writeln!(o, "\nThe format gate rejected {} of these lines:", errs.len());
    for e in errs {
        let _ = writeln!(o, "- line {}: `{}` - {}", e.no, e.raw, e.msg);
    }
    o.push_str("Write a corrected command for each rejected line, one per line, as @<line number> <command> (for example: @12 move C1 to=L2 ^s5). Keep the same ids and anchors. Only these lines, nothing else.\n");
    o
}

/// Insert the fixed lines `@N command` in place of the rejected ones.
pub fn splice(out: &str, fix: &str) -> (String, usize) {
    let mut lines: Vec<String> = out.lines().map(str::to_string).collect();
    let mut n = 0;
    for l in fix.lines() {
        let Some(rest) = l.trim().strip_prefix('@') else { continue };
        let Some((no, cmd)) = rest.split_once(' ') else { continue };
        let Ok(no) = no.parse::<usize>() else { continue };
        if no >= 1 && no <= lines.len() {
            lines[no - 1] = cmd.trim().to_string();
            n += 1;
        }
    }
    (lines.join("\n"), n)
}

/// LLM v2 for a tale: parts (up to `max` sentences), one call per part; one retry for the whole tale, only on
/// a format rejection, asking to fix only the rejected lines. At most 3 calls in total. Finished
/// `cmds-<name>.txt` are not overwritten.
pub fn call_v2(t: &Text, name: &str, dir: &Path, max: usize) -> Result<String> {
    let done = dir.join(format!("cmds-{name}.txt"));
    if done.exists() {
        bail!("{} already exists — not calling the LLM again", done.display());
    }
    let ps = parts(t, max);
    if ps.len() > 2 {
        bail!("{name}: {} parts — with a retry that is over 3 calls", ps.len());
    }
    std::fs::create_dir_all(dir)?;
    let opus = Opus::from_env(dir.join("cwd"));
    let calls = dir.join("calls.jsonl");
    let mut all = String::new();
    let mut repaired = false;
    for (i, &(a, b)) in ps.iter().enumerate() {
        let prev = if a > 1 {
            let (lines, _) = parse_all(&all);
            Some(crate::world::run(t.clone(), &lines).world)
        } else {
            None
        };
        let body = body_v2(t, (a, b), prev.as_ref());
        let p = format!("{body}Commands:\n");
        std::fs::write(dir.join(format!("prompt-{name}-p{}.txt", i + 1)), &p)?;
        let (mut out, c) = opus.ask(&format!("world-v2 {name} p{}", i + 1), t.doc as usize, (b - a + 1) as usize, &p)?;
        append_call(&calls, &c)?;
        std::fs::write(dir.join(format!("raw-{name}-p{}.txt", i + 1)), &out)?;
        let (_, errs) = parse_all(&out);
        if !errs.is_empty() && !repaired {
            repaired = true;
            eprintln!("{name} p{}: format gate rejected {} lines — one retry (these lines only)", i + 1, errs.len());
            let rp = repair_prompt(&body, &out, &errs);
            std::fs::write(dir.join(format!("repair-prompt-{name}.txt")), &rp)?;
            let (fix, c2) = opus.ask(&format!("world-v2 {name} repair"), t.doc as usize, errs.len(), &rp)?;
            append_call(&calls, &c2)?;
            std::fs::write(dir.join(format!("repair-{name}.txt")), &fix)?;
            let (spliced, n) = splice(&out, &fix);
            eprintln!("{name}: lines fixed {n}");
            out = spliced;
        }
        if !all.is_empty() && !all.ends_with('\n') {
            all.push('\n');
        }
        all.push_str(out.trim_end());
        all.push('\n');
    }
    std::fs::write(&done, &all)?;
    Ok(all)
}

/// Document sentences from the database (read only); the license must be public domain, otherwise stop.
pub fn read_text(db: &Path, doc: i64, tmp: &Path) -> Result<Text> {
    let sql = include_str!("../sql/fable.sql").replace("{doc}", &doc.to_string());
    let rows = prag::data::duck_query(db, &sql, tmp)?;
    if rows.is_empty() {
        bail!("doc {doc}: no sentences");
    }
    let mut title = String::new();
    let mut sents = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        // columns: title license para ord text
        if r.len() != 5 {
            bail!("fable.sql: {} columns instead of 5", r.len());
        }
        if !r[1].starts_with("public domain") {
            bail!("doc {doc}: license «{}» is not public domain", r[1]);
        }
        let ord: usize = r[3].parse().with_context(|| format!("ord {:?}", r[3]))?;
        if ord != i + 1 {
            bail!("doc {doc}: sentence {ord} at position {}", i + 1);
        }
        let para: u16 = if r[2] == "\\N" { 1 } else { r[2].parse().with_context(|| format!("para {:?}", r[2]))? };
        title = r[0].clone();
        sents.push((para, r[4].clone()));
    }
    Ok(Text { doc, title, sents })
}

/// LLM call for a document: one call, a retry — only one and only if the format gate rejected lines.
/// Accounting — `calls.jsonl`; prompt, raw output and final commands — in the run folder.
pub fn call(t: &Text, dir: &Path) -> Result<String> {
    let doc = t.doc;
    let done = dir.join(format!("cmds-{doc}.txt"));
    if done.exists() {
        bail!("{} already exists — not calling the LLM again", done.display());
    }
    std::fs::create_dir_all(dir)?;
    let opus = Opus::from_env(dir.join("cwd"));
    let calls = dir.join("calls.jsonl");
    let p = prompt(t);
    std::fs::write(dir.join(format!("prompt-{doc}.txt")), &p)?;
    let (out, c) = opus.ask("world-vmm", doc as usize, t.sents.len(), &p)?;
    append_call(&calls, &c)?;
    std::fs::write(dir.join(format!("raw-{doc}.txt")), &out)?;
    let (_, errs) = parse_all(&out);
    let text = if errs.is_empty() {
        out
    } else {
        let fb = feedback(&errs);
        eprintln!("doc {doc}: format gate rejected {} lines — one retry", errs.len());
        std::fs::write(dir.join(format!("format-rejects-{doc}.txt")), &fb)?;
        let p2 = format!("{}\n{fb}\nCommands:\n", body(t));
        let (out2, c2) = opus.ask("world-vmm-retry", doc as usize, t.sents.len(), &p2)?;
        append_call(&calls, &c2)?;
        std::fs::write(dir.join(format!("raw-{doc}-retry.txt")), &out2)?;
        out2
    };
    std::fs::write(&done, &text)?;
    Ok(text)
}

// ── v3 ─────────────────────────────────────────────────────────────────────────────────────────────

/// Example v3 (an invented story, not from the test grounds): a past frame (testimony), a canonical event from its first
/// mention, a claim and a change of its status, a reveal (role, cause, false claim), a memory
/// `before=`, a `time` relation, absolute time from the text, a lie in branch `lie`, knowledge.
pub const EXAMPLE3_TEXT: &[(u16, &str)] = &[
    (1, "Old Tom kept a small shop in the village, and his cat Ginger slept on the counter."),
    (1, "One evening his neighbour Mary came in and found the shop empty and the blue vase broken on the floor."),
    (2, "\"The wind must have knocked it down,\" said Mary."),
    (3, "Soon Tom came back from the market, and Mary showed him the pieces."),
    (3, "Tom sat down and told her what had happened that morning."),
    (4, "\"At dawn I put the vase on the high shelf, and Ginger watched me from the counter.\""),
    (4, "\"Before I left, I shut the window and gave her a saucer of milk.\""),
    (5, "\"So the window was shut, and the wind is innocent,\" said Mary."),
    (5, "Then Tom lifted the cat and showed Mary the blue dust on its paws."),
    (5, "Ginger had jumped onto the shelf while the shop was empty, and the vase had fallen."),
    (6, "Mary had once told the baker that the shop was haunted, though she never believed it herself."),
    (6, "Years later the villagers still told the story of the cat and the vase."),
];

pub const EXAMPLE3_CMDS: &str = r#"loc L1 type=village ^s1
loc L2 type=shop in=L1 ^s1
char C1 type=human class=shopkeeper name="Old Tom" gender=male age=old ^s1
char C2 type=animal class=cat name="Ginger" at=L2 gender=female ^s1
rel R1 kind=master a=C1 b=C2 ^s1
char C3 type=human class=neighbour name="Mary" gender=female ^s2
event E1 verb=come agent=C3 to=L2 when=evening ^s2
move C3 to=L2 cause=E1 ^s2
obj O1 type=container class=vase name="the blue vase" at=L2 ^s2
event E2 verb=break patient=O1 at=L2 before=E1 ^s2
set O1 status=broken cause=E2 ^s2
event E3 verb=find agent=C3 patient=O1 at=L2 how="broken on the floor" ^s2
say C3 act=assert means="the wind must have knocked the vase down" id=E4 ^s3
claim K1 who=C3 about=E2 status=believes interp="the wind knocked the vase down" from=E4 ^s3
loc L3 type=market in=L1 here=0 ^s4
event E5 verb=come_back agent=C1 to=L2 ^s4
move C1 to=L2 cause=E5 ^s4
event E6 verb=show agent=C3 patient=O1 to=C1 at=L2 ^s4
say C1 to=C3 act=narrate means="what happened that morning" id=E7 ^s5
frame open F1 narrator=C1 to=C3 level=1 src=E7 ^s5
cursor at=L2 time=morning ^s6
loc L4 type=place class=shelf name="the high shelf" in=L2 here=0 ^s6
move C1 to=L2 ^s6
event E8 verb=put agent=C1 patient=O1 to=L4 at=L2 ^s6
time E8 at="At dawn" ^s6
move O1 to=L4 cause=E8 ^s6
move C2 to=L2 ^s6
event E9 verb=watch agent=C2 patient=C1 at=L2 ^s6
obj O2 type=thing class=window at=L2 ^s7
event E10 verb=shut agent=C1 patient=O2 at=L2 ^s7
obj O3 type=food class=milk name="a saucer of milk" ^s7
have C1 O3 ^s7
event E11 verb=give agent=C1 patient=O3 to=C2 at=L2 ^s7
give C1 O3 to=C2 cause=E11 ^s7
event E12 verb=leave agent=C1 at=L2 why="to go to the market" ^s7
move C1 to=L3 cause=E12 ^s7
frame close F1 ^s7
say C3 act=evaluate means="the window was shut, so the wind did not break the vase" id=E13 ^s8
claim K1 status=doubts by=E13 ^s8
event E14 verb=show agent=C1 patient=C2 to=C3 at=L2 how="the blue dust on its paws" ^s9
know who=C1 fact=E2 from=^s9 how=inferred by=E14 ^s9
event E15 verb=jump agent=C2 to=L4 before=E1 ^s10
time E15 AFTER E12 ^s10
reveal E2 to=C3 agent=C2 cause=E15 wrong=K1 by=E14 interp="Ginger jumped onto the shelf and the vase fell" ^s10
char C4 type=human class=baker name="the baker" ^s11
say C3 to=C4 act=assert sincere=0 means="the shop is haunted" before=E1 id=E16 ^s11
event E17 verb=haunt patient=L2 about=E16 ^s11
branch E17 lie by=E16 ^s11
claim K2 who=C4 about=E17 status=believes interp="the shop is haunted" from=E16 ^s11
char C5 type=human class=villagers name="the villagers" num=many ^s12
event E18 verb=tell agent=C5 about=E2 when=years_later after=E14 ^s12"#;

/// Rules v3: change language v2 plus nonlinear narration — frames, event identity, the story time axis,
/// reality branches, claims, knowledge, reveals.
pub fn rules_v3() -> String {
    let mut o = rules_v2();
    o.push_str(
        r#"
NONLINEAR STORY (v3): the world keeps apart WHAT HAPPENED (canonical events), WHAT PEOPLE THINK ABOUT IT (claims, knowledge) and THE ORDER IT IS TOLD IN (sentences) versus the order it happened in (story time). Questions will ask: what happened earlier; what was told first but happened later; what X believed at sentence N; what really happened; when and how it came out; who tells whom at which level.

FRAMES - who tells the story
frame open F<n> narrator=C<x> level=0 [to=reader]
  Only if the book's narrator is a character (first person, "I"): name the root narrator once, as the first frame command.
frame open F<n> narrator=C<x>|narrator [to=C<y>[+C<z>]] level=<current level + 1> [world=same|new] [time=past|now] [before=E<k>] [src=E<k>] [branch=dream|lie|plan|if]
  A character starts telling a story of their own (a testimony, a memory, an explanation, a tale). Open it where the telling starts, one level deeper than the current frame; every command after it belongs to the frame until frame close.
  - world=same time=past (default): a testimony, memory or explanation about the same world. The frame has its OWN world where places, feelings and states are unknown until the told story gives them: move the teller and others to where the told events happen. Its events happened in the past: give before=E<k> - the first event of the present scene that the whole told story happened before (usually the teller's arrival or the meeting) - and src=E<k> - the say (act=narrate) that starts the telling.
  - world=new: an invented story told inside the story (a tale told to entertain): a new world with its own characters and its own time line; create its characters inside the frame; they cannot act in the teller's world.
  - time=now: someone else narrates what happens now (the same world and time line).
  - branch=...: a dream, lie, plan or hypothesis told at length.
frame close F<n>
  The telling ends and the narration returns to the enclosing frame. Frames nest like brackets: close the innermost first. An interrupted telling that goes on later is re-opened with the same id, narrator and level.

ONE EVENT, ONE ID
- A real event gets one E id, at its first mention. When the text speaks of it again (a retelling, a clue, a rumour, the solution of a mystery), do NOT create another event for it: refer to its id with claim, know, reveal or time.
- An event mentioned before it happens (a rumour, a prophecy, "as I learned later"): create it once, placed with after=E<k>; when the narration reaches it, give its time (time E<m> AFTER E<k>) and write its consequences with cause=E<m>. Never create it a second time.
- An event told after it happened (a memory, "she had died two years before", "I knew it before we came"): write it inside a past frame, or with before=E<k> (it happened before E<k>).
- Every id in before=, after=, during=, same=, cause=, about=, src=, by= must already exist on an EARLIER line. To place a new event before an event you have not written yet, write that other event first and then the new one with before=, or write both and then add time E<a> BEFORE E<b>. A line that names a not-yet-written id is refused, and so is every later line that depends on it.

STORY TIME - a partial order, not labels
- Events told one after another without a mark happen one after another on the narration line of their frame.
- event ... before=E<k> | after=E<k> | during=E<k> | same=E<k>: the event is off the narration line; it happened before / after / during / at about the same time as E<k>.
time E<a> BEFORE|AFTER|DURING|SAME E<b>
  A relation between two existing events of the same world (for example when the text later says what happened first).
time E<a> at="<words of the anchor sentence>"
  An explicit time exactly as the anchor sentence writes it ("early in April in the year '83", "two years ago", "at dawn"). Never compute dates.

REALITY
branch E<k> dream|plan|intent|lie|if|report|belief [by=E<j>]
  The event did not really happen: it is dreamt, planned, intended, a lie, a hypothesis, an unconfirmed report, a mistaken belief. Mark it right after writing it. Only real events change the world: never use such an event as cause= of a change. The say that tells a lie is real; the lie's content is a separate event in branch lie.

WHAT PEOPLE THINK AND KNOW
claim K<n> who=C<x>|reader|narrator about=E<k> status=believes|doubts|wrong|knows interp="text" [from=E<j>]
  How an observer understands an existing event: its cause, who did it, what it was. believes = holds it; doubts = unsure; wrong = the story already shows it false; knows = true and certain. from = the say or event it comes from.
claim K<n> status=believes|doubts|wrong|knows by=E<k>
  The observer changes their mind because of E<k>.
know who=C<x>|reader fact=E<k>|K<n>|P<n> from=^s<N> [how=saw|heard|told|inferred|revealed|guessed] [by=E<j>]
  The observer knows the fact from sentence N (N not later than this line's anchor). Listeners of a frame know its events without know; use know for knowledge the text points out.
reveal E<k> [to=C<x>[+C<y>|reader]] [by=E<j>] [interp="the truth"] [wrong=K<n>[+K<m>]] [right=K<n>] [agent=C<x>] [patient=C<x>|O<x>] [instr=O<x>] [cause=E<j>] [before=E<j>|after=E<j>|during=E<j>|same=E<j>]
  The truth about an EXISTING event comes out: a role it did not have yet (the real killer, the weapon), its cause, its time, and which claims it shows false or true. by = the event or say that reveals it; to = who learns it (the reader always does). A reveal changes knowledge, not the past: it never creates an event and never replaces a role the event already has.

MORE GATES (v3)
- A frame opens exactly one level deeper than the current frame; frames close innermost first; the root is never closed.
- Time relations must not contradict each other (a cycle is refused); time at= only with words of its anchor sentence.
- In the present line a character's place is checked on the story time line: a past event told without a frame or before= at a place where the teller is not now is refused as teleportation. Inside a past frame the teller's place is unknown until you move them there.
- Only real events change the world (cause= and by= of changes must be real events of the same world); marking an event as a lie or dream is refused once it has changed the world.
- The dead do not act in the present; a second death of the same character is refused - it is the same event: reveal it or claim about it.
- Claims, knowledge and reveals refer only to existing events; know from= cannot be a later sentence.
- Characters of an invented story (world=new) exist only in its frame; its events cannot cause changes in the teller's world.
"#,
    );
    o
}

/// Short view of the world v3 for continuation: frames (open ones on top), entities of the current world,
/// claims, events off the line, last events, free ids (across all worlds).
pub fn world_brief_v3(w: &crate::world::World, upto: u16) -> String {
    use crate::narr::world_frame;
    let Some(st) = w.story() else { return world_brief(w, upto) };
    let mut o = format!("WORLD SO FAR (commands for s1-s{upto} were executed; these ids exist)\n");
    // entities: the root and the world of the current frame
    let top = st.top();
    let wi = world_frame(st, top);
    let cur = if wi == 0 { w } else { st.frames[wi].world.as_deref().unwrap_or(w) };
    let one = world_brief(cur, upto);
    for l in one.lines().skip(1) {
        if l.starts_with("next free ids") || l.starts_with("cursor") || l.starts_with("last events") {
            continue;
        }
        let _ = writeln!(o, "{l}");
    }
    if wi != 0 {
        let _ = writeln!(o, "(these are the entities of the world of the open frame {}; places there are the told past)", st.frames[wi].name());
    }
    // frames
    let fr: Vec<String> = st
        .frames
        .iter()
        .map(|f| {
            format!(
                "{} level {} {} -> {} {}{}",
                f.name(),
                f.level,
                f.narrator,
                if f.to.is_empty() { "-".into() } else { f.to.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("+") },
                match f.kind {
                    crate::story::FWorld::Root => "root",
                    crate::story::FWorld::Past => "world=same time=past",
                    crate::story::FWorld::Now => "time=now",
                    crate::story::FWorld::New => "world=new",
                    crate::story::FWorld::Branch => "branch",
                },
                if f.is_open() { " [OPEN]" } else { " [closed]" }
            )
        })
        .collect();
    let _ = writeln!(o, "frames: {}", fr.join(" | "));
    let stack: Vec<String> = st.stack.iter().map(|i| st.frames[*i].name()).collect();
    let _ = writeln!(
        o,
        "open frames (outermost first): {} - you are inside {} at level {}; close it when that telling ends",
        stack.join(" > "),
        st.frames[top].name(),
        st.frames[top].level
    );
    // claims
    let cl: Vec<String> = w
        .ents(Reg::K)
        .map(|k| {
            let s = k.now();
            format!("{} {} about {} [{}] {}", k.id, crate::narr::val_obs(&s.get(Field::Who)).map(|x| x.to_string()).unwrap_or_default(), s.get(Field::About), s.get(Field::Stage), s.get(Field::What))
        })
        .collect();
    if !cl.is_empty() {
        let _ = writeln!(o, "claims: {}", cl.join(" | "));
    }
    // events off the line: not yet placed in time or with after= (may happen later)
    let pend: Vec<String> = st
        .ev
        .iter()
        .filter(|(_, m)| m.placed.is_some_and(|(r, _)| r == crate::lang::TRel::After) || m.branch.as_str() != "real")
        .map(|(id, m)| format!("{} ({}, {} ^s{})", w.label_any(*id), m.placed.map(|(r, x)| format!("{} {x}", r.name())).unwrap_or_default(), m.branch, m.sent))
        .collect();
    if !pend.is_empty() {
        let _ = writeln!(o, "events off the line (announced before they happen, or not real): {}", pend.join("; "));
    }
    // important events from all worlds: the last 15
    let mut evs: Vec<(u16, usize, Id)> = st.ev.iter().map(|(id, m)| (m.sent, m.line, *id)).collect();
    evs.sort();
    let last: Vec<String> = evs.iter().rev().take(15).rev().map(|(s, _, id)| format!("{} ^s{s} [{}]", w.label_any(*id), st.frames[st.ev[id].frame].name())).collect();
    let _ = writeln!(o, "last events: {}", last.join("; "));
    if let Some(c) = cur.cursor().last() {
        let _ = writeln!(o, "cursor of the current frame: at {} time={}", c.at.map(|l| l.to_string()).unwrap_or("?".into()), c.time.clone().unwrap_or("-".into()));
    }
    let free: Vec<String> = next_free_v3(w).iter().map(|(r, n)| format!("{}{n}", r.letter())).collect();
    let _ = writeln!(o, "next free ids: {}", free.join(" "));
    o
}

/// Next free ids across all worlds (frames keep new entities in their own worlds until closed).
pub fn next_free_v3(w: &crate::world::World) -> Vec<(Reg, u32)> {
    let mut max: std::collections::BTreeMap<Reg, u32> = std::collections::BTreeMap::new();
    for x in w.worlds() {
        for (r, n) in x.next_free() {
            let e = max.entry(r).or_insert(1);
            *e = (*e).max(n);
        }
    }
    let k = w.ents(Reg::K).map(|e| e.id.n).max().unwrap_or(0) + 1;
    let f = w.story().map(|s| s.frames.iter().filter_map(|f| f.id.map(|i| i.n)).max().unwrap_or(0)).unwrap_or(0) + 1;
    let mut v: Vec<(Reg, u32)> = max.into_iter().collect();
    v.push((Reg::K, k));
    v.push((Reg::F, f));
    v
}

/// Prompt body v3 without the `Commands:` tail.
pub fn body_v3(t: &Text, part: (u16, u16), prev: Option<&crate::world::World>) -> String {
    let mut o = rules_v3();
    o.push_str("\nEXAMPLE 1 (format only; a different story)\n");
    story(&mut o, "The Miller's Goose", EXAMPLE2_TEXT);
    o.push_str("Commands:\n");
    o.push_str(EXAMPLE2_CMDS);
    o.push_str("\n\nEXAMPLE 2 (nonlinear; format only; a different story)\n");
    story(&mut o, "The Cat and the Vase", EXAMPLE3_TEXT);
    o.push_str("Commands:\n");
    o.push_str(EXAMPLE3_CMDS);
    o.push_str("\n\nNOW THE TASK\n");
    let (a, b) = part;
    if a > 1 {
        let _ = writeln!(o, "The story is long and goes in parts. Sentences s1-s{} were already turned into commands; do NOT write commands for them.", a - 1);
        let _ = writeln!(o, "STORY SO FAR \"{}\" (context only)", t.title);
        for (i, (p, s)) in t.sents.iter().enumerate().take(a as usize - 1) {
            let _ = writeln!(o, "s{} (para {p}): {s}", i + 1);
        }
        if let Some(w) = prev {
            o.push('\n');
            o.push_str(&world_brief_v3(w, a - 1));
        }
        let _ = writeln!(o, "\nCONTINUE with sentences s{a}-s{b}: keep the existing ids, start new ids from the next free ones, first anchor ^s{a}. Stay inside the open frames until their tellings end.");
    }
    let _ = writeln!(o, "STORY \"{}\"{}", t.title, if a > 1 || (b as usize) < t.sents.len() { format!(" (sentences s{a}-s{b})") } else { String::new() });
    for (i, (p, s)) in t.sents.iter().enumerate() {
        let n = i as u16 + 1;
        if n >= a && n <= b {
            let _ = writeln!(o, "s{n} (para {p}): {s}");
        }
    }
    o
}

/// Accounting: sum of $ in `calls.jsonl` (with the API — the actual price).
pub fn spent(dir: &Path) -> Result<f64> {
    Ok(prag::opus::read_calls(&dir.join("calls.jsonl"))?.iter().map(|c| c.cost_usd).sum())
}

/// Upper estimate of a call: the fixed overhead of `claude -p` plus output (including thinking) and the prompt cache write.
pub fn estimate_call(sents: usize, prompt_chars: usize) -> f64 {
    0.35 + sents as f64 * 800.0 * 25e-6 + prompt_chars as f64 / 3.5 * 10e-6
}

/// LLM v3 for a test ground: parts (up to `max` sentences), one call per part, a retry — only on a format rejection,
/// once per part and only for the rejected lines. Before each call — the budget cap (`WORLD_V3_BUDGET`,
/// $40 by default): if spent + call estimate exceeds it — stop (fail-fast). Finished parts
/// (`raw-<name>-p<N>.txt`) are not re-requested.
pub fn call_v3(t: &Text, name: &str, dir: &Path, max: usize, budget: f64) -> Result<String> {
    let done = dir.join(format!("cmds-{name}.txt"));
    if done.exists() {
        bail!("{} already exists — not calling the LLM again", done.display());
    }
    let ps = parts(t, max);
    std::fs::create_dir_all(dir)?;
    let opus = Opus::from_env(dir.join("cwd"));
    let calls = dir.join("calls.jsonl");
    let mut all = String::new();
    let stop: usize = std::env::var("V3_STOP_AFTER").ok().and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
    for (i, &(a, b)) in ps.iter().enumerate() {
        if i >= stop {
            bail!("{name}: stopping after {stop} parts (V3_STOP_AFTER) — pilot; finished parts saved");
        }
        let tag = format!("{name}-p{}", i + 1);
        let fixed = dir.join(format!("fixed-{tag}.txt"));
        let raw = dir.join(format!("raw-{tag}.txt"));
        let out = if fixed.exists() {
            std::fs::read_to_string(&fixed)?
        } else {
            let prev = if a > 1 {
                let (lines, _) = parse_all(&all);
                Some(crate::world::run_v3(t.clone(), &lines).world)
            } else {
                None
            };
            let body = body_v3(t, (a, b), prev.as_ref());
            let p = format!("{body}Commands:\n");
            std::fs::write(dir.join(format!("prompt-{tag}.txt")), &p)?;
            let mut out = if raw.exists() {
                std::fs::read_to_string(&raw)?
            } else {
                let est = estimate_call((b - a + 1) as usize, p.len());
                let sp = spent(dir)?;
                if sp + est > budget {
                    bail!("{tag}: spent ${sp:.2} + estimate ${est:.2} > cap ${budget:.2} — stopping");
                }
                eprintln!("{tag}: s{a}-s{b}, spent ${sp:.2}, call estimate ${est:.2}");
                let (out, c) = opus.ask(&format!("world-v3 {tag}"), t.doc as usize, (b - a + 1) as usize, &p)?;
                append_call(&calls, &c)?;
                eprintln!("{tag}: ${:.3}, output {} tok., {:.0} s", c.cost_usd, c.output_tokens, c.secs);
                std::fs::write(&raw, &out)?;
                out
            };
            let (_, errs) = parse_all(&out);
            if !errs.is_empty() {
                let rp = repair_prompt(&body, &out, &errs);
                let est = estimate_call(errs.len(), rp.len());
                let sp = spent(dir)?;
                if sp + est > budget {
                    bail!("{tag} repair: spent ${sp:.2} + estimate ${est:.2} > cap ${budget:.2} — stopping");
                }
                eprintln!("{tag}: format gate rejected {} lines — one repair (these lines only)", errs.len());
                std::fs::write(dir.join(format!("repair-prompt-{tag}.txt")), &rp)?;
                let (fix, c2) = opus.ask(&format!("world-v3 {tag} repair"), t.doc as usize, errs.len(), &rp)?;
                append_call(&calls, &c2)?;
                std::fs::write(dir.join(format!("repair-{tag}.txt")), &fix)?;
                let (spliced, n) = splice(&out, &fix);
                eprintln!("{tag}: lines fixed {n}");
                out = spliced;
            }
            std::fs::write(&fixed, &out)?;
            out
        };
        if !all.is_empty() && !all.ends_with('\n') {
            all.push('\n');
        }
        all.push_str(out.trim_end());
        all.push('\n');
    }
    std::fs::write(&done, &all)?;
    Ok(all)
}
