//! Gates for langseg that need no data on disk: script runs, transliteration, cue features, the classifier and
//! the Viterbi split on a small invented fixture, each with a negative control that must show red.

use langseg::{skeleton, strip_points, Counts, Featurizer, Groups, Lang, Model, Segmenter, HEB};

/// Invented sentences in the style of each variety (not quoted from any edition).
const TMR: &[&str] = &[
    "אמר רב מאי טעמא קא אמר הכי",
    "האי גברא איכא למימר דלא ידע",
    "לימא מסייע ליה הני מילי",
    "השתא מאי איכא למימר אמאי לא קאמר",
    "היכי דמי האי גוונא ליכא",
    "אמר ליה אביי מאי קא קשיא לך",
];
const MISH: &[&str] = &[
    "האומר הרי עלי שאין לו חייב",
    "כשהוא יוצא פטור ואם נכנס חייב",
    "שאין אדם אוסר את שלו אלא אם כן אמר",
    "הרי זה חייב לפיכך אם אמר פטור",
    "אימתי בזמן שאין לו אלא אחד",
    "כל שהוא חייב בזה פטור בזה",
];

fn model(swap: bool) -> Model {
    let fz = Featurizer::new(Groups::all());
    let mut c = Counts::default();
    let (t, m) = (Lang::Tmr.col().unwrap(), Lang::HboMishnaic.col().unwrap());
    let (t, m) = if swap { (m, t) } else { (t, m) };
    for s in TMR {
        c.add(&fz, s, t);
    }
    for s in MISH {
        c.add(&fz, s, m);
    }
    c.model(Groups::all(), 0.5, 1)
}

/// Held-out sentences of the fixture: the two that must be told apart.
const HELD: &[(&str, Lang)] = &[("מאי טעמא האי גברא קא אמר", Lang::Tmr), ("הרי זה פטור שאין לו", Lang::HboMishnaic)];

#[test]
fn script_runs_and_transliteration() {
    let text = "The king, מַלְכָּא (*malkā* \"the king\"), is in Dan 2:4; ת\"ח and ר' are abbreviations.";
    let sg = Segmenter::new(model(false), 16.0);
    let segs = sg.segment(text, 0);
    let got: Vec<(&str, &str)> = segs.iter().map(|s| (&text[s.start..s.end], s.lang.code())).collect();
    assert_eq!(got[0], ("The king", "en"));
    assert_eq!(got[1].0, "מַלְכָּא");
    assert!(got[1].1 != "en" && got[1].1 != "translit");
    assert_eq!(got[2], ("malkā", "translit"));
    assert!(got[3].0.starts_with("the king") && got[3].1 == "en", "an italic span holds the transliteration only: {got:?}");
    assert!(got.iter().any(|g| g.0 == "ת\"ח"), "gershayim inside a Hebrew word stays in the word: {got:?}");
    assert!(got.iter().any(|g| g.0 == "ר'"), "a final geresh stays in the word: {got:?}");
    // negative control: plain English with no transliteration signs is never transliteration
    let plain = "Mishnah and Talmud are read in English here.";
    assert!(sg.segment(plain, 0).iter().all(|s| s.lang == Lang::En));
    // another script is `other`
    let greek = "the word λόγος here";
    assert!(sg.segment(greek, 0).iter().any(|s| s.lang == Lang::Other && &greek[s.start..s.end] == "λόγος"));
}

#[test]
fn cue_features_fire_on_pointed_and_unpointed_forms() {
    let fz = Featurizer::new(Groups::all());
    for w in ["דִּי", "די", "וְדִי"] {
        assert!(fz.features(w).iter().any(|f| f == "C:rel_di"), "{w}");
    }
    assert!(fz.features("הַשְׁתָּא").iter().any(|f| f == "C:now_hashta"));
    assert!(fz.features("מַלְכָּא").iter().any(|f| f == "A:emph_a"));
    let accented = "וַיֹּ֣אמֶר";
    let f = fz.features(accented);
    assert!(f.iter().any(|x| x == "P:accent") && f.iter().any(|x| x == "P:niqqud") && f.iter().any(|x| x == "P:wayyiqtol"));
    // negative control: the same word without points fires no pointing feature, and the skeleton is unchanged
    let plain = strip_points(accented);
    assert_eq!(plain, "ויאמר");
    assert!(!fz.features(&plain).iter().any(|x| x.starts_with("P:")));
    assert_eq!(skeleton(accented), skeleton(&plain));
    // negative control: a word that is no cue fires no cue
    assert!(!fz.features("ספר").iter().any(|x| x.starts_with("C:")));
}

#[test]
fn classifier_learns_and_shuffled_labels_fail() {
    let sg = Segmenter::new(model(false), 16.0);
    for (s, l) in HELD {
        assert_eq!(sg.classify(s).0, *l, "{s}");
    }
    // negative control: with the labels swapped the same gate must show red on every held-out sentence
    let bad = Segmenter::new(model(true), 16.0);
    for (s, l) in HELD {
        assert_ne!(bad.classify(s).0, *l, "the swapped model still got {s} right: the gate cannot fail");
    }
}

#[test]
fn viterbi_cuts_a_planted_sentence_and_only_with_a_finite_switch() {
    let text = "אמר רב מאי טעמא קא אמר הכי האי גברא איכא למימר הרי זה חייב שאין לו אלא אם כן אמר לפיכך פטור השתא מאי איכא למימר אמאי לא קאמר";
    let sg = Segmenter::new(model(false), 4.0);
    let segs = sg.segment(text, 3);
    let langs: Vec<Lang> = segs.iter().map(|s| s.lang).collect();
    assert_eq!(langs, vec![Lang::Tmr, Lang::HboMishnaic, Lang::Tmr], "{segs:?}");
    let mid = &text[segs[1].start..segs[1].end];
    assert!(mid.starts_with("הרי זה חייב") || mid.contains("הרי זה חייב"), "{mid}");
    // every segment explains itself
    assert!(segs.iter().all(|s| s.why.as_ref().is_some_and(|w| !w.2.is_empty())));
    // negative control: an infinite switch cost cannot cut
    let one = Segmenter::new(model(false), f64::INFINITY);
    assert_eq!(one.segment(text, 0).len(), 1);
}

#[test]
fn deterministic_and_model_round_trip() {
    let m = model(false);
    let dir = std::env::temp_dir().join(format!("langseg-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("model.tsv");
    m.save(&p).unwrap();
    let back = Model::load(&p).unwrap();
    assert_eq!(m.logp.len(), back.logp.len());
    let (a, b) = (Segmenter::new(m, 4.0), Segmenter::new(back, 4.0));
    for (s, _) in HELD {
        let (la, sa) = a.classify(s);
        let (lb, sb) = b.classify(s);
        assert_eq!(la, lb);
        for k in 0..HEB.len() {
            assert!((sa[k] - sb[k]).abs() < 0.01);
        }
        let x: Vec<_> = a.segment(s, 2).iter().map(|g| (g.start, g.end, g.lang)).collect();
        let y: Vec<_> = a.segment(s, 2).iter().map(|g| (g.start, g.end, g.lang)).collect();
        assert_eq!(x, y);
    }
    std::fs::remove_dir_all(&dir).ok();
}
