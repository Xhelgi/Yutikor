use crate::data::{Object, Page};
use eframe::egui;

pub fn create_background_context_menu(
    ctx: &egui::Context,
    bg_resp: &egui::Response,
    crt_page: &mut Page,
) {
    bg_resp.context_menu(|ui| {
        if ui.button("Add new").clicked() {
            let mouse_pos = ctx
                .input(|i| i.pointer.interact_pos())
                .unwrap_or(egui::Pos2::new(10.0, 10.0));

            crt_page.objects.push(Object::new("New Object", mouse_pos));
        }
    });
}
