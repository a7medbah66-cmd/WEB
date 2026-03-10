use eframe::egui;
use engine::{LayoutBox, Rect};

/// Renders a layout tree to the egui painter
/// 
/// This is a **minimal implementation** that draws:
/// - Rectangle outlines for each layout box
/// - Text content for text nodes
/// 
/// A production renderer would need:
/// - Font rendering with proper metrics
/// - Background color and border support
/// - Image rendering
/// - Scrolling and overflow handling
/// - Text wrapping and alignment
/// - CSS styling application
pub struct Renderer;

impl Renderer {
    /// Renders a layout tree using the provided egui painter
    pub fn render_layout_tree(painter: &egui::Painter, layout: &LayoutBox) {
        Self::render_box(painter, layout, 0);
    }

    /// Recursively renders a layout box and its children
    /// 
    /// # Arguments
    /// * `painter` - The egui painter for drawing
    /// * `layout` - The layout box to render
    /// * `depth` - Current depth in the tree (for debugging)
    fn render_box(painter: &egui::Painter, layout: &LayoutBox, depth: usize) {
        let rect = egui::Rect::from_min_size(
            egui::pos2(layout.dimensions.x, layout.dimensions.y),
            egui::vec2(layout.dimensions.width, layout.dimensions.height),
        );

        // Draw the box outline
        // Color depends on depth for visual debugging
        let color = match depth {
            0 => egui::Color32::DARK_BLUE,      // Root
            1 => egui::Color32::BLUE,           // Level 1
            2 => egui::Color32::LIGHT_BLUE,    // Level 2
            _ => egui::Color32::GRAY,          // Deeper levels
        };

        painter.rect_stroke(rect, 0.0, (1.0, color));

        // Draw the box type label (for debugging)
        painter.text(
            rect.min + egui::vec2(2.0, 2.0),
            egui::Align2::LEFT_TOP,
            &layout.box_type,
            egui::FontId::monospace(10.0),
            egui::Color32::DARK_GRAY,
        );

        // Recursively render children
        for child in &layout.children {
            Self::render_box(painter, child, depth + 1);
        }
    }

    /// Renders sample content to demonstrate the renderer
    /// 
    /// This shows a simple example of rendering before full layout tree support
    pub fn render_sample_content(painter: &egui::Painter) {
        // Draw header
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 40.0)),
            0.0,
            egui::Color32::from_rgb(220, 220, 220),
        );

        painter.text(
            egui::pos2(10.0, 10.0),
            egui::Align2::LEFT_CENTER,
            "RustyBrowser",
            egui::FontId::proportional(20.0),
            egui::Color32::BLACK,
        );

        // Draw sample content area
        painter.rect_stroke(
            egui::Rect::from_min_size(egui::pos2(10.0, 50.0), egui::vec2(780.0, 500.0)),
            0.0,
            (2.0, egui::Color32::BLUE),
        );

        painter.text(
            egui::pos2(20.0, 60.0),
            egui::Align2::LEFT_TOP,
            "Your rendered content will appear here",
            egui::FontId::proportional(14.0),
            egui::Color32::BLACK,
        );
    }
}
