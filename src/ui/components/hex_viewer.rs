use eframe::egui;
use egui_extras::{Column, TableBuilder};


pub struct HexViewer {
    selected_offset: usize,
}

impl HexViewer {
    pub fn new() -> Self {
        Self { selected_offset: 0 }
    }

    pub fn set_selected_offset(&mut self, offset: usize) {
        self.selected_offset = offset;
    }

    pub fn get_selected_offset(&self) -> usize {
        self.selected_offset
    }

    pub fn render(&mut self, ui: &mut egui::Ui, file_data: &Option<Vec<u8>>) {
        ui.group(|ui| {
            ui.label("Hex Viewer");

            if let Some(data) = file_data {
                let bpl = 16;
                let lines = (data.len() + bpl - 1) / bpl;

                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        let table = TableBuilder::new(ui)
                            .striped(false)
                            .column(Column::exact(80.0)) // Address
                            .columns(Column::exact(14.0), bpl) // 16 columns for bytes
                            .column(Column::remainder()); // ASCII

                        table
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.monospace("Address");
                                });
                                for i in 0..bpl {
                                    header.col(|ui| {
                                        ui.monospace(format!("{:02X}", i));
                                    });
                                }
                                header.col(|ui| {
                                    ui.monospace("ASCII");
                                });
                            })
                            .body(|mut body| {
                                for line in 0..lines {
                                    let start = line * bpl;
                                    let end = (start + bpl).min(data.len());

                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.monospace(format!("{:08X}", start));
                                        });
                                        for i in 0..bpl {
                                            row.col(|ui| {
                                                if start + i < data.len() {
                                                    let off = start + i;
                                                    let text = format!("{:02X}", data[off]);

                                                    let resp = ui.monospace(text);
                                                    
                                                    // Selection highlighting - only highlight if this byte is selected
                                                    if off == self.selected_offset {
                                                        let r = resp.rect.expand2(egui::vec2(0.0, 0.0));
                                                        ui.painter().rect_filled(
                                                            r,
                                                            2.0,
                                                            egui::Color32::from_rgb(100, 150, 255),
                                                        );
                                                        // Redraw text to ensure it's on top
                                                        ui.painter().text(
                                                            r.center(),
                                                            egui::Align2::CENTER_CENTER,
                                                            format!("{:02X}", data[off]),
                                                            egui::TextStyle::Monospace
                                                                .resolve(ui.style()),
                                                            ui.visuals().strong_text_color(),
                                                        );
                                                    }
                                                    
                                                    if resp.clicked() {
                                                        self.selected_offset = off;
                                                    }
                                                } else {
                                                    ui.monospace("  ");
                                                }
                                            });
                                        }
                                        row.col(|ui| {
                                            let ascii: String = data[start..end]
                                                .iter()
                                                .map(|&b| {
                                                    if b.is_ascii_graphic() {
                                                        b as char
                                                    } else {
                                                        '.'
                                                    }
                                                })
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
}
