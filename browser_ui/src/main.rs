mod address_bar;
mod renderer;
mod tabs;

use eframe::egui;
use engine::HtmlParser;
use address_bar::AddressBar;
use tabs::TabManager;
use engine::LayoutBox;
use engine::Rect;

/// The main browser application
/// 
/// This struct manages the overall state of the browser including:
/// - Current tab and multiple tabs
/// - Address bar with history
/// - Loading state
/// - Layout and rendering
struct BrowserApp {
    /// Tab manager for handling multiple browser tabs
    tab_manager: TabManager,
    /// Address bar for URL input and navigation
    address_bar: AddressBar,
    /// Flag indicating if a page is currently loading
    is_loading: bool,
}

impl Default for BrowserApp {
    fn default() -> Self {
        let mut address_bar = AddressBar::new();
        address_bar.set_url("about:blank".to_string());

        BrowserApp {
            tab_manager: TabManager::new(),
            address_bar,
            is_loading: false,
        }
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Top Panel: Address Bar ---
        egui::TopBottomPanel::top("address_bar").show(ctx, |ui| {
            let should_load = address_bar::render_address_bar(ui, &mut self.address_bar, self.is_loading);

            if should_load {
                self.load_page();
            }

            // --- Tab Bar (inline with address bar if multiple tabs) ---
            if self.tab_manager.tab_count() > 1 {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Tabs:");
                    
                    // Collect tab info first to avoid borrow conflicts
                    let tab_count = self.tab_manager.tab_count();
                    let active_index = self.tab_manager.get_active_tab_index();
                    let mut tab_info: Vec<(usize, String)> = Vec::new();
                    
                    for index in 0..tab_count {
                        if let Some(tab) = self.tab_manager.get_all_tabs().get(index) {
                            tab_info.push((index, tab.url.clone()));
                        }
                    }
                    
                    // Now iterate over collected info to avoid borrow checker issues
                    let mut new_active_tab = None;
                    for (index, url) in tab_info {
                        let button_text = if index == active_index {
                            format!("📄 {} ✕", url)
                        } else {
                            format!("📄 {}", url)
                        };

                        if ui.button(&button_text).clicked() {
                            new_active_tab = Some(index);
                        }
                    }
                    
                    // Apply tab switch after the loop
                    if let Some(index) = new_active_tab {
                        self.tab_manager.switch_to_tab(index);
                    }

                    if ui.button("+").clicked() {
                        self.tab_manager.new_tab("about:blank".to_string());
                        self.address_bar.set_url("about:blank".to_string());
                    }
                });
            }
        });

        // --- Main Content Area: Rendered Page ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab) = self.tab_manager.get_active_tab() {
                if let Some(layout) = &tab.layout_tree {
                    let painter = ui.painter();
                    renderer::Renderer::render_layout_tree(&painter, layout);
                } else {
                    ui.label("Loading...");
                }
            }
        });
    }
}

impl BrowserApp {
    /// Loads the page from the current URL in the address bar
    fn load_page(&mut self) {
        self.is_loading = true;

        // For now, use a hardcoded dummy DOM instead of making actual HTTP requests
        // This prevents blocking the UI thread
        // TODO: Implement async networking using tokio or similar
        let html = r#"
            <html>
                <body>
                    <div>
                        <h1>Welcome to RustyBrowser!</h1>
                        <p>This is a minimal experimental web browser built from scratch in Rust.</p>
                        <p>Currently, the parser returns a hardcoded DOM tree.</p>
                    </div>
                </body>
            </html>
        "#.to_string();

        // Parse HTML into DOM
        let mut parser = HtmlParser::new(html);
        let dom_tree = parser.parse_nodes();

        // Layout the DOM tree
        let viewport = Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        };
        let layout_tree = LayoutBox::layout_tree(&dom_tree, viewport);

        // Update current tab with the new layout
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.set_layout_tree(layout_tree);
        }

        self.is_loading = false;
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RustyBrowser - A Minimal Experimental Web Browser",
        options,
        Box::new(|_cc| Box::<BrowserApp>::default()),
    )
}
