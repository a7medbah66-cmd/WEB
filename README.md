# RustyBrowser

A minimal experimental web browser and browser engine written entirely from scratch in Rust, without relying on Chromium, WebKit, or Gecko.

## Project Overview

RustyBrowser is an educational project designed to demonstrate the basic components of a web browser and rendering engine. It includes:

- **HTML Parser**: Converts HTML strings into a DOM tree
- **CSS Parser**: Basic CSS selector and style parsing
- **Layout Engine**: Implements block layout algorithm
- **Renderer**: Draws layout boxes to the screen
- **Network Layer**: Fetches HTML pages over HTTP/HTTPS
- **Browser UI**: Tab system, address bar, navigation controls

This is **NOT** a production-ready browser. It's designed to be understandable and extensible by developers who want to learn how browsers work.

## Architecture

```
rust_browser/
├── engine/              # Core browser engine
│   └── src/
│       ├── lib.rs       # Module declarations and re-exports
│       ├── dom.rs       # DOM tree structures
│       ├── html_parser.rs    # HTML tokenizer and parser
│       ├── css_parser.rs     # CSS parser
│       ├── layout.rs         # Layout engine
│       ├── renderer.rs       # Rendering logic (stub)
│       └── network.rs        # HTTP networking
├── browser_ui/          # Browser UI and main application
│   └── src/
│       ├── main.rs      # Main application entry point
│       ├── address_bar.rs    # Address bar with history
│       ├── tabs.rs      # Tab management
│       ├── renderer.rs  # egui rendering integration
│       └── window.rs    # Window management (stub)
└── engine_server/      # Headless web server for Railway deployment
    └── src/
        └── main.rs      # Axum web server exposing engine API
```

## Building and Running

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- **Linux/macOS/Windows** with graphics support (wayland, X11, or native windowing)

### Build Instructions

1. **Clone the repository**:
```bash
cd /workspaces/WEB/rust_browser
```

2. **Build the project**:
```bash
cargo build --release
```

3. **Run the browser**:
```bash
cargo run --release -p browser_ui
```

4. **Run the headless server** (for Railway deployment):
```bash
cargo run --release -p engine_server
```

The server will start on port 3000 (or PORT environment variable) and provide a JSON API endpoint.

### Server API

The `engine_server` package provides a REST API that demonstrates your browser engine:

**GET /** - Engine status and demo parsing
```json
{
  "message": "Your custom Rust Engine is live!",
  "engine_status": "DOM successfully initialized"
}
```

This endpoint uses your HTML parser to process a sample HTML string and returns the status.

### Railway Deployment

The project includes a `engine_server` package designed for deployment on Railway:

1. **Build the server**:
```bash
cargo build --release -p engine_server
```

2. **Deploy to Railway**:
   - Connect your GitHub repository to Railway
   - Set the build command: `cargo build --release -p engine_server`
   - Set the start command: `./target/release/engine_server`
   - Railway will automatically detect the PORT environment variable

3. **Docker Deployment** (alternative):
```bash
# Build the Docker image
docker build -t rustybrowser-server .

# Run locally
docker run -p 3000:3000 rustybrowser-server
```

The server is stateless and ready for cloud deployment!

### Development Build (with debug info)

```bash
cargo build
cargo run -p browser_ui
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p engine
cargo test -p browser_ui

# Run tests with output
cargo test -- --nocapture
```

## Features

### Currently Implemented ✅

- [x] Basic DOM tree structure
- [x] HTML parser (dummy implementation returning hardcoded DOM)
- [x] CSS parser skeleton with selector types
- [x] Layout engine (returns dummy layout boxes)
- [x] Basic renderer (draws boxes with egui)
- [x] Address bar with history navigation
- [x] Tab management system
- [x] HTTP network loader (reqwest)
- [x] Comprehensive code comments

### In Progress 🚧

- [ ] Actual HTML tokenization and parsing
- [ ] CSS property parsing and specificity calculation
- [ ] Block layout algorithm implementation
- [ ] Text rendering with proper font metrics
- [ ] Image loading and rendering

### Future Features 🗺️

- [ ] Inline layout algorithm
- [ ] CSS cascade and inheritance
- [ ] Event handling (click, keyboard, etc.)
- [ ] Form elements and input handling
- [ ] JavaScript engine (v8/SpiderMonkey integration)
- [ ] Session storage and cookies
- [ ] Responsive design (media queries)
- [ ] Full CSS support (flexbox, grid, etc.)
- [ ] Video/audio playback
- [ ] Web API compatibility
- [ ] Security sandboxing

## Key Components Explained

### 1. DOM (Document Object Model)

The DOM represents the structure of an HTML document as a tree of nodes.

**File**: `engine/src/dom.rs`

```rust
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}
```

### 2. HTML Parser

Converts HTML strings into DOM trees.

**File**: `engine/src/html_parser.rs`

Currently returns a hardcoded DOM for testing. To implement:
1. Tokenize input into HTML tokens
2. Maintain a stack of open elements
3. Emit nodes when closing tags
4. Handle attributes and text nodes

### 3. CSS Parser

Parses CSS selectors and properties.

**File**: `engine/src/css_parser.rs`

Supports three selector types:
- Element selectors: `div`, `p`, `body`
- Class selectors: `.container`, `.button`
- ID selectors: `#header`, `#main`

### 4. Layout Engine

Computes element dimensions and positions using CSS Box Model.

**File**: `engine/src/layout.rs`

The layout tree contains:
- Dimensions (x, y, width, height) for each box
- Box type (block, inline, etc.)
- Children for nested layouts

### 5. Renderer

Draws the layout tree to the screen using egui.

**File**: `browser_ui/src/renderer.rs`

Currently draws:
- Rectangle outlines for each box (color-coded by depth)
- Box type labels for debugging
- Recursive rendering of children

### 6. Address Bar

Manages URL input and navigation history.

**File**: `browser_ui/src/address_bar.rs`

Features:
- Back/forward navigation
- History tracking
- URL normalization (adds https:// if needed)
- URL validation

### 7. Tab Manager

Manages multiple browser tabs.

**File**: `browser_ui/src/tabs.rs`

Features:
- Create new tabs
- Switch between tabs
- Close tabs
- Per-tab layout state

## Development Roadmap

### Phase 1: HTML Parsing ✅ In Progress
- [ ] Implement HTML tokenizer (state machine)
- [ ] Parse tags and attributes
- [ ] Build complete DOM tree
- [ ] Handle nested elements
- [ ] Support common HTML tags (h1-h6, img, a, span, etc.)

### Phase 2: CSS Support
- [ ] Full CSS tokenizer
- [ ] Parse property values (colors, sizes, etc.)
- [ ] Implement selector matching
- [ ] Apply styles to DOM nodes
- [ ] Handle CSS cascade and specificity

### Phase 3: Layout Engine
- [ ] Implement block layout algorithm
- [ ] Calculate element dimensions
- [ ] Handle margins, padding, borders
- [ ] Implement text layout
- [ ] Handle inline elements

### Phase 4: Rendering
- [ ] Render styled boxes
- [ ] Text rendering with fonts
- [ ] Image rendering
- [ ] Background colors and gradients
- [ ] Border rendering

### Phase 5: JavaScript (Optional)
- [ ] Integrate JavaScript engine
- [ ] Handle async operations
- [ ] Document object model (DOM) API
- [ ] Event handling

## Code Guidelines

All code is heavily commented to aid learning. When adding new features:

1. **Add doc comments** to all public structures and functions
2. **Explain the purpose** of complex algorithms
3. **Include TODO comments** for unimplemented features
4. **Add examples** to documentation where helpful
5. **Write tests** for new functionality

Example:

```rust
/// Parses HTML input into a DOM tree
/// 
/// This is a minimal implementation. Production parsers need:
/// - Full HTML5 spec compliance
/// - Error recovery
/// - Malformed HTML handling
pub fn parse_html(input: &str) -> Node {
    // TODO: Implement actual parsing
    Node::default()
}
```

## Testing

The project includes unit tests for core components:

```bash
# Test the parser
cargo test -p engine -- test_parsing

# Test the CSS parser
cargo test -p engine -- css_parser

# Test the address bar
cargo test -p browser_ui -- address_bar
```

## Common Issues and Solutions

### Build fails with OpenGL errors

Some systems require additional development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora
sudo dnf install openssl-devel

# macOS
brew install pkg-config openssl
```

### Cannot find native library

Ensure you have all required dependencies:

```bash
cargo build --verbose
```

The verbose flag will show which libraries are being linked.

## Contributing

To extend this project:

1. **Pick a component** from the roadmap
2. **Read the TODO comments** in the relevant file
3. **Implement the feature** with heavy comments
4. **Add tests** to verify functionality
5. **Run `cargo test`** to ensure nothing breaks

## Learning Resources

- [Web Technologies (MDN)](https://developer.mozilla.org/en-US/docs/Web/HTML)
- [Rust Book](https://doc.rust-lang.org/book/)
- [How Browsers Work](https://www.html5rocks.com/en/tutorials/internals/howbrowserswork/)
- [Crafting Interpreters](https://craftinginterpreters.com/)

## License

This project is open source and available under the MIT License.

## Acknowledgments

This project is inspired by:
- [The Servo Browser Engine](https://servo.org/)
- [Let's Build a Browser Engine](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)
- [Robinson](https://github.com/mbrubeck/robinson)

## Next Steps

1. **Read the code**: Start with `engine/src/dom.rs` to understand the data structures
2. **Implement HTML parsing**: Extend `html_parser.rs` with actual tokenization
3. **Build up gradually**: Add CSS support, then layout, then rendering
4. **Test frequently**: Write tests as you implement features
5. **Learn and experiment**: Try different approaches and see what works

Happy hacking! 🚀
