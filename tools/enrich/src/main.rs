use anyhow::{Result,bail};
use std::{env,path::Path};
fn main()->Result<()> {
 let start=std::time::Instant::now();let result=run();
 eprintln!("mova-enrich wall_seconds={:.6}",start.elapsed().as_secs_f64());result
}
fn run()->Result<()> {
 let a:Vec<_>=env::args().skip(1).collect();
 match a.first().map(String::as_str) {
  Some("study") if a.len()==3||a.len()==4=>mova_enrich::study::study_with_base(Path::new(&a[1]),Path::new(&a[2]),a.get(3).map(|p|Path::new(p))),
  Some("finish-improve") if a.len()==2=>mova_enrich::study::finish_improve(Path::new(&a[1])),
  Some("study-exam") if a.len()==4=>mova_enrich::study::exam_process(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3])),
  Some("prepare-next") if a.len()==4=>mova_enrich::study::prepare_next(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3])),
  Some("improve") if a.len()==3||a.len()==4=>mova_enrich::study::improve_with_base(Path::new(&a[1]),Path::new(&a[2]),a.get(3).map(|p|Path::new(p))),
  Some("prepare") if a.len()==6 => mova_enrich::prepare(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),a[5].parse()?),
  Some("check-source") if a.len()==5=>mova_enrich::check_source(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
  Some("run") if a.len()==6=>{
   mova_enrich::prepare(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),a[5].parse()?)?;
   mova_enrich::review(Path::new(&a[4]))?;mova_enrich::stage(Path::new(&a[4]))
  },
  Some("refine") if a.len()==3=>mova_enrich::refine(Path::new(&a[1]),Path::new(&a[2])),
  Some("require-staged") if a.len()==2=>mova_enrich::require_staged(Path::new(&a[1])),
  Some("restage") if a.len()==3=>mova_enrich::restage(Path::new(&a[1]),Path::new(&a[2])),
  Some("profile") if a.len()==2=>{println!("{}",serde_json::to_string_pretty(&mova_enrich::profile(Path::new(&a[1]))?)?);Ok(())},
  Some("review") if a.len()==2=>mova_enrich::review(Path::new(&a[1])),
  Some("stage") if a.len()==2=>mova_enrich::stage(Path::new(&a[1])),
  Some("status") if a.len()==2=>{println!("{}",serde_json::to_string_pretty(&mova_enrich::status(Path::new(&a[1]))?)?);Ok(())},
  Some("simulate") if a.len()==2=>mova_enrich::write_new(Path::new(&a[1]),&serde_json::to_vec_pretty(&mova_enrich::simulate())?),
  _=>bail!("usage: mova-enrich study STAGED NEW_STUDY [BASE_MODEL] | study-exam STATE EXAM NEW_REPORT | finish-improve ROOT | prepare-next STAGED STUDY NEW_REVIEW | improve STAGED NEW_ROOT [BASE_MODEL] | prepare|run ROUND DICTIONARY SCHOOL NEW_RUN MAX_SENTENCES | check-source RUN ROUND DICTIONARY SCHOOL | review RUN | stage RUN | refine OLD_RUN NEW_RUN | require-staged RUN | restage OLD_RUN NEW_RUN | status RUN | profile RUN | simulate NEW_JSON"),
 }
}
