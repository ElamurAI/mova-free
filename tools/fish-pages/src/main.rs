use mova_fish_pages::{Config, Pager, Result};
use std::{path::Path, time::Duration};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: mova-fish-pages run|status CONFIG".into());
    }
    let c = Config::load(Path::new(&a[2]))?;
    match a[1].as_str() {
        "status" => println!("{}", std::fs::read_to_string(c.root.join("status.json"))?),
        "run" => {
            let mut p = Pager::open(c)?;
            loop {
                let s = p.step(mova_fish_swarm::now)?;
                if matches!(s["state"].as_str(), Some("stopped" | "deadline_reached")) {
                    break;
                }
                if s["state"] == "uncertain" {
                    return Err(
                        "pager uncertain; inspect retained attempts before any restart".into(),
                    );
                }
                if !matches!(
                    s["state"].as_str(),
                    Some("job_completed" | "page_completed")
                ) {
                    std::thread::sleep(Duration::from_secs(p.config.poll_seconds.min(5)));
                }
            }
        }
        _ => return Err("unknown command".into()),
    }
    Ok(())
}
