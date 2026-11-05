//! A bounded CPU reading smoke for a staged lesson, including three controls.
//! This is training-source diagnostic evidence, not an independent test benchmark.
use anyhow::{Result,ensure};
use std::{collections::BTreeSet,env,fs,path::PathBuf};
use world::knowledge::{Lesson,PragStudent,shuffled,key_line,control_red};
fn teach(l:&Lesson,read:bool)->Result<PragStudent> {
 let mut s=PragStudent::default();let mut texts=Vec::new();
 for x in l.explain.iter().chain(&l.facts){texts.push((x.clone(),false));}
 for(q,r,w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
  texts.push((q.clone(),true));texts.push((r.clone(),false));
  texts.extend(w.iter().cloned().map(|x|(x,false)));
 }
 s.preparse(&texts)?;s.read_kinds();if read{s.read(l);}
 for _ in 0..2 {for(i,(q,r,w)) in l.exercises.iter().enumerate(){
  let(o,at)=shuffled((l.n as u64)<<20|(i as u64)<<1,r,w);s.learn(q,&o,at);
 }}
 Ok(s)
}
fn score(s:&PragStudent,l:&Lesson,other_question:bool)->(usize,usize){
 let(mut right,mut understood)=(0,0);
 for (questions,extra) in [(&l.tests,0),(&l.exams,0xE8A3)] {
  for(i,(q,r,w)) in questions.iter().enumerate() {
   let(o,at)=shuffled(((l.n as u64)<<20|(i as u64)<<1|1)^extra,r,w);
   let query=if other_question{&questions[(i+1)%questions.len()].0}else{q};
   let(choice,u)=s.choose(query,&o);right+=usize::from(choice==at);understood+=usize::from(u);
  }
 }
 (right,understood)
}
fn main()->Result<()> {
 let root=PathBuf::from(env::args().nth(1).expect("staged run directory"));
 let status=mova_enrich::status(&root)?;ensure!(status["state"]=="staged","not staged");
 let lesson=world::knowledge::parse(&fs::read_to_string(root.join("staged/books.lesson"))?,"pilot.lesson")?;
 let lines:Vec<_>=lesson.explain.iter().chain(&lesson.facts).cloned().collect();
 let removed:BTreeSet<_>=lesson.tests.iter().chain(&lesson.exams).filter_map(|(q,r,_)|key_line(&lines,q,r)).collect();
 let mut cut=lesson.clone();let n=lesson.explain.len();
 cut.explain=lesson.explain.iter().enumerate().filter(|(i,_)|!removed.contains(i)).map(|(_,x)|x.clone()).collect();
 cut.facts=lesson.facts.iter().enumerate().filter(|(i,_)|!removed.contains(&(i+n))).map(|(_,x)|x.clone()).collect();
 let t=std::time::Instant::now();
 let real=teach(&lesson,true)?;let no_lesson=teach(&lesson,false)?;let no_key=teach(&cut,true)?;
 let (right,understood)=score(&real,&lesson,false);let total=lesson.tests.len()+lesson.exams.len();
 let controls=[("no lesson",score(&no_lesson,&lesson,false).0),("questions shuffled",score(&real,&lesson,true).0),("key lines removed",score(&no_key,&lesson,false).0)];
 let v=serde_json::json!({"right":right,"total":total,"understood":understood,"seconds":t.elapsed().as_secs_f64(),
  "controls":controls.iter().map(|(name,n)|serde_json::json!({"name":name,"right":n,"red":control_red(right,total,*n,total)})).collect::<Vec<_>>(),
  "all_controls_red":controls.iter().all(|(_,n)|control_red(right,total,*n,total)),
  "scope":"One staged lesson, CPU parser, training-source resubstitution smoke; not held-out quality or whole-course throughput."});
 mova_enrich::write_new(&root.join("lesson-smoke.json"),&serde_json::to_vec_pretty(&v)?)?;
 println!("{}",serde_json::to_string_pretty(&v)?);Ok(())
}
