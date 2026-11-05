//! Lesson-local memoization of exercise evaluation, not of weight updates.
//! Michie (1968), doi:10.1038/218019a0 (memo functions).
//! The world, options and labels are fixed during this call. Only weights move;
//! answer() reads the 119 weights selected by its question's Ask.
//! A memo never survives the call, including when the next lesson adds facts.
use super::*;

#[derive(Default, Debug, serde::Serialize)]
pub(super) struct LearnWork {
    pub evaluated: usize,
    pub reused: usize,
    pub inert: usize,
    pub pass_ns: Vec<u64>,
}
impl LearnWork {
    pub fn add(&mut self, other: Self) {
        self.evaluated += other.evaluated;
        self.reused += other.reused;
        self.inert += other.inert;
        self.pass_ns.resize(self.pass_ns.len().max(other.pass_ns.len()), 0);
        for (a, b) in self.pass_ns.iter_mut().zip(other.pass_ns) { *a += b; }
    }
}
enum Memo {
    Empty,
    Inert,
    Scored { answer: Answer, weights: Vec<i64> },
}
impl Memo {
    fn remember(st: &PragStudent, a: Answer, right: usize) -> Self {
        // These eligibility facts depend on structural support, not weights.
        // A currently correct supported answer is NOT inert: its winner may change.
        if !a.understood || a.count.is_some() || a.options[right].best.is_none() {
            Self::Inert
        } else {
            let weights = st.ask_weights(a.ask).to_vec();
            Self::Scored { answer: a, weights }
        }
    }
    fn valid(&self, st: &PragStudent) -> bool {
        match self {
            Self::Empty => false,
            Self::Inert => true,
            Self::Scored { answer, weights } => weights == st.ask_weights(answer.ask),
        }
    }
}
fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("MOVA_LEARN_REUSE").as_deref() != Ok("0"))
}
impl PragStudent {
    fn ask_weights(&self, ask: Ask) -> &[i64] {
        let width = VIAS.len() * NF;
        &self.w[ask_idx(ask) * width..(ask_idx(ask) + 1) * width]
    }

    pub(super) fn learn_lesson(&mut self, lesson: &Lesson, passes: usize) -> LearnWork {
        self.learn_lesson_with(lesson, passes, enabled(), Self::answer_by_weights, |_| {})
    }

    fn learn_lesson_with(
        &mut self, lesson: &Lesson, passes: usize, reuse: bool,
        mut evaluate: impl FnMut(&Self, &str, &[String]) -> Answer,
        mut observe: impl FnMut(&Self),
    ) -> LearnWork {
        let mut work = LearnWork::default();
        if passes == 0 { return work; }
        let prepared: Vec<_> = lesson.exercises.iter().enumerate().map(|(i, (q, r, w))| {
            let (options, right) = shuffled((lesson.n as u64) << 20 | (i as u64) << 1, r, w);
            (q, options, right)
        }).collect();
        let mut memo: Vec<_> = prepared.iter().map(|_| Memo::Empty).collect();
        let timed = timing_on();
        for _ in 0..passes {
            let start = timed.then(std::time::Instant::now);
            for (i, (q, options, right)) in prepared.iter().enumerate() {
                if reuse && memo[i].valid(self) {
                    match &memo[i] {
                        Memo::Inert => { work.inert += 1; }
                        Memo::Scored { answer, .. } => {
                            work.reused += 1;
                            self.learn_answer(answer, *right);
                        }
                        Memo::Empty => unreachable!(),
                    }
                } else {
                    let a = evaluate(self, q, options);
                    work.evaluated += 1;
                    if reuse {
                        // Snapshot BEFORE the update. A reused wrong answer can
                        // update its own block, invalidating itself for the next pass.
                        memo[i] = Memo::remember(self, a, *right);
                        if let Memo::Scored { answer, .. } = &memo[i] {
                            self.learn_answer(answer, *right);
                        }
                    } else {
                        self.learn_answer(&a, *right);
                    }
                }
                observe(self);
            }
            if let Some(start) = start { work.pass_ns.push(start.elapsed().as_nanos() as u64); }
        }
        work
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test observations preserve timing-vector length, including zero suffixes.
    type WorkObservation = (usize, usize, usize, Vec<u64>);

    fn observe_work(work: &LearnWork) -> WorkObservation {
        (work.evaluated, work.reused, work.inert, work.pass_ns.clone())
    }

    fn report(value: &WorkObservation) -> LearnWork {
        LearnWork { evaluated: value.0, reused: value.1, inert: value.2,
            pass_ns: value.3.clone() }
    }

    #[test]
    fn accounting_add_matches_independent_table() {
        let inputs = [
            (0, 0, 0, vec![]), (1, 0, 0, vec![0]),
            (0, 2, 0, vec![3, 0]), (0, 0, 4, vec![5, 7, 0]),
        ];
        // Literal expected observations, not calculated through the operation.
        let expected = [
            (0,0,0,vec![]), (1,0,0,vec![0]), (0,2,0,vec![3,0]), (0,0,4,vec![5,7,0]),
            (1,0,0,vec![0]), (2,0,0,vec![0]), (1,2,0,vec![3,0]), (1,0,4,vec![5,7,0]),
            (0,2,0,vec![3,0]), (1,2,0,vec![3,0]), (0,4,0,vec![6,0]), (0,2,4,vec![8,7,0]),
            (0,0,4,vec![5,7,0]), (1,0,4,vec![5,7,0]), (0,2,4,vec![8,7,0]), (0,0,8,vec![10,14,0]),
        ];
        for (i, x) in inputs.iter().enumerate() {
            for (j, y) in inputs.iter().enumerate() {
                let mut actual = report(x);
                actual.add(report(y));
                assert_eq!(observe_work(&actual), expected[4*i+j], "pair {i},{j}");
            }
        }
    }

    #[test]
    fn accounting_add_preserves_ledger_under_all_three_call_partitions() {
        // Independent event ledger: count kind, optional pass, synthetic duration.
        // Grouping counts events and bins durations without calling LearnWork::add.
        let ledger = [(0,Some(0),4), (0,Some(1),7), (0,Some(2),5),
            (1,Some(0),3), (1,Some(0),4), (2,None,0)];
        for assignment in 0..729 {
            let mut owner = assignment;
            let mut calls: [Vec<usize>; 3] = Default::default();
            for event in 0..ledger.len() {
                calls[owner % 3].push(event);
                owner /= 3;
            }
            let reports: Vec<WorkObservation> = calls.iter().map(|events| {
                let counts: Vec<_> = (0..3).map(|kind|
                    events.iter().filter(|&&i| ledger[i].0 == kind).count()).collect();
                let length = events.iter().filter_map(|&i| ledger[i].1)
                    .max().map_or(0, |pass| pass + 1);
                let times = (0..length).map(|pass| events.iter()
                    .filter(|&&i| ledger[i].1 == Some(pass)).map(|&i| ledger[i].2).sum())
                    .collect();
                (counts[0], counts[1], counts[2], times)
            }).collect();
            let mut left = report(&reports[0]);
            left.add(report(&reports[1]));
            left.add(report(&reports[2]));
            let mut tail = report(&reports[1]);
            tail.add(report(&reports[2]));
            let mut right = report(&reports[0]);
            right.add(tail);
            let expected = (3, 2, 1, vec![11, 7, 5]);
            assert_eq!(observe_work(&left), expected, "left partition {assignment}");
            assert_eq!(observe_work(&right), expected, "right partition {assignment}");
            for value in &reports {
                let mut left_identity = LearnWork::default();
                left_identity.add(report(value));
                let mut right_identity = report(value);
                right_identity.add(LearnWork::default());
                assert_eq!(observe_work(&left_identity), *value);
                assert_eq!(observe_work(&right_identity), *value);
            }
        }
    }

    #[test]
    fn accounting_add_bounded_preimages_preserve_shape_and_lose_attribution() {
        let shapes = [vec![], vec![0], vec![1], vec![0,0],
            vec![1,0], vec![0,1], vec![1,1]];
        let mut domain = Vec::new();
        for e in 0..2 {
            for r in 0..2 {
                for i in 0..2 {
                    for t in &shapes { domain.push((e,r,i,t.clone())); }
                }
            }
        }
        let target = (1,1,0,vec![1,0]);
        let mut preimages = std::collections::BTreeSet::new();
        let mut pairs = 0;
        for x in &domain {
            for y in &domain {
                let mut actual = report(x);
                actual.add(report(y));
                let mut reversed = report(y);
                reversed.add(report(x));
                assert_eq!(observe_work(&actual), observe_work(&reversed));
                // Declarative converse of the fixed target; lengths are observations.
                let compatible = x.0 + y.0 == target.0 && x.1 + y.1 == target.1
                    && x.2 + y.2 == target.2 && x.3.len().max(y.3.len()) == target.3.len()
                    && target.3.iter().enumerate().all(|(j, &t)|
                        u128::from(x.3.get(j).copied().unwrap_or(0))
                        + u128::from(y.3.get(j).copied().unwrap_or(0)) == u128::from(t));
                assert_eq!(observe_work(&actual) == target, compatible, "{x:?} + {y:?}");
                if compatible { preimages.insert((x.clone(), y.clone())); }
                pairs += 1;
            }
        }
        assert_eq!(pairs, 3136);
        // Four count partitions times eight shape-aware timing partitions.
        assert_eq!(preimages.len(), 32);
        assert!(preimages.contains(&((1,0,0,vec![1]), (0,1,0,vec![0,0]))));
        assert!(preimages.contains(&((0,1,0,vec![]), (1,0,0,vec![1,0]))));
        let mut explicit_zero = LearnWork::default();
        explicit_zero.add(report(&(0,0,0,vec![0])));
        assert_ne!(observe_work(&explicit_zero), observe_work(&LearnWork::default()));
    }

    fn answer(st: &PragStudent, q: &str, opts: &[String]) -> Answer {
        let ask = if q.starts_with("cause") { Ask::Cause } else { Ask::Frame };
        let unsupported = q == "unsupported" && !st.texts.contains_key(&(999, 0));
        let options: Vec<_> = opts.iter().map(|text| {
            let f = if text == "left" { Feat::Match(Slot::Agent) } else { Feat::Match(Slot::Patient) };
            let best = if unsupported || q == "unlearnable-right" && text == "left" { None }
                else { Some((st.w[widx(ask, Via::Own, f)], 0, 0, Via::Own, vec![f])) };
            OptionRead { text: text.clone(), best, ..Default::default() }
        }).collect();
        let choice = options.iter().enumerate().filter_map(|(i,o)|o.best.as_ref().map(|b|(b.0,i)))
            .max_by(|a,b|a.0.cmp(&b.0).then(b.1.cmp(&a.1))).map(|x|x.1).unwrap_or(0);
        Answer { qframes: vec![], ask, gap: None, anchors: vec![], targets: vec![], options,
            choice, understood: !unsupported,
            count: (q == "count").then(||Count {value:7,how:"independent rule".into()}), rule: None }
    }
    fn student() -> PragStudent {
        let mut st=PragStudent::default();
        st.w.fill(0);
        st.w[widx(Ask::Frame,Via::Own,Feat::Match(Slot::Patient))]=2;
        st.w[widx(Ask::Cause,Via::Own,Feat::Match(Slot::Agent))]=3;
        st
    }
    fn lesson() -> Lesson {
        let exercises = [("frame-a","left"),("cause-a","right"),("frame-b","right"),
            ("unsupported","left"),("count","left"),("unlearnable-right","left")]
            .iter().map(|(q,r)|(q.to_string(),r.to_string(),
                vec![if *r=="left" {"right".into()} else {"left".into()}])).collect();
        Lesson { n:3, exercises, ..Default::default() }
    }
    #[test]
    fn bounded_cache_reference_traces_cover_weights_orders_seeds_and_passes() {
        // Correspondence for the groupant's weight-subspace quotient:
        // memoized evaluation must preserve every ordered update, not only totals.
        let mut cases = 0;
        for frame_weight in [-2, 0, 2] {
            for cause_weight in [-2, 0, 3] {
                for reverse in [false, true] {
                    for number in [0, 1, 3] {
                        for passes in [0, 1, 2, 5, 8] {
                            let mut l = lesson();
                            l.n = number;
                            if reverse { l.exercises.reverse(); }
                            let mut base = student();
                            base.w[widx(Ask::Frame, Via::Own, Feat::Match(Slot::Patient))] = frame_weight;
                            base.w[widx(Ask::Cause, Via::Own, Feat::Match(Slot::Agent))] = cause_weight;
                            let mut plain = base;
                            let mut cached = student();
                            cached.w.clone_from(&plain.w);
                            let (mut reference, mut actual) = (Vec::new(), Vec::new());
                            let old = plain.learn_lesson_with(&l, passes, false, answer,
                                |s| reference.push(s.w.clone()));
                            let new = cached.learn_lesson_with(&l, passes, true, answer,
                                |s| actual.push(s.w.clone()));
                            assert_eq!(actual, reference, "case {cases}");
                            assert_eq!(cached.w, plain.w, "case {cases}");
                            let count = passes * l.exercises.len();
                            assert_eq!(reference.len(), count);
                            assert_eq!(old.evaluated, count);
                            assert_eq!(new.evaluated + new.reused + new.inert, count);
                            cases += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(cases, 270);
    }

    #[test]
    fn changed_winner_invalidates_but_another_weight_block_does_not() {
        let mut st=student();
        let opts=vec!["left".into(),"right".into()];
        let a=answer(&st,"frame",&opts);
        assert_eq!(a.choice,1);
        let memo=Memo::remember(&st,a,1); // currently correct is still score-dependent
        st.w[widx(Ask::Cause,Via::Own,Feat::Match(Slot::Agent))]=100;
        assert!(memo.valid(&st));
        st.w[widx(Ask::Frame,Via::Own,Feat::Match(Slot::Agent))]=10;
        assert!(!memo.valid(&st));
        assert_eq!(answer(&st,"frame",&opts).choice,0);
        // Negative control: the cached winner would now be wrong.
        if let Memo::Scored { answer, .. }=memo {assert_ne!(answer.choice,0);} else {panic!("not scored");}
    }
    #[test]
    fn every_ordered_update_matches_recomputation_over_multiple_passes() {
        let l=lesson();
        let (mut a,mut b)=(student(),student());
        let (mut reference,mut cached)=(Vec::new(),Vec::new());
        let old=a.learn_lesson_with(&l,5,false,answer,|s|reference.push(s.w.clone()));
        let new=b.learn_lesson_with(&l,5,true,answer,|s|cached.push(s.w.clone()));
        assert_eq!(reference,cached);
        assert_eq!(reference.len(),30);
        assert!(reference.windows(2).any(|x|x[0]!=x[1]),"fixture must exercise updates");
        assert_eq!(old.evaluated,30);
        assert!(new.evaluated>l.exercises.len(),"changed blocks must be recomputed");
        assert!(new.reused>0 && new.inert>=12);
        assert_eq!(new.evaluated+new.reused+new.inert,30);
    }
    #[test]
    fn new_facts_cannot_reuse_an_inert_previous_lesson() {
        let mut st=student();
        let mut l=lesson();
        l.exercises.retain(|x|x.0=="unsupported");
        let before=st.w.clone();
        let first=st.learn_lesson_with(&l,2,true,answer,|_|{});
        assert_eq!((first.evaluated,first.inert),(1,1));
        assert_eq!(st.w,before);
        st.texts.insert((999,0),"new supporting fact".into());
        l.n+=1;
        let next=st.learn_lesson_with(&l,2,true,answer,|_|{});
        assert!(next.evaluated>=1);
        assert_eq!(next.inert,0);
        assert_ne!(st.w,before,"fresh support must permit teaching");
        let empty=st.learn_lesson_with(&l,0,true,|_,_,_|panic!("zero passes evaluated"),|_|{});
        assert_eq!(empty.evaluated+empty.reused+empty.inert,0);
    }
}
