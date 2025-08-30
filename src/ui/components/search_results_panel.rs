use eframe::egui;
use egui_extras::{TableBuilder, Column};
use crate::ui::util::SearchResult;

pub struct SearchResultsPanel {
    search_results: Vec<SearchResult>,
}

impl SearchResultsPanel {
    pub fn new() -> Self {
        Self {
            search_results: Vec::new(),
        }
    }

    pub fn get_search_results(&self) -> &Vec<SearchResult> {
        &self.search_results
    }

    pub fn set_search_results(&mut self, results: Vec<SearchResult>) {
        self.search_results = results;
    }

    pub fn clear_results(&mut self) {
        self.search_results.clear();
    }

    pub fn add_search_results(&mut self, mut results: Vec<SearchResult>) {
        // Update indices to be continuous
        let start_index = self.search_results.len();
        println!("Adding {} to search results", results.len());
        for (i, result) in results.iter_mut().enumerate() {
            result.index = start_index + i;
        }
        
        self.search_results.extend(results);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let mut selected_offset = None;

        // Search results section using TableBuilder
        ui.group(|ui| ui.vertical(|ui| {
            ui.label("Search Results");
            
            // Use TableBuilder which handles scrolling automatically
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(100.)) // Index column
                .column(Column::remainder()) // Offset column  
                .column(Column::exact(100.))            // Action column
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Index");
                    });
                    header.col(|ui| {
                        ui.strong("Offset");
                    });
                    header.col(|ui| {
                        ui.strong("Action");
                    });
                })
                .body(|body| {
                    body.rows(18.0, self.search_results.len(), |mut row| {
                        let row_index = row.index();
                        let result = &self.search_results[row_index];
                        
                        row.col(|ui| {
                            ui.label(egui::RichText::new(format!("{}", result.index))
                                .text_style(egui::TextStyle::Monospace));
                        });
                        row.col(|ui| {
                            ui.label(egui::RichText::new(format!("0x{:08X}", result.offset))
                                .text_style(egui::TextStyle::Monospace));
                        });
                        row.col(|ui| {
                            if ui.button("Go").clicked() {
                                // TODO: Implement scroll to offset in hex viewer
                                selected_offset = Some(result.offset);
                            }
                        });
                    });
                });
        }));

    }
}

