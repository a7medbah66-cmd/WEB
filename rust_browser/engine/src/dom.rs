//! DOM (Document Object Model) - the tree representation of an HTML document
//!
//! The DOM is the core data structure that represents the structure of an HTML document.
//! It's a tree of nodes, where each node is either:
//! - A text node (contains text content)
//! - An element node (contains tag name and attributes)
//!
//! Example DOM tree for `<div id="main">Hello World</div>`:
//! ```
//! Node {
//!     children: [],
//!     node_type: Element(ElementData {
//!         tag_name: "div",
//!         attributes: {"id": "main"}
//!     })
//! }
//! └── Node {
//!         children: [],
//!         node_type: Text("Hello World")
//!     }
//! ```

use std::collections::HashMap;

/// Represents a node in the DOM tree
/// 
/// Each node can have:
/// - A list of child nodes
/// - A type indicating whether it's text or element content
/// 
/// The DOM is built by the HTML parser by creating Node instances
/// and nesting them hierarchically.
#[derive(Debug, Clone)]
pub struct Node {
    /// Child nodes - empty for leaf nodes
    pub children: Vec<Node>,
    /// The type of this node (Element or Text)
    pub node_type: NodeType,
}

/// Represents the type of a node in the DOM tree
/// 
/// A node is either:
/// - **Text**: Contains plain text content (e.g., "Hello World")
/// - **Element**: An HTML element with a tag name and attributes (e.g., `<div id="main">`)
#[derive(Debug, Clone)]
pub enum NodeType {
    /// Text node: plain text content
    /// Contains the actual text string (e.g., "Hello", "World")
    Text(String),
    /// Element node: HTML element with tag name and attributes
    /// Examples: div, p, span, h1, body, html
    Element(ElementData),
}

/// Represents an HTML element in the DOM
/// 
/// An element has:
/// - A tag name (e.g., "div", "p", "a")
/// - Attributes (e.g., id="main", class="container", href="/")
/// - Child nodes (content inside the tags)
/// 
/// Example: `<a href="/about" class="nav-link">About</a>`
/// ```
/// ElementData {
///     tag_name: "a",
///     attributes: {
///         "href": "/about",
///         "class": "nav-link"
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ElementData {
    /// The HTML tag name (lowercase)
    /// Examples: "div", "p", "span", "h1", "body", "html", etc.
    pub tag_name: String,
    /// HTML attributes as key-value pairs
    /// Key examples: "id", "class", "style", "href", "src", "alt"
    /// Value examples: "main", "container", "color: red", "/about", "image.png"
    pub attributes: HashMap<String, String>,
}

impl Node {
    /// Creates a text node with the given text content
    /// 
    /// # Arguments
    /// * `data` - The text content as a String
    /// 
    /// # Example
    /// ```
    /// let text_node = Node::text("Hello World".to_string());
    /// // Represents a text node containing "Hello World"
    /// ```
    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    /// Creates an element node with the given tag name, attributes, and children
    /// 
    /// # Arguments
    /// * `name` - The HTML tag name (e.g., "div", "p", "a")
    /// * `attrs` - A HashMap of attribute names to values
    /// * `children` - A vector of child nodes
    /// 
    /// # Example
    /// ```
    /// let attrs = vec![("id", "main")].iter().cloned().collect();
    /// let text = Node::text("Content".to_string());
    /// let div = Node::elem("div".to_string(), attrs, vec![text]);
    /// // Represents: <div id="main">Content</div>
    /// ```
    pub fn elem(name: String, attrs: HashMap<String, String>, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}