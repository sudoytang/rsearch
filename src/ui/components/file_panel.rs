use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use memmap2::Mmap;

pub struct FilePanel {
    file_path: Option<PathBuf>,
    file_data: Option<Arc<Mmap>>,
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
        self.file_data.as_deref().map(|m| {
            m.as_ref()
        })
    }
    
    pub fn get_file_data_arc(&self) -> Option<Arc<Mmap>> {
        self.file_data.clone()
    }

    pub fn clear_file(&mut self) {
        self.file_path = None;
        self.file_data = None;
    }

    fn open_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Show native file dialog
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            // Open the file
            let file = File::open(&path)?;
            
            // Create memory-mapped file
            let mmap = unsafe { Mmap::map(&file)? };
            
            // Update state
            self.file_path = Some(path);
            self.file_data = Some(Arc::new(mmap));
        }
        
        Ok(())
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut file_opened = false;

        // Top section - File controls
        ui.horizontal(|ui| {
            if ui.button("Open File").clicked() {
                match self.open_file() {
                    Ok(()) => {
                        if self.file_data.is_some() {
                            file_opened = true;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to open file: {}", e);
                        // Optionally show error in UI
                    }
                }
            }
            
            if let Some(path) = &self.file_path {
                ui.label(format!("File: {}", path.file_name().unwrap_or(std::ffi::OsStr::new("??")).to_string_lossy()));
            } else {
                ui.label("No file loaded");
            }
        });

        file_opened
    }
}
