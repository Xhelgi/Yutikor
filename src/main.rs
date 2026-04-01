// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

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
        "Yutikor",
        options,
        Box::new(|cc| Ok(Box::new(Yuti::new(cc)))),
    )
}
