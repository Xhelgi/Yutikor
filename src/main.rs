mod app;
mod data;
mod ui;

use app::Yuti;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "Yutikora",
        options,
        Box::new(|cc| Ok(Box::new(Yuti::new(cc)))),
    )
}
