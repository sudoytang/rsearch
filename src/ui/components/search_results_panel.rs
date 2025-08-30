use eframe::egui;
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

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let mut selected_offset = None;

        // Search results section
        ui.group(|ui| {
            ui.label("Search Results");
            
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    egui::Grid::new("results_grid")
                        .spacing(egui::vec2(8.0, 4.0))
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Index").text_style(egui::TextStyle::Monospace));
                            ui.label(egui::RichText::new("Offset").text_style(egui::TextStyle::Monospace));
                            ui.label(egui::RichText::new("Action").text_style(egui::TextStyle::Monospace));
                            ui.end_row();

                            if self.search_results.is_empty() {
                                ui.label("No results found");
                                ui.end_row();
                            } else {
                                for result in &self.search_results {
                                    ui.label(egui::RichText::new(format!("{}", result.index)).text_style(egui::TextStyle::Monospace));
                                    ui.label(egui::RichText::new(format!("0x{:08X}", result.offset)).text_style(egui::TextStyle::Monospace));
                                    if ui.button("Go to").clicked() {
                                        // TODO: Implement scroll to offset in hex viewer
                                        selected_offset = Some(result.offset);
                                    }
                                    ui.end_row();
                                }
                            }
                        });
                });
        });

    }
}

