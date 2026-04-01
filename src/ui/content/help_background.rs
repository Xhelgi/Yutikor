// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use crate::data::{Object, Page};
use eframe::egui;
use std::fs;

pub fn create_background_context_menu(
    ctx: &egui::Context,
    bg_resp: &egui::Response,
    crt_page: &mut Page,
    project_path: &std::path::Path,
) {
    bg_resp.context_menu(|ui| {
        let mouse_pos = ctx
            .input(|i| i.pointer.interact_pos())
            .unwrap_or(egui::Pos2::new(10.0, 10.0));

        if ui.button("Add Object").clicked() {
            crt_page.objects.push(Object::new("New Object", mouse_pos));
            ui.close();
            // ui.close_menu();
        }

        if ui.button("Add Image").clicked() {
            // Открываем файловый диалог синхронно
            if let Some(src_path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
                let file_name = src_path.file_name().unwrap_or_default();
                let dest_path = project_path.join(file_name);

                // // Копируем файл в папку проекта
                // let file_name = src_path
                //     .file_name()
                //     .unwrap_or_default()
                //     .to_string_lossy()
                //     .to_string();
                // let dest_path = project_path.join(&file_name);

                if src_path != dest_path {
                    _ = fs::copy(&src_path, &dest_path);
                }

                // Сохраняем только имя файла — путь относительный
                crt_page.objects.push(Object::new_image(
                    std::path::PathBuf::from(&file_name),
                    mouse_pos,
                ));
            }
        }
    });
}
