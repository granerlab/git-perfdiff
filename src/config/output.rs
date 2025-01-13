use crate::measurement::Results;
use anyhow::Result;
// TODO: Replace with crate that supports eager template validation.
use minijinja::Environment;

/// Wrapper struct for interaction with the templating engine.
pub struct Formatter<'a> {
    /// The actual templating engine.
    engine: Environment<'a>,
}

impl Formatter<'_> {
    /// Create a template engine populated with the output template.
    ///
    /// # Errors
    ///
    /// Surfaces any error encountered in the internal engine.
    pub fn from_template_string(output_template: String) -> Result<Self> {
        let mut engine = Environment::new();
        engine.add_template_owned("output".to_string(), output_template)?;
        Ok(Self { engine })
    }

    /// Use the engine to render the output template using measurement results.
    ///
    /// # Errors
    ///
    /// Surfaces any error encountered in the internal engine.
    pub fn render_results(&self, results: Results) -> Result<String> {
        Ok(self.engine.get_template("output")?.render(results)?)
    }
}
