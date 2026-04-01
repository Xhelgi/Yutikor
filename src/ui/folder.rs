// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use eframe::egui;
use rfd::FileDialog;
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::app::FolderState;

pub fn draw_folder_select_panel(
    ctx: &egui::Context,
    folder_state: &mut FolderState,
    path: &mut Option<PathBuf>,
) {
    let text_edit_width = 600.0;
    let options_width = 500.0;
    let text_edit_height = 40.0;
    let options_height = 20.0;

    let error_id = egui::Id::new("PathErrorPopup");

    let mut last_readed_folders: VecDeque<PathBuf> = VecDeque::new();
    let mut cache_file: Option<PathBuf> = None;

    if let Some(cache) = dirs::cache_dir() {
        let cache_dir = cache.join("yutikor");
        if let Ok(_) = fs::create_dir_all(&cache_dir) {
            let cf = cache_dir.join("last_readed");
            if let Ok(json_string) = fs::read_to_string(&cf) {
                last_readed_folders = serde_json::from_str(&json_string).unwrap_or_default();
            } else {
                fs::write(&cf, "").unwrap_or_else(
                    |_| { println!("Cannot write into cache File! List of last-selected pathes will be unaviable!"); }
                );
            }
            while last_readed_folders.len() > 10 {
                last_readed_folders.pop_front();
            }
            cache_file = Some(cf);
        } else {
            println!("Cannot create a directory ~/.cache/yutikor/");
        }
    } else {
        println!("Cannot get cache path to save last selected folders");
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            let center = ui.available_rect_before_wrap().center();
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);

                ui.add_sized([text_edit_width, text_edit_height], |ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [text_edit_width * 0.1, text_edit_height],
                                egui::Button::new("<|=|>"),
                            )
                            .clicked()
                        {
                            if let Some(file) = FileDialog::new()
                                .set_directory("/")
                                .pick_folder()
                                && let Some(sclice) = file.to_str() {
                                    folder_state.path_line = String::from(sclice);
                                }
                        }
                        ui.add_sized(
                            [text_edit_width * 0.8, text_edit_height],
                            egui::TextEdit::singleline(&mut folder_state.path_line),
                        );
                        if ui
                            .add_sized(
                                [text_edit_width * 0.1, text_edit_height],
                                egui::Button::new("Select"),
                            )
                            .clicked()
                            && !folder_state.path_line.is_empty()
                        {
                            let new_path = PathBuf::from(&folder_state.path_line);
                            let test_name = format!("test_file_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().subsec_nanos());
                            println!("test name == {test_name}");
                            if let Ok(exist) = fs::exists(&new_path)
                                && exist 
                                && let Ok(_) = fs::write(&new_path.join(&test_name), "")
                                && let Ok(_) = fs::read(&new_path.join(&test_name))
                                && let Ok(_) = fs::remove_file(&new_path.join(&test_name))
                            {
                                *path = Some(new_path.clone());
                                if !last_readed_folders.contains(&new_path) {
                                    last_readed_folders.push_back(new_path);
                                }
                            } else {
                                ctx.data_mut(|d| d.insert_temp(error_id, true));
                            }
                        }
                    })
                    .response
                });
                ui.add_space(20.0);

                ui.label("Last pathes:");

                let mut index_to_remove: Option<usize> = None;
                for (index, f_path) in last_readed_folders.iter().enumerate() {
                    if let Some(text) = f_path.to_str() {
                        let resp =
                            ui.add_sized([options_width, options_height], egui::Button::new(text));
                        if resp.clicked() {
                            folder_state.path_line = String::from(text);
                        }
                        resp.context_menu(|ui| {
                            if ui.button("Remove").clicked() {
                                index_to_remove = Some(index);
                            }
                        });
                    } else {
                        index_to_remove = Some(index);
                    }
                }
                if let Some(index) = index_to_remove {
                    last_readed_folders.remove(index);
                }

                let err: bool = ctx.data(|d| d.get_temp(error_id).unwrap_or(false));
                if err {
                    egui::Window::new("Error")
                        .collapsible(false)
                        .resizable(false)
                        .default_pos(center)
                        .show(ctx, |ui| {
                            ui.label("Wrong Folder-Path!");
                            if ui.button("Ok").clicked() {
                                ctx.data_mut(|d| d.insert_temp(error_id, false));
                            }
                        });
                }
            });
        });

    if let Some(cf) = cache_file {
        let json_string = serde_json::to_string(&last_readed_folders).unwrap_or_default();
        if let Err(e) = fs::write(&cf, json_string) {
            println!("{e}");
        }
    }
}
