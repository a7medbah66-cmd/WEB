/// Core DOM (Document Object Model) structures for representing HTML documents
pub mod dom;
/// HTML parser: converts HTML strings into DOM trees
pub mod html_parser;
/// CSS parser: converts CSS strings into style rules
pub mod css_parser;
/// Layout engine: converts DOM + CSS into a layout tree with dimensions
pub mod layout;
/// Networking: fetches HTML/resources from URLs
pub mod network;

// Re-export commonly used types for convenience
pub use dom::{Node, NodeType, ElementData};
pub use html_parser::HtmlParser;
pub use layout::{LayoutBox, Rect};
pub use network::load_url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let mut parser = HtmlParser::new("<html></html>".to_string());
        let node = parser.parse_nodes();
        println!("Parsed DOM: {:?}", node);
    }
}