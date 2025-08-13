use rsearch::ui::BinarySearchApp;
use eframe::egui;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Binary Search Tool",
        options,
        Box::new(|cc| Ok(Box::new(BinarySearchApp::new(cc)))),
    )
}
