use eframe::egui::{self, Response};
use egui_extras::{Column, TableBuilder};

#[derive(Clone, Debug)]
pub struct Selection {
    start: usize,
    end: usize,
    // Both end inclusive, end may be SMALLER than start.
    // (this implies that this type cannot express a null set)
}

impl Selection {
    pub fn new(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }
    
    pub fn lower(&self) -> usize {
        return usize::min(self.start, self.end);
    }

    pub fn upper(&self) -> usize {
        return usize::max(self.start, self.end);
    }

    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.lower() && offset <= self.upper()
    }
    
    pub fn update_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn update_start(&mut self, start: usize) {
        self.start = start;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragStatus {
    Idle,
    Bytes(usize),
    ASCII(usize),
}

impl DragStatus {
    fn type_matches(&self, other: Self) -> bool {
        match (self, other) {
            (DragStatus::Idle, DragStatus::Idle) => true,
            (DragStatus::Bytes(_), DragStatus::Bytes(_)) => true,
            (DragStatus::ASCII(_), DragStatus::ASCII(_)) => true,
            _ => false,
        }
    }
}

pub struct HexViewer {
    selection: Option<Selection>,
    drag_status: DragStatus,
    drag_counter: usize,
}

impl HexViewer {
    const BPL: usize = 16;
    const BYTE_COL_WIDTH: f32 = 14.;
    const ADDRESS_COL_MIN_WIDTH: f32 = 70.;
    const DEFAULT_SPACING: f32 = 8.;
    const BYTE_COLS_MIN_WIDTH: f32 = (Self::BYTE_COL_WIDTH + Self::DEFAULT_SPACING) * Self::BPL as f32;
    const ASCII_COL_MIN_WIDTH: f32 = 120.;
    pub const WIDGET_MIN_WIDTH: f32 = 
        Self::DEFAULT_SPACING           // Margin
      + Self::DEFAULT_SPACING           // Padding
      + Self::ADDRESS_COL_MIN_WIDTH
      + Self::DEFAULT_SPACING
      + Self::BYTE_COLS_MIN_WIDTH
      + Self::DEFAULT_SPACING
      + Self::ASCII_COL_MIN_WIDTH
      + Self::DEFAULT_SPACING
      + Self::DEFAULT_SPACING;
}

impl HexViewer {
    pub fn new() -> Self {
        Self { 
            selection: None,
            drag_status: DragStatus::Idle,
            drag_counter: 0,
        }
    }

    pub fn set_selected_offset(&mut self, offset: usize) {
        self.selection = Some(Selection::new(offset));
    }

    pub fn get_selected_offset(&self) -> Option<usize> {
        self.selection.as_ref().map(|s: &Selection| s.lower())
    }
    
    pub fn get_selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }
    
    pub fn clear_selection(&mut self) {
        self.selection = None;
        self.drag_status = DragStatus::Idle;
    }

    fn handle_drag(&mut self, resp: &Response, status: DragStatus) {
        // Handle mouse interactions
        let off = match status {
            DragStatus::Idle => { return; },
            DragStatus::ASCII(offset) => offset,
            DragStatus::Bytes(offset) => offset,
        };
        if resp.clicked() {
            println!("Clicked");
            self.selection = Some(Selection::new(off));
            self.drag_status = DragStatus::Idle;
        }
        
        // Handle drag start
        if resp.drag_started() {
            println!("Drag Started {:?}", status);
            self.drag_status = status;
            self.selection = Some(Selection::new(off));
            self.drag_counter = 0;
        }
        
        // Handle drag
        if self.drag_status.type_matches(status) && resp.contains_pointer() {
            println!("Dragged {:?} to {:?} {}", self.drag_status, status, self.drag_counter);
            if let Some(ref mut sel) = self.selection {
                sel.update_end(off);
            }
            self.drag_counter += 1;
        }
        
        // Handle drag released - check if we were dragging and now stopped
        if self.drag_status == status && !resp.dragged() {
            println!("Drag Released {:?}", status);
            self.drag_status = DragStatus::Idle;
            self.drag_counter = 0;
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, file_data: Option<&[u8]>) {
        ui.group(|ui| {
            ui.label("Hex Viewer");
            let data = file_data.unwrap_or(&[]);
            let lines = (data.len() + Self::BPL - 1) / Self::BPL;

            
            egui::ScrollArea::vertical()
            .show(ui, |ui| {
                let available_width = ui.available_width();
                let bytes_width = Self::BPL as f32 * (Self::BYTE_COL_WIDTH + ui.spacing().item_spacing.x);
                let remain_width = available_width - bytes_width;
                let address_width = remain_width * Self::ADDRESS_COL_MIN_WIDTH / (Self::ADDRESS_COL_MIN_WIDTH + Self::ASCII_COL_MIN_WIDTH);
                let table = TableBuilder::new(ui)
                    .striped(false)
                    .column(Column::exact(address_width)) // Address
                    .columns(Column::exact(Self::BYTE_COL_WIDTH), Self::BPL) // 16 columns for bytes
                    .column(Column::remainder().at_least(Self::ASCII_COL_MIN_WIDTH)); // ASCII

                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.monospace("Address");
                        });
                        for i in 0..Self::BPL {
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
                            let start = line * Self::BPL;
                            let end = (start + Self::BPL).min(data.len());

                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.monospace(format!("{:08X}", start));
                                });
                                for i in 0..Self::BPL {
                                    row.col(|ui: &mut egui::Ui| {
                                        if start + i < data.len() {
                                            let off = start + i;
                                            let text = format!("{:02X}", data[off]);

                                            // Create a clickable area without text selection
                                            let (rect, resp) = ui.allocate_exact_size(
                                                egui::vec2(14.0, 18.0),
                                                egui::Sense::click_and_drag()
                                            );
                                            
                                            // Draw the text manually
                                            ui.painter().text(
                                                rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                text,
                                                egui::TextStyle::Monospace.resolve(ui.style()),
                                                ui.visuals().text_color(),
                                            );
                                            
                                            // Check if this byte is in selection range
                                            let is_selected = self.selection.as_ref()
                                                .map(|sel| sel.contains(off))
                                                .unwrap_or(false);
                                            
                                            // Selection highlighting
                                            if is_selected {
                                                let r = rect.expand2(egui::vec2(1.0, 1.0));
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
                                            self.handle_drag(&resp, DragStatus::Bytes(off));

                                        } else {
                                            ui.monospace("  ");
                                        }
                                    });
                                }
                                row.col(|ui| {
                                    // Render ASCII characters with individual interaction
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 0.0; // No spacing between chars
                                        
                                        for i in 0..(end - start) {
                                            let off = start + i;
                                            let byte = data[off];
                                            let ch = if byte.is_ascii_graphic() {
                                                byte as char
                                            } else {
                                                '.'
                                            };
                                            
                                            // Check if this character is selected
                                            let is_selected = self.selection.as_ref()
                                                .map(|sel| sel.contains(off))
                                                .unwrap_or(false);
                                            
                                            // Create a clickable area for each character without text selection
                                            let char_width = ui.fonts(|f| f.glyph_width(&egui::TextStyle::Monospace.resolve(ui.style()), 'W'));
                                            let (rect, resp) = ui.allocate_exact_size(
                                                egui::vec2(char_width, 18.0),
                                                egui::Sense::click_and_drag()
                                            );
                                            
                                            // Draw the character manually
                                            ui.painter().text(
                                                rect.center(),
                                                egui::Align2::CENTER_CENTER,
                                                ch.to_string(),
                                                egui::TextStyle::Monospace.resolve(ui.style()),
                                                ui.visuals().text_color(),
                                            );
                                            
                                            // Highlight selected characters
                                            if is_selected {
                                                let r = rect.expand2(egui::vec2(0.0, 1.0));
                                                ui.painter().rect_filled(
                                                    r,
                                                    2.0,
                                                    egui::Color32::from_rgb(100, 150, 255),
                                                );
                                                // Redraw character on top
                                                ui.painter().text(
                                                    r.center(),
                                                    egui::Align2::CENTER_CENTER,
                                                    ch.to_string(),
                                                    egui::TextStyle::Monospace
                                                        .resolve(ui.style()),
                                                    ui.visuals().strong_text_color(),
                                                );
                                            }

                                            self.handle_drag(&resp, DragStatus::ASCII(off));

                                        }
                                    });
                                });
                            });
                        }
                    });
            });

        });
    }
}
