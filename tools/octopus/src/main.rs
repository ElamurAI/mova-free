use anyhow::{Result, bail, ensure};
use mova_octopus::{Config, Deputy, socket};
use std::{
    fs,
    io::{self, Read},
    path::Path,
};
const USAGE: &str = "usage: mova-octopus plan|run CONFIG INVOCATION_ID < ordinary-english.txt\n       mova-octopus serve CONFIG SOCKET\n       mova-octopus angel CONFIG SOCKET\n       mova-octopus ask SOCKET plan|run INVOCATION_ID < ordinary-english.txt";
fn prompt() -> Result<String> {
    bounded_prompt(io::stdin())
}
/// Reads at most 8193 bytes. A prefix over the 8192-byte budget may end inside a multi-byte
/// character, so it is decoded lossily (never shorter) and the library refuses it by length;
/// input within the budget must be valid UTF-8.
fn bounded_prompt(r: impl Read) -> Result<String> {
    let mut b = Vec::new();
    r.take(8193).read_to_end(&mut b)?;
    if b.len() > 8192 {
        return Ok(String::from_utf8_lossy(&b).into_owned());
    }
    Ok(String::from_utf8(b)?)
}
fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    ensure!(!a.is_empty(), USAGE);
    match a[0].as_str() {
        "plan" | "run" if a.len() == 3 => {
            let config: Config = serde_json::from_slice(&fs::read(&a[1])?)?;
            let deputy = Deputy::load(config)?;
            let report = deputy.run(&prompt()?, &a[2], a[0] == "run")?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "serve" if a.len() == 3 => {
            let config: Config = serde_json::from_slice(&fs::read(&a[1])?)?;
            let deputy = Deputy::load(config)?;
            socket::serve(Path::new(&a[2]), &|id, execute, prompt| {
                Ok(serde_json::to_value(deputy.run(prompt, id, execute)?)?)
            })?;
        }
        "angel" if a.len() == 3 => {
            let path = a[1].clone();
            socket::angel(
                Path::new(&a[2]),
                &|| {
                    let config: Config = serde_json::from_slice(&fs::read(&path)?)?;
                    let deputy = Deputy::load(config)?;
                    Ok(move |id: &str, execute: bool, prompt: &str| {
                        Ok(serde_json::to_value(deputy.run(prompt, id, execute)?)?)
                    })
                },
                std::time::Duration::from_secs(60),
            )?;
        }
        "ask" if a.len() == 4 && ["plan", "run"].contains(&a[2].as_str()) => {
            let reply = socket::ask(Path::new(&a[1]), &a[3], a[2] == "run", &prompt()?)?;
            println!("{}", serde_json::to_string_pretty(&reply)?);
            if reply["ok"] != true {
                std::process::exit(1);
            }
        }
        _ => bail!(USAGE),
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::bounded_prompt;
    /// A two-byte character straddling byte 8193 yields an over-budget prompt, not a UTF-8 error
    /// (red with `read_to_string` on the 8193-byte prefix).
    #[test]
    fn over_budget_prefix_cut_inside_a_character_stays_over_budget() {
        let input = format!("{}\u{e9}{}", "a".repeat(8192), "b".repeat(10));
        let p = bounded_prompt(input.as_bytes()).unwrap();
        assert!(p.len() > 8192);
        let fits = "a".repeat(8190) + "\u{e9}";
        assert_eq!(bounded_prompt(fits.as_bytes()).unwrap(), fits);
        assert!(bounded_prompt(&[b'a', 0xff][..]).is_err());
    }
}
