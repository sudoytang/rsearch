use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;
use crate::search::Endianness;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchType {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
    Bytes,
    String,
}

impl SearchType {
    pub fn is_endianness_enabled(&self) -> bool {
        matches!(self, SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64)
    }

    pub fn is_signedness_enabled(&self) -> bool {
        matches!(self, SearchType::Bit8 | SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub index: usize,
    pub offset: usize,
}

pub struct BinarySearchApp {
    // File handling
    file_path: Option<PathBuf>,
    file_data: Option<Vec<u8>>,
    
    // Search controls
    search_type: SearchType,
    search_input: String,
    endianness: Endianness,
    is_signed: bool,
    
    // Search results
    search_results: Vec<SearchResult>,
    
    // Hex viewer
    selected_offset: Option<usize>,
    
    // UI state
    show_file_dialog: bool,
}

impl Default for BinarySearchApp {
    fn default() -> Self {
        Self {
            file_path: None,
            file_data: None,
            search_type: SearchType::Bit8,
            search_input: String::new(),
            endianness: Endianness::LittleEndian,
            is_signed: false,
            search_results: Vec::new(),
            selected_offset: None,
            show_file_dialog: false,
        }
    }
}

impl BinarySearchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn open_file(&mut self) {
        // TODO: Implement file opening logic
        // - Show native file dialog
        // - Read file into memory
        // - Update file_path and file_data
        // - Clear previous search results
        
        // WARN(cursor): Mocking file dialog for UI testing
        // In a real implementation, use rfd::FileDialog::new().pick_file()
        self.show_file_dialog = true;
    }

    fn perform_search(&mut self) {
        // TODO: Implement search logic
        // - Parse search_input based on search_type
        // - Create Needle from parsed data
        // - Use AsyncSearch to find matches
        // - Update search_results with found offsets
        self.search_results.clear();
        
        // Mock search results for UI testing
        if !self.search_input.is_empty() {
            for i in 0..10 {
                self.search_results.push(SearchResult {
                    index: i,
                    offset: i * 16,
                });
            }
        }
    }



    fn render_hex_viewer(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Hex Viewer");

            if let Some(data) = &self.file_data {
                let bpl = 16;
                let lines = (data.len() + bpl - 1) / bpl;

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let table = TableBuilder::new(ui)
                            .striped(false)
                            .column(Column::exact(80.0))              // Address
                            .columns(Column::exact(24.0), bpl)        // 16 columns for bytes
                            .column(Column::remainder());             // ASCII

                        table.header(20.0, |mut header| {
                            header.col(|ui| { ui.monospace("Address"); });
                            for i in 0..bpl {
                                header.col(|ui| { ui.monospace(format!("{:02X}", i)); });
                            }
                            header.col(|ui| { ui.monospace("ASCII"); });
                        })
                        .body(|mut body| {
                            for line in 0..lines {
                                let start = line * bpl;
                                let end = (start + bpl).min(data.len());

                                body.row(18.0, |mut row| {
                                    row.col(|ui| { ui.monospace(format!("{:08X}", start)); });
                                    for i in 0..bpl {
                                        row.col(|ui| {
                                            if start + i < data.len() {
                                                let off = start + i;
                                                let text = format!("{:02X}", data[off]);

                                                // Selection highlighting
                                                let resp = ui.monospace(text);
                                                if self.selected_offset == Some(off) {
                                                    let r = resp.rect.expand2(egui::vec2(0.0, 0.0));
                                                    ui.painter().rect_filled(r, 2.0, egui::Color32::from_rgb(100, 150, 255));
                                                    // Redraw text to ensure it's on top
                                                    ui.painter().text(
                                                        r.center(),
                                                        egui::Align2::CENTER_CENTER,
                                                        format!("{:02X}", data[off]),
                                                        egui::TextStyle::Monospace.resolve(ui.style()),
                                                        ui.visuals().strong_text_color()
                                                    );
                                                }
                                                if resp.clicked() {
                                                    self.selected_offset = Some(off);
                                                }
                                            } else {
                                                ui.monospace("  ");
                                            }
                                        });
                                    }
                                    row.col(|ui| {
                                        let ascii: String = data[start..end]
                                            .iter()
                                            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                                            .collect();
                                        ui.monospace(ascii);
                                    });
                                });
                            }
                        });
                    });
            } else {
                ui.label("No file loaded");
            }
        });
    }

    fn render_data_inspector(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Data Inspector");
            
            if let Some(offset) = self.selected_offset {
                if let Some(data) = &self.file_data {
                    if offset < data.len() {
                        ui.label(format!("Offset: 0x{:08X}", offset));
                        
                        // TODO: Implement data interpretation logic
                        // - Read bytes at offset based on selected type
                        // - Display interpreted values (u8, i8, u16, i16, u32, i32, u64, i64, string)
                        // - Handle endianness and signedness
                        
                        let byte = data[offset];
                        ui.label(format!("U8: {}", byte));
                        ui.label(format!("I8: {}", byte as i8));
                        
                        if offset + 1 < data.len() {
                            let bytes = &data[offset..offset + 2];
                            ui.label(format!("U16 (LE): {}", u16::from_le_bytes([bytes[0], bytes[1]])));
                            ui.label(format!("U16 (BE): {}", u16::from_be_bytes([bytes[0], bytes[1]])));
                        }
                        
                        if offset + 3 < data.len() {
                            let bytes = &data[offset..offset + 4];
                            ui.label(format!("U32 (LE): {}", u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])));
                            ui.label(format!("U32 (BE): {}", u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])));
                        }
                        
                        if offset + 7 < data.len() {
                            let bytes = &data[offset..offset + 8];
                            ui.label(format!("U64 (LE): {}", u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])));
                            ui.label(format!("U64 (BE): {}", u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])));
                        }
                    }
                }
            } else {
                ui.label("No data selected");
            }
        });
    }
}

impl eframe::App for BinarySearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top section - File controls
            ui.horizontal(|ui| {
                if ui.button("Open File").clicked() {
                    self.open_file();
                }
                
                if let Some(path) = &self.file_path {
                    ui.label(format!("File: {}", path.display()));
                } else {
                    ui.label("No file loaded");
                }
            });
            
            ui.separator();
            
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
                        self.perform_search();
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
                                            self.selected_offset = Some(result.offset);
                                        }
                                        ui.end_row();
                                    }
                                }
                            });
                    });
            });
            
            // Bottom section - Hex viewer and data inspector
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.render_hex_viewer(ui);
                });
                
                ui.vertical(|ui| {
                    self.render_data_inspector(ui);
                });
            });
        });
        
        // Handle file dialog
        if self.show_file_dialog {
            // TODO: Implement native file dialog
            // - Use rfd or similar crate for native file dialog
            // - Handle file selection and loading
            
            // WARN(cursor): Mocking file loading for UI testing
            // In a real implementation, load actual file data
            self.file_path = Some(PathBuf::from("mock_file.bin"));
            self.file_data = Some(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00, 0x12, 0x34, 0x56, 0x78]);
            self.search_results.clear();
            self.show_file_dialog = false;
        }
    }
}
