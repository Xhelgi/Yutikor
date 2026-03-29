use eframe::egui;

use crate::{
    app::{NodeState, State},
    data::{LinkType, Page},
};

mod help_background;
mod help_objects;
mod help_page_links;

pub mod logic;

pub fn create_content_panel(
    ctx: &egui::Context,
    crt_page: &mut Page,
    node_state: &mut NodeState,
    state: &mut State,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            draw_and_process_page_links(ctx, ui, node_state);
            let inner_rect = ui.available_rect_before_wrap().shrink(30.0);
            ui.scope_builder(egui::UiBuilder::new().max_rect(inner_rect), |ui| {
                draw_and_process_background(ctx, ui, crt_page, state);
                draw_and_process_objects(ctx, ui, crt_page, state);
            });
        });
}

fn draw_and_process_page_links(ctx: &egui::Context, ui: &mut egui::Ui, node_state: &mut NodeState) {
    // * SETTINGS
    let margin = 30.0;
    let parent_link_color = egui::Color32::RED;
    let child_link_color = egui::Color32::GREEN;
    let line_width = 30.0;
    let assept_angle_diff = 0.12;
    // *

    let aviable_rect = ui.available_rect_before_wrap();
    let painter = ui.painter();

    // * 0. Draw Line
    for page_link in node_state.page_links.iter() {
        let stroke_color = if page_link.link_type == LinkType::ParentLink {
            parent_link_color
        } else {
            child_link_color
        };
        let stroke = egui::Stroke::new(line_width, stroke_color);

        painter.arrow(
            aviable_rect.center(),
            egui::Vec2::new(page_link.direction_vec.0, page_link.direction_vec.1),
            stroke,
        );
    }

    // * 1. Check Trigger
    let in_link_area = help_page_links::is_mouse_in_link_area(ui, aviable_rect, margin);

    // * 2. Process Click
    if in_link_area {
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
            let mouse_vec_angle = (mouse_pos - aviable_rect.center()).angle();
            for link in node_state.page_links.iter() {
                let link_vec_angle =
                    egui::Vec2::new(link.direction_vec.0, link.direction_vec.1).angle();
                let delta = (link_vec_angle - mouse_vec_angle).abs();
                if delta < assept_angle_diff {
                    node_state.page_to_switch = Some(link.file_name.clone());
                }
            }
        }
    }
}

fn draw_and_process_background(
    ctx: &egui::Context,
    ui: &egui::Ui,
    crt_page: &mut Page,
    state: &mut State,
) {
    // * Settings
    let background_color = egui::Color32::WHITE;
    // *

    let aviable_rect = ui.available_rect_before_wrap();

    // * 0. Draw Backgound
    ui.painter()
        .rect_filled(aviable_rect, 0.0, background_color);

    // * 1. Process Events
    let bg_response = ui.interact(
        aviable_rect,
        egui::Id::new("BackgourndResponse"),
        egui::Sense::click(),
    );
    if bg_response.clicked() {
        state.selected_object_id = None;
        state.is_selected_for_text_edit = false;
    }

    // * 2. Create ContextMenu
    help_background::create_background_context_menu(ctx, &bg_response, crt_page);
}

fn draw_and_process_objects(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    crt_page: &mut Page,
    state: &mut State,
) {
    let aviable_rect = ui.available_rect_before_wrap();
    let painter = ui.painter().with_clip_rect(aviable_rect);

    for (index, obj) in crt_page.objects.iter_mut().enumerate() {
        let rect = egui::Rect::from_min_max(obj.get_start_pos(), obj.get_end_pos());
        let is_selected = state.selected_object_id == Some(index);
        let colors = help_objects::get_colors(obj, is_selected, state.is_selected_for_text_edit);

        // * 0. Draw Rect
        painter.rect(
            rect,
            obj.corner_radius,
            colors.bg_color,
            egui::Stroke::new(obj.stroke_width, colors.stroke_color),
            egui::StrokeKind::Inside,
        );
        // * 1. Draw Text
        if is_selected && state.is_selected_for_text_edit {
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

        //* 2. Process Events
        let obj_resp = help_objects::process_object_events(ui, obj, index, rect, state);

        //* 3. Create ContextMenu
        help_objects::create_object_context_menu(&obj_resp, index, &mut state.object_to_remove_id);

        //* 4. Add Resize-Markers for selected Object
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

        //* 5. Pos/Size <---> GridSize && Pos <---> AviableRect
        help_objects::fix_object_size_to_grid_standart(obj, state.grid_size);
        help_objects::fix_object_position_to_grid_standart(obj, state.grid_size, &is_dragged);
        help_objects::fix_object_position_to_aviable_rect(
            obj,
            aviable_rect.left_top(),
            aviable_rect.right_bottom(),
        );
    }
}
