// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use eframe::egui;
use std::path::Path;

use crate::{
    app::{EditorState, GraphState},
    data::{LinkType, Page},
};

mod help_background;
mod help_objects;
mod help_page_links;

pub mod logic;

pub fn create_content_panel(
    ctx: &egui::Context,
    crt_page: &mut Page,
    graph_state: &mut GraphState,
    editor_state: &mut EditorState,
    project_path: &Path,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            draw_and_process_page_links(ctx, ui, graph_state);
            let inner_rect = ui.available_rect_before_wrap().shrink(30.0);
            ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| {
                draw_and_process_background(ctx, ui, crt_page, editor_state, project_path);
                draw_and_process_objects(ctx, ui, crt_page, editor_state, project_path);
            });
        });
}

fn draw_and_process_page_links(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    graph_state: &mut GraphState,
) {
    let margin = 30.0;
    let parent_link_color = egui::Color32::RED;
    let child_link_color = egui::Color32::GREEN;
    let line_width = 30.0;
    let accept_angle_diff = 0.12;

    let available_rect = ui.available_rect_before_wrap();
    let painter = ui.painter();

    for page_link in graph_state.page_links.iter() {
        let stroke_color = if page_link.link_type == LinkType::ParentLink {
            parent_link_color
        } else {
            child_link_color
        };
        painter.arrow(
            available_rect.center(),
            egui::Vec2::new(page_link.direction_vec.0, page_link.direction_vec.1),
            egui::Stroke::new(line_width, stroke_color),
        );
    }

    let in_link_area = help_page_links::is_mouse_in_link_area(ui, available_rect, margin);
    if in_link_area {
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
            let mouse_vec_angle = (mouse_pos - available_rect.center()).angle();
            for link in graph_state.page_links.iter() {
                let link_vec_angle =
                    egui::Vec2::new(link.direction_vec.0, link.direction_vec.1).angle();
                let delta = (link_vec_angle - mouse_vec_angle).abs();
                if delta < accept_angle_diff {
                    graph_state.page_to_switch = Some(link.file_name.clone());
                }
            }
        }
    }
}

fn draw_and_process_background(
    ctx: &egui::Context,
    ui: &egui::Ui,
    crt_page: &mut Page,
    editor_state: &mut EditorState,
    project_path: &Path,
) {
    let available_rect = ui.available_rect_before_wrap();
    ui.painter()
        .rect_filled(available_rect, 0.0, egui::Color32::WHITE);

    let bg_response = ui.interact(
        available_rect,
        egui::Id::new("BackgroundResponse"),
        egui::Sense::click(),
    );
    if bg_response.clicked() {
        editor_state.selected_object_id = None;
        editor_state.is_selected_for_text_edit = false;
    }
    help_background::create_background_context_menu(ctx, &bg_response, crt_page, project_path);
}

fn draw_and_process_objects(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    crt_page: &mut Page,
    editor_state: &mut EditorState,
    project_path: &Path,
) {
    let available_rect = ui.available_rect_before_wrap();
    let painter = ui.painter().with_clip_rect(available_rect);

    for (index, obj) in crt_page.objects.iter_mut().enumerate() {
        let rect = egui::Rect::from_min_max(obj.get_start_pos(), obj.get_end_pos());
        let is_selected = editor_state.selected_object_id == Some(index);
        let is_editing = is_selected && editor_state.is_selected_for_text_edit;
        let colors =
            help_objects::get_colors(obj, is_selected, editor_state.is_selected_for_text_edit);

        // 0. Draw Rect (рамка всегда, даже для картинок)
        painter.rect(
            rect,
            obj.corner_radius,
            colors.bg_color,
            egui::Stroke::new(obj.stroke_width, colors.stroke_color),
            egui::StrokeKind::Inside,
        );

        // 1. Контент — картинка или текст
        if obj.image_path.is_some() {
            help_objects::add_image(ui, ctx, obj, rect, project_path);
        } else if is_editing {
            help_objects::add_edit_text(
                ui,
                rect,
                &mut obj.text,
                colors.font_color,
                obj.font_size,
                obj.text_offset,
            );
        } else {
            help_objects::add_label_text(
                &painter,
                rect,
                &mut obj.text,
                obj.font_size,
                colors.font_color,
                obj.text_offset,
                obj.text_align,
            );
        }

        // 2. События — для картинок двойной клик не открывает текстовый редактор
        let obj_resp = if is_editing {
            let hover_resp = ui.interact(rect, egui::Id::new(index), egui::Sense::hover());
            help_objects::create_object_context_menu(
                &hover_resp,
                index,
                &mut editor_state.object_to_remove_id,
            );
            hover_resp
        } else {
            let obj_resp = help_objects::process_object_events(ui, obj, index, rect, editor_state);
            help_objects::create_object_context_menu(
                &obj_resp,
                index,
                &mut editor_state.object_to_remove_id,
            );
            obj_resp
        };
        let _ = obj_resp;

        // 3. Resize-маркеры
        let mut is_dragged = false;
        if is_selected {
            help_objects::create_left_top_corner(ctx, ui, obj, rect.left_top(), &mut is_dragged);
            help_objects::create_right_bottom_corner(
                ctx,
                ui,
                obj,
                rect.right_bottom(),
                &mut is_dragged,
            );
        }

        // 4. Привязка к сетке и доступной области
        help_objects::fix_object_size_to_grid_standart(obj, editor_state.grid_size);
        help_objects::fix_object_position_to_grid_standart(
            obj,
            editor_state.grid_size,
            &is_dragged,
        );
        // help_objects::fix_object_position_to_aviable_rect(
        //     obj,
        //     available_rect.left_top(),
        //     available_rect.right_bottom(),
        // );
    }
}
