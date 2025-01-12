use std::time::Duration;

use crate::measurement::Results;

use super::{render_with_engine, setup_template_engine};
use anyhow::Result;

#[test]
fn output_template() -> Result<()> {
    let template = "No output!".to_string();
    let engine = setup_template_engine(template.clone())?;
    let results = Results {
        wall_time: Duration::from_millis(100),
        avg_cpu: 10.0,
        avg_ram: 1025,
    };
    assert_eq!(render_with_engine(&engine, results)?, template);
    Ok(())
}
