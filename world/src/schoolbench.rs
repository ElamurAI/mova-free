//! Multi-fidelity school probes: small samples between periodic larger/full checks.
//! Li et al. (2018), Hyperband, https://jmlr.org/papers/v18/16-558.html:
//! allocate cheap evaluation budgets before expensive ones. This module uses a
//! fixed cadence, not Hyperband's adaptive candidate elimination.
//! Sampling whole lessons changes the learning trajectory; scores are proxies.
use crate::{
    knowledge::Lesson,
    parsecache::{self, Cache},
};
use anyhow::{Context, Result, bail, ensure};
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Quick,
    Medium,
    Full,
}
impl Tier {
    pub fn for_round(round: u64) -> Self {
        if round % 20 == 0 {
            Self::Full
        } else if round % 5 == 0 {
            Self::Medium
        } else {
            Self::Quick
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Medium => "medium",
            Self::Full => "full",
        }
    }
    fn divisor(self) -> usize {
        match self {
            Self::Quick => 8,
            Self::Medium => 3,
            Self::Full => 1,
        }
    }
    fn parse(s: &str) -> Result<Self> {
        match s {
            "quick" => Ok(Self::Quick),
            "medium" => Ok(Self::Medium),
            "full" => Ok(Self::Full),
            _ => bail!("tier must be quick, medium or full"),
        }
    }
}

/// Stable hash ranks within difficulty strata, then original chronological order.
/// The same seed yields nested subsets and the same lesson/question identities.
pub fn selection(ls: &[Lesson], tier: Tier, seed: u64) -> Vec<usize> {
    let mut groups: BTreeMap<usize, Vec<(String, usize)>> = BTreeMap::new();
    for (i, l) in ls.iter().enumerate() {
        let key = format!("{seed}\0{}\0{}", l.course, l.n);
        groups
            .entry(l.stars)
            .or_default()
            .push((parsecache::hash(key.as_bytes()), i));
    }
    let mut chosen = Vec::new();
    for group in groups.values_mut() {
        group.sort();
        chosen.extend(
            group
                .iter()
                .take(group.len().div_ceil(tier.divisor()))
                .map(|x| x.1),
        );
    }
    chosen.sort_unstable();
    chosen
}
struct Options {
    course: String,
    round: Option<u64>,
    tier: Option<Tier>,
    seed: u64,
    plan: bool,
    cache: bool,
}
fn options(args: &[String]) -> Result<Options> {
    let mut o = Options {
        course: args.first().context("bench needs a course")?.clone(),
        round: None,
        tier: None,
        seed: 42,
        plan: false,
        cache: true,
    };
    ensure!(
        !o.course.is_empty()
            && o.course
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        "invalid course name"
    );
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--plan" => o.plan = true,
            "--no-cache" => o.cache = false,
            "--round" | "--tier" | "--seed" => {
                let v = args.get(i + 1).context("missing benchmark option value")?;
                match args[i].as_str() {
                    "--round" => {
                        let n = v.parse()?;
                        ensure!(n > 0, "round must be positive");
                        o.round = Some(n);
                    }
                    "--tier" => o.tier = Some(Tier::parse(v)?),
                    _ => o.seed = v.parse()?,
                }
                i += 1;
            }
            x => bail!("unknown benchmark option {x}"),
        }
        i += 1;
    }
    ensure!(
        o.round.is_none() || o.tier.is_none(),
        "use --round or --tier, not both"
    );
    Ok(o)
}
struct Lock(PathBuf);
impl Lock {
    fn take(path: PathBuf) -> Result<Self> {
        fs::create_dir(&path).with_context(||format!("benchmark already active or stale lock {}; remove only after checking the old process",path.display()))?;
        fs::write(path.join("pid"), std::process::id().to_string())?;
        Ok(Self(path))
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn cursor(root: &Path) -> Result<u64> {
    match fs::read_to_string(root.join("completed-round")) {
        Ok(x) => Ok(x.trim().parse().context("invalid benchmark cursor")?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(e) => Err(e.into()),
    }
}
fn counts(ls: &[Lesson]) -> (usize, usize, usize) {
    (
        ls.iter().map(|l| l.exercises.len() * 2).sum(),
        ls.iter().map(|l| l.tests.len()).sum(),
        ls.iter().map(|l| l.exams.len()).sum(),
    )
}
/// Aggregate the recorded rows only; non-exam rows retain the legacy test bucket.
fn score_trace(text: &str, tests: usize, exams: usize) -> Result<(usize, usize, usize, usize)> {
    let (mut test_right, mut exam_right, mut test_n, mut exam_n) = (0, 0, 0, 0);
    for line in text.lines().skip(1).filter(|l| !l.starts_with('#')) {
        let cells: Vec<_> = line.split('\t').collect();
        ensure!(cells.len() == 15, "invalid benchmark trace");
        let right = usize::from(cells[11] == "right");
        if cells[3] == "exam" {
            exam_n += 1;
            exam_right += right;
        } else {
            test_n += 1;
            test_right += right;
        }
    }
    ensure!(
        (test_n, exam_n) == (tests, exams),
        "benchmark produced a partial trace"
    );
    Ok((test_right, exam_right, test_n, exam_n))
}
pub fn main_args(args: &[String]) -> Result<()> {
    let started = Instant::now();
    let o = options(args)?;
    let root = std::env::var_os("MOVA_BENCH_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| parsecache::data_root().join("runs/school-bench"));
    let schedule = root.join(&o.course).join(format!("seed-{}", o.seed));
    let _lock = if o.plan {
        None
    } else {
        fs::create_dir_all(&schedule)?;
        Some(Lock::take(schedule.join("running"))?)
    };
    let auto = o.round.is_none() && o.tier.is_none();
    let round = if let Some(n) = o.round {
        n
    } else {
        cursor(&schedule)?
            .checked_add(1)
            .context("round overflow")?
    };
    let tier = o.tier.unwrap_or_else(|| Tier::for_round(round));
    let all = crate::knowledge::course(&o.course)?;
    ensure!(!all.is_empty(), "empty or unknown course {}", o.course);
    let chosen = selection(&all, tier, o.seed);
    let selected: Vec<_> = chosen.iter().map(|&i| all[i].clone()).collect();
    let (updates, tests, exams) = counts(&selected);
    let controls = tier != Tier::Quick;
    let before = tier == Tier::Full;
    let settings: BTreeMap<_, _> = [
        "MOVA_DATA",
        "MOVA_EXEC",
        "WORLD_EN_MODEL",
        "MOVA_GPARSE_MODEL",
        "MOVA_CAND_CAP",
        "MOVA_ALIGN",
        "MOVA_STRENGTH",
        "MOVA_LEARN_REUSE",
        "MOVA_STUDENT_FORGET",
        "PRAG_OFF",
        "PRAG_VN",
        "WORLD_NO_REPAIR",
        "WORLD_REPAIR",
        "WORLD_DOMAIN",
        "WORLD_RULES",
        "GLOBAL_HOT",
        "GP_CPU",
        "GP_BATCH",
        "GP_FUSED_SOFTMAX",
        "GP_GELU",
        "CUDA_VISIBLE_DEVICES",
        "CUDA_CACHE_PATH",
        "CUDA_CACHE_DISABLE",
        "HOME",
        "MOVA_TIMING",
    ]
    .into_iter()
    .map(|k| (k, std::env::var(k).ok()))
    .collect();
    let plan = json!({
        "schema":1,"environment":settings,"course":o.course,"tier":tier.name(),"approximate":tier!=Tier::Full,
        "round":round,"automatic_cadence":auto,"seed":o.seed,
        "lessons":selected.iter().map(|l|l.n).collect::<Vec<_>>(),"total_lessons":all.len(),
        "training_updates":updates,"tests":tests,"exams":exams,
        "controls":controls,"before_tests":before,"cache":o.cache,"refresh_parses":tier==Tier::Full,
        "course_sha256":parsecache::hash(format!("{all:?}").as_bytes()),
        "scope":"whole-lesson subset; chronological order retained; learning state differs from full course"
    });
    if o.plan {
        println!("{}", serde_json::to_string_pretty(&plan)?);
        return Ok(());
    }
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let run = schedule.join(format!(
        "{}-{round:06}-{stamp}",
        if auto { "round" } else { "manual" }
    ));
    fs::create_dir(&run)?;
    fs::write(run.join("plan.json"), serde_json::to_vec_pretty(&plan)?)?;
    let mut cache = if o.cache {
        Some(Cache::open(
            &root.join(&o.course).join("parse-cache"),
            tier == Tier::Full,
        )?)
    } else {
        None
    };
    let trace = run.join("trace.tsv");
    let compute_start = Instant::now();
    let report = crate::knowledge::run_course_lessons(
        &o.course,
        &selected,
        Some(&trace),
        controls,
        before,
        cache.as_mut(),
        false,
    )?;
    let compute_seconds = compute_start.elapsed().as_secs_f64();
    if let Some(c) = &cache {
        c.finish()?;
    }
    let mut text = fs::read_to_string(&trace)?;
    let (test_right, exam_right, test_n, exam_n) = score_trace(&text, tests, exams)?;
    let controls_red = controls.then(|| {
        text.lines().filter(|l| l.starts_with("#control\t")).count() == 4
            && text
                .lines()
                .filter(|l| l.starts_with("#control\t"))
                .all(|l| l.ends_with("\tred"))
    });
    text += &format!(
        "#measurement\ttier={}\tseed={}\tlessons={}/{}\tcontrols={}\n",
        tier.name(),
        o.seed,
        selected.len(),
        all.len(),
        controls
    );
    fs::write(&trace, &text)?;
    let headline = format!(
        "school benchmark: tier={}, lessons={}/{}, seed={}, controls={}, before-tests={}, approximate={}\n",
        tier.name(),
        selected.len(),
        all.len(),
        o.seed,
        controls,
        before,
        tier != Tier::Full
    );
    fs::write(run.join("report.txt"), format!("{headline}{report}"))?;
    let elapsed = started.elapsed().as_secs_f64();
    let result = json!({
        "schema":1,"plan":plan,"wall_seconds":elapsed,"compute_seconds":compute_seconds,
        "tests":{"right":test_right,"total":test_n},"exams":{"right":exam_right,"total":exam_n},
        "controls_red":controls_red,"trace_sha256":parsecache::hash(text.as_bytes()),
        "parse_cache":{"hits":cache.as_ref().map_or(0,|c|c.hits),"misses":cache.as_ref().map_or(0,|c|c.misses),
            "discarded_corrupt":cache.as_ref().is_some_and(|c|c.repaired)},
        "binary_sha256":parsecache::file_hash(&std::env::current_exe()?)?,
        "parser_signature":cache.as_ref().map(|c|c.signature()),
        "run":run
    });
    crate::store::write_atomic(
        &run.join("result.json"),
        &serde_json::to_vec_pretty(&result)?,
    )?;
    // Only a technically completed run advances the automatic schedule.
    // Explicit --round/--tier probes do not consume its periodic full check.
    if auto {
        crate::store::write_atomic(
            &schedule.join("completed-round"),
            round.to_string().as_bytes(),
        )?;
    }
    println!("{headline}{report}");
    println!(
        "benchmark: {:.3}s wall, {:.3}s compute; result {}",
        elapsed,
        compute_seconds,
        run.join("result.json").display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cadence_has_sixteen_quick_three_medium_one_full() {
        let mut counts = [0; 3];
        for n in 1..=20 {
            counts[match Tier::for_round(n) {
                Tier::Quick => 0,
                Tier::Medium => 1,
                Tier::Full => 2,
            }] += 1;
        }
        assert_eq!(counts, [16, 3, 1]);
        assert_eq!(Tier::for_round(40), Tier::Full);
    }
    #[test]
    fn subsets_are_nested_ordered_cover_difficulty_and_keep_ids() {
        let ls: Vec<_> = (0..96)
            .map(|i| Lesson {
                course: "fixture".into(),
                n: i + 1,
                stars: i % 4,
                ..Default::default()
            })
            .collect();
        let q = selection(&ls, Tier::Quick, 42);
        let m = selection(&ls, Tier::Medium, 42);
        assert_eq!(q.len(), 12);
        assert_eq!(m.len(), 32);
        assert!(q.windows(2).all(|w| w[0] < w[1]));
        assert!(q.iter().all(|i| m.contains(i)));
        for stars in 0..4 {
            assert!(q.iter().any(|&i| ls[i].stars == stars));
        }
        assert_eq!(
            selection(&ls, Tier::Full, 42),
            (0..ls.len()).collect::<Vec<_>>()
        );
        assert_eq!(q, selection(&ls, Tier::Quick, 42));
        assert_ne!(q, selection(&ls, Tier::Quick, 7));
        assert!(options(&["fixture".into(), "--round".into(), "0".into()]).is_err());
        assert!(options(&["../fixture".into()]).is_err());
    }
}

#[cfg(test)]
mod wave9_models {
    use super::*;
    #[test]
    fn cadence_fibers() {
        let cycle = [
            Tier::Full,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Medium,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Medium,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Medium,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
            Tier::Quick,
        ];
        for r in (0..400).chain([u64::MAX]) {
            assert_eq!(Tier::for_round(r), cycle[(r % 20) as usize]);
        }
        assert_eq!(
            (1..=100)
                .filter(|&r| Tier::for_round(r) == Tier::Full)
                .collect::<Vec<_>>(),
            vec![20, 40, 60, 80, 100]
        );
    }
    #[test]
    fn selection_rank_oracle() {
        for len in 0..=48 {
            for seed in [0, 42, u64::MAX] {
                let ls: Vec<_> = (0..len)
                    .map(|i| Lesson {
                        course: "synthetic".into(),
                        n: i / 2,
                        stars: i % 3,
                        ..Default::default()
                    })
                    .collect();
                for (tier, divisor) in [(Tier::Quick, 8), (Tier::Medium, 3), (Tier::Full, 1)] {
                    let expected: Vec<_> = (0..len)
                        .filter(|&i| {
                            let peers: Vec<_> =
                                (0..len).filter(|&j| ls[j].stars == ls[i].stars).collect();
                            let key = |j: usize| {
                                (
                                    parsecache::hash(
                                        format!("{seed}\0{}\0{}", ls[j].course, ls[j].n).as_bytes(),
                                    ),
                                    j,
                                )
                            };
                            let rank = peers.iter().filter(|&&j| key(j) < key(i)).count();
                            rank * divisor < peers.len()
                        })
                        .collect();
                    assert_eq!(selection(&ls, tier, seed), expected);
                }
            }
        }
    }
    #[test]
    fn option_refusal_and_overwrite() {
        let parse = |a: &[&str]| options(&a.iter().map(|x| x.to_string()).collect::<Vec<_>>());
        for a in [
            vec![],
            vec![""],
            vec!["../x"],
            vec!["x", "--round", "0"],
            vec!["x", "--seed", "-1"],
            vec!["x", "--tier", "Quick"],
            vec!["x", "--tier"],
            vec!["x", "--round", "1", "--tier", "quick"],
        ] {
            assert!(parse(&a).is_err());
        }
        let o = parse(&["x", "--seed", "0", "--seed", "42", "--plan", "--no-cache"]).unwrap();
        assert_eq!(o.seed, 42);
        assert!(o.plan);
        assert!(!o.cache);
        assert!(parse(&["x", "--round", "18446744073709551616"]).is_err());
        assert_eq!(counts(&[]), (0, 0, 0));
    }
    #[test]
    fn recorded_score_scope() {
        let row = |kind: &str, right: &str| {
            let mut c = vec!["x"; 15];
            c[3] = kind;
            c[11] = right;
            c.join("\t")
        };
        for kind in ["test", "before", "unknown", "exam"] {
            for right in ["right", "wrong", "RIGHT", ""] {
                let text = format!("header\n{}\n#control\tx\tred\n", row(kind, right));
                let (t, e) = if kind == "exam" { (0, 1) } else { (1, 0) };
                let r = usize::from(right == "right");
                assert_eq!(score_trace(&text, t, e).unwrap(), (t * r, e * r, t, e));
                assert!(score_trace(&text, t + 1, e).is_err());
            }
        }
        assert_eq!(score_trace("", 0, 0).unwrap(), (0, 0, 0, 0));
        assert!(score_trace("header\nshort\n", 0, 0).is_err());
    }
}
