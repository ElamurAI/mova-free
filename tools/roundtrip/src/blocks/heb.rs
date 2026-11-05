//! Hebrew blocks: the WLC chapters through the Hebrew route of `roundtrip mixed` (`hbo` segmenter, `gparse`,
//! `macula prag`, one world per volume). A block's verses are the lines of one volume; the verse unit makes every
//! verse a volume of its own; the shuffle permutes the verses inside a block; the damage drops the Hebrew tree
//! residual in the exact decode (the gate's negative control of `mixed`).

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use en::annotate::Annotator;
use langseg::{Rng, Segmenter, fnv};

use super::Block;
use crate::mixed::link::{self, LinkMode};
use crate::mixed::route::Mode;
use crate::mixed::sample::{self, Item, Span, Stream};
use crate::mixed::{self, heb as mheb};
use crate::tally::Tally;
use crate::vol::Piece;

pub struct Paths {
    pub judaism: PathBuf,
    pub hbo: PathBuf,
    pub langseg: PathBuf,
    pub gparse: String,
    /// where the parses go
    pub out: PathBuf,
}

/// The Hebrew runs, by name (`block`, `sentence`, `shuffle`, `damage`): one tally a block, in the blocks' order.
pub type Runs = BTreeMap<String, Vec<Tally>>;

fn lines_of(b: &Block) -> Vec<String> {
    b.vol.pieces.iter().filter_map(|p| if let Piece::Text(k) = p { Some(b.vol.blocks[*k].text.clone()) } else { None }).collect()
}

fn tally_of(runs: &[mixed::Run]) -> Tally {
    let mut t = Tally { volumes: 1, ..Default::default() };
    let mut ok = true;
    for r in runs {
        for x in r.by.values() {
            t.merge(&Tally { volumes: 0, exact_ok: 0, ..x.clone() });
        }
        if r.exact_ok != r.volumes {
            ok = false;
            t.exact_fail.extend(r.fails.iter().take(1).cloned());
        }
    }
    t.exact_ok = ok as u64;
    t
}

pub fn run(a: &Annotator, blocks: &[Block], p: &Paths, threads: usize, modes: &[&str]) -> Result<Runs> {
    let var = sample::variety("hbo-accented").unwrap();
    let mut items = Vec::new();
    let mut st = Stream { name: "hbo-blocks".into(), lines: Vec::new(), gold: Vec::new() };
    let mut ranges = Vec::new();
    for b in blocks {
        let a0 = st.lines.len();
        for (k, l) in lines_of(b).into_iter().enumerate() {
            let n = l.len();
            items.push(Item { id: format!("{}:{k}", b.id), var, text: l.clone(), verse: None });
            st.gold.push(vec![Span { start: 0, end: n, item: Some(items.len() - 1) }]);
            st.lines.push(l);
        }
        ranges.push(a0..st.lines.len());
    }
    // the shuffled stream: the verses of every block permuted inside it, each with its own item
    let mut sh = Stream { name: "hbo-shuffled".into(), lines: Vec::new(), gold: Vec::new() };
    for (b, r) in blocks.iter().zip(&ranges) {
        let mut idx: Vec<usize> = r.clone().collect();
        Rng(0x5b10c ^ fnv(&b.id)).shuffle(&mut idx);
        for i in idx {
            sh.lines.push(st.lines[i].clone());
            sh.gold.push(st.gold[i].clone());
        }
    }
    let lm = langseg::Model::load(&p.langseg).map_err(|e| anyhow!(e))?;
    let sg = Segmenter::new(lm, 16.0);
    let mut models = BTreeMap::new();
    let mut lemmas = std::collections::BTreeSet::new();
    for lvl in ["pointed", "unpointed"] {
        let m = mheb::HModel::load(&p.hbo.join(lvl))?;
        lemmas.extend(m.lemmas.iter().cloned());
        models.insert(p.hbo.join(lvl), m);
    }
    let _ = crate::class::HEB_LEMMAS.set(lemmas);
    let lex = link::Lex::load(&p.judaism)?;
    let mut ctx = mixed::Ctx { a, sg: &sg, models, hbo_dir: p.hbo.clone(), cache: BTreeMap::new(), lex, items, rules: macula::prag::Rules::new(&[], &[]), quote_cut: true, damage: false };
    let cores = mixed::hebrew_cores(&ctx, &st, Mode::Oracle, 1);
    std::fs::create_dir_all(&p.out)?;
    mixed::parse_cores(&mut ctx, &cores, &p.gparse, &p.out)?;
    let mut out = Runs::new();
    for &m in modes {
        ctx.damage = m == "damage";
        let (s, unit) = match m {
            "shuffle" => (&sh, false),
            "sentence" => (&st, true),
            _ => (&st, false),
        };
        // the volumes of a block: the block, or every verse alone
        let jobs: Vec<(usize, std::ops::Range<usize>)> =
            ranges.iter().enumerate().flat_map(|(b, r)| if unit { r.clone().map(|i| (b, i..i + 1)).collect::<Vec<_>>() } else { vec![(b, r.clone())] }).collect();
        let res = par(&jobs, threads, |j, (_, r)| mixed::run_volume(&ctx, s, r.clone(), Mode::Oracle, LinkMode::Align, j))?;
        let mut per: Vec<Vec<mixed::Run>> = (0..blocks.len()).map(|_| Vec::new()).collect();
        for ((b, _), r) in jobs.iter().zip(res) {
            per[*b].push(r);
        }
        let dmg = ctx.damage as u64;
        out.insert(m.to_string(), per.iter().map(|rs| Tally { damaged: dmg, ..tally_of(rs) }).collect());
        eprintln!("hbo / {m}: {} blocks, exact {}", blocks.len(), out[m].iter().filter(|t| t.exact_ok == 1).count());
    }
    Ok(out)
}

/// `f` over the jobs on `threads` threads; the results in the jobs' order.
pub fn par<J: Sync, R: Send>(jobs: &[J], threads: usize, f: impl Fn(usize, &J) -> Result<R> + Sync) -> Result<Vec<R>> {
    par_each(jobs, threads, f).into_iter().collect()
}

/// `f` over the jobs on `threads` threads: every job's result (a panic is its error), in the jobs' order.
pub fn par_each<J: Sync, R: Send>(jobs: &[J], threads: usize, f: impl Fn(usize, &J) -> Result<R> + Sync) -> Vec<Result<R>> {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<Result<R>>>> = Mutex::new((0..jobs.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= jobs.len() {
                        break;
                    }
                    // a panic is this job's error, not the run's end
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(i, &jobs[i]))).unwrap_or_else(|p| {
                        let m = p.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| p.downcast_ref::<String>().cloned()).unwrap_or_default();
                        Err(anyhow!("panic: {m}"))
                    });
                    out.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    out.into_inner().unwrap().into_iter().map(|r| r.unwrap()).collect()
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn ordered_jobs_and_panics() {
        for threads in [0, 1, 3] {
            let rs = par_each(&[7, 8, 9], threads, |i, x| {
                if i == 1 {
                    panic!("synthetic");
                }
                Ok(i + x)
            });
            assert_eq!(*rs[0].as_ref().unwrap(), 7);
            assert!(
                rs[1]
                    .as_ref()
                    .unwrap_err()
                    .to_string()
                    .contains("synthetic")
            );
            assert_eq!(*rs[2].as_ref().unwrap(), 11);
            assert_eq!(par(&[2, 4], threads, |i, x| Ok(i + x)).unwrap(), [2, 5]);
        }
        assert!(par_each::<u8, u8>(&[], 0, |_, x| Ok(*x)).is_empty());
        assert_eq!(tally_of(&[]).exact_ok, 1);
    }
}
