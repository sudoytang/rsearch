use eframe::egui;

pub struct DataInspector {
}

impl DataInspector {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected_offset: Option<usize>, file_data: &Option<Vec<u8>>) {
        ui.group(|ui| {
            ui.label("Data Inspector");
            
            if let Some(offset) = selected_offset {
                if let Some(data) = file_data {
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
