//! Shared library for the r3st binaries.

/// Return a greeting for the given component name.
pub fn greeting(component: &str) -> String {
    format!("Hello from {component}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_includes_component() {
        assert_eq!(greeting("server"), "Hello from server!");
    }
}
