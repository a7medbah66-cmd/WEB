use std::collections::HashMap;

/// Represents a CSS selector (simple implementation)
/// Supports: element selectors, class selectors, ID selectors
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Element selector: div, p, span, etc.
    Element(String),
    /// Class selector: .container, .button, etc.
    Class(String),
    /// ID selector: #header, #main, etc.
    Id(String),
}

/// Represents a single CSS property and its value
/// Examples: ("color", "red"), ("font-size", "16px")
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

/// Represents a CSS rule: selector(s) with a set of properties
/// Example: div { color: red; background: blue; }
#[derive(Debug, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

/// A basic CSS parser that converts CSS strings into a set of rules
/// 
/// This is a **minimal implementation** for demonstration purposes.
/// A production parser would need:
/// - Full CSS syntax support (combinators, media queries, etc.)
/// - Proper tokenization and error recovery
/// - Support for pseudo-selectors and pseudo-elements
/// - Proper precedence and specificity calculation
pub struct CssParser {
    input: String,
    pos: usize,
}

impl CssParser {
    /// Creates a new CSS parser with the given input string
    pub fn new(input: String) -> Self {
        CssParser { input, pos: 0 }
    }

    /// Parses the CSS input into a vector of rules
    /// 
    /// Currently returns an empty vector (stub implementation).
    /// TODO: Implement actual CSS tokenization and parsing
    pub fn parse_rules(&mut self) -> Vec<Rule> {
        // TODO: Implement the following:
        // 1. Tokenize CSS into tokens (selectors, properties, braces, etc.)
        // 2. Parse selector blocks
        // 3. Parse property declarations
        // 4. Handle whitespace and comments
        // 5. Build Rule structs from parsed tokens
        
        // TEMPORARY: Return empty vec - no CSS parsing implemented yet
        vec![]
    }

    /// Parses a single CSS rule (selector block + properties)
    fn parse_rule(&mut self) -> Option<Rule> {
        // TODO: Implement rule parsing
        // This should handle:
        // - Multiple selectors separated by commas
        // - Properties inside { }
        None
    }

    /// Parses a CSS selector string into a Selector enum
    fn parse_selector(selector_str: &str) -> Option<Selector> {
        let trimmed = selector_str.trim();
        
        if trimmed.starts_with('#') {
            Some(Selector::Id(trimmed[1..].to_string()))
        } else if trimmed.starts_with('.') {
            Some(Selector::Class(trimmed[1..].to_string()))
        } else if !trimmed.is_empty() {
            Some(Selector::Element(trimmed.to_string()))
        } else {
            None
        }
    }

    /// Parses a CSS property (name: value;)
    fn parse_property(&mut self) -> Option<Property> {
        // TODO: Implement property parsing
        // This should handle:
        // - Property name and colon
        // - Property value
        // - Semicolon terminator
        None
    }
}

/// Stores parsed CSS rules and provides lookup by selector
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        StyleSheet { rules: Vec::new() }
    }

    /// Collects properties for an element matching a selector
    /// 
    /// Returns a HashMap of property names to values matching the selector
    pub fn properties_for_selector(&self, selector: &Selector) -> HashMap<String, String> {
        let mut props = HashMap::new();
        
        for rule in &self.rules {
            if rule.selectors.contains(selector) {
                for prop in &rule.properties {
                    props.insert(prop.name.clone(), prop.value.clone());
                }
            }
        }
        
        props
    }
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_parsing() {
        assert_eq!(CssParser::parse_selector("div"), Some(Selector::Element("div".to_string())));
        assert_eq!(CssParser::parse_selector(".container"), Some(Selector::Class("container".to_string())));
        assert_eq!(CssParser::parse_selector("#header"), Some(Selector::Id("header".to_string())));
    }
}
