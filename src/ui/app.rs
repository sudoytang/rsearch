use eframe::egui;
use crate::ui;
use crate::ui::components::{HexViewer, DataInspector, FilePanel, SearchControlPanel, SearchResultsPanel};






pub struct BinarySearchApp {
    // UI components
    file_panel: FilePanel,
    search_control_panel: SearchControlPanel,
    search_results_panel: SearchResultsPanel,
    hex_viewer: HexViewer,
    data_inspector: DataInspector,
}

impl Default for BinarySearchApp {
    fn default() -> Self {
        Self {
            file_panel: FilePanel::new(),
            search_control_panel: SearchControlPanel::new(),
            search_results_panel: SearchResultsPanel::new(),
            hex_viewer: HexViewer::new(),
            data_inspector: DataInspector::new(),
        }
    }
}

impl BinarySearchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }



    fn perform_search(&mut self) {
        // TODO: Implement search logic
        // - Parse search_input based on search_type
        // - Create Needle from parsed data
        // - Use AsyncSearch to find matches
        // - Update search_results with found offsets
        self.search_results_panel.clear_results();
        
        // Mock search results for UI testing
        if !self.search_control_panel.get_search_input().is_empty() {
            let mut results = Vec::new();
            for i in 0..10 {
                results.push(ui::SearchResult {
                    index: i,
                    offset: i * 16,
                });
            }
            self.search_results_panel.set_search_results(results);
        }
    }






}

impl eframe::App for BinarySearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // File panel
            if self.file_panel.render(ui) {
                // File was opened, clear search results
                self.search_results_panel.clear_results();
            }
            
            ui.separator();
            
            // Search controls panel
            if self.search_control_panel.render(ui) {
                self.perform_search();
            }
            
            // Search results panel
            if let Some(offset) = self.search_results_panel.render(ui) {
                self.hex_viewer.set_selected_offset(Some(offset));
            }
            
            // Bottom section - Hex viewer and data inspector
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.hex_viewer.render(ui, self.file_panel.get_file_data());
                });
                
                ui.vertical(|ui| {
                    self.data_inspector.render(ui, self.hex_viewer.get_selected_offset(), self.file_panel.get_file_data());
                });
            });
        });
        
        
    }
}
