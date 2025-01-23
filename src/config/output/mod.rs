use std::collections::HashSet;

use crate::measurement::Results;
use anyhow::{anyhow, Result};
use filters::add_filters_to_engine;
use minijinja::Environment;

#[cfg(test)]
mod tests;

/// Custom Jinja filters for use in templates.
mod filters;

/// Output template name
const OUTPUT_TEMPLATE: &str = "output";

/// Wrapper struct for interaction with the templating engine.
#[derive(Debug)]
pub struct Formatter<'a> {
    /// The actual templating engine.
    engine: Environment<'a>,
}

fn extract_struct_fields(value: &serde_json::Value) -> HashSet<String> {
    match value {
        serde_json::Value::Object(object) => object
            .iter()
            .flat_map(|(key, val)| {
                extract_struct_fields(val)
                    .iter()
                    .map(|nested_key| format!("{key}.{nested_key}"))
                    .chain(std::iter::once(key.clone()))
                    .collect::<HashSet<_>>()
            })
            .collect(),
        _ => HashSet::new(),
    }
}

impl Formatter<'_> {
    /// Validate the output template against an empty result.
    ///
    /// # Errors
    ///
    /// Returns an error if the template fails to validate.
    fn validate_output_template(self) -> Result<Self> {
        let template = self.engine.get_template(OUTPUT_TEMPLATE)?;
        let default_results = &Results::default();

        let template_vars = template.undeclared_variables(true);
        let available_vars = extract_struct_fields(&serde_json::to_value(default_results)?);
        if !template_vars.is_subset(&available_vars) {
            let undefined_vars = template_vars.difference(&available_vars);
            return Err(anyhow!(
                "Template validation failed. Undefined variables used: {undefined_vars:?}.",
            ));
        }

        Ok(template.render(default_results).map(|_| self)?)
    }

    /// Create a template engine populated with the output template.
    ///
    /// # Errors
    ///
    /// Surfaces any error encountered in the internal engine.
    pub fn from_template_string(output_template: String) -> Result<Self> {
        let mut engine = Environment::new();
        engine.add_template_owned(OUTPUT_TEMPLATE.to_string(), output_template)?;
        add_filters_to_engine(&mut engine);

        Self { engine }.validate_output_template()
    }

    /// Use the engine to render the output template using measurement results.
    ///
    /// # Errors
    ///
    /// Surfaces any error encountered in the internal engine.
    pub fn render_results(&self, results: Results) -> Result<String> {
        Ok(self.engine.get_template(OUTPUT_TEMPLATE)?.render(results)?)
    }
}
