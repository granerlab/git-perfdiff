use super::Formatter;

mod template {

    use crate::measurement::Results;

    use super::Formatter;

    fn test_output(template: &str, results: Results, expected: &str) {
        let formatter = Formatter::from_template_string(template.to_string()).unwrap();
        let rendered_results = formatter.render_results(results).unwrap();
        assert_eq!(rendered_results, expected.to_string());
    }

    #[test]
    fn trivial() {
        test_output("No output", Results::default(), "No output");
    }

    #[test]
    fn undefined_values() {
        let template = "I'm a {{ bad_value }} with {{ cpu | min }} CPU usage".to_string();
        let formatter = Formatter::from_template_string(template);
        let expected_error =
            r#"Template validation failed. Undefined variables used: ["bad_value"]."#.to_string();
        assert!(formatter.is_err());
        let err = formatter.unwrap_err();
        assert_eq!(err.to_string(), expected_error);
    }

    #[test]
    fn nested_values() {
        test_output(
            "Ran in {{ wall_time.secs }} whole seconds",
            Results::default(),
            "Ran in 0 whole seconds",
        );
    }

    mod filters {
        use super::{test_output, Results};

        #[test]
        fn jinja_filters() {
            test_output(
                "Min CPU usage: {{ cpu | min }}",
                Results {
                    cpu: vec![30.0, 50.0, 10.0, 40.0],
                    ..Results::default()
                },
                "Min CPU usage: 10.0",
            );
        }

        #[test]
        fn custom_cpu_filters() {
            test_output(
                "Avg CPU usage: {{ cpu | avg }}",
                Results {
                    cpu: vec![30.0, 10.0],
                    ..Results::default()
                },
                "Avg CPU usage: 20.0",
            );
        }

        #[test]
        fn custom_ram_filters() {
            test_output(
                "avg kb: {{ ram | avg | as_kb }}",
                Results {
                    ram: vec![1024.0, 2048.0],
                    ..Results::default()
                },
                "avg kb: 1.5",
            );
        }

        #[test]
        fn custom_time_filters() {
            test_output(
                "millis: {{ wall_time | as_millis }}",
                Results {
                    wall_time: std::time::Duration::from_millis(12345),
                    ..Results::default()
                },
                "millis: 12345",
            );
        }
    }
}
