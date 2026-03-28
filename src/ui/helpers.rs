use std::{fs, path::Path};

use eframe::egui::{
    self, Align, Align2, Color32, FontId, Id, Margin, Pos2, Rect, Sense, Stroke, TextFormat, Vec2,
    text::LayoutJob,
};

use crate::{
    app::{NodeState, Yuti},
    data::{LinkType, Node, Object, Page, PageLink},
};

pub struct ObjectsMainColors {
    pub bg_color: Color32,
    pub font_color: Color32,
    pub stroke_color: Color32,
}

impl Yuti {
    pub fn save_page(to_save_path: &Path, path: &Path, crt_page: &Page) {
        let json_string =
            serde_json::to_string_pretty(crt_page).expect("Cannot serialize User Page");
        fs::write(path.join(to_save_path), json_string).expect("Cannot save UserPage file!");
    }

    pub fn get_links_by_path(path: &Path, root_node: &Node) -> Vec<PageLink> {
        let mut links: Vec<PageLink> = Vec::new();
        let root_pos = root_node.get_pos();
        if root_node.path == path {
            for sub_page in root_node.sub_nodes.iter().flatten() {
                let sub_pos = sub_page.get_pos();
                let vec = (sub_pos - root_pos).normalized() * 10_000.0;
                links.push(PageLink {
                    link_type: LinkType::ChildLink,
                    direction_vec: (vec.x, vec.y),
                    file_name: sub_page.path.clone(),
                });
            }
            return links;
        } else {
            for sub_page in root_node.sub_nodes.iter().flatten() {
                if sub_page.path == path {
                    let sub_pos = sub_page.get_pos();
                    let vec = (root_pos - sub_pos).normalized() * 10_000.0;
                    links.push(PageLink {
                        link_type: LinkType::ParentLink,
                        direction_vec: (vec.x, vec.y),
                        file_name: root_node.path.clone(),
                    });
                    for sub_sub_page in sub_page.sub_nodes.iter().flatten() {
                        let sub_sub_pos = sub_sub_page.get_pos();
                        let sub_vec = (sub_sub_pos - sub_pos).normalized() * 10_000.0;
                        links.push(PageLink {
                            link_type: LinkType::ChildLink,
                            direction_vec: (sub_vec.x, sub_vec.y),
                            file_name: sub_sub_page.path.clone(),
                        });
                    }
                    return links;
                }
            }
            for sub_page in root_node.sub_nodes.iter().flatten() {
                let res = Self::get_links_by_path(path, sub_page);
                if res.len() > 0 {
                    return res;
                }
            }
        };
        Vec::new()
    }

    pub fn draw_node_recursiv(
        ui: &mut egui::Ui,
        node: &mut Node,
        is_root: bool,
        node_state: &mut NodeState,
    ) {
        // * Settings
        let circle_radius = 30.0;
        let circle_color = if is_root { Color32::RED } else { Color32::GRAY };
        let circle_stroke = Stroke::new(2.0, Color32::DARK_RED);
        let line_stroke = Stroke::new(4.0, Color32::LIGHT_RED);
        let arrow_length = 50.0;
        let font_size = 16.0;
        let font_color = Color32::BLACK;

        let start_coord = node_state.start_coord;
        let start_coord_vec = Vec2::new(start_coord.0, start_coord.1);
        let node_pos = node.get_pos() + start_coord_vec;
        let node_pos_clear = node.get_pos();
        if let Some(sub_nodes) = &mut node.sub_nodes {
            for sub_node in sub_nodes.iter_mut() {
                {
                    let painter = ui.painter();
                    let sub_node_pos = sub_node.get_pos() + start_coord_vec;
                    let vec_to_sub_node = (sub_node_pos - node_pos).normalized();
                    painter.line_segment([node_pos, sub_node_pos], line_stroke);
                    painter.arrow(node_pos, vec_to_sub_node * arrow_length, line_stroke);
                }
                Self::draw_node_recursiv(ui, sub_node, false, node_state);
            }
        }
        {
            let painter = ui.painter();
            painter.circle(node_pos, circle_radius, circle_color, circle_stroke);
            painter.text(
                node_pos,
                Align2::CENTER_CENTER,
                &node.name,
                FontId::monospace(font_size),
                font_color,
            );

            // ! Logic Process
            let resp: egui::Response = ui.allocate_rect(
                Rect::from_center_size(node_pos, Vec2::splat(circle_radius * 2.0)),
                Sense::click_and_drag(),
            );
            if resp.clicked() {
                node_state.node_to_load_by_path = Some(node.path.clone());
            }
            if resp.dragged() {
                node.pos.0 += resp.drag_delta().x;
                node.pos.1 += resp.drag_delta().y;
            }
            resp.context_menu(|ui| {
                if ui.button("Add SubNode").clicked() {
                    let mut new_node = Node::default();
                    new_node.pos = (node_pos_clear.x + 80.0, node_pos_clear.y + 40.0);
                    if let Some(sub_nodes) = &mut node.sub_nodes {
                        sub_nodes.push(new_node);
                    } else {
                        node.sub_nodes = Some(vec![new_node]);
                    }
                }
                if ui.button("Remove Node").clicked() {
                    node_state.node_to_remove_by_path = Some(node.path.clone());
                }
                ui.text_edit_singleline(&mut node.name);
            });
        }
    }

    pub fn search_and_remove_node_recursiv(node: &mut Node, path: &Path) {
        if let Some(sub_nodes) = &mut node.sub_nodes {
            if let Some((index, _)) = sub_nodes.iter().enumerate().find(|(_, n)| n.path == path) {
                sub_nodes.remove(index);
                return;
            }
            for sub_node in sub_nodes.iter_mut() {
                Self::search_and_remove_node_recursiv(sub_node, path);
            }
        }
    }

    pub fn save_graph(&self) {
        if let Some(graph_root) = &self.graph_root_node {
            if let Ok(json_string) = serde_json::to_string(graph_root) {
                fs::write(self.path.join("graph.base"), &json_string)
                    .expect("Cannot save GraphFile!");
            }
        }
    }

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
            colors.bg_color = Color32::LIGHT_GRAY;
            colors.font_color = Color32::DARK_GRAY;
        }

        colors
    }

    pub fn add_edit_text(
        ui: &mut egui::Ui,
        rect: Rect,
        text: &mut String,
        font_color: Color32,
        font_size: f32,
        text_offset: (f32, f32),
    ) {
        ui.put(
            rect,
            egui::TextEdit::multiline(text)
                .frame(false)
                .text_color(font_color)
                .font(FontId::monospace(font_size))
                .margin(Margin {
                    left: text_offset.0 as i8,
                    right: text_offset.0 as i8,
                    top: text_offset.1 as i8,
                    bottom: text_offset.1 as i8,
                }),
        );
    }

    pub fn add_label_text(
        painter: &egui::Painter,
        rect: Rect,
        text: &mut String,
        font_size: f32,
        font_color: Color32,
        text_offset: (f32, f32),
        text_align: u8,
    ) {
        let font_id = FontId::monospace(font_size);

        let row_height = painter.fonts_mut(|i| i.row_height(&font_id));
        let max_rows = ((rect.height() - text_offset.1 * 2.0) / row_height).floor() as usize;

        let align = match text_align {
            0 => Align::Center,
            1 => Align::LEFT,
            2 => Align::RIGHT,
            _ => panic!("Wrong align-number!"),
        };

        let pos = match text_align {
            0 => rect.center_top() + Vec2::new(0.0, text_offset.1),
            1 => rect.left_top() + Vec2::new(text_offset.0, text_offset.1),
            2 => rect.right_top() + Vec2::new(-text_offset.0, text_offset.1),
            _ => panic!("Wrong align-number!"),
        };

        if max_rows != 0 {
            let mut job = LayoutJob::default();
            job.halign = align;
            job.append(
                text,
                0.0,
                TextFormat {
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
        point: Pos2,
        is_dragged: &mut bool,
    ) {
        let painter = ui.painter();
        let left_top_corner_rect = Rect::from_center_size(point, Vec2::splat(10.0));
        painter.rect_filled(left_top_corner_rect, 0.0, Color32::DARK_RED);
        if ui
            .interact(left_top_corner_rect, Id::new("lefttop"), Sense::drag())
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
        point: Pos2,
        is_dragged: &mut bool,
    ) {
        let painter = ui.painter();
        let right_buttom_corner_rect = Rect::from_center_size(point, Vec2::splat(10.0));
        painter.rect_filled(right_buttom_corner_rect, 0.0, Color32::DARK_RED);
        if ui
            .interact(
                right_buttom_corner_rect,
                Id::new("rightbottom"),
                Sense::drag(),
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

    pub fn fix_object_position_to_grid_standart(
        obj: &mut Object,
        grid_size: f32,
        is_dragged: &bool,
    ) {
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
        left_top_point: Pos2,
        right_bottom_point: Pos2,
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
}
