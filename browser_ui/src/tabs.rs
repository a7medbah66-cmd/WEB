use engine::LayoutBox;

/// Represents a single browser tab with its state
#[derive(Clone)]
pub struct Tab {
    /// The URL currently loaded in this tab
    pub url: String,
    /// The rendered layout tree for this tab
    pub layout_tree: Option<LayoutBox>,
    /// Current loading status
    pub status: String,
    /// Unique identifier for this tab
    pub id: usize,
}

impl Tab {
    /// Creates a new tab with the given URL and ID
    pub fn new(url: String, id: usize) -> Self {
        Tab {
            url,
            layout_tree: None,
            status: "Not loaded".to_string(),
            id,
        }
    }

    /// Updates the loading status for this tab
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    /// Updates the layout tree for this tab
    pub fn set_layout_tree(&mut self, layout_tree: LayoutBox) {
        self.layout_tree = Some(layout_tree);
        self.status = format!("Loaded: {}", self.url);
    }
}

/// Manages a collection of browser tabs
/// 
/// This is a **minimal implementation** that handles:
/// - Creating new tabs
/// - Switching between tabs
/// - Closing tabs
/// 
/// A production tab system would need:
/// - History (back/forward) per tab
/// - Favicon support
/// - Proper cleanup when closing tabs
/// - Session restoration
/// - Tab groups/organization
pub struct TabManager {
    /// All open tabs
    tabs: Vec<Tab>,
    /// Index of the currently active tab
    active_tab_index: usize,
    /// Counter for generating unique tab IDs
    next_tab_id: usize,
}

impl TabManager {
    /// Creates a new tab manager with an initial home tab
    pub fn new() -> Self {
        let mut manager = TabManager {
            tabs: Vec::new(),
            active_tab_index: 0,
            next_tab_id: 1,
        };
        
        // Create initial home tab
        manager.new_tab("about:blank".to_string());
        manager
    }

    /// Creates a new tab and switches to it
    pub fn new_tab(&mut self, url: String) -> usize {
        let tab = Tab::new(url, self.next_tab_id);
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;
        
        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
        tab_id
    }

    /// Switches to the tab with the given index
    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
        }
    }

    /// Closes the tab at the given index
    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() && self.tabs.len() > 1 {
            self.tabs.remove(index);
            
            // Adjust active tab index if needed
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
        }
    }

    /// Returns the currently active tab
    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    /// Returns a mutable reference to the currently active tab
    pub fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }

    /// Returns a list of all tabs
    pub fn get_all_tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Returns the index of the active tab
    pub fn get_active_tab_index(&self) -> usize {
        self.active_tab_index
    }

    /// Returns the total number of open tabs
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tab_manager() {
        let manager = TabManager::new();
        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.get_active_tab_index(), 0);
    }

    #[test]
    fn test_create_new_tab() {
        let mut manager = TabManager::new();
        manager.new_tab("https://example.com".to_string());
        assert_eq!(manager.tab_count(), 2);
        assert_eq!(manager.get_active_tab_index(), 1);
    }

    #[test]
    fn test_switch_tab() {
        let mut manager = TabManager::new();
        manager.new_tab("https://example.com".to_string());
        manager.switch_to_tab(0);
        assert_eq!(manager.get_active_tab_index(), 0);
    }
}
