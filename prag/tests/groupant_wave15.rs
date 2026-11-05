//! Supplied-input joins. No parser process or global configuration mutation.
use en::annotate::Word;
use prag::{
    frames::{self, Ask, Asked, Ctx, Ent, Frame, How, Kind, PTag, Read, Rel, Tam, Tree, UPos},
    frames_md,
    rules::{self, Action, Cand, Site},
    schema::{self, Act},
    vn::{Clause, Filler, Lex},
    vnconv,
};
use std::collections::BTreeSet;
fn ent(id: &str) -> Ent {
    Ent {
        id: id.into(),
        text: id.into(),
        ..Default::default()
    }
}
fn frame(verb: &str) -> Frame {
    Frame {
        kind: Kind::Event,
        agent: None,
        verb: verb.into(),
        neg: false,
        patient: None,
        recipient: None,
        place: None,
        attrs: vec![],
        manner: vec![],
        obls: vec![],
        quote: vec![],
        quoted: false,
        cause: None,
        next: None,
        cond: false,
        irrealis: false,
        src: (0, 0),
        how: How::default(),
        unsaid: vec![],
        words: vec![],
        nps: vec![],
        mark: vec![],
        cc: None,
        link: None,
        tam: Tam::default(),
    }
}
fn tree(rows: &[(&str, &str, UPos, PTag, usize, Rel)]) -> Tree {
    Tree::of_words(
        rows.iter()
            .map(|&(form, lemma, upos, tag, head, rel)| Word {
                form: form.into(),
                lemma: lemma.into(),
                upos,
                tag,
                head,
                rel,
                feats: Default::default(),
            })
            .collect(),
    )
}
#[test]
fn converted_tsv_preserves_supplied_lex_role_coordinates() {
    let xml = r#"<ROOT><VNCLASS ID="aaa-1"><MEMBERS><MEMBER name="distract"/></MEMBERS></VNCLASS>
<VNCLASS ID="give-13.1"><MEMBERS><MEMBER name="give"/></MEMBERS><THEMROLES><THEMROLE type="Agent"/><THEMROLE type="Theme"/><THEMROLE type="Recipient"/></THEMROLES><FRAMES><FRAME><DESCRIPTION primary="NP V NP PP.recipient"/><SYNTAX><NP value="Agent"/><VERB/><NP value="Theme"/><PREP value="to"/><NP value="Recipient"/></SYNTAX><SEMANTICS></SEMANTICS><EXAMPLES><EXAMPLE>Ada gave a box to Bob.</EXAMPLE></EXAMPLES></FRAME></FRAMES></VNCLASS></ROOT>"#;
    let (tsv, nc, nm, nf) = vnconv::verbnet_xml(xml).unwrap();
    assert_eq!((nc, nm, nf), (2, 2, 1));
    assert!(tsv.contains("Agent V Theme {to} Recipient"));
    let lx = Lex::from_tsv(&tsv, "", "");
    let cl = Clause {
        verbs: vec!["give".into()],
        subj: Some(Filler::default()),
        objs: vec![Filler::default()],
        pps: vec![("to".into(), Filler::default())],
        ..Default::default()
    };
    for stable in [false, true] {
        let m = lx.best(&cl, stable).unwrap();
        assert_eq!((m.class, m.fclass, m.frame), (1, 1, 0));
        // Match indices belong to lx; class_id()/vframe() resolve against the global Lex.
        assert_eq!(lx.classes[m.class].id, "give-13.1");
        assert_eq!(
            lx.classes[m.fclass].frames[m.frame].primary,
            "NP V NP PP.recipient"
        );
        assert_eq!(m.subj.as_deref(), Some("Agent"));
        assert_eq!(m.objs, vec![Some("Theme".into())]);
        assert_eq!(m.pps, vec![Some("Recipient".into())]);
        assert_eq!(m.score, 8);
    }
    assert!(
        lx.best(
            &Clause {
                verbs: vec!["absent".into()],
                ..cl
            },
            false
        )
        .is_none()
    );
}
#[test]
fn supplied_matcher_transports_active_and_passive_role_coordinates() {
    let lx = Lex::from_tsv(
        "class\tthrow-17\t-\tthrow\tAgent Theme\nframe\tthrow-17\tNP V NP\tAgent V Theme\t\tfixture\n",
        "",
        "",
    );
    let active = Clause {
        verbs: vec!["throw".into()],
        subj: Some(Filler::default()),
        objs: vec![Filler::default()],
        ..Default::default()
    };
    let passive = Clause {
        passive: true,
        by: Some(Filler::default()),
        objs: vec![],
        ..active.clone()
    };
    let a = lx.best(&active, false).unwrap();
    let p = lx.best(&passive, false).unwrap();
    assert_eq!(a.subj.as_deref(), Some("Agent"));
    assert_eq!(a.objs, vec![Some("Theme".into())]);
    assert_eq!(a.by, None);
    assert_eq!(p.subj.as_deref(), Some("Theme"));
    assert_eq!(p.by.as_deref(), Some("Agent"));
    assert!(p.objs.is_empty());
    assert_eq!((a.score, p.score), (4, 8));
    assert_ne!(a.subj, p.subj);
}
#[test]
fn sentence_token_pairs_select_head_agent_and_render_schema_act() {
    use {PTag as T, Rel as R, UPos as U};
    let a = tree(&[
        ("Alice", "alice", U::PROPN, T::NNP, 2, R::Nsubj),
        ("said", "say", U::VERB, T::VBD, 0, R::Root),
        ("run", "run", U::VERB, T::VB, 2, R::Ccomp),
    ]);
    let b = tree(&[
        ("Bob", "bob", U::PROPN, T::NNP, 2, R::Nsubj),
        ("said", "say", U::VERB, T::VBD, 0, R::Root),
        ("walk", "walk", U::VERB, T::VB, 2, R::Ccomp),
    ]);
    let mut fs = vec![frame("say"), frame("run"), frame("say"), frame("walk")];
    fs[0].agent = Some(ent("alice"));
    fs[2].agent = Some(ent("bob"));
    let r = Read {
        frames: fs,
        trees: vec![a, b],
        pred: vec![(0, 1), (0, 2), (1, 1), (1, 2)],
        ..Default::default()
    };
    let x = rules::apply_at(&r, Site::Frame(3), Action::AgentFromHead).unwrap();
    assert_eq!(x.frames[3].agent.as_ref().unwrap().id, "bob");
    assert!(x.frames[1].agent.is_none());
    assert!(r.frames[3].agent.is_none());
    let y = rules::apply_at(&x, Site::Frame(3), Action::KindSay(Act::Ask)).unwrap();
    assert_eq!(y.frames[3].kind, Kind::Say(Act::Ask));
    assert!(frames_md::table(&y.frames[3..]).contains("*speech: ask*"));
    let labels = schema::parse_labels(&["q", "ask", "0", "0", "pos", "none", "-"]).unwrap();
    assert_eq!(labels.act, Act::Ask);
    assert_eq!(schema::labels_tsv(&labels).split('\t').nth(1), Some("ask"));
}
#[test]
fn deletion_reindexes_all_frame_links_but_preserves_token_coordinates() {
    let mut fs = vec![frame("say"), frame("drop"), frame("remove"), frame("stay")];
    fs[0].quote = vec![1, 3];
    fs[0].cause = Some(2);
    fs[0].next = Some(3);
    fs[0].link = Some((3, Rel::Ccomp, None));
    let mut r = Read {
        frames: fs,
        pred: vec![(0, 2), (0, 4), (1, 2), (1, 4)],
        roots: vec![0, 2],
        asked: vec![
            Asked {
                word: "who".into(),
                frame: 1,
                what: Ask::Frame,
            },
            Asked {
                word: "why".into(),
                frame: 3,
                what: Ask::Cause,
            },
        ],
        ..Default::default()
    };
    rules::drop_frames(&mut r, &BTreeSet::from([1, 2]));
    assert_eq!(
        r.frames.iter().map(|f| f.verb.as_str()).collect::<Vec<_>>(),
        ["say", "stay"]
    );
    assert_eq!(r.pred, [(0, 2), (1, 4)]);
    assert_eq!(r.roots, [0]);
    assert_eq!(
        r.asked,
        vec![Asked {
            word: "why".into(),
            frame: 1,
            what: Ask::Cause
        }]
    );
    let f = &r.frames[0];
    assert_eq!(f.quote, [1]);
    assert_eq!(f.cause, None);
    assert_eq!(f.next, Some(1));
    assert_eq!(f.link, Some((1, Rel::Ccomp, None)));
    assert_eq!(f.show(10), "Event _ say | says→#11 next→#11");
}
#[test]
fn participle_site_addition_preserves_sentence_and_token_identity() {
    use {PTag as T, Rel as R, UPos as U};
    let a = tree(&[
        ("broken", "break", U::VERB, T::VBN, 2, R::Amod),
        ("cup", "cup", U::NOUN, T::NN, 0, R::Root),
    ]);
    let b = tree(&[
        ("painted", "paint", U::VERB, T::VBN, 2, R::Amod),
        ("wall", "wall", U::NOUN, T::NN, 0, R::Root),
    ]);
    let r = Read {
        trees: vec![a, b],
        ..Default::default()
    };
    assert_eq!(rules::sites(&r), [Site::Token(0, 0), Site::Token(1, 0)]);
    let x = rules::apply_at(&r, Site::Token(0, 0), Action::AddNounPatient).unwrap();
    assert_eq!(x.pred, [(0, 0)]);
    assert_eq!(x.frames.len(), 1);
    assert_eq!(x.frames[0].verb, "break");
    assert_eq!(x.frames[0].patient.as_ref().unwrap().id, "cup");
    assert!(x.frames[0].agent.is_none());
    assert_eq!(rules::sites(&x), [Site::Frame(0), Site::Token(1, 0)]);
    let md = frames_md::table(&x.frames);
    assert!(md.contains("**break**"));
    assert!(md.contains("cup"));
    assert!(!md.contains("wall"));
}
#[test]
fn oblique_pronoun_site_resolution_changes_only_the_selected_role() {
    let pron = || Ent {
        id: "she".into(),
        text: "her".into(),
        pron: Some("her".into()),
        ..Default::default()
    };
    let mut f = frame("walk");
    f.recipient = Some(pron());
    f.obls = vec![("near".into(), ent("tree")), ("with".into(), pron())];
    let r = Read {
        frames: vec![f],
        pred: vec![(0, usize::MAX)],
        before: vec![ent("alice")],
        ..Default::default()
    };
    assert_eq!(
        rules::sites(&r),
        [Site::Frame(0), Site::Ent(0, 2), Site::Ent(0, 5)]
    );
    let x = rules::apply_at(&r, Site::Ent(0, 5), Action::Resolve(Cand::Before)).unwrap();
    assert_eq!(x.frames[0].recipient, r.frames[0].recipient);
    assert_eq!(x.frames[0].obls[0], r.frames[0].obls[0]);
    assert_eq!(x.frames[0].obls[1].0, "with");
    assert_eq!(x.frames[0].obls[1].1.id, "alice");
    assert_eq!(x.frames[0].obls[1].1.pron.as_deref(), Some("her"));
    assert_eq!(
        x.frames[0].show(0),
        "Event _ walk to=she(«her») near=tree with=alice(«her»)"
    );
    assert!(frames_md::table(&x.frames).contains("with alice"));
    assert!(rules::apply_at(&x, Site::Ent(0, 5), Action::Resolve(Cand::Before)).is_none());
}
#[test]
fn extracted_frame_selected_rules_role_transfer_and_serde_preserve_identity() {
    use {PTag as T, Rel as R, UPos as U};
    let t = tree(&[
        ("Ada", "ada", U::PROPN, T::NNP, 2, R::Nsubj),
        ("carries", "carry", U::VERB, T::VBZ, 0, R::Root),
        ("box", "box", U::NOUN, T::NN, 2, R::Obj),
    ]);
    let (fs, asked, aliases) = frames::frames_of_tree(&t, &mut Ctx::default(), false);
    assert_eq!(fs.len(), 1);
    assert!(asked.is_empty());
    assert_eq!(fs[0].agent.as_ref().unwrap().id, "ada");
    assert_eq!(fs[0].patient.as_ref().unwrap().id, "box");
    let r = Read {
        frames: fs,
        asked,
        aliases,
        trees: vec![t],
        pred: vec![(0, 1)],
        ..Default::default()
    };
    let text = format!(
        "{}\nmove\taccepted\tpatient-to-to\t\t\t0\t0\tfixture\nback\tproposed\tto-to-patient\t\t\t0\t0\tfixture\nremove\trejected\tdrop\t\t\t0\t0\tfixture\n",
        rules::HEADER
    );
    assert!(rules::select_rules(&text, "off").is_empty());
    let accepted = rules::select_rules(&text, "");
    assert_eq!(
        accepted.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ["move"]
    );
    let selected = rules::select_rules(&text, "proposed");
    assert_eq!(
        selected.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ["move", "back"]
    );
    for rule in &selected {
        assert_eq!(Action::parse(&rule.action.code()), Some(rule.action));
    }
    let mut x = r.clone();
    assert_eq!(rules::apply_rule(&mut x, &accepted[0]), 1);
    assert!(x.frames[0].patient.is_none());
    assert_eq!(x.frames[0].recipient.as_ref().unwrap().id, "box");
    assert_eq!(x.frames[0].agent, r.frames[0].agent);
    assert_eq!(x.pred, r.pred);
    let encoded = serde_json::to_string(&x.frames[0]).unwrap();
    let decoded: Frame = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, x.frames[0]);
    assert_eq!(decoded.show(0), x.frames[0].show(0));
    assert_eq!(rules::apply_rule(&mut x, &selected[1]), 1);
    assert_eq!(x.frames, r.frames);
    assert_eq!(x.pred, [(0, 1)]);
}
