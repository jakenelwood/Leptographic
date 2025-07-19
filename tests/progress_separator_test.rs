use leptos_radix_ui::{Progress, ProgressIndicator, Separator};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_component_exists() {
        // Test that Progress component types exist and can be referenced
        // This is a compilation test - if it compiles, the components are properly exported
        let _progress_type = std::any::type_name::<Progress>();
        let _indicator_type = std::any::type_name::<ProgressIndicator>();
        assert!(true);
    }

    #[test]
    fn test_separator_component_exists() {
        // Test that Separator component type exists and can be referenced
        // This is a compilation test - if it compiles, the component is properly exported
        let _separator_type = std::any::type_name::<Separator>();
        assert!(true);
    }

    #[test]
    fn test_component_compilation() {
        // Test that the components can be compiled without runtime issues
        // This ensures the component definitions are syntactically correct
        assert!(true);
    }
}
