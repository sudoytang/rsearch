use eframe::egui;
use crate::search::Endianness;
use crate::ui::SearchType;

pub struct SearchControlPanel {
    search_type: SearchType,
    search_input: String,
    endianness: Endianness,
    is_signed: bool,
}

impl SearchControlPanel {
    pub fn new() -> Self {
        Self {
            search_type: SearchType::Bit8,
            search_input: String::new(),
            endianness: Endianness::LittleEndian,
            is_signed: false,
        }
    }

    pub fn get_search_type(&self) -> SearchType {
        self.search_type
    }

    pub fn get_search_input(&self) -> &str {
        &self.search_input
    }

    pub fn get_endianness(&self) -> Endianness {
        self.endianness
    }

    pub fn get_is_signed(&self) -> bool {
        self.is_signed
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut search_requested = false;

        // Search controls section
        ui.group(|ui| {
            ui.label("Search Controls");
            
            ui.horizontal(|ui| {
                // Search type dropdown
                egui::ComboBox::from_label("Type")
                    .selected_text(format!("{:?}", self.search_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.search_type, SearchType::Bit8, "8-bit");
                        ui.selectable_value(&mut self.search_type, SearchType::Bit16, "16-bit");
                        ui.selectable_value(&mut self.search_type, SearchType::Bit32, "32-bit");
                        ui.selectable_value(&mut self.search_type, SearchType::Bit64, "64-bit");
                        ui.selectable_value(&mut self.search_type, SearchType::Bytes, "Bytes");
                        ui.selectable_value(&mut self.search_type, SearchType::String, "String");
                    });
                
                // Search input
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.search_input);
                
                // Search button
                if ui.button("Search").clicked() {
                    search_requested = true;
                }
            });
            
            ui.horizontal(|ui| {
                // Endianness radio buttons
                ui.label("Endianness:");
                ui.add_enabled_ui(self.search_type.is_endianness_enabled(), |ui| {
                    ui.radio_value(&mut self.endianness, Endianness::LittleEndian, "Little Endian");
                    ui.radio_value(&mut self.endianness, Endianness::BigEndian, "Big Endian");
                });
                
                ui.separator();
                
                // Signedness radio buttons
                ui.label("Signedness:");
                ui.add_enabled_ui(self.search_type.is_signedness_enabled(), |ui| {
                    ui.radio_value(&mut self.is_signed, false, "Unsigned");
                    ui.radio_value(&mut self.is_signed, true, "Signed");
                });
            });
        });

        search_requested
    }
}

