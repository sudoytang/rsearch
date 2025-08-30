use eframe::egui;
use std::path::PathBuf;

pub struct FilePanel {
    file_path: Option<PathBuf>,
    file_data: Option<Vec<u8>>,
}

impl FilePanel {
    pub fn new() -> Self {
        Self {
            file_path: None,
            file_data: None,
        }
    }

    pub fn get_file_path(&self) -> &Option<PathBuf> {
        &self.file_path
    }

    pub fn get_file_data(&self) -> Option<&[u8]> {
        self.file_data.as_ref().map(|v| v.as_slice())
    }

    pub fn clear_file(&mut self) {
        self.file_path = None;
        self.file_data = None;
    }

    fn open_file(&mut self) {
        // TODO: Implement file opening logic
        // - Show native file dialog
        // - Read file into memory
        // - Update file_path and file_data
        // - Clear previous search results
        
        // WARN(cursor): Mocking file dialog for UI testing
        // In a real implementation, use rfd::FileDialog::new().pick_file()
        self.file_path = Some(PathBuf::from("mock_file.bin"));
        self.file_data = Some((0..1024).map(|v| { v as u8 }).collect());
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut file_opened = false;

        // Top section - File controls
        ui.horizontal(|ui| {
            if ui.button("Open File").clicked() {
                self.open_file();
                file_opened = true;
            }
            
            if let Some(path) = &self.file_path {
                ui.label(format!("File: {}", path.display()));
            } else {
                ui.label("No file loaded");
            }
        });

        file_opened
    }
}
