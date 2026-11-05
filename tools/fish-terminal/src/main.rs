use anyhow::{Result, bail, ensure};
use peta_fish_terminal::{Context, terminal::Tmux};
use std::{path::Path, thread, time::Duration};
fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    ensure!(
        a.len() >= 2,
        "usage: peta-fish-terminal prepare|launch|once|watch|status|review CONFIG [FISH|TASK DECISION NOTE]"
    );
    let ctx = Context::load(Path::new(&a[1]))?;
    let mut term = Tmux::new(&ctx.config);
    let value = match a[0].as_str() {
        "prepare" if a.len() == 2 => {
            ctx.prepare()?;
            ctx.status()?
        }
        "launch" if a.len() == 3 => ctx.launch(&a[2], &mut term)?,
        "once" if a.len() == 2 => ctx.once(&mut term)?,
        "status" if a.len() == 2 => ctx.status()?,
        "review" if a.len() == 5 => ctx.review(&a[2], &a[3], &a[4])?,
        "watch" if a.len() == 2 => loop {
            if ctx.config.root.join("STOP").exists() {
                println!("{}", ctx.status()?);
                return Ok(());
            }
            println!("{}", ctx.once(&mut term)?);
            thread::sleep(Duration::from_millis(ctx.config.poll_ms));
        },
        _ => bail!("invalid bounded command arguments"),
    };
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
