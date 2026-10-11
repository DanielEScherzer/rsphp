use bindgen::callbacks::ParseCallbacks;

/// Callbacks to fix up some comments pulled from the C documentation.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct CommentProcessor;

impl CommentProcessor {
    /// Create a new callbacks instance to process comments.
    #[must_use]
    pub fn new() -> Self {
        CommentProcessor
    }
}

impl ParseCallbacks for CommentProcessor {
    fn process_comment(&self, comment: &str) -> Option<String> {
        // Fix up the zend_ini_parse_quantity comment
        if !comment.contains("Parses an ini quantity") {
            return None;
        }
        let lines = comment.split('\n');
        let mut in_backticks = false;
        let mut result = String::new();
        for line in lines {
            let should_be_code = line.starts_with("   ");
            match (should_be_code, in_backticks) {
                (true, false) => {
                    in_backticks = true;
                    result += "```text\n";
                }
                (false, true) => {
                    in_backticks = false;
                    result += "```\n";
                }
                _ => (),
            }
            result += line;
            result += "\n";
        }
        Some(result)
    }
}
