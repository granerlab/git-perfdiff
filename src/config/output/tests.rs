use super::Formatter;

mod template {

    use crate::measurement::Results;

    use super::Formatter;

    #[test]
    fn trivial() {
        let template = "No output!".to_string();
        let formatter = Formatter::from_template_string(template.clone()).unwrap();
        let rendered_results = formatter.render_results(Results::default()).unwrap();
        assert_eq!(rendered_results, template);
    }

    #[test]
    fn undefined_values() {
        let template = "I'm a {{ bad_value }} with {{ avg_cpu }}".to_string();
        let formatter = Formatter::from_template_string(template);
        let expected_error =
            r#"Template validation failed. Undefined variables used: ["bad_value"]."#.to_string();
        assert!(formatter.is_err());
        let err = formatter.unwrap_err();
        assert_eq!(err.to_string(), expected_error);
    }

    #[test]
    fn nested_values() {
        let template = "Ran in {{ wall_time.secs }} whole seconds".to_string();
        let formatter = Formatter::from_template_string(template).unwrap();
        let rendered_results = formatter.render_results(Results::default()).unwrap();
        let expected_results = "Ran in 0 whole seconds".to_string();
        assert_eq!(rendered_results, expected_results);
    }
}
