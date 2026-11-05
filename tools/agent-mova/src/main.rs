//! CLI for bounded native capabilities. Concrete inputs/configuration stay external.
use anyhow::{Context, Result, bail};
use std::path::Path;
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("capabilities") if a.len() == 1 => println!(
            "{}",
            serde_json::json!({"schema":1,"capabilities":["world_location_trace","retained_location_recall","bounded_report_collection"],"general_planner":false,"automatic_semantic_correction":false})
        ),
        Some("example") if a.len() == 2 => agent_mova::example(Path::new(&a[1]))?,
        Some("trace") if a.len() == 3 => {
            let task = serde_json::from_slice(&agent_mova::read_bounded(Path::new(&a[1]))?)?;
            let r = agent_mova::run_trace(&task, Path::new(&a[2]))?;
            println!("{}", serde_json::to_string(&r)?);
        }
        Some("prepare-synthetic-recall") if a.len() == 4 => {
            let task = agent_mova::prepare_synthetic_recall(
                Path::new(&a[1]),
                Path::new(&a[2]),
                "synthetic-recall",
                "specialist-whale",
            )?;
            agent_mova::json_new(Path::new(&a[3]), &task)?;
        }
        Some("recall") if a.len() == 5 => {
            let task = serde_json::from_slice(&agent_mova::read_bounded(Path::new(&a[1]))?)?;
            let r = agent_mova::recall(&task, Path::new(&a[2]), Path::new(&a[3]))?;
            agent_mova::json_new(Path::new(&a[4]), &r)?;
            println!("{}", serde_json::to_string(&r)?);
        }
        Some("collect") if a.len() == 4 => {
            let index = serde_json::from_slice(&agent_mova::read_bounded(Path::new(&a[2]))?)
                .context("report index JSON")?;
            let r = agent_mova::collect(Path::new(&a[1]), &index, Path::new(&a[3]))?;
            println!("{}", serde_json::to_string(&r)?);
        }
        _ => bail!(
            "agent-mova capabilities | example NEW_DIR | trace TASK NEW_DIR | prepare-synthetic-recall TRACE_REPORT STATE NEW_TASK | recall TASK STATE TRACE_REPORT NEW_REPORT | collect TRUSTED_ROOT INDEX NEW_DIR"
        ),
    }
    Ok(())
}
