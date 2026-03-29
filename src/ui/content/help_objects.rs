use eframe::egui;

use crate::{
    app::State,
    data::{Object, ObjectsMainColors},
};

pub fn get_colors(
    obj: &Object,
    is_selected: bool,
    is_selected_for_text_edit: bool,
) -> ObjectsMainColors {
    let mut colors = ObjectsMainColors {
        bg_color: obj.get_color(),
        font_color: obj.get_font_color(),
        stroke_color: obj.get_stroke_color(),
    };

    if is_selected && is_selected_for_text_edit {
        colors.bg_color = egui::Color32::LIGHT_GRAY;
        colors.font_color = egui::Color32::DARK_GRAY;
    }

    colors
}

pub fn process_object_events(
    ui: &egui::Ui,
    obj: &mut Object,
    index: usize,
    rect: egui::Rect,
    state: &mut State,
) -> egui::Response {
    let obj_resp = ui.interact(rect, egui::Id::new(index), egui::Sense::click_and_drag());
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

pub fn create_object_context_menu(
    obj_resp: &egui::Response,
    index: usize,
    object_to_remove_id: &mut Option<usize>,
) {
    obj_resp.context_menu(|ui| {
        if ui.button("Remove").clicked() {
            *object_to_remove_id = Some(index);
        };
    });
}

pub fn add_edit_text(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    text: &mut String,
    font_color: egui::Color32,
    font_size: f32,
    text_offset: (f32, f32),
) {
    ui.put(
        rect,
        egui::TextEdit::multiline(text)
            .frame(false)
            .text_color(font_color)
            .font(egui::FontId::monospace(font_size))
            .margin(egui::Margin {
                left: text_offset.0 as i8,
                right: text_offset.0 as i8,
                top: text_offset.1 as i8,
                bottom: text_offset.1 as i8,
            }),
    );
}

pub fn add_label_text(
    painter: &egui::Painter,
    rect: egui::Rect,
    text: &mut String,
    font_size: f32,
    font_color: egui::Color32,
    text_offset: (f32, f32),
    text_align: u8,
) {
    let font_id = egui::FontId::monospace(font_size);

    let row_height = painter.fonts_mut(|i| i.row_height(&font_id));
    let max_rows = ((rect.height() - text_offset.1 * 2.0) / row_height).floor() as usize;

    let align = match text_align {
        0 => egui::Align::Center,
        1 => egui::Align::LEFT,
        2 => egui::Align::RIGHT,
        _ => panic!("Wrong align-number!"),
    };

    let pos = match text_align {
        0 => rect.center_top() + egui::Vec2::new(0.0, text_offset.1),
        1 => rect.left_top() + egui::Vec2::new(text_offset.0, text_offset.1),
        2 => rect.right_top() + egui::Vec2::new(-text_offset.0, text_offset.1),
        _ => panic!("Wrong align-number!"),
    };

    if max_rows != 0 {
        let mut job = egui::text::LayoutJob::default();
        job.halign = align;
        job.append(
            text,
            0.0,
            egui::TextFormat {
                font_id,
                color: font_color,
                ..Default::default()
            },
        );

        job.wrap.max_width = rect.width() - text_offset.0 * 2.0;
        job.wrap.max_rows = max_rows;

        let galley = painter.fonts_mut(|i| i.layout_job(job));
        painter.galley(pos, galley, font_color);
    }
}

pub fn create_left_top_corner(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    obj: &mut Object,
    point: egui::Pos2,
    is_dragged: &mut bool,
) {
    let painter = ui.painter();
    let left_top_corner_rect = egui::Rect::from_center_size(point, egui::Vec2::splat(10.0));
    painter.rect_filled(left_top_corner_rect, 0.0, egui::Color32::DARK_RED);
    if ui
        .interact(
            left_top_corner_rect,
            egui::Id::new("lefttop"),
            egui::Sense::drag(),
        )
        .dragged()
    {
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
            let end_pos = obj.get_end_pos();
            obj.pos.0 = mouse_pos.x;
            obj.pos.1 = mouse_pos.y;
            obj.size.0 = end_pos.x - obj.pos.0;
            obj.size.1 = end_pos.y - obj.pos.1;
            *is_dragged = true;
        }
    }
}

pub fn create_right_bottom_corner(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    obj: &mut Object,
    point: egui::Pos2,
    is_dragged: &mut bool,
) {
    let painter = ui.painter();
    let right_buttom_corner_rect = egui::Rect::from_center_size(point, egui::Vec2::splat(10.0));
    painter.rect_filled(right_buttom_corner_rect, 0.0, egui::Color32::DARK_RED);
    if ui
        .interact(
            right_buttom_corner_rect,
            egui::Id::new("rightbottom"),
            egui::Sense::drag(),
        )
        .dragged()
    {
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
            obj.size.0 = mouse_pos.x - obj.pos.0;
            obj.size.1 = mouse_pos.y - obj.pos.1;
            *is_dragged = true;
        }
    }
}

pub fn fix_object_size_to_grid_standart(obj: &mut Object, grid_size: f32) {
    if obj.size.0 < grid_size {
        obj.size.0 = grid_size
    }
    if obj.size.1 < grid_size {
        obj.size.1 = grid_size
    }
}

pub fn fix_object_position_to_grid_standart(obj: &mut Object, grid_size: f32, is_dragged: &bool) {
    if !is_dragged {
        if obj.pos.0 % grid_size != 0.0 {
            obj.pos.0 = (obj.pos.0 / grid_size).round() * grid_size;
        };
        if obj.pos.1 % grid_size != 0.0 {
            obj.pos.1 = (obj.pos.1 / grid_size).round() * grid_size;
        };
        if obj.size.0 % grid_size != 0.0 {
            obj.size.0 = (obj.size.0 / grid_size).round() * grid_size;
        };
        if obj.size.1 % grid_size != 0.0 {
            obj.size.1 = (obj.size.1 / grid_size).round() * grid_size;
        };
    }
}

pub fn fix_object_position_to_aviable_rect(
    obj: &mut Object,
    left_top_point: egui::Pos2,
    right_bottom_point: egui::Pos2,
) {
    if obj.pos.0 < left_top_point.x {
        obj.pos.0 = left_top_point.x
    }
    if obj.pos.1 < left_top_point.y {
        obj.pos.1 = left_top_point.y
    }
    let end_pos = obj.get_end_pos();
    if end_pos.x > right_bottom_point.x {
        obj.pos.0 = right_bottom_point.x - obj.size.0;
    }
    if end_pos.y > right_bottom_point.y {
        obj.pos.1 = right_bottom_point.y - obj.size.1;
    }
}
