// Enforce strict documentation guidelines
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! TODO: vgpu crate documentation goes here.

/// Returns the string slice provided.
///
/// # Arguments
///
/// * `message` - A string slice that is returned
///
/// # Examples
///
/// ```
/// use vgpu::echo;
/// let echo = echo("message");
/// ```
pub fn echo(message: &'static str) -> &'static str {
    message
}

#[cfg(test)]
mod tests {
    use crate::echo;

    #[test]
    fn example_test() {
        assert_eq!(echo("Hello, World!"), "Hello, World!");
    }
}
