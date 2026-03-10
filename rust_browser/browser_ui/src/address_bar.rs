use eframe::egui;

/// Represents the state of the address bar
pub struct AddressBar {
    /// Current URL input
    pub url_input: String,
    /// Previously visited URLs for history
    pub history: Vec<String>,
    /// Current position in history (for back/forward navigation)
    pub history_index: usize,
}

impl AddressBar {
    /// Creates a new address bar with a default URL
    pub fn new() -> Self {
        let mut history = Vec::new();
        history.push("about:blank".to_string());

        AddressBar {
            url_input: "about:blank".to_string(),
            history,
            history_index: 0,
        }
    }

    /// Sets the current URL and updates history
    pub fn set_url(&mut self, url: String) {
        // Remove forward history if we're not at the end
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }

        // Add new URL to history if it's different from the current one
        if url != self.url_input {
            self.history.push(url.clone());
            self.history_index = self.history.len() - 1;
        }

        self.url_input = url;
    }

    /// Navigate backwards in history
    pub fn go_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.url_input = self.history[self.history_index].clone();
        }
    }

    /// Navigate forwards in history
    pub fn go_forward(&mut self) {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.url_input = self.history[self.history_index].clone();
        }
    }

    /// Checks if we can go back
    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    /// Checks if we can go forward
    pub fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len() - 1
    }

    /// Returns the current URL
    pub fn current_url(&self) -> &str {
        &self.url_input
    }

    /// Validates if a string is a valid URL
    /// 
    /// This is a **minimal check** for demonstration purposes.
    /// A production validator would verify against RFC 3986.
    pub fn is_valid_url(url: &str) -> bool {
        let trimmed = url.trim();
        
        // Allow special URLs
        if trimmed.starts_with("about:") {
            return true;
        }
        
        // Check for http:// or https://
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return trimmed.len() > 8; // At least some domain
        }

        // Allow file:// URLs
        if trimmed.starts_with("file://") {
            return true;
        }

        false
    }

    /// Normalizes a URL (adds https:// if no protocol is specified)
    /// 
    /// This is a **simple normalization** for demonstration purposes.
    pub fn normalize_url(input: &str) -> String {
        let trimmed = input.trim();

        // If already has a protocol or is a special URL, return as-is
        if trimmed.starts_with("http://")
            || trimmed.starts_with("https://")
            || trimmed.starts_with("file://")
            || trimmed.starts_with("about:")
        {
            return trimmed.to_string();
        }

        // If looks like a domain, add https://
        if trimmed.contains('.') && !trimmed.contains(' ') {
            return format!("https://{}", trimmed);
        }

        // Otherwise, treat as a search query (for now, just return as-is)
        trimmed.to_string()
    }
}

impl Default for AddressBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders the address bar UI using egui
pub fn render_address_bar(
    ui: &mut egui::Ui,
    address_bar: &mut AddressBar,
    loading: bool,
) -> bool {
    let mut should_load = false;

    ui.horizontal(|ui| {
        // Back button
        if ui.button("⬅").clicked() && address_bar.can_go_back() {
            address_bar.go_back();
            should_load = true;
        }

        // Forward button
        if ui.button("➡").clicked() && address_bar.can_go_forward() {
            address_bar.go_forward();
            should_load = true;
        }

        // Reload button
        if ui.button("🔄").clicked() {
            should_load = true;
        }

        // URL input field
        let response = ui.text_edit_singleline(&mut address_bar.url_input);

        // Go button
        if ui.button("Go").clicked() || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let normalized_url = AddressBar::normalize_url(&address_bar.url_input);
            address_bar.set_url(normalized_url);
            should_load = true;
        }

        // Loading indicator
        if loading {
            ui.label("⏳ Loading...");
        }
    });

    should_load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_bar_creation() {
        let bar = AddressBar::new();
        assert_eq!(bar.current_url(), "about:blank");
        assert!(!bar.can_go_back());
        assert!(!bar.can_go_forward());
    }

    #[test]
    fn test_url_validation() {
        assert!(AddressBar::is_valid_url("https://example.com"));
        assert!(AddressBar::is_valid_url("http://example.com"));
        assert!(AddressBar::is_valid_url("about:blank"));
        assert!(!AddressBar::is_valid_url("not a url"));
    }

    #[test]
    fn test_url_normalization() {
        assert_eq!(
            AddressBar::normalize_url("example.com"),
            "https://example.com"
        );
        assert_eq!(
            AddressBar::normalize_url("https://example.com"),
            "https://example.com"
        );
        assert_eq!(
            AddressBar::normalize_url("about:blank"),
            "about:blank"
        );
    }

    #[test]
    fn test_history_navigation() {
        let mut bar = AddressBar::new();
        bar.set_url("https://example.com".to_string());
        bar.set_url("https://example2.com".to_string());

        assert!(bar.can_go_back());
        bar.go_back();
        assert_eq!(bar.current_url(), "https://example.com");

        bar.go_forward();
        assert_eq!(bar.current_url(), "https://example2.com");
    }
}
