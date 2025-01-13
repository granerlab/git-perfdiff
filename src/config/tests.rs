use std::time::Duration;

use crate::measurement::Results;

use super::Formatter;
use anyhow::Result;

#[test]
fn output_template() -> Result<()> {
    let template = "No output!".to_string();
    let formatter = Formatter::from_template_string(template.clone())?;
    let results = Results {
        wall_time: Duration::from_millis(100),
        avg_cpu: 10.0,
        avg_ram: 1025,
    };
    assert_eq!(formatter.render_results(results)?, template);
    Ok(())
}
