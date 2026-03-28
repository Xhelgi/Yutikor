use eframe::{
    egui::{self, Color32, Id, Pos2, Rect, Response, Sense, Stroke, Vec2},
};

use crate::{
    app::{NodeState, State, Yuti},
    data::{LinkType, Object, Page},
};

impl Yuti {
    pub fn process_page_links(ctx: &egui::Context, ui: &mut egui::Ui, node_state: &mut NodeState) {
        let aviable_rect = ui.available_rect_before_wrap();
        let painter = ui.painter();
        for page_link in node_state.page_links.iter() {
            let stroke_color = if page_link.link_type == LinkType::ParentLink {
                Color32::RED
            } else {
                Color32::GREEN
            };
            let stroke = Stroke::new(30.0, stroke_color);
            painter.arrow(
                aviable_rect.center(),
                Vec2::new(page_link.direction_vec.0, page_link.direction_vec.1),
                stroke,
            );
        }

        let mut in_link_area = false;

        let left_rect = Rect::from_min_max(
            aviable_rect.left_top(),
            aviable_rect.left_bottom() + Vec2::new(30.0, 0.0),
        );
        let right_rect = Rect::from_min_max(
            aviable_rect.right_top() + Vec2::new(-30.0, 0.0),
            aviable_rect.right_bottom(),
        );
        let top_rect = Rect::from_min_max(
            aviable_rect.left_top(),
            aviable_rect.right_top() + Vec2::new(0.0, 30.0),
        );
        let bottom_rect = Rect::from_min_max(
            aviable_rect.left_bottom() + Vec2::new(0.0, -30.0),
            aviable_rect.right_bottom(),
        );

        if ui
            .interact(left_rect, Id::new("left_link_panel"), Sense::click())
            .clicked()
        {
            in_link_area = true;
        }
        if ui
            .interact(right_rect, Id::new("right_link_panel"), Sense::click())
            .clicked()
        {
            in_link_area = true;
        }
        if ui
            .interact(top_rect, Id::new("top_link_panel"), Sense::click())
            .clicked()
        {
            in_link_area = true;
        }
        if ui
            .interact(bottom_rect, Id::new("bottom_link_panel"), Sense::click())
            .clicked()
        {
            in_link_area = true;
        }

        if in_link_area {
            if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                let mouse_vec_angle = (mouse_pos - aviable_rect.center()).angle();
                for link in node_state.page_links.iter() {
                    let link_vec_angle =
                        Vec2::new(link.direction_vec.0, link.direction_vec.1).angle();
                    let delta = (link_vec_angle - mouse_vec_angle).abs();
                    if delta < 0.12 {
                        node_state.page_to_switch = Some(link.file_name.clone());
                    }
                }
            }
        }
    }

    pub fn process_background_events(
        ctx: &egui::Context,
        ui: &egui::Ui,
        crt_page: &mut Page,
        state: &mut State,
    ) {
        let aviable_rect = ui.available_rect_before_wrap();
        let bg_response = ui.interact(aviable_rect, Id::new("BackgourndResponse"), Sense::click());
        ui.painter().rect_filled(aviable_rect, 0.0, Color32::WHITE);
        bg_response.context_menu(|ui| {
            if ui.button("Add new").clicked() {
                let mouse_pos = ctx
                    .input(|i| i.pointer.interact_pos())
                    .unwrap_or(Pos2::new(10.0, 10.0));

                crt_page.objects.push(Object::new("New Object", mouse_pos));
            }
        });

        if bg_response.clicked() {
            state.selected_object_id = None;
            state.is_selected_for_text_edit = false;
        }
    }

    pub fn process_objects_list(
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        crt_page: &mut Page,
        state: &mut State,
    ) {
        let aviable_rect = ui.available_rect_before_wrap();
        let painter = ui.painter().with_clip_rect(aviable_rect);

        for (index, obj) in crt_page.objects.iter_mut().enumerate() {
            let rect = Rect::from_min_max(obj.get_start_pos(), obj.get_end_pos());
            let is_selected = state.selected_object_id == Some(index);

            // Get colors (bg, font, stroke)
            let colors = Self::get_colors(obj, is_selected, state.is_selected_for_text_edit);

            // Rect
            painter.rect(
                rect,
                obj.corner_radius,
                colors.bg_color,
                Stroke::new(obj.stroke_width, colors.stroke_color),
                egui::StrokeKind::Inside,
            );

            // Response (click & double-click & drag)
            let obj_resp = Self::create_object_response(ui, obj, index, rect, state);

            // Create Context Menu
            Self::create_context_menu(&obj_resp, index, &mut state.object_to_remove_id);

            // Text Block
            if is_selected && state.is_selected_for_text_edit {
                Self::add_edit_text(
                    ui,
                    rect,
                    &mut obj.text,
                    colors.font_color,
                    obj.font_size,
                    obj.text_offset,
                );
            } else {
                Self::add_label_text(
                    &painter,
                    rect,
                    &mut obj.text,
                    obj.font_size,
                    colors.font_color,
                    obj.text_offset,
                    obj.text_align,
                );
            }

            // Add Resize-Blocks if selected
            // ? PositionFix only if resize was ended (else BUGs)
            let mut is_dragged = false;
            if is_selected {
                Self::create_left_top_corner(ctx, ui, obj, rect.left_top(), &mut is_dragged);
                Self::create_right_bottom_corner(
                    ctx,
                    ui,
                    obj,
                    rect.right_bottom(),
                    &mut is_dragged,
                );
            }

            // End Fix
            Self::fix_object_size_to_grid_standart(obj, state.grid_size);
            Self::fix_object_position_to_grid_standart(obj, state.grid_size, &is_dragged);
            Self::fix_object_position_to_aviable_rect(
                obj,
                aviable_rect.left_top(),
                aviable_rect.right_bottom(),
            );
        }
    }

    fn create_object_response(
        ui: &egui::Ui,
        obj: &mut Object,
        index: usize,
        rect: Rect,
        state: &mut State,
    ) -> Response {
        let obj_resp = ui.interact(rect, Id::new(index), Sense::click_and_drag());
        if obj_resp.clicked() {
            state.selected_object_id = Some(index);
            state.is_selected_for_text_edit = false;
        }
        if obj_resp.double_clicked() {
            state.selected_object_id = Some(index);
            state.is_selected_for_text_edit = true;
        }
        if obj_resp.dragged() {
            obj.pos.0 += obj_resp.drag_motion().x;
            obj.pos.1 += obj_resp.drag_motion().y;
        };

        obj_resp
    }

    fn create_context_menu(
        obj_resp: &Response,
        index: usize,
        object_to_remove_id: &mut Option<usize>,
    ) {
        obj_resp.context_menu(|ui| {
            if ui.button("Remove").clicked() {
                *object_to_remove_id = Some(index);
            };
        });
    }
}
