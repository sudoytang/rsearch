use rsearch::ui::BinarySearchApp;
use eframe::egui;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([BinarySearchApp::APP_MIN_WIDTH, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Binary Search Tool",
        options,
        Box::new(|cc| Ok(Box::new(BinarySearchApp::new(cc)))),
    )
}
