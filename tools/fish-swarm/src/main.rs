use mova_fish_swarm::{now, status, Config, Result, Supervisor};
use std::{path::Path, time::Duration};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: mova-fish-swarm run|status CONFIG_JSON".into());
    }
    let c = Config::load(Path::new(&a[2]))?;
    match a[1].as_str() {
        "status" => println!("{}", serde_json::to_string_pretty(&status(&c)?)?),
        "run" => {
            let s = Supervisor::open(c)?;
            loop {
                let v = s.step(now)?;
                match v["state"].as_str().unwrap_or("invalid") {
                    "stopped" | "deadline_reached" => break,
                    "uncertain" | "failed" => return Err(format!("fail-closed: {}", v).into()),
                    _ => std::thread::sleep(Duration::from_secs(s.config.poll_seconds.min(5))),
                }
            }
        }
        _ => return Err("unknown command".into()),
    }
    Ok(())
}
